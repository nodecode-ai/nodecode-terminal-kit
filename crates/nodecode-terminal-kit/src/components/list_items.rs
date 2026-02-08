use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::ListItem;

use crate::theme::{to_ratatui, Theme, ThemeElement};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToggleTone {
    SuccessError,
    SuccessSurface,
}

pub fn toggle_item(
    label: impl Into<String>,
    enabled: bool,
    selected: bool,
    theme: &Theme,
    tone: ToggleTone,
) -> ListItem<'static> {
    let status_color = match tone {
        ToggleTone::SuccessError => {
            if enabled {
                theme.success
            } else {
                theme.error
            }
        }
        ToggleTone::SuccessSurface => {
            if enabled {
                theme.success
            } else {
                theme.foreground
            }
        }
    };

    let base_bg = to_ratatui(theme.background_surface);
    let base_fg = if selected {
        to_ratatui(theme.selection)
    } else {
        to_ratatui(theme.foreground)
    };
    let base_style = Style::default().bg(base_bg).fg(base_fg);
    let status_fg = to_ratatui(status_color);
    let mut label_style = theme
        .style(ThemeElement::Primary)
        .add_modifier(Modifier::BOLD)
        .bg(base_bg);
    if selected {
        label_style = label_style.fg(to_ratatui(theme.selection));
    } else {
        label_style = label_style.fg(status_fg);
    }

    let line = Line::from(vec![
        Span::styled("  ", base_style),
        Span::styled(label.into(), label_style),
    ]);
    ListItem::new(line).style(base_style)
}

pub fn plain_item(label: impl Into<String>, selected: bool, theme: &Theme) -> ListItem<'static> {
    let mut style = theme.style(ThemeElement::Primary);
    if selected {
        style = theme
            .style(ThemeElement::Selection)
            .add_modifier(Modifier::BOLD);
    }
    let line = Line::from(vec![
        Span::styled("  ", style),
        Span::styled(label.into(), style),
    ]);
    ListItem::new(line).style(style)
}

pub fn value_item(
    label: impl Into<String>,
    value: impl Into<String>,
    selected: bool,
    theme: &Theme,
    value_style: ThemeElement,
) -> ListItem<'static> {
    let base_style = crate::theme::list_item_style(theme, ThemeElement::Base, selected, false);
    let line = Line::from(vec![
        Span::raw("  "),
        Span::styled(
            label.into(),
            theme
                .style(ThemeElement::Primary)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(value.into(), theme.style(value_style)),
    ]);
    ListItem::new(line).style(base_style)
}
