use crate::components::input_box::InputBox;
use crate::components::text_input::TextInput;
use crate::theme::{to_ratatui, Theme};
use crate::wizard::framework::{StepAction, WizardItem, WizardStep};
use crate::wizard::layout::input_step_layout;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

#[derive(Debug)]
pub struct SimpleTextStep<T: WizardItem> {
    title: &'static str,
    help: &'static str,
    field_label: &'static str,
    placeholder: &'static str,
    getter: fn(&T) -> String,
    setter: fn(&mut T, String),
    validator: fn(&str) -> Result<(), String>,
    input: TextInput,
    validation_error: Option<String>,
    status: Option<fn(&T) -> Option<Result<String, String>>>,
}

impl<T: WizardItem> SimpleTextStep<T> {
    pub fn new(
        title: &'static str,
        help: &'static str,
        field_label: &'static str,
        placeholder: &'static str,
        getter: fn(&T) -> String,
        setter: fn(&mut T, String),
        validator: fn(&str) -> Result<(), String>,
    ) -> Self {
        let mut input = TextInput::new();
        input.set_placeholder(placeholder);

        Self {
            title,
            help,
            field_label,
            placeholder,
            getter,
            setter,
            validator,
            input,
            validation_error: None,
            status: None,
        }
    }

    pub fn with_status(mut self, status: fn(&T) -> Option<Result<String, String>>) -> Self {
        self.status = Some(status);
        self
    }
}

impl<T: WizardItem> WizardStep<T> for SimpleTextStep<T> {
    fn title(&self) -> &str {
        self.title
    }

    fn help_text(&self) -> &str {
        self.help
    }

    fn render(&self, frame: &mut Frame, area: Rect, theme: &Theme, item: &T) {
        let [_, input_area, validation_area, _] = input_step_layout(area);

        let mut input = self.input.clone();
        input.set_placeholder(self.placeholder);
        input.set_text((self.getter)(item));

        let _ = InputBox::new(&input, theme)
            .follow_cursor(true)
            .title_override(Some(self.field_label))
            .cursor_active(true)
            .render(frame, input_area);

        if let Some(status_fn) = self.status {
            if let Some(status) = status_fn(item) {
                let (text, color) = match status {
                    Ok(msg) => (format!("✓ {}", msg), to_ratatui(theme.success)),
                    Err(err) => (format!("✗ {}", err), to_ratatui(theme.error)),
                };
                let para = Paragraph::new(text)
                    .style(Style::default().fg(color).add_modifier(Modifier::BOLD));
                frame.render_widget(para, validation_area);
                return;
            }
        }

        if let Some(error) = &self.validation_error {
            let error_para = Paragraph::new(format!("✗ {}", error)).style(
                Style::default()
                    .fg(to_ratatui(theme.error))
                    .add_modifier(Modifier::BOLD),
            );
            frame.render_widget(error_para, validation_area);
        }
    }

    fn handle_key(&mut self, key: KeyEvent, item: &mut T) -> StepAction {
        match key.code {
            KeyCode::Esc => return StepAction::Cancel,
            KeyCode::Enter => {
                (self.setter)(item, self.input.text().to_string());
                match (self.validator)(self.input.text()) {
                    Ok(_) => {
                        self.validation_error = None;
                        return StepAction::Next;
                    }
                    Err(err) => {
                        self.validation_error = Some(err);
                        return StepAction::Continue;
                    }
                }
            }
            _ => {}
        }

        if self.input.handle_key(key) {
            (self.setter)(item, self.input.text().to_string());
            self.validation_error = None;
        }

        StepAction::Continue
    }

    fn validate(&self, item: &T) -> Result<(), String> {
        (self.validator)((self.getter)(item).as_str())
    }
}
