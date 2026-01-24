//! Query and inspection methods for UndoHistory

use super::types::UndoHistory;

impl<T> UndoHistory<T> {
    /// Get the number of undoable operations
    #[inline]
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }

    /// Get the number of redoable operations
    #[inline]
    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }

    /// Clear all history
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// Clear only the redo stack
    pub fn clear_redo(&mut self) {
        self.redo_stack.clear();
    }

    /// Peek at the last undoable operation without removing it
    pub fn peek_undo(&self) -> Option<&T> {
        self.undo_stack.back()
    }

    /// Peek at the last redoable operation without removing it
    pub fn peek_redo(&self) -> Option<&T> {
        self.redo_stack.last()
    }

    /// Get the maximum history size
    #[inline]
    pub fn max_size(&self) -> usize {
        self.max_size
    }

    /// Set the maximum history size
    ///
    /// If the new size is smaller than current undo stack,
    /// older operations will be removed.
    pub fn set_max_size(&mut self, max_size: usize) {
        self.max_size = max_size;
        while self.undo_stack.len() > max_size {
            self.undo_stack.pop_front();
        }
    }
}

impl<T: Clone> UndoHistory<T> {
    /// Undo without moving to redo stack
    ///
    /// Useful when you want to peek and decide whether to actually undo.
    pub fn undo_peek(&mut self) -> Option<T> {
        self.undo_stack.back().cloned()
    }

    /// Actually commit the undo (call after undo_peek)
    pub fn undo_commit(&mut self) {
        if let Some(op) = self.undo_stack.pop_back() {
            self.redo_stack.push(op);
        }
    }
}
