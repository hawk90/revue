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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widget::Text;

    #[test]
    fn test_grid_function() {
        let g = grid();
        let _ = g;
    }

    #[test]
    fn test_grid_item_function() {
        let item = grid_item(Text::new("test"));
        let _ = item;
    }

    #[test]
    fn test_grid_template_function() {
        let g = grid_template(3, 2);
        let _ = g;
    }

    #[test]
    fn test_grid_template_square() {
        let g = grid_template(4, 4);
        let _ = g;
    }
}
