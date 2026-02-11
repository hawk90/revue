//! Sortable list widget with drag-and-drop reordering
//!
//! A list widget that allows items to be reordered via drag-and-drop.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::SortableList;
//!
//! let items = vec!["First", "Second", "Third"];
//! SortableList::new(items)
//!     .on_reorder(|from, to| {
//!         println!("Moved item from {} to {}", from, to);
//!     })
//! ```

mod builder;
mod core;
mod helper;
mod impls;
mod types;
mod view;

// Re-export main types
pub use core::SortableList;
pub use helper::sortable_list;
pub use types::SortableItem;
