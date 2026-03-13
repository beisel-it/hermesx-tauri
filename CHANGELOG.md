## [1.9.12](https://github.com/beisel-it/hermesx-tauri/compare/v1.9.11...v1.9.12) (2026-03-13)


### Bug Fixes

* **tray:** register positioner on_tray_event hook to fix TrayBottomCenter panic; suppress dead_code in selectors.rs ([12d1e1e](https://github.com/beisel-it/hermesx-tauri/commit/12d1e1eb5928ed9d4f8a6828e8d4dc8ab9291999))

## [1.9.11](https://github.com/beisel-it/hermesx-tauri/compare/v1.9.10...v1.9.11) (2026-03-13)


### Bug Fixes

* **lint:** svelte-eslint-parser with typescript, remove unused ActionResult import, any→unknown ([db2552a](https://github.com/beisel-it/hermesx-tauri/commit/db2552a32d9c3348b4bc4487096d4683da7eb1c2))
* linux build green — credentials cache in AppState, dispatch signature, positioner move_window, clean unused imports ([93c3571](https://github.com/beisel-it/hermesx-tauri/commit/93c3571040e12dd0419e3452cdcf372feea10b5a))

## [1.9.10](https://github.com/beisel-it/hermesx-tauri/compare/v1.9.9...v1.9.10) (2026-03-13)


### Bug Fixes

* **capabilities:** core:window:allow-hide + positioner:allow-move-window (correct permission keys) ([2f60a52](https://github.com/beisel-it/hermesx-tauri/commit/2f60a52047a1b9c48a31830b05a4db75efd5ffb9))

## [1.9.9](https://github.com/beisel-it/hermesx-tauri/compare/v1.9.8...v1.9.9) (2026-03-13)


### Bug Fixes

* **ci:** remove invalid positioner:default capability (plugin uses WindowExt directly, no permission key needed) ([b1807ed](https://github.com/beisel-it/hermesx-tauri/commit/b1807ed4656dc5afab9a25ce8d5e0b3c7995791e))

## [1.9.8](https://github.com/beisel-it/hermesx-tauri/compare/v1.9.7...v1.9.8) (2026-03-13)


### Bug Fixes

* **tray:** close button via hide_window IPC, tray click positions window at TrayBottomCenter via plugin-positioner ([635195e](https://github.com/beisel-it/hermesx-tauri/commit/635195e39e495dfe75f5f09d51ef42d6a67ff23d))

## [1.9.7](https://github.com/beisel-it/hermesx-tauri/compare/v1.9.6...v1.9.7) (2026-03-13)


### Bug Fixes

* **credentials:** load once at startup into AppState cache, never prompt keychain per-action again ([8d863c6](https://github.com/beisel-it/hermesx-tauri/commit/8d863c66e8e7a48dd9f4e0501f1ed219f02a2ab8))

## [1.9.6](https://github.com/beisel-it/hermesx-tauri/compare/v1.9.5...v1.9.6) (2026-03-13)


### Bug Fixes

* **sidecar:** align action keys with selectors.rs (mobiles-arbeiten-start/end, pause-mobil etc), use correct INTERSPORT terminal selectors ([51d8649](https://github.com/beisel-it/hermesx-tauri/commit/51d86490fefe438ddf5ea9437e6b223c75ac11a8))

## [1.9.5](https://github.com/beisel-it/hermesx-tauri/compare/v1.9.4...v1.9.5) (2026-03-13)


### Bug Fixes

* **core:** zeusX_action key mismatch (start_work→mobiles-arbeiten-start); stub windows screen lock until florian-pc test ([b97d604](https://github.com/beisel-it/hermesx-tauri/commit/b97d604cea8a7e6b7ab8f3e709b99a1eb5134cbd))

## [1.9.4](https://github.com/beisel-it/hermesx-tauri/compare/v1.9.3...v1.9.4) (2026-03-13)


### Bug Fixes

* **windows:** PWSTR in windows::core not Win32::Foundation, add Win32_Core feature ([18a8348](https://github.com/beisel-it/hermesx-tauri/commit/18a83483bad3b7bd62e6e7b0c5dac7d809222e7c))

## [1.9.3](https://github.com/beisel-it/hermesx-tauri/compare/v1.9.2...v1.9.3) (2026-03-13)


### Bug Fixes

* **windows:** WTSQuerySessionInformationW expects *mut PWSTR not &mut *mut u16 ([220eacd](https://github.com/beisel-it/hermesx-tauri/commit/220eacddbc81f76ddca5495d0c5722c8bcf92b08))

## [1.9.2](https://github.com/beisel-it/hermesx-tauri/compare/v1.9.1...v1.9.2) (2026-03-13)


### Bug Fixes

* **windows:** WTSQuerySessionInformationW returns Result<()> not BOOL, fix .ok()? and pointer offset ([6a4befd](https://github.com/beisel-it/hermesx-tauri/commit/6a4befd90c03ae781238e37598adc87e10638594))

## [1.9.1](https://github.com/beisel-it/hermesx-tauri/compare/v1.9.0...v1.9.1) (2026-03-13)


### Bug Fixes

* **ci:** linux futures_util->futures+StreamExt, windows polling via WTSQuerySessionInformation (no WNDPROC), remove invalid on_window_event ([9b52ffe](https://github.com/beisel-it/hermesx-tauri/commit/9b52ffe5c678ae5a77af2f07d1eefb05cdf3abf4))

# [1.9.0](https://github.com/beisel-it/hermesx-tauri/compare/v1.8.1...v1.9.0) (2026-03-13)


### Features

* **ui:** dry-run toast feedback, manual booking buttons (behind manual_mode setting), perform_manual_action IPC ([50922bb](https://github.com/beisel-it/hermesx-tauri/commit/50922bb7a7d9eeefa030502ca548698178cfbcac))

## [1.8.1](https://github.com/beisel-it/hermesx-tauri/compare/v1.8.0...v1.8.1) (2026-03-13)


### Bug Fixes

* **settings:** load config via get_config IPC (not from status), dry_run now persists correctly ([81830ce](https://github.com/beisel-it/hermesx-tauri/commit/81830ce146666eca2a1819ea065222eed2282f5a))

# [1.8.0](https://github.com/beisel-it/hermesx-tauri/compare/v1.7.3...v1.8.0) (2026-03-13)


### Bug Fixes

* **tray:** close hides window instead of quitting (prevent_close on CloseRequested) ([a14b1b0](https://github.com/beisel-it/hermesx-tauri/commit/a14b1b08ae09dc1afde3b910992265b89696cac5))


### Features

* **ui:** proper tray popup — 320×520, no decorations, custom titlebar with drag, dark design system ([598a50d](https://github.com/beisel-it/hermesx-tauri/commit/598a50dbee820e6bce76132e516a26090fed701d))

## [1.7.3](https://github.com/beisel-it/hermesx-tauri/compare/v1.7.2...v1.7.3) (2026-03-13)


### Bug Fixes

* **rust:** import tauri::Listener trait for app.handle().listen() ([c46c965](https://github.com/beisel-it/hermesx-tauri/commit/c46c965570e4f8457fc201c5ed346c46be15ac0c))

## [1.7.2](https://github.com/beisel-it/hermesx-tauri/compare/v1.7.1...v1.7.2) (2026-03-13)


### Bug Fixes

* **macos:** replace objc2 block API with CGSession polling (no FFI blocks), fix app.handle().listen() ([16a7674](https://github.com/beisel-it/hermesx-tauri/commit/16a76744790a1f9d5c6ae1f61aef33de6773d733))

## [1.7.1](https://github.com/beisel-it/hermesx-tauri/compare/v1.7.0...v1.7.1) (2026-03-13)


### Bug Fixes

* **rust:** unused BufReader, SidecarResponse missing Serialize, keyring delete_password API ([74d93e3](https://github.com/beisel-it/hermesx-tauri/commit/74d93e3fc523401ee6eb9e754d2a628df9b59020))

# [1.7.0](https://github.com/beisel-it/hermesx-tauri/compare/v1.6.1...v1.7.0) (2026-03-13)


### Features

* **us-007:** screen lock detection — Windows WTS, macOS DistributedNotificationCenter, Linux D-Bus (zbus) ([831bab8](https://github.com/beisel-it/hermesx-tauri/commit/831bab8cd62efc3e5b5d73282ddcd230e2e714bf))

## [1.6.1](https://github.com/beisel-it/hermesx-tauri/compare/v1.6.0...v1.6.1) (2026-03-13)


### Bug Fixes

* **tauri:** remove externalBin (node spawned directly, not Tauri sidecar), add shell capabilities ([3edb6be](https://github.com/beisel-it/hermesx-tauri/commit/3edb6beaf461a847f4c242bc8d727a1ec5173b67))

# [1.6.0](https://github.com/beisel-it/hermesx-tauri/compare/v1.5.0...v1.6.0) (2026-03-13)


### Features

* **keychain:** OS credential storage via keyring crate (US-019) + credential UI in Settings (US-020) ([653b0a8](https://github.com/beisel-it/hermesx-tauri/commit/653b0a82d986bcf6a211a27da2fa105854400b81))

# [1.5.0](https://github.com/beisel-it/hermesx-tauri/compare/v1.4.0...v1.5.0) (2026-03-13)


### Features

* **sidecar:** ZeusX Node.js/Playwright sidecar with JSON stdio protocol, dry-run, ping/pong (US-001/002/003) ([4b6834a](https://github.com/beisel-it/hermesx-tauri/commit/4b6834a2e909c7cb9a3e263f2fbffb530079b705))

# [1.4.0](https://github.com/beisel-it/hermesx-tauri/compare/v1.3.3...v1.4.0) (2026-03-13)


### Features

* **monitor:** WorkMonitor timer loop — 60s tick, OS notifications, NotificationManager cooldown (US-006–012) ([495c9e1](https://github.com/beisel-it/hermesx-tauri/commit/495c9e1304d42cca6ed5c8182a87baba4eb0bd2f))

## [1.3.3](https://github.com/beisel-it/hermesx-tauri/compare/v1.3.2...v1.3.3) (2026-03-13)


### Bug Fixes

* **ci:** add full icon set (32x32, 128x128, ico) required by Tauri bundler ([c74a30c](https://github.com/beisel-it/hermesx-tauri/commit/c74a30cdf3fa41e48054b918309d3354eca63b6f))

## [1.3.2](https://github.com/beisel-it/hermesx-tauri/compare/v1.3.1...v1.3.2) (2026-03-13)


### Bug Fixes

* **rust:** MutexGuard not Send across await (scope drop before dispatch), RGBA icon ([7bfc6ab](https://github.com/beisel-it/hermesx-tauri/commit/7bfc6ab08d719c7c83a2c66e04ff14cccf32d3d3))

## [1.3.1](https://github.com/beisel-it/hermesx-tauri/compare/v1.3.0...v1.3.1) (2026-03-13)


### Bug Fixes

* **frontend:** vite.config.mts (ESM), svelte-plugin@4 for Svelte 5, dist builds clean ([345938b](https://github.com/beisel-it/hermesx-tauri/commit/345938bd24e897a8ed66aed56209a44b011b0924))

# [1.3.0](https://github.com/beisel-it/hermesx-tauri/compare/v1.2.0...v1.3.0) (2026-03-13)


### Features

* proper tauri.conf.json (window config, tray, bundle) + CI streamline ([e6fa94b](https://github.com/beisel-it/hermesx-tauri/commit/e6fa94ba503785bf15a3a94a0e57185337d3b5f5))

# [1.2.0](https://github.com/beisel-it/hermesx-tauri/compare/v1.1.1...v1.2.0) (2026-03-13)


### Features

* **rust:** add persistence (US-005) + systray setup (US-004/022) + autostart wiring ([166d383](https://github.com/beisel-it/hermesx-tauri/commit/166d383bd91a103eaa4e5d3a068d238347adf4e1))

## [1.1.1](https://github.com/beisel-it/hermesx-tauri/compare/v1.1.0...v1.1.1) (2026-03-13)


### Bug Fixes

* **rust:** resolve clippy warnings (snake_case zeusX fields, Range::contains) ([735de1c](https://github.com/beisel-it/hermesx-tauri/commit/735de1c8eaca33f175d170a81b2abd3e8fa7cffd))

# [1.1.0](https://github.com/beisel-it/hermesx-tauri/compare/v1.0.0...v1.1.0) (2026-03-13)


### Features

* **frontend:** add Svelte 5 UI (App, Settings, tauri IPC wrappers, vite config) ([d291ebe](https://github.com/beisel-it/hermesx-tauri/commit/d291ebe2ba6d5a85616d44af1e69289ee75b6330))

# 1.0.0 (2026-03-13)


### Bug Fixes

* **ci:** remove type:module — breaks semantic-release CJS plugin loader ([e39edc7](https://github.com/beisel-it/hermesx-tauri/commit/e39edc72197baa57c452e90ed5ec147939371466))


### Features

* implement core modules (config, state_machine, work_monitor, zeusX scaffold) ([e6f7db1](https://github.com/beisel-it/hermesx-tauri/commit/e6f7db16dacc89e309e8b3573d9420d27fc11bfa))
* project scaffold + all analysis docs (architecture, user-stories, selectors, tech-stack, feature-parity) ([9dfb8f1](https://github.com/beisel-it/hermesx-tauri/commit/9dfb8f1890a322132a6bcbe4ea218dce8e4fd8cd))
* **rust:** add CredentialManager (storage-agnostic trait + InMemoryStore + 6 tests) ([47116f5](https://github.com/beisel-it/hermesx-tauri/commit/47116f5cedbe6d6ed398d8dde47a99490a19afa1))
* **rust:** add NotificationManager (cooldown/suppress/override, 8 tests) ([6b436c9](https://github.com/beisel-it/hermesx-tauri/commit/6b436c9e53f4003f679f7bf6e800e54f6cf27a43))

# Changelog

All notable changes to HermesX (Tauri) are documented here.

Managed by [semantic-release](https://semantic-release.gitbook.io/).
Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Versioning follows [Semantic Versioning](https://semver.org/).

<!-- semantic-release will prepend generated release notes here -->
