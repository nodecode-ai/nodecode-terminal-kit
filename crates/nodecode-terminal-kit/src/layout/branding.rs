use ratatui::layout::{Alignment, Rect};
use ratatui::style::Modifier;
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Clear, Paragraph};
use ratatui::Frame;
use unicode_width::UnicodeWidthStr;

use crate::theme::{Theme, ThemeElement};

/// Compact 3-row logo for welcome screens and headers.
pub const LOGO_COMPACT: [&str; 3] = [
    "▄  ▄  ▄▄  ▄▄▄  ▄▄▄  ▄▄▄  ▄▄  ▄▄▄  ▄▄▄",
    "█▀▄█ █  █ █  █ █▄  █    █  █ █  █ █▄ ",
    "▀  ▀  ▀▀  ▀▀▀  ▀▀▀  ▀▀▀  ▀▀  ▀▀▀  ▀▀▀",
];

/// Extended 6-row stacked logo for sidebar display.
pub const LOGO_STACKED: [&str; 6] = [
    "▄  ▄  ▄▄  ▄▄▄  ▄▄▄",
    "█▀▄█ █  █ █  █ █▄ ",
    "▀  ▀  ▀▀  ▀▀▀  ▀▀▀",
    " ▄▄▄  ▄▄  ▄▄▄  ▄▄▄",
    "█    █  █ █  █ █▄ ",
    " ▀▀▀  ▀▀  ▀▀▀  ▀▀▀",
];

/// Height of the compact logo in terminal rows.
pub const LOGO_COMPACT_HEIGHT: u16 = LOGO_COMPACT.len() as u16;

/// Height of the stacked logo in terminal rows.
pub const LOGO_STACKED_HEIGHT: u16 = LOGO_STACKED.len() as u16;

/// Build styled logo lines from a logo definition.
pub fn logo_lines(theme: &Theme, rows: &[&str]) -> Vec<Line<'static>> {
    rows.iter()
        .map(|row| {
            Line::from(Span::styled(
                row.to_string(),
                theme.primary_style().add_modifier(Modifier::BOLD),
            ))
        })
        .collect()
}

/// Build styled logo lines with right padding for sidebar alignment.
pub fn logo_lines_padded(theme: &Theme, rows: &[&str], max_width: u16) -> Vec<Line<'static>> {
    let pad_width = max_width as usize;
    let right_padding = 1usize;

    rows.iter()
        .map(|row| {
            let row_width = UnicodeWidthStr::width(*row);
            let mut text = row.to_string();
            if row_width + right_padding <= pad_width {
                text.push_str(&" ".repeat(right_padding));
            }
            Line::from(Span::styled(
                text,
                theme.primary_style().add_modifier(Modifier::BOLD),
            ))
        })
        .collect()
}

/// Build a version string line.
pub fn version_line(theme: &Theme, version: &str) -> Line<'static> {
    Line::from(Span::styled(
        version.to_string(),
        theme.primary_style().add_modifier(Modifier::DIM),
    ))
}

/// Build a version string line with right padding.
pub fn version_line_padded(theme: &Theme, version: &str, max_width: u16) -> Line<'static> {
    let right_padding = 1usize;
    let pad_width = max_width as usize;

    let mut value = version.to_string();
    if UnicodeWidthStr::width(value.as_str()) + right_padding <= pad_width {
        value.push_str(&" ".repeat(right_padding));
    }
    Line::from(Span::styled(
        value,
        theme.primary_style().add_modifier(Modifier::DIM),
    ))
}

/// Render a centered logo with version in the given area.
pub fn render_logo_centered(theme: &Theme, frame: &mut Frame, area: Rect, version: &str) {
    if area.width == 0 || area.height == 0 {
        return;
    }

    let lines = logo_lines(theme, &LOGO_COMPACT);
    let logo_height = lines.len() as u16;

    let layout = ratatui::layout::Layout::vertical([
        ratatui::layout::Constraint::Length(logo_height),
        ratatui::layout::Constraint::Length(1),
        ratatui::layout::Constraint::Min(0),
    ])
    .split(area);

    let logo_area = layout[0];
    let version_area = layout[1];

    frame.render_widget(Clear, logo_area);
    frame.render_widget(Clear, version_area);
    let bg = Block::default().style(theme.style(ThemeElement::Base));
    frame.render_widget(bg.clone(), logo_area);
    frame.render_widget(bg, version_area);

    let paragraph = Paragraph::new(Text::from(lines)).alignment(Alignment::Center);
    frame.render_widget(paragraph, logo_area);

    let version =
        Paragraph::new(Text::from(vec![version_line(theme, version)])).alignment(Alignment::Center);
    frame.render_widget(version, version_area);
}
