use ratatui::{
    layout::{Position, Rect},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Paragraph},
    Frame,
};
use unicode_width::UnicodeWidthStr;

use crate::components::text_input::TextInput;
use crate::layout::text::wrapped_row_ranges;
use crate::primitives::shimmer::shimmer_spans;
use crate::theme::{to_ratatui, Color, Theme, ThemeElement};
use std::time::Instant;

/// Geometry info for an input box's internal scrollbar.
#[derive(Debug, Clone, Copy, Default)]
pub struct ScrollbarGeometry {
    pub bar_x: u16,
    pub top: u16,
    pub height: u16,
    pub thumb_top: u16,
    pub thumb_h: u16,
    pub max_scroll: u16,
}

/// Outcome of a render pass, including updated scroll and geometry.
#[derive(Debug, Clone, Default)]
pub struct InputBoxOutcome {
    pub scroll_offset: u16,
    pub scrollbar: Option<ScrollbarGeometry>,
    pub cursor_screen_pos: Position,
}

#[derive(Debug, Clone, Copy)]
pub struct ShimmerSpec {
    pub start: usize,
    pub end: usize,
    pub started_at: Instant,
}

pub struct InputBox<'a> {
    input: &'a TextInput,
    theme: &'a Theme,
    scroll_offset: u16,
    follow_cursor: bool,
    show_scrollbar: bool,
    placeholder_style: Option<ThemeElement>,
    placeholder_color_override: Option<(u8, u8, u8)>,
    placeholder_bold: bool,
    placeholder_shimmer: Option<ShimmerSpec>,
    suggestion: Option<&'a str>,
    suggestion_style: Option<Style>,
    text_style: Option<Style>,
    prompt_override: Option<&'a str>,
    prompt_highlight_style: Option<Style>,
    title_override: Option<&'a str>,
    right_hint: Option<&'a str>,
    cursor_active: bool,
    agent: Option<String>,
    agent_color: Option<(u8, u8, u8)>,
    input_bg: Option<Color>,
    prompt_gap: u16,
    prompt_padding_left: u16,
    padding_top: u16,
    padding_bottom: u16,
}

impl<'a> InputBox<'a> {
    pub fn new(input: &'a TextInput, theme: &'a Theme) -> Self {
        Self {
            input,
            theme,
            scroll_offset: 0,
            follow_cursor: false,
            show_scrollbar: false,
            placeholder_style: None,
            placeholder_color_override: None,
            placeholder_bold: false,
            placeholder_shimmer: None,
            suggestion: None,
            suggestion_style: None,
            text_style: None,
            prompt_override: None,
            prompt_highlight_style: None,
            title_override: None,
            right_hint: None,
            cursor_active: true,
            agent: None,
            agent_color: None,
            input_bg: None,
            prompt_gap: 1,
            prompt_padding_left: 0,
            padding_top: 1,
            padding_bottom: 1,
        }
    }

    pub fn scroll_offset(mut self, val: u16) -> Self {
        self.scroll_offset = val;
        self
    }

    pub fn follow_cursor(mut self, val: bool) -> Self {
        self.follow_cursor = val;
        self
    }

    pub fn show_scrollbar(mut self, val: bool) -> Self {
        self.show_scrollbar = val;
        self
    }

    pub fn placeholder_style(mut self, val: impl Into<Option<ThemeElement>>) -> Self {
        self.placeholder_style = val.into();
        self
    }

    pub fn placeholder_color_override(mut self, val: impl Into<Option<(u8, u8, u8)>>) -> Self {
        self.placeholder_color_override = val.into();
        self
    }

    pub fn placeholder_bold(mut self, val: bool) -> Self {
        self.placeholder_bold = val;
        self
    }

    pub fn placeholder_shimmer(mut self, val: impl Into<Option<ShimmerSpec>>) -> Self {
        self.placeholder_shimmer = val.into();
        self
    }

    pub fn suggestion(mut self, val: impl Into<Option<&'a str>>, style: Style) -> Self {
        self.suggestion = val.into();
        self.suggestion_style = Some(style);
        self
    }

