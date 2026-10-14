//! The edit model: inserting and deleting at the cursor, cursor moves,
//! keys, and parsing the query after each change

use super::SearchBar;
use crate::query::Query;

impl SearchBar {
    /// Handle character input
    pub fn input(&mut self, ch: char) {
        let byte_idx = self
            .input
            .char_indices()
            .nth(self.cursor)
            .map_or(self.input.len(), |(i, _)| i);
        self.input.insert(byte_idx, ch);
        self.cursor += 1;
        self.parse_input();
    }

    /// Handle backspace
    pub fn backspace(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            let byte_idx = self
                .input
                .char_indices()
                .nth(self.cursor)
                .map_or(self.input.len(), |(i, _)| i);
            self.input.remove(byte_idx);
            self.parse_input();
        }
    }

    /// Handle delete
    pub fn delete(&mut self) {
        if self.cursor < self.input.chars().count() {
            let byte_idx = self
                .input
                .char_indices()
                .nth(self.cursor)
                .map_or(self.input.len(), |(i, _)| i);
            self.input.remove(byte_idx);
            self.parse_input();
        }
    }

    /// Move cursor left
    pub fn cursor_left(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }

    /// Move cursor right
    pub fn cursor_right(&mut self) {
        self.cursor = (self.cursor + 1).min(self.input.chars().count());
    }

    /// Move cursor to start
    pub fn cursor_home(&mut self) {
        self.cursor = 0;
    }

    /// Move cursor to end
    pub fn cursor_end(&mut self) {
        self.cursor = self.input.chars().count();
    }

    /// Handle key input
    pub fn handle_key(&mut self, key: &crate::event::Key) -> bool {
        use crate::event::Key;

        match key {
            Key::Char(ch) => {
                self.input(*ch);
                true
            }
            Key::Backspace => {
                self.backspace();
                true
            }
            Key::Delete => {
                self.delete();
                true
            }
            Key::Left => {
                self.cursor_left();
                true
            }
            Key::Right => {
                self.cursor_right();
                true
            }
            Key::Home => {
                self.cursor_home();
                true
            }
            Key::End => {
                self.cursor_end();
                true
            }
            Key::Escape => {
                self.clear();
                true
            }
            _ => false,
        }
    }

    /// Parse input and update query
    pub(super) fn parse_input(&mut self) {
        match Query::parse(&self.input) {
            Ok(query) => {
                self.parsed_query = Some(query);
                self.parse_error = None;
            }
            Err(err) => {
                self.parsed_query = None;
                self.parse_error = Some(err);
            }
        }
    }
}
