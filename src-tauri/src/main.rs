// HermesX (Tauri) — main entry point
// Migration from: https://github.com/florianbeisel/hermesx (Electron v0.5.0)

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod persistence;
mod zeusX;

use hermesx_core::config::UserConfig;
use hermesx_core::state_machine::{PersistedState, apply_transition, available_actions};
use zeusX::{action_from_key, dispatch};

use std::sync::Mutex;
use tauri::{
    AppHandle, Manager, State,
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};

// ── App State ──────────────────────────────────────────────────────────────

struct AppState {
    work_state: Mutex<PersistedState>,
    config: Mutex<UserConfig>,
}

// ── Tray helpers ───────────────────────────────────────────────────────────

fn build_tray_menu(app: &AppHandle, status_label: &str) -> tauri::Result<Menu<tauri::Wry>> {
    let status = MenuItem::with_id(app, "status", status_label, false, None::<&str>)?;
    let sep    = tauri::menu::PredefinedMenuItem::separator(app)?;
    let show   = MenuItem::with_id(app, "show",   "Open HermesX", true, None::<&str>)?;
    let quit   = MenuItem::with_id(app, "quit",   "Quit",          true, None::<&str>)?;
    Menu::with_items(app, &[&status, &sep, &show, &quit])
}

fn update_tray(app: &AppHandle, emoji: &str, label: &str) {
    if let Some(tray) = app.tray_by_id("main") {
        let title = format!("{} {}", emoji, label);
        let _ = tray.set_tooltip(Some(&title));
        let _ = tray.set_title(Some(emoji));
        if let Ok(menu) = build_tray_menu(app, &title) {
            let _ = tray.set_menu(Some(menu));
        }
    }
}

// ── IPC Commands ───────────────────────────────────────────────────────────

#[tauri::command]
fn get_status(state: State<AppState>) -> serde_json::Value {
    let ws  = state.work_state.lock().unwrap();
    let cfg = state.config.lock().unwrap();
    let actions = available_actions(&ws.current_state);
    serde_json::json!({
        "state":              ws.current_state,
        "emoji":              ws.current_state.emoji(),
        "label":              ws.current_state.label(),
        "start_time_ms":      ws.start_time_ms,
        "total_worked_ms":    ws.total_worked_ms,
        "finished_for_today": ws.finished_for_today,
        "dry_run":            cfg.dry_run,
        "available_actions":  actions,
    })
}

#[tauri::command]
async fn perform_action(
    action_label: String,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let cfg     = state.config.lock().unwrap().clone();
    let dry_run = cfg.dry_run;

    let mut ws = state.work_state.lock().unwrap();
    let valid  = available_actions(&ws.current_state);
    let action = valid.into_iter()
        .find(|a| a.label == action_label)
        .ok_or_else(|| format!("Unknown action: {}", action_label))?;

    let result = apply_transition(&mut ws, &action, dry_run)?;

    // Persist state + update tray
    persistence::save_state(&app, &ws);
    update_tray(&app, ws.current_state.emoji(), ws.current_state.label());

    // ZeusX dispatch
    let zeus_result = if let Some(key) = &action.zeusX_action {
        if let Some(zeus_action) = action_from_key(key) {
            Some(dispatch(zeus_action, dry_run).await)
        } else { None }
    } else { None };

    Ok(serde_json::json!({
        "transition": result,
        "zeusX": zeus_result.map(|r| match r {
            Ok(d)  => serde_json::json!(d),
            Err(e) => serde_json::json!({ "error": e }),
        }),
    }))
}

#[tauri::command]
fn get_config(state: State<AppState>) -> UserConfig {
    state.config.lock().unwrap().clone()
}

#[tauri::command]
fn set_config(config: UserConfig, app: AppHandle, state: State<AppState>) -> Result<(), String> {
    persistence::save_config(&app, &config);
    *state.config.lock().unwrap() = config;
    Ok(())
}

#[tauri::command]
fn set_dry_run(enabled: bool, state: State<AppState>) -> serde_json::Value {
    let mut cfg = state.config.lock().unwrap();
    cfg.dry_run = enabled;
    serde_json::json!({ "dry_run": cfg.dry_run })
}

// ── App Setup ──────────────────────────────────────────────────────────────

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Load persisted state + config
            let work_state = persistence::load_state(app.handle());
            let config     = persistence::load_config(app.handle());

            let emoji = work_state.current_state.emoji().to_string();
            let label = work_state.current_state.label().to_string();

            app.manage(AppState {
                work_state: Mutex::new(work_state),
                config:     Mutex::new(config),
            });

            // Build systray (US-004, US-022)
            let menu = build_tray_menu(app.handle(), &format!("{} {}", emoji, label))?;
            TrayIconBuilder::with_id("main")
                .menu(&menu)
                .tooltip(format!("{} {}", emoji, label))
                .title(&emoji)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(win) = app.get_webview_window("main") {
                            let _ = win.show();
                            let _ = win.set_focus();
                        }
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event {
                        let app = tray.app_handle();
                        if let Some(win) = app.get_webview_window("main") {
                            let _ = win.show();
                            let _ = win.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_status,
            perform_action,
            get_config,
            set_config,
            set_dry_run,
        ])
        .run(tauri::generate_context!())
        .expect("error while running HermesX");
}
