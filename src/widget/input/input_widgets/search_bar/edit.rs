//! The edit model: inserting and deleting at the cursor, cursor moves,
//! keys, and parsing the query after each change

use unicode_segmentation::UnicodeSegmentation;

use super::SearchBar;
use crate::query::Query;
use crate::utils::{EditMotion, EditOutcome};

impl SearchBar {
    /// Handle character input
    pub fn input(&mut self, ch: char) {
        if self.state.insert(ch.encode_utf8(&mut [0; 4])) {
            self.parse_input();
        }
    }

    /// Handle backspace
    pub fn backspace(&mut self) {
        if self.state.backspace() {
            self.parse_input();
        }
    }

    /// Handle delete
    pub fn delete(&mut self) {
        if self.state.delete() {
            self.parse_input();
        }
    }

    /// Move cursor left
    pub fn cursor_left(&mut self) {
        self.state.move_cursors(EditMotion::Left, false);
    }

    /// Move cursor right
    pub fn cursor_right(&mut self) {
        self.state.move_cursors(EditMotion::Right, false);
    }

    /// Move cursor to start
    pub fn cursor_home(&mut self) {
        self.state.move_cursors(EditMotion::LineStart, false);
    }

    /// Move cursor to end
    pub fn cursor_end(&mut self) {
        self.state.move_cursors(EditMotion::LineEnd, false);
    }

    /// Handle key input: Escape clears the query; the other keys are the
    /// standard editing keys every single-line field shares (#985).
    /// Returns whether the key was the search bar's.
    pub fn handle_key(&mut self, key: &crate::event::Key) -> bool {
        use crate::event::{Key, KeyEvent};

        if *key == Key::Escape {
            self.clear();
            return true;
        }
        match self.state.edit_key(&KeyEvent::new(*key), 1) {
            EditOutcome::Edited => {
                self.parse_input();
                true
            }
            EditOutcome::Moved => true,
            EditOutcome::Unhandled => false,
        }
    }

    /// The cursor as a character index into the query
    pub(super) fn cursor_char(&self) -> usize {
        let line = self.get_input();
        line.graphemes(true)
            .take(self.state.cursor().col)
            .map(|g| g.chars().count())
            .sum()
    }

    /// Parse input and update query
    pub(super) fn parse_input(&mut self) {
        match Query::parse(self.get_input()) {
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
