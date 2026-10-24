//! Code editor modes (go-to-line, find)

use crate::event::{Key, KeyEvent};
use crate::utils::text::find_chars;
use crate::utils::EditOutcome;

impl super::CodeEditor {
    // =========================================================================
    // Go-to-Line
    // =========================================================================

    /// Open go-to-line dialog
    pub fn open_goto_line(&mut self) {
        self.goto_line_mode = true;
        self.goto_line_input.set_text("");
    }

    /// Close go-to-line dialog
    pub fn close_goto_line(&mut self) {
        self.goto_line_mode = false;
        self.goto_line_input.set_text("");
    }

    /// Check if go-to-line is active
    pub fn is_goto_line_active(&self) -> bool {
        self.goto_line_mode
    }

    /// Go to specific line
    pub fn goto_line(&mut self, line: usize) {
        self.clear_selection();
        self.set_cursor(line.saturating_sub(1), 0);
        self.close_goto_line();
    }

    /// Handle go-to-line input: Enter goes, Escape closes, and the other
    /// keys edit the line number as every single-line field is edited
    /// (#985), typing digits only
    pub(super) fn handle_goto_input(&mut self, event: &KeyEvent) -> bool {
        match event.key {
            Key::Enter => {
                let line = self.goto_line_input.line(0).unwrap_or("").parse::<usize>();
                if let Ok(line) = line {
                    self.goto_line(line);
                }
                self.close_goto_line();
                true
            }
            Key::Escape => {
                self.close_goto_line();
                true
            }
            // Only digits are typed; other letters are not this box's
            Key::Char(c) if event.ctrl == event.alt && !c.is_ascii_digit() => false,
            _ => self.goto_line_input.edit_key(event, 1) != EditOutcome::Unhandled,
        }
    }

    // =========================================================================
    // Find
    // =========================================================================

    /// Open find dialog
    pub fn open_find(&mut self) {
        self.find_mode = true;
        self.find_query.set_text("");
        self.find_matches.clear();
        self.find_index = 0;
    }

    /// Close find dialog
    pub fn close_find(&mut self) {
        self.find_mode = false;
        // The highlights go with the dialog (#975)
        self.find_matches.clear();
    }

    /// Check if find is active
    pub fn is_find_active(&self) -> bool {
        self.find_mode
    }

    /// Set find query
    pub fn set_find_query(&mut self, query: &str) {
        self.find_query.set_text(query);
        let end = self.find_query.end();
        self.find_query.set_cursor(end);
        self.refresh_find_matches();
    }

    /// Refresh find matches
    pub(super) fn refresh_find_matches(&mut self) {
        self.find_matches.clear();

        if self.find_query.is_empty() {
            return;
        }
        let query = self.find_query.line(0).unwrap_or("");

        // Columns are char indices: search on chars, case-insensitively. A
        // byte offset from `str::find` is not a column, and restarting one
        // byte after a match slices into a multibyte char.
        let query: Vec<char> = query.chars().collect();
        let mut matches = Vec::new();
        for (line_idx, line) in self.edit.lines().iter().enumerate() {
            let chars: Vec<char> = line.chars().collect();
            let mut start = 0;
            while let Some(match_start) = find_chars(&chars, &query, start, false) {
                let match_end = match_start + query.len();
                matches.push((line_idx, match_start, match_end));
                start = match_start + 1;
            }
        }
        // A match must start and end between characters the user sees as
        // one: the cursor cannot go inside one
        let on_boundary =
            |line: usize, col: usize| self.edit.char_index(self.edit.pos_at_char(line, col)) == col;
        matches.retain(|&(line, start, end)| on_boundary(line, start) && on_boundary(line, end));
        self.find_matches = matches;

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
            self.set_cursor(line, col);
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

    /// Handle find input: Enter goes to the next match, Escape closes, and
    /// the other keys edit the query as every single-line field is edited
    /// (#985); an edit finds the matches again and goes to the first
    pub(super) fn handle_find_input(&mut self, event: &KeyEvent) -> bool {
        match event.key {
            Key::Enter => {
                self.find_next();
                true
            }
            Key::Escape => {
                self.close_find();
                true
            }
            _ => match self.find_query.edit_key(event, 1) {
                EditOutcome::Edited => {
                    self.refresh_find_matches();
                    if !self.find_matches.is_empty() {
                        self.jump_to_current_match();
                    }
                    true
                }
                EditOutcome::Moved => true,
                EditOutcome::Unhandled => false,
            },
        }
    }
}

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
        let handled = editor.handle_goto_input(&crate::event::KeyEvent::new(Key::Char('5')));
        assert!(handled);
    }

    #[test]
    fn test_handle_goto_input_backspace() {
        let mut editor = CodeEditor::default();
        editor.open_goto_line();
        let handled = editor.handle_goto_input(&crate::event::KeyEvent::new(Key::Backspace));
        assert!(handled);
    }

    #[test]
    fn test_handle_goto_input_enter() {
        let mut editor = CodeEditor::default();
        editor.open_goto_line();
        // Enter closes goto line even with empty input
        let handled = editor.handle_goto_input(&crate::event::KeyEvent::new(Key::Enter));
        assert!(handled);
        assert!(!editor.is_goto_line_active());
    }

    #[test]
    fn test_handle_goto_input_escape() {
        let mut editor = CodeEditor::default();
        editor.open_goto_line();
        let handled = editor.handle_goto_input(&crate::event::KeyEvent::new(Key::Escape));
        assert!(handled);
        assert!(!editor.is_goto_line_active());
    }

    #[test]
    fn test_handle_find_input_char() {
        let mut editor = CodeEditor::default();
        editor.open_find();
        let handled = editor.handle_find_input(&crate::event::KeyEvent::new(Key::Char('a')));
        assert!(handled);
    }

    #[test]
    fn test_handle_find_input_backspace() {
        let mut editor = CodeEditor::default();
        editor.open_find();
        editor.handle_find_input(&crate::event::KeyEvent::new(Key::Char('a')));
        let handled = editor.handle_find_input(&crate::event::KeyEvent::new(Key::Backspace));
        assert!(handled);
    }

    #[test]
    fn test_handle_find_input_enter() {
        let mut editor = CodeEditor::default();
        editor.open_find();
        let handled = editor.handle_find_input(&crate::event::KeyEvent::new(Key::Enter));
        assert!(handled);
    }

    #[test]
    fn test_handle_find_input_escape() {
        let mut editor = CodeEditor::default();
        editor.open_find();
        let handled = editor.handle_find_input(&crate::event::KeyEvent::new(Key::Escape));
        assert!(handled);
        assert!(!editor.is_find_active());
    }
}
