//! Code editor cursor and navigation

#[cfg(test)]
mod tests {
    use super::super::*;

    // =========================================================================
    // cursor_position tests
    // =========================================================================

    #[test]
    fn test_cursor_position_default() {
        let editor = CodeEditor::new();
        assert_eq!(editor.cursor_position(), (0, 0));
    }

    #[test]
    fn test_cursor_position_after_movement() {
        let mut editor = CodeEditor::new().content("ab\ncd");
        editor.move_right();
        editor.move_down();
        assert_eq!(editor.cursor_position(), (1, 1));
    }

    // =========================================================================
    // set_cursor tests
    // =========================================================================

    #[test]
    fn test_set_cursor_basic() {
        let mut editor = CodeEditor::new().content("line1\nline2\nline3");
        editor.set_cursor(1, 2);
        assert_eq!(editor.cursor, (1, 2));
    }

    #[test]
    fn test_set_cursor_out_of_bounds_line() {
        let mut editor = CodeEditor::new().content("line1\nline2");
        editor.set_cursor(10, 0);
        assert_eq!(editor.cursor.0, 1); // Clamped to last line
    }

    #[test]
    fn test_set_cursor_out_of_bounds_col() {
        let mut editor = CodeEditor::new().content("hello");
        editor.set_cursor(0, 100);
        assert_eq!(editor.cursor.1, 5); // Clamped to line length
    }

    #[test]
    fn test_set_cursor_extends_selection() {
        let mut editor = CodeEditor::new().content("test");
        editor.start_selection();
        editor.move_right();
        editor.set_cursor(0, 3);
        // set_cursor should extend selection when in selection mode
        assert!(editor.has_selection());
    }

    // =========================================================================
    // line_count tests
    // =========================================================================

    #[test]
    fn test_line_count_default() {
        let editor = CodeEditor::new();
        assert_eq!(editor.line_count(), 1);
    }

    #[test]
    fn test_line_count_multiple() {
        let editor = CodeEditor::new().content("a\nb\nc\nd");
        assert_eq!(editor.line_count(), 4);
    }

    #[test]
    fn test_line_count_empty() {
        let editor = CodeEditor::new().content("");
        assert_eq!(editor.line_count(), 1);
    }

    // =========================================================================
    // move_left tests
    // =========================================================================

    #[test]
    fn test_move_left_basic() {
        let mut editor = CodeEditor::new().content("hello");
        editor.cursor = (0, 3);
        editor.move_left();
        assert_eq!(editor.cursor, (0, 2));
    }

    #[test]
    fn test_move_left_at_start() {
        let mut editor = CodeEditor::new().content("hello");
        editor.cursor = (0, 0);
        editor.move_left();
        assert_eq!(editor.cursor, (0, 0));
    }

    #[test]
    fn test_move_left_to_previous_line() {
        let mut editor = CodeEditor::new().content("line1\nline2");
        editor.cursor = (1, 0);
        editor.move_left();
        assert_eq!(editor.cursor, (0, 5));
    }

    #[test]
    fn test_move_left_extends_selection() {
        let mut editor = CodeEditor::new().content("test");
        editor.start_selection();
        editor.move_right();
        editor.move_right();
        editor.move_left();
        // Movement should extend selection when in selection mode
        assert!(editor.has_selection());
    }

    // =========================================================================
    // move_right tests
    // =========================================================================

    #[test]
    fn test_move_right_basic() {
        let mut editor = CodeEditor::new().content("hello");
        editor.move_right();
        assert_eq!(editor.cursor, (0, 1));
    }

    #[test]
    fn test_move_right_at_end() {
        let mut editor = CodeEditor::new().content("hi");
        editor.cursor = (0, 2);
        editor.move_right();
        assert_eq!(editor.cursor, (0, 2));
    }

