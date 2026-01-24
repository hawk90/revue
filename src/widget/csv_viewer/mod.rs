//! CSV Viewer widget for displaying tabular CSV data
//!
//! Features:
//! - Auto-detect delimiters (comma, tab, semicolon, pipe)
//! - Header row detection
//! - Column sorting (ascending/descending)
//! - Search across all cells
//! - Virtual scrolling for large files
//! - Column width auto-sizing
//! - Row numbering

mod core;
mod helpers;
mod types;
mod view;

pub use core::CsvViewer;
pub use helpers::csv_viewer;
pub use types::{Delimiter, SortOrder};

crate::impl_styled_view!(CsvViewer);
crate::impl_props_builders!(CsvViewer);
