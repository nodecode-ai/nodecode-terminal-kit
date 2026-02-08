use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Clear, Padding},
    Frame,
};

use std::sync::atomic::{AtomicBool, Ordering};

use crate::theme::Theme;
use crate::theme::ThemeElement;

#[inline]
fn center_position(container_size: u16, item_size: u16) -> u16 {
    container_size.saturating_sub(item_size) / 2
}

#[inline]
fn calculate_dialog_area_clamped(
    parent: Rect,
    width_pct: f32,
    height_pct: f32,
    min_width: u16,
    max_width: u16,
    min_height: u16,
    max_height: u16,
) -> Rect {
    let dialog_width = ((parent.width as f32 * width_pct) as u16).clamp(min_width, max_width);
    let dialog_height = ((parent.height as f32 * height_pct) as u16).clamp(min_height, max_height);
    let dialog_x = center_position(parent.width, dialog_width);
    let dialog_y = center_position(parent.height, dialog_height);

    Rect {
        x: parent.x.saturating_add(dialog_x),
        y: parent.y.saturating_add(dialog_y),
        width: dialog_width,
        height: dialog_height,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DialogOptions {
    pub width_pct: f32,
    pub height_pct: f32,
    pub max_width: u16,
    pub max_height: u16,
    pub header_rows: u16,
    pub footer_rows: u16,
    pub padding: Padding,
}

impl Default for DialogOptions {
    fn default() -> Self {
        Self {
            width_pct: 1.0,
            height_pct: 1.0,
            max_width: u16::MAX,
            max_height: u16::MAX,
            header_rows: 0,
            footer_rows: 0,
            padding: Padding::new(0, 0, 0, 0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DialogLayout {
    pub area: Rect,
    pub header: Rect,
    pub body: Rect,
    pub footer: Rect,
}

static INLINE_DIALOG_MODE: AtomicBool = AtomicBool::new(false);

/// Guard that enables inline dialog sizing overrides for the duration of a render pass.
pub struct InlineDialogGuard {
    prev: bool,
}

impl InlineDialogGuard {
    pub fn enable() -> Self {
        let prev = INLINE_DIALOG_MODE.swap(true, Ordering::SeqCst);
        Self { prev }
    }
}

impl Drop for InlineDialogGuard {
    fn drop(&mut self) {
        INLINE_DIALOG_MODE.store(self.prev, Ordering::SeqCst);
    }
}

#[inline]
fn apply_inline_overrides(mut opts: DialogOptions, parent: Rect) -> DialogOptions {
    if INLINE_DIALOG_MODE.load(Ordering::SeqCst) {
        // Inline viewport dialogs should expand to the full available width.
        opts.width_pct = 1.0;
        opts.max_width = parent.width;
    }
    opts
}

fn apply_padding(area: Rect, padding: Padding) -> Rect {
    let horizontal = padding.left.saturating_add(padding.right);
    let vertical = padding.top.saturating_add(padding.bottom);

    Rect {
        x: area.x.saturating_add(padding.left),
        y: area.y.saturating_add(padding.top),
        width: area.width.saturating_sub(horizontal),
        height: area.height.saturating_sub(vertical),
    }
}

/// Compute a centered dialog layout (no rendering). Useful for hit-testing/mouse logic.
#[must_use]
pub fn compute_centered(parent: Rect, opts: DialogOptions) -> DialogLayout {
    let opts = apply_inline_overrides(opts, parent);
    let area = calculate_dialog_area_clamped(
        parent,
        opts.width_pct,
        opts.height_pct,
        0,
        opts.max_width,
        0,
        opts.max_height,
    );
    let padded = apply_padding(area, opts.padding);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(opts.header_rows),
            Constraint::Min(0),
            Constraint::Length(opts.footer_rows),
        ])
        .split(padded);
    DialogLayout {
        area,
        header: chunks[0],
        body: chunks[1],
        footer: chunks[2],
    }
}

/// Compute a centered dialog layout, then clear and paint a solid background matching the theme.
pub fn layout_centered(
    frame: &mut Frame,
    parent: Rect,
    theme: &Theme,
    opts: DialogOptions,
) -> DialogLayout {
    let layout = compute_centered(parent, opts);
    // Clear and paint a raised background
    frame.render_widget(Clear, layout.area);
    let bg = Block::default().style(theme.style(ThemeElement::BackgroundSurface));
    frame.render_widget(bg, layout.area);
    layout
}

/// Compute a centered dialog layout with a border and title.
/// The footer is placed outside the dialog, below it.
pub fn layout_centered_bordered(
    frame: &mut Frame,
    parent: Rect,
    theme: &Theme,
    opts: DialogOptions,
    title: &str,
) -> DialogLayout {
    // Compute dialog area without footer
    let opts = apply_inline_overrides(opts, parent);
    let dialog_opts = DialogOptions {
        footer_rows: 0,
        ..opts
    };
    let dialog_area = calculate_dialog_area_clamped(
        parent,
        dialog_opts.width_pct,
        dialog_opts.height_pct,
        0,
        dialog_opts.max_width,
        0,
        dialog_opts.max_height,
    );

    // Clear and render bordered block
    frame.render_widget(Clear, dialog_area);
    let block = Block::default()
        .borders(ratatui::widgets::Borders::ALL)
        .border_type(theme.border_type)
        .title(title)
        .border_style(theme.border_focused_style())
        .style(theme.style(ThemeElement::BackgroundSurface));
    let inner = block.inner(dialog_area);
    frame.render_widget(block, dialog_area);

    // Split inner area into header/body only
    let padded_inner = apply_padding(inner, opts.padding);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(opts.header_rows), Constraint::Min(0)])
        .split(padded_inner);

    // Calculate footer area below the dialog
    let footer = Rect {
        x: padded_inner.x,
        y: dialog_area.y.saturating_add(dialog_area.height),
        width: padded_inner.width,
        height: opts.footer_rows,
    };

    DialogLayout {
        area: dialog_area,
        header: chunks[0],
        body: chunks[1],
        footer,
    }
}

/// Compute a centered dialog layout with a border and title.
/// The footer is placed INSIDE the dialog border (at the bottom).
pub fn layout_centered_bordered_contained(
    frame: &mut Frame,
    parent: Rect,
    theme: &Theme,
    opts: DialogOptions,
    title: &str,
) -> DialogLayout {
    // Use standard calculation
    let opts = apply_inline_overrides(opts, parent);
    let dialog_area = calculate_dialog_area_clamped(
        parent,
        opts.width_pct,
        opts.height_pct,
        0,
        opts.max_width,
        0,
        opts.max_height,
    );

    // Clear and render bordered block
    frame.render_widget(Clear, dialog_area);
    let block = Block::default()
        .borders(ratatui::widgets::Borders::ALL)
        .border_type(theme.border_type)
        .title(title)
        .border_style(theme.border_focused_style())
        .style(theme.style(ThemeElement::BackgroundSurface));
    let inner = block.inner(dialog_area);
    frame.render_widget(block, dialog_area);

    // Split inner area into header/body/footer
    let padded_inner = apply_padding(inner, opts.padding);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(opts.header_rows),
            Constraint::Min(0),
            Constraint::Length(opts.footer_rows),
        ])
        .split(padded_inner);

    DialogLayout {
        area: dialog_area,
        header: chunks[0],
        body: chunks[1],
        footer: chunks[2],
    }
}

/// Compute layout inside a fixed rect (no centering), split into header/body/footer without rendering.
#[must_use]
pub fn compute_fixed(area: Rect, header_rows: u16, footer_rows: u16) -> DialogLayout {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(header_rows),
            Constraint::Min(0),
            Constraint::Length(footer_rows),
        ])
        .split(area);
    DialogLayout {
        area,
        header: chunks[0],
        body: chunks[1],
        footer: chunks[2],
    }
}

/// Paint background for a fixed rect and return a split layout for convenience.
pub fn layout_fixed(
    frame: &mut Frame,
    area: Rect,
    theme: &Theme,
    header_rows: u16,
    footer_rows: u16,
) -> DialogLayout {
    let layout = compute_fixed(area, header_rows, footer_rows);
    frame.render_widget(Clear, layout.area);
    let bg = Block::default().style(theme.style(ThemeElement::BackgroundSurface));
    frame.render_widget(bg, layout.area);
    layout
}

/// Just clear and paint background for the given rect.
pub fn paint_background(frame: &mut Frame, area: Rect, theme: &Theme) {
    frame.render_widget(Clear, area);
    let bg = Block::default().style(theme.style(ThemeElement::BackgroundSurface));
    frame.render_widget(bg, area);
}
