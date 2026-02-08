use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{ListItem, Padding, Paragraph, Wrap};
use ratatui::Frame;
use unicode_width::UnicodeWidthStr;

use crate::components::dialog_shell::{layout_centered, DialogLayout, DialogOptions};
use crate::components::list::{render_list_with_chrome, ListChrome, ListState};
use crate::theme::{Theme, ThemeElement};

pub const DEFAULT_TABBED_DIALOG_OPTS: DialogOptions = DialogOptions {
    width_pct: 1.0,
    height_pct: 1.0,
    max_width: 60,
    max_height: 20,
    header_rows: 3,
    footer_rows: 2,
    padding: Padding::new(1, 1, 1, 1),
};

pub const LIST_VIEWPORT_HEIGHT_ESTIMATE: usize = 10;

pub struct TextBody {
    pub text: Text<'static>,
    pub style: ThemeElement,
}

pub struct ListBody<'a> {
    pub list_state: &'a ListState,
    pub total: usize,
    pub hint_lines: Vec<Line<'static>>,
    pub hint_rows: Option<u16>,
    pub render_item: Box<dyn Fn(usize, bool) -> ListItem<'static> + 'a>,
    pub empty: Option<TextBody>,
}

impl ListBody<'_> {
    fn resolved_hint_rows(&self) -> u16 {
        self.hint_rows
            .unwrap_or_else(|| self.hint_lines.len() as u16)
    }
}

pub enum TabBody<'a> {
    List(ListBody<'a>),
    Text(TextBody),
    Custom(Box<dyn FnOnce(&mut Frame, Rect, &Theme) + 'a>),
}

pub struct ListKeySpec<'a, CmdT> {
    pub list_state: &'a mut ListState,
    pub len: usize,
    pub on_confirm: Option<Box<dyn FnOnce(Option<usize>) -> CmdT + 'a>>,
    pub on_key: Option<Box<dyn FnMut(KeyEvent, Option<usize>) -> Option<CmdT> + 'a>>,
}

impl<'a, CmdT> ListKeySpec<'a, CmdT> {
    pub fn new(list_state: &'a mut ListState, len: usize) -> Self {
        Self {
            list_state,
            len,
            on_confirm: None,
            on_key: None,
        }
    }

    pub fn on_confirm(mut self, handler: impl FnOnce(Option<usize>) -> CmdT + 'a) -> Self {
        self.on_confirm = Some(Box::new(handler));
        self
    }

    pub fn on_key(
        mut self,
        handler: impl FnMut(KeyEvent, Option<usize>) -> Option<CmdT> + 'a,
    ) -> Self {
        self.on_key = Some(Box::new(handler));
        self
    }
}

pub struct PlainKeySpec<'a, CmdT> {
    pub on_confirm: Option<Box<dyn FnOnce() -> CmdT + 'a>>,
    pub on_key: Option<Box<dyn FnMut(KeyEvent) -> Option<CmdT> + 'a>>,
}

impl<'a, CmdT> PlainKeySpec<'a, CmdT> {
    pub fn new() -> Self {
        Self {
            on_confirm: None,
            on_key: None,
        }
    }

    pub fn on_confirm(mut self, handler: impl FnOnce() -> CmdT + 'a) -> Self {
        self.on_confirm = Some(Box::new(handler));
        self
    }

    pub fn on_key(mut self, handler: impl FnMut(KeyEvent) -> Option<CmdT> + 'a) -> Self {
        self.on_key = Some(Box::new(handler));
        self
    }
}

