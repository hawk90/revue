//! Code editor bracket matching
//!
//! Public API tests extracted to tests/widget/code_editor/bracket.rs

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
