use ratatui::layout::Rect;

/// Returns true if (col,row) lies within the rectangle (inclusive of top-left, exclusive of bottom-right)
#[inline]
pub fn contains(r: Rect, col: u16, row: u16) -> bool {
    col >= r.x && col < r.x + r.width && row >= r.y && row < r.y + r.height
}

/// Returns the inner rect by removing a 1px border on all sides, saturating at zero.
#[inline]
pub fn inner_1px(r: Rect) -> Rect {
    Rect {
        x: r.x.saturating_add(1),
        y: r.y.saturating_add(1),
        width: r.width.saturating_sub(2),
        height: r.height.saturating_sub(2),
    }
}
