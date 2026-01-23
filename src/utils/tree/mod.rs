//! Tree navigation and rendering utilities
//!
//! Provides utilities for hierarchical/tree-structured UIs with collapsible sections.
//!
//! # Components
//!
//! - [`TreeNav`]: Navigation logic for tree structures with collapsible nodes
//! - [`TreePrefix`]: Tree line prefix generator (├─, └─, │)
//! - [`Indent`]: Indentation level management
//! - [`TreeIcons`]: Icons for tree UI elements (selection, collapse, etc.)

mod navigation;
mod prefix;
mod tests;
mod types;

// Re-exports
pub use navigation::TreeNav;
pub use prefix::tree_chars;
pub use prefix::TreePrefix;
pub use types::Indent;
pub use types::TreeIcons;
pub use types::TreeItem;
