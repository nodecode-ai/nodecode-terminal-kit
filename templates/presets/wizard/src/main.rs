use crossterm::event::{KeyCode, KeyEvent};
use nodecode_terminal_kit_runtime::{Command, Model, Program, ProgramConfig};
use nodecode_terminal_kit::theme::{to_ratatui, Theme};
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

#[derive(Debug)]
struct WizardModel {
    step: usize,
    steps: Vec<&'static str>,
}

#[derive(Debug, Clone, Copy)]
enum WizardMsg {
    Next,
    Previous,
}

impl Model for WizardModel {
    type Msg = WizardMsg;

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            WizardMsg::Next => {
                self.step = (self.step + 1).min(self.steps.len().saturating_sub(1));
            }
            WizardMsg::Previous => {
                self.step = self.step.saturating_sub(1);
            }
        }
        Command::none()
    }

    fn view(&self, frame: &mut Frame, area: Rect, theme: &Theme) {
        let palette = theme.palette();
        let lines = vec![
            Line::from("Environment onboarding"),
            Line::from(""),
            Line::from(format!(
                "Step {}/{}: {}",
                self.step + 1,
                self.steps.len(),
                self.steps[self.step]
            )),
            Line::from(""),
            Line::from("All previous steps validated successfully."),
            Line::from("Final apply will enable the full workflow."),
            Line::from(""),
            Line::from("Use n/p to move, q to quit"),
        ];

        let widget = Paragraph::new(lines).block(
            Block::default()
                .title("{{title}}")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(to_ratatui(palette.border_focused))),
        );
        frame.render_widget(widget, area);
    }

    fn on_key(&mut self, key: KeyEvent) -> Option<Self::Msg> {
        match key.code {
            KeyCode::Char('n') => Some(WizardMsg::Next),
            KeyCode::Char('p') => Some(WizardMsg::Previous),
            _ => None,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model = WizardModel {
        step: 0,
        steps: vec!["Connect account", "Choose region", "Review", "Apply"],
    };

    Program::new(model, ProgramConfig::new("{{title}}")).run()?;
    Ok(())
}
