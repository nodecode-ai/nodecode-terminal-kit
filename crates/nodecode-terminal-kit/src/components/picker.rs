#![allow(clippy::too_many_arguments)] // Backlog 2025-10-20: Collapse picker render parameters into context struct (Story 2.2).

use ratatui::{
    layout::Alignment,
    layout::Rect,
    style::Style,
    widgets::{Paragraph, Wrap},
    Frame,
};

use crate::theme::Theme;

/// Render a simple centered message without borders.
/// Useful for loading/empty states inside already-bordered dialogs.
pub fn render_centered_message(frame: &mut Frame, area: Rect, theme: &Theme, text: &str) {
    // Compute vertical centering
    let lines = vertically_centered_text(text, area.height);

    let widget = Paragraph::new(lines)
        .style(Style::default().fg(crate::theme::to_ratatui(theme.tertiary)))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    frame.render_widget(widget, area);
}

fn vertically_centered_text(text: &str, total_height: u16) -> Vec<ratatui::text::Line<'static>> {
    let text_line = total_height / 2;
    let mut lines = Vec::with_capacity(text_line as usize + 1);
    for _ in 0..text_line {
        lines.push(ratatui::text::Line::from(""));
    }
    lines.push(ratatui::text::Line::from(text.to_string()));
    lines
}