    #[test]
    fn test_move_right_to_next_line() {
        let mut editor = CodeEditor::new().content("line1\nline2");
        editor.cursor = (0, 5);
        editor.move_right();
        assert_eq!(editor.cursor, (1, 0));
    }

    #[test]
    fn test_move_right_extends_selection() {
        let mut editor = CodeEditor::new().content("test");
        editor.start_selection();
        editor.move_right();
        editor.move_right();
        // Movement should extend selection when in selection mode
        assert!(editor.has_selection());
    }

    // =========================================================================
    // move_up tests
    // =========================================================================

    #[test]
    fn test_move_up_basic() {
        let mut editor = CodeEditor::new().content("line1\nline2\nline3");
        editor.cursor = (2, 3);
        editor.move_up();
        assert_eq!(editor.cursor, (1, 3));
    }

    #[test]
    fn test_move_up_at_top() {
        let mut editor = CodeEditor::new().content("line1\nline2");
        editor.cursor = (0, 2);
        editor.move_up();
        assert_eq!(editor.cursor, (0, 2));
    }

    #[test]
    fn test_move_up_clamps_col() {
        let mut editor = CodeEditor::new().content("short\nlonger line");
        editor.cursor = (1, 10);
        editor.move_up();
        assert_eq!(editor.cursor, (0, 5)); // Clamped to "short" length
    }

    #[test]
    fn test_move_up_extends_selection() {
        let mut editor = CodeEditor::new().content("a\nb");
        editor.cursor = (1, 0);
        editor.start_selection();
        editor.move_up();
        // Movement should extend selection when in selection mode
        assert!(editor.has_selection());
    }

    // =========================================================================
    // move_down tests
    // =========================================================================

    #[test]
    fn test_move_down_basic() {
        let mut editor = CodeEditor::new().content("line1\nline2\nline3");
        editor.cursor = (0, 3);
        editor.move_down();
        assert_eq!(editor.cursor, (1, 3));
    }

    #[test]
    fn test_move_down_at_bottom() {
        let mut editor = CodeEditor::new().content("line1\nline2");
        editor.cursor = (1, 2);
        editor.move_down();
        assert_eq!(editor.cursor, (1, 2));
    }

    #[test]
    fn test_move_down_clamps_col() {
        let mut editor = CodeEditor::new().content("longer line\nshort");
        editor.cursor = (0, 10);
        editor.move_down();
        assert_eq!(editor.cursor, (1, 5)); // Clamped to "short" length
    }

    #[test]
    fn test_move_down_extends_selection() {
        let mut editor = CodeEditor::new().content("a\nb");
        editor.start_selection();
        editor.move_down();
        // Movement should extend selection when in selection mode
        assert!(editor.has_selection());
    }

    // =========================================================================
    // move_home tests
    // =========================================================================

    #[test]
    fn test_move_home_from_middle() {
        let mut editor = CodeEditor::new().content("    hello");
        editor.cursor = (0, 6);
        editor.move_home();
        assert_eq!(editor.cursor, (0, 4)); // First non-whitespace
    }

    #[test]
    fn test_move_home_from_start() {
        let mut editor = CodeEditor::new().content("hello");
        editor.cursor = (0, 3);
        editor.move_home();
        assert_eq!(editor.cursor, (0, 0));
    }

    #[test]
    fn test_move_home_toggle() {
        let mut editor = CodeEditor::new().content("    test");
        editor.cursor = (0, 8);
        editor.move_home();
        let pos1 = editor.cursor;
        editor.move_home();
        let pos2 = editor.cursor;
        assert_ne!(pos1, pos2);
    }

    #[test]
    fn test_move_home_extends_selection() {
        let mut editor = CodeEditor::new().content("  test");
        editor.start_selection();
        editor.move_home();
        // Movement should extend selection when in selection mode
        assert!(editor.has_selection());
    }

    // =========================================================================
    // move_end tests
    // =========================================================================

