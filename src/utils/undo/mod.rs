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

mod core;
mod group;
mod merge;
mod query;
mod tests;
mod types;
mod undo_redo;

// Re-export all public types
pub use group::{GroupedUndoHistory, UndoGroup};
pub use types::{Mergeable, UndoHistory, DEFAULT_MAX_HISTORY};
