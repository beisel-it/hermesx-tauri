# Tech Stack Decision

## Framework: Tauri 2.x

**Warum Tauri statt Electron:**
- Bundle-Größe: ~5MB (Tauri) vs ~150MB (Electron) — kein eingebetteter Chromium
- Memory: ~30MB vs ~150MB+ RAM footprint
- Security: Rust-Backend, kein Node.js in Main-Process, kleinere Attack Surface
- Native APIs: Tauri-Plugins für Keychain, Notifications, Autostart out-of-the-box
- Windows-Performance: Nutzt WebView2 (vorinstalliert ab Win10), kein eigenes Runtime-Bundle

## Frontend: Svelte 5

**Warum Svelte:**
- Settings-Fenster ist einfaches Form-UI — kein React/Vue Overhead nötig
- Svelte kompiliert zu vanilla JS, kein Virtual DOM, minimaler Bundle
- Gute Tauri-Integration (kein zusätzlicher State-Manager nötig)
- Alternative war React — rejected (zu schwer für diesen Use Case)

## Backend: Rust (Tauri core)

Die Kernlogik landet in Rust:
- StateMachine (States, Transitions, Persistence)
- WorkMonitor (Idle-Detection via `tauri-plugin-system-info` oder `windows-rs`)
- ZeusX-Automation (Browser-Control via Playwright Node-sidecar)

**ZeusX-Automation Sonderfall:**
Playwright läuft weiterhin als Node.js Sidecar-Prozess — Tauri kann externe Prozesse spawnen. Die Selektor-Logik aus `zeus-punch` wird direkt übernommen.

## Tauri Plugins

| Funktion | Plugin |
|---------|--------|
| Keychain | `tauri-plugin-keyring` |
| Notifications | `tauri-plugin-notification` |
| Autostart | `tauri-plugin-autostart` |
| System Idle | `tauri-plugin-system-info` oder custom Rust |
| Config persistence | `tauri-plugin-store` |
| Single instance | `tauri-plugin-single-instance` |
| Systray | Tauri built-in (`SystemTray`) |
| Shell/Sidecar | `tauri-plugin-shell` |
| Auto-updater | Tauri built-in |

## Project Structure

```
hermesx-tauri/
├── src/                    # Svelte frontend
│   ├── App.svelte
│   ├── Settings.svelte
│   └── lib/
├── src-tauri/
│   ├── src/
│   │   ├── main.rs         # Tauri app entry
│   │   ├── state_machine.rs
│   │   ├── work_monitor.rs
│   │   ├── config.rs
│   │   └── zeusX/
│   │       ├── mod.rs
│   │       ├── selectors.rs
│   │       └── automation.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
├── zeus-sidecar/           # Node.js Playwright sidecar
│   ├── package.json
│   └── src/
│       └── index.ts        # ZeusX automation (from zeus-punch)
└── docs/
```

## ZeusX Sidecar Architecture

```
Tauri (Rust)  ──IPC──►  zeus-sidecar (Node.js + Playwright)  ──►  ZeusX Browser
     ▲                                                                    │
     └──────────────────── result (success/error) ───────────────────────┘
```

Sidecar wird via `tauri-plugin-shell` gespawnt. Kommunikation: stdin/stdout JSON oder Unix Socket.

## Build Targets

- **Windows** (primary — ZeusX läuft auf Windows im INTERSPORT-Kontext)
- **macOS** (secondary — Florian's Entwicklungsmaschine)
- Linux: nice-to-have

## CI/CD

GitHub Actions auf `florianbeisel/hermesx-tauri`:
- `windows-latest` runner für Windows-Build
- Test auf `florian-pc` Node via OpenClaw runner
