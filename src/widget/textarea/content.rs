//! Content access methods for TextArea

use super::cursor::{CursorPos, CursorSet};
use super::edit::EditOperation;
use super::selection::Selection;
use crate::widget::syntax::{Language, SyntaxHighlighter};

impl TextArea {
    /// Get the current highlighting language
    pub fn get_syntax_language(&self) -> Language {
        self.highlighter
            .as_ref()
            .map(|h| h.get_language())
            .unwrap_or(Language::None)
    }

    /// Set the syntax highlighting language
    pub fn set_language(&mut self, language: Language) {
        if language == Language::None {
            self.highlighter = None;
        } else {
            self.highlighter = Some(SyntaxHighlighter::new(language));
        }
    }

    /// Get the current text content
    pub fn get_content(&self) -> String {
        self.lines.join("\n")
    }

    /// Set the text content
    pub fn set_content(&mut self, text: &str) {
        self.lines = text.lines().map(String::from).collect();
        if self.lines.is_empty() {
            self.lines.push(String::new());
        }
        self.cursors = CursorSet::default();
        self.scroll = (0, 0);
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// Get the number of lines
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    /// Get the cursor position (primary cursor for backward compatibility)
    pub fn cursor_position(&self) -> (usize, usize) {
        let pos = self.cursors.primary().pos;
        (pos.line, pos.col)
    }

    /// Get all cursor positions
    pub fn cursor_positions(&self) -> Vec<(usize, usize)> {
        self.cursors
            .iter()
            .map(|c| (c.pos.line, c.pos.col))
            .collect()
    }

    /// Get the number of cursors
    pub fn cursor_count(&self) -> usize {
        self.cursors.len()
    }

    /// Set the cursor position (primary cursor, clears secondary cursors)
    pub fn set_cursor(&mut self, line: usize, col: usize) {
        let line = line.min(self.lines.len().saturating_sub(1));
        let col = col.min(self.line_len(line));
        self.cursors = CursorSet::new(CursorPos::new(line, col));
    }

    /// Get length of a specific line
    pub(super) fn line_len(&self, line: usize) -> usize {
        self.lines.get(line).map(|l| l.len()).unwrap_or(0)
    }

    /// Get selected text (from primary cursor)
    pub fn get_selection(&self) -> Option<String> {
        let sel = self.cursors.primary().selection()?.normalized();
        self.get_text_in_selection(&sel)
    }

    /// Get text within a selection range
    fn get_text_in_selection(&self, sel: &Selection) -> Option<String> {
        let mut result = String::new();

        for line_idx in sel.start.0..=sel.end.0 {
            if line_idx >= self.lines.len() {
                break;
            }
            let line = &self.lines[line_idx];
            let start_col = if line_idx == sel.start.0 {
                sel.start.1
            } else {
                0
            };
            let end_col = if line_idx == sel.end.0 {
                sel.end.1
            } else {
                line.len()
            };

            if start_col < line.len() {
                result.push_str(&line[start_col..end_col.min(line.len())]);
            }
            if line_idx < sel.end.0 {
                result.push('\n');
            }
        }

        Some(result)
    }

    /// Delete selected text (from primary cursor)
    pub fn delete_selection(&mut self) {
        let sel = match self.cursors.primary().selection() {
            Some(s) => s.normalized(),
            None => return,
        };

        if sel.start.0 == sel.end.0 {
            // Single line selection
            if let Some(line) = self.lines.get_mut(sel.start.0) {
                let deleted: String = line.drain(sel.start.1..sel.end.1.min(line.len())).collect();
                self.push_undo(EditOperation::Delete {
                    line: sel.start.0,
                    col: sel.start.1,
                    text: deleted,
                });
            }
        } else {
            // Multi-line selection
            // Get the content before and after selection
            let before: String = self
                .lines
                .get(sel.start.0)
                .map(|l| l.chars().take(sel.start.1).collect())
                .unwrap_or_default();
            let after: String = self
                .lines
                .get(sel.end.0)
                .map(|l| l.chars().skip(sel.end.1).collect())
                .unwrap_or_default();

            // Remove lines between start and end
            for _ in sel.start.0..=sel.end.0 {
                if sel.start.0 < self.lines.len() {
                    self.lines.remove(sel.start.0);
                }
            }

            // Insert merged line
            self.lines
                .insert(sel.start.0, format!("{}{}", before, after));
        }

        // Update cursor to selection start
        self.cursors = CursorSet::new(CursorPos::new(sel.start.0, sel.start.1));
    }

    /// Check if primary cursor has a selection
    pub fn has_selection(&self) -> bool {
        self.cursors.primary().is_selecting()
    }

    /// Start selection at current cursor (primary)
    pub fn start_selection(&mut self) {
        self.cursors.primary_mut().start_selection();
    }

    /// Update selection to current cursor position
    pub(super) fn update_selection(&mut self) {
        // Selection is automatically updated because anchor stays fixed
        // while cursor position moves. No explicit update needed.
    }

    /// Clear selection (all cursors)
    pub fn clear_selection(&mut self) {
        for cursor in self.cursors.iter_mut() {
            cursor.clear_selection();
        }
    }

    /// Push an operation to the undo stack
    pub(super) fn push_undo(&mut self, op: EditOperation) {
        self.undo_stack.push(op);
        if self.undo_stack.len() > super::MAX_UNDO_HISTORY {
            self.undo_stack.remove(0);
        }
        self.redo_stack.clear();
    }

    /// Set primary cursor position (internal helper)
    pub(super) fn set_primary_cursor(&mut self, line: usize, col: usize) {
        self.cursors.set_primary(CursorPos::new(line, col));
    }
}

// These methods need access to TextArea fields
use crate::widget::textarea::TextArea;
