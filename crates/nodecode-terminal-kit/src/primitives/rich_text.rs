use crate::primitives::path::{MentionPathFormatter, PathDisplayConfig, PathFormatter};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

/// Rich text wrapper with path-aware transformations.
pub struct RichText {
    formatter: Box<dyn PathFormatter>,
    text: String,
    base_style: Style,
}

impl RichText {
    #[must_use]
    pub fn new(text: String) -> Self {
        Self {
            formatter: Box::new(MentionPathFormatter::with_config(
                PathDisplayConfig::default(),
            )),
            text,
            base_style: Style::default(),
        }
    }

    #[must_use]
    pub fn with_path_formatter(mut self, formatter: Box<dyn PathFormatter>) -> Self {
        self.formatter = formatter;
        self
    }

    #[must_use]
    pub fn with_style(mut self, style: Style) -> Self {
        self.base_style = style;
        self
    }

    #[must_use]
    pub fn render_spans(&self) -> Vec<Span<'static>> {
        self.formatter
            .transform_text_with_style(&self.text, self.base_style)
    }

    #[must_use]
    pub fn render_line(&self) -> Line<'static> {
        Line::from(self.render_spans())
    }

    #[must_use]
    pub fn transform_for_display(&self) -> String {
        self.render_spans()
            .iter()
            .map(|span| span.content.as_ref())
            .collect::<String>()
    }
}

pub fn render_text_with_paths(
    text: &str,
    style: Style,
    path_color: Option<Color>,
) -> Vec<Span<'static>> {
    let config = PathDisplayConfig {
        color: path_color,
        ..PathDisplayConfig::default()
    };
    let formatter = MentionPathFormatter::with_config(config);
    formatter.transform_text_with_style(text, style)
}
