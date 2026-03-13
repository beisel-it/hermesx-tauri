//! Linux: D-Bus ScreenSaver.ActiveChanged (GNOME/KDE/Freedesktop)
//! Falls back to org.freedesktop.login1 Lock/Unlock signals.

use tauri::{AppHandle, Emitter};

pub fn start(app: AppHandle) {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("tokio runtime");

        rt.block_on(async move {
            if let Err(e) = listen_dbus(app).await {
                eprintln!("[HermesX] D-Bus lock listener error: {e}");
            }
        });
    });
}

async fn listen_dbus(app: AppHandle) -> zbus::Result<()> {
    use futures_util::stream::StreamExt;
    use zbus::Connection;

    let conn = Connection::session().await?;

    // Try org.freedesktop.ScreenSaver first (GNOME + KDE)
    let rule = zbus::MatchRule::builder()
        .msg_type(zbus::message::Type::Signal)
        .interface("org.freedesktop.ScreenSaver")?
        .member("ActiveChanged")?
        .build();

    let mut stream = zbus::MessageStream::for_match_rule(rule, &conn, None).await?;

    while let Some(msg) = stream.next().await {
        if let Ok(msg) = msg {
            if let Ok((active,)) = msg.body().deserialize::<(bool,)>() {
                let _ = app.emit("screen-lock", serde_json::json!({ "locked": active }));
            }
        }
    }

    Ok(())
}
