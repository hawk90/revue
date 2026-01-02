//! Generic Undo/Redo history management
//!
//! Provides a reusable undo/redo system that can be used with any operation type.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::utils::{UndoHistory, Mergeable};
//!
//! #[derive(Clone)]
//! enum TextOp {
//!     Insert { pos: usize, text: String },
//!     Delete { pos: usize, text: String },
//! }
//!
//! let mut history: UndoHistory<TextOp> = UndoHistory::new();
//! history.push(TextOp::Insert { pos: 0, text: "Hello".into() });
//!
//! // Later, undo:
//! if let Some(op) = history.undo() {
//!     // Apply reverse of op to your state
//! }
//! ```

use std::collections::VecDeque;

/// Default maximum history size
pub const DEFAULT_MAX_HISTORY: usize = 100;

/// Trait for operations that can be merged (coalesced)
///
/// Implement this trait to enable automatic merging of consecutive
/// similar operations (e.g., multiple character insertions into one).
///
/// # Example
///
/// ```rust,ignore
/// impl Mergeable for TextOp {
///     fn can_merge(&self, other: &Self) -> bool {
///         match (self, other) {
///             (TextOp::Insert { pos: p1, .. }, TextOp::Insert { pos: p2, .. }) => {
///                 // Can merge if inserting at consecutive positions
///                 *p2 == *p1 + 1
///             }
///             _ => false,
///         }
///     }
///
///     fn merge(self, other: Self) -> Self {
///         match (self, other) {
///             (TextOp::Insert { pos, text: mut t1 }, TextOp::Insert { text: t2, .. }) => {
///                 t1.push_str(&t2);
///                 TextOp::Insert { pos, text: t1 }
///             }
///             _ => self,
///         }
///     }
/// }
/// ```
pub trait Mergeable {
    /// Check if this operation can be merged with `other`
    ///
    /// `self` is the older operation, `other` is the newer one.
    fn can_merge(&self, other: &Self) -> bool;

    /// Merge two operations into one
    ///
    /// `self` is the older operation, `other` is the newer one.
    /// Returns a single operation that represents both.
    fn merge(self, other: Self) -> Self;
}

/// A generic undo/redo history manager
///
/// Stores operations in two stacks (undo and redo) and provides
/// methods to navigate through the history.
///
/// # Type Parameters
///
/// * `T` - The operation type. Each widget can define its own operation enum.
///
/// # Usage Pattern
///
/// 1. When user performs an action, `push()` the operation
/// 2. When user requests undo, `undo()` returns the operation to reverse
/// 3. When user requests redo, `redo()` returns the operation to re-apply
///
/// The history manager doesn't apply operations - it just tracks them.
/// Your code is responsible for applying/reversing operations on your state.
#[derive(Clone, Debug)]
pub struct UndoHistory<T> {
    /// Stack of operations that can be undone
    undo_stack: VecDeque<T>,
    /// Stack of operations that can be redone
    redo_stack: Vec<T>,
    /// Maximum number of operations to keep
    max_size: usize,
}

impl<T> Default for UndoHistory<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> UndoHistory<T> {
    /// Create a new history with default max size
    pub fn new() -> Self {
        Self::with_max_size(DEFAULT_MAX_HISTORY)
    }

    /// Create a new history with custom max size
    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            undo_stack: VecDeque::with_capacity(max_size.min(100)),
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

impl<T: Clone> UndoHistory<T> {
    /// Pop an operation from the undo stack and push it to redo
    ///
    /// Returns the operation that should be reversed, if any.
    pub fn undo(&mut self) -> Option<T> {
        self.undo_stack.pop_back().map(|op| {
            self.redo_stack.push(op.clone());
            op
        })
    }

    /// Pop an operation from the redo stack and push it to undo
    ///
    /// Returns the operation that should be re-applied, if any.
    pub fn redo(&mut self) -> Option<T> {
        self.redo_stack.pop().map(|op| {
            self.undo_stack.push_back(op.clone());
            op
        })
    }
}

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

/// A checkpoint marker for grouping operations
///
/// Use this to group multiple operations into a single undo unit.
///
/// # Example
///
/// ```rust,ignore
/// // Start a group
/// history.begin_group();
///
/// // Multiple operations...
/// history.push(op1);
/// history.push(op2);
/// history.push(op3);
///
/// // End group - all ops become one undo unit
/// history.end_group();
/// ```
#[derive(Clone, Debug)]
pub struct UndoGroup<T> {
    operations: Vec<T>,
}

impl<T> Default for UndoGroup<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> UndoGroup<T> {
    /// Create a new empty group
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    /// Add an operation to the group
    pub fn push(&mut self, op: T) {
        self.operations.push(op);
    }

