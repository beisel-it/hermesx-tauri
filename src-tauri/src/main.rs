// HermesX (Tauri) — main entry point
// Migration from: https://github.com/florianbeisel/hermesx (Electron v0.5.0)
//
// Architecture:
//   config       → UserConfig + defaults (no I/O)
//   state_machine → WorkState FSM + pure transitions
//   work_monitor  → Activity/schedule evaluation (pure, testable)
//   zeusX         → ZeusX dispatch (dry-run-aware, sidecar stub)
//   main          → Tauri app, IPC commands, tray, event loop

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod state_machine;
mod work_monitor;
mod zeusX;

use config::UserConfig;
use state_machine::{PersistedState, WorkAction, WorkState, apply_transition, available_actions};
use zeusX::{ZeusXAction, action_from_key, dispatch};

use std::sync::Mutex;
use tauri::State;

// --- App State (shared across IPC commands) ---

struct AppState {
    work_state: Mutex<PersistedState>,
    config: Mutex<UserConfig>,
}

// --- Tauri IPC Commands ---

/// Return current state + available actions (for tray menu + UI).
#[tauri::command]
fn get_status(state: State<AppState>) -> serde_json::Value {
    let ws = state.work_state.lock().unwrap();
    let cfg = state.config.lock().unwrap();
    let actions = available_actions(&ws.current_state);
    serde_json::json!({
        "state": ws.current_state,
        "emoji": ws.current_state.emoji(),
        "label": ws.current_state.label(),
        "start_time_ms": ws.start_time_ms,
        "total_worked_ms": ws.total_worked_ms,
        "finished_for_today": ws.finished_for_today,
        "dry_run": cfg.dry_run,
        "available_actions": actions,
    })
}

/// Perform a work state transition.
/// dry_run is read from config — callers cannot override it to false.
#[tauri::command]
async fn perform_action(
    action_label: String,
    state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let cfg = state.config.lock().unwrap().clone();
    let dry_run = cfg.dry_run;

    let mut ws = state.work_state.lock().unwrap();
    let valid = available_actions(&ws.current_state);
    let action = valid
        .into_iter()
        .find(|a| a.label == action_label)
        .ok_or_else(|| format!("Unknown action: {}", action_label))?;

    let result = apply_transition(&mut ws, &action, dry_run)?;

    // Dispatch to ZeusX (or dry-run it)
    let zeusX_result = if let Some(key) = &action.zeusX_action {
        if let Some(zeusX_action) = action_from_key(key) {
            Some(dispatch(zeusX_action, dry_run).await)
        } else {
            None
        }
    } else {
        None
    };

    Ok(serde_json::json!({
        "transition": result,
        "zeusX": zeusX_result.map(|r| match r {
            Ok(d) => serde_json::json!(d),
            Err(e) => serde_json::json!({ "error": e }),
        }),
    }))
}

/// Toggle dry-run mode at runtime (persisted to config).
#[tauri::command]
fn set_dry_run(enabled: bool, state: State<AppState>) -> serde_json::Value {
    let mut cfg = state.config.lock().unwrap();
    cfg.dry_run = enabled;
    serde_json::json!({ "dry_run": cfg.dry_run })
}

/// Get/set full config.
#[tauri::command]
fn get_config(state: State<AppState>) -> UserConfig {
    state.config.lock().unwrap().clone()
}

// --- App Entry ---

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            work_state: Mutex::new(PersistedState::default()),
            config:     Mutex::new(UserConfig::default()),
        })
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_status,
            perform_action,
            set_dry_run,
            get_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running HermesX");
}
