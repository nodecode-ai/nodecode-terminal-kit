use crossterm::event::KeyEvent;
use nodecode_terminal_kit::theme::Theme;
use ratatui::layout::Rect;
use ratatui::Frame;

use crate::Command;

/// Runtime contract for terminal app state transitions and rendering.
pub trait Model {
    type Msg: Send + 'static;

    /// Optional startup command executed once before the event loop starts.
    fn init(&mut self) -> Command<Self::Msg> {
        Command::none()
    }

    /// Apply a message to state and optionally emit follow-up command(s).
    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg>;

    /// Render the full application content inside the provided area.
    fn view(&self, frame: &mut Frame, area: Rect, theme: &Theme);

    /// Optional key handler that maps keyboard events to messages.
    fn on_key(&mut self, _key: KeyEvent) -> Option<Self::Msg> {
        None
    }
}