    /// Check if the group is empty
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Get the number of operations in the group
    pub fn len(&self) -> usize {
        self.operations.len()
    }

    /// Get the operations (consumes the group)
    pub fn into_operations(self) -> Vec<T> {
        self.operations
    }

    /// Get a reference to the operations
    pub fn operations(&self) -> &[T] {
        &self.operations
    }

    /// Iterate over operations
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.operations.iter()
    }

    /// Iterate over operations in reverse (for undo)
    pub fn iter_rev(&self) -> impl Iterator<Item = &T> {
        self.operations.iter().rev()
    }
}

impl<T: Clone> UndoGroup<T> {
    /// Clone operations in reverse order (for undo)
    pub fn reversed(&self) -> Vec<T> {
        self.operations.iter().rev().cloned().collect()
    }
}

/// A history manager that supports operation groups
///
/// Groups multiple operations into single undo/redo units.
#[derive(Clone, Debug)]
pub struct GroupedUndoHistory<T> {
    /// Inner history storing groups
    history: UndoHistory<UndoGroup<T>>,
    /// Current group being built (if any)
    current_group: Option<UndoGroup<T>>,
}

impl<T> Default for GroupedUndoHistory<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> GroupedUndoHistory<T> {
    /// Create a new grouped history
    pub fn new() -> Self {
        Self {
            history: UndoHistory::new(),
            current_group: None,
        }
    }

    /// Create with custom max size
    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            history: UndoHistory::with_max_size(max_size),
            current_group: None,
        }
    }

    /// Begin a new operation group
    ///
    /// All operations pushed until `end_group()` will be grouped together.
    pub fn begin_group(&mut self) {
        if self.current_group.is_none() {
            self.current_group = Some(UndoGroup::new());
        }
    }

    /// End the current operation group
    ///
    /// The group becomes a single undo/redo unit.
    pub fn end_group(&mut self) {
        if let Some(group) = self.current_group.take() {
            if !group.is_empty() {
                self.history.push(group);
            }
        }
    }

    /// Check if currently building a group
    pub fn in_group(&self) -> bool {
        self.current_group.is_some()
    }

    /// Push an operation
    ///
    /// If in a group, adds to the group. Otherwise, creates a single-op group.
    pub fn push(&mut self, op: T) {
        if let Some(ref mut group) = self.current_group {
            group.push(op);
        } else {
            let mut group = UndoGroup::new();
            group.push(op);
            self.history.push(group);
        }
    }

    /// Check if undo is available
    #[inline]
    pub fn can_undo(&self) -> bool {
        self.history.can_undo()
    }

    /// Check if redo is available
    #[inline]
    pub fn can_redo(&self) -> bool {
        self.history.can_redo()
    }

    /// Clear all history
    pub fn clear(&mut self) {
        self.history.clear();
        self.current_group = None;
    }
}

impl<T: Clone> GroupedUndoHistory<T> {
    /// Undo the last group
    ///
    /// Returns the group of operations that should be reversed, if any.
    /// Operations in the group should typically be applied in reverse order.
    pub fn undo(&mut self) -> Option<UndoGroup<T>> {
        self.history.undo()
    }

