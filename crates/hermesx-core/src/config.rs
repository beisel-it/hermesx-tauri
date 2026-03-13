// config.rs — UserConfig + defaults
// Migrated from: src/ConfigManager.ts

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartTime {
    pub hour: u8,
    pub minute: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkSchedule {
    pub start_time: StartTime,
    pub work_duration: f32,  // hours
    pub break_duration: u32, // minutes
    pub is_flexible: bool,
    pub workdays: [bool; 7], // [Sun, Mon, Tue, Wed, Thu, Fri, Sat]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPreferences {
    pub quiet_mode: bool,
    pub smart_flexibility: bool,
    pub work_mode_detection: bool,
    pub suppress_during_calls: bool,
    pub suppress_during_gaming: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InactivityThresholds {
    pub short_break_reminder: u32,  // minutes (default 240 = 4h)
    pub auto_break_suggestion: u32, // minutes idle (default 15)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    pub schedule: WorkSchedule,
    pub notifications: NotificationPreferences,
    pub inactivity: InactivityThresholds,
    pub debug: bool,
    pub dry_run: bool, // First-class: never touch ZeusX when true
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            schedule: WorkSchedule {
                start_time: StartTime {
                    hour: 8,
                    minute: 30,
                },
                work_duration: 8.0,
                break_duration: 30,
                is_flexible: true,
                workdays: [false, true, true, true, true, true, false],
            },
            notifications: NotificationPreferences {
                quiet_mode: false,
                smart_flexibility: true,
                work_mode_detection: true,
                suppress_during_calls: true,
                suppress_during_gaming: true,
            },
            inactivity: InactivityThresholds {
                short_break_reminder: 240,
                auto_break_suggestion: 15,
            },
            debug: false,
            dry_run: false,
        }
    }
}
