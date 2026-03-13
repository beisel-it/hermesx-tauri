//! Windows screen lock detection via WTSQuerySessionInformation polling (5s).
//! Uses WTSSessionInfoEx to read SessionFlags: 0=locked, 1=unlocked.

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
    unsafe { is_session_locked_impl().unwrap_or(false) }
}

unsafe fn is_session_locked_impl() -> Option<bool> {
    use windows::Win32::System::RemoteDesktop::{
        WTSQuerySessionInformationW, WTSFreeMemory, WTSSessionInfoEx,
        WTS_CURRENT_SERVER_HANDLE, WTS_CURRENT_SESSION,
    };

    let mut buffer: *mut u16 = std::ptr::null_mut();
    let mut bytes_returned: u32 = 0;

    WTSQuerySessionInformationW(
        WTS_CURRENT_SERVER_HANDLE,
        WTS_CURRENT_SESSION,
        WTSSessionInfoEx,
        &mut buffer,
        &mut bytes_returned,
    ).ok()?; // Returns Result<()> in windows-rs

    if buffer.is_null() {
        return None;
    }

    // WTSINFOEXW layout:
    // - Level: DWORD (u32) at offset 0
    // - Data: WTSINFOEX_LEVEL_W union at offset 4
    //   - WTSINFOEX_LEVEL1_W.SessionFlags: LONG (i32) at offset 0 within union
    // So SessionFlags is at byte offset 4 from the start of WTSINFOEXW
    let ptr = buffer as *const u8;
    let session_flags = *(ptr.add(4) as *const i32);

    WTSFreeMemory(buffer as *mut _);

    // WTS_SESSIONSTATE_LOCK = 0x00000000
    // WTS_SESSIONSTATE_UNLOCK = 0x00000001
    Some(session_flags == 0)
}
