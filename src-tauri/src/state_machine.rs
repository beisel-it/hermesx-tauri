// StateMachine — migrated from HermesX Electron
// Original: src/StateMachine.ts

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WorkState {
    NotWorking,
    Working,
    Paused,
    Finished,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersistentState {
    pub current_state: WorkState,
    pub start_time: Option<i64>,   // Unix epoch ms
    pub total_worked_time: i64,    // ms
    pub finished_for_today: bool,
}

impl Default for PersistentState {
    fn default() -> Self {
        Self {
            current_state: WorkState::NotWorking,
            start_time: None,
            total_worked_time: 0,
            finished_for_today: false,
        }
    }
}

// TODO: implement transitions, persistence, ZeusX action dispatch
