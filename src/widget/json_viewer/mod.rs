//! JSON Viewer widget for displaying and navigating JSON data
//!
//! Features:
//! - Collapsible tree structure
//! - Syntax highlighting by type (string, number, boolean, null)
//! - Search functionality
//! - Expand/collapse all
//! - Copy path/value support
//! - Virtual scrolling for large documents

#![allow(dead_code)]

mod helpers;
mod parser;
mod search;
mod types;
mod view;

pub use search::Search;
pub use types::{JsonNode, JsonType};
pub use view::JsonViewer;

// Re-export helper
pub use helpers::json_viewer;

#[cfg(test)]
mod tests;
