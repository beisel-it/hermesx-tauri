// zeusX/mod.rs — ZeusX automation scaffold
//
// ARCHITECTURE:
//   All ZeusX calls go through `dispatch()`.
//   dry_run=true → log what *would* happen, never touch ZeusX.
//   dry_run=false → invoke the Node.js sidecar (Playwright).
//
// WORK PACKAGE: WILBUR-20260313-015
//   The sidecar implementation is a separate WP.
//   This module defines the interface contract.

pub mod selectors;

use serde::{Deserialize, Serialize};

/// All supported ZeusX terminal button actions.
/// Maps 1:1 to selectors::TERMINAL_BUTTONS keys.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ZeusXAction {
    MobilesArbeitenStart,
    MobilesArbeitenEnd,
    PauseMobil,
    In,
    Out,
    InOut,
}

impl ZeusXAction {
    pub fn selector_key(&self) -> &'static str {
        match self {
            ZeusXAction::MobilesArbeitenStart => "mobiles-arbeiten-start",
            ZeusXAction::MobilesArbeitenEnd   => "mobiles-arbeiten-end",
            ZeusXAction::PauseMobil           => "pause-mobil",
            ZeusXAction::In                   => "in",
            ZeusXAction::Out                  => "out",
            ZeusXAction::InOut                => "in-out",
        }
    }

    pub fn human_label(&self) -> &'static str {
        match self {
            ZeusXAction::MobilesArbeitenStart => "Mobiles Arbeiten starten",
            ZeusXAction::MobilesArbeitenEnd   => "Mobiles Arbeiten beenden",
            ZeusXAction::PauseMobil           => "Pause (Mobiles Arbeiten)",
            ZeusXAction::In                   => "Kommen (IN)",
            ZeusXAction::Out                  => "Gehen (OUT)",
            ZeusXAction::InOut                => "IN / OUT Toggle",
        }
    }
}

/// Result returned to callers — same shape for dry-run and real runs.
#[derive(Debug, Serialize, Deserialize)]
pub struct DispatchResult {
    pub action: ZeusXAction,
    pub dry_run: bool,
    pub success: bool,
    pub message: String,
}

/// Central dispatch point.
///
/// In dry-run mode: logs intent, returns Ok immediately, never spawns sidecar.
/// In real mode: delegates to sidecar (WILBUR-20260313-015, not yet implemented).
pub async fn dispatch(action: ZeusXAction, dry_run: bool) -> Result<DispatchResult, String> {
    let label = action.human_label();
    let key = action.selector_key();

    if dry_run {
        let msg = format!(
            "[DRY RUN] Would click '{}' (selector key: '{}')",
            label, key
        );
        log::info!("{}", msg);
        return Ok(DispatchResult {
            action,
            dry_run: true,
            success: true,
            message: msg,
        });
    }

    // TODO (WILBUR-20260313-015): spawn Node.js sidecar, pass action key, await result
    Err(format!(
        "ZeusX real dispatch not yet implemented. \
         Use dry_run=true or wait for WILBUR-20260313-015. Action: {}",
        label
    ))
}

/// Convenience: parse a state_machine zeusX_action string into a ZeusXAction.
pub fn action_from_key(key: &str) -> Option<ZeusXAction> {
    match key {
        "mobiles-arbeiten-start" => Some(ZeusXAction::MobilesArbeitenStart),
        "mobiles-arbeiten-end"   => Some(ZeusXAction::MobilesArbeitenEnd),
        "pause-mobil"            => Some(ZeusXAction::PauseMobil),
        "in"                     => Some(ZeusXAction::In),
        "out"                    => Some(ZeusXAction::Out),
        "in-out"                 => Some(ZeusXAction::InOut),
        _                        => None,
    }
}
