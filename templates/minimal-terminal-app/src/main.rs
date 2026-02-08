use crossterm::event::{KeyCode, KeyEvent};
use nodecode_terminal_kit_runtime::{Command, Model, Program, ProgramConfig};
use nodecode_terminal_kit::theme::{to_ratatui, Theme};
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

#[derive(Debug)]
struct MinimalModel {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum MinimalMsg {
    Increase,
    Decrease,
}

impl Model for MinimalModel {
    type Msg = MinimalMsg;

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            MinimalMsg::Increase => self.value = (self.value + 1).min(100),
            MinimalMsg::Decrease => self.value = (self.value - 1).max(0),
        }
        Command::none()
    }

    fn view(&self, frame: &mut Frame, area: Rect, theme: &Theme) {
        let palette = theme.palette();
        let lines = vec![
            Line::from("Minimal runtime starter"),
            Line::from(""),
            Line::from(format!("Value: {}", self.value)),
            Line::from("Use +/- to adjust value, q to quit"),
        ];

        let widget = Paragraph::new(lines).block(
            Block::default()
                .title("Minimal Terminal App")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(to_ratatui(palette.border_focused))),
        );
        frame.render_widget(widget, area);
    }

    fn on_key(&mut self, key: KeyEvent) -> Option<Self::Msg> {
        match key.code {
            KeyCode::Char('+') => Some(MinimalMsg::Increase),
            KeyCode::Char('-') => Some(MinimalMsg::Decrease),
            _ => None,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model = MinimalModel { value: 50 };
    Program::new(model, ProgramConfig::new("Minimal Terminal App")).run()?;
    Ok(())
}