    pub fn text_style(mut self, val: impl Into<Option<Style>>) -> Self {
        self.text_style = val.into();
        self
    }

    pub fn prompt_override(mut self, val: impl Into<Option<&'a str>>) -> Self {
        self.prompt_override = val.into();
        self
    }

    pub fn prompt_highlight_style(mut self, val: impl Into<Option<Style>>) -> Self {
        self.prompt_highlight_style = val.into();
        self
    }

    pub fn title_override(mut self, val: impl Into<Option<&'a str>>) -> Self {
        self.title_override = val.into();
        self
    }

    pub fn prompt_gap(mut self, val: u16) -> Self {
        self.prompt_gap = val;
        self
    }

    pub fn prompt_padding_left(mut self, val: u16) -> Self {
        self.prompt_padding_left = val;
        self
    }

    pub fn padding_top(mut self, val: u16) -> Self {
        self.padding_top = val;
        self
    }

    pub fn padding_bottom(mut self, val: u16) -> Self {
        self.padding_bottom = val;
        self
    }

    pub fn right_hint(mut self, val: impl Into<Option<&'a str>>) -> Self {
        self.right_hint = val.into();
        self
    }

    pub fn cursor_active(mut self, val: bool) -> Self {
        self.cursor_active = val;
        self
    }

    pub fn agent(mut self, val: impl Into<Option<String>>) -> Self {
        self.agent = val.into();
        self
    }

    pub fn agent_color(mut self, val: impl Into<Option<(u8, u8, u8)>>) -> Self {
        self.agent_color = val.into();
        self
    }

    pub fn input_bg(mut self, val: impl Into<Option<Color>>) -> Self {
        self.input_bg = val.into();
        self
    }

