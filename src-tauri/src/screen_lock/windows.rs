//! Windows screen lock detection.
//! TODO: Implement via WTSQuerySessionInformation once testable on florian-pc.
//! Stub emits nothing — app works without screen-lock events on Windows.

use tauri::AppHandle;

pub fn start(_app: AppHandle) {
    // Stub: no-op until Windows hardware test available
}
