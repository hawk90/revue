//! Generic undo/redo stack pattern
//!
//! Provides a reusable undo/redo history stack for any `Clone` type.
//!
//! # Example
//!
//! ```ignore
//! use revue::patterns::UndoStack;
//!
//! let mut stack = UndoStack::new(100);
//! stack.push("initial".to_string());
//! stack.push("modified".to_string());
//!
//! assert_eq!(stack.undo(), Some(&"initial".to_string()));
//! assert_eq!(stack.redo(), Some(&"modified".to_string()));
//! ```

/// Generic undo/redo stack
///
/// Maintains a history of states with configurable maximum depth.
/// Pushing a new state clears the redo stack, following standard
/// undo/redo semantics.
///
/// # Example
///
/// ```rust,ignore
/// use revue::patterns::UndoStack;
///
/// let mut stack = UndoStack::new(100);
/// stack.push("initial".to_string());
/// stack.push("modified".to_string());
///
/// assert_eq!(stack.undo(), Some(&"initial".to_string()));
/// assert_eq!(stack.redo(), Some(&"modified".to_string()));
/// ```
pub struct UndoStack<T: Clone> {
    undo_stack: Vec<T>,
    redo_stack: Vec<T>,
    max_history: usize,
}

impl<T: Clone> UndoStack<T> {
    /// Create a new undo stack with the given maximum history depth
    pub fn new(max_history: usize) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_history,
        }
    }

    /// Create with default capacity of 100
    pub fn with_default_capacity() -> Self {
        Self::new(100)
    }

    /// Push a new state, clearing the redo stack
    ///
    /// If the undo stack exceeds `max_history`, the oldest entry is removed.
    pub fn push(&mut self, state: T) {
        self.undo_stack.push(state);
        self.redo_stack.clear();

        // Trim to max_history
        if self.undo_stack.len() > self.max_history {
            self.undo_stack.remove(0);
        }
    }

    /// Undo: pop from undo stack, push current to redo, return previous state
    ///
    /// Returns `None` if there is nothing to undo (need at least 2 entries:
    /// the current state and a previous state to revert to).
    pub fn undo(&mut self) -> Option<&T> {
        if self.undo_stack.len() <= 1 {
            return None;
        }

        let current = self.undo_stack.pop().unwrap();
        self.redo_stack.push(current);
        self.undo_stack.last()
    }

    /// Redo: pop from redo stack, push to undo, return restored state
    ///
    /// Returns `None` if there is nothing to redo.
    pub fn redo(&mut self) -> Option<&T> {
        if self.redo_stack.is_empty() {
            return None;
        }

        let state = self.redo_stack.pop().unwrap();
        self.undo_stack.push(state);
        self.undo_stack.last()
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        self.undo_stack.len() > 1
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Clear all history
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// Get current state (top of undo stack)
    pub fn current(&self) -> Option<&T> {
        self.undo_stack.last()
    }

    /// Get undo depth (number of states that can be undone)
    pub fn undo_depth(&self) -> usize {
        self.undo_stack.len().saturating_sub(1)
    }

    /// Get redo depth (number of states that can be redone)
    pub fn redo_depth(&self) -> usize {
        self.redo_stack.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let stack: UndoStack<String> = UndoStack::new(50);
        assert!(!stack.can_undo());
        assert!(!stack.can_redo());
        assert_eq!(stack.current(), None);
        assert_eq!(stack.undo_depth(), 0);
        assert_eq!(stack.redo_depth(), 0);
    }

    #[test]
    fn test_with_default_capacity() {
        let stack: UndoStack<i32> = UndoStack::with_default_capacity();
        assert_eq!(stack.max_history, 100);
    }

    #[test]
    fn test_push_and_current() {
        let mut stack = UndoStack::new(10);
        stack.push("first".to_string());
        assert_eq!(stack.current(), Some(&"first".to_string()));

        stack.push("second".to_string());
        assert_eq!(stack.current(), Some(&"second".to_string()));
    }

    #[test]
    fn test_undo() {
        let mut stack = UndoStack::new(10);
        stack.push("initial".to_string());
        stack.push("modified".to_string());

        assert!(stack.can_undo());
        let result = stack.undo();
        assert_eq!(result, Some(&"initial".to_string()));
        assert_eq!(stack.current(), Some(&"initial".to_string()));
    }

    #[test]
    fn test_redo() {
        let mut stack = UndoStack::new(10);
        stack.push("initial".to_string());
        stack.push("modified".to_string());

        stack.undo();
        assert!(stack.can_redo());

        let result = stack.redo();
        assert_eq!(result, Some(&"modified".to_string()));
        assert_eq!(stack.current(), Some(&"modified".to_string()));
    }

    #[test]
    fn test_undo_clears_on_push() {
        let mut stack = UndoStack::new(10);
        stack.push("a".to_string());
        stack.push("b".to_string());
        stack.push("c".to_string());

        stack.undo(); // back to "b"
        assert!(stack.can_redo());

        // Pushing new state clears redo
        stack.push("d".to_string());
        assert!(!stack.can_redo());
        assert_eq!(stack.current(), Some(&"d".to_string()));
    }

    #[test]
    fn test_undo_nothing_to_undo() {
        let mut stack: UndoStack<String> = UndoStack::new(10);
        assert_eq!(stack.undo(), None);

        stack.push("only".to_string());
        // Only one state, nothing to undo to
        assert_eq!(stack.undo(), None);
    }

    #[test]
    fn test_redo_nothing_to_redo() {
        let mut stack: UndoStack<String> = UndoStack::new(10);
        assert_eq!(stack.redo(), None);

        stack.push("a".to_string());
        assert_eq!(stack.redo(), None);
    }

    #[test]
    fn test_max_history_trimming() {
        let mut stack = UndoStack::new(3);
        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.push(4); // Should trim oldest (1)

        assert_eq!(stack.undo_depth(), 2); // Can undo to 2 (3 items, depth = 2)
        assert_eq!(stack.current(), Some(&4));

        stack.undo();
        assert_eq!(stack.current(), Some(&3));

        stack.undo();
        assert_eq!(stack.current(), Some(&2));

        // No more undo (1 was trimmed)
        assert!(!stack.can_undo());
    }

    #[test]
    fn test_clear() {
        let mut stack = UndoStack::new(10);
        stack.push("a".to_string());
        stack.push("b".to_string());
        stack.undo();

        stack.clear();
        assert!(!stack.can_undo());
        assert!(!stack.can_redo());
        assert_eq!(stack.current(), None);
        assert_eq!(stack.undo_depth(), 0);
        assert_eq!(stack.redo_depth(), 0);
    }

    #[test]
    fn test_undo_redo_depths() {
        let mut stack = UndoStack::new(10);
        assert_eq!(stack.undo_depth(), 0);
        assert_eq!(stack.redo_depth(), 0);

        stack.push("a".to_string());
        assert_eq!(stack.undo_depth(), 0);
        assert_eq!(stack.redo_depth(), 0);

        stack.push("b".to_string());
        assert_eq!(stack.undo_depth(), 1);
        assert_eq!(stack.redo_depth(), 0);

        stack.push("c".to_string());
        assert_eq!(stack.undo_depth(), 2);
        assert_eq!(stack.redo_depth(), 0);

        stack.undo();
        assert_eq!(stack.undo_depth(), 1);
        assert_eq!(stack.redo_depth(), 1);

        stack.undo();
        assert_eq!(stack.undo_depth(), 0);
        assert_eq!(stack.redo_depth(), 2);
    }

    #[test]
    fn test_multiple_undo_redo_cycles() {
        let mut stack = UndoStack::new(10);
        stack.push(1);
        stack.push(2);
        stack.push(3);

        // Undo all the way
        assert_eq!(stack.undo(), Some(&2));
        assert_eq!(stack.undo(), Some(&1));
        assert_eq!(stack.undo(), None);

        // Redo all the way
        assert_eq!(stack.redo(), Some(&2));
        assert_eq!(stack.redo(), Some(&3));
        assert_eq!(stack.redo(), None);
    }

    #[test]
    fn test_can_undo_and_can_redo() {
        let mut stack = UndoStack::new(10);
        assert!(!stack.can_undo());
        assert!(!stack.can_redo());

        stack.push("a".to_string());
        assert!(!stack.can_undo()); // Only 1 item
        assert!(!stack.can_redo());

        stack.push("b".to_string());
        assert!(stack.can_undo());
        assert!(!stack.can_redo());

        stack.undo();
        assert!(!stack.can_undo()); // Back to 1 item
        assert!(stack.can_redo());

        stack.redo();
        assert!(stack.can_undo());
        assert!(!stack.can_redo());
    }

    #[test]
    fn test_with_complex_type() {
        #[derive(Clone, Debug, PartialEq)]
        struct State {
            name: String,
            value: i32,
        }

        let mut stack = UndoStack::new(10);
        stack.push(State {
            name: "first".into(),
            value: 1,
        });
        stack.push(State {
            name: "second".into(),
            value: 2,
        });

        let prev = stack.undo().unwrap();
        assert_eq!(prev.name, "first");
        assert_eq!(prev.value, 1);
    }
}
