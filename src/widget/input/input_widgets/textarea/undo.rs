//! Undo/redo methods for TextArea

use super::edit::EditOperation;

impl TextArea {
    /// Undo the last operation
    pub fn undo(&mut self) {
        if let Some(op) = self.undo_stack.pop() {
            match &op {
                EditOperation::Insert { line, col, text } => {
                    if let Some(l) = self.lines.get_mut(*line) {
                        let end = (*col + text.len()).min(l.len());
                        l.drain(*col..end);
                    }
                    self.set_primary_cursor(*line, *col);
                }
                EditOperation::Delete { line, col, text } => {
                    if let Some(l) = self.lines.get_mut(*line) {
                        l.insert_str(*col, text);
                    }
                    self.set_primary_cursor(*line, *col + text.len());
                }
                EditOperation::InsertLine { line, .. } => {
                    if *line < self.lines.len() {
                        self.lines.remove(*line);
                    }
                    self.set_primary_cursor(line.saturating_sub(1), 0);
                }
                EditOperation::DeleteLine { line, content } => {
                    self.lines.insert(*line, content.clone());
                    self.set_primary_cursor(*line, 0);
                }
                EditOperation::SplitLine { line, col } => {
                    // Merge lines back
                    if *line + 1 < self.lines.len() {
                        let next = self.lines.remove(*line + 1);
                        if let Some(l) = self.lines.get_mut(*line) {
                            l.push_str(&next);
                        }
                    }
                    self.set_primary_cursor(*line, *col);
                }
                EditOperation::MergeLines { line, col } => {
                    // Split line again
                    if let Some(l) = self.lines.get_mut(*line) {
                        let rest: String = l.drain(*col..).collect();
                        self.lines.insert(*line + 1, rest);
                    }
                    self.set_primary_cursor(*line + 1, 0);
                }
            }
            self.redo_stack.push(op);
        }
    }

    /// Redo the last undone operation
    pub fn redo(&mut self) {
        if let Some(op) = self.redo_stack.pop() {
            match &op {
                EditOperation::Insert { line, col, text } => {
                    if let Some(l) = self.lines.get_mut(*line) {
                        l.insert_str(*col, text);
                    }
                    self.set_primary_cursor(*line, *col + text.len());
                }
                EditOperation::Delete { line, col, text } => {
                    if let Some(l) = self.lines.get_mut(*line) {
                        let end = (*col + text.len()).min(l.len());
                        l.drain(*col..end);
                    }
                    self.set_primary_cursor(*line, *col);
                }
                EditOperation::InsertLine { line, content } => {
                    self.lines.insert(*line, content.clone());
                    self.set_primary_cursor(*line, 0);
                }
                EditOperation::DeleteLine { line, .. } => {
                    if *line < self.lines.len() {
                        self.lines.remove(*line);
                    }
                    let new_line = *line.min(&self.lines.len().saturating_sub(1));
                    self.set_primary_cursor(new_line, 0);
                }
                EditOperation::SplitLine { line, col } => {
                    if let Some(l) = self.lines.get_mut(*line) {
                        let rest: String = l.drain(*col..).collect();
                        self.lines.insert(*line + 1, rest);
                    }
                    self.set_primary_cursor(*line + 1, 0);
                }
                EditOperation::MergeLines { line, col } => {
                    if *line + 1 < self.lines.len() {
                        let next = self.lines.remove(*line + 1);
                        if let Some(l) = self.lines.get_mut(*line) {
                            l.push_str(&next);
                        }
                    }
                    self.set_primary_cursor(*line, *col);
                }
            }
            self.push_undo_internal(op);
        }
    }

    /// Push an operation to the undo stack without clearing redo (internal use)
    fn push_undo_internal(&mut self, op: EditOperation) {
        self.undo_stack.push(op);
        if self.undo_stack.len() > super::MAX_UNDO_HISTORY {
            self.undo_stack.remove(0);
        }
    }
}

use super::TextArea;