    pub fn render(self, frame: &mut Frame, area: Rect) -> InputBoxOutcome {
        let frame_area = frame.area();
        if frame_area.width == 0 || frame_area.height == 0 || area.width == 0 || area.height == 0 {
            return InputBoxOutcome {
                scroll_offset: self.scroll_offset,
                scrollbar: None,
                cursor_screen_pos: Position::new(frame_area.x, frame_area.y),
            };
        }

        let bg_color = self.input_bg.unwrap_or(self.theme.background_surface);
        let base_style = self
            .theme
            .style(ThemeElement::Base)
            .bg(to_ratatui(bg_color));
        let text_style = self
            .text_style
            .map(|style| style.bg(to_ratatui(bg_color)))
            .unwrap_or(base_style);
        frame.render_widget(Block::default().style(base_style), area);

        if let Some(title) = self.title_override {
            if area.height >= 3 {
                let title_style = self
                    .theme
                    .style(ThemeElement::Secondary)
                    .add_modifier(Modifier::BOLD);
                let title_area = Rect {
                    x: area.x,
                    y: area.y,
                    width: area.width,
                    height: 1,
                };
                frame.render_widget(
                    Paragraph::new(Line::from(Span::styled(title, title_style))).style(base_style),
                    title_area,
                );
            }
        }

        let prompt = self.prompt_override.unwrap_or_else(|| self.input.prefix());
        let prompt_cols =
            UnicodeWidthStr::width(prompt) as u16 + self.prompt_gap + self.prompt_padding_left;

        let pad_top = self.padding_top.min(area.height);
        let pad_bottom = self.padding_bottom.min(area.height.saturating_sub(pad_top));
        let inner_area = if area.height > pad_top.saturating_add(pad_bottom) {
            Rect {
                x: area.x,
                y: area.y.saturating_add(pad_top),
                width: area.width,
                height: area
                    .height
                    .saturating_sub(pad_top)
                    .saturating_sub(pad_bottom),
            }
        } else {
            area
        };

        let content_area = Rect {
            x: inner_area.x.saturating_add(prompt_cols),
            y: inner_area.y,
            width: inner_area.width.saturating_sub(prompt_cols).max(1),
            height: inner_area.height,
        };

        let content_width = content_area.width.max(1);
        let display_text = self.input.text();
        let use_placeholder = display_text.is_empty() && self.input.placeholder().is_some();
        let suggestion = self
            .suggestion
            .and_then(|value| (!value.is_empty()).then_some(value));

        let content = if use_placeholder {
            self.input.placeholder().unwrap_or_default()
        } else {
            display_text
        };
        let (rows, used_placeholder) = if use_placeholder {
            let shimmer = self.placeholder_shimmer.map(|spec| {
                let len = content.len();
                let start = spec.start.min(len);
                let end = spec.end.min(len);
                ShimmerSpec {
                    start,
                    end,
                    started_at: spec.started_at,
                }
            });
            if let Some(shimmer) = shimmer.filter(|spec| spec.start < spec.end) {
                (
                    build_placeholder_shimmer_lines(
                        content,
                        content_width,
                        shimmer,
                        self.theme,
                        bg_color,
                        self.placeholder_style,
                        self.placeholder_color_override,
                        self.placeholder_bold,
                    ),
                    true,
                )
            } else {
                (build_text_lines(content, content_width), true)
            }
        } else if let Some(suggestion) = suggestion {
            let style = self.suggestion_style.unwrap_or_else(|| {
                self.theme
                    .style(ThemeElement::Secondary)
                    .bg(to_ratatui(bg_color))
                    .add_modifier(Modifier::DIM)
            });
            (
                build_suggestion_lines(display_text, suggestion, style, content_width),
                false,
            )
        } else {
            (build_text_lines(content, content_width), false)
        };

        let total_rows = rows.len().max(1) as u16;

        let cursor_pos = self.input.cursor_visual_position(content_width);
        let max_scroll = total_rows.saturating_sub(content_area.height);
        let mut scroll = self.scroll_offset.min(max_scroll);
        if self.follow_cursor {
            if cursor_pos.0 < scroll {
                scroll = cursor_pos.0;
            } else if cursor_pos.0 >= scroll.saturating_add(content_area.height) {
                scroll = cursor_pos
                    .0
                    .saturating_sub(content_area.height.saturating_sub(1));
            }
            scroll = scroll.min(max_scroll);
        }

        let visible_rows: Vec<Line<'static>> = rows
            .into_iter()
            .skip(scroll as usize)
            .take(content_area.height as usize)
            .collect();

        let mut text = Text::from(visible_rows);
        if used_placeholder && self.placeholder_shimmer.is_none() {
            let style = placeholder_style(
                self.theme,
                self.placeholder_style,
                self.placeholder_color_override,
                self.placeholder_bold,
                bg_color,
            );
            text = text.style(style);
        } else if used_placeholder {
            text = text.style(base_style);
        } else {
            text = text.style(text_style);
        }

        let paragraph = Paragraph::new(text);
        frame.render_widget(paragraph, content_area);

        render_prompt(
            frame,
            self.theme,
            inner_area,
            prompt,
            self.prompt_padding_left,
            self.prompt_gap,
            bg_color,
            self.agent.as_deref(),
            self.agent_color,
            self.prompt_highlight_style,
        );

        if let Some(hint) = self.right_hint {
            render_hint(
                frame,
                self.theme,
                content_area,
                hint,
                bg_color,
                self.show_scrollbar,
            );
        }

        let scrollbar = if self.show_scrollbar {
            Some(render_scrollbar(
                frame,
                content_area,
                scroll,
                total_rows,
                bg_color,
                self.theme,
            ))
        } else {
            None
        };

        let cursor_screen_pos = cursor_screen_pos(content_area, cursor_pos, scroll);
        if self.cursor_active {
            frame.set_cursor_position(cursor_screen_pos);
        }

        InputBoxOutcome {
            scroll_offset: scroll,
            scrollbar,
            cursor_screen_pos,
        }
    }
}

fn build_text_lines(text: &str, content_width: u16) -> Vec<Line<'static>> {
    if text.is_empty() {
        return vec![Line::from("")];
    }

    let ranges = wrapped_row_ranges(text, content_width);
    ranges
        .into_iter()
        .map(|(start, end)| Line::raw(text[start..end].to_string()))
        .collect()
}

