//! Code editor bracket matching

#[cfg(test)]
mod tests {
    use super::super::*;

    // =========================================================================
    // find_matching_bracket tests
    // =========================================================================

    #[test]
    fn test_find_matching_bracket_paren_forward() {
        let mut editor = CodeEditor::new()
            .content("function()")
            .bracket_matching(true);
        // Position 8 is the opening '(' in "function()"
        editor.set_cursor(0, 8);
        let result = editor.find_matching_bracket();
        assert!(result.is_some());
        assert_eq!(result.unwrap().position, (0, 9));
    }

    #[test]
    fn test_find_matching_bracket_paren_backward() {
        let mut editor = CodeEditor::new()
            .content("function()")
            .bracket_matching(true);
        // Position 9 is the closing ')' in "function()"
        editor.set_cursor(0, 9);
        let result = editor.find_matching_bracket();
        assert!(result.is_some());
        assert_eq!(result.unwrap().position, (0, 8));
    }

    #[test]
    fn test_find_matching_bracket_bracket_forward() {
        let mut editor = CodeEditor::new().content("array[0]").bracket_matching(true);
        editor.set_cursor(0, 5);
        let result = editor.find_matching_bracket();
        assert!(result.is_some());
        assert_eq!(result.unwrap().position, (0, 7));
    }

    #[test]
    fn test_find_matching_bracket_bracket_backward() {
        let mut editor = CodeEditor::new().content("array[0]").bracket_matching(true);
        editor.set_cursor(0, 7);
        let result = editor.find_matching_bracket();
        assert!(result.is_some());
        assert_eq!(result.unwrap().position, (0, 5));
    }

    #[test]
    fn test_find_matching_bracket_brace_forward() {
        let mut editor = CodeEditor::new()
            .content("{ key: value }")
            .bracket_matching(true);
        editor.set_cursor(0, 0);
        let result = editor.find_matching_bracket();
        assert!(result.is_some());
        assert_eq!(result.unwrap().position, (0, 13));
    }

    #[test]
    fn test_find_matching_bracket_brace_backward() {
        let mut editor = CodeEditor::new()
            .content("{ key: value }")
            .bracket_matching(true);
        editor.set_cursor(0, 13);
        let result = editor.find_matching_bracket();
        assert!(result.is_some());
        assert_eq!(result.unwrap().position, (0, 0));
    }

    #[test]
    fn test_find_matching_bracket_nested() {
        let mut editor = CodeEditor::new()
            .content("func(()())")
            .bracket_matching(true);
        editor.set_cursor(0, 4);
        let result = editor.find_matching_bracket();
        assert!(result.is_some());
        // Should match the outermost closing paren at position 9
        assert_eq!(result.unwrap().position, (0, 9));
    }

    #[test]
    fn test_find_matching_bracket_no_match() {
        let mut editor = CodeEditor::new()
            .content("function(")
            .bracket_matching(true);
        editor.set_cursor(0, 9);
        let result = editor.find_matching_bracket();
        assert!(result.is_none());
    }

    #[test]
    fn test_find_matching_bracket_disabled() {
        let mut editor = CodeEditor::new()
            .content("function()")
            .bracket_matching(false);
        editor.set_cursor(0, 9);
        let result = editor.find_matching_bracket();
        assert!(result.is_none());
    }

    #[test]
    fn test_find_matching_bracket_non_bracket() {
        let mut editor = CodeEditor::new().content("hello").bracket_matching(true);
        editor.set_cursor(0, 2);
        let result = editor.find_matching_bracket();
        assert!(result.is_none());
    }

    #[test]
    fn test_find_matching_bracket_out_of_bounds() {
        let mut editor = CodeEditor::new().content("()").bracket_matching(true);
        editor.set_cursor(0, 10);
        let result = editor.find_matching_bracket();
        assert!(result.is_none());
    }

