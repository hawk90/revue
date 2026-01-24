//! Breadcrumb navigation widget
//!
//! Shows hierarchical navigation path with clickable segments.

mod core;
mod helper;
mod types;

pub use types::{BreadcrumbItem, SeparatorStyle};

// Re-export core types
pub use core::Breadcrumb;

// Re-export helper functions
pub use helper::{breadcrumb, crumb};

#[cfg(test)]
mod tests;
