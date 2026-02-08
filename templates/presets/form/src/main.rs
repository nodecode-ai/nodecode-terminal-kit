use crossterm::event::KeyEvent;
use nodecode_terminal_kit_runtime::{Command, Model, Program, ProgramConfig};
use nodecode_terminal_kit::components::text_input::TextInput;
use nodecode_terminal_kit::theme::{to_ratatui, Theme};
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

#[derive(Debug)]
struct FormModel {
    input: TextInput,
}

#[derive(Debug, Clone)]
enum FormMsg {
    Key(KeyEvent),
}

impl Model for FormModel {
    type Msg = FormMsg;

    fn init(&mut self) -> Command<Self::Msg> {
        self.input.set_text("Starter Project".to_string());
        Command::none()
    }

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            FormMsg::Key(key) => self.input.handle_key(key),
        }
        Command::none()
    }

    fn view(&self, frame: &mut Frame, area: Rect, theme: &Theme) {
        let palette = theme.palette();
        let lines = vec![
            Line::from("Create a new project"),
            Line::from(""),
            Line::from(format!("Project name: {}", self.input.text())),
            Line::from("Owner: Team Alpha"),
            Line::from("Region: us-east-1"),
            Line::from("Status: Ready to submit"),
            Line::from(""),
            Line::from("Type to edit project name, q to quit"),
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
        Some(FormMsg::Key(key))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = TextInput::new();
    input.set_placeholder("Project name");

    let model = FormModel { input };
    Program::new(model, ProgramConfig::new("{{title}}")).run()?;
    Ok(())
}
