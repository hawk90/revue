//! Multiple cursor methods for TextArea

use super::cursor::{Cursor, CursorPos};

impl TextArea {
    /// Add cursor at position (Alt+Click)
    pub fn add_cursor_at(&mut self, line: usize, col: usize) {
        let line = line.min(self.lines.len().saturating_sub(1));
        let col = col.min(self.line_len(line));
        self.cursors.add_at(CursorPos::new(line, col));
    }

    /// Add cursor above current (Ctrl+Alt+Up)
    pub fn add_cursor_above(&mut self) {
        let primary = self.cursors.primary().pos;
        if primary.line > 0 {
            let new_line = primary.line - 1;
            let new_col = primary.col.min(self.line_len(new_line));
            self.cursors.add_at(CursorPos::new(new_line, new_col));
        }
    }

    /// Add cursor below current (Ctrl+Alt+Down)
    pub fn add_cursor_below(&mut self) {
        let primary = self.cursors.primary().pos;
        if primary.line + 1 < self.lines.len() {
            let new_line = primary.line + 1;
            let new_col = primary.col.min(self.line_len(new_line));
            self.cursors.add_at(CursorPos::new(new_line, new_col));
        }
    }

    /// Clear all secondary cursors (Escape)
    pub fn clear_secondary_cursors(&mut self) {
        self.cursors.clear_secondary();
    }

    /// Get word at cursor position
    fn get_word_at_cursor(&self) -> String {
        let pos = self.cursors.primary().pos;
        let Some(line) = self.lines.get(pos.line) else {
            return String::new();
        };
        let chars: Vec<char> = line.chars().collect();

        if chars.is_empty() || pos.col >= chars.len() {
            return String::new();
        }

        let mut start = pos.col;
        let mut end = pos.col;

        // Expand left
        while start > 0 && chars[start - 1].is_alphanumeric() {
            start -= 1;
        }

        // Expand right
        while end < chars.len() && chars[end].is_alphanumeric() {
            end += 1;
        }

        chars[start..end].iter().collect()
    }

    /// Get current word or selection text
    fn get_word_or_selection(&self) -> String {
        // If selection exists, return selected text
        if let Some(text) = self.get_selection() {
            return text;
        }
        // Otherwise get word under cursor
        self.get_word_at_cursor()
    }

    /// Find next occurrence of text from a given position
    fn find_next_from(&self, text: &str, from: CursorPos) -> Option<CursorPos> {
        if text.is_empty() {
            return None;
        }

        let text_lower = text.to_lowercase();

        // Search from the position after `from`
        for line_idx in from.line..self.lines.len() {
            let Some(line) = self.lines.get(line_idx) else {
                continue;
            };
            let line_lower = line.to_lowercase();

            let start_col = if line_idx == from.line {
                from.col + 1
            } else {
                0
            };

            if start_col < line.len() {
                if let Some(pos) = line_lower[start_col..].find(&text_lower) {
                    return Some(CursorPos::new(line_idx, start_col + pos));
                }
            }
        }

        // Wrap around to beginning
        for line_idx in 0..=from.line.min(self.lines.len().saturating_sub(1)) {
            let Some(line) = self.lines.get(line_idx) else {
                continue;
            };
            let line_lower = line.to_lowercase();

            let end_col = if line_idx == from.line {
                from.col + 1
            } else {
                line.len()
            };

            if let Some(pos) = line_lower[..end_col].find(&text_lower) {
                let found_pos = CursorPos::new(line_idx, pos);
                // Don't return if it's the same as one of our existing cursors
                if !self.cursors.iter().any(|c| c.pos == found_pos) {
                    return Some(found_pos);
                }
            }
        }

        None
    }

    /// Select next occurrence of current word/selection (Ctrl+D)
    pub fn select_next_occurrence(&mut self) {
        let text = self.get_word_or_selection();
        if text.is_empty() {
            return;
        }

        // Find next occurrence after the last cursor
        let last_pos = self
            .cursors
            .iter()
            .map(|c| c.pos)
            .max()
            .unwrap_or(CursorPos::new(0, 0));

        if let Some(match_pos) = self.find_next_from(&text, last_pos) {
            let end_col = match_pos.col + text.len();
            let new_cursor =
                Cursor::with_selection(CursorPos::new(match_pos.line, end_col), match_pos);
            self.cursors.add(new_cursor);
        }
    }
}

use super::TextArea;
