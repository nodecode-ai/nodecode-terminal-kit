use crate::theme::Theme;
use crate::wizard::{StepAction, WizardItem, WizardStep};
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Paragraph};
use ratatui::{text::Text, Frame};

/// Lightweight summary/review step driven by callbacks.
#[derive(Debug)]
pub struct SummaryStep<T: WizardItem> {
    title: &'static str,
    help: &'static str,
    nav_hint: Option<&'static str>,
    lines: fn(&T) -> Vec<ratatui::text::Line<'static>>,
    on_key: Option<fn(KeyEvent, &mut T) -> Option<StepAction>>,
    height: Option<u16>,
}

impl<T: WizardItem> SummaryStep<T> {
    pub fn new(
        title: &'static str,
        help: &'static str,
        lines: fn(&T) -> Vec<ratatui::text::Line<'static>>,
    ) -> Self {
        Self {
            title,
            help,
            nav_hint: None,
            lines,
            on_key: None,
            height: None,
        }
    }

    pub fn with_nav_hint(mut self, hint: &'static str) -> Self {
        self.nav_hint = Some(hint);
        self
    }

    pub fn with_key_handler(mut self, handler: fn(KeyEvent, &mut T) -> Option<StepAction>) -> Self {
        self.on_key = Some(handler);
        self
    }

    pub fn with_height(mut self, height: u16) -> Self {
        self.height = Some(height);
        self
    }
}

impl<T: WizardItem> WizardStep<T> for SummaryStep<T> {
    fn title(&self) -> &str {
        self.title
    }

    fn help_text(&self) -> &str {
        self.help
    }

    fn navigation_hint(&self) -> Option<&str> {
        self.nav_hint
    }

    fn content_height(&self) -> u16 {
        self.height.unwrap_or(6)
    }

    fn render(&self, frame: &mut Frame, area: Rect, _theme: &Theme, item: &T) {
        let para = Paragraph::new(Text::from((self.lines)(item))).block(Block::default());
        frame.render_widget(para, area);
    }

    fn handle_key(&mut self, key: KeyEvent, item: &mut T) -> StepAction {
        if let Some(handler) = self.on_key {
            if let Some(action) = handler(key, item) {
                return action;
            }
        }
        match key.code {
            KeyCode::Esc => StepAction::Cancel,
            KeyCode::Left | KeyCode::Char('h') => StepAction::Previous,
            KeyCode::Enter => StepAction::Save,
            _ => StepAction::Continue,
        }
    }

    fn validate(&self, _item: &T) -> Result<(), String> {
        Ok(())
    }
}
