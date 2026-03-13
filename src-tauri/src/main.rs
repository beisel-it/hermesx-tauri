// HermesX Tauri — main entry point
// Migration from: https://github.com/florianbeisel/hermesx

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod state_machine;
mod work_monitor;
mod config;
mod zeusX;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
