//! ZeusX dispatcher — spawns the Node.js sidecar and communicates via JSON stdio.
//!
//! Protocol (per line, JSON):
//!   →  { "id": string, "action": string, "dry_run"?: bool, "credentials"?: {...} }
//!   ←  { "id": string, "ok": bool, "result"?: string, "error"?: string }


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
        "dienstgang"             => Some(ZeusAction { key: key.to_string() }),
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
fn find_node() -> String {
    // macOS apps often have a stripped PATH; check common node locations
    let candidates = [
        "/opt/homebrew/bin/node",   // Apple Silicon homebrew
        "/usr/local/bin/node",      // Intel homebrew / nvm default
        "/usr/bin/node",
    ];
    for c in &candidates {
        if std::path::Path::new(c).exists() {
            return c.to_string();
        }
    }
    "node".to_string() // fall back to PATH
}

fn sidecar_cmd() -> Command {
    // Dev: CARGO_MANIFEST_DIR = src-tauri/, parent = workspace root
    // Production: zeus-sidecar.js bundled alongside binary
    let dev_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace root")
        .join("src-sidecar/dist/index.js");

    let script = if dev_path.exists() {
        dev_path
    } else {
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.join("zeus-sidecar.js")))
            .unwrap_or(dev_path)
    };

    let mut cmd = Command::new(find_node());
    cmd.arg(script);
    cmd
}

/// Dispatch a ZeusX action via the Node.js sidecar.
/// Returns the sidecar response or an error string.
pub async fn dispatch(action: ZeusAction, dry_run: bool, creds: Option<crate::credentials::StoredCredentials>) -> Result<SidecarResponse, String> {

    let req = SidecarRequest {
        id:          Uuid::new_v4().to_string(),
        action:      &action.key,
        dry_run,
        credentials: creds.map(|c| SidecarCredentials { username: c.username, password: c.password }),
    };

    let json = serde_json::to_string(&req).map_err(|e| e.to_string())?;

    // Spawn sidecar, write request, read response — one-shot per call.
    // TODO (US-015): Keep sidecar process alive for session reuse.
    let mut cmd = sidecar_cmd();
    let script_path = format!("{:?}", cmd.get_args().next().unwrap_or(std::ffi::OsStr::new("?")));
    let node_bin   = format!("{:?}", cmd.get_program());

    let mut child = cmd
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("spawn failed — node={node_bin} script={script_path}: {e}"))?;

    {
        let stdin = child.stdin.as_mut().ok_or("no stdin")?;
        stdin.write_all(format!("{json}\n").as_bytes())
            .map_err(|e| e.to_string())?;
    }

    let output = child.wait_with_output().map_err(|e| e.to_string())?;

    if !output.status.success() || output.stdout.is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "sidecar error (node={node_bin} script={script_path} exit={:?}): {}",
            output.status.code(),
            if stderr.is_empty() { "no stdout" } else { stderr.trim() }
        ));
    }

    let line = output.stdout
        .lines()
        .next()
        .and_then(|l| l.ok())
        .ok_or_else(|| "no response from sidecar".to_string())?;

    serde_json::from_str(&line).map_err(|e| format!("bad sidecar response: {e}"))
}


