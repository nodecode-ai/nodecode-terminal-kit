use crossterm::event::{KeyCode, KeyEvent};
use nodecode_terminal_kit_runtime::{Command, Model, Program, ProgramConfig};
use nodecode_terminal_kit::theme::{to_ratatui, Theme};
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

#[derive(Debug)]
struct DashboardModel {
    reliability: i32,
}

#[derive(Debug, Clone, Copy)]
enum DashboardMsg {
    Increase,
    Decrease,
}

impl Model for DashboardModel {
    type Msg = DashboardMsg;

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            DashboardMsg::Increase => self.reliability = (self.reliability + 1).min(100),
            DashboardMsg::Decrease => self.reliability = (self.reliability - 1).max(0),
        }
        Command::none()
    }

    fn view(&self, frame: &mut Frame, area: Rect, theme: &Theme) {
        let palette = theme.palette();
        let lines = vec![
            Line::from("Release dashboard"),
            Line::from(""),
            Line::from(format!("Reliability score: {}%", self.reliability)),
            Line::from("Green deployments: 12"),
            Line::from("Pending blockers: 0"),
            Line::from(""),
            Line::from("Use +/- to adjust reliability, q to quit"),
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
            KeyCode::Char('+') => Some(DashboardMsg::Increase),
            KeyCode::Char('-') => Some(DashboardMsg::Decrease),
            _ => None,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model = DashboardModel { reliability: 95 };
    Program::new(model, ProgramConfig::new("{{title}}")).run()?;
    Ok(())
}
