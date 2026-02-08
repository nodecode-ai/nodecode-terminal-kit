use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use nodecode_terminal_kit::components::text_input::TextInput;
use nodecode_terminal_kit::layout::branding::LOGO_COMPACT;
use nodecode_terminal_kit::layout::text::wrapped_row_ranges;
use nodecode_terminal_kit::theme::{to_ratatui, ThemeFacade};
use ratatui::{backend::CrosstermBackend, Terminal};
use ratatui::{
    style::Style,
    text::Line,
    widgets::{Block, Borders, Paragraph},
};
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let theme = ThemeFacade::default();
    let palette = theme.palette();
    let mut input = TextInput::new();
    input.set_text("Design-only terminal kit demo input".to_string());

    loop {
        terminal.draw(|f| {
            let area = f.area();
            let lines = vec![
                Line::from(LOGO_COMPACT[0].to_string()),
                Line::from(LOGO_COMPACT[1].to_string()),
                Line::from(LOGO_COMPACT[2].to_string()),
                Line::from("".to_string()),
                Line::from(format!("Input text: {}", input.text())),
                Line::from(format!(
                    "Wrapped rows(18 cols): {}",
                    wrapped_row_ranges(input.text(), 18).len()
                )),
                Line::from("Press q to quit".to_string()),
            ];

            let widget = Paragraph::new(lines).block(
                Block::default()
                    .title("nodecode-terminal-kit")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(to_ratatui(palette.border_focused))),
            );
            f.render_widget(widget, area);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
