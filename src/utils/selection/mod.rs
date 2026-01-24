//! List selection with viewport scrolling
//!
//! Provides wrap-around navigation for lists with automatic viewport management.
//!
//! # Example
//! ```ignore
//! let mut sel = Selection::new(100); // 100 items
//! sel.set_visible(10); // 10 visible rows
//!
//! sel.next(); // Move to next item (wraps around)
//! sel.prev(); // Move to previous item (wraps around)
//!
//! // Render only visible items
//! for i in sel.visible_range() {
//!     render_item(i, i == sel.index);
//! }
//! ```

mod core;
mod helper;
mod sectioned;
mod types;

pub use types::{SectionedSelection, Selection};

// Re-export helper functions
pub use helper::{wrap_next, wrap_prev};

#[cfg(test)]
mod tests;
