//! Core UndoHistory implementation

use super::types::{UndoHistory, DEFAULT_MAX_HISTORY};

impl<T> UndoHistory<T> {
    /// Create a new history with default max size
    pub fn new() -> Self {
        Self::with_max_size(DEFAULT_MAX_HISTORY)
    }

    /// Create a new history with custom max size
    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            undo_stack: std::collections::VecDeque::with_capacity(max_size.min(100)),
            redo_stack: Vec::new(),
            max_size,
        }
    }

    /// Push an operation to the undo stack
    ///
    /// This clears the redo stack, as any new action after an undo
    /// invalidates the redo history.
    pub fn push(&mut self, op: T) {
        // Clear redo stack on new action
        self.redo_stack.clear();

        // Add to undo stack
        self.undo_stack.push_back(op);

        // Enforce max size
        while self.undo_stack.len() > self.max_size {
            self.undo_stack.pop_front();
        }
    }

    /// Check if undo is available
    #[inline]
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available
    #[inline]
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }
}
