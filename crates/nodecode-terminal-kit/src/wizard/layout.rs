use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Standard vertical split used by most single-input wizard steps:
/// 1. Top padding (1)
/// 2. Input box (3)
/// 3. Validation/status line (2)
/// 4. Filler
#[must_use]
pub fn input_step_layout(area: Rect) -> [Rect; 4] {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Min(0),
        ])
        .split(area);
    [chunks[0], chunks[1], chunks[2], chunks[3]]
}

/// Vertical split with a single blank line followed by the main body.
#[must_use]
pub fn padded_list_layout(area: Rect) -> [Rect; 2] {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(0)])
        .split(area);
    [chunks[0], chunks[1]]
}
