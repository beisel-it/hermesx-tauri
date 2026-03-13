//! Background monitor loop — evaluates work schedule every 60s
//! and fires OS notifications when warranted.

use crate::AppState;
use hermesx_core::config::UserConfig;
use hermesx_core::notification::NotificationManager;
use hermesx_core::state_machine::{PersistedState, WorkState};
use hermesx_core::work_monitor::{evaluate, MonitorContext, MonitorEvent};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{AppHandle, Manager};
use tauri_plugin_notification::NotificationExt;

pub fn spawn_monitor(app: AppHandle, nm: Arc<Mutex<NotificationManager>>) {
    std::thread::spawn(move || loop {
        std::thread::sleep(Duration::from_secs(60));
        tick(&app, &nm);
    });
}

fn build_context(ws: &PersistedState, cfg: &UserConfig) -> MonitorContext {
    use hermesx_core::work_monitor::now_ms;

    let now_ts   = now_ms();
    let midnight = now_ts - (now_ts % 86_400_000);

    let start_ms = midnight
        + (cfg.schedule.start_time.hour as i64 * 3600
            + cfg.schedule.start_time.minute as i64 * 60)
            * 1000;
    let end_ms = start_ms + (cfg.schedule.work_duration * 3_600_000.0) as i64;

    // workdays: [Sun, Mon, Tue, Wed, Thu, Fri, Sat]
    let day_idx = {
        use chrono::Datelike;
        chrono::Local::now().weekday().num_days_from_sunday() as usize
    };

    MonitorContext {
        is_working:               matches!(ws.current_state, WorkState::Working),
        is_paused:                matches!(ws.current_state, WorkState::Paused),
        is_finished_for_today:    ws.finished_for_today,
        is_workday:               cfg.schedule.workdays[day_idx],
        idle_ms:                  0,
        continuous_work_start_ms: ws.start_time_ms,
        expected_break_return_ms: None,
        scheduled_start_ms:       start_ms,
        scheduled_end_ms:         end_ms,
        last_notified:            HashMap::new(),
        inactivity_threshold_ms:  (cfg.inactivity.auto_break_suggestion as u64) * 60_000,
        short_break_threshold_ms: (cfg.inactivity.short_break_reminder as u64) * 60_000,
    }
}

fn event_to_notification(event: &MonitorEvent) -> (&'static str, String) {
    match event {
        MonitorEvent::MorningReminder { minutes_late } => (
            "⏰ Noch nicht eingebucht",
            format!("{} Minuten nach geplantem Start.", minutes_late),
        ),
        MonitorEvent::LateStartEscalation { minutes_late } => (
            "⚠️ Verspäteter Start",
            format!("Über {} Minuten nach Arbeitsbeginn.", minutes_late),
        ),
        MonitorEvent::ShortBreakReminder { worked_ms } => {
            let h = worked_ms / 3_600_000;
            let m = (worked_ms % 3_600_000) / 60_000;
            ("☕ Pause fällig", format!("Durchgehend {}h{}m gearbeitet.", h, m))
        }
        MonitorEvent::BreakOverrun { overrun_ms } => {
            let m = overrun_ms / 60_000;
            ("🔔 Pause überzogen", format!("Bereits {} Minuten zu lang.", m))
        }
        MonitorEvent::EndOfDay { minutes_over } => (
            "🏁 Feierabend",
            format!("Planmäßiges Ende vor {} Minuten.", minutes_over),
        ),
        MonitorEvent::Overtime { minutes_over } => (
            "😮 Überstunden",
            format!("Bereits {} Minuten über Planzeit.", minutes_over),
        ),
        MonitorEvent::InactivityDetected { .. } => {
            ("💤 Inaktiv", "Noch eingecheckt — alles okay?".into())
        }
        MonitorEvent::LockScreenWhileWorking => {
            ("🔒 Bildschirm gesperrt", "Du bist noch eingecheckt.".into())
        }
    }
}

fn event_key(event: &MonitorEvent) -> &'static str {
    match event {
        MonitorEvent::InactivityDetected { .. }  => "inactivity",
        MonitorEvent::ShortBreakReminder { .. }  => "short_break",
        MonitorEvent::BreakOverrun { .. }        => "break_overrun",
        MonitorEvent::MorningReminder { .. }     => "morning",
        MonitorEvent::LateStartEscalation { .. } => "late_start",
        MonitorEvent::EndOfDay { .. }            => "end_of_day",
        MonitorEvent::Overtime { .. }            => "overtime",
        MonitorEvent::LockScreenWhileWorking     => "lock_screen",
    }
}

fn tick(app: &AppHandle, nm: &Arc<Mutex<NotificationManager>>) {
    let state = app.state::<AppState>();

    let (ws, cfg) = {
        let ws  = state.work_state.lock().unwrap().clone();
        let cfg = state.config.lock().unwrap().clone();
        (ws, cfg)
    };

    if cfg.notifications.quiet_mode {
        return;
    }

    let ctx    = build_context(&ws, &cfg);
    let events = evaluate(&ctx);

    for event in &events {
        let key = event_key(event);
        let mut nm_guard = nm.lock().unwrap();
        if nm_guard.notify_if_ready(key) {
            drop(nm_guard);
            let (title, body) = event_to_notification(event);
            let _ = app.notification().builder().title(title).body(body).show();
        }
    }
}
