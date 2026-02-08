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
    use super::super::types::GridPlacement;
    use super::*;
    use crate::widget::grid::GridAlign;
    use crate::widget::Text;

    #[test]
    fn test_grid_function_creates_grid() {
        let g = grid();
        // Grid should be created successfully
        let _ = g;
    }

    #[test]
    fn test_grid_function_chainable() {
        let g = grid()
            .cols(3)
            .rows_count(2)
            .gap(5)
            .justify_items(GridAlign::Center);
        let _ = g;
    }

    #[test]
    fn test_grid_item_function() {
        let item = grid_item(Text::new("test"));
        let _ = item;
    }

    #[test]
    fn test_grid_item_with_placement() {
        let item = grid_item(Text::new("test")).at(0, 0);
        let _ = item;
    }

    #[test]
    fn test_grid_item_with_span() {
        let item = grid_item(Text::new("test")).col_span(2).row_span(3);
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

    #[test]
    fn test_grid_template_single_row() {
        let g = grid_template(1, 1);
        let _ = g;
    }

    #[test]
    fn test_grid_template_large() {
        let g = grid_template(10, 10);
        let _ = g;
    }

    #[test]
    fn test_grid_with_cols() {
        let g = grid().cols(3);
        let _ = g;
    }

    #[test]
    fn test_grid_with_gap() {
        let g = grid().gap(10);
        let _ = g;
    }

    #[test]
    fn test_grid_with_col_gap() {
        let g = grid().col_gap(5);
        let _ = g;
    }

    #[test]
    fn test_grid_with_row_gap() {
        let g = grid().row_gap(5);
        let _ = g;
    }

    #[test]
    fn test_grid_with_justify_items() {
        let g = grid().justify_items(GridAlign::End);
        let _ = g;
    }

    #[test]
    fn test_grid_with_align_items() {
        let g = grid().align_items(GridAlign::Center);
        let _ = g;
    }

    #[test]
    fn test_grid_auto_flow_row() {
        let g = grid().auto_flow_row();
        let _ = g;
    }

    #[test]
    fn test_grid_auto_flow_col() {
        let g = grid().auto_flow_col();
        let _ = g;
    }

    #[test]
    fn test_grid_with_child() {
        let g = grid().child(Text::new("test"));
        let _ = g;
    }

    #[test]
    fn test_grid_with_item() {
        let g = grid().item(grid_item(Text::new("test")));
        let _ = g;
    }

    #[test]
    fn test_grid_item_cell_placement() {
        let item = grid_item(Text::new("test")).place(GridPlacement::cell(1, 1));
        let _ = item;
    }

    #[test]
    fn test_grid_item_area_placement() {
        let item = grid_item(Text::new("test")).place(GridPlacement::area(0, 0, 2, 2));
        let _ = item;
    }

    #[test]
    fn test_grid_item_col_position() {
        let item = grid_item(Text::new("test")).col(5);
        let _ = item;
    }

    #[test]
    fn test_grid_item_row_position() {
        let item = grid_item(Text::new("test")).row(3);
        let _ = item;
    }
}
