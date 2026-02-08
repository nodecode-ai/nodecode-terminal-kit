use std::io::{self, Write};

use crate::theme::Color;

/// Best-effort: set terminal cursor color using OSC 12.
pub fn set_cursor_color(color: Color) {
    if let Color::Rgb { r, g, b } = color {
        let seq = format!("\x1b]12;#{:02X}{:02X}{:02X}\x07", r, g, b);
        let _ = io::stdout().write_all(seq.as_bytes());
        let _ = io::stdout().flush();
    }
}

/// Reset terminal cursor color to terminal default (OSC 112).
pub fn reset_cursor_color() {
    let _ = io::stdout().write_all(b"\x1b]112\x07");
    let _ = io::stdout().flush();
}

/// Best-effort: set terminal background color using OSC 11.
pub fn set_terminal_background_color(color: Color) {
    if let Color::Rgb { r, g, b } = color {
        let seq = format!("\x1b]11;#{:02X}{:02X}{:02X}\x07", r, g, b);
        let _ = io::stdout().write_all(seq.as_bytes());
        let _ = io::stdout().flush();
    }
}

/// Reset terminal background color to terminal default (OSC 111).
pub fn reset_terminal_background_color() {
    let _ = io::stdout().write_all(b"\x1b]111\x07");
    let _ = io::stdout().flush();
}
