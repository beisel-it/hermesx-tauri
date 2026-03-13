// work_monitor.rs — Activity + Schedule monitoring
// Migrated from: src/WorkMonitor.ts

use std::time::{SystemTime, UNIX_EPOCH};

/// Milliseconds since UNIX epoch.
pub fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

/// Reasons the monitor may want to fire a notification.
#[derive(Debug, Clone, PartialEq)]
pub enum MonitorEvent {
    /// User has been idle longer than threshold while Working
    InactivityDetected { idle_ms: u64 },
    /// Continuous work exceeds short-break reminder threshold
    ShortBreakReminder { worked_ms: u64 },
    /// Break has run past expected return time
    BreakOverrun { overrun_ms: u64 },
    /// Past scheduled start, still NOT_WORKING
    MorningReminder { minutes_late: u32 },
    /// Past scheduled start + 30min, still NOT_WORKING
    LateStartEscalation { minutes_late: u32 },
    /// Reached scheduled end time while WORKING
    EndOfDay { minutes_over: u32 },
    /// 30+ minutes past scheduled end
    Overtime { minutes_over: u32 },
    /// System lock/suspend detected while WORKING
    LockScreenWhileWorking,
}

/// Immutable snapshot of the state the monitor needs to evaluate.
/// Passed in by the Tauri command layer — no internal state here.
#[derive(Debug)]
pub struct MonitorContext {
    pub is_working: bool,
    pub is_paused: bool,
    pub is_finished_for_today: bool,
    pub is_workday: bool,
    pub idle_ms: u64,
    pub continuous_work_start_ms: Option<i64>,
    pub expected_break_return_ms: Option<i64>,
    pub scheduled_start_ms: i64,
    pub scheduled_end_ms: i64,
    /// Minimum ms between repeated notifications of the same type (default: 5 min)
    pub last_notified: std::collections::HashMap<String, i64>,
    pub inactivity_threshold_ms: u64,  // from config
    pub short_break_threshold_ms: u64, // from config
}

const COOLDOWN_MS: i64 = 5 * 60 * 1000;

impl MonitorContext {
    fn should_notify(&self, key: &str) -> bool {
        let now = now_ms();
        self.last_notified
            .get(key)
            .map(|&last| now - last > COOLDOWN_MS)
            .unwrap_or(true)
    }
}

/// Evaluate all monitor conditions and return any events that should fire.
/// Pure function — no timers, no side effects, fully testable.
pub fn evaluate(ctx: &MonitorContext) -> Vec<MonitorEvent> {
    let mut events = Vec::new();
    let now = now_ms();

    if ctx.is_finished_for_today || !ctx.is_workday {
        return events;
    }

    // --- WORKING state checks ---
    if ctx.is_working {
        // Inactivity
        if ctx.idle_ms >= ctx.inactivity_threshold_ms && ctx.should_notify("inactivity") {
            events.push(MonitorEvent::InactivityDetected {
                idle_ms: ctx.idle_ms,
            });
        }

        // Continuous work → break reminder
        if let Some(start) = ctx.continuous_work_start_ms {
            let worked = (now - start) as u64;
            if worked >= ctx.short_break_threshold_ms && ctx.should_notify("break-reminder") {
                events.push(MonitorEvent::ShortBreakReminder { worked_ms: worked });
            }
        }

        // End-of-day / overtime
        let minutes_over = ((now - ctx.scheduled_end_ms) / 60_000) as u32;
        if now > ctx.scheduled_end_ms {
            if minutes_over < 30 && ctx.should_notify("end-of-day") {
                events.push(MonitorEvent::EndOfDay { minutes_over });
            } else if minutes_over >= 30 && ctx.should_notify("overtime") {
                events.push(MonitorEvent::Overtime { minutes_over });
            }
        }
    }

    // --- PAUSED state checks ---
    if ctx.is_paused {
        if let Some(expected) = ctx.expected_break_return_ms {
            if now > expected && ctx.should_notify("break-overrun") {
                let overrun = (now - expected) as u64;
                events.push(MonitorEvent::BreakOverrun {
                    overrun_ms: overrun,
                });
            }
        }
    }

    // --- NOT_WORKING (not paused, not finished) ---
    if !ctx.is_working && !ctx.is_paused {
        let minutes_late = ((now - ctx.scheduled_start_ms) / 60_000) as u32;
        if now > ctx.scheduled_start_ms {
            if (15..30).contains(&minutes_late) && ctx.should_notify("morning-reminder") {
                events.push(MonitorEvent::MorningReminder { minutes_late });
            } else if minutes_late >= 30 && ctx.should_notify("late-start") {
                events.push(MonitorEvent::LateStartEscalation { minutes_late });
            }
        }
    }

    events
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_ctx() -> MonitorContext {
        MonitorContext {
            is_working: true,
            is_paused: false,
            is_finished_for_today: false,
            is_workday: true,
            idle_ms: 0,
            continuous_work_start_ms: None,
            expected_break_return_ms: None,
            scheduled_start_ms: now_ms() - 2 * 3600_000, // started 2h ago
            scheduled_end_ms: now_ms() + 6 * 3600_000,   // ends in 6h
            last_notified: Default::default(),
            inactivity_threshold_ms: 15 * 60 * 1000,
            short_break_threshold_ms: 4 * 3600_000,
        }
    }

    #[test]
    fn no_events_normally() {
        let ctx = base_ctx();
        assert!(evaluate(&ctx).is_empty());
    }

    #[test]
    fn inactivity_fires_when_idle_enough() {
        let mut ctx = base_ctx();
        ctx.idle_ms = 20 * 60 * 1000; // 20 min idle
        let events = evaluate(&ctx);
        assert!(events
            .iter()
            .any(|e| matches!(e, MonitorEvent::InactivityDetected { .. })));
    }

    #[test]
    fn no_events_when_finished() {
        let mut ctx = base_ctx();
        ctx.is_finished_for_today = true;
        assert!(evaluate(&ctx).is_empty());
    }

    #[test]
    fn break_overrun_fires() {
        let mut ctx = base_ctx();
        ctx.is_working = false;
        ctx.is_paused = true;
        ctx.expected_break_return_ms = Some(now_ms() - 10 * 60 * 1000); // 10min overdue
        let events = evaluate(&ctx);
        assert!(events
            .iter()
            .any(|e| matches!(e, MonitorEvent::BreakOverrun { .. })));
    }

    #[test]
    fn end_of_day_fires() {
        let mut ctx = base_ctx();
        ctx.scheduled_end_ms = now_ms() - 10 * 60 * 1000; // 10min past end
        let events = evaluate(&ctx);
        assert!(events
            .iter()
            .any(|e| matches!(e, MonitorEvent::EndOfDay { .. })));
    }
}
