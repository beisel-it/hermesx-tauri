//! Windows screen lock detection.
//! Called from main.rs setup() with the main WebviewWindow.
//! Registers WTSRegisterSessionNotification on the HWND.
//! The actual WM_WTSSESSION_CHANGE handling requires WNDPROC subclassing
//! which is complex — for now we use a polling fallback via WTSQuerySessionInformation.

use tauri::{AppHandle, Emitter};

pub fn start(app: AppHandle) {
    std::thread::spawn(move || {
        let mut was_locked = false;
        loop {
            let locked = is_session_locked();
            if locked != was_locked {
                was_locked = locked;
                let _ = app.emit("screen-lock", serde_json::json!({ "locked": locked }));
            }
            std::thread::sleep(std::time::Duration::from_secs(5));
        }
    });
}

fn is_session_locked() -> bool {
    use windows::Win32::System::RemoteDesktop::{
        WTSQuerySessionInformationW, WTSFreeMemory, WTSSessionInfoEx,
        WTS_CURRENT_SERVER_HANDLE, WTS_CURRENT_SESSION,
    };
    use windows::Win32::Foundation::BOOL;

    unsafe {
        let mut buffer: *mut u8 = std::ptr::null_mut();
        let mut bytes_returned: u32 = 0;

        // WTSQuerySessionInformation with WTSSessionInfoEx returns WTSINFOEXW
        // which contains SessionFlags with WTS_SESSIONSTATE_LOCK = 0
        let ok = WTSQuerySessionInformationW(
            WTS_CURRENT_SERVER_HANDLE,
            WTS_CURRENT_SESSION,
            WTSSessionInfoEx,
            &mut buffer as *mut _ as _,
            &mut bytes_returned,
        );

        if !ok.as_bool() || buffer.is_null() {
            return false;
        }

        // WTSINFOEX_LEVEL1_W.SessionFlags: 0=locked, 1=unlocked, 4=unknown
        // Offset: DWORD(Level=4) + DWORD(SessionFlags=8) ... structure layout
        // Level field at offset 0 (u32), then WTSINFOEX_LEVEL1_W at offset 4
        // SessionFlags is the first field of WTSINFOEX_LEVEL1_W
        let ptr = buffer as *const u32;
        let _level = *ptr; // should be 1
        let flags = *ptr.add(1); // SessionFlags
        WTSFreeMemory(buffer as _);

        flags == 0 // 0 = WTS_SESSIONSTATE_LOCK
    }
}
