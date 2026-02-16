//! Grid widget helper functions

use super::core::Grid;

/// Helper to create a grid
pub fn grid() -> Grid {
    Grid::new()
}

/// Helper to create a grid item
pub fn grid_item(widget: impl crate::widget::traits::View + 'static) -> super::types::GridItem {
    super::types::GridItem::new(widget)
}

/// Create a simple NxM grid
pub fn grid_template(cols: usize, rows: usize) -> Grid {
    Grid::new().cols(cols).rows_count(rows)
}
