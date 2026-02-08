use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Clear, Padding};
use ratatui::Frame;

use crate::theme::{Theme, ThemeElement};

#[derive(Debug, Clone, Copy)]
pub struct OverlayDialogOptions {
    pub padding: Padding,
    pub header_rows: u16,
    pub footer_gap: u16,
    pub footer_rows: u16,
}

impl OverlayDialogOptions {
    pub fn overlay(header_rows: u16, footer_rows: u16) -> Self {
        Self {
            padding: default_padding(),
            header_rows,
            footer_gap: 1,
            footer_rows,
        }
    }

    pub fn chrome_rows(self) -> u16 {
        self.header_rows
            .saturating_add(self.footer_gap)
            .saturating_add(self.footer_rows)
            .saturating_add(self.padding.top)
            .saturating_add(self.padding.bottom)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OverlayDialogLayout {
    pub area: Rect,
    pub header: Rect,
    pub body: Rect,
    pub footer_gap: Rect,
    pub footer: Rect,
}

pub fn default_padding() -> Padding {
    Padding::new(1, 1, 1, 1)
}

pub fn total_height(body_rows: u16, opts: OverlayDialogOptions) -> u16 {
    body_rows.saturating_add(opts.chrome_rows())
}

pub fn compute_layout(area: Rect, opts: OverlayDialogOptions) -> OverlayDialogLayout {
    let block = Block::default().padding(opts.padding);
    let inner = block.inner(area);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(opts.header_rows),
            Constraint::Min(0),
            Constraint::Length(opts.footer_gap),
            Constraint::Length(opts.footer_rows),
        ])
        .split(inner);

    OverlayDialogLayout {
        area,
        header: chunks[0],
        body: chunks[1],
        footer_gap: chunks[2],
        footer: chunks[3],
    }
}

pub fn layout_overlay(
    frame: &mut Frame,
    area: Rect,
    theme: &Theme,
    opts: OverlayDialogOptions,
) -> OverlayDialogLayout {
    let layout = compute_layout(area, opts);
    frame.render_widget(Clear, layout.area);
    let block = Block::default()
        .style(theme.style(ThemeElement::BackgroundSurface))
        .padding(opts.padding);
    frame.render_widget(block, layout.area);
    layout
}
