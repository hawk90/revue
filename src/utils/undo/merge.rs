//! Merge operations for UndoHistory

use super::types::{Mergeable, UndoHistory};

impl<T: Mergeable + Clone> UndoHistory<T> {
    /// Push an operation, attempting to merge with the previous one
    ///
    /// If the new operation can be merged with the last undo operation,
    /// they are combined into a single operation. This is useful for
    /// coalescing multiple small changes (like individual character inserts)
    /// into larger operations.
    pub fn push_merge(&mut self, op: T) {
        // Clear redo stack on new action
        self.redo_stack.clear();

        // Try to merge with last operation
        if let Some(last) = self.undo_stack.pop_back() {
            if last.can_merge(&op) {
                let merged = last.merge(op);
                self.undo_stack.push_back(merged);
                return;
            }
            // Can't merge, push both back
            self.undo_stack.push_back(last);
        }

        // Add new operation
        self.undo_stack.push_back(op);

        // Enforce max size
        while self.undo_stack.len() > self.max_size {
            self.undo_stack.pop_front();
        }
    }
}
