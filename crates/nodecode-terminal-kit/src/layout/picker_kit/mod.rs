#![allow(clippy::too_many_arguments)] // Backlog 2025-10-20: Simplify picker toolkit callbacks (Story 2.2).

//! Pure picker framework utilities shared by design-only consumers.

use crate::components::{dialog_shell, text_input::TextInput};
use crate::theme::{to_ratatui, Theme};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use ratatui::crossterm::event::{KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{ListItem, Padding};

pub use crate::components::list::{index_at as list_index_at, ListState};

type FieldSetExtractor<T> = dyn Fn(&T) -> Vec<String> + Send + Sync;
type SurfaceExtractor<T> = dyn Fn(&T) -> String + Send + Sync;

/// Fuzzy-filter items by matching a single query string against multiple fields per item.
/// Returns indices ordered by descending score. A row is included if ANY field matches.
pub fn fuzzy_indices_any_field<T, I, F>(items: &[T], query: &str, fields: F) -> Vec<usize>
where
    F: Fn(&T) -> I,
    I: IntoIterator<Item = String>,
{
    if query.trim().is_empty() {
        return (0..items.len()).collect();
    }
    let matcher = SkimMatcherV2::default();
    let q = query.to_string();
    let mut scored: Vec<(usize, i64)> = items
        .iter()
        .enumerate()
        .filter_map(|(i, it)| {
            let best = fields(it)
                .into_iter()
                .filter(|s| !s.is_empty())
                .filter_map(|s| matcher.fuzzy_match(&s, &q))
                .max();
            best.map(|score| (i, score))
        })
        .collect();
    scored.sort_by(|a, b| b.1.cmp(&a.1));
    scored.into_iter().map(|(i, _)| i).collect()
}

/// Fuzzy-filter items by splitting query into tokens and matching a single surface string per item.
/// A row is included if ALL tokens match the surface. A full-query score is also considered as a boost.
/// Returns indices ordered by descending total score.
pub fn fuzzy_indices_tokenized_surface<T, F>(items: &[T], query: &str, surface: F) -> Vec<usize>
where
    F: Fn(&T) -> String,
{
    if query.trim().is_empty() {
        return (0..items.len()).collect();
    }
    let matcher = SkimMatcherV2::default();
    let q_lower = query.to_lowercase();
    let tokens: Vec<&str> = q_lower
        .split_whitespace()
        .filter(|t| !t.is_empty())
        .collect();

    let mut scored: Vec<(usize, i64)> = items
        .iter()
        .enumerate()
        .filter_map(|(i, it)| {
            let surf_lc = surface(it).to_lowercase();
            let mut total: i64 = 0;
            for t in &tokens {
                match matcher.fuzzy_match(&surf_lc, t) {
                    Some(s) => total += s,
                    None => return None,
                }
            }
            if let Some(full) = matcher.fuzzy_match(&surf_lc, &q_lower) {
                total = total.max(full + (tokens.len() as i64) * 5);
            }
            Some((i, total))
        })
        .collect();
    scored.sort_by(|a, b| b.1.cmp(&a.1));
    scored.into_iter().map(|(i, _)| i).collect()
}

/// Fuzzy-match a lowercased tokenized query against a lowercased surface.
/// Returns the aggregate score when all tokens match; otherwise None.
pub fn fuzzy_match_tokenized_surface_with(
    matcher: &SkimMatcherV2,
    query_lc: &str,
    surface_lc: &str,
) -> Option<i64> {
    let query = query_lc.trim();
    if query.is_empty() {
        return Some(0);
    }
    let tokens: Vec<&str> = query.split_whitespace().filter(|t| !t.is_empty()).collect();
    if tokens.is_empty() {
        return Some(0);
    }
    let mut total: i64 = 0;
    for t in &tokens {
        match matcher.fuzzy_match(surface_lc, t) {
            Some(s) => total += s,
            None => return None,
        }
    }
    if let Some(full) = matcher.fuzzy_match(surface_lc, query) {
        total = total.max(full + (tokens.len() as i64) * 5);
    }
    Some(total)
}

/// Convenience wrapper that lowercases inputs before matching.
pub fn fuzzy_match_tokenized_surface(query: &str, surface: &str) -> Option<i64> {
    let matcher = SkimMatcherV2::default();
    let query_lc = query.to_lowercase();
    let surface_lc = surface.to_lowercase();
    fuzzy_match_tokenized_surface_with(&matcher, &query_lc, &surface_lc)
}

/// Map mouse coords to index within a list and set selection + update offset.
pub fn mouse_select_update_offset(
    list: &mut ListState,
    list_area: Rect,
    total: usize,
    visible_height: usize,
    col: u16,
    row: u16,
) -> Option<usize> {
    if let Some(idx) = list_index_at(
        list_area,
        list.selected,
        list.viewport_offset,
        total,
        col,
        row,
    ) {
        list.selected = idx.min(total.saturating_sub(1));
        list.update_offset(visible_height);
        Some(list.selected)
    } else {
        None
    }
}

/// Filtering strategy for a picker.
pub enum FilterMode<T> {
    /// Fuzzy-match a query against multiple fields; include row if ANY field matches.
    AnyField { fields: Box<FieldSetExtractor<T>> },
    /// Tokenized fuzzy-match against a single synthesized surface string; require ALL tokens to match.
    TokenizedSurface { surface: Box<SurfaceExtractor<T>> },
}

/// Picker options controlling behavior.
pub struct PickerOptions<T> {
    pub filter: FilterMode<T>,
}

impl<T> PickerOptions<T> {
    #[must_use]
    pub fn no_section(filter: FilterMode<T>) -> Self {
        Self { filter }
    }
}

/// Generic picker state shared by concrete pickers.
#[derive(Debug)]
pub struct PickerState<T> {
    pub is_open: bool,
    pub is_loading: bool,
    pub items: Vec<T>,
    pub search_input: TextInput,
    pub filtered_indices: Vec<usize>,
    pub list: ListState,
}

impl<T> Default for PickerState<T> {
    fn default() -> Self {
        Self {
            is_open: false,
            is_loading: false,
            items: Vec::new(),
            search_input: TextInput::new(),
            filtered_indices: Vec::new(),
            list: ListState::new(),
        }
    }
}

impl<T> PickerState<T> {
    pub fn open(&mut self, items: Vec<T>, opts: &PickerOptions<T>) {
        self.is_open = true;
        self.is_loading = false;
        self.items = items;
        self.search_input.clear();
        self.filtered_indices.clear();
        self.list.selected = 0;
        self.update_filter(opts);
    }

    pub fn open_loading(&mut self) {
        self.is_open = true;
        self.is_loading = true;
        self.items.clear();
        self.filtered_indices.clear();
        self.list.selected = 0;
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.is_loading = false;
        self.search_input.clear();
        self.filtered_indices.clear();
    }

    pub fn update_filter(&mut self, opts: &PickerOptions<T>) {
        let q = self.search_input.text();
        self.filtered_indices = match &opts.filter {
            FilterMode::AnyField { fields } => {
                fuzzy_indices_any_field(&self.items, q, |t| (fields)(t))
            }
            FilterMode::TokenizedSurface { surface } => {
                fuzzy_indices_tokenized_surface(&self.items, q, |t| (surface)(t))
            }
        };

        let len = self.filtered_indices.len();
        if len == 0 {
            self.list.selected = 0;
        } else {
            self.list.selected = self.list.selected.min(len - 1);
        }
    }

    #[must_use]
    pub fn selectable_len(&self) -> usize {
        self.filtered_indices.len()
    }

    #[must_use]
    pub fn to_item_index(&self, visible_idx: usize) -> Option<usize> {
        self.filtered_indices.get(visible_idx).copied()
    }
}

/// Generic picker messages.
#[derive(Debug, Clone)]
pub enum PickerMsg<Custom = ()> {
    UpdateSearch(String),
    ClearSearch,
    SelectNext,
    SelectPrevious,
    SelectIndex(usize),
    PageUp(usize),
    PageDown(usize),
    JumpTop,
    JumpBottom,
    ScrollUp,
    ScrollDown,
    Confirm,
    Close,
    Custom(Custom),
}

/// Apply a picker message to state.
pub fn picker_update_state<T, Custom>(
    state: &mut PickerState<T>,
    msg: PickerMsg<Custom>,
    opts: &PickerOptions<T>,
) where
    Custom: Clone + Send + 'static,
{
    use PickerMsg::*;
    match msg {
        UpdateSearch(query) => {
            state.search_input.set_text(query);
            state.update_filter(opts);
            state.list.selected = 0;
            state.list.viewport_offset = 0;
        }
        ClearSearch => {
            state.search_input.clear();
            state.update_filter(opts);
            state.list.selected = 0;
            state.list.viewport_offset = 0;
        }
        SelectNext => {
            let len = state.filtered_indices.len();
            if len > 0 {
                state.list.select_next(len);
            }
        }
        SelectPrevious => {
            let len = state.filtered_indices.len();
            if len > 0 {
                state.list.select_prev(len);
            }
        }
        SelectIndex(i) => {
            let len = state.filtered_indices.len();
            if len > 0 {
                let max = len - 1;
                state.list.selected = i.min(max);
            }
        }
        PageUp(visible_h) => {
            let len = state.filtered_indices.len();
            if len > 0 {
                state.list.page_up(visible_h, len);
            }
        }
        PageDown(visible_h) => {
            let len = state.filtered_indices.len();
            if len > 0 {
                state.list.page_down(visible_h, len);
            }
        }
        JumpTop => {
            state.list.jump_top();
        }
        JumpBottom => {
            let len = state.filtered_indices.len();
            state.list.jump_bottom(len);
        }
        ScrollUp => {
            let len = state.filtered_indices.len();
            if len > 0 {
                state.list.scroll_lines(-3, len);
            }
        }
        ScrollDown => {
            let len = state.filtered_indices.len();
            if len > 0 {
                state.list.scroll_lines(3, len);
            }
        }
        Confirm => {}
        Close => {
            state.close();
        }
        Custom(_) => {}
    }
}

/// Optional hooks to extend key handling.
pub trait PickerHooks<T, Custom = ()> {
    fn before_key(&self, _state: &PickerState<T>, _key: KeyEvent) -> Option<PickerMsg<Custom>> {
        None
    }

    fn after_key(&self, _state: &PickerState<T>, _key: KeyEvent) -> Option<PickerMsg<Custom>> {
        None
    }
}

impl<T, Custom> PickerHooks<T, Custom> for () {}

/// Translate a key event into a picker message.
pub fn key_to_picker_msg<T, H, Custom>(
    state: &mut PickerState<T>,
    key: KeyEvent,
    visible_height: Option<usize>,
    _opts: &PickerOptions<T>,
    hooks: &H,
) -> Option<PickerMsg<Custom>>
where
    H: PickerHooks<T, Custom>,
    T: Send + 'static,
    Custom: Clone + Send + 'static,
{
    if !state.is_open {
        return None;
    }

    if let Some(msg) = hooks.before_key(state, key) {
        return Some(msg);
    }

    match key.code {
        KeyCode::Esc => return Some(PickerMsg::Close),
        KeyCode::Enter => return Some(PickerMsg::Confirm),
        KeyCode::Up => return Some(PickerMsg::SelectPrevious),
        KeyCode::Down => return Some(PickerMsg::SelectNext),
        KeyCode::PageUp => return Some(PickerMsg::PageUp(visible_height.unwrap_or(10))),
        KeyCode::PageDown => return Some(PickerMsg::PageDown(visible_height.unwrap_or(10))),
        KeyCode::Home => return Some(PickerMsg::JumpTop),
        KeyCode::End => return Some(PickerMsg::JumpBottom),
        _ => {}
    }

    if let Some(after) = state.search_input.handle_search_key(key) {
        return if after.is_empty() {
            Some(PickerMsg::ClearSearch)
        } else {
            Some(PickerMsg::UpdateSearch(after))
        };
    }

    hooks.after_key(state, key)
}

/// Renderer contract for generic pickers.
pub trait PickerRenderer<T> {
    fn render_item(&self, item: &T, is_selected: bool, theme: &Theme) -> ListItem<'static>;
}

/// Translate a mouse event into a picker message.
pub fn mouse_to_picker_msg<T, Custom>(
    state: &mut PickerState<T>,
    mouse: MouseEvent,
    area: Rect,
    dialog_opts: dialog_shell::DialogOptions,
) -> Option<PickerMsg<Custom>>
where
    T: Send + 'static,
    Custom: Clone + Send + 'static,
{
    if !state.is_open {
        return None;
    }

    let layout = dialog_shell::compute_centered(area, dialog_opts);
    let list_area = layout.body;
    let total = state.filtered_indices.len();
    if total == 0 {
        return None;
    }

    match mouse.kind {
        MouseEventKind::ScrollUp => Some(PickerMsg::ScrollUp),
        MouseEventKind::ScrollDown => Some(PickerMsg::ScrollDown),
        MouseEventKind::Moved => list_index_at(
            list_area,
            state.list.selected,
            state.list.viewport_offset,
            total,
            mouse.column,
            mouse.row,
        )
        .map(PickerMsg::SelectIndex),
        MouseEventKind::Down(MouseButton::Left) => {
            let vh = list_area.height.saturating_sub(2) as usize;
            mouse_select_update_offset(
                &mut state.list,
                list_area,
                total,
                vh,
                mouse.column,
                mouse.row,
            )
            .map(PickerMsg::SelectIndex)
        }
        MouseEventKind::Up(MouseButton::Left) => list_index_at(
            list_area,
            state.list.selected,
            state.list.viewport_offset,
            total,
            mouse.column,
            mouse.row,
        )
        .map(|_| PickerMsg::Confirm),
        _ => None,
    }
}

/// Standard dialog options for hooks-style pickers.
pub const DIALOG_OPTS_HOOKS: dialog_shell::DialogOptions = dialog_shell::DialogOptions {
    width_pct: 1.0,
    height_pct: 1.0,
    max_width: 60,
    max_height: 20,
    header_rows: 3,
    footer_rows: 1,
    padding: Padding::new(1, 1, 1, 1),
};

/// Compute base style for a picker item considering selection state.
pub fn picker_item_base_style(is_selected: bool, default_style: Style, theme: &Theme) -> Style {
    if !is_selected {
        return default_style;
    }
    default_style
        .fg(to_ratatui(theme.selection))
        .add_modifier(Modifier::BOLD)
}
