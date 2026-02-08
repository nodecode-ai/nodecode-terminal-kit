use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;
use std::path::PathBuf;

pub trait PathFormatter {
    fn format_for_display(&self, path: &str) -> String;
    fn transform_text_with_style(&self, text: &str, base_style: Style) -> Vec<Span<'static>>;
}

#[derive(Debug, Clone)]
pub struct PathDisplayConfig {
    pub prefix: String,
    pub show_relative: bool,
    pub bold: bool,
    pub color: Option<Color>,
    pub base_dir: Option<PathBuf>,
}

impl Default for PathDisplayConfig {
    fn default() -> Self {
        Self {
            prefix: "@".to_string(),
            show_relative: true,
            bold: false,
            color: None,
            base_dir: None,
        }
    }
}

pub struct MentionPathFormatter {
    config: PathDisplayConfig,
}

impl MentionPathFormatter {
    const PATH_BOUNDARY_CHARS: &'static str = "\"',;)]}";

    pub fn with_config(config: PathDisplayConfig) -> Self {
        Self { config }
    }

    fn to_relative_path(&self, path: &str) -> String {
        let base_dir = self
            .config
            .base_dir
            .as_ref()
            .cloned()
            .or_else(|| std::env::current_dir().ok())
            .unwrap_or_else(|| PathBuf::from("."));

        let path_buf = PathBuf::from(path);
        if let Ok(relative) = path_buf.strip_prefix(&base_dir) {
            return relative.to_string_lossy().to_string();
        }

        if let (Ok(abs_path), Ok(abs_base)) = (path_buf.canonicalize(), base_dir.canonicalize()) {
            if let Ok(relative) = abs_path.strip_prefix(&abs_base) {
                return relative.to_string_lossy().to_string();
            }
        }

        path_buf.to_string_lossy().to_string()
    }

    fn apply_style(&self, path: &str) -> String {
        let display_path = if self.config.show_relative {
            self.to_relative_path(path)
        } else {
            path.to_string()
        };
        format!("{}{}", self.config.prefix, display_path)
    }

    fn find_next_abs_segment<'a>(
        &self,
        s: &'a str,
        base_str: &str,
    ) -> Option<(usize, usize, usize)> {
        let pos = s.find(base_str)?;
        let at_precedes = pos > 0 && s[..pos].chars().last() == Some('@');
        let before_end = if at_precedes {
            pos - '@'.len_utf8()
        } else {
            pos
        };

        let path_remaining = &s[pos..];
        let rel_end = path_remaining
            .find(|c: char| c.is_whitespace() || Self::PATH_BOUNDARY_CHARS.contains(c))
            .unwrap_or(path_remaining.len());
        let path_end = pos + rel_end;
        Some((before_end, pos, path_end))
    }
}

impl PathFormatter for MentionPathFormatter {
    fn format_for_display(&self, path: &str) -> String {
        self.apply_style(path)
    }

    fn transform_text_with_style(&self, text: &str, base_style: Style) -> Vec<Span<'static>> {
        let base_dir = self
            .config
            .base_dir
            .as_ref()
            .cloned()
            .or_else(|| std::env::current_dir().ok())
            .unwrap_or_else(|| PathBuf::from("."));

        let base_str = base_dir.to_string_lossy();
        let mut spans = Vec::new();
        let mut remaining = text;

        while let Some((before_end, path_start, path_end)) =
            self.find_next_abs_segment(remaining, base_str.as_ref())
        {
            if before_end > 0 {
                spans.push(Span::styled(
                    remaining[..before_end].to_string(),
                    base_style,
                ));
            }
            let path = &remaining[path_start..path_end];
            let formatted = self.format_for_display(path);
            let mut style = base_style;
            if self.config.bold {
                style = style.add_modifier(Modifier::BOLD);
            }
            if let Some(color) = self.config.color {
                style = style.fg(color);
            }
            spans.push(Span::styled(formatted, style));
            remaining = &remaining[path_end..];
        }
        if !remaining.is_empty() {
            spans.push(Span::styled(remaining.to_string(), base_style));
        }

        spans
    }
}
