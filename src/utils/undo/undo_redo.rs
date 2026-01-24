//! Undo and redo operations

use super::types::UndoHistory;

impl<T: Clone> UndoHistory<T> {
    /// Pop an operation from the undo stack and push it to redo
    ///
    /// Returns the operation that should be reversed, if any.
    pub fn undo(&mut self) -> Option<T> {
        self.undo_stack
            .pop_back()
            .inspect(|op| self.redo_stack.push(op.clone()))
    }

    /// Pop an operation from the redo stack and push it to undo
    ///
    /// Returns the operation that should be re-applied, if any.
    pub fn redo(&mut self) -> Option<T> {
        self.redo_stack
            .pop()
            .inspect(|op| self.undo_stack.push_back(op.clone()))
    }
}
