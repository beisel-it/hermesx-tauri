# HermesX Architecture Analysis

> Source: https://github.com/florianbeisel/hermesx (Electron v0.5.0)

## Component Overview

```
┌─────────────────────────────────────────────────────────┐
│                        main.ts                          │
│  App entry · Tray · Menu · IPC · Lifecycle              │
│                                                         │
│  ┌──────────────┐  ┌──────────────┐  ┌───────────────┐ │
│  │ StateMachine │  │ WorkMonitor  │  │Notification   │ │
│  │              │◄─┤              │  │Manager        │ │
│  │ State FSM    │  │ Activity     │  │               │ │
│  │ Persistence  │  │ Schedule     │  │ Cooldowns     │ │
│  │ Transitions  │  │ Idle detect  │  │ Suppression   │ │
│  └──────┬───────┘  └──────────────┘  └───────────────┘ │
│         │                                               │
│  ┌──────▼───────┐  ┌──────────────┐  ┌───────────────┐ │
│  │CredentialMgr │  │ ConfigManager│  │ SettingsWindow│ │
│  │              │  │              │  │               │ │
│  │safeStorage   │  │ UserConfig   │  │ BrowserWindow │ │
│  │credentials   │  │ JSON persist │  │ IPC bridge    │ │
│  └──────────────┘  └──────────────┘  └───────────────┘ │
└─────────────────────────────────────────────────────────┘
                          │
                    ZeusX Browser
                  (Playwright/puppeteer)
```

## State Machine

### States
```
NOT_WORKING ──[Start Work]──► WORKING ──[Start Break]──► PAUSED
     ▲                           │                          │
     │                           │[Finish Work]             │[Return]
     │                           ▼                          │
     └──────────────────── FINISHED ◄──────────────────────┘
                          (resets to NOT_WORKING)
```

### Persistence
- `work-state.json` in Electron userData
- Restored on app start
- Contains: `currentState`, `startTime` (epoch ms), `totalWorkedTime` (ms)

### Button Mappings
- `button-mappings.json` in userData (overrides hardcoded defaults)
- Hardcoded defaults → **outdated** (see ZeusX selectors doc)

### Actions per State
| State | Action | ZeusX Button |
|-------|--------|-------------|
| NOT_WORKING | Start Work | TerminalButton4 |
| WORKING | Start Break | TerminalButton6 |
| WORKING | Finish Work | TerminalButton5 |
| PAUSED | Return from Break | TerminalButton4 |
| PAUSED | Finish Work | TerminalButton5 |

## WorkMonitor

### Activity Detection
- `powerMonitor.on('lock-screen' | 'suspend' | 'shutdown')` → pause reminder
- `powerMonitor.getSystemIdleTime()` → checked every 60s
- Inactivity threshold: configurable (default 15 min → break suggestion)

### Schedule Monitoring (every 60s)
- Checks if current time > scheduledStart → morning reminder (15min grace → 30min escalation)
- Checks if current time > scheduledEnd → end-of-day reminder (0min) / overtime warning (30min+)
- Tracks break overtime → notifies every 5min if break runs long

### Break Tracking
- `continuousWorkStartTime` → short break reminder after configured interval (default 4h)
- `expectedBreakReturnTime` = now + `breakDuration` (minutes)
- `lastBreakTime` → tracked but not currently used for logic

## NotificationManager

### Suppression Logic (checked before every notification)
1. `quietMode` (config flag) → suppress all
2. Meeting detection:
   - macOS: microphone access granted = in meeting
   - Windows/Linux: process scan (`zoom`, `teams`, `webex`, `discord`, `slack`, etc.)
3. Gaming detection: placeholder (always false currently)
4. Cooldown: 2 min per notification ID

### Notification Types
| ID | Trigger | Cooldown |
|----|---------|---------|
| `inactivity` | Idle > threshold | 2min |
| `break-reminder` | Continuous work > shortBreakReminder | 2min |
| `break-overrun` | Break > expected return time | 5min |
| `morning-reminder` | 15min after scheduled start, NOT_WORKING | 5min |
| `late-start` | Same as above (first wave) | 5min |
| `very-late-start` | 30min after scheduled start | 5min |
| `end-of-day` | scheduledEnd reached, WORKING | 5min |
| `overtime` | 30min past scheduledEnd, WORKING | 5min |
| `lock-screen` | Lock/suspend while WORKING | instant (no cooldown) |

## CredentialManager

- Electron `safeStorage.encryptString()` → AES-256 encrypted file
- Fallback: Base64 (dev only, insecure)
- Stores: `{ username: string, password: string }`
- File: `credentials.enc` in userData

## ConfigManager

### UserConfig Schema
```typescript
{
  schedule: {
    startTime: { hour: number, minute: number },  // default 08:30
    workDuration: number,   // hours, default 8
    breakDuration: number,  // minutes, default 30
    isFlexible: boolean,
    workdays: boolean[7],   // [Sun..Sat], default Mon-Fri
  },
  notifications: {
    quietMode: boolean,
    smartFlexibility: boolean,
    workModeDetection: boolean,
    autoCheckIn: boolean,       // not implemented
    autoCheckOut: boolean,      // not implemented
    suppressDuringCalls: boolean,
    suppressDuringGaming: boolean,
  },
  inactivityThresholds: {
    shortBreakReminder: 240,   // minutes (4h)
    longBreakReminder: 270,    // minutes (4.5h) — not used currently
    autoBreakSuggestion: 15,   // minutes of idle
    autoCheckOut: 30,          // minutes — not implemented
  },
  debug: boolean,
}
```

## AutoUpdater
- Electron `autoUpdater` with GitHub releases
- Checks on startup (packaged builds only)
- Shows notification on update available, installs on quit

## SettingsWindow
- BrowserWindow with `preload.ts` IPC bridge
- Settings sent via `ipcMain`/`ipcRenderer` channels
- Covers: schedule, notifications, credentials, inactivity thresholds

## Known Issues / Tech Debt
1. **ZeusX selectors hardcoded** in StateMachine (outdated button IDs)
2. `autoCheckIn`/`autoCheckOut` config flags exist but are **not implemented**
3. `longBreakReminder` threshold exists in config but is **never triggered**
4. Gaming detection is a **stub** (always returns false)
5. `suppressDuringCalls` on Windows uses brittle process-name scanning
6. Electron `safeStorage` is platform-specific and non-portable
7. `main.ts` is 773 lines — monolithic, hard to test
