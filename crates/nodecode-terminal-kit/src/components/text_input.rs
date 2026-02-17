use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tui_textarea::{CursorMove, Input as TaInput, Key as TaKey, TextArea};
use unicode_width::UnicodeWidthChar;

use crate::layout::text::{
    byte_offset_to_cursor, cursor_to_byte_offset, find_visual_row, visual_column,
    wrapped_row_ranges,
};

#[derive(Debug, Clone)]
pub struct TextInput {
    inner: TextArea<'static>,
    text_cache: String,
    prefix: String,
    placeholder: Option<String>,
}

impl Default for TextInput {
    fn default() -> Self {
        Self::new()
    }
}

impl TextInput {
    pub fn new() -> Self {
        Self::with_text(String::new())
    }

    pub fn with_text(text: String) -> Self {
        let mut inner = textarea_from(&text);
        inner.move_cursor(CursorMove::Bottom);
        inner.move_cursor(CursorMove::End);
        let mut this = Self {
            inner,
            text_cache: text,
            prefix: "❯".to_string(),
            placeholder: None,
        };
        this.sync_placeholder();
        this
    }

    pub fn text(&self) -> &str {
        &self.text_cache
    }

    pub fn is_empty(&self) -> bool {
        self.text_cache.is_empty()
    }

    pub fn cursor(&self) -> usize {
        cursor_to_byte_offset(self.inner.lines(), self.inner.cursor())
    }

