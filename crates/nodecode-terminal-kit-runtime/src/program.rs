use std::collections::VecDeque;
use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::{Command, ExitKeys, Model, ProgramConfig, ProgramError};

struct TerminalSession {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl TerminalSession {
    fn new() -> Result<Self, ProgramError> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal })
    }
}

impl Drop for TerminalSession {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(self.terminal.backend_mut(), LeaveAlternateScreen);
        let _ = self.terminal.show_cursor();
    }
}

pub struct Program<M: Model> {
    model: M,
    config: ProgramConfig,
}

impl<M: Model> Program<M> {
    #[must_use]
    pub fn new(model: M, config: ProgramConfig) -> Self {
        Self { model, config }
    }

    pub fn run(mut self) -> Result<(), ProgramError> {
        let mut session = TerminalSession::new()?;
        let mut pending = VecDeque::new();

        enqueue_command(self.model.init(), &mut pending);
        drain_pending(&mut self.model, &mut pending);

        loop {
            session.terminal.draw(|f| {
                let area = f.area();
                self.model.view(f, area, self.config.theme.theme());
            })?;

            if !event::poll(self.config.tick_rate)? {
                continue;
            }

            let Event::Key(key) = event::read()? else {
                continue;
            };

            if should_exit_key(&key, &self.config.exit_keys) {
                break;
            }

            if let Some(msg) = self.model.on_key(key) {
                pending.push_back(msg);
                drain_pending(&mut self.model, &mut pending);
            }
        }

        Ok(())
    }
}

fn enqueue_command<Msg>(command: Command<Msg>, pending: &mut VecDeque<Msg>) {
    match command {
        Command::None => {}
        Command::One(op) => pending.push_back(op()),
        Command::Batch(commands) => {
            for command in commands {
                pending.push_back(command());
            }
        }
    }
}

fn drain_pending<M: Model>(model: &mut M, pending: &mut VecDeque<M::Msg>) {
    while let Some(msg) = pending.pop_front() {
        enqueue_command(model.update(msg), pending);
    }
}

fn should_exit_key(key: &KeyEvent, exit_keys: &ExitKeys) -> bool {
    match key.code {
        KeyCode::Esc => exit_keys.esc,
        KeyCode::Char('q') if key.modifiers.is_empty() => exit_keys.q,
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => exit_keys.ctrl_c,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use nodecode_terminal_kit::theme::Theme;
    use ratatui::layout::Rect;
    use ratatui::Frame;

    use crate::{Command, ExitKeys, Model};

    use super::{drain_pending, enqueue_command, should_exit_key};

    #[derive(Default)]
    struct TestModel {
        updates: Vec<i32>,
        init_calls: usize,
    }

    impl Model for TestModel {
        type Msg = i32;

        fn init(&mut self) -> Command<Self::Msg> {
            self.init_calls += 1;
            Command::one(|| 1)
        }

        fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
            self.updates.push(msg);
            Command::none()
        }

        fn view(&self, _frame: &mut Frame, _area: Rect, _theme: &Theme) {}
    }

    #[test]
    fn init_command_runs_once_and_updates_model() {
        let mut model = TestModel::default();
        let mut pending = VecDeque::new();

        enqueue_command(model.init(), &mut pending);
        drain_pending(&mut model, &mut pending);

        assert_eq!(model.init_calls, 1);
        assert_eq!(model.updates, vec![1]);
    }

    #[test]
    fn batch_command_preserves_insertion_order() {
        let mut pending = VecDeque::new();
        enqueue_command(Command::batch([|| 1, || 2, || 3]), &mut pending);

        let drained: Vec<i32> = pending.into_iter().collect();
        assert_eq!(drained, vec![1, 2, 3]);
    }

    #[test]
    fn default_exit_keys_match_expected_shortcuts() {
        let keys = ExitKeys::default();

        assert!(should_exit_key(
            &KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
            &keys
        ));
        assert!(should_exit_key(
            &KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE),
            &keys
        ));
        assert!(should_exit_key(
            &KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
            &keys
        ));
        assert!(!should_exit_key(
            &KeyEvent::new(KeyCode::Char('q'), KeyModifiers::SHIFT),
            &keys
        ));

        let no_q = ExitKeys {
            esc: true,
            q: false,
            ctrl_c: true,
        };
        assert!(!should_exit_key(
            &KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE),
            &no_q
        ));
    }
}
