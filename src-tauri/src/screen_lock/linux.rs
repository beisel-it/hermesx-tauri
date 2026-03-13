//! Linux: D-Bus ScreenSaver.ActiveChanged (GNOME/KDE/Freedesktop)

use tauri::{AppHandle, Emitter};

pub fn start(app: AppHandle) {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("tokio runtime for dbus");
        rt.block_on(async move {
            if let Err(e) = listen_dbus(app).await {
                eprintln!("[HermesX] D-Bus screen-lock listener: {e}");
            }
        });
    });
}

async fn listen_dbus(app: AppHandle) -> zbus::Result<()> {
    use zbus::Connection;
            use futures::stream::StreamExt; // futures crate, not futures_util

    let conn = Connection::session().await?;

    let rule = zbus::MatchRule::builder()
        .msg_type(zbus::message::Type::Signal)
        .interface("org.freedesktop.ScreenSaver")?
        .member("ActiveChanged")?
        .build();

    let mut stream = zbus::MessageStream::for_match_rule(rule, &conn, None).await?;

    while let Some(Ok(msg)) = stream.next().await {
        if let Ok((active,)) = msg.body().deserialize::<(bool,)>() {
            let _ = app.emit("screen-lock", serde_json::json!({ "locked": active }));
        }
    }

    Ok(())
}