fn build_suggestion_lines(
    text: &str,
    suggestion: &str,
    suggestion_style: Style,
    content_width: u16,
) -> Vec<Line<'static>> {
    let full_text = format!("{}{}", text, suggestion);
    if full_text.is_empty() {
        return vec![Line::from("")];
    }
    let row_ranges = wrapped_row_ranges(&full_text, content_width);
    let text_len = text.len();

    row_ranges
        .into_iter()
        .map(|(start, end)| {
            if end <= text_len {
                Line::raw(full_text[start..end].to_string())
            } else if start >= text_len {
                Line::styled(full_text[start..end].to_string(), suggestion_style)
            } else {
                let text_part = &full_text[start..text_len];
                let sugg_part = &full_text[text_len..end];
                Line::from(vec![
                    Span::raw(text_part.to_string()),
                    Span::styled(sugg_part.to_string(), suggestion_style),
                ])
            }
        })
        .collect()
}

fn build_placeholder_shimmer_lines(
    text: &str,
    content_width: u16,
    shimmer: ShimmerSpec,
    theme: &Theme,
    bg_color: Color,
    placeholder_style_elem: Option<ThemeElement>,
    placeholder_color_override: Option<(u8, u8, u8)>,
    placeholder_bold: bool,
) -> Vec<Line<'static>> {
    if text.is_empty() {
        return vec![Line::from("")];
    }
    let base_style = placeholder_style(
        theme,
        placeholder_style_elem,
        placeholder_color_override,
        placeholder_bold,
        bg_color,
    );
    let line_ranges = wrapped_row_ranges(text, content_width);
    let mut lines = Vec::with_capacity(line_ranges.len());
    let bg = to_ratatui(bg_color);

    for (line_start, line_end) in line_ranges {
        let shimmer_start = shimmer.start.max(line_start);
        let shimmer_end = shimmer.end.min(line_end);
        let mut spans = Vec::new();

        if line_start < shimmer_start {
            spans.push(Span::styled(
                text[line_start..shimmer_start].to_string(),
                base_style,
            ));
        }
        if shimmer_start < shimmer_end {
            spans.extend(
                shimmer_spans(
                    &text[shimmer_start..shimmer_end],
                    shimmer.started_at,
                    theme.secondary,
                    theme.primary,
                )
                .into_iter()
                .map(|mut span| {
                    span.style = span.style.bg(bg);
                    span
                }),
            );
        }
        if shimmer_end < line_end {
            spans.push(Span::styled(
                text[shimmer_end..line_end].to_string(),
                base_style,
            ));
        }

        lines.push(Line::from(spans));
    }

    lines
}

fn placeholder_style(
    theme: &Theme,
    placeholder_style: Option<ThemeElement>,
    placeholder_color_override: Option<(u8, u8, u8)>,
    placeholder_bold: bool,
    bg: Color,
) -> Style {
    let mut style = if let Some(rgb) = placeholder_color_override {
        Style::default().fg(to_ratatui(Color::Rgb {
            r: rgb.0,
            g: rgb.1,
            b: rgb.2,
        }))
    } else {
        theme.style(placeholder_style.unwrap_or(ThemeElement::Tertiary))
    };
    style = style.bg(to_ratatui(bg));
    style = style.add_modifier(if placeholder_bold {
        Modifier::BOLD
    } else {
        Modifier::DIM
    });
    style
}

