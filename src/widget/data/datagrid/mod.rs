//! DataGrid widget for advanced data display
//!
//! A feature-rich data grid with sorting, filtering, and cell editing.

mod core;
mod editing;
mod export;
mod filter;
mod footer;
mod freeze;
mod mouse;
mod navigation;
mod render;
mod reorder;
mod resize;
mod tree;
mod types;
mod width;

// KEEP HERE - Private implementation tests in tests/ directory (accesses private fields of DataGrid)
#[cfg(test)]
mod tests;

// Re-export all types (explicit to avoid conflict with mod export)
#[allow(unused_imports)]
pub use types::{
    AggregationType, Alignment, ColumnAggregation, ColumnType, ExportFormat, ExportOptions,
    FooterRow, GridColors, GridColumn, GridOptions, GridRow, SortDirection,
};

// Re-export main widget
pub use core::DataGrid;

// Helper functions

/// Create a new data grid
pub fn datagrid() -> DataGrid {
    DataGrid::new()
}

/// Create a new grid column with key and title
pub fn grid_column(key: impl Into<String>, title: impl Into<String>) -> GridColumn {
    GridColumn::new(key, title)
}

/// Create a new grid row
pub fn grid_row() -> GridRow {
    GridRow::new()
}