    /// Redo the last undone group
    ///
    /// Returns the group of operations that should be re-applied, if any.
    pub fn redo(&mut self) -> Option<UndoGroup<T>> {
        self.history.redo()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Simple test operation
    #[derive(Clone, Debug, PartialEq)]
    enum TestOp {
        Insert { pos: usize, text: String },
        Delete { pos: usize, text: String },
    }

    impl Mergeable for TestOp {
        fn can_merge(&self, other: &Self) -> bool {
            match (self, other) {
                (
                    TestOp::Insert { pos: p1, text: t1 },
                    TestOp::Insert { pos: p2, .. },
                ) => *p2 == *p1 + t1.chars().count(),
                _ => false,
            }
        }

        fn merge(self, other: Self) -> Self {
            match (self, other) {
                (
                    TestOp::Insert { pos, text: mut t1 },
                    TestOp::Insert { text: t2, .. },
                ) => {
                    t1.push_str(&t2);
                    TestOp::Insert { pos, text: t1 }
                }
                // can_merge should have returned false for other cases
                (op, _) => op,
            }
        }
    }

    // =========================================================================
    // Basic UndoHistory Tests
    // =========================================================================

    #[test]
    fn test_new() {
        let history: UndoHistory<TestOp> = UndoHistory::new();
        assert!(!history.can_undo());
        assert!(!history.can_redo());
        assert_eq!(history.undo_count(), 0);
        assert_eq!(history.redo_count(), 0);
    }

    #[test]
    fn test_push() {
        let mut history = UndoHistory::new();
        history.push(TestOp::Insert { pos: 0, text: "a".into() });

        assert!(history.can_undo());
        assert!(!history.can_redo());
        assert_eq!(history.undo_count(), 1);
    }

    #[test]
    fn test_undo() {
        let mut history = UndoHistory::new();
        history.push(TestOp::Insert { pos: 0, text: "a".into() });
        history.push(TestOp::Insert { pos: 1, text: "b".into() });

        let op = history.undo();
        assert_eq!(op, Some(TestOp::Insert { pos: 1, text: "b".into() }));
        assert!(history.can_undo());
        assert!(history.can_redo());

        let op = history.undo();
        assert_eq!(op, Some(TestOp::Insert { pos: 0, text: "a".into() }));
        assert!(!history.can_undo());
        assert!(history.can_redo());
    }

    #[test]
    fn test_redo() {
        let mut history = UndoHistory::new();
        history.push(TestOp::Insert { pos: 0, text: "a".into() });
        history.push(TestOp::Insert { pos: 1, text: "b".into() });

        history.undo();
        history.undo();

        let op = history.redo();
        assert_eq!(op, Some(TestOp::Insert { pos: 0, text: "a".into() }));

        let op = history.redo();
        assert_eq!(op, Some(TestOp::Insert { pos: 1, text: "b".into() }));

        assert!(!history.can_redo());
    }

    #[test]
    fn test_undo_clears_redo_on_push() {
        let mut history = UndoHistory::new();
        history.push(TestOp::Insert { pos: 0, text: "a".into() });
        history.push(TestOp::Insert { pos: 1, text: "b".into() });

        history.undo();
        assert!(history.can_redo());

        // New action should clear redo
        history.push(TestOp::Insert { pos: 1, text: "c".into() });
        assert!(!history.can_redo());
    }

    #[test]
    fn test_max_size() {
        let mut history: UndoHistory<TestOp> = UndoHistory::with_max_size(3);

        for i in 0..5 {
            history.push(TestOp::Insert { pos: i, text: i.to_string() });
        }

        assert_eq!(history.undo_count(), 3);

        // Should have kept the last 3 operations
        let op = history.undo();
        assert_eq!(op, Some(TestOp::Insert { pos: 4, text: "4".into() }));
    }

    #[test]
    fn test_clear() {
        let mut history = UndoHistory::new();
        history.push(TestOp::Insert { pos: 0, text: "a".into() });
        history.undo();

        history.clear();
        assert!(!history.can_undo());
        assert!(!history.can_redo());
    }

    #[test]
    fn test_peek() {
        let mut history = UndoHistory::new();
        history.push(TestOp::Insert { pos: 0, text: "a".into() });
        history.push(TestOp::Insert { pos: 1, text: "b".into() });

        assert_eq!(
            history.peek_undo(),
            Some(&TestOp::Insert { pos: 1, text: "b".into() })
        );

        history.undo();
        assert_eq!(
            history.peek_redo(),
            Some(&TestOp::Insert { pos: 1, text: "b".into() })
        );
    }

    // =========================================================================
    // Merge Tests
    // =========================================================================

    #[test]
    fn test_push_merge() {
        let mut history = UndoHistory::new();

        // Push consecutive inserts that can merge
        history.push_merge(TestOp::Insert { pos: 0, text: "H".into() });
        history.push_merge(TestOp::Insert { pos: 1, text: "i".into() });
        history.push_merge(TestOp::Insert { pos: 2, text: "!".into() });

        // Should be merged into one operation
        assert_eq!(history.undo_count(), 1);

        let op = history.undo();
        assert_eq!(op, Some(TestOp::Insert { pos: 0, text: "Hi!".into() }));
    }

    #[test]
    fn test_push_merge_different_types() {
        let mut history = UndoHistory::new();

        history.push_merge(TestOp::Insert { pos: 0, text: "a".into() });
        history.push_merge(TestOp::Delete { pos: 0, text: "a".into() });

        // Different types can't merge
        assert_eq!(history.undo_count(), 2);
    }

    #[test]
    fn test_push_merge_non_consecutive() {
        let mut history = UndoHistory::new();

        history.push_merge(TestOp::Insert { pos: 0, text: "a".into() });
        history.push_merge(TestOp::Insert { pos: 5, text: "b".into() }); // Not consecutive

        // Non-consecutive inserts can't merge
        assert_eq!(history.undo_count(), 2);
    }

    // =========================================================================
    // Group Tests
    // =========================================================================

    #[test]
    fn test_grouped_history() {
        let mut history = GroupedUndoHistory::new();

        // Single operations
        history.push(TestOp::Insert { pos: 0, text: "a".into() });
        history.push(TestOp::Insert { pos: 1, text: "b".into() });

        assert!(history.can_undo());

        // Each push creates its own group
        let group = history.undo();
        assert!(group.is_some());
        assert_eq!(group.unwrap().len(), 1);
    }

    #[test]
    fn test_grouped_history_groups() {
        let mut history = GroupedUndoHistory::new();

        // Create a group
        history.begin_group();
        history.push(TestOp::Insert { pos: 0, text: "a".into() });
        history.push(TestOp::Insert { pos: 1, text: "b".into() });
        history.push(TestOp::Insert { pos: 2, text: "c".into() });
        history.end_group();

        // Single undo gets all three
        let group = history.undo().unwrap();
        assert_eq!(group.len(), 3);

        // Verify order
        let ops: Vec<_> = group.iter().collect();
        assert_eq!(ops[0], &TestOp::Insert { pos: 0, text: "a".into() });
        assert_eq!(ops[1], &TestOp::Insert { pos: 1, text: "b".into() });
        assert_eq!(ops[2], &TestOp::Insert { pos: 2, text: "c".into() });
    }

    #[test]
    fn test_grouped_history_nested_begin() {
        let mut history = GroupedUndoHistory::new();

        history.begin_group();
        history.push(TestOp::Insert { pos: 0, text: "a".into() });

        // Nested begin_group is ignored
        history.begin_group();
        history.push(TestOp::Insert { pos: 1, text: "b".into() });

        history.end_group();

        // Should be one group with 2 ops
        let group = history.undo().unwrap();
        assert_eq!(group.len(), 2);
    }

    #[test]
    fn test_undo_group_reversed() {
        let group = {
            let mut g = UndoGroup::new();
            g.push(TestOp::Insert { pos: 0, text: "a".into() });
            g.push(TestOp::Insert { pos: 1, text: "b".into() });
            g.push(TestOp::Insert { pos: 2, text: "c".into() });
            g
        };

        let reversed = group.reversed();
        assert_eq!(reversed[0], TestOp::Insert { pos: 2, text: "c".into() });
        assert_eq!(reversed[1], TestOp::Insert { pos: 1, text: "b".into() });
        assert_eq!(reversed[2], TestOp::Insert { pos: 0, text: "a".into() });
    }

    // =========================================================================
    // Edge Cases
    // =========================================================================

    #[test]
    fn test_undo_empty() {
        let mut history: UndoHistory<TestOp> = UndoHistory::new();
        assert_eq!(history.undo(), None);
    }

    #[test]
    fn test_redo_empty() {
        let mut history: UndoHistory<TestOp> = UndoHistory::new();
        assert_eq!(history.redo(), None);
    }

    #[test]
    fn test_set_max_size() {
        let mut history: UndoHistory<TestOp> = UndoHistory::new();

        for i in 0..10 {
            history.push(TestOp::Insert { pos: i, text: i.to_string() });
        }

        assert_eq!(history.undo_count(), 10);

        history.set_max_size(5);
        assert_eq!(history.undo_count(), 5);

        // Should have kept the last 5
        let op = history.undo();
        assert_eq!(op, Some(TestOp::Insert { pos: 9, text: "9".into() }));
    }
}
