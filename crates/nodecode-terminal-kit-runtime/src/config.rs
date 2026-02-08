use std::time::Duration;

use nodecode_terminal_kit::theme::ThemeFacade;

#[derive(Debug, Clone)]
pub struct ExitKeys {
    pub esc: bool,
    pub q: bool,
    pub ctrl_c: bool,
}

impl Default for ExitKeys {
    fn default() -> Self {
        Self {
            esc: true,
            q: true,
            ctrl_c: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProgramConfig {
    pub title: String,
    pub theme: ThemeFacade,
    pub tick_rate: Duration,
    pub exit_keys: ExitKeys,
}

impl ProgramConfig {
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            theme: ThemeFacade::default(),
            tick_rate: Duration::from_millis(100),
            exit_keys: ExitKeys::default(),
        }
    }

    #[must_use]
    pub fn theme(mut self, theme: ThemeFacade) -> Self {
        self.theme = theme;
        self
    }

    #[must_use]
    pub fn tick_rate(mut self, tick_rate: Duration) -> Self {
        self.tick_rate = tick_rate;
        self
    }

    #[must_use]
    pub fn exit_keys(mut self, exit_keys: ExitKeys) -> Self {
        self.exit_keys = exit_keys;
        self
    }
}

impl Default for ProgramConfig {
    fn default() -> Self {
        Self::new("nodecode-terminal-kit-runtime")
    }
}
