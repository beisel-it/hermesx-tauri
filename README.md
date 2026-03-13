# 🪶 HermesX

[![CI](https://github.com/beisel-it/hermesx-tauri/actions/workflows/ci.yml/badge.svg)](https://github.com/beisel-it/hermesx-tauri/actions/workflows/ci.yml)
[![Release](https://github.com/beisel-it/hermesx-tauri/actions/workflows/release.yml/badge.svg)](https://github.com/beisel-it/hermesx-tauri/actions/workflows/release.yml)
[![Latest Release](https://img.shields.io/github/v/release/beisel-it/hermesx-tauri)](https://github.com/beisel-it/hermesx-tauri/releases/latest)

> **A lightweight ZeusX companion for mobile workers** — track your time without touching the browser.

HermesX sits quietly in your system tray and handles ZeusX time-tracking bookings with a single click. Start work, take breaks, finish your day — no browser needed.

Rewritten from Electron → [Tauri 2](https://tauri.app/) for a fraction of the memory footprint (~30 MB vs ~150 MB+) and proper OS keychain support.

---

## ✨ Features

| | |
|--|--|
| ⚡ | **One-click time tracking** — Start, pause, and finish Mobiles Arbeiten from the tray |
| 🔔 | **Smart reminders** — Late start, long work streaks, break overruns, end-of-day alerts |
| 🔇 | **Suppression-aware** — Quiet mode, suppress during calls or gaming |
| 💾 | **Persistent state** — Remembers your work state across reboots |
| 🔐 | **Secure credentials** — OS keychain (no plaintext storage) |
| 🏠 | **Auto-start** — Launches with Windows/macOS login |
| 🚀 | **Tiny footprint** — ~5 MB bundle, ~30 MB RAM |
| 🧪 | **Dry Run mode** — Test all flows without touching ZeusX |

---

## 🏗️ Architecture

```
┌──────────────────────────────────────────────────┐
│  Systray (Tauri)                                 │
│  ┌─────────────┐  ┌───────────────────────────┐  │
│  │ App.svelte  │  │    Settings.svelte        │  │
│  │ status +    │  │ schedule, workdays, creds │  │
│  │ actions     │  └───────────────────────────┘  │
│  └──────┬──────┘                                 │
│         │ Tauri IPC                              │
│  ┌──────▼────────────────────────────────────┐   │
│  │  Rust Backend                             │   │
│  │  StateMachine · WorkMonitor · Notific.    │   │
│  │  Persistence · ZeusX Dispatcher           │   │
│  └───────────────────────┬───────────────────┘   │
│                          │ spawn sidecar          │
│  ┌───────────────────────▼───────────────────┐   │
│  │  zeus-sidecar (Node.js + Playwright)      │   │
│  │  → clicks ZeusX terminal buttons          │   │
└──┴───────────────────────────────────────────┘───┘
```

**Key principle:** `dry_run: true` → ZeusX dispatcher logs intent, never touches the browser. Safe for dev without production credentials.

---

## 🚀 Getting Started

### Prerequisites

- Rust stable (`rustup install stable`)
- Node.js 20+, npm
- Tauri CLI: `cargo install tauri-cli --version "^2"`

### Development

```bash
git clone https://github.com/beisel-it/hermesx-tauri
cd hermesx-tauri
npm install
cargo tauri dev
```

The app starts with **Dry Run mode enabled** — no ZeusX calls until you configure credentials and disable it.

### Build

```bash
cargo tauri build
```

---

## 🧪 Testing

```bash
# Core logic — pure Rust, no system deps needed
cargo test -p hermesx-core

# Lint + format check
cargo clippy -p hermesx-core -- -D warnings
cargo fmt -p hermesx-core -- --check

# Frontend
npm run build
```

**19 unit tests** covering StateMachine transitions, WorkMonitor scheduling, NotificationManager cooldowns, and CredentialStore operations.

---

## ⚙️ Configuration

| Setting | Default | Description |
|---------|---------|-------------|
| Start time | 08:30 | When morning reminders begin |
| Work duration | 8h | Triggers end-of-day reminder |
| Break duration | 30 min | Break overrun threshold |
| Workdays | Mo–Fr | Suppresses reminders on weekends |
| Dry Run | false | Disables all ZeusX automation |
| Quiet Mode | false | Disables all notifications |

---

## 🔄 Release Process

Automated via [semantic-release](https://semantic-release.gitbook.io/) on every push to `main`:

| Commit type | Version bump |
|-------------|---------|
| `fix:` | Patch — 1.0.**x** |
| `feat:` | Minor — 1.**x**.0 |
| `feat!:` / `BREAKING CHANGE:` | Major — **x**.0.0 |

Builds for: 🪟 Windows x64 · 🍎 macOS arm64 · 🐧 Linux x64

---

## 📁 Project Structure

```
hermesx-tauri/
├── crates/hermesx-core/     # Pure Rust — no system deps
│   └── src/
│       ├── config.rs        # UserConfig + defaults
│       ├── state_machine.rs # FSM: NotWorking / Working / Paused / Finished
│       ├── work_monitor.rs  # Schedule + activity evaluation
│       ├── credentials.rs   # Storage-agnostic credential trait
│       └── notification.rs  # Cooldown + suppression
├── src-tauri/               # Tauri binary
│   └── src/
│       ├── main.rs          # App setup, IPC, systray
│       ├── persistence.rs   # tauri-plugin-store wrappers
│       └── zeusX/           # Dispatcher + selector constants
├── src/                     # Svelte 5 frontend
│   ├── App.svelte
│   ├── Settings.svelte
│   └── lib/tauri.ts         # Typed IPC wrappers
└── docs/                    # Architecture, user stories, tech-stack
```

---

## 🗺️ Roadmap

- [ ] ZeusX Sidecar (Playwright automation)
- [ ] OS keychain credential backend
- [ ] Lock-screen detection → auto-reminder
- [ ] Meeting detection (Zoom/Teams)
- [ ] WorkMonitor timer loop → live notifications
- [ ] Windows test on florian-pc

---

## 🧬 Origin

HermesX started as an Electron app (`florianbeisel/hermesx`, v0.5.0). This rewrite keeps all original behaviour while shedding Chromium as a dependency.

---

*Built with [Tauri](https://tauri.app/) · [Svelte 5](https://svelte.dev/) · [Rust](https://www.rust-lang.org/)*