    #[test]
    fn test_move_end_basic() {
        let mut editor = CodeEditor::new().content("hello");
        editor.cursor = (0, 2);
        editor.move_end();
        assert_eq!(editor.cursor, (0, 5));
    }

    #[test]
    fn test_move_end_empty_line() {
        let mut editor = CodeEditor::new().content("hello\n\nworld");
        editor.cursor = (1, 0);
        editor.move_end();
        assert_eq!(editor.cursor, (1, 0));
    }

    #[test]
    fn test_move_end_extends_selection() {
        let mut editor = CodeEditor::new().content("test");
        editor.start_selection();
        editor.move_end();
        // Movement should extend selection when in selection mode
        assert!(editor.has_selection());
    }

    // =========================================================================
    // move_document_start tests
    // =========================================================================

    #[test]
    fn test_move_document_start_basic() {
        let mut editor = CodeEditor::new().content("line1\nline2\nline3");
        editor.cursor = (2, 5);
        editor.move_document_start();
        assert_eq!(editor.cursor, (0, 0));
    }

    #[test]
    fn test_move_document_start_extends_selection() {
        let mut editor = CodeEditor::new().content("a\nb\nc");
        editor.cursor = (2, 0);
        editor.start_selection();
        editor.move_document_start();
        // Movement should extend selection when in selection mode
        assert!(editor.has_selection());
    }

    // =========================================================================
    // move_document_end tests
    // =========================================================================

    #[test]
    fn test_move_document_end_basic() {
        let mut editor = CodeEditor::new().content("line1\nline2\nline3");
        editor.move_document_end();
        assert_eq!(editor.cursor, (2, 5));
    }

    #[test]
    fn test_move_document_end_empty() {
        let mut editor = CodeEditor::new();
        editor.move_document_end();
        assert_eq!(editor.cursor, (0, 0));
    }

    #[test]
    fn test_move_document_end_extends_selection() {
        let mut editor = CodeEditor::new().content("a\nb");
        editor.start_selection();
        editor.move_document_end();
        // Movement should extend selection when in selection mode
        assert!(editor.has_selection());
    }

    // =========================================================================
    // move_word_left tests
    // =========================================================================

    #[test]
    fn test_move_word_left_basic() {
        let mut editor = CodeEditor::new().content("hello world");
        editor.cursor = (0, 8);
        editor.move_word_left();
        assert_eq!(editor.cursor, (0, 6)); // Start of "world"
    }

    #[test]
    fn test_move_word_left_at_start() {
        let mut editor = CodeEditor::new().content("hello");
        editor.cursor = (0, 0);
        editor.move_word_left();
        assert_eq!(editor.cursor, (0, 0));
    }

    #[test]
    fn test_move_word_left_over_whitespace() {
        let mut editor = CodeEditor::new().content("hello   world");
        editor.cursor = (0, 10);
        editor.move_word_left();
        // From position 10 ('r'), move_word_left goes to start of "world" (position 8)
        assert_eq!(editor.cursor, (0, 8));
    }

    #[test]
    fn test_move_word_left_to_previous_line() {
        let mut editor = CodeEditor::new().content("hello\nworld");
        editor.cursor = (1, 0);
        editor.move_word_left();
        assert_eq!(editor.cursor, (0, 5));
    }

    #[test]
    fn test_move_word_left_extends_selection() {
        let mut editor = CodeEditor::new().content("hello world");
        editor.cursor = (0, 11);
        editor.start_selection();
        editor.move_word_left();
        // Movement should extend selection when in selection mode
        assert!(editor.has_selection());
    }

    // =========================================================================
    // move_word_right tests
    // =========================================================================

    #[test]
    fn test_move_word_right_basic() {
        let mut editor = CodeEditor::new().content("hello world");
        editor.cursor = (0, 2);
        editor.move_word_right();
        assert_eq!(editor.cursor, (0, 6)); // After "hello"
    }

