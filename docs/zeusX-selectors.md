# ZeusX Selectors

> Source of truth: `/home/florian/projects/zeus-punch/src/selectors.ts`

## Login

| Field | Selector |
|-------|---------|
| Username | `#uiUserName_I` |
| Password | `#uiPassword_I` |
| Step 1 (Weiter/Continue) | `#uiNextButton` |
| Step 2 (Anmelden/Login) | `#uiLogOnButton` |
| Dashboard verify | `h1` (contains "My ZEUS") |

## Terminal Buttons

| Action | Button ID | Text Fallback |
|--------|-----------|--------------|
| IN / OUT | `#TerminalButton0` | `button:has-text("IN / OUT")` |
| IN (Kommen) | `#TerminalButton1` | `button:has-text("IN"):not(:has-text("/"))` |
| OUT (Gehen) | `#TerminalButton2` | `button:has-text("OUT"):not(:has-text("/"))` |
| Pause (Büro) | `#TerminalButton3` | `button:has-text("Pause"):not(:has-text("mob"))` |
| **Mobiles Arbeiten START** | **`#TerminalButton4`** | `button:has-text("Mobiles Arbeiten beg")` |
| **Mobiles Arbeiten END** | **`#TerminalButton5`** | `button:has-text("Mobiles Arbeiten end")` |
| **Pause mob. Arbeiten** | **`#TerminalButton6`** | `button:has-text("Pause mob. Arbeiten")` |
| Bereitschaft START | `#TerminalButton7` | `button:has-text("Bereitschaft START")` |
| Bereitschaft STOP | `#TerminalButton8` | `button:has-text("Bereitschaft STOP")` |
| Dienstgang | `#TerminalButton9` | `button:has-text("Dienstgang")` |

**Bold** = used by HermesX state machine

## Dashboard Widgets

| Widget | Selector |
|--------|---------|
| Terminal | `[aria-label="Terminal Test Widget"]` |
| Buchungen | `[aria-label="Meine Buchungen"]` |
| Status/Konten | `[aria-label="Status und Konten Information"]` |
| Session extend | `[aria-label="Sitzung verlängern"]` |
| Logout | `[aria-label="Abmeldung in"]` |
| User display | `text=Beisel` |

## Delta: Old HermesX vs zeus-punch

| Action | Old buttonId (HermesX) | New (zeus-punch) | Changed? |
|--------|----------------------|-----------------|---------|
| Start Work | `TerminalButton4` | `#TerminalButton4` | `#` prefix added |
| Finish Work | `TerminalButton5` | `#TerminalButton5` | `#` prefix added |
| Start Break | `TerminalButton6` | `#TerminalButton6` | `#` prefix added |
| Old button text | `"Pause Mobiles Arbeit"` | `"Pause mob. Arbeiten"` | **Text changed!** |

**Critical:** The pause button label changed from `"Pause Mobiles Arbeit"` to `"Pause mob. Arbeiten"`.
HermesX's hardcoded `buttonText` in StateMachine.ts is stale.

## Implementation Strategy for Tauri

Use **primary ID** (`#TerminalButtonN`) with **text fallback**:
```rust
// Pseudocode — actual impl via tauri-plugin-shell + playwright or webview
let selector = format!("#TerminalButton{}", button_index);
// fallback: text-based
```

Selectors live in `src-tauri/src/zeusX/selectors.rs` (single source of truth).
