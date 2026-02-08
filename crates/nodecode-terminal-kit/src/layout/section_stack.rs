//! Section stack rendering utilities.
//!
//! Provides reusable primitives for rendering vertically stacked sections
//! with consistent padding, styling, and truncation behavior.

use ratatui::layout::{Alignment, Rect};
use ratatui::style::Modifier;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Padding, Paragraph, Wrap};
use ratatui::Frame;

use crate::theme::{Theme, ThemeElement};

/// A section is a group of styled lines.
pub type Section = Vec<Line<'static>>;

/// Render a vertical stack of sections with consistent padding.
pub fn render_block_stack(
    frame: &mut Frame,
    inner: Rect,
    _theme: &Theme,
    blocks: Vec<Section>,
    padding_left: u16,
    padding_right: u16,
    padding_top: u16,
    padding_bottom: u16,
) {
    render_block_stack_with(
        frame,
        inner,
        blocks,
        padding_left,
        padding_right,
        padding_top,
        padding_bottom,
        Alignment::Left,
        |_, _| Block::default(),
    );
}

/// Render a vertical stack of sections with custom block styling.
///
/// The `block_for` closure receives the section index and section content,
/// returning a styled `Block` for that section.
pub fn render_block_stack_with(
    frame: &mut Frame,
    inner: Rect,
    blocks: Vec<Section>,
    padding_left: u16,
    padding_right: u16,
    padding_top: u16,
    padding_bottom: u16,
    alignment: Alignment,
    mut block_for: impl FnMut(usize, &Section) -> Block<'static>,
) {
    if inner.width == 0 || inner.height == 0 {
        return;
    }

    let pad_height = padding_top + padding_bottom;
    let mut cursor = inner.y;
    let bottom = inner.y + inner.height;

    for (idx, lines) in blocks
        .into_iter()
        .enumerate()
        .filter(|(_, b)| !b.is_empty())
    {
        if cursor >= bottom {
            break;
        }
        let height = (lines.len() as u16 + pad_height).min(bottom.saturating_sub(cursor));
        if height == 0 {
            break;
        }

        let area = Rect {
            x: inner.x,
            y: cursor,
            width: inner.width,
            height,
        };

        let block = block_for(idx, &lines).padding(Padding::new(
            padding_left,
            padding_right,
            padding_top,
            padding_bottom,
        ));

        let paragraph = Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .alignment(alignment)
            .block(block);
        frame.render_widget(paragraph, area);
        cursor = cursor.saturating_add(height);
    }
}

/// Wrap a section body with a styled title header.
///
/// Returns `None` if the body is empty.
pub fn wrap_section(title: &'static str, theme: &Theme, body: Section) -> Option<Section> {
    if body.is_empty() {
        return None;
    }
    let mut lines = Vec::with_capacity(body.len() + 2);
    lines.push(Line::from(vec![Span::styled(
        title,
        theme
            .style(ThemeElement::Primary)
            .add_modifier(Modifier::BOLD),
    )]));
    lines.push(Line::default());
    lines.extend(body);
    Some(lines)
}

/// Create a simple styled text line.
pub fn text_line(text: impl Into<String>, style: ratatui::style::Style) -> Line<'static> {
    Line::from(Span::styled(text.into(), style))
}

/// Clear an area and fill with base theme background.
pub fn clear_area(
    theme: &crate::theme::Theme,
    frame: &mut ratatui::Frame,
    area: ratatui::layout::Rect,
) {
    use crate::theme::ThemeElement;
    use ratatui::widgets::{Block, Clear};

    if area.width == 0 || area.height == 0 {
        return;
    }
    frame.render_widget(Clear, area);
    frame.render_widget(
        Block::default().style(theme.style(ThemeElement::Base)),
        area,
    );
}

/// Truncate text to fit within a maximum display width, adding ellipsis if needed.
pub fn truncate_to_width(text: &str, max_width: usize) -> String {
    use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

    if max_width == 0 {
        return String::new();
    }

    let width = UnicodeWidthStr::width(text);
    if width <= max_width {
        return text.to_string();
    }

    let mut acc = String::new();
    let mut current = 0usize;
    for ch in text.chars() {
        let ch_width = UnicodeWidthChar::width(ch).unwrap_or(0);
        if ch_width == 0 {
            continue;
        }
        if current + ch_width >= max_width.saturating_sub(1) {
            break;
        }
        acc.push(ch);
        current += ch_width;
    }
    acc.push('…');
    acc
}