    #[test]
    fn test_find_matching_bracket_multiline() {
        let mut editor = CodeEditor::new().content("(\n)").bracket_matching(true);
        editor.set_cursor(0, 0);
        let result = editor.find_matching_bracket();
        assert!(result.is_some());
        assert_eq!(result.unwrap().position, (1, 0));
    }

    #[test]
    fn test_find_matching_bracket_char_paren() {
        let mut editor = CodeEditor::new().content("()").bracket_matching(true);
        editor.set_cursor(0, 0);
        let result = editor.find_matching_bracket();
        assert_eq!(result.unwrap().char, ')');
    }

    #[test]
    fn test_find_matching_bracket_char_bracket() {
        let mut editor = CodeEditor::new().content("[]").bracket_matching(true);
        editor.set_cursor(0, 0);
        let result = editor.find_matching_bracket();
        assert_eq!(result.unwrap().char, ']');
    }

    #[test]
    fn test_find_matching_bracket_char_brace() {
        let mut editor = CodeEditor::new().content("{}").bracket_matching(true);
        editor.set_cursor(0, 0);
        let result = editor.find_matching_bracket();
        assert_eq!(result.unwrap().char, '}');
    }
}

use super::types::BracketMatch;

impl super::CodeEditor {
    // =========================================================================
    // Bracket Matching
    // =========================================================================

    /// Find matching bracket at cursor
    pub fn find_matching_bracket(&self) -> Option<BracketMatch> {
        if !self.config.bracket_matching {
            return None;
        }

        let line = self.lines.get(self.cursor.0)?;
        let chars: Vec<char> = line.chars().collect();
        let col = self.cursor.1;

        if col >= chars.len() {
            return None;
        }

        let ch = chars[col];
        let (open, close, forward) = match ch {
            '(' => ('(', ')', true),
            ')' => ('(', ')', false),
            '[' => ('[', ']', true),
            ']' => ('[', ']', false),
            '{' => ('{', '}', true),
            '}' => ('{', '}', false),
            _ => return None,
        };

        if forward {
            self.find_matching_bracket_forward(open, close, self.cursor.0, col)
        } else {
            self.find_matching_bracket_backward(open, close, self.cursor.0, col)
        }
    }

    /// Find matching bracket forward
    pub(super) fn find_matching_bracket_forward(
        &self,
        open: char,
        close: char,
        start_line: usize,
        start_col: usize,
    ) -> Option<BracketMatch> {
        let mut depth = 1;
        let mut line_idx = start_line;
        let mut col_idx = start_col + 1;

        while line_idx < self.lines.len() {
            let line = &self.lines[line_idx];
            let chars: Vec<char> = line.chars().collect();

            while col_idx < chars.len() {
                let ch = chars[col_idx];
                if ch == open {
                    depth += 1;
                } else if ch == close {
                    depth -= 1;
                    if depth == 0 {
                        return Some(BracketMatch {
                            position: (line_idx, col_idx),
                            char: ch,
                        });
                    }
                }
                col_idx += 1;
            }

            line_idx += 1;
            col_idx = 0;
        }

        None
    }

    /// Find matching bracket backward
    pub(super) fn find_matching_bracket_backward(
        &self,
        open: char,
        close: char,
        start_line: usize,
        start_col: usize,
    ) -> Option<BracketMatch> {
        let mut depth = 1;
        let mut line_idx = start_line;
        let mut col_idx = start_col;

        loop {
            let line = &self.lines[line_idx];
            let chars: Vec<char> = line.chars().collect();

            while col_idx > 0 {
                col_idx -= 1;
                let ch = chars[col_idx];
                if ch == close {
                    depth += 1;
                } else if ch == open {
                    depth -= 1;
                    if depth == 0 {
                        return Some(BracketMatch {
                            position: (line_idx, col_idx),
                            char: ch,
                        });
                    }
                }
            }

            if line_idx == 0 {
                break;
            }
            line_idx -= 1;
            col_idx = self.lines[line_idx].len();
        }

        None
    }
}
