use std::time::Instant;

use ratatui::style::{Modifier, Style};
use ratatui::text::Span;

use crate::theme::{to_ratatui, Color};

fn color_to_rgb(color: Color) -> Option<(u8, u8, u8)> {
    match color {
        Color::Rgb { r, g, b } => Some((r, g, b)),
        Color::Indexed(_) => None,
    }
}

fn blend(fg: (u8, u8, u8), bg: (u8, u8, u8), alpha: f32) -> (u8, u8, u8) {
    let alpha = alpha.clamp(0.0, 1.0);
    let r = (fg.0 as f32 * alpha + bg.0 as f32 * (1.0 - alpha)) as u8;
    let g = (fg.1 as f32 * alpha + bg.1 as f32 * (1.0 - alpha)) as u8;
    let b = (fg.2 as f32 * alpha + bg.2 as f32 * (1.0 - alpha)) as u8;
    (r, g, b)
}

fn style_for_level(intensity: f32) -> Style {
    if intensity < 0.2 {
        Style::default().add_modifier(Modifier::DIM)
    } else if intensity < 0.6 {
        Style::default()
    } else {
        Style::default().add_modifier(Modifier::BOLD)
    }
}

/// Create per-character ratatui spans that animate a shimmer band across `text`.
///
/// Ported from Codex's TUI shimmer implementation.
pub fn shimmer_spans(
    text: &str,
    started_at: Instant,
    base: Color,
    highlight: Color,
) -> Vec<Span<'static>> {
    let chars: Vec<char> = text.chars().collect();
    if chars.is_empty() {
        return Vec::new();
    }

    // Use time-based sweep synchronized to a caller-provided start time.
    let padding = 10usize;
    let period = chars.len() + padding * 2;
    let sweep_seconds = 2.0f32;
    let pos_f =
        (started_at.elapsed().as_secs_f32() % sweep_seconds) / sweep_seconds * period as f32;
    let pos = pos_f as isize;
    let band_half_width = 8.0f32;

    let base_rgb = color_to_rgb(base);
    let highlight_rgb = color_to_rgb(highlight);

    let mut spans = Vec::with_capacity(chars.len());
    for (i, ch) in chars.iter().enumerate() {
        let i_pos = i as isize + padding as isize;
        let dist = (i_pos - pos).abs() as f32;

        let t = if dist <= band_half_width {
            let x = std::f32::consts::PI * (dist / band_half_width);
            0.5 * (1.0 + x.cos())
        } else {
            0.0
        };

        let style = match (base_rgb, highlight_rgb) {
            (Some(base_rgb), Some(highlight_rgb)) => {
                let alpha = t.clamp(0.0, 1.0);
                let (r, g, b) = blend(highlight_rgb, base_rgb, alpha);
                Style::default()
                    .fg(to_ratatui(Color::Rgb { r, g, b }))
                    .add_modifier(Modifier::BOLD)
            }
            _ => Style::default()
                .fg(to_ratatui(base))
                .patch(style_for_level(t)),
        };

        spans.push(Span::styled(ch.to_string(), style));
    }

    spans
}
