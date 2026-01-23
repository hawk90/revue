//! Text editing methods for TextArea

use super::edit::EditOperation;

impl TextArea {
    /// Insert a character at cursor
    pub fn insert_char(&mut self, ch: char) {
        if self.read_only {
            return;
        }

        // Delete selection first if any
        if self.has_selection() {
            self.delete_selection();
        }

        if ch == '\n' {
            self.insert_newline();
            return;
        }

        if ch == '\t' {
            // Insert spaces for tab
            let spaces = " ".repeat(self.tab_width);
            self.insert_str(&spaces);
            return;
        }

        let cursor_pos = self.cursors.primary().pos;
        if let Some(line) = self.lines.get_mut(cursor_pos.line) {
            let col = cursor_pos.col.min(line.len());
            line.insert(col, ch);
            self.push_undo(EditOperation::Insert {
                line: cursor_pos.line,
                col,
                text: ch.to_string(),
            });
            self.set_primary_cursor(cursor_pos.line, col + 1);
        }
    }

    /// Insert a string at cursor
    pub fn insert_str(&mut self, s: &str) {
        if self.read_only {
            return;
        }

        if self.has_selection() {
            self.delete_selection();
        }

        // Handle multi-line inserts
        let parts: Vec<&str> = s.split('\n').collect();
        if parts.len() == 1 {
            // Single line insert
            let cursor_pos = self.cursors.primary().pos;
            if let Some(line) = self.lines.get_mut(cursor_pos.line) {
                let col = cursor_pos.col.min(line.len());
                line.insert_str(col, s);
                self.push_undo(EditOperation::Insert {
                    line: cursor_pos.line,
                    col,
                    text: s.to_string(),
                });
                self.set_primary_cursor(cursor_pos.line, col + s.len());
            }
        } else {
            // Multi-line insert
            for (i, part) in parts.iter().enumerate() {
                let cursor_pos = self.cursors.primary().pos;
                if i == 0 {
                    if let Some(line) = self.lines.get_mut(cursor_pos.line) {
                        line.insert_str(cursor_pos.col, part);
                    }
                    self.set_primary_cursor(cursor_pos.line, cursor_pos.col + part.len());
                } else {
                    self.insert_newline();
                    let cursor_pos = self.cursors.primary().pos;
                    if let Some(line) = self.lines.get_mut(cursor_pos.line) {
                        line.insert_str(0, part);
                    }
                    self.set_primary_cursor(cursor_pos.line, part.len());
                }
            }
        }
    }

    /// Insert a newline at cursor
    fn insert_newline(&mut self) {
        if self.read_only {
            return;
        }

        if self.max_lines > 0 && self.lines.len() >= self.max_lines {
            return;
        }

        let cursor_pos = self.cursors.primary().pos;
        let (line, col) = (cursor_pos.line, cursor_pos.col);
        if let Some(current) = self.lines.get_mut(line) {
            let rest: String = current.drain(col.min(current.len())..).collect();
            self.lines.insert(line + 1, rest);
            self.push_undo(EditOperation::SplitLine { line, col });
            self.set_primary_cursor(line + 1, 0);
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

        let cursor_pos = self.cursors.primary().pos;
        let (line, col) = (cursor_pos.line, cursor_pos.col);
        if col > 0 {
            // Delete character in current line
            if let Some(l) = self.lines.get_mut(line) {
                if col <= l.len() {
                    let deleted = l.remove(col - 1);
                    self.push_undo(EditOperation::Delete {
                        line,
                        col: col - 1,
                        text: deleted.to_string(),
                    });
                    self.set_primary_cursor(line, col - 1);
                }
            }
        } else if line > 0 {
            // Merge with previous line
            let current = self.lines.remove(line);
            if let Some(prev_line) = self.lines.get_mut(line - 1) {
                let prev_len = prev_line.len();
                prev_line.push_str(&current);
                self.push_undo(EditOperation::MergeLines {
                    line: line - 1,
                    col: prev_len,
                });
                self.set_primary_cursor(line - 1, prev_len);
            }
        }
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

        let cursor_pos = self.cursors.primary().pos;
        let (line, col) = (cursor_pos.line, cursor_pos.col);

        // Check if we can delete within the current line
        let can_delete_in_line = self.lines.get(line).map(|l| col < l.len()).unwrap_or(false);

        if can_delete_in_line {
            if let Some(l) = self.lines.get_mut(line) {
                let deleted = l.remove(col);
                self.push_undo(EditOperation::Delete {
                    line,
                    col,
                    text: deleted.to_string(),
                });
            }
        } else if line + 1 < self.lines.len() {
            // Merge with next line
            let next = self.lines.remove(line + 1);
            if let Some(current_line) = self.lines.get_mut(line) {
                current_line.push_str(&next);
                self.push_undo(EditOperation::MergeLines { line, col });
            }
        }
    }

    /// Delete the current line
    pub fn delete_line(&mut self) {
        if self.read_only || self.lines.len() <= 1 {
            return;
        }

        let cursor_pos = self.cursors.primary().pos;
        let line = cursor_pos.line;
        let content = self.lines.remove(line);
        self.push_undo(EditOperation::DeleteLine { line, content });

        let new_line = line.min(self.lines.len().saturating_sub(1));
        self.set_primary_cursor(new_line, 0);
    }

    /// Duplicate the current line
    pub fn duplicate_line(&mut self) {
        if self.read_only {
            return;
        }

        let cursor_pos = self.cursors.primary().pos;
        let line = cursor_pos.line;
        if let Some(content) = self.lines.get(line).cloned() {
            self.lines.insert(line + 1, content.clone());
            self.push_undo(EditOperation::InsertLine {
                line: line + 1,
                content,
            });
            self.set_primary_cursor(line + 1, cursor_pos.col);
        }
    }
}

use crate::widget::textarea::TextArea;
