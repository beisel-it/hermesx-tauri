# Changelog

All notable changes to HermesX (Tauri) will be documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Versioning follows [Semantic Versioning](https://semver.org/).
Commits follow [Conventional Commits](https://www.conventionalcommits.org/).

---

## [Unreleased]

### Added
- Initial Tauri project structure (migration from Electron v0.5.0)
- `config.rs` — UserConfig with first-class `dry_run` field
- `state_machine.rs` — pure FSM (NotWorking/Working/Paused/Finished)
- `work_monitor.rs` — pure activity/schedule evaluation with unit tests
- `zeusX/` — dispatch scaffold with dry-run guard (real impl: WILBUR-20260313-015)
- `zeusX/selectors.rs` — all terminal button selectors from zeus-punch
- GitHub Actions CI (lint + test, 3 OS) and Release (multi-OS, multi-arch)
- Conventional commits enforcement via commitlint
