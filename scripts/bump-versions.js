#!/usr/bin/env node
// Bumps version in package.json, src-tauri/Cargo.toml, src-tauri/tauri.conf.json
// Called by semantic-release @semantic-release/exec prepareCmd

const fs   = require('fs');
const path = require('path');

const version = process.argv[2];
if (!version) { console.error('Usage: bump-versions.js <version>'); process.exit(1); }

const root = path.resolve(__dirname, '..');

// 1. package.json
const pkgPath = path.join(root, 'package.json');
const pkg = JSON.parse(fs.readFileSync(pkgPath, 'utf8'));
pkg.version = version;
fs.writeFileSync(pkgPath, JSON.stringify(pkg, null, 2) + '\n');
console.log(`✓ package.json → ${version}`);

// 2. src-tauri/Cargo.toml  (only [package] section, not workspace deps)
const cargoPath = path.join(root, 'src-tauri', 'Cargo.toml');
let cargo = fs.readFileSync(cargoPath, 'utf8');
cargo = cargo.replace(
  /^(version\s*=\s*)"[^"]*"/m,
  `$1"${version}"`
);
fs.writeFileSync(cargoPath, cargo);
console.log(`✓ src-tauri/Cargo.toml → ${version}`);

// 3. crates/hermesx-core/Cargo.toml
const corePath = path.join(root, 'crates', 'hermesx-core', 'Cargo.toml');
let core = fs.readFileSync(corePath, 'utf8');
core = core.replace(
  /^(version\s*=\s*)"[^"]*"/m,
  `$1"${version}"`
);
fs.writeFileSync(corePath, core);
console.log(`✓ crates/hermesx-core/Cargo.toml → ${version}`);

// 4. src-tauri/tauri.conf.json
const tauriPath = path.join(root, 'src-tauri', 'tauri.conf.json');
const tauri = JSON.parse(fs.readFileSync(tauriPath, 'utf8'));
tauri.version = version;
fs.writeFileSync(tauriPath, JSON.stringify(tauri, null, 2) + '\n');
console.log(`✓ src-tauri/tauri.conf.json → ${version}`);

console.log(`\nAll versions bumped to ${version}`);
