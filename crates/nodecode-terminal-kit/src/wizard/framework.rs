//! Generic wizard framework for multi-step configuration flows

use ratatui::crossterm::event::KeyEvent;
use ratatui::layout::Rect;
use ratatui::Frame;
use std::fmt::Debug;
use std::hash::Hash;

use crate::theme::Theme;

/// Trait for items that can be configured via wizards
pub trait WizardItem: Clone + Debug + Send + 'static {
    type Id: Clone + Eq + Hash + Debug + Send;

    fn id(&self) -> Self::Id;
    fn display_name(&self) -> String;
    fn is_valid(&self) -> Result<(), String>;
    fn default_item() -> Self;
}

/// Trait for individual wizard steps
pub trait WizardStep<T: WizardItem>: Debug + Send {
    fn title(&self) -> &str;
    fn help_text(&self) -> &str;
    fn render(&self, frame: &mut Frame, area: Rect, theme: &Theme, item: &T);
    fn handle_key(&mut self, key: KeyEvent, item: &mut T) -> StepAction;
    fn validate(&self, item: &T) -> Result<(), String>;

    fn can_skip(&self) -> bool {
        false
    }

    /// Optional step-specific navigation hint to override default navigation keybinds.
    ///
    /// Returns custom navigation text for the footer (e.g., "space toggle  enter next  esc cancel").
    /// If None, the wizard view uses default navigation based on step position.
    ///
    /// Format: Double-space separated "key description" pairs (e.g., "enter next  ← back").
    fn navigation_hint(&self) -> Option<&str> {
        None // Default: use standard navigation
    }

    /// Content area height in lines for this step.
    ///
    /// Default is 3 lines (suitable for single-input steps with border).
    /// List-based steps can override to request more space (e.g., 8 lines for 6-item list).
    ///
    /// Note: This is the step content area only (excludes help text and footer).
    fn content_height(&self) -> u16 {
        3 // Default: 3 lines for input boxes
    }
}

/// Action returned by step's handle_key
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepAction {
    Continue,
    Next,
    Previous,
    Cancel,
    Save,
}

/// Wizard mode (creating new or editing existing)
#[derive(Debug, Clone)]
pub enum WizardMode<T: WizardItem> {
    Creating,
    Editing(T::Id),
}

/// Wizard flow orchestrator
pub struct WizardFlow<T: WizardItem> {
    pub(crate) steps: Vec<Box<dyn WizardStep<T>>>,
    pub(crate) current_step_idx: usize,
    pub(crate) item: T,
    pub mode: WizardMode<T>,
}

impl<T: WizardItem> WizardFlow<T> {
    pub fn new(steps: Vec<Box<dyn WizardStep<T>>>, mode: WizardMode<T>) -> Self {
        let item = match &mode {
            WizardMode::Creating => T::default_item(),
            WizardMode::Editing(_) => T::default_item(),
        };

        Self {
            steps,
            current_step_idx: 0,
            item,
            mode,
        }
    }

    pub fn set_item(&mut self, item: T) {
        self.item = item;
    }

    pub fn current_step(&self) -> &dyn WizardStep<T> {
        self.steps[self.current_step_idx].as_ref()
    }

    pub fn current_step_mut(&mut self) -> &mut dyn WizardStep<T> {
        self.steps[self.current_step_idx].as_mut()
    }

    pub fn item(&self) -> &T {
        &self.item
    }

    pub fn item_mut(&mut self) -> &mut T {
        &mut self.item
    }

    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    pub fn current_step_number(&self) -> usize {
        self.current_step_idx + 1
    }

    /// Dispatch a key event to the currently active step.
    pub fn handle_key(&mut self, key: KeyEvent) -> StepAction {
        let idx = self.current_step_idx;
        let item = &mut self.item;
        let step = &mut self.steps[idx];
        step.handle_key(key, item)
    }

    pub fn can_go_back(&self) -> bool {
        self.current_step_idx > 0
    }

    pub fn can_go_forward(&self) -> bool {
        self.current_step_idx < self.steps.len() - 1
    }

    pub fn advance(&mut self) -> Result<(), String> {
        if !self.can_go_forward() {
            return Err("Already at last step".into());
        }
        self.current_step_idx += 1;
        Ok(())
    }

    pub fn go_back(&mut self) -> Result<(), String> {
        if !self.can_go_back() {
            return Err("Already at first step".into());
        }
        self.current_step_idx -= 1;
        Ok(())
    }
}

/// Trait for customizing list view rendering
pub trait ItemListView<T: WizardItem>: Debug + Send {
    fn render_item(
        &self,
        item: &T,
        is_selected: bool,
        theme: &Theme,
    ) -> ratatui::widgets::ListItem<'static>;

    fn item_actions(&self) -> Vec<(&'static str, char)> {
        vec![("Edit", 'e'), ("Delete", 'd')]
    }

    fn supports_toggle(&self) -> bool {
        false
    }

    fn toggle_item(&self, _item: &mut T) {}
}
