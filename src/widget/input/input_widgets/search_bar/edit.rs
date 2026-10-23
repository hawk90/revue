//! The edit model: inserting and deleting at the cursor, cursor moves,
//! keys, and parsing the query after each change

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

    /// Handle key input without modifiers: Escape clears the query; the
    /// other keys are the standard editing keys every single-line field
    /// shares (#985). Returns whether the key was the search bar's.
    pub fn handle_key(&mut self, key: &crate::event::Key) -> bool {
        self.key(&crate::event::KeyEvent::new(*key))
    }

    /// Handle a key with its modifiers, as [`handle_key`](Self::handle_key)
    /// does, so the editing keys that need them work too: Ctrl+Z and
    /// Ctrl+Y, word moves and deletes, Shift selection, Ctrl+A. Returns
    /// false when the search bar is not focused, and for keys it leaves to
    /// the app.
    pub fn handle_key_event(&mut self, event: &crate::event::KeyEvent) -> bool {
        self.focused && self.key(event)
    }

    fn key(&mut self, event: &crate::event::KeyEvent) -> bool {
        use crate::event::Key;

        if event.key == Key::Escape && !event.ctrl && !event.alt {
            self.clear();
            return true;
        }
        match self.state.edit_key(event, 1) {
            EditOutcome::Edited => {
                self.parse_input();
                true
            }
            EditOutcome::Moved => true,
            EditOutcome::Unhandled => false,
        }
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
