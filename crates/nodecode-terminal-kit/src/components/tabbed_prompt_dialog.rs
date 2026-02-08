use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Padding, Paragraph, Wrap};
use ratatui::Frame;

use crate::components::dialog_shell::{layout_centered, DialogOptions};
use crate::components::text_input::TextInput;
use crate::components::{help_bar, search_bar, tabbed_dialog};
use crate::theme::{Theme, ThemeElement};

const TAB_BAR_ROWS: u16 = 1;
const TITLE_ROWS: u16 = 1;
const SEARCH_ROWS: u16 = 3;
const FOOTER_ROWS: u16 = 2;
const DEFAULT_HEADER_ROWS: u16 = TAB_BAR_ROWS + TITLE_ROWS + SEARCH_ROWS;

pub const DEFAULT_PROMPT_DIALOG_OPTS: DialogOptions = DialogOptions {
    width_pct: 1.0,
    height_pct: 1.0,
    max_width: 60,
    max_height: 20,
    header_rows: DEFAULT_HEADER_ROWS,
    footer_rows: FOOTER_ROWS,
    padding: Padding::new(1, 1, 1, 1),
};

pub struct SearchSpec<'a> {
    pub input: &'a TextInput,
    pub title: Option<&'a str>,
}

pub fn prompt_dialog_opts(has_search: bool, has_title: bool) -> DialogOptions {
    let mut opts = DEFAULT_PROMPT_DIALOG_OPTS;
    opts.header_rows = header_rows(has_search, has_title);
    opts.footer_rows = FOOTER_ROWS;
    opts
}

fn header_rows(has_search: bool, has_title: bool) -> u16 {
    TAB_BAR_ROWS + if has_title { TITLE_ROWS } else { 0 } + if has_search { SEARCH_ROWS } else { 0 }
}

pub fn render_tabbed_prompt_dialog<Tab, Label, StyleFn>(
    frame: &mut Frame,
    area: Rect,
    theme: &Theme,
    opts: DialogOptions,
    order: &[Tab],
    active: Tab,
    label: Label,
    style: StyleFn,
    title: &str,
    search: Option<SearchSpec<'_>>,
    body: tabbed_dialog::TabBody<'_>,
    footer_text: &str,
) where
    Tab: Copy + Eq,
    Label: FnMut(Tab) -> String,
    StyleFn: FnMut(Tab, bool, &Theme) -> Style,
{
    render_tabbed_prompt_dialog_with_title_hints(
        frame,
        area,
        theme,
        opts,
        order,
        active,
        label,
        style,
        title,
        None,
        search,
        body,
        footer_text,
    );
}

pub fn render_tabbed_prompt_dialog_with_title_hints<Tab, Label, StyleFn>(
    frame: &mut Frame,
    area: Rect,
    theme: &Theme,
    opts: DialogOptions,
    order: &[Tab],
    active: Tab,
    mut label: Label,
    mut style: StyleFn,
    title: &str,
    title_right_hints: Option<&str>,
    search: Option<SearchSpec<'_>>,
    body: tabbed_dialog::TabBody<'_>,
    footer_text: &str,
) where
    Tab: Copy + Eq,
    Label: FnMut(Tab) -> String,
    StyleFn: FnMut(Tab, bool, &Theme) -> Style,
{
    let mut opts = opts;
    let has_title = !title.is_empty();
    let has_search = search.is_some();
    opts.header_rows = header_rows(has_search, has_title);

    let layout = layout_centered(frame, area, theme, opts);

    let header_chunks = match (has_search, has_title) {
        (true, true) => Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(TAB_BAR_ROWS),
                Constraint::Length(TITLE_ROWS),
                Constraint::Length(SEARCH_ROWS),
            ])
            .split(layout.header),
        (true, false) => Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(TAB_BAR_ROWS),
                Constraint::Length(SEARCH_ROWS),
            ])
            .split(layout.header),
        (false, true) => Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(TAB_BAR_ROWS),
                Constraint::Length(TITLE_ROWS),
            ])
            .split(layout.header),
        (false, false) => Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(TAB_BAR_ROWS)])
            .split(layout.header),
    };

    let tab_line = tabbed_dialog::tab_bar_line_from_order(
        order,
        active,
        header_chunks[0].width,
        |tab| label(tab),
        |tab, is_active| style(tab, is_active, theme),
    );

    let tab_bar = Paragraph::new(Text::from(vec![Line::from(tab_line)]))
        .style(theme.base_style())
        .wrap(Wrap { trim: false });
    frame.render_widget(tab_bar, header_chunks[0]);

    if has_title {
        let title_style = theme
            .style(ThemeElement::Primary)
            .add_modifier(Modifier::BOLD);
        let title_row = header_chunks[1];
        if let Some(right_hints) = title_right_hints.map(str::trim).filter(|h| !h.is_empty()) {
            let title_min_width = title.chars().count() as u16;
            let available_for_hints = title_row
                .width
                .saturating_sub(title_min_width.saturating_add(1));
            let truncated = truncate_with_ellipsis(right_hints, available_for_hints as usize);
            if !truncated.is_empty() {
                let hint_width = truncated.chars().count() as u16;
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Min(0), Constraint::Length(hint_width)])
                    .split(title_row);
                let title_widget = Paragraph::new(title)
                    .style(title_style)
                    .wrap(Wrap { trim: false });
                frame.render_widget(title_widget, chunks[0]);
                help_bar::render_help_bar(
                    frame,
                    chunks[1],
                    theme,
                    &truncated,
                    Some(Alignment::Right),
                );
            } else {
                let title_widget = Paragraph::new(title)
                    .style(title_style)
                    .wrap(Wrap { trim: false });
                frame.render_widget(title_widget, title_row);
            }
        } else {
            let title_widget = Paragraph::new(title)
                .style(title_style)
                .wrap(Wrap { trim: false });
            frame.render_widget(title_widget, title_row);
        }
    }

    if let Some(search) = search {
        let search_idx = if has_title { 2 } else { 1 };
        search_bar::render_search_bar(
            frame,
            header_chunks[search_idx],
            theme,
            search.input,
            search.title,
        );
    }

    tabbed_dialog::render_tabbed_body(frame, layout.body, theme, body);

    let footer_area = if layout.footer.height > 1 {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(1)])
            .split(layout.footer);
        rows[1]
    } else {
        layout.footer
    };
    help_bar::render_help_bar(frame, footer_area, theme, footer_text, None);
}

fn truncate_with_ellipsis(text: &str, max_chars: usize) -> String {
    if max_chars == 0 {
        return String::new();
    }

    let len = text.chars().count();
    if len <= max_chars {
        return text.to_string();
    }

    if max_chars <= 3 {
        return ".".repeat(max_chars);
    }

    let keep = max_chars - 3;
    let mut out = String::with_capacity(max_chars);
    out.extend(text.chars().take(keep));
    out.push_str("...");
    out
}
