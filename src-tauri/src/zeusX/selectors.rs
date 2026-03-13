// zeusX/selectors.rs — Single source of truth for all ZeusX DOM selectors
// Source: zeus-punch/src/selectors.ts (2026-03-13)
//
// Strategy: primary ID selector, text-based fallback.
// NEVER hardcode button text in logic — always use this module.

pub struct ButtonSelector {
    pub key: &'static str,
    pub id: &'static str,      // CSS ID selector, e.g. "#TerminalButton4"
    pub text: &'static str,    // Playwright text fallback
    pub label: &'static str,   // Human-readable
}

pub const TERMINAL_BUTTONS: &[ButtonSelector] = &[
    ButtonSelector {
        key:   "in-out",
        id:    "#TerminalButton0",
        text:  "button:has-text(\"IN / OUT\")",
        label: "IN / OUT Toggle",
    },
    ButtonSelector {
        key:   "in",
        id:    "#TerminalButton1",
        text:  "button:has-text(\"IN\")",
        label: "Kommen",
    },
    ButtonSelector {
        key:   "out",
        id:    "#TerminalButton2",
        text:  "button:has-text(\"OUT\")",
        label: "Gehen",
    },
    ButtonSelector {
        key:   "pause",
        id:    "#TerminalButton3",
        text:  "button:has-text(\"Pause\")",
        label: "Pause (Büro)",
    },
    ButtonSelector {
        key:   "mobiles-arbeiten-start",
        id:    "#TerminalButton4",
        text:  "button:has-text(\"Mobiles Arbeiten beg\")",
        label: "Mobiles Arbeiten START",
    },
    ButtonSelector {
        key:   "mobiles-arbeiten-end",
        id:    "#TerminalButton5",
        text:  "button:has-text(\"Mobiles Arbeiten end\")",
        label: "Mobiles Arbeiten END",
    },
    ButtonSelector {
        key:   "pause-mobil",
        id:    "#TerminalButton6",
        text:  "button:has-text(\"Pause mob. Arbeiten\")",
        label: "Pause (Mobiles Arbeiten)",
    },
    ButtonSelector {
        key:   "bereitschaft-start",
        id:    "#TerminalButton7",
        text:  "button:has-text(\"Bereitschaft START\")",
        label: "Bereitschaft START",
    },
    ButtonSelector {
        key:   "bereitschaft-stop",
        id:    "#TerminalButton8",
        text:  "button:has-text(\"Bereitschaft STOP\")",
        label: "Bereitschaft STOP",
    },
    ButtonSelector {
        key:   "dienstgang",
        id:    "#TerminalButton9",
        text:  "button:has-text(\"Dienstgang\")",
        label: "Dienstgang",
    },
];

pub fn find(key: &str) -> Option<&'static ButtonSelector> {
    TERMINAL_BUTTONS.iter().find(|b| b.key == key)
}

pub const LOGIN_USERNAME:   &str = "#uiUserName_I";
pub const LOGIN_PASSWORD:   &str = "#uiPassword_I";
pub const LOGIN_CONTINUE:   &str = "#uiNextButton";
pub const LOGIN_SUBMIT:     &str = "#uiLogOnButton";
pub const DASHBOARD_VERIFY: &str = "h1"; // contains "My ZEUS"
pub const SESSION_EXTEND:   &str = "[aria-label=\"Sitzung verlängern\"]";
