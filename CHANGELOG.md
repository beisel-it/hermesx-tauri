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
