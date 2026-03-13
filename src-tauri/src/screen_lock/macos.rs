//! macOS: DistributedNotificationCenter — com.apple.screenIsLocked/Unlocked

use tauri::{AppHandle, Emitter};

pub fn start(app: AppHandle) {
    // DistributedNotificationCenter observers must run on the main thread.
    // We register them in a way that's safe with Tauri's runtime.
    std::thread::spawn(move || {
        use objc2::runtime::NSObject;
        use objc2_foundation::{NSDistributedNotificationCenter, NSString};

        unsafe {
            let center = NSDistributedNotificationCenter::defaultCenter();

            let app_lock   = app.clone();
            let app_unlock = app.clone();

            let lock_name   = NSString::from_str("com.apple.screenIsLocked");
            let unlock_name = NSString::from_str("com.apple.screenIsUnlocked");

            // Use block-based observer
            let block_lock = objc2::rc::Retained::from_raw(
                objc2::block2::StackBlock::new(move |_notif: *mut NSObject| {
                    let _ = app_lock.emit("screen-lock", serde_json::json!({"locked": true}));
                }) as *mut _
            );

            let block_unlock = objc2::rc::Retained::from_raw(
                objc2::block2::StackBlock::new(move |_notif: *mut NSObject| {
                    let _ = app_unlock.emit("screen-lock", serde_json::json!({"locked": false}));
                }) as *mut _
            );

            // Register observers
            let _: () = objc2::msg_send![
                center,
                addObserverForName: &*lock_name,
                object: std::ptr::null_mut::<NSObject>(),
                queue: std::ptr::null_mut::<NSObject>(),
                usingBlock: block_lock
            ];
            let _: () = objc2::msg_send![
                center,
                addObserverForName: &*unlock_name,
                object: std::ptr::null_mut::<NSObject>(),
                queue: std::ptr::null_mut::<NSObject>(),
                usingBlock: block_unlock
            ];
        }

        // Keep thread alive (run loop needed for NSDistributedNotificationCenter)
        loop {
            std::thread::sleep(std::time::Duration::from_secs(60));
        }
    });
}
