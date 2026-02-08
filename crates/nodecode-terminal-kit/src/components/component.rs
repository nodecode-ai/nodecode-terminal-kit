//! Design-only component contracts.
//!
//! These traits define reusable UI component contracts without runtime command
//! buses or app-level orchestration.

use ratatui::crossterm::event::{KeyEvent, MouseEvent};
use ratatui::layout::Rect;
use ratatui::Frame;

use crate::theme::Theme;

/// A pure updatable component with no side-effect command type.
pub trait UpdatableComponent<Action> {
    /// Apply an action to the component state.
    fn update(&mut self, action: Action);
}

/// A design-only UI component contract.
///
/// Components can emit follow-up actions from input handlers, but do not own
/// scheduling, tasks, or app runtime dispatch.
pub trait UiComponent {
    /// Component action/message type.
    type Action: Send + 'static;

    /// Apply an action to the component state.
    fn update(&mut self, action: Self::Action);

    /// Render within the provided area.
    fn view(&mut self, frame: &mut Frame, area: Rect, theme: &Theme);

    /// Optional key handler that can emit a follow-up action.
    fn handle_key(&mut self, _key: KeyEvent) -> Option<Self::Action> {
        None
    }

    /// Optional mouse handler that can emit a follow-up action.
    fn handle_mouse(&mut self, _mouse: MouseEvent, _area: Rect) -> Option<Self::Action> {
        None
    }
}
