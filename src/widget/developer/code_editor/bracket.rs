//! Code editor bracket matching

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

        let (line_idx, col) = self.cursor_position();
        let chars: Vec<char> = self.edit.line(line_idx)?.chars().collect();

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
            self.find_matching_bracket_forward(open, close, line_idx, col)
        } else {
            self.find_matching_bracket_backward(open, close, line_idx, col)
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

        let lines = self.edit.lines();
        while line_idx < lines.len() {
            let line = &lines[line_idx];
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

        let lines = self.edit.lines();
        loop {
            let line = &lines[line_idx];
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
            col_idx = lines[line_idx].chars().count();
        }

        None
    }
}
