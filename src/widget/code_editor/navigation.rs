//! Code editor cursor and navigation

impl super::CodeEditor {
    // =========================================================================
    // Cursor and Navigation
    // =========================================================================

    /// Get cursor position
    pub fn cursor_position(&self) -> (usize, usize) {
        self.cursor
    }

    /// Set cursor position
    pub fn set_cursor(&mut self, line: usize, col: usize) {
        let line = line.min(self.lines.len().saturating_sub(1));
        let col = col.min(self.line_len(line));
        self.cursor = (line, col);
        self.ensure_cursor_visible();
    }

    /// Get line count
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    /// Get line length
    pub(super) fn line_len(&self, line: usize) -> usize {
        self.lines.get(line).map(|l| l.len()).unwrap_or(0)
    }

    /// Move cursor left
    pub fn move_left(&mut self) {
        if self.cursor.1 > 0 {
            self.cursor.1 -= 1;
        } else if self.cursor.0 > 0 {
            self.cursor.0 -= 1;
            self.cursor.1 = self.line_len(self.cursor.0);
        }
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Move cursor right
    pub fn move_right(&mut self) {
        let line_len = self.line_len(self.cursor.0);
        if self.cursor.1 < line_len {
            self.cursor.1 += 1;
        } else if self.cursor.0 + 1 < self.lines.len() {
            self.cursor.0 += 1;
            self.cursor.1 = 0;
        }
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Move cursor up
    pub fn move_up(&mut self) {
        if self.cursor.0 > 0 {
            self.cursor.0 -= 1;
            self.cursor.1 = self.cursor.1.min(self.line_len(self.cursor.0));
        }
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Move cursor down
    pub fn move_down(&mut self) {
        if self.cursor.0 + 1 < self.lines.len() {
            self.cursor.0 += 1;
            self.cursor.1 = self.cursor.1.min(self.line_len(self.cursor.0));
        }
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Move to start of line
    pub fn move_home(&mut self) {
        // Smart home: first go to first non-whitespace, then to column 0
        let line = &self.lines[self.cursor.0];
        let first_non_ws = line.chars().position(|c| !c.is_whitespace()).unwrap_or(0);

        if self.cursor.1 == first_non_ws || self.cursor.1 == 0 {
            self.cursor.1 = if self.cursor.1 == 0 { first_non_ws } else { 0 };
        } else {
            self.cursor.1 = first_non_ws;
        }
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Move to end of line
    pub fn move_end(&mut self) {
        self.cursor.1 = self.line_len(self.cursor.0);
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Move to start of document
    pub fn move_document_start(&mut self) {
        self.cursor = (0, 0);
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Move to end of document
    pub fn move_document_end(&mut self) {
        let last_line = self.lines.len().saturating_sub(1);
        self.cursor = (last_line, self.line_len(last_line));
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Move by word left
    pub fn move_word_left(&mut self) {
        if self.cursor.1 == 0 {
            if self.cursor.0 > 0 {
                self.cursor.0 -= 1;
                self.cursor.1 = self.line_len(self.cursor.0);
            }
            return;
        }

        let line = &self.lines[self.cursor.0];
        let chars: Vec<char> = line.chars().collect();
        let mut col = self.cursor.1.min(chars.len());

        // Skip whitespace
        while col > 0 && chars[col - 1].is_whitespace() {
            col -= 1;
        }
        // Skip word
        while col > 0 && !chars[col - 1].is_whitespace() {
            col -= 1;
        }

        self.cursor.1 = col;
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Move by word right
    pub fn move_word_right(&mut self) {
        let line = &self.lines[self.cursor.0];
        let chars: Vec<char> = line.chars().collect();
        let mut col = self.cursor.1;

        if col >= chars.len() {
            if self.cursor.0 + 1 < self.lines.len() {
                self.cursor.0 += 1;
                self.cursor.1 = 0;
            }
            return;
        }

        // Skip current word
        while col < chars.len() && !chars[col].is_whitespace() {
            col += 1;
        }
        // Skip whitespace
        while col < chars.len() && chars[col].is_whitespace() {
            col += 1;
        }

        self.cursor.1 = col;
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Page up
    pub fn page_up(&mut self, page_size: usize) {
        self.cursor.0 = self.cursor.0.saturating_sub(page_size);
        self.cursor.1 = self.cursor.1.min(self.line_len(self.cursor.0));
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Page down
    pub fn page_down(&mut self, page_size: usize) {
        self.cursor.0 = (self.cursor.0 + page_size).min(self.lines.len().saturating_sub(1));
        self.cursor.1 = self.cursor.1.min(self.line_len(self.cursor.0));
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Ensure cursor is visible
    pub(super) fn ensure_cursor_visible(&mut self) {
        // Adjust vertical scroll
        if self.cursor.0 < self.scroll.0 {
            self.scroll.0 = self.cursor.0;
        }
        // Horizontal scroll adjustment would need view width
    }
}
