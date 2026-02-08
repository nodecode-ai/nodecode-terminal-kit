use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::Widget;
use ratatui::Frame;

/// Lightweight renderer for a pre-wrapped slice of lines.
///
/// This avoids cloning the full `Vec<Line>` for every frame/scroll tick. The caller is expected
/// to pass only the visible viewport slice (e.g. `[start..start + height]`).
pub struct LinesViewport<'a> {
    lines: &'a [Line<'static>],
    style: Style,
}

impl<'a> LinesViewport<'a> {
    pub fn new(lines: &'a [Line<'static>], style: Style) -> Self {
        Self { lines, style }
    }
}

impl Widget for LinesViewport<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = area.intersection(buf.area);
        if area.is_empty() {
            return;
        }

        buf.set_style(area, self.style);

        let area_width = area.width;
        let max_rows = area.height as usize;

        for (row, line) in self.lines.iter().take(max_rows).enumerate() {
            let alignment = line.alignment.unwrap_or(Alignment::Left);
            let line_width = line.width().min(area_width as usize) as u16;
            let x_offset = match alignment {
                Alignment::Left => 0,
                Alignment::Center => (area_width / 2).saturating_sub(line_width / 2),
                Alignment::Right => area_width.saturating_sub(line_width),
            };

            let max_width = area_width.saturating_sub(x_offset);
            if max_width == 0 {
                continue;
            }

            buf.set_line(
                area.x.saturating_add(x_offset),
                area.y.saturating_add(row as u16),
                line,
                max_width,
            );
        }
    }
}

pub fn render_lines_slice(
    frame: &mut Frame,
    area: Rect,
    base_style: Style,
    history_lines: &[Line<'static>],
    tail_lines: &[Line<'static>],
    start_u32: u32,
) {
    let start_idx = start_u32 as usize;
    let history_len = history_lines.len();
    let mut remaining = area.height as usize;
    let mut y = area.y;

    if start_idx < history_len {
        let end_history = (start_idx + remaining).min(history_len);
        let slice = &history_lines[start_idx..end_history];
        let h = slice.len() as u16;
        if h > 0 {
            frame.render_widget(
                LinesViewport::new(slice, base_style),
                Rect {
                    x: area.x,
                    y,
                    width: area.width,
                    height: h,
                },
            );
            y = y.saturating_add(h);
            remaining = remaining.saturating_sub(slice.len());
        }

        if remaining > 0 {
            let end_tail = remaining.min(tail_lines.len());
            let slice = &tail_lines[..end_tail];
            let h = slice.len() as u16;
            if h > 0 {
                frame.render_widget(
                    LinesViewport::new(slice, base_style),
                    Rect {
                        x: area.x,
                        y,
                        width: area.width,
                        height: h,
                    },
                );
            }
        }
    } else {
        let tail_start = start_idx.saturating_sub(history_len);
        let end_tail = (tail_start + remaining).min(tail_lines.len());
        let slice = &tail_lines[tail_start..end_tail];
        frame.render_widget(LinesViewport::new(slice, base_style), area);
    }
}
