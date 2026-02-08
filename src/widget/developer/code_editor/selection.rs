//! Code editor selection

#[cfg(test)]
mod tests {
    use super::super::*;

    // =========================================================================
    // start_selection tests
    // =========================================================================

    #[test]
    fn test_start_selection_basic() {
        let mut editor = CodeEditor::new().content("hello world");
        editor.start_selection();
        assert!(editor.has_selection());
        assert_eq!(editor.anchor, Some((0, 0)));
    }

    #[test]
    fn test_start_selection_at_position() {
        let mut editor = CodeEditor::new().content("hello");
        editor.cursor = (0, 2);
        editor.start_selection();
        assert!(editor.has_selection());
        assert_eq!(editor.anchor, Some((0, 2)));
    }

    #[test]
    fn test_start_selection_overwrites() {
        let mut editor = CodeEditor::new().content("test");
        editor.start_selection();
        editor.move_right();
        let anchor1 = editor.anchor;
        editor.start_selection();
        let anchor2 = editor.anchor;
        assert_eq!(anchor1, anchor2);
    }

    // =========================================================================
    // clear_selection tests
    // =========================================================================

    #[test]
    fn test_clear_selection_basic() {
        let mut editor = CodeEditor::new().content("hello");
        editor.start_selection();
        editor.clear_selection();
        assert!(!editor.has_selection());
        assert!(editor.anchor.is_none());
    }

    #[test]
    fn test_clear_selection_when_none() {
        let mut editor = CodeEditor::new();
        editor.clear_selection();
        assert!(!editor.has_selection());
    }

    #[test]
    fn test_clear_selection_after_movement() {
        let mut editor = CodeEditor::new().content("hello world");
        editor.start_selection();
        editor.move_right();
        editor.move_right();
        assert!(editor.has_selection());
        editor.clear_selection();
        assert!(!editor.has_selection());
    }

    // =========================================================================
    // has_selection tests
    // =========================================================================

    #[test]
    fn test_has_selection_false_by_default() {
        let editor = CodeEditor::new();
        assert!(!editor.has_selection());
    }

    #[test]
    fn test_has_selection_true_after_start() {
        let mut editor = CodeEditor::new();
        editor.start_selection();
        assert!(editor.has_selection());
    }

    #[test]
    fn test_has_selection_false_after_clear() {
        let mut editor = CodeEditor::new();
        editor.start_selection();
        editor.clear_selection();
        assert!(!editor.has_selection());
    }

    // =========================================================================
    // get_selection tests
    // =========================================================================

    #[test]
    fn test_get_selection_single_line() {
        let mut editor = CodeEditor::new().content("hello world");
        editor.start_selection();
        editor.move_right();
        editor.move_right();
        editor.move_right();
        editor.move_right();
        editor.move_right();
        let selected = editor.get_selection();
        assert_eq!(selected, Some("hello".to_string()));
    }

    #[test]
    fn test_get_selection_none_when_no_selection() {
        let editor = CodeEditor::new().content("hello");
        assert_eq!(editor.get_selection(), None);
    }

    #[test]
    fn test_get_selection_multi_line() {
        let mut editor = CodeEditor::new().content("line1\nline2\nline3");
        editor.cursor = (0, 3);
        editor.start_selection();
        editor.move_down();
        editor.move_down();
        let selected = editor.get_selection();
        assert!(selected.is_some());
        let text = selected.unwrap();
        assert!(text.contains("line2"));
    }

    #[test]
    fn test_get_selection_reverse() {
        let mut editor = CodeEditor::new().content("hello");
        editor.cursor = (0, 5);
        editor.start_selection();
        editor.move_left();
        editor.move_left();
        let selected = editor.get_selection();
        // Position 5 to 3: "hello"[3..5] = "lo"
        assert_eq!(selected, Some("lo".to_string()));
    }

    // =========================================================================
    // delete_selection tests
    // =========================================================================

    #[test]
    fn test_delete_selection_single_line() {
        let mut editor = CodeEditor::new().content("hello world");
        editor.start_selection();
        for _ in 0..6 {
            editor.move_right();
        }
        editor.delete_selection();
        assert_eq!(editor.get_content(), "world");
        assert!(!editor.has_selection());
    }

    #[test]
    fn test_delete_selection_multi_line() {
        let mut editor = CodeEditor::new().content("line1\nline2\nline3");
        editor.start_selection();
        editor.move_down();
        editor.move_down();
        editor.delete_selection();
        assert!(editor.lines.len() < 3);
        assert!(!editor.has_selection());
    }

