// ZeusX integration — migrated from zeus-punch selectors
// Source: https://github.com/florianbeisel/zeus-punch/src/selectors.ts

pub mod selectors;

pub const TERMINAL_BUTTONS: &[(&str, &str, &str)] = &[
    ("in-out",                 "#TerminalButton0", "button:has-text(\"IN / OUT\")"),
    ("in",                     "#TerminalButton1", "button:has-text(\"IN\")"),
    ("out",                    "#TerminalButton2", "button:has-text(\"OUT\")"),
    ("pause",                  "#TerminalButton3", "button:has-text(\"Pause\")"),
    ("mobiles-arbeiten-start", "#TerminalButton4", "button:has-text(\"Mobiles Arbeiten beg\")"),
    ("mobiles-arbeiten-end",   "#TerminalButton5", "button:has-text(\"Mobiles Arbeiten end\")"),
    ("pause-mobil",            "#TerminalButton6", "button:has-text(\"Pause mob. Arbeiten\")"),
];
