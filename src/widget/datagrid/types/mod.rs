//! DataGrid types and configurations
//!
//! This module contains all standalone types used by the DataGrid widget.

// Submodules
pub mod aggregation;
pub mod colors;
pub mod column;
pub mod column_types;
pub mod export;
pub mod options;
pub mod row;

#[cfg(test)]
mod tests;

// Re-exports for backward compatibility
pub use aggregation::{AggregationType, ColumnAggregation, FooterRow};
pub use colors::GridColors;
pub use column::GridColumn;
pub use column_types::{Alignment, ColumnType, SortDirection};
pub use export::{ExportFormat, ExportOptions};
pub use options::GridOptions;
pub use row::GridRow;