    pub fn handle_search_key(&mut self, key: KeyEvent) -> Option<String> {
        let before = self.text_cache.clone();
        if self.handle_key(key) && before != self.text_cache {
            Some(self.text_cache.clone())
        } else {
            None
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text_cache = text.clone();
        self.inner = textarea_from(&text);
        self.inner.move_cursor(CursorMove::Bottom);
        self.inner.move_cursor(CursorMove::End);
        self.sync_placeholder();
    }

    pub fn clear(&mut self) {
        self.set_text(String::new());
    }

    pub fn insert_char(&mut self, ch: char) {
        self.apply_edit(|ta| ta.insert_char(ch));
    }

    pub fn insert_str(&mut self, s: &str) {
        self.apply_edit(|ta| {
            ta.insert_str(s);
        });
    }

    pub fn delete_backward(&mut self) {
        self.apply_edit(|ta| {
            let _ = ta.delete_char();
        });
    }

    pub fn delete_forward(&mut self) {
        self.apply_edit(|ta| {
            let _ = ta.delete_next_char();
        });
    }

    pub fn delete_to_line_start(&mut self) {
        self.apply_edit(|ta| {
            let _ = ta.delete_line_by_head();
        });
    }

    pub fn delete_word_forward(&mut self) {
        self.apply_edit(|ta| {
            let _ = ta.delete_next_word();
        });
    }

    pub fn delete_word_backward(&mut self) {
        self.apply_edit(|ta| {
            let _ = ta.delete_word();
        });
    }

    pub fn cursor_start(&mut self) {
        self.inner.move_cursor(CursorMove::Head);
    }

    pub fn cursor_end(&mut self) {
        self.inner.move_cursor(CursorMove::Bottom);
        self.inner.move_cursor(CursorMove::End);
    }

    pub fn cursor_visual_line_end(&mut self, content_width: u16) {
        if content_width == 0 || self.text_cache.is_empty() {
            self.cursor_end();
            return;
        }
        let ranges = wrapped_row_ranges(&self.text_cache, content_width);
        if ranges.is_empty() {
            return;
        }
        let row_idx = find_visual_row(&ranges, self.cursor().min(self.text_cache.len()));
        let (_, end) = ranges[row_idx];
        self.set_cursor_byte_offset(end.min(self.text_cache.len()));
    }

    pub fn cursor_left(&mut self) {
        self.inner.move_cursor(CursorMove::Back);
    }

    pub fn cursor_right(&mut self) {
        self.inner.move_cursor(CursorMove::Forward);
    }

    pub fn cursor_word_left(&mut self) {
        self.inner.move_cursor(CursorMove::WordBack);
    }

    pub fn cursor_word_right(&mut self) {
        self.inner.move_cursor(CursorMove::WordForward);
    }

    pub fn cursor_move_vertical(&mut self, up: bool) -> bool {
        let before = self.cursor();
        if up {
            self.inner.move_cursor(CursorMove::Up);
        } else {
            self.inner.move_cursor(CursorMove::Down);
        }
        before != self.cursor()
    }

    pub fn cursor_up_line(&mut self) -> bool {
        self.cursor_move_vertical(true)
    }

    pub fn cursor_down_line(&mut self) -> bool {
        self.cursor_move_vertical(false)
    }

    pub fn cursor_move_visual_vertical(&mut self, up: bool, content_width: u16) -> bool {
        if content_width == 0 || self.text_cache.is_empty() {
            return false;
        }

        let ranges = wrapped_row_ranges(&self.text_cache, content_width);
        if ranges.len() <= 1 {
            return false;
        }

        let current = self.cursor().min(self.text_cache.len());
        let current_idx = find_visual_row(&ranges, current);
        let target_idx = if up {
            current_idx.checked_sub(1)
        } else if current_idx + 1 < ranges.len() {
            Some(current_idx + 1)
        } else {
            None
        };
        let target_idx = match target_idx {
            Some(idx) => idx,
            None => return false,
        };

        let goal_col = self.cursor_visual_position(content_width).1 as usize;
        let (start, end) = ranges[target_idx];
        let slice = &self.text_cache[start..end];
        let mut target_offset = start;
        let mut acc = 0usize;
        for (byte_idx, ch) in slice.char_indices() {
            let w = UnicodeWidthChar::width(ch).unwrap_or(0).max(1);
            if acc + w > goal_col {
                break;
            }
            acc += w;
            target_offset = start + byte_idx + ch.len_utf8();
        }
        if acc < goal_col {
            target_offset = end;
        }

        self.set_cursor_byte_offset(target_offset);
        true
    }

    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    pub fn set_prefix<S: Into<String>>(&mut self, prefix: S) {
        self.prefix = prefix.into();
    }

    pub fn placeholder(&self) -> Option<&str> {
        self.placeholder.as_deref()
    }

    pub fn set_placeholder<S: Into<String>>(&mut self, text: S) {
        let value = text.into();
        self.placeholder = if value.is_empty() { None } else { Some(value) };
        self.sync_placeholder();
    }

    pub fn display_text(&self) -> String {
        format!("{}{}", self.prefix, self.text_cache)
    }

    pub fn cursor_visual_position(&self, content_width: u16) -> (u16, u16) {
        let cursor = self.cursor().min(self.text_cache.len());
        let ranges = wrapped_row_ranges(&self.text_cache, content_width);
        if ranges.is_empty() {
            return (0, 0);
        }
        let row_idx = find_visual_row(&ranges, cursor);
        let (start, end) = ranges[row_idx];
        let clamped_cursor = cursor.min(end).max(start);
        let col = visual_column(&self.text_cache[start..clamped_cursor]);
        (row_idx as u16, col as u16)
    }

    pub fn wrapped_rows(&self, content_width: u16) -> Vec<(usize, usize)> {
        wrapped_row_ranges(&self.text_cache, content_width)
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        match (key.code, key.modifiers) {
            (KeyCode::Backspace, KeyModifiers::NONE) => {
                let had_content = self.cursor() > 0;
                self.delete_backward();
                had_content
            }
            (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
                self.delete_to_line_start();
                true
            }
            (KeyCode::Char('w'), KeyModifiers::CONTROL) => {
                self.delete_word_backward();
                true
            }
            (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                self.delete_forward();
                true
            }
            _ => self.apply_edit(|ta| {
                ta.input(ta_input_from_key_event(key));
            }),
        }
    }

    pub fn visual_rows(&self, content_width: u16) -> usize {
        wrapped_row_ranges(&self.text_cache, content_width).len()
    }

    pub fn set_cursor_byte_offset(&mut self, offset: usize) {
        let (row, col_chars) = byte_offset_to_cursor(self.inner.lines(), offset);
        self.inner
            .move_cursor(CursorMove::Jump(row as u16, col_chars as u16));
    }

    fn apply_edit(&mut self, f: impl FnOnce(&mut TextArea<'static>)) -> bool {
        let before = self.text_cache.clone();
        f(&mut self.inner);
        self.text_cache = join_lines(self.inner.lines());
        self.sync_placeholder();
        before != self.text_cache
    }

    fn sync_placeholder(&mut self) {
        self.inner
            .set_placeholder_text(self.placeholder.clone().unwrap_or_default());
    }
}

fn textarea_from(text: &str) -> TextArea<'static> {
    if text.is_empty() {
        TextArea::default()
    } else {
        TextArea::from(text.split('\n'))
    }
}

fn join_lines(lines: &[String]) -> String {
    lines.join("\n")
}

fn ta_input_from_key_event(key: KeyEvent) -> TaInput {
    let mapped = match key.code {
        KeyCode::Backspace => TaKey::Backspace,
        KeyCode::Enter => TaKey::Enter,
        KeyCode::Left => TaKey::Left,
        KeyCode::Right => TaKey::Right,
        KeyCode::Up => TaKey::Up,
        KeyCode::Down => TaKey::Down,
        KeyCode::Tab | KeyCode::BackTab => TaKey::Tab,
        KeyCode::Delete => TaKey::Delete,
        KeyCode::Home => TaKey::Home,
        KeyCode::End => TaKey::End,
        KeyCode::PageUp => TaKey::PageUp,
        KeyCode::PageDown => TaKey::PageDown,
        KeyCode::Esc => TaKey::Esc,
        KeyCode::Char(ch) => TaKey::Char(ch),
        KeyCode::F(n) => TaKey::F(n),
        _ => TaKey::Null,
    };
    TaInput {
        key: mapped,
        ctrl: key.modifiers.contains(KeyModifiers::CONTROL),
        alt: key.modifiers.contains(KeyModifiers::ALT),
        shift: key.modifiers.contains(KeyModifiers::SHIFT),
    }
}
