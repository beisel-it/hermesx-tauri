//! Screen lock detection — platform-specific implementations.
//! Emits Tauri event "screen-lock" with payload { locked: bool }.

use tauri::AppHandle;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "linux")]
mod linux;

/// Start listening for screen lock/unlock events.
/// Spawns a background thread per platform.
pub fn start_listener(app: AppHandle) {
    #[cfg(target_os = "windows")]
    windows::start(app);

    #[cfg(target_os = "macos")]
    macos::start(app);

    #[cfg(target_os = "linux")]
    linux::start(app);

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    let _ = app; // unsupported platform — no-op
}
