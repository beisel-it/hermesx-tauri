// notification.rs — Notification cooldown + suppression
// Migrated from: src/NotificationManager.ts

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

/// Per-event-type cooldown tracking + global suppression flags.
#[derive(Debug)]
pub struct NotificationManager {
    /// Last notification time per event type key (ms epoch)
    last_notified: HashMap<String, i64>,
    /// Default cooldown between same-type notifications (ms)
    default_cooldown_ms: i64,
    /// Per-type cooldown overrides
    cooldown_overrides: HashMap<String, i64>,
    /// Global quiet mode — suppresses all notifications
    pub quiet_mode: bool,
    /// Suppress during calls (mic active / video app running)
    pub suppress_during_calls: bool,
    /// Suppress during gaming (fullscreen)
    pub suppress_during_gaming: bool,
}

impl Default for NotificationManager {
    fn default() -> Self {
        Self {
            last_notified: HashMap::new(),
            default_cooldown_ms: 5 * 60 * 1000, // 5 min
            cooldown_overrides: HashMap::new(),
            quiet_mode: false,
            suppress_during_calls: true,
            suppress_during_gaming: true,
        }
    }
}

impl NotificationManager {
    pub fn new(default_cooldown_ms: i64) -> Self {
        Self {
            default_cooldown_ms,
            ..Default::default()
        }
    }

    /// Set a per-type cooldown override (e.g. overtime reminder every 30min).
    pub fn set_cooldown(&mut self, event_type: &str, cooldown_ms: i64) {
        self.cooldown_overrides.insert(event_type.to_string(), cooldown_ms);
    }

    /// Check if a notification of this type can fire right now.
    /// Returns false if: quiet_mode, or cooldown not expired yet.
    /// Does NOT record the notification — call `mark_notified` separately.
    pub fn should_notify(&self, event_type: &str) -> bool {
        if self.is_suppressed() {
            return false;
        }
        let cooldown = self.cooldown_overrides
            .get(event_type)
            .copied()
            .unwrap_or(self.default_cooldown_ms);

        match self.last_notified.get(event_type) {
            None => true,
            Some(&last) => now_ms() - last >= cooldown,
        }
    }

    /// Record that a notification was sent for this event type.
    pub fn mark_notified(&mut self, event_type: &str) {
        self.last_notified.insert(event_type.to_string(), now_ms());
    }

    /// Convenience: check + mark in one call.
    /// Returns true if the notification should fire (and marks it).
    pub fn notify_if_ready(&mut self, event_type: &str) -> bool {
        if self.should_notify(event_type) {
            self.mark_notified(event_type);
            true
        } else {
            false
        }
    }

    /// Whether all notifications are currently suppressed.
    pub fn is_suppressed(&self) -> bool {
        self.quiet_mode
    }

    /// Reset cooldown for a specific event type (e.g. after state change).
    pub fn reset(&mut self, event_type: &str) {
        self.last_notified.remove(event_type);
    }

    /// Reset all cooldowns (e.g. start of new workday).
    pub fn reset_all(&mut self) {
        self.last_notified.clear();
    }

    /// Time remaining until next notification of this type is allowed (ms).
    /// Returns 0 if ready now.
    pub fn cooldown_remaining_ms(&self, event_type: &str) -> i64 {
        let cooldown = self.cooldown_overrides
            .get(event_type)
            .copied()
            .unwrap_or(self.default_cooldown_ms);

        match self.last_notified.get(event_type) {
            None => 0,
            Some(&last) => {
                let elapsed = now_ms() - last;
                (cooldown - elapsed).max(0)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_notification_always_fires() {
        let mgr = NotificationManager::default();
        assert!(mgr.should_notify("inactivity"));
    }

    #[test]
    fn second_notification_blocked_during_cooldown() {
        let mut mgr = NotificationManager::new(60_000); // 1min cooldown
        mgr.mark_notified("inactivity");
        assert!(!mgr.should_notify("inactivity"));
    }

    #[test]
    fn different_event_types_independent() {
        let mut mgr = NotificationManager::new(60_000);
        mgr.mark_notified("inactivity");
        assert!(mgr.should_notify("break-reminder")); // different type → ready
    }

    #[test]
    fn quiet_mode_suppresses_all() {
        let mut mgr = NotificationManager::default();
        mgr.quiet_mode = true;
        assert!(!mgr.should_notify("anything"));
    }

    #[test]
    fn reset_clears_cooldown() {
        let mut mgr = NotificationManager::new(60_000);
        mgr.mark_notified("inactivity");
        assert!(!mgr.should_notify("inactivity"));
        mgr.reset("inactivity");
        assert!(mgr.should_notify("inactivity"));
    }

    #[test]
    fn notify_if_ready_marks_on_fire() {
        let mut mgr = NotificationManager::new(60_000);
        assert!(mgr.notify_if_ready("overtime"));    // fires + marks
        assert!(!mgr.notify_if_ready("overtime"));   // blocked
    }

    #[test]
    fn cooldown_override_respected() {
        let mut mgr = NotificationManager::default();
        mgr.set_cooldown("overtime", 1_800_000); // 30min override
        mgr.mark_notified("overtime");
        let remaining = mgr.cooldown_remaining_ms("overtime");
        assert!(remaining > 1_000); // well above 0
    }

    #[test]
    fn reset_all_clears_everything() {
        let mut mgr = NotificationManager::new(60_000);
        mgr.mark_notified("a");
        mgr.mark_notified("b");
        mgr.reset_all();
        assert!(mgr.should_notify("a"));
        assert!(mgr.should_notify("b"));
    }
}
