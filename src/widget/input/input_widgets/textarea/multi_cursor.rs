//! Multiple cursor methods for TextArea
//!
//! An added cursor becomes the primary one (the one the view follows and
//! [`cursor_position`](TextArea::cursor_position) gives), so adding a cursor
//! below again goes one more line down.

use super::cursor::{CursorPos, MAX_CURSORS};
use crate::utils::EditCursor;

impl TextArea {
    /// Add a cursor `cursor`, unless there are [`MAX_CURSORS`] already
    fn push_cursor(&mut self, cursor: EditCursor) {
        if self.edit.cursors().len() < MAX_CURSORS {
            self.edit.add_cursor(cursor);
        }
    }

    /// Add cursor at position, in chars (Alt+Click)
    pub fn add_cursor_at(&mut self, line: usize, col: usize) {
        let pos = self.edit_pos(CursorPos::new(line, col));
        self.push_cursor(EditCursor::at(pos));
    }

    /// Add a cursor on the line above the topmost cursor, at its screen
    /// column (Ctrl+Alt+Up)
    pub fn add_cursor_above(&mut self) {
        let top = self.char_pos(self.edit.cursors()[0].head());
        if top.line > 0 {
            self.add_cursor_on(top, top.line - 1);
        }
    }

    /// Add a cursor on the line below the bottommost cursor, at its screen
    /// column (Ctrl+Alt+Down)
    pub fn add_cursor_below(&mut self) {
        let cursors = self.edit.cursors();
        let bottom = self.char_pos(cursors[cursors.len() - 1].head());
        if bottom.line + 1 < self.edit.line_count() {
            self.add_cursor_on(bottom, bottom.line + 1);
        }
    }

    fn add_cursor_on(&mut self, from: CursorPos, line: usize) {
        let col = self.same_column_on(from, line);
        self.add_cursor_at(line, col);
    }

    /// Clear all secondary cursors (Escape)
    pub fn clear_secondary_cursors(&mut self) {
        self.edit.clear_secondary();
    }

    /// The start and end (in chars) of the word (letters, digits) at the
    /// primary cursor, touching it on either side; `None` when there is none
    fn word_at_cursor(&self) -> Option<(CursorPos, CursorPos)> {
        let pos = self.primary_pos();
        let chars: Vec<char> = self.line_text(pos.line).chars().collect();
        let mut start = pos.col.min(chars.len());
        let mut end = start;
        while start > 0 && chars[start - 1].is_alphanumeric() {
            start -= 1;
        }
        while end < chars.len() && chars[end].is_alphanumeric() {
            end += 1;
        }
        (start < end).then(|| {
            (
                CursorPos::new(pos.line, start),
                CursorPos::new(pos.line, end),
            )
        })
    }

    /// Find next occurrence of text from a given position
    ///
    /// Columns are char indices, so the search runs on chars: a byte offset
    /// from `str::find` would land the cursor in the wrong place, and slicing
    /// a line at a char column panics inside a multibyte char.
    fn find_next_from(&self, text: &str, from: CursorPos) -> Option<CursorPos> {
        let query: Vec<char> = text.chars().collect();
        if query.is_empty() {
            return None;
        }
        let line_count = self.edit.line_count();
        let heads: Vec<CursorPos> = self
            .edit
            .cursors()
            .iter()
            .map(|c| self.char_pos(c.head()))
            .collect();

        // Search from the position after `from`
        for line_idx in from.line..line_count {
            let chars: Vec<char> = self.line_text(line_idx).chars().collect();
            let start_col = if line_idx == from.line {
                from.col + 1
            } else {
                0
            };
            if let Some(col) = find_chars(&chars, &query, start_col, false) {
                return Some(CursorPos::new(line_idx, col));
            }
        }

        // Wrap around to beginning
        for line_idx in 0..=from.line.min(line_count - 1) {
            let chars: Vec<char> = self.line_text(line_idx).chars().collect();
            // On `from`'s own line, only matches starting at or before it.
            let limit = if line_idx == from.line {
                (from.col + query.len()).min(chars.len())
            } else {
                chars.len()
            };

            if let Some(col) = find_chars(&chars[..limit], &query, 0, false) {
                let found_pos = CursorPos::new(line_idx, col);
                // Don't return if it's the same as one of our existing cursors
                if !heads.contains(&found_pos) {
                    return Some(found_pos);
                }
            }
        }

        None
    }

    /// Select the word under the cursor; when the primary cursor already
    /// selects text, add a cursor selecting its next occurrence after the
    /// last cursor, wrapping to the start (Ctrl+D)
    pub fn select_next_occurrence(&mut self) {
        let Some(text) = self.get_selection() else {
            if let Some((start, end)) = self.word_at_cursor() {
                let (start, end) = (self.edit_pos(start), self.edit_pos(end));
                let mut cursors = self.edit.cursors().to_vec();
                let primary = cursors
                    .iter()
                    .position(|c| c == self.edit.primary())
                    .unwrap_or(0);
                cursors[primary] = EditCursor::selecting(start, end);
                self.set_cursors_keeping_primary(cursors);
            }
            return;
        };

        // Find next occurrence after the last cursor
        let last = self.edit.cursors().last().map(|c| c.head());
        let last_pos = last.map_or(CursorPos::new(0, 0), |p| self.char_pos(p));

        if let Some(start) = self.find_next_from(&text, last_pos) {
            let end = CursorPos::new(start.line, start.col + text.chars().count());
            let cursor = EditCursor::selecting(self.edit_pos(start), self.edit_pos(end));
            self.push_cursor(cursor);
        }
    }
}

use super::TextArea;
use crate::utils::text::find_chars;