fn render_prompt(
    frame: &mut Frame,
    theme: &Theme,
    area: Rect,
    prompt: &str,
    padding_left: u16,
    gap: u16,
    bg: Color,
    agent: Option<&str>,
    agent_color: Option<(u8, u8, u8)>,
    prompt_highlight_style: Option<Style>,
) {
    if prompt.is_empty() || area.height == 0 || area.width == 0 {
        return;
    }
    let prompt_style = if let Some(agent) = agent {
        let agent_color = theme.agent_color(agent, agent_color);
        Style::default()
            .fg(to_ratatui(agent_color))
            .bg(to_ratatui(bg))
            .add_modifier(Modifier::BOLD)
    } else {
        theme
            .style(ThemeElement::Primary)
            .bg(to_ratatui(bg))
            .add_modifier(Modifier::BOLD)
    };

    let prompt_str = format!(
        "{}{}{}",
        " ".repeat(padding_left as usize),
        prompt,
        " ".repeat(gap as usize)
    );

    let max_x = area.x.saturating_add(area.width);
    let y = area.y;
    let buf = frame.buffer_mut();
    let highlight_idx = prompt_str.chars().position(|ch| !ch.is_whitespace());
    for (i, ch) in prompt_str.chars().enumerate() {
        let x = area.x.saturating_add(i as u16);
        if x >= max_x {
            break;
        }
        let style = if Some(i) == highlight_idx {
            prompt_highlight_style.unwrap_or(prompt_style)
        } else {
            prompt_style
        };
        buf[(x, y)].set_symbol(&ch.to_string()).set_style(style);
    }
}

fn render_hint(
    frame: &mut Frame,
    theme: &Theme,
    content_area: Rect,
    hint: &str,
    bg: Color,
    has_scrollbar: bool,
) {
    if hint.is_empty() || content_area.width == 0 || content_area.height == 0 {
        return;
    }
    let hint_cols = UnicodeWidthStr::width(hint) as u16;
    let reserved = if has_scrollbar { 1 } else { 0 };
    let available = content_area.width.saturating_sub(reserved);
    if hint_cols == 0 || hint_cols > available {
        return;
    }
    let start_x = content_area.x.saturating_add(available - hint_cols);
    let y = content_area.y;
    let max_x = content_area.x.saturating_add(content_area.width);
    let buf = frame.buffer_mut();
    let style = theme
        .style(ThemeElement::Info)
        .bg(to_ratatui(bg))
        .add_modifier(Modifier::DIM)
        .add_modifier(Modifier::BOLD)
        .add_modifier(Modifier::ITALIC);

    for (i, ch) in hint.chars().enumerate() {
        let x = start_x.saturating_add(i as u16);
        if x >= max_x {
            break;
        }
        buf[(x, y)].set_symbol(&ch.to_string()).set_style(style);
    }
}

fn render_scrollbar(
    frame: &mut Frame,
    area: Rect,
    scroll: u16,
    total_rows: u16,
    bg: Color,
    theme: &Theme,
) -> ScrollbarGeometry {
    let height = area.height.max(1);
    let max_scroll = total_rows.saturating_sub(height);
    let bar_x = area.x.saturating_add(area.width.saturating_sub(1));

    let thumb_h = if total_rows == 0 {
        height
    } else {
        let raw = height.saturating_mul(height) / total_rows.max(1);
        raw.max(1)
    };
    let track_h = height.saturating_sub(thumb_h);
    let thumb_top = if max_scroll == 0 {
        0
    } else {
        ((scroll as u32 * track_h as u32) / max_scroll as u32) as u16
    };

    let track_style = theme
        .style(ThemeElement::BackgroundTrack)
        .bg(to_ratatui(bg));
    let thumb_style = theme
        .style(ThemeElement::BackgroundThumb)
        .bg(to_ratatui(bg));
    for y in 0..height {
        let cell_style = if y >= thumb_top && y < thumb_top.saturating_add(thumb_h) {
            thumb_style
        } else {
            track_style
        };
        frame.buffer_mut()[(bar_x, area.y.saturating_add(y))]
            .set_symbol(" ")
            .set_style(cell_style);
    }

    ScrollbarGeometry {
        bar_x,
        top: area.y,
        height,
        thumb_top: area.y.saturating_add(thumb_top),
        thumb_h,
        max_scroll,
    }
}

fn cursor_screen_pos(content_area: Rect, cursor: (u16, u16), scroll: u16) -> Position {
    let row = cursor
        .0
        .saturating_sub(scroll)
        .min(content_area.height.saturating_sub(1));
    let col = cursor.1.min(content_area.width.saturating_sub(1));
    Position::new(
        content_area.x.saturating_add(col),
        content_area.y.saturating_add(row),
    )
}
