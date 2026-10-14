//! Code editor editing operations

use super::types::{EditOp, IndentStyle};

/// Byte offset of char index `col` in `line` (its end when `col` is past
/// it). Cursor columns are char indices; `String` edits take bytes.
pub(super) fn byte_at(line: &str, col: usize) -> usize {
    crate::utils::text::char_to_byte_index(line, col)
}

impl super::CodeEditor {
    // =========================================================================
    // Editing
    // =========================================================================

    /// Insert character
    pub fn insert_char(&mut self, ch: char) {
        if self.read_only {
            return;
        }

        if self.has_selection() {
            self.delete_selection();
        }

        if ch == '\n' {
            self.insert_newline();
            return;
        }

        if ch == '\t' {
            self.insert_indent();
            return;
        }

        if let Some(line) = self.lines.get_mut(self.cursor.0) {
            let col = self.cursor.1.min(line.chars().count());
            line.insert(byte_at(line, col), ch);
            self.push_undo(EditOp::Insert {
                line: self.cursor.0,
                col,
                text: ch.to_string(),
            });
            self.cursor.1 = col + 1;
        }

        // Auto-close brackets
        if self.config.bracket_matching {
            let close = match ch {
                '(' => Some(')'),
                '[' => Some(']'),
                '{' => Some('}'),
                '"' => Some('"'),
                '\'' => Some('\''),
                _ => None,
            };
            if let Some(close_ch) = close {
                if let Some(line) = self.lines.get_mut(self.cursor.0) {
                    let at = byte_at(line, self.cursor.1);
                    line.insert(at, close_ch);
                }
            }
        }
    }

    /// Insert string
    pub fn insert_str(&mut self, s: &str) {
        if self.read_only {
            return;
        }

        if self.has_selection() {
            self.delete_selection();
        }

        for ch in s.chars() {
            if ch == '\n' {
                self.insert_newline();
            } else if let Some(line) = self.lines.get_mut(self.cursor.0) {
                let col = self.cursor.1.min(line.chars().count());
                line.insert(byte_at(line, col), ch);
                self.cursor.1 = col + 1;
            }
        }
    }

    /// Insert newline with auto-indent
    pub(super) fn insert_newline(&mut self) {
        if self.read_only {
            return;
        }

        let (line_idx, col) = self.cursor;

        // Get current line's indentation
        let indent = if self.config.auto_indent {
            let current_line = &self.lines[line_idx];
            let leading_ws: String = current_line
                .chars()
                .take_while(|c| c.is_whitespace())
                .collect();

            // Check if we should add extra indent (after opening bracket)
            let trimmed = current_line.trim_end();
            let extra_indent = trimmed
                .chars()
                .last()
                .is_some_and(|c| matches!(c, '{' | '[' | '(' | ':'));

            let base = leading_ws;
            if extra_indent {
                let indent_str = self.indent_unit();
                format!("{}{}", base, indent_str)
            } else {
                base
            }
        } else {
            String::new()
        };

        // Split line
        if let Some(current) = self.lines.get_mut(line_idx) {
            let rest: String = current.drain(byte_at(current, col)..).collect();
            let new_line = format!("{}{}", indent, rest);
            self.lines.insert(line_idx + 1, new_line);
            self.push_undo(EditOp::SplitLine {
                line: line_idx,
                col,
            });
            self.cursor = (line_idx + 1, indent.chars().count());
        }

        self.ensure_cursor_visible();
    }

    /// Insert indent
    pub(super) fn insert_indent(&mut self) {
        if self.read_only {
            return;
        }

        let indent = self.indent_unit();

        if let Some(line) = self.lines.get_mut(self.cursor.0) {
            let col = self.cursor.1.min(line.chars().count());
            line.insert_str(byte_at(line, col), &indent);
            self.push_undo(EditOp::Insert {
                line: self.cursor.0,
                col,
                text: indent.clone(),
            });
            self.cursor.1 = col + indent.chars().count();
        }
    }

    /// Delete character before cursor (backspace)
    pub fn delete_char_before(&mut self) {
        if self.read_only {
            return;
        }

        if self.has_selection() {
            self.delete_selection();
            return;
        }

        let (line_idx, col) = self.cursor;
        if col > 0 {
            if let Some(line) = self.lines.get_mut(line_idx) {
                if col <= line.chars().count() {
                    let deleted = line.remove(byte_at(line, col - 1));
                    self.push_undo(EditOp::Delete {
                        line: line_idx,
                        col: col - 1,
                        text: deleted.to_string(),
                    });
                    self.cursor.1 = col - 1;
                }
            }
        } else if line_idx > 0 {
            // Merge with previous line
            let current = self.lines.remove(line_idx);
            let prev_len = self.lines[line_idx - 1].chars().count();
            self.lines[line_idx - 1].push_str(&current);
            self.push_undo(EditOp::MergeLine {
                line: line_idx - 1,
                col: prev_len,
            });
            self.cursor = (line_idx - 1, prev_len);
        }

        self.ensure_cursor_visible();
    }

    /// Delete character at cursor (delete key)
    pub fn delete_char_at(&mut self) {
        if self.read_only {
            return;
        }

        if self.has_selection() {
            self.delete_selection();
            return;
        }

        let (line_idx, col) = self.cursor;
        if let Some(line) = self.lines.get_mut(line_idx) {
            if col < line.chars().count() {
                let deleted = line.remove(byte_at(line, col));
                self.push_undo(EditOp::Delete {
                    line: line_idx,
                    col,
                    text: deleted.to_string(),
                });
            } else if line_idx + 1 < self.lines.len() {
                // Merge with next line
                let next = self.lines.remove(line_idx + 1);
                self.lines[line_idx].push_str(&next);
                self.push_undo(EditOp::MergeLine {
                    line: line_idx,
                    col,
                });
            }
        }
    }

    /// Delete current line
    pub fn delete_line(&mut self) {
        if self.read_only || self.lines.len() <= 1 {
            return;
        }

        let line_idx = self.cursor.0;
        self.lines.remove(line_idx);
        self.cursor.0 = line_idx.min(self.lines.len().saturating_sub(1));
        self.cursor.1 = 0;
        self.ensure_cursor_visible();
    }

    /// Duplicate current line
    pub fn duplicate_line(&mut self) {
        if self.read_only {
            return;
        }

        let line_idx = self.cursor.0;
        let content = self.lines[line_idx].clone();
        self.lines.insert(line_idx + 1, content);
        self.cursor.0 = line_idx + 1;
        self.ensure_cursor_visible();
    }

    /// One level of indentation: `indent_size` spaces or a tab
    fn indent_unit(&self) -> String {
        match self.config.indent_style {
            IndentStyle::Spaces => " ".repeat(self.config.indent_size),
            IndentStyle::Tabs => "\t".to_string(),
        }
    }
}
