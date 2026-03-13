//! ZeusX dispatcher — spawns the Node.js sidecar and communicates via JSON stdio.
//!
//! Protocol (per line, JSON):
//!   →  { "id": string, "action": string, "dry_run"?: bool, "credentials"?: {...} }
//!   ←  { "id": string, "ok": bool, "result"?: string, "error"?: string }

pub mod selectors;

use crate::credentials;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, Write};
use std::process::{Command, Stdio};
use uuid::Uuid;

/// One ZeusX action the sidecar can execute.
#[derive(Debug, Clone)]
pub struct ZeusAction {
    pub key: String,
}

/// A Tauri WorkAction may reference a ZeusX action by key.
pub fn action_from_key(key: &str) -> Option<ZeusAction> {
    // Keys must match selectors.rs TERMINAL_BUTTONS keys
    match key {
        "mobiles-arbeiten-start" |
        "mobiles-arbeiten-end"   |
        "pause-mobil"            |
        "in"                     |
        "out"                    |
        "pause"                  |
        "in-out"                 |
        "bereitschaft-start"     |
        "bereitschaft-stop"      |
        "dienstgang"             => Some(ZeusAction { key: Box::leak(key.to_string().into_boxed_str()) }),
        _ => None,
    }
}

#[derive(Serialize)]
struct SidecarRequest<'a> {
    id:       String,
    action:   &'a str,
    dry_run:  bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    credentials: Option<SidecarCredentials>,
}

#[derive(Serialize, Clone)]
struct SidecarCredentials {
    username: String,
    password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SidecarResponse {
    pub id:     String,
    pub ok:     bool,
    pub result: Option<String>,
    pub error:  Option<String>,
}

/// Find the sidecar binary.
/// In dev: look for node + dist/index.js relative to cargo workspace.
/// In production: Tauri bundles the sidecar via tauri.conf.json externalBin.
fn sidecar_cmd() -> Command {
    // Production path (Tauri sidecar): tauri resolves this automatically.
    // Dev path: node + script alongside binary.
    let script = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.join("zeus-sidecar.js")))
        .unwrap_or_else(|| {
            // Fallback: relative to workspace root during dev
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .parent().unwrap()
                .join("src-sidecar/dist/index.js")
        });

    let mut cmd = Command::new("node");
    cmd.arg(script);
    cmd
}

/// Dispatch a ZeusX action via the Node.js sidecar.
/// Returns the sidecar response or an error string.
pub async fn dispatch(action: ZeusAction, dry_run: bool) -> Result<SidecarResponse, String> {
    // Credentials: load from credential store (US-019, currently empty stub)
    let creds = load_credentials();

    let req = SidecarRequest {
        id:          Uuid::new_v4().to_string(),
        action:      &action.key,
        dry_run,
        credentials: creds,
    };

    let json = serde_json::to_string(&req).map_err(|e| e.to_string())?;

    // Spawn sidecar, write request, read response — one-shot per call.
    // TODO (US-015): Keep sidecar process alive for session reuse.
    let mut child = sidecar_cmd()
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("Failed to spawn zeus-sidecar: {e}"))?;

    {
        let stdin = child.stdin.as_mut().ok_or("no stdin")?;
        stdin.write_all(format!("{json}\n").as_bytes())
            .map_err(|e| e.to_string())?;
    } // stdin closed → sidecar reads EOF, processes, exits

    let output = child.wait_with_output().map_err(|e| e.to_string())?;

    let line = output.stdout
        .lines()
        .next()
        .and_then(|l| l.ok())
        .ok_or_else(|| "no response from sidecar".to_string())?;

    serde_json::from_str(&line).map_err(|e| format!("bad sidecar response: {e}"))
}

/// Load credentials from OS keychain (stub — US-019).
fn load_credentials() -> Option<SidecarCredentials> {
    credentials::load_credentials().map(|c| SidecarCredentials {
        username: c.username,
        password: c.password,
    })
}