pub enum TabKeySpec<'a, CmdT> {
    List(ListKeySpec<'a, CmdT>),
    Plain(PlainKeySpec<'a, CmdT>),
}

impl<'a, CmdT> TabKeySpec<'a, CmdT> {
    pub fn list(list_state: &'a mut ListState, len: usize) -> Self {
        Self::List(ListKeySpec::new(list_state, len))
    }

    pub fn plain() -> Self {
        Self::Plain(PlainKeySpec::new())
    }
}

pub fn next_tab<T: Copy + Eq>(order: &[T], current: T) -> T {
    let idx = order.iter().position(|t| *t == current).unwrap_or(0);
    let next = (idx + 1) % order.len();
    order[next]
}

pub fn prev_tab<T: Copy + Eq>(order: &[T], current: T) -> T {
    let idx = order.iter().position(|t| *t == current).unwrap_or(0);
    let prev = (idx + order.len() - 1) % order.len();
    order[prev]
}

pub fn apply_tab_navigation<T: Copy + Eq>(order: &[T], current: &mut T, key: &KeyEvent) -> bool {
    match key.code {
        KeyCode::BackTab => {
            *current = prev_tab(order, *current);
            true
        }
        KeyCode::Tab | KeyCode::Char('\t') => {
            if key.modifiers.contains(KeyModifiers::SHIFT) {
                *current = prev_tab(order, *current);
            } else {
                *current = next_tab(order, *current);
            }
            true
        }
        _ => false,
    }
}

pub fn handle_list_navigation(
    key: &KeyEvent,
    list: &mut ListState,
    len: usize,
    viewport_estimate: usize,
) -> bool {
    match key.code {
        KeyCode::Up => {
            list.select_prev(len);
            list.update_offset(viewport_estimate);
            true
        }
        KeyCode::Down => {
            list.select_next(len);
            list.update_offset(viewport_estimate);
            true
        }
        _ => false,
    }
}

pub fn is_confirm_key(key: &KeyEvent) -> bool {
    matches!(key.code, KeyCode::Enter | KeyCode::Char(' '))
}

pub fn hint_line(
    theme: &Theme,
    label: &str,
    value: impl Into<String>,
    value_style: ThemeElement,
) -> Line<'static> {
    Line::from(vec![
        Span::styled(label.to_string(), theme.style(ThemeElement::Tertiary)),
        Span::styled(value.into(), theme.style(value_style)),
    ])
}

pub fn tab_bar_line_from_order<T, L, S>(
    order: &[T],
    active: T,
    width: u16,
    mut label: L,
    mut style: S,
) -> Line<'static>
where
    T: Copy + Eq,
    L: FnMut(T) -> String,
    S: FnMut(T, bool) -> Style,
{
    let mut labels: Vec<(String, Style)> = Vec::with_capacity(order.len());
    let mut used_width: u16 = 0;

    for (i, tab) in order.iter().enumerate() {
        let text = label(*tab);
        used_width = used_width.saturating_add(UnicodeWidthStr::width(text.as_str()) as u16);
        if i + 1 < order.len() {
            used_width = used_width.saturating_add(1);
        }
        labels.push((text, style(*tab, *tab == active)));
    }

    if used_width < width {
        let remaining = (width - used_width) as usize;
        if let Some((last_text, _)) = labels.last_mut() {
            last_text.push_str(&" ".repeat(remaining));
        }
    }

    let tab_count = labels.len();
    let mut spans: Vec<Span> = Vec::with_capacity(tab_count.saturating_mul(2).saturating_sub(1));
    for (i, (text, style)) in labels.into_iter().enumerate() {
        spans.push(Span::styled(text, style));
        if i + 1 < tab_count {
            spans.push(Span::raw(" "));
        }
    }

    Line::from(spans)
}

