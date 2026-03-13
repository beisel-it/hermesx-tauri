# Feature Parity: HermesX Electron → Tauri

## ✅ 1:1 Migrieren

| Feature | HermesX | Tauri-Impl | Notes |
|---------|---------|-----------|-------|
| Systray Icon | Electron Tray | Tauri SystemTray | Status-Emoji im Titel |
| State Machine | TypeScript | Rust | Gleiche States + Transitions |
| State Persistence | JSON file | `tauri-plugin-store` | |
| Config (Schedule etc) | JSON file | `tauri-plugin-store` | |
| Settings Window | BrowserWindow | Tauri Window (Svelte) | |
| Credential Storage | `safeStorage` | `tauri-plugin-keyring` | Echter System-Keychain |
| Auto-Start | Electron `app.setLoginItemSettings` | `tauri-plugin-autostart` | |
| Auto-Updater | Electron `autoUpdater` | Tauri built-in updater | |
| Lock-Screen Detection | `powerMonitor.on('lock-screen')` | Tauri Event / custom Rust | |
| Idle Detection | `powerMonitor.getSystemIdleTime()` | `tauri-plugin-system-info` | |
| Notifications | Electron `Notification` | `tauri-plugin-notification` | |
| Meeting Detection (macOS) | `systemPreferences.getMediaAccessStatus` | macOS API via Rust | |
| Meeting Detection (Win) | Process scan | Process scan via Rust | |

## 🆕 Neu / Verbessert in Tauri

| Feature | Was sich ändert |
|---------|----------------|
| **ZeusX Selektoren** | Aktualisiert auf zeus-punch Stand (`#TerminalButton4-6` + Text-Fallback) |
| **Pause-Button Text** | `"Pause Mobiles Arbeit"` → `"Pause mob. Arbeiten"` (war in Electron falsch) |
| **Credential Security** | Echter System-Keychain statt Custom-Encryption (echtes OS-Keyring) |
| **Bundle Size** | ~150MB → ~5MB |
| **Memory Footprint** | ~150MB+ → ~30MB |
| **Gaming Detection** | Stub → echte Fullscreen-Detection (Windows API) |

## ❌ Nicht in v1 (Backlog)

| Feature | Warum nicht v1 |
|---------|---------------|
| Auto Check-In | Config-Flag existed, never implemented — deliberate deferral |
| Auto Check-Out | Same |
| Long break reminder | Config-Threshold existed, never triggered — re-evaluate |
| Calendar integration (holidays) | Nice-to-have, new feature |
| `suppressDuringGaming` (Windows) | Stub in Electron, proper impl in backlog |

## ZeusX Selector Delta (kritisch)

| Action | Old (HermesX) | New (zeus-punch) | Fix needed |
|--------|-------------|-----------------|-----------|
| Start Work | `TerminalButton4` (no #) | `#TerminalButton4` | `#` prefix |
| Finish Work | `TerminalButton5` | `#TerminalButton5` | `#` prefix |
| Start Break | `TerminalButton6` | `#TerminalButton6` | `#` prefix |
| Break text | `"Pause Mobiles Arbeit"` | `"Pause mob. Arbeiten"` | **Text wrong** |
