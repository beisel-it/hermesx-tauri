# HermesX (Tauri)

A companion app for ISGUS ZeusX time tracking — rebuilt with Tauri.

> Migration from the original [HermesX (Electron)](https://github.com/florianbeisel/hermesx).

## Status

🚧 In development — architecture & migration phase

## Docs

- [Architecture](docs/architecture.md)
- [User Stories](docs/user-stories.md)
- [ZeusX Selectors](docs/zeusX-selectors.md)
- [Tech Stack Decision](docs/tech-stack.md)
- [Feature Parity](docs/feature-parity.md)

## Development

### Prerequisites
- Rust stable (`rustup install stable`)
- Node.js 20+, Yarn
- Tauri CLI: `cargo install tauri-cli`

### Setup
```bash
yarn install
yarn tauri dev
```

### Code Quality
```bash
cargo fmt --manifest-path src-tauri/Cargo.toml   # format Rust
cargo clippy --manifest-path src-tauri/Cargo.toml # lint Rust
cargo test --manifest-path src-tauri/Cargo.toml   # unit tests
yarn lint                                          # ESLint
yarn check                                         # svelte-check
yarn test:unit                                     # Vitest
```

### Commits
This project enforces [Conventional Commits](https://www.conventionalcommits.org/):
```
feat(rust): add system idle detection
fix(zeusX): correct pause button selector text
ci: add windows arm64 build target
```

### Release
1. Update `CHANGELOG.md` under `[Unreleased]`
2. Bump version in `src-tauri/Cargo.toml`, `package.json`, `src-tauri/tauri.conf.json`
3. Commit: `chore(release): v1.0.0`
4. Tag: `git tag v1.0.0 && git push --tags`
5. CI builds all platforms → GitHub Release (draft) → review → publish

### Dry Run Mode
All ZeusX interactions respect `dry_run`. Enable via settings or:
```rust
// config default
dry_run: false  // set to true for development without ZeusX access
```
