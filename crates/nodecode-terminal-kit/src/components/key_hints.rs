use ratatui::layout::{Alignment, Rect};
use ratatui::style::Style;
use ratatui::widgets::Block;
use ratatui::Frame;

use super::help_bar::{render_help_bar_entries_aligned_with_bracket_key_style, HelpEntry};
use crate::theme::{to_ratatui, Theme, ThemeElement};

fn hint_to_entry(hint: &str) -> HelpEntry<'_> {
    if let Some((binding, rest)) = hint.split_once(' ') {
        HelpEntry::new(binding, rest.trim_start())
    } else {
        HelpEntry::new(hint, "")
    }
}

fn entry_text_width(entry: &HelpEntry<'_>) -> usize {
    let key = entry.key.as_ref();
    let desc = entry.description.as_ref();
    let mut width = key.chars().count();
    if !desc.is_empty() {
        if !key.is_empty() {
            width += 1;
        }
        width += desc.chars().count();
    }
    width
}

fn hints_text_width(hints: &[String]) -> usize {
    let mut width = 0;
    for (idx, hint) in hints.iter().enumerate() {
        let entry = hint_to_entry(hint);
        width += entry_text_width(&entry);
        if idx + 1 < hints.len() {
            width += 2;
        }
    }
    width
}

fn text_width(s: &str) -> usize {
    s.chars().count()
}

fn bracket_key_style(
    theme: &Theme,
    agent: Option<&str>,
    agent_color_override: Option<(u8, u8, u8)>,
) -> Option<Style> {
    let agent = agent.filter(|name| !name.is_empty())?;
    let agent_color = theme.agent_color(agent, agent_color_override);
    Some(Style::default().fg(to_ratatui(agent_color)))
}

/// Render a dedicated key-hints block anchored beneath the chat area.
pub fn view(hints: &[String], frame: &mut Frame, area: Rect, theme: &Theme) {
    view_with_background_style_and_right_info(
        hints,
        None,
        frame,
        area,
        theme,
        Some(theme.style(ThemeElement::BackgroundVoid)),
        None,
    );
}

/// Render a key-hints block with an optional right-aligned info summary.
pub fn view_with_right_info(
    hints: &[String],
    right_info: Option<&str>,
    frame: &mut Frame,
    area: Rect,
    theme: &Theme,
    agent: Option<&str>,
    agent_color_override: Option<(u8, u8, u8)>,
) {
    let bracket_key_style_override = bracket_key_style(theme, agent, agent_color_override);
    view_with_background_style_and_right_info(
        hints,
        right_info,
        frame,
        area,
        theme,
        Some(theme.style(ThemeElement::BackgroundVoid)),
        bracket_key_style_override,
    );
}

/// Render key hints with an optional background fill.
pub fn view_with_background_style(
    hints: &[String],
    frame: &mut Frame,
    area: Rect,
    theme: &Theme,
    background_style: Option<Style>,
) {
    view_with_background_style_and_right_info(
        hints,
        None,
        frame,
        area,
        theme,
        background_style,
        None,
    );
}

fn view_with_background_style_and_right_info(
    hints: &[String],
    right_info: Option<&str>,
    frame: &mut Frame,
    area: Rect,
    theme: &Theme,
    background_style: Option<Style>,
    bracket_key_style_override: Option<Style>,
) {
    if area.height == 0 {
        return;
    }

    if let Some(style) = background_style {
        frame.render_widget(Block::default().style(style), area);
    }

    // Use area directly - padding is handled by layout alignment
    let padded_area = area;

    // Check if the last hint is the help hint
    let (main_hints, right_hint) = if let Some(last) = hints.last() {
        if last == "? for help" {
            (&hints[..hints.len() - 1], Some(last.as_str()))
        } else {
            (hints, None)
        }
    } else {
        (hints, None)
    };

    let right_info = right_info.map(str::trim).filter(|info| !info.is_empty());

    let right_width = if right_hint.is_some() {
        15u16.min(padded_area.width)
    } else {
        0
    };
    let middle_width = padded_area.width.saturating_sub(right_width);
    let middle_area = Rect {
        width: middle_width,
        ..padded_area
    };
    let right_area = Rect {
        x: padded_area.x.saturating_add(middle_width),
        width: right_width,
        ..padded_area
    };

    let min_left_width = hints_text_width(main_hints) as u16;
    let render_left = |frame: &mut Frame, area: Rect| {
        if area.width == 0 {
            return;
        }
        render_help_bar_entries_aligned_with_bracket_key_style(
            frame,
            area,
            theme,
            main_hints.iter().map(|h| hint_to_entry(h)),
            "  ",
            Alignment::Left,
            bracket_key_style_override,
        );
    };

    if let Some(info) = right_info {
        let info_width = text_width(info) as u16;
        let max_info_width = middle_width.saturating_sub(min_left_width);
        let info_width = info_width.min(max_info_width);
        if info_width > 0 && middle_width > 0 {
            let left_width = middle_width.saturating_sub(info_width);
            let left_area = Rect {
                width: left_width,
                ..padded_area
            };
            let info_area = Rect {
                x: padded_area.x.saturating_add(left_width),
                width: info_width,
                ..padded_area
            };
            render_left(frame, left_area);
            let info_entry = HelpEntry::new("", info);
            render_help_bar_entries_aligned_with_bracket_key_style(
                frame,
                info_area,
                theme,
                vec![info_entry],
                "",
                Alignment::Right,
                bracket_key_style_override,
            );
        } else {
            render_left(frame, middle_area);
        }
    } else {
        render_left(frame, middle_area);
    }

    if let Some(rh) = right_hint {
        if right_area.width > 0 {
            let right_entry = hint_to_entry(rh);
            render_help_bar_entries_aligned_with_bracket_key_style(
                frame,
                right_area,
                theme,
                vec![right_entry],
                "",
                Alignment::Right,
                bracket_key_style_override,
            );
        }
    }
}
