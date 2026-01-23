//! Multi-select widget for choosing multiple options from a list
//!
//! Provides a dropdown with:
//! - Multiple selection with tag display
//! - Fuzzy search filtering
//! - Tag navigation and removal
//! - Optional maximum selection limit

mod filter;
mod helpers;
mod key_handling;
mod navigation;
mod render;
mod selection;
mod tests;
mod types;

// Re-export public types
pub use types::{MultiSelect, MultiSelectOption};

// Re-export constructor functions
pub use helpers::{multi_select, multi_select_from};

// Macro implementations
crate::impl_styled_view!(MultiSelect);
crate::impl_widget_builders!(MultiSelect);
