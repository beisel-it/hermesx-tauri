// state_machine.rs — WorkState FSM
// Migrated from: src/StateMachine.ts

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkState {
    NotWorking,
    Working,
    Paused,
    Finished,
}

impl WorkState {
    pub fn emoji(&self) -> &'static str {
        match self {
            WorkState::NotWorking => "🏠",
            WorkState::Working => "⚡",
            WorkState::Paused => "☕",
            WorkState::Finished => "🏠",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            WorkState::NotWorking => "Not Working",
            WorkState::Working => "Working",
            WorkState::Paused => "On Break",
            WorkState::Finished => "Done for today",
        }
    }
}

/// Every available user action from a given state.
/// `zeusX_action` maps to a key in zeusX::selectors — None = no ZeusX call needed.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct WorkAction {
    pub label: String,
    pub next_state: WorkState,
    pub zeusX_action: Option<String>, // e.g. "mobiles-arbeiten-start"
}

/// Result of a state transition attempt.
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct TransitionResult {
    pub previous_state: WorkState,
    pub new_state: WorkState,
    pub elapsed_ms: Option<i64>,
    pub dry_run: bool,
    /// True if ZeusX was called (or would have been called in dry-run)
    pub zeusX_triggered: bool,
}

/// Persisted to disk via tauri-plugin-store.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedState {
    pub current_state: WorkState,
    pub start_time_ms: Option<i64>,
    pub total_worked_ms: i64,
    pub finished_for_today: bool,
}

impl Default for PersistedState {
    fn default() -> Self {
        Self {
            current_state: WorkState::NotWorking,
            start_time_ms: None,
            total_worked_ms: 0,
            finished_for_today: false,
        }
    }
}

pub fn available_actions(state: &WorkState) -> Vec<WorkAction> {
    match state {
        WorkState::NotWorking => vec![WorkAction {
            label: "Start Work".into(),
            next_state: WorkState::Working,
            zeusX_action: Some("mobiles-arbeiten-start".into()),
        }],
        WorkState::Working => vec![
            WorkAction {
                label: "Start Break".into(),
                next_state: WorkState::Paused,
                zeusX_action: Some("pause-mobil".into()),
            },
            WorkAction {
                label: "Finish Work".into(),
                next_state: WorkState::Finished,
                zeusX_action: Some("mobiles-arbeiten-end".into()),
            },
        ],
        WorkState::Paused => vec![
            WorkAction {
                label: "Return from Break".into(),
                next_state: WorkState::Working,
                zeusX_action: Some("mobiles-arbeiten-start".into()),
            },
            WorkAction {
                label: "Finish Work".into(),
                next_state: WorkState::Finished,
                zeusX_action: Some("mobiles-arbeiten-end".into()),
            },
        ],
        WorkState::Finished => vec![],
    }
}

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

/// Pure transition logic — no I/O, no ZeusX calls here.
/// Caller is responsible for persistence and ZeusX dispatch.
pub fn apply_transition(
    state: &mut PersistedState,
    action: &WorkAction,
    dry_run: bool,
) -> Result<TransitionResult, String> {
    let valid = available_actions(&state.current_state);
    if !valid.iter().any(|a| a.label == action.label) {
        return Err(format!(
            "Invalid action '{}' in state {:?}",
            action.label, state.current_state
        ));
    }

    let previous = state.current_state.clone();
    let now = now_ms();
    let elapsed = state.start_time_ms.map(|t| now - t);
    #[allow(non_snake_case)]
    let zeusX_triggered = action.zeusX_action.is_some();

    if !dry_run {
        // Update worked time
        if let Some(e) = elapsed {
            if matches!(action.next_state, WorkState::Paused | WorkState::Finished) {
                state.total_worked_ms += e;
            }
        }

        // Update timestamps
        match &action.next_state {
            WorkState::Working => {
                state.start_time_ms = Some(now);
                state.finished_for_today = false;
            }
            WorkState::Finished => {
                state.start_time_ms = None;
                state.finished_for_today = true;
                state.total_worked_ms = 0; // reset for next day
            }
            WorkState::Paused => {
                state.start_time_ms = None;
            }
            _ => {}
        }

        // FINISHED collapses back to NOT_WORKING (same as original)
        state.current_state = if action.next_state == WorkState::Finished {
            WorkState::NotWorking
        } else {
            action.next_state.clone()
        };
    }

    Ok(TransitionResult {
        previous_state: previous,
        new_state: state.current_state.clone(),
        elapsed_ms: elapsed,
        dry_run,
        zeusX_triggered,
    })
}
