use ratatui::{layout::Rect, style::Modifier, Frame};

use super::input_box::InputBox;
use super::text_input::TextInput;
use crate::theme::Theme;
use crate::theme::ThemeElement;

/// Render a standardized search bar using TextInput with an optional title line and placeholder.
/// Also places the terminal cursor at the correct position within the input box.
pub fn render_search_bar(
    frame: &mut Frame,
    area: Rect,
    theme: &Theme,
    input: &TextInput,
    title: Option<&str>,
) {
    // Use the common input box renderer with an optional title and placeholder; no scrollbar or status line.
    let _ = InputBox::new(input, theme)
        .follow_cursor(true)
        .title_override(title)
        .placeholder_style(ThemeElement::Tertiary)
        .prompt_highlight_style(
            theme
                .style(ThemeElement::Selection)
                .add_modifier(Modifier::BOLD),
        )
        .render(frame, area);
}
