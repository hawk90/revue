//! Grouped undo history support

use super::types::UndoHistory;

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
    /// Operations in the group
    pub operations: Vec<T>,
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
    pub history: UndoHistory<UndoGroup<T>>,
    /// Current group being built (if any)
    pub current_group: Option<UndoGroup<T>>,
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
