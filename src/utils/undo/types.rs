//! Type definitions for undo/redo history management

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
    pub undo_stack: VecDeque<T>,
    /// Stack of operations that can be redone
    pub redo_stack: Vec<T>,
    /// Maximum number of operations to keep
    pub max_size: usize,
}

impl<T> Default for UndoHistory<T> {
    fn default() -> Self {
        Self::new()
    }
}
