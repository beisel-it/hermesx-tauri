//! macOS screen lock detection via CGSession polling.
//!
//! Polls CGSessionCopyCurrentDictionary() every 5 seconds.
//! No Objective-C runtime, no blocks — pure C FFI via libloading.
//!
//! CGSSessionScreenIsLocked key is undocumented but stable since macOS 10.6.

use std::ffi::CString;
use std::os::raw::c_void;
use tauri::{AppHandle, Emitter};

pub fn start(app: AppHandle) {
    std::thread::spawn(move || {
        let mut was_locked = false;
        loop {
            let locked = is_screen_locked();
            if locked != was_locked {
                was_locked = locked;
                let _ = app.emit("screen-lock", serde_json::json!({ "locked": locked }));
            }
            std::thread::sleep(std::time::Duration::from_secs(5));
        }
    });
}

fn is_screen_locked() -> bool {
    unsafe { is_screen_locked_impl().unwrap_or(false) }
}

unsafe fn is_screen_locked_impl() -> Option<bool> {
    use libloading::Library;

    let cg = Library::new(
        "/System/Library/Frameworks/CoreGraphics.framework/CoreGraphics",
    ).ok()?;
    let cf = Library::new(
        "/System/Library/Frameworks/CoreFoundation.framework/CoreFoundation",
    ).ok()?;

    // CGSessionCopyCurrentDictionary() -> CFDictionaryRef (caller releases)
    type CopyDictFn = unsafe extern "C" fn() -> *mut c_void;
    let copy_dict: libloading::Symbol<CopyDictFn> =
        cg.get(b"CGSessionCopyCurrentDictionary\0").ok()?;

    // CFStringCreateWithCString(alloc, cStr, encoding) -> CFStringRef
    type CfStrFn = unsafe extern "C" fn(*const c_void, *const i8, u32) -> *mut c_void;
    let cf_str_create: libloading::Symbol<CfStrFn> =
        cf.get(b"CFStringCreateWithCString\0").ok()?;

    // CFDictionaryGetValue(dict, key) -> *const void
    type CfDictGetFn = unsafe extern "C" fn(*mut c_void, *mut c_void) -> *mut c_void;
    let cf_dict_get: libloading::Symbol<CfDictGetFn> =
        cf.get(b"CFDictionaryGetValue\0").ok()?;

    // CFBooleanGetValue(boolean) -> bool
    type CfBoolFn = unsafe extern "C" fn(*mut c_void) -> bool;
    let cf_bool_get: libloading::Symbol<CfBoolFn> =
        cf.get(b"CFBooleanGetValue\0").ok()?;

    // CFRelease(cf_type)
    type CfRelFn = unsafe extern "C" fn(*mut c_void);
    let cf_release: libloading::Symbol<CfRelFn> =
        cf.get(b"CFRelease\0").ok()?;

    let dict = copy_dict();
    if dict.is_null() {
        return None;
    }

    // kCFStringEncodingUTF8 = 0x08000100
    let key_cstr = CString::new("CGSSessionScreenIsLocked").ok()?;
    let key = cf_str_create(std::ptr::null(), key_cstr.as_ptr(), 0x0800_0100);
    if key.is_null() {
        cf_release(dict);
        return None;
    }

    let value = cf_dict_get(dict, key);
    cf_release(key);
    cf_release(dict);

    if value.is_null() {
        return Some(false);
    }

    Some(cf_bool_get(value))
}
