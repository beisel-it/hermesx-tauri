//! Windows screen lock detection via WTSQuerySessionInformation polling (5s).

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
    use windows::core::PWSTR;
    use windows::Win32::System::RemoteDesktop::{
        WTSFreeMemory, WTSQuerySessionInformationW, WTSSessionInfoEx,
        WTS_CURRENT_SERVER_HANDLE, WTS_CURRENT_SESSION,
    };

    let mut buffer = PWSTR::null();
    let mut bytes_returned: u32 = 0;

    WTSQuerySessionInformationW(
        WTS_CURRENT_SERVER_HANDLE,
        WTS_CURRENT_SESSION,
        WTSSessionInfoEx,
        &mut buffer,
        &mut bytes_returned,
    ).ok()?;

    if buffer.is_null() {
        return None;
    }

    // WTSINFOEXW: Level (u32 at offset 0), then Data union
    // WTSINFOEX_LEVEL1_W.SessionFlags is i32 at offset 4
    let ptr = buffer.as_ptr() as *const u8;
    let session_flags = *(ptr.add(4) as *const i32);

    WTSFreeMemory(buffer.as_ptr() as *mut _);

    // WTS_SESSIONSTATE_LOCK = 0
    Some(session_flags == 0)
}
