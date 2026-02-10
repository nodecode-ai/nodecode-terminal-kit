use std::borrow::Cow;

use ratatui::text::{Line, Span};
use ratatui::{
    layout::{Alignment, Rect},
    style::Style,
    widgets::Paragraph,
    Frame,
};

use crate::theme::{Theme, ThemeElement};
use ratatui::style::Modifier;

/// Structured representation of a help bar entry (keybind + description).
#[derive(Clone, Debug)]
pub struct HelpEntry<'a> {
    pub key: Cow<'a, str>,
    pub description: Cow<'a, str>,
}

impl<'a> HelpEntry<'a> {
    #[must_use]
    pub fn new<K, D>(key: K, description: D) -> Self
    where
        K: Into<Cow<'a, str>>,
        D: Into<Cow<'a, str>>,
    {
        Self {
            key: key.into(),
            description: description.into(),
        }
    }
}

/// Render a left-aligned, dimmed help bar from structured entries.
/// Callers may customise the separator (e.g. "  ", " · ").
pub fn render_help_bar_entries<'a, I>(
    frame: &mut Frame,
    area: Rect,
    theme: &Theme,
    entries: I,
    separator: &str,
) where
    I: IntoIterator<Item = HelpEntry<'a>>,
{
    render_help_bar_entries_aligned(frame, area, theme, entries, separator, Alignment::Left);
}

/// Render a help bar from structured entries with custom alignment.
/// Callers may customise the separator (e.g. "  ", " · ") and alignment.
pub fn render_help_bar_entries_aligned<'a, I>(
    frame: &mut Frame,
    area: Rect,
    theme: &Theme,
    entries: I,
    separator: &str,
    alignment: Alignment,
) where
    I: IntoIterator<Item = HelpEntry<'a>>,
{
    render_help_bar_entries_aligned_with_bracket_key_style(
        frame, area, theme, entries, separator, alignment, None, None,
    );
}

/// Render a help bar from structured entries with custom alignment and optional
/// style override for bracketed keys (e.g. `[EXEC]`) or one exact plain key
/// match (e.g. `EXEC`).
pub fn render_help_bar_entries_aligned_with_bracket_key_style<'a, I>(
    frame: &mut Frame,
    area: Rect,
    theme: &Theme,
    entries: I,
    separator: &str,
    alignment: Alignment,
    bracket_key_style_override: Option<Style>,
    plain_key_style_override_match: Option<&str>,
) where
    I: IntoIterator<Item = HelpEntry<'a>>,
{
    let keybind_style = theme.style(ThemeElement::Secondary);
    let desc_style = theme.style(ThemeElement::Tertiary);
    let separator_span = if separator.is_empty() {
        None
    } else {
        Some(Span::styled(separator.to_string(), desc_style))
    };

    let mut spans: Vec<Span> = Vec::new();
    let mut iter = entries.into_iter().peekable();

    while let Some(entry) = iter.next() {
        let key = entry.key;
        let description = entry.description;
        let key_is_empty = key.is_empty();
        let desc_is_empty = description.is_empty();

        if !key_is_empty {
            let plain_key_matches = plain_key_style_override_match
                .map(|plain_key| key.as_ref() == plain_key)
                .unwrap_or(false);
            let styled_key = if key.starts_with('[') || plain_key_matches {
                bracket_key_style_override
                    .unwrap_or(keybind_style)
                    .add_modifier(Modifier::BOLD)
            } else {
                keybind_style
            };
            spans.push(Span::styled(key, styled_key));
        }

        if !desc_is_empty {
            if !key_is_empty {
                spans.push(Span::styled(" ", desc_style));
            }
            spans.push(Span::styled(description, desc_style));
        }

        if iter.peek().is_some() {
            if let Some(sep) = &separator_span {
                spans.push(sep.clone());
            }
        }
    }

    let help = Paragraph::new(Line::from(spans)).alignment(alignment);
    frame.render_widget(help, area);
}

/// Render a help text bar from a formatted string with optional alignment override.
/// Expected format: "key description  key description  ..." (double-space separated).
///
/// Note: For wizard help text (plain description without keybinds), just pass the text directly.
/// The function will render it as a wrapped paragraph with specified alignment (defaults to left-aligned).
///
/// Alignment parameter allows overrides while preserving left-aligned default
/// for dialogs and pickers (established global pattern).
pub fn render_help_bar(
    frame: &mut Frame,
    area: Rect,
    theme: &Theme,
    text: &str,
    alignment: Option<Alignment>,
) {
    let alignment = alignment.unwrap_or(Alignment::Left);

    // Check if this is formatted help entries (contains double spaces) or plain text
    if text.contains("  ") {
        // Formatted keybind entries - use the structured renderer
        let entries: Vec<HelpEntry> = text
            .split("  ")
            .filter(|pair| !pair.is_empty())
            .map(|pair| {
                if let Some(space_pos) = pair.find(' ') {
                    let keybind = &pair[..space_pos];
                    let description = &pair[space_pos + 1..];
                    HelpEntry::new(keybind, description)
                } else {
                    HelpEntry::new(pair, "")
                }
            })
            .collect();

        render_help_bar_entries_aligned(frame, area, theme, entries, "  ", alignment);
    } else {
        // Plain text (wizard help) - render as wrapped paragraph with specified alignment
        use ratatui::widgets::Wrap;
        use ratatui::widgets::{Block, Padding};

        let desc_style = theme.style(ThemeElement::Tertiary);

        // Add left padding when left-aligned (for proper visual indentation)
        let (help, render_area) = if alignment == Alignment::Left {
            let padding_block = Block::default().padding(Padding::left(1));
            let inner = padding_block.inner(area);
            let help = Paragraph::new(text)
                .style(desc_style)
                .alignment(alignment)
                .wrap(Wrap { trim: true });
            (help, inner)
        } else {
            let help = Paragraph::new(text)
                .style(desc_style)
                .alignment(alignment)
                .wrap(Wrap { trim: true });
            (help, area)
        };

        frame.render_widget(help, render_area);
    }
}
