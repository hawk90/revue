//! Code editor modes (go-to-line, find)

use crate::event::Key;

impl super::CodeEditor {
    // =========================================================================
    // Go-to-Line
    // =========================================================================

    /// Open go-to-line dialog
    pub fn open_goto_line(&mut self) {
        self.goto_line_mode = true;
        self.goto_line_input.clear();
    }

    /// Close go-to-line dialog
    pub fn close_goto_line(&mut self) {
        self.goto_line_mode = false;
        self.goto_line_input.clear();
    }

    /// Check if go-to-line is active
    pub fn is_goto_line_active(&self) -> bool {
        self.goto_line_mode
    }

    /// Go to specific line
    pub fn goto_line(&mut self, line: usize) {
        let target = line
            .saturating_sub(1)
            .min(self.lines.len().saturating_sub(1));
        self.cursor = (target, 0);
        self.clear_selection();
        self.ensure_cursor_visible();
        self.close_goto_line();
    }

    /// Handle go-to-line input
    pub(super) fn handle_goto_input(&mut self, key: &Key) -> bool {
        match key {
            Key::Char(ch) if ch.is_ascii_digit() => {
                self.goto_line_input.push(*ch);
                true
            }
            Key::Backspace => {
                self.goto_line_input.pop();
                true
            }
            Key::Enter => {
                if let Ok(line) = self.goto_line_input.parse::<usize>() {
                    self.goto_line(line);
                }
                self.close_goto_line();
                true
            }
            Key::Escape => {
                self.close_goto_line();
                true
            }
            _ => false,
        }
    }

    // =========================================================================
    // Find
    // =========================================================================

    /// Open find dialog
    pub fn open_find(&mut self) {
        self.find_mode = true;
        self.find_query.clear();
        self.find_matches.clear();
        self.find_index = 0;
    }

    /// Close find dialog
    pub fn close_find(&mut self) {
        self.find_mode = false;
    }

    /// Check if find is active
    pub fn is_find_active(&self) -> bool {
        self.find_mode
    }

    /// Set find query
    pub fn set_find_query(&mut self, query: &str) {
        self.find_query = query.to_string();
        self.refresh_find_matches();
    }

    /// Refresh find matches
    pub(super) fn refresh_find_matches(&mut self) {
        self.find_matches.clear();

        if self.find_query.is_empty() {
            return;
        }

        let query_lower = self.find_query.to_lowercase();
        for (line_idx, line) in self.lines.iter().enumerate() {
            let line_lower = line.to_lowercase();
            let mut start = 0;
            while let Some(pos) = line_lower[start..].find(&query_lower) {
                let match_start = start + pos;
                let match_end = match_start + self.find_query.len();
                self.find_matches.push((line_idx, match_start, match_end));
                start = match_start + 1;
            }
        }

        if !self.find_matches.is_empty() && self.find_index >= self.find_matches.len() {
            self.find_index = 0;
        }
    }

    /// Find next match
    pub fn find_next(&mut self) {
        if self.find_matches.is_empty() {
            return;
        }

        self.find_index = (self.find_index + 1) % self.find_matches.len();
        self.jump_to_current_match();
    }

    /// Find previous match
    pub fn find_previous(&mut self) {
        if self.find_matches.is_empty() {
            return;
        }

        self.find_index = if self.find_index == 0 {
            self.find_matches.len() - 1
        } else {
            self.find_index - 1
        };
        self.jump_to_current_match();
    }

    /// Jump to current match
    pub(super) fn jump_to_current_match(&mut self) {
        if let Some(&(line, col, _)) = self.find_matches.get(self.find_index) {
            self.cursor = (line, col);
            self.ensure_cursor_visible();
        }
    }

    /// Get find match count
    pub fn find_match_count(&self) -> usize {
        self.find_matches.len()
    }

    /// Get current find index (1-based)
    pub fn current_find_index(&self) -> usize {
        if self.find_matches.is_empty() {
            0
        } else {
            self.find_index + 1
        }
    }

    /// Handle find input
    pub(super) fn handle_find_input(&mut self, key: &Key) -> bool {
        match key {
            Key::Char(ch) => {
                self.find_query.push(*ch);
                self.refresh_find_matches();
                if !self.find_matches.is_empty() {
                    self.jump_to_current_match();
                }
                true
            }
            Key::Backspace => {
                self.find_query.pop();
                self.refresh_find_matches();
                true
            }
            Key::Enter => {
                self.find_next();
                true
            }
            Key::Escape => {
                self.close_find();
                true
            }
            _ => false,
        }
    }
}

// Public API tests extracted to tests/widget/code_editor/modes.rs
// KEEP HERE - Private helper method tests (handle_goto_input, handle_find_input)

#[cfg(test)]
mod tests {
    use super::super::CodeEditor;
    use crate::event::Key;

    // =========================================================================
    // Input handling tests (private methods)
    // =========================================================================

    #[test]
    fn test_handle_goto_input_digit() {
        let mut editor = CodeEditor::default();
        editor.open_goto_line();
        let handled = editor.handle_goto_input(&Key::Char('5'));
        assert!(handled);
    }

    #[test]
    fn test_handle_goto_input_backspace() {
        let mut editor = CodeEditor::default();
        editor.open_goto_line();
        let handled = editor.handle_goto_input(&Key::Backspace);
        assert!(handled);
    }

    #[test]
    fn test_handle_goto_input_enter() {
        let mut editor = CodeEditor::default();
        editor.open_goto_line();
        // Enter closes goto line even with empty input
        let handled = editor.handle_goto_input(&Key::Enter);
        assert!(handled);
        assert!(!editor.is_goto_line_active());
    }

    #[test]
    fn test_handle_goto_input_escape() {
        let mut editor = CodeEditor::default();
        editor.open_goto_line();
        let handled = editor.handle_goto_input(&Key::Escape);
        assert!(handled);
        assert!(!editor.is_goto_line_active());
    }

    #[test]
    fn test_handle_find_input_char() {
        let mut editor = CodeEditor::default();
        editor.open_find();
        let handled = editor.handle_find_input(&Key::Char('a'));
        assert!(handled);
    }

    #[test]
    fn test_handle_find_input_backspace() {
        let mut editor = CodeEditor::default();
        editor.open_find();
        editor.handle_find_input(&Key::Char('a'));
        let handled = editor.handle_find_input(&Key::Backspace);
        assert!(handled);
    }

    #[test]
    fn test_handle_find_input_enter() {
        let mut editor = CodeEditor::default();
        editor.open_find();
        let handled = editor.handle_find_input(&Key::Enter);
        assert!(handled);
    }

    #[test]
    fn test_handle_find_input_escape() {
        let mut editor = CodeEditor::default();
        editor.open_find();
        let handled = editor.handle_find_input(&Key::Escape);
        assert!(handled);
        assert!(!editor.is_find_active());
    }
}
