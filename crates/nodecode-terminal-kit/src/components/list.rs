#![allow(clippy::too_many_arguments)] // Backlog 2025-10-20: Reduce list widget parameter count (Story 2.2).

use crate::primitives::geom::{contains, inner_1px};
use crate::theme::{to_ratatui, Theme, ThemeElement};
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::{Block, Borders, List, ListItem, Padding};
use ratatui::Frame;

/// Visual chrome options for list rendering.
#[derive(Debug, Clone)]
pub enum ListChrome {
    /// Default bordered chrome with a title and standard padding.
    Bordered { title: String, padding: Padding },
    /// Borderless chrome intended for lightweight dropdowns.
    Plain { padding: Padding },
}

impl ListChrome {
    #[must_use]
    pub fn bordered(title: impl Into<String>) -> Self {
        Self::Bordered {
            title: title.into(),
            padding: Padding::new(0, 0, 0, 0),
        }
    }

    #[must_use]
    pub fn plain() -> Self {
        Self::Plain {
            padding: Padding::new(0, 0, 0, 0),
        }
    }

    #[must_use]
    pub fn with_padding(self, padding: Padding) -> Self {
        match self {
            Self::Bordered { title, .. } => Self::Bordered { title, padding },
            Self::Plain { .. } => Self::Plain { padding },
        }
    }

    fn block<'a>(&'a self, theme: &Theme) -> Block<'a> {
        match self {
            Self::Bordered { title, padding } => Block::default()
                .borders(Borders::ALL)
                .border_type(theme.border_type)
                .title(title.clone())
                .border_style(theme.style(ThemeElement::BorderFocused))
                .padding(*padding)
                .style(theme.style(ThemeElement::BackgroundSurface)),
            Self::Plain { padding } => Block::default()
                .borders(Borders::NONE)
                .padding(*padding)
                .style(theme.style(ThemeElement::BackgroundSurface)),
        }
    }
}

/// Shared list state for selection and virtual scrolling.
#[derive(Debug, Default, Clone)]
pub struct ListState {
    pub selected: usize,
    pub viewport_offset: usize,
}

impl ListState {
    pub fn new() -> Self {
        Self {
            selected: 0,
            viewport_offset: 0,
        }
    }

    /// Clamp selection to `[0, len-1]` if len>0 else 0.
    pub fn clamp_selection(&mut self, len: usize) {
        if len == 0 {
            self.selected = 0;
        } else if self.selected >= len {
            self.selected = len - 1;
        }
    }

    /// Set selected index with clamping by len.
    pub fn set_selected(&mut self, idx: usize, len: usize) {
        self.selected = if len == 0 { 0 } else { idx.min(len - 1) };
    }

    pub fn select_next(&mut self, len: usize) {
        self.selected = select_next(self.selected, len);
    }

    pub fn select_prev(&mut self, len: usize) {
        self.selected = select_prev(self.selected, len);
    }

    pub fn jump_top(&mut self) {
        self.selected = 0;
    }
    pub fn jump_bottom(&mut self, len: usize) {
        if len > 0 {
            self.selected = len - 1;
        }
    }

    /// Scroll by signed lines and clamp selection accordingly.
    pub fn scroll_lines(&mut self, delta: isize, len: usize) {
        if len == 0 {
            self.selected = 0;
            return;
        }
        let cur = self.selected as isize;
        let max = (len - 1) as isize;
        let ns = (cur + delta).clamp(0, max) as usize;
        self.selected = ns;
    }

    /// Page down by visible height.
    pub fn page_down(&mut self, visible_height: usize, len: usize) {
        let step = visible_height.max(1);
        self.scroll_lines(step as isize, len);
    }

    /// Page up by visible height.
    pub fn page_up(&mut self, visible_height: usize, len: usize) {
        let step = visible_height.max(1);
        self.scroll_lines(-(step as isize), len);
    }

    /// Compute and update `viewport_offset`.
    pub fn update_offset(&mut self, visible_height: usize) {
        let h = visible_height.max(1);
        self.viewport_offset =
            calculate_viewport_offset_internal(self.selected, h, self.viewport_offset);
    }
}

/// Calculate viewport offset to ensure selected item is always visible
///
/// UX: anchor based on direction-of-travel semantics inferred from position:
/// - If selected is above the current viewport, anchor to the top (selected at first row).
/// - If selected is below the current viewport, anchor to the bottom (selected at last row).
fn calculate_viewport_offset_internal(
    selected: usize,
    visible_height: usize,
    current_offset: usize,
) -> usize {
    let visible_height = visible_height.max(1);
    if selected < current_offset {
        // Selected is above current viewport → bring it to the top
        selected
    } else if selected >= current_offset + visible_height {
        // Selected is below current viewport → bring it so it appears as last visible
        selected.saturating_sub(visible_height - 1)
    } else {
        // Selected already visible → keep current offset
        current_offset
    }
}

// Note: external callers should use ListState::update_offset instead of manual offset math.

