//! Cursor navigation methods for TextArea

impl TextArea {
    /// Move cursor left
    pub fn move_left(&mut self) {
        let pos = self.cursors.primary().pos;
        if pos.col > 0 {
            self.set_primary_cursor(pos.line, pos.col - 1);
        } else if pos.line > 0 {
            let new_line = pos.line - 1;
            let new_col = self.line_len(new_line);
            self.set_primary_cursor(new_line, new_col);
        }
        self.update_selection();
    }

    /// Move cursor right
    pub fn move_right(&mut self) {
        let pos = self.cursors.primary().pos;
        let line_len = self.line_len(pos.line);
        if pos.col < line_len {
            self.set_primary_cursor(pos.line, pos.col + 1);
        } else if pos.line + 1 < self.lines.len() {
            self.set_primary_cursor(pos.line + 1, 0);
        }
        self.update_selection();
    }

    /// Move cursor up
    pub fn move_up(&mut self) {
        let pos = self.cursors.primary().pos;
        if pos.line > 0 {
            let new_line = pos.line - 1;
            let new_col = pos.col.min(self.line_len(new_line));
            self.set_primary_cursor(new_line, new_col);
        }
        self.update_selection();
    }

    /// Move cursor down
    pub fn move_down(&mut self) {
        let pos = self.cursors.primary().pos;
        if pos.line + 1 < self.lines.len() {
            let new_line = pos.line + 1;
            let new_col = pos.col.min(self.line_len(new_line));
            self.set_primary_cursor(new_line, new_col);
        }
        self.update_selection();
    }

    /// Move to start of line
    pub fn move_home(&mut self) {
        let pos = self.cursors.primary().pos;
        self.set_primary_cursor(pos.line, 0);
        self.update_selection();
    }

    /// Move to end of line
    pub fn move_end(&mut self) {
        let pos = self.cursors.primary().pos;
        let line_len = self.line_len(pos.line);
        self.set_primary_cursor(pos.line, line_len);
        self.update_selection();
    }

    /// Move to start of document
    pub fn move_document_start(&mut self) {
        self.set_primary_cursor(0, 0);
        self.update_selection();
    }

    /// Move to end of document
    pub fn move_document_end(&mut self) {
        let last_line = self.lines.len().saturating_sub(1);
        let last_col = self.line_len(last_line);
        self.set_primary_cursor(last_line, last_col);
        self.update_selection();
    }

    /// Move cursor by word to the left
    pub fn move_word_left(&mut self) {
        let pos = self.cursors.primary().pos;
        if pos.col == 0 {
            if pos.line > 0 {
                let new_line = pos.line - 1;
                let new_col = self.line_len(new_line);
                self.set_primary_cursor(new_line, new_col);
            }
            return;
        }

        let Some(line) = self.lines.get(pos.line) else {
            return;
        };
        let chars: Vec<char> = line.chars().collect();
        let mut col = pos.col.min(chars.len());

        // Skip spaces
        while col > 0 && chars[col - 1].is_whitespace() {
            col -= 1;
        }
        // Skip word
        while col > 0 && !chars[col - 1].is_whitespace() {
            col -= 1;
        }

        self.set_primary_cursor(pos.line, col);
        self.update_selection();
    }

    /// Move cursor by word to the right
    pub fn move_word_right(&mut self) {
        let pos = self.cursors.primary().pos;
        let Some(line) = self.lines.get(pos.line) else {
            return;
        };
        let chars: Vec<char> = line.chars().collect();
        let mut col = pos.col;

        if col >= chars.len() {
            if pos.line + 1 < self.lines.len() {
                self.set_primary_cursor(pos.line + 1, 0);
            }
            return;
        }

        // Skip current word
        while col < chars.len() && !chars[col].is_whitespace() {
            col += 1;
        }
        // Skip spaces
        while col < chars.len() && chars[col].is_whitespace() {
            col += 1;
        }

        self.set_primary_cursor(pos.line, col);
        self.update_selection();
    }

    /// Page up
    pub fn page_up(&mut self, page_size: usize) {
        let pos = self.cursors.primary().pos;
        let new_line = pos.line.saturating_sub(page_size);
        let new_col = pos.col.min(self.line_len(new_line));
        self.set_primary_cursor(new_line, new_col);
        self.update_selection();
    }

    /// Page down
    pub fn page_down(&mut self, page_size: usize) {
        let pos = self.cursors.primary().pos;
        let new_line = (pos.line + page_size).min(self.lines.len().saturating_sub(1));
        let new_col = pos.col.min(self.line_len(new_line));
        self.set_primary_cursor(new_line, new_col);
        self.update_selection();
    }

    /// Select all text
    pub fn select_all(&mut self) {
        use super::cursor::CursorPos;
        use super::cursor::CursorSet;

        let last_line = self.lines.len().saturating_sub(1);
        let last_col = self.lines.last().map(|l| l.len()).unwrap_or(0);
        // Create cursor at end with anchor at start
        self.cursors = CursorSet::new(CursorPos::new(last_line, last_col));
        self.cursors.primary_mut().anchor = Some(CursorPos::new(0, 0));
    }
}

use crate::widget::textarea::TextArea;
