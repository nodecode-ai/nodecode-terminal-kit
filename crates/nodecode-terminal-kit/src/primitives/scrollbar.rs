/// Compute scrollbar thumb geometry for a vertical scroller.
///
/// Inputs:
/// - `track_h`: drawable track height in rows (e.g., inner height minus any reserved rows)
/// - `visible_rows`: number of visible content rows
/// - `total_rows`: total content rows
/// - `scroll`: current scroll offset in rows (topmost visible row index)
///
/// Returns `(thumb_h, thumb_top, max_scroll)` where:
/// - `thumb_h` is at least 1
/// - `thumb_top` is within `[0, track_h - thumb_h]`
/// - `max_scroll` is `total_rows.saturating_sub(visible_rows as u32)`
#[must_use]
pub fn compute_thumb(
    track_h: u16,
    visible_rows: u16,
    total_rows: u32,
    scroll: u32,
) -> (u16, u16, u32) {
    let max_scroll = total_rows.saturating_sub(visible_rows as u32);
    let thumb_h = if total_rows == 0 {
        1
    } else {
        ((track_h as u32 * visible_rows as u32) / total_rows).max(1) as u16
    };
    let max_thumb_top = track_h.saturating_sub(thumb_h);
    let thumb_top = if max_scroll == 0 {
        0
    } else {
        ((scroll.min(max_scroll) * max_thumb_top as u32) / max_scroll) as u16
    };
    (thumb_h, thumb_top, max_scroll)
}

// ---- Consolidated helpers for scrollbar dragging ----

#[inline]
pub fn start_drag(my: u16, thumb_top: u16, thumb_h: u16) -> (bool, u16) {
    let within_thumb = my >= thumb_top && my < thumb_top.saturating_add(thumb_h);
    let grab_offset = if within_thumb {
        my.saturating_sub(thumb_top)
    } else {
        thumb_h / 2
    };
    (within_thumb, grab_offset)
}

#[inline]
pub fn desired_from_click(
    my: u16,
    top: u16,
    height: u16,
    grab_offset: u16,
    thumb_h: u16,
) -> (u16, u16) {
    let max_thumb_top = height.saturating_sub(thumb_h);
    let desired = my
        .saturating_sub(top)
        .saturating_sub(grab_offset)
        .min(max_thumb_top);
    (desired, max_thumb_top)
}

#[inline]
pub fn map_thumb_to_scroll_u32(thumb_pos: u16, max_thumb_top: u16, max_scroll: u32) -> u32 {
    if max_thumb_top == 0 || max_scroll == 0 {
        0
    } else {
        ((thumb_pos as u32) * max_scroll) / (max_thumb_top as u32)
    }
}

#[inline]
pub fn map_thumb_to_scroll_u16(thumb_pos: u16, max_thumb_top: u16, max_scroll: u16) -> u16 {
    if max_thumb_top == 0 || max_scroll == 0 {
        0
    } else {
        (((thumb_pos as u32) * (max_scroll as u32)) / (max_thumb_top as u32)) as u16
    }
}

use ratatui::layout::Rect;

/// Compute the vertical scrollbar bar position and track height inside the chat area.
/// If `reserve_bottom_row` is true, leaves one extra row at the bottom (for overlays).
#[inline]
pub fn bar_geometry(area: Rect, reserve_bottom_row: bool) -> (u16, u16, u16) {
    let bar_x = area.x + area.width.saturating_sub(1);
    let top = area.y;
    let mut height = area.height;
    if reserve_bottom_row {
        height = height.saturating_sub(1);
    }
    (bar_x, top, height)
}

/// Compute internal input-box scrollbar geometry inside the input content area.
/// Returns (bar_x, top, height, thumb_top_abs, thumb_h, max_scroll) if a scrollbar is needed.
#[inline]
pub fn internal_input_geometry(
    area: Rect,
    content_h: u16,
    total_rows: u16,
    scroll: u16,
) -> Option<(u16, u16, u16, u16, u16, u16)> {
    if total_rows <= content_h || area.height == 0 || area.width == 0 {
        return None;
    }
    let height = content_h.min(area.height);
    if height == 0 {
        return None;
    }
    let bar_x = area.x + area.width.saturating_sub(1);
    let top = area.y;
    let (thumb_h, thumb_top, _max_scroll) =
        compute_thumb(height, content_h, total_rows as u32, scroll as u32);
    let max_scroll = total_rows.saturating_sub(content_h);
    Some((bar_x, top, height, top + thumb_top, thumb_h, max_scroll))
}
