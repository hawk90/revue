//! Grid widget helper functions tests

use revue::widget::grid;
use revue::widget::grid_item;
use revue::widget::grid_template;
use revue::widget::GridAlign;
use revue::widget::GridPlacement;
use revue::widget::Text;
use revue::widget::TrackSize;

#[test]
fn test_grid_function_creates_grid() {
    let g = grid();
    // The same empty grid as Grid::new()
    assert!(g.items.is_empty());
    assert!(g.columns.is_empty());
    assert!(g.rows.is_empty());
    assert!(g.auto_flow_row);
}

#[test]
fn test_grid_function_chainable() {
    let g = grid()
        .cols(3)
        .rows_count(2)
        .gap(5)
        .justify_items(GridAlign::Center);
    assert_eq!(g.columns, vec![TrackSize::Fr(1.0); 3]);
    assert_eq!(g.rows, vec![TrackSize::Fr(1.0); 2]);
    assert_eq!((g.col_gap, g.row_gap), (5, 5));
    assert_eq!(g.justify_items, GridAlign::Center);
}

#[test]
fn test_grid_item_function() {
    let item = grid_item(Text::new("test"));
    // Auto-placed until told otherwise
    assert_eq!(item.placement, GridPlacement::default());
}

#[test]
fn test_grid_item_with_placement() {
    let item = grid_item(Text::new("test")).at(2, 3);
    assert_eq!(item.placement, GridPlacement::cell(2, 3));
}

#[test]
fn test_grid_item_with_span() {
    let item = grid_item(Text::new("test"))
        .at(1, 1)
        .col_span(2)
        .row_span(3);
    assert_eq!(item.placement, GridPlacement::area(1, 1, 2, 3));
}

#[test]
fn test_grid_template_function() {
    let g = grid_template(3, 2);
    assert_eq!(g.columns, vec![TrackSize::Fr(1.0); 3]);
    assert_eq!(g.rows, vec![TrackSize::Fr(1.0); 2]);
    assert!(g.items.is_empty());
}

#[test]
fn test_grid_template_square() {
    let g = grid_template(4, 4);
    assert_eq!(g.columns.len(), 4);
    assert_eq!(g.rows.len(), 4);
}

#[test]
fn test_grid_template_single_row() {
    let g = grid_template(1, 1);
    assert_eq!(g.columns, vec![TrackSize::Fr(1.0)]);
    assert_eq!(g.rows, vec![TrackSize::Fr(1.0)]);
}

#[test]
fn test_grid_template_large() {
    let g = grid_template(10, 10);
    assert_eq!(g.columns.len(), 10);
    assert_eq!(g.rows.len(), 10);
}

#[test]
fn test_grid_with_cols() {
    let g = grid().cols(3);
    assert_eq!(g.columns, vec![TrackSize::Fr(1.0); 3]);
    assert!(g.rows.is_empty());
}

#[test]
fn test_grid_with_gap() {
    let g = grid().gap(10);
    assert_eq!((g.col_gap, g.row_gap), (10, 10));
}

#[test]
fn test_grid_with_col_gap() {
    let g = grid().col_gap(5);
    assert_eq!((g.col_gap, g.row_gap), (5, 0));
}

#[test]
fn test_grid_with_row_gap() {
    let g = grid().row_gap(5);
    assert_eq!((g.col_gap, g.row_gap), (0, 5));
}

#[test]
fn test_grid_with_justify_items() {
    let g = grid().justify_items(GridAlign::End);
    assert_eq!(g.justify_items, GridAlign::End);
    assert_eq!(g.align_items, GridAlign::Stretch);
}

#[test]
fn test_grid_with_align_items() {
    let g = grid().align_items(GridAlign::Center);
    assert_eq!(g.align_items, GridAlign::Center);
    assert_eq!(g.justify_items, GridAlign::Stretch);
}

#[test]
fn test_grid_auto_flow_row() {
    let g = grid().auto_flow_col().auto_flow_row();
    assert!(g.auto_flow_row);
}

#[test]
fn test_grid_auto_flow_col() {
    let g = grid().auto_flow_col();
    assert!(!g.auto_flow_row);
}

#[test]
fn test_grid_with_child() {
    let g = grid().child(Text::new("test"));
    assert_eq!(g.items.len(), 1);
    assert_eq!(g.items[0].placement, GridPlacement::default());
}

#[test]
fn test_grid_with_item() {
    let g = grid().item(grid_item(Text::new("test")).at(2, 1));
    assert_eq!(g.items.len(), 1);
    assert_eq!(g.items[0].placement, GridPlacement::cell(2, 1));
}

#[test]
fn test_grid_item_cell_placement() {
    let item = grid_item(Text::new("test")).place(GridPlacement::cell(1, 1));
    assert_eq!(
        item.placement,
        GridPlacement {
            col_start: 1,
            col_end: 2,
            row_start: 1,
            row_end: 2
        }
    );
}

#[test]
fn test_grid_item_area_placement() {
    let item = grid_item(Text::new("test")).place(GridPlacement::area(0, 0, 2, 2));
    assert_eq!(
        item.placement,
        GridPlacement {
            col_start: 0,
            col_end: 2,
            row_start: 0,
            row_end: 2
        }
    );
}

#[test]
fn test_grid_item_col_position() {
    let item = grid_item(Text::new("test")).col(5);
    assert_eq!((item.placement.col_start, item.placement.col_end), (5, 6));
    assert_eq!((item.placement.row_start, item.placement.row_end), (0, 0));
}

#[test]
fn test_grid_item_row_position() {
    let item = grid_item(Text::new("test")).row(3);
    assert_eq!((item.placement.row_start, item.placement.row_end), (3, 4));
    assert_eq!((item.placement.col_start, item.placement.col_end), (0, 0));
}
