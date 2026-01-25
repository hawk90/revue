//! Undo/redo functionality for the Input widget

use super::types::{EditOperation, Input};

impl Input {
    // ─────────────────────────────────────────────────────────────────────────
    // Undo/Redo
    // ─────────────────────────────────────────────────────────────────────────

    /// Push an operation to the undo stack
    pub(super) fn push_undo(&mut self, op: EditOperation) {
        self.undo_stack.push(op);
        if self.undo_stack.len() >= super::types::MAX_UNDO_HISTORY {
            self.undo_stack.remove(0);
        }
        self.redo_stack.clear();
    }

    /// Undo the last operation
    pub fn undo(&mut self) -> bool {
        if let Some(op) = self.undo_stack.pop() {
            match op {
                EditOperation::Insert { pos, ref text } => {
                    // Undo insert by deleting the inserted text
                    let end = pos + text.chars().count();
                    self.remove_char_range(pos, end);
                    self.cursor = pos;
                    self.redo_stack.push(op);
                }
                EditOperation::Delete { pos, ref text } => {
                    // Undo delete by inserting the deleted text
                    self.insert_at_char(pos, text);
                    self.cursor = pos + text.chars().count();
                    self.redo_stack.push(op);
                }
                EditOperation::Replace {
                    ref old_value,
                    old_cursor,
                    ref new_value,
                    new_cursor,
                } => {
                    // Undo replace by restoring old value
                    self.value = old_value.clone();
                    self.cursor = old_cursor;
                    self.redo_stack.push(EditOperation::Replace {
                        old_value: new_value.clone(),
                        old_cursor: new_cursor,
                        new_value: old_value.clone(),
                        new_cursor: old_cursor,
                    });
                }
            }
            self.clear_selection();
            true
        } else {
            false
        }
    }

    /// Redo the last undone operation
    pub fn redo(&mut self) -> bool {
        if let Some(op) = self.redo_stack.pop() {
            match op {
                EditOperation::Insert { pos, ref text } => {
                    // Redo insert
                    self.insert_at_char(pos, text);
                    self.cursor = pos + text.chars().count();
                    self.undo_stack.push(op);
                }
                EditOperation::Delete { pos, ref text } => {
                    // Redo delete
                    let end = pos + text.chars().count();
                    self.remove_char_range(pos, end);
                    self.cursor = pos;
                    self.undo_stack.push(op);
                }
                EditOperation::Replace {
                    ref old_value,
                    old_cursor,
                    ref new_value,
                    new_cursor,
                } => {
                    // Redo replace
                    self.value = new_value.clone();
                    self.cursor = new_cursor;
                    self.undo_stack.push(EditOperation::Replace {
                        old_value: old_value.clone(),
                        old_cursor,
                        new_value: new_value.clone(),
                        new_cursor,
                    });
                }
            }
            self.clear_selection();
            true
        } else {
            false
        }
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Clear undo/redo history
    pub fn clear_history(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}
