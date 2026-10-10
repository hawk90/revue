//! Code editor cursor and navigation

use crate::widget::traits::render_context::edit_line::{char_at_col, col_of};

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

    /// Get line length (in chars: cursor columns are char indices)
    pub(super) fn line_len(&self, line: usize) -> usize {
        self.lines.get(line).map(|l| l.chars().count()).unwrap_or(0)
    }

    /// Move cursor left
    pub fn move_left(&mut self) {
        if self.cursor.1 > 0 {
            self.cursor.1 -= 1;
        } else if self.cursor.0 > 0 {
            self.cursor.0 -= 1;
            self.cursor.1 = self.line_len(self.cursor.0);
        }
        // Only clear selection if not in selection mode
        if self.anchor.is_none() {
            self.clear_selection();
        }
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
        // Only clear selection if not in selection mode
        if self.anchor.is_none() {
            self.clear_selection();
        }
        self.ensure_cursor_visible();
    }

    /// Move cursor up
    pub fn move_up(&mut self) {
        if self.cursor.0 > 0 {
            self.move_to_line(self.cursor.0 - 1);
        }
        // Only clear selection if not in selection mode
        if self.anchor.is_none() {
            self.clear_selection();
        }
        self.ensure_cursor_visible();
    }

    /// Move cursor down
    pub fn move_down(&mut self) {
        if self.cursor.0 + 1 < self.lines.len() {
            self.move_to_line(self.cursor.0 + 1);
        }
        // Only clear selection if not in selection mode
        if self.anchor.is_none() {
            self.clear_selection();
        }
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
        // Only clear selection if not in selection mode
        if self.anchor.is_none() {
            self.clear_selection();
        }
        self.ensure_cursor_visible();
    }

    /// Move to end of line
    pub fn move_end(&mut self) {
        self.cursor.1 = self.line_len(self.cursor.0);
        // Only clear selection if not in selection mode
        if self.anchor.is_none() {
            self.clear_selection();
        }
        self.ensure_cursor_visible();
    }

    /// Move to start of document
    pub fn move_document_start(&mut self) {
        self.cursor = (0, 0);
        // Only clear selection if not in selection mode
        if self.anchor.is_none() {
            self.clear_selection();
        }
        self.ensure_cursor_visible();
    }

    /// Move to end of document
    pub fn move_document_end(&mut self) {
        let last_line = self.lines.len().saturating_sub(1);
        self.cursor = (last_line, self.line_len(last_line));
        // Only clear selection if not in selection mode
        if self.anchor.is_none() {
            self.clear_selection();
        }
        self.ensure_cursor_visible();
    }

    /// Move by word left
    pub fn move_word_left(&mut self) {
        if self.cursor.1 == 0 {
            if self.cursor.0 > 0 {
                self.cursor.0 -= 1;
                self.cursor.1 = self.line_len(self.cursor.0);
            }
            // Only clear selection if not in selection mode
            if self.anchor.is_none() {
                self.clear_selection();
            }
            self.ensure_cursor_visible();
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
        // Only clear selection if not in selection mode
        if self.anchor.is_none() {
            self.clear_selection();
        }
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
            // Only clear selection if not in selection mode
            if self.anchor.is_none() {
                self.clear_selection();
            }
            self.ensure_cursor_visible();
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
        // Only clear selection if not in selection mode
        if self.anchor.is_none() {
            self.clear_selection();
        }
        self.ensure_cursor_visible();
    }

    /// Page up: scroll the view up `page_size` lines and move the cursor up
    /// the same number, keeping its screen row and column. When the view is
    /// already at the top, the cursor moves to the first line instead.
    pub fn page_up(&mut self, page_size: usize) {
        let page = page_size.max(1);
        let top = self.scroll_for_cursor(page);
        if top == 0 {
            self.scroll.set(top);
            self.move_to_line(0);
        } else {
            let new_top = top.saturating_sub(page);
            self.scroll.set(new_top);
            self.move_to_line(self.cursor.0 - (top - new_top));
        }
        // Only clear selection if not in selection mode
        if self.anchor.is_none() {
            self.clear_selection();
        }
        self.ensure_cursor_visible();
    }

    /// Page down: scroll the view down `page_size` lines and move the cursor
    /// down the same number, keeping its screen row and column. The view
    /// stops with the last line on its bottom row; once there, the cursor
    /// moves to the last line instead.
    pub fn page_down(&mut self, page_size: usize) {
        let page = page_size.max(1);
        let top = self.scroll_for_cursor(page);
        let last = self.lines.len().saturating_sub(1);
        let max_top = self.lines.len().saturating_sub(page);
        let new_top = (top + page).min(max_top);
        if new_top <= top {
            self.scroll.set(top);
            self.move_to_line(last);
        } else {
            self.scroll.set(new_top);
            self.move_to_line((self.cursor.0 + (new_top - top)).min(last));
        }
        // Only clear selection if not in selection mode
        if self.anchor.is_none() {
            self.clear_selection();
        }
        self.ensure_cursor_visible();
    }

    /// The first visible line that keeps the cursor's line in a view
    /// `visible_lines` high, moving the current scroll as little as possible.
    pub(super) fn scroll_for_cursor(&self, visible_lines: usize) -> usize {
        let cursor_line = self.cursor.0.min(self.lines.len().saturating_sub(1));
        self.scroll
            .get()
            .min(cursor_line)
            .max((cursor_line + 1).saturating_sub(visible_lines))
    }

    /// Move the cursor to `line`, keeping its screen column: a char index
    /// alone drifts one column per wide glyph before it.
    fn move_to_line(&mut self, line: usize) {
        let from = self.lines.get(self.cursor.0).map_or("", String::as_str);
        let col = col_of(from, self.cursor.1);
        let to = self.lines.get(line).map_or("", String::as_str);
        self.cursor = (line, char_at_col(to, col));
    }

    /// Ensure cursor is visible. Only scrolling up can be settled here;
    /// render scrolls down, and sideways, once it knows the view size.
    pub(super) fn ensure_cursor_visible(&mut self) {
        if self.cursor.0 < self.scroll.get() {
            self.scroll.set(self.cursor.0);
        }
    }
}