/// Map a mouse position (col,row) to a list index within a bordered list area.
/// Returns `Some(index)` when the point lies inside the list's inner content area,
/// taking into account current selection/viewport. Returns `None` if outside.
#[must_use]
pub fn index_at(
    list_area: Rect,
    selected: usize,
    viewport_offset: usize,
    total: usize,
    col: u16,
    row: u16,
) -> Option<usize> {
    let inner = inner_1px(list_area);
    index_at_content(inner, selected, viewport_offset, total, col, row)
}

/// Map a mouse position (col,row) to a list index within a content area.
/// Returns `Some(index)` when the point lies inside the content area, taking
/// into account current selection/viewport. Returns `None` if outside.
#[must_use]
pub fn index_at_content(
    content_area: Rect,
    selected: usize,
    viewport_offset: usize,
    total: usize,
    col: u16,
    row: u16,
) -> Option<usize> {
    if total == 0 {
        return None;
    }
    if !contains(content_area, col, row) {
        return None;
    }
    let inner_top = content_area.y;
    let hovered_row = row.saturating_sub(inner_top) as usize;
    let visible_h = content_area.height as usize;
    if visible_h == 0 {
        return None;
    }
    let offset = calculate_viewport_offset_internal(selected, visible_h, viewport_offset);
    let idx = (offset + hovered_row).min(total.saturating_sub(1));
    Some(idx)
}

/// Select the next item (wraps to 0 when `selected` is the last one). No-op when len == 0.
#[must_use]
pub fn select_next(selected: usize, len: usize) -> usize {
    if len == 0 {
        return 0;
    }
    (selected + 1) % len
}

/// Select the previous item (wraps to last when `selected` is 0). No-op when len == 0.
#[must_use]
pub fn select_prev(selected: usize, len: usize) -> usize {
    if len == 0 {
        return 0;
    }
    if selected == 0 {
        len - 1
    } else {
        selected - 1
    }
}

/// Generic list rendering with configurable chrome; renders only the visible range.
pub fn render_list_with_chrome<'a, F>(
    frame: &mut Frame,
    area: Rect,
    theme: &Theme,
    chrome: ListChrome,
    selected: usize,
    current_offset: usize,
    len: usize,
    mut render_item: F,
) where
    F: FnMut(usize, bool) -> ListItem<'a>,
{
    let block = chrome.block(theme);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.width == 0 || inner.height == 0 {
        return;
    }

    let visible_height = inner.height.max(1) as usize;
    let offset = calculate_viewport_offset_internal(selected, visible_height, current_offset);
    let end = (offset + visible_height).min(len);

    let mut items = Vec::with_capacity(end.saturating_sub(offset));
    for i in offset..end {
        items.push(render_item(i, i == selected));
    }

    if items.is_empty() && len == 0 {
        items.push(make_list_item_with_element(
            "No items",
            theme,
            ThemeElement::Tertiary,
            false,
            false,
        ));
    }

    let show_scrollbar = len > visible_height && inner.width > 1;
    let mut content_area = inner;
    if show_scrollbar {
        content_area.width = content_area.width.saturating_sub(1);
    }

    let list = List::new(items).style(theme.style(ThemeElement::BackgroundSurface));
    frame.render_widget(list, content_area);

    if show_scrollbar {
        let (thumb_h, thumb_top, _max_scroll) = crate::primitives::scrollbar::compute_thumb(
            inner.height,
            inner.height,
            len as u32,
            offset as u32,
        );
        let bar_x = inner.x + inner.width.saturating_sub(1);
        let style = Style::default().fg(to_ratatui(theme.tertiary));
        let thumb_symbol = "\u{2588}";
        for i in 0..thumb_h.min(inner.height) {
            let y = inner.y + thumb_top + i;
            if y < inner.y.saturating_add(inner.height) {
                frame.buffer_mut()[(bar_x, y)]
                    .set_symbol(thumb_symbol)
                    .set_style(style);
            }
        }
    }
}

/// Convenience wrapper that renders using the bordered chrome with the given title.
pub fn render_list<'a, F>(
    frame: &mut Frame,
    area: Rect,
    theme: &Theme,
    title: impl Into<String>,
    selected: usize,
    current_offset: usize,
    len: usize,
    render_item: F,
) where
    F: FnMut(usize, bool) -> ListItem<'a>,
{
    render_list_with_chrome(
        frame,
        area,
        theme,
        ListChrome::bordered(title.into()),
        selected,
        current_offset,
        len,
        render_item,
    );
}

/// Convenience to build a `ListItem` with stateful style.
#[must_use]
pub fn make_list_item<'a>(
    text: impl Into<String>,
    theme: &Theme,
    selected: bool,
    hovered: bool,
) -> ListItem<'a> {
    let style = crate::theme::list_item_style(theme, ThemeElement::Base, selected, hovered);
    ListItem::new(text.into()).style(style)
}

/// Build a `ListItem` using a specific theme element for its base style,
/// honoring selected/hovered via `list_item_style`.
#[must_use]
pub fn make_list_item_with_element<'a>(
    text: impl Into<String>,
    theme: &Theme,
    element: ThemeElement,
    selected: bool,
    hovered: bool,
) -> ListItem<'a> {
    let style = crate::theme::list_item_style(theme, element, selected, hovered);
    ListItem::new(text.into()).style(style)
}
