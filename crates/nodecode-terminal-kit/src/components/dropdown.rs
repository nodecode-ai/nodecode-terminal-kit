use crate::components::dialog_shell;
use crate::components::list::{render_list_with_chrome, ListChrome, ListState};
use crate::theme::{to_ratatui, Theme};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    widgets::{ListItem, Padding},
    Frame,
};
use std::sync::Arc;
use std::{fmt, fmt::Formatter};

#[derive(Debug, Clone)]
pub struct DropdownList<T> {
    filtered: Vec<T>,
    filter_text: String,
    list: ListState,
}

impl<T> DropdownList<T> {
    pub fn new() -> Self {
        Self {
            filtered: Vec::new(),
            filter_text: String::new(),
            list: ListState::new(),
        }
    }

    pub fn reset(&mut self, filter_text: &str, filtered: Vec<T>) {
        self.filter_text = filter_text.to_string();
        self.filtered = filtered;
        self.list.selected = 0;
        self.list.viewport_offset = 0;
    }

    pub fn is_empty(&self) -> bool {
        self.filtered.is_empty()
    }

    pub fn len(&self) -> usize {
        self.filtered.len()
    }

    pub fn items(&self) -> &[T] {
        &self.filtered
    }

    pub fn filter_text(&self) -> &str {
        &self.filter_text
    }

    pub fn selected_index(&self) -> Option<usize> {
        if self.filtered.is_empty() {
            None
        } else {
            Some(self.list.selected)
        }
    }

    pub fn selected(&self) -> Option<&T> {
        self.selected_index().and_then(|idx| self.filtered.get(idx))
    }

    pub fn set_selected(&mut self, idx: usize) {
        let len = self.filtered.len();
        if len > 0 {
            self.list.set_selected(idx, len);
        }
    }

    pub fn select_prev(&mut self) {
        let len = self.filtered.len();
        if len > 0 {
            self.list.select_prev(len);
        }
    }

    pub fn select_next(&mut self) {
        let len = self.filtered.len();
        if len > 0 {
            self.list.select_next(len);
        }
    }

    pub fn clamp_and_scroll(&mut self, visible_items: usize) {
        let len = self.filtered.len();
        self.list.clamp_selection(len);
        self.list.update_offset(visible_items);
    }

    pub fn selected_for_render(&self) -> usize {
        self.list.selected
    }

    pub fn viewport_offset(&self) -> usize {
        self.list.viewport_offset
    }
}

impl<T> Default for DropdownList<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct FuzzyDropdown<T> {
    items: Arc<Vec<T>>,
    list: DropdownList<usize>,
    matcher: Arc<SkimMatcherV2>,
}

impl<T: fmt::Debug> fmt::Debug for FuzzyDropdown<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("FuzzyDropdown")
            .field("items_len", &self.items.len())
            .field("filtered_len", &self.list.len())
            .finish()
    }
}

impl<T> FuzzyDropdown<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self {
            items: Arc::new(items),
            list: DropdownList::new(),
            matcher: Arc::new(SkimMatcherV2::default()),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn items(&self) -> &[T] {
        self.items.as_ref()
    }

    pub fn items_arc(&self) -> Arc<Vec<T>> {
        Arc::clone(&self.items)
    }

    pub fn selected_index(&self) -> Option<usize> {
        self.list.selected_index()
    }

    pub fn visible_count(&self) -> usize {
        self.list.len()
    }

    pub fn set_selected_index(&mut self, idx: usize) {
        self.list.set_selected(idx);
    }

    pub fn select_previous(&mut self) {
        self.list.select_prev();
    }

    pub fn select_next(&mut self) {
        self.list.select_next();
    }

    pub fn update_items(&mut self, items: Vec<T>) {
        self.items = Arc::new(items);
    }

    pub fn list(&self) -> &DropdownList<usize> {
        &self.list
    }

    pub fn list_mut(&mut self) -> &mut DropdownList<usize> {
        &mut self.list
    }

    pub fn update_filter(
        &mut self,
        filter: &str,
        empty_limit: usize,
        filtered_limit: usize,
        key_fn: impl Fn(&T) -> &str,
    ) {
        let mut matches: Vec<(usize, i64)> = if filter.is_empty() {
            self.items
                .iter()
                .enumerate()
                .take(empty_limit)
                .map(|(idx, _)| (idx, i64::MAX))
                .collect()
        } else {
            self.items
                .iter()
                .enumerate()
                .filter_map(|(idx, item)| {
                    self.matcher
                        .fuzzy_match(key_fn(item), filter)
                        .map(|score| (idx, score))
                })
                .collect()
        };

        matches.sort_by(|a, b| b.1.cmp(&a.1));
        let limit = if filter.is_empty() {
            empty_limit
        } else {
            filtered_limit
        };
        matches.truncate(limit);
        let indices = matches.into_iter().map(|(idx, _)| idx).collect();
        self.list.reset(filter, indices);
    }
}

pub fn resolve_dropdown_area(
    frame_area: Rect,
    input_area: Rect,
    placement_area: Option<Rect>,
    total_items: usize,
) -> Option<(Rect, usize)> {
    if total_items == 0 {
        return None;
    }

    if let Some(area) = placement_area {
        if area.height == 0 {
            return None;
        }
        let rows = area.height as usize;
        if rows == 0 {
            return None;
        }
        let visible = total_items.min(rows).max(1);
        return Some((area, visible));
    }

    let input_bottom = input_area.y.saturating_add(input_area.height);
    let space_below = frame_area.height.saturating_sub(input_bottom);
    if space_below > 0 {
        let visible = total_items.min(space_below as usize).max(1);
        let dropdown_height = visible as u16;
        let area = Rect {
            x: input_area.x,
            y: input_bottom,
            width: input_area.width,
            height: dropdown_height,
        };
        return Some((area, visible));
    }

    if input_area.y == 0 {
        return None;
    }
    let max_items_above = input_area.y as usize;
    let visible = total_items.min(max_items_above).max(1);
    let dropdown_height = visible as u16;
    let area = Rect {
        x: input_area.x,
        y: input_area.y.saturating_sub(dropdown_height),
        width: input_area.width,
        height: dropdown_height,
    };
    Some((area, visible))
}

pub fn render_dropdown<'a, T>(
    frame: &mut Frame,
    input_area: Rect,
    placement_area: Option<Rect>,
    theme: &Theme,
    list: &mut DropdownList<T>,
    item_builder: impl Fn(usize, bool) -> ListItem<'a>,
) -> bool {
    let total = list.len();
    if total == 0 {
        return false;
    }

    let Some((dropdown_area, visible_items)) =
        resolve_dropdown_area(frame.area(), input_area, placement_area, total)
    else {
        return false;
    };

    dialog_shell::paint_background(frame, dropdown_area, theme);
    list.clamp_and_scroll(visible_items);

    render_list_with_chrome(
        frame,
        dropdown_area,
        theme,
        ListChrome::plain().with_padding(Padding::new(0, 0, 0, 0)),
        list.selected_for_render(),
        list.viewport_offset(),
        total,
        item_builder,
    );
    true
}

pub fn dropdown_item_base_style(theme: &Theme, is_selected: bool) -> Style {
    let surface_bg = to_ratatui(theme.background_surface);
    let base_fg = if is_selected {
        to_ratatui(theme.selection)
    } else {
        to_ratatui(theme.foreground)
    };
    let mut style = Style::default().bg(surface_bg).fg(base_fg);
    if is_selected {
        style = style.add_modifier(Modifier::BOLD);
    }
    style
}