pub fn render_tabbed_header<F>(
    frame: &mut Frame,
    area: Rect,
    theme: &Theme,
    opts: DialogOptions,
    tab_line: F,
    title: &str,
) -> DialogLayout
where
    F: FnOnce(u16) -> Line<'static>,
{
    let layout = layout_centered(frame, area, theme, opts);

    let header_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(layout.header);

    let tab_bar = Paragraph::new(Text::from(vec![tab_line(header_chunks[0].width)]))
        .style(theme.base_style())
        .wrap(Wrap { trim: false });
    frame.render_widget(tab_bar, header_chunks[0]);

    let title = Paragraph::new(title)
        .style(
            theme
                .style(ThemeElement::Primary)
                .add_modifier(Modifier::BOLD),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(title, header_chunks[2]);

    layout
}

pub fn render_tabbed_body(frame: &mut Frame, area: Rect, theme: &Theme, body: TabBody<'_>) {
    match body {
        TabBody::List(body) => {
            if body.total == 0 {
                if let Some(empty) = body.empty {
                    let paragraph = Paragraph::new(empty.text)
                        .style(theme.style(empty.style))
                        .wrap(Wrap { trim: false });
                    frame.render_widget(paragraph, area);
                } else {
                    render_list_with_hints(
                        frame,
                        area,
                        theme,
                        body.list_state,
                        body.total,
                        body.resolved_hint_rows(),
                        |idx, selected| (body.render_item)(idx, selected),
                        body.hint_lines,
                    );
                }
                return;
            }

            render_list_with_hints(
                frame,
                area,
                theme,
                body.list_state,
                body.total,
                body.resolved_hint_rows(),
                |idx, selected| (body.render_item)(idx, selected),
                body.hint_lines,
            );
        }
        TabBody::Text(body) => {
            let paragraph = Paragraph::new(body.text)
                .style(theme.style(body.style))
                .wrap(Wrap { trim: false });
            frame.render_widget(paragraph, area);
        }
        TabBody::Custom(render) => {
            render(frame, area, theme);
        }
    }
}

pub fn handle_tabbed_key<Tab, CmdT>(
    key: KeyEvent,
    order: &[Tab],
    active: &mut Tab,
    mut spec: TabKeySpec<'_, CmdT>,
) -> Option<CmdT>
where
    Tab: Copy + Eq,
{
    if apply_tab_navigation(order, active, &key) {
        return None;
    }

    match &mut spec {
        TabKeySpec::List(list) => {
            if handle_list_navigation(
                &key,
                list.list_state,
                list.len,
                LIST_VIEWPORT_HEIGHT_ESTIMATE,
            ) {
                return None;
            }
            let selected = if list.len == 0 {
                None
            } else {
                Some(list.list_state.selected.min(list.len.saturating_sub(1)))
            };
            if let Some(handler) = list.on_key.as_mut() {
                if let Some(cmd) = handler(key, selected) {
                    return Some(cmd);
                }
            }
            if is_confirm_key(&key) {
                if let Some(on_confirm) = list.on_confirm.take() {
                    return Some(on_confirm(selected));
                }
            }
        }
        TabKeySpec::Plain(plain) => {
            if let Some(handler) = plain.on_key.as_mut() {
                if let Some(cmd) = handler(key) {
                    return Some(cmd);
                }
            }
            if is_confirm_key(&key) {
                if let Some(on_confirm) = plain.on_confirm.take() {
                    return Some(on_confirm());
                }
            }
        }
    }

    None
}

pub fn render_list_with_hints(
    frame: &mut Frame,
    area: Rect,
    theme: &Theme,
    list_state: &ListState,
    total: usize,
    hint_rows: u16,
    render_item: impl Fn(usize, bool) -> ListItem<'static>,
    hint_lines: Vec<Line<'static>>,
) {
    if hint_rows == 0 {
        render_list_with_chrome(
            frame,
            area,
            theme,
            ListChrome::Plain {
                padding: Padding::new(0, 0, 0, 0),
            },
            list_state.selected,
            list_state.viewport_offset,
            total,
            render_item,
        );
        return;
    }

    let body_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(hint_rows)])
        .split(area);

    render_list_with_chrome(
        frame,
        body_chunks[0],
        theme,
        ListChrome::Plain {
            padding: Padding::new(0, 0, 0, 0),
        },
        list_state.selected,
        list_state.viewport_offset,
        total,
        render_item,
    );

    let hint = Paragraph::new(Text::from(hint_lines))
        .style(theme.style(ThemeElement::Secondary))
        .wrap(Wrap { trim: false });
    frame.render_widget(hint, body_chunks[1]);
}

#[macro_export]
macro_rules! define_tabbed_dialog {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $(
                $variant:ident => { label: $label:expr, title: $title:expr }
            ),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        $vis enum $name {
            $($variant),+
        }

        impl $name {
            pub const ORDER: &'static [$name] = &[
                $($name::$variant),+
            ];

            pub fn label(self) -> &'static str {
                match self {
                    $($name::$variant => $label),+
                }
            }

            pub fn title(self) -> &'static str {
                match self {
                    $($name::$variant => $title),+
                }
            }
        }
    };
}