    #[test]
    fn test_delete_selection_when_none() {
        let mut editor = CodeEditor::new().content("hello");
        editor.delete_selection();
        assert_eq!(editor.get_content(), "hello");
    }

    #[test]
    fn test_delete_selection_clears_anchor() {
        let mut editor = CodeEditor::new().content("test");
        editor.start_selection();
        editor.move_right();
        editor.delete_selection();
        assert!(editor.anchor.is_none());
    }

    #[test]
    fn test_delete_selection_moves_cursor() {
        let mut editor = CodeEditor::new().content("hello world");
        editor.start_selection();
        for _ in 0..6 {
            editor.move_right();
        }
        editor.delete_selection();
        assert_eq!(editor.cursor, (0, 0));
    }

    // =========================================================================
    // select_all tests
    // =========================================================================

    #[test]
    fn test_select_all_basic() {
        let mut editor = CodeEditor::new().content("hello\nworld");
        editor.select_all();
        assert!(editor.has_selection());
        let selected = editor.get_selection();
        assert_eq!(selected, Some("hello\nworld".to_string()));
    }

    #[test]
    fn test_select_all_empty() {
        let mut editor = CodeEditor::new();
        editor.select_all();
        assert!(editor.has_selection());
        assert_eq!(editor.anchor, Some((0, 0)));
        assert_eq!(editor.cursor, (0, 0));
    }

    #[test]
    fn test_select_all_single_line() {
        let mut editor = CodeEditor::new().content("hello");
        editor.select_all();
        assert!(editor.has_selection());
        assert_eq!(editor.get_selection(), Some("hello".to_string()));
    }

    #[test]
    fn test_select_all_multiple_lines() {
        let mut editor = CodeEditor::new().content("a\nb\nc\nd");
        editor.select_all();
        assert_eq!(editor.anchor, Some((0, 0)));
        assert_eq!(editor.cursor, (3, 1));
    }

    #[test]
    fn test_select_all_overwrites() {
        let mut editor = CodeEditor::new().content("test");
        editor.start_selection();
        editor.move_right();
        editor.select_all();
        assert_eq!(editor.anchor, Some((0, 0)));
    }
}

impl super::CodeEditor {
    // =========================================================================
    // Selection
    // =========================================================================

    /// Start selection
    pub fn start_selection(&mut self) {
        // Only set anchor if there isn't one already (don't overwrite existing selection)
        if self.anchor.is_none() {
            self.anchor = Some(self.cursor);
        }
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.anchor = None;
    }

    /// Check if there's a selection
    pub fn has_selection(&self) -> bool {
        self.anchor.is_some()
    }

    /// Get selected text
    pub fn get_selection(&self) -> Option<String> {
        let anchor = self.anchor?;
        let (start, end) = if anchor < self.cursor {
            (anchor, self.cursor)
        } else {
            (self.cursor, anchor)
        };

        let mut result = String::new();
        for line_idx in start.0..=end.0 {
            if line_idx >= self.lines.len() {
                break;
            }
            let line = &self.lines[line_idx];
            let start_col = if line_idx == start.0 { start.1 } else { 0 };
            let end_col = if line_idx == end.0 { end.1 } else { line.len() };

            if start_col < line.len() {
                result.push_str(&line[start_col..end_col.min(line.len())]);
            }
            if line_idx < end.0 {
                result.push('\n');
            }
        }

        Some(result)
    }

    /// Delete selection
    pub fn delete_selection(&mut self) {
        let anchor = match self.anchor {
            Some(a) => a,
            None => return,
        };

        let (start, end) = if anchor < self.cursor {
            (anchor, self.cursor)
        } else {
            (self.cursor, anchor)
        };

        if start.0 == end.0 {
            // Single line
            if let Some(line) = self.lines.get_mut(start.0) {
                let deleted: String = line.drain(start.1..end.1.min(line.len())).collect();
                self.push_undo(super::types::EditOp::Delete {
                    line: start.0,
                    col: start.1,
                    text: deleted,
                });
            }
        } else {
            // Multi-line
            let before: String = self.lines[start.0].chars().take(start.1).collect();
            let after: String = self.lines[end.0].chars().skip(end.1).collect();

            for _ in start.0..=end.0 {
                if start.0 < self.lines.len() {
                    self.lines.remove(start.0);
                }
            }

            self.lines.insert(start.0, format!("{}{}", before, after));
        }

        self.cursor = start;
        self.anchor = None;
    }

    /// Select all
    pub fn select_all(&mut self) {
        self.anchor = Some((0, 0));
        let last_line = self.lines.len().saturating_sub(1);
        self.cursor = (last_line, self.line_len(last_line));
    }
}
