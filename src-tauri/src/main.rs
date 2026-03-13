#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod credentials;
mod persistence;
mod monitor;
mod screen_lock;
mod zeus_x;

use hermesx_core::config::UserConfig;
use hermesx_core::notification::NotificationManager;
use hermesx_core::state_machine::{apply_transition, available_actions};
use zeus_x::{action_from_key, dispatch};

use std::sync::{Arc, Mutex};
use tauri_plugin_positioner::{Position, WindowExt};
use tauri::{Listener,
    AppHandle, Manager, State,
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};

struct AppState {
    notification_mgr: Arc<Mutex<NotificationManager>>,
    work_state: Mutex<hermesx_core::state_machine::PersistedState>,
    config: Mutex<UserConfig>,
    credentials: Mutex<Option<credentials::StoredCredentials>>,
}

fn build_tray_menu(app: &AppHandle, label: &str) -> tauri::Result<Menu<tauri::Wry>> {
    let status = MenuItem::with_id(app, "status", label, false, None::<&str>)?;
    let sep    = tauri::menu::PredefinedMenuItem::separator(app)?;
    let show   = MenuItem::with_id(app, "show", "Open HermesX", true, None::<&str>)?;
    let quit   = MenuItem::with_id(app, "quit", "Quit",         true, None::<&str>)?;
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

// Fix: MutexGuard darf nicht über .await gehalten werden
// Lösung: alle Daten aus dem Lock kopieren, dann Lock droppen, dann await
#[tauri::command]
async fn perform_action(
    action_label: String,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    // --- Lock scope: alles was wir brauchen rauskopieren ---
    let (action, dry_run, mut ws_snapshot) = {
        let cfg = state.config.lock().unwrap();
        let ws  = state.work_state.lock().unwrap();
        let dry_run = cfg.dry_run;

        let valid  = available_actions(&ws.current_state);
        let action = valid.into_iter()
            .find(|a| a.label == action_label)
            .ok_or_else(|| format!("Unknown action: {}", action_label))?;

        (action, dry_run, ws.clone())
    }; // Lock wird hier gedroppt

    // --- Transition auf geklontem State (kein Lock mehr) ---
    let result = apply_transition(&mut ws_snapshot, &action, dry_run)?;

    // --- ZeusX dispatch (async, kein Lock gehalten) ---
    let zeus_result = if let Some(key) = &action.zeusX_action {
        if let Some(zeus_action) = action_from_key(key) {
            {
                let creds = state.credentials.lock().unwrap().clone();
                Some(dispatch(zeus_action, dry_run, creds).await)
            }
        } else { None }
    } else { None };

    // --- State zurückschreiben ---
    {
        let mut ws = state.work_state.lock().unwrap();
        *ws = ws_snapshot;
        persistence::save_state(&app, &ws);
        update_tray(&app, ws.current_state.emoji(), ws.current_state.label());
    }

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

#[tauri::command]
fn save_credentials(username: String, password: String) -> Result<(), String> {
    credentials::save_credentials(&username, &password)
}

#[tauri::command]
fn load_credentials_status() -> serde_json::Value {
    match credentials::load_credentials() {
        Some(c) => serde_json::json!({ "stored": true, "username": c.username }),
        None    => serde_json::json!({ "stored": false }),
    }
}

#[tauri::command]
fn delete_credentials() -> Result<(), String> {
    credentials::delete_credentials()
}

#[tauri::command]
async fn perform_manual_action(
    action_key: String,
    _app: AppHandle,
    state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let dry_run = state.config.lock().unwrap().dry_run;
    let zeus_action = zeus_x::action_from_key(&action_key)
        .ok_or_else(|| format!("Unknown zeus action: {}", action_key))?;

    let creds = state.credentials.lock().unwrap().clone();
    let result = zeus_x::dispatch(zeus_action, dry_run, creds).await;
    Ok(serde_json::json!({
        "action": action_key,
        "dry_run": dry_run,
        "result": match result {
            Ok(r) => serde_json::json!(r),
            Err(e) => serde_json::json!({ "error": e }),
        }
    }))
}


#[tauri::command]
fn hide_window(app: AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.hide();
    }
}


fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_positioner::init())
            .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let work_state = persistence::load_state(app.handle());
            let config     = persistence::load_config(app.handle());
            let emoji = work_state.current_state.emoji().to_string();
            let label = work_state.current_state.label().to_string();

            let nm = Arc::new(Mutex::new(NotificationManager::new(5 * 60 * 1000)));
            app.manage(AppState {
                work_state:       Mutex::new(work_state),
                config:           Mutex::new(config),
                notification_mgr: Arc::clone(&nm),
                credentials:      Mutex::new(credentials::load_credentials()),
            });
            monitor::spawn_monitor(app.handle().clone(), nm);
            screen_lock::start_listener(app.handle().clone());

            // React to screen-lock events
            let nm_lock = Arc::clone(&app.state::<AppState>().notification_mgr);
            let app_for_lock = app.handle().clone();
            app.handle().listen("screen-lock", move |event| {
                if let Ok(payload) = serde_json::from_str::<serde_json::Value>(event.payload()) {
                    let locked = payload.get("locked").and_then(|v| v.as_bool()).unwrap_or(false);
                    monitor::handle_screen_lock_event(&app_for_lock, locked, &nm_lock);
                }
            });

            let menu = build_tray_menu(app.handle(), &format!("{} {}", emoji, label))?;
            TrayIconBuilder::with_id("main")
                .menu(&menu)
                .tooltip(format!("{} {}", emoji, label))
                .title(&emoji)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => { if let Some(w) = app.get_webview_window("main") { let _ = w.show(); let _ = w.set_focus(); } }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    tauri_plugin_positioner::on_tray_event(tray.app_handle(), &event);
                    if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = event {
                        let app = tray.app_handle();
                        if let Some(w) = app.get_webview_window("main") {
                            // macOS: tray is top-right → position window below tray icon
                            // Windows/Linux: tray is bottom-right → BottomRight uses work area (above taskbar)
                            #[cfg(target_os = "macos")]
                            let _ = w.move_window(Position::TrayBottomCenter);
                            #[cfg(not(target_os = "macos"))]
                            let _ = w.move_window(Position::BottomRight);
                            let _ = w.show();
                            let _ = w.set_focus();
                        }
                    }
                })
                .build(app)?;
            // Hide window on close instead of quitting
            let main_win = app.get_webview_window("main").expect("main window");
            main_win.on_window_event(|event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_status, perform_action, get_config, set_config, set_dry_run,
            save_credentials, load_credentials_status, delete_credentials,
            perform_manual_action, hide_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running HermesX");
}