    #[test]
    fn test_move_word_right_at_end() {
        let mut editor = CodeEditor::new().content("hello");
        editor.cursor = (0, 5);
        editor.move_word_right();
        assert_eq!(editor.cursor, (0, 5));
    }

    #[test]
    fn test_move_word_right_over_whitespace() {
        let mut editor = CodeEditor::new().content("hello   world");
        editor.cursor = (0, 2);
        editor.move_word_right();
        // move_word_right lands at start of next word ("world" at position 8)
        assert_eq!(editor.cursor, (0, 8));
    }

    #[test]
    fn test_move_word_right_to_next_line() {
        let mut editor = CodeEditor::new().content("hello\nworld");
        editor.cursor = (0, 5);
        editor.move_word_right();
        assert_eq!(editor.cursor, (1, 0)); // Start of next line
    }

    #[test]
    fn test_move_word_right_extends_selection() {
        let mut editor = CodeEditor::new().content("hello world");
        editor.start_selection();
        editor.move_word_right();
        // Movement should extend selection when in selection mode
        assert!(editor.has_selection());
    }

    // =========================================================================
    // page_up tests
    // =========================================================================

    #[test]
    fn test_page_up_basic() {
        let mut editor = CodeEditor::new().content("1\n2\n3\n4\n5\n6\n7\n8\n9\n10");
        editor.cursor = (9, 0);
        editor.page_up(5);
        assert_eq!(editor.cursor, (4, 0));
    }

    #[test]
    fn test_page_up_clamps_to_top() {
        let mut editor = CodeEditor::new().content("1\n2\n3");
        editor.cursor = (2, 0);
        editor.page_up(10);
        assert_eq!(editor.cursor, (0, 0));
    }

    #[test]
    fn test_page_up_extends_selection() {
        let mut editor = CodeEditor::new().content("1\n2\n3\n4\n5");
        editor.cursor = (4, 0);
        editor.start_selection();
        editor.page_up(2);
        // Movement should extend selection when in selection mode
        assert!(editor.has_selection());
    }

    // =========================================================================
    // page_down tests
    // =========================================================================

    #[test]
    fn test_page_down_basic() {
        let mut editor = CodeEditor::new().content("1\n2\n3\n4\n5\n6\n7\n8\n9\n10");
        editor.cursor = (0, 0);
        editor.page_down(5);
        assert_eq!(editor.cursor, (5, 0));
    }

    #[test]
    fn test_page_down_clamps_to_bottom() {
        let mut editor = CodeEditor::new().content("1\n2\n3");
        editor.cursor = (0, 0);
        editor.page_down(10);
        assert_eq!(editor.cursor, (2, 0));
    }

    #[test]
    fn test_page_down_extends_selection() {
        let mut editor = CodeEditor::new().content("1\n2\n3\n4\n5");
        editor.start_selection();
        editor.page_down(2);
        // Movement should extend selection when in selection mode
        assert!(editor.has_selection());
    }
}

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
            self.cursor.0 -= 1;
            self.cursor.1 = self.cursor.1.min(self.line_len(self.cursor.0));
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
            self.cursor.0 += 1;
            self.cursor.1 = self.cursor.1.min(self.line_len(self.cursor.0));
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

    /// Page up
    pub fn page_up(&mut self, page_size: usize) {
        self.cursor.0 = self.cursor.0.saturating_sub(page_size);
        self.cursor.1 = self.cursor.1.min(self.line_len(self.cursor.0));
        // Only clear selection if not in selection mode
        if self.anchor.is_none() {
            self.clear_selection();
        }
        self.ensure_cursor_visible();
    }

    /// Page down
    pub fn page_down(&mut self, page_size: usize) {
        self.cursor.0 = (self.cursor.0 + page_size).min(self.lines.len().saturating_sub(1));
        self.cursor.1 = self.cursor.1.min(self.line_len(self.cursor.0));
        // Only clear selection if not in selection mode
        if self.anchor.is_none() {
            self.clear_selection();
        }
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
