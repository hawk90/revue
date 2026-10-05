//! Grid widget core implementation tests

use revue::widget::{Grid, GridAlign, GridItem, TrackSize};

// =========================================================================
// Constructor tests
// =========================================================================

#[test]
fn test_grid_new() {
    let grid = Grid::new();
    assert!(grid.items.is_empty());
    assert!(grid.columns.is_empty());
    assert!(grid.rows.is_empty());
    assert_eq!(grid.col_gap, 0);
    assert_eq!(grid.row_gap, 0);
    assert_eq!(grid.justify_items, GridAlign::Stretch);
    assert_eq!(grid.align_items, GridAlign::Stretch);
    assert!(grid.auto_flow_row);
    assert_eq!(grid.auto_cols, TrackSize::Fr(1.0));
    assert_eq!(grid.auto_rows, TrackSize::Fr(1.0));
}

// =========================================================================
// Builder method tests
// =========================================================================

#[test]
fn test_grid_columns() {
    let grid = Grid::new().columns(vec![TrackSize::Fixed(10), TrackSize::Fr(1.0)]);
    assert_eq!(grid.columns.len(), 2);
    assert_eq!(grid.columns[0], TrackSize::Fixed(10));
    assert_eq!(grid.columns[1], TrackSize::Fr(1.0));
}

#[test]
fn test_grid_rows() {
    let grid = Grid::new().rows(vec![TrackSize::Auto, TrackSize::Fixed(5)]);
    assert_eq!(grid.rows.len(), 2);
    assert_eq!(grid.rows[0], TrackSize::Auto);
    assert_eq!(grid.rows[1], TrackSize::Fixed(5));
}

#[test]
fn test_grid_cols() {
    let grid = Grid::new().cols(3);
    assert_eq!(grid.columns.len(), 3);
    assert_eq!(grid.columns[0], TrackSize::Fr(1.0));
    assert_eq!(grid.columns[1], TrackSize::Fr(1.0));
    assert_eq!(grid.columns[2], TrackSize::Fr(1.0));
}

#[test]
fn test_grid_rows_count() {
    let grid = Grid::new().rows_count(4);
    assert_eq!(grid.rows.len(), 4);
    for row in &grid.rows {
        assert_eq!(*row, TrackSize::Fr(1.0));
    }
}

#[test]
fn test_grid_col_gap() {
    let grid = Grid::new().col_gap(5);
    assert_eq!(grid.col_gap, 5);
}

#[test]
fn test_grid_row_gap() {
    let grid = Grid::new().row_gap(3);
    assert_eq!(grid.row_gap, 3);
}

#[test]
fn test_grid_justify_items() {
    let grid = Grid::new().justify_items(GridAlign::Center);
    assert_eq!(grid.justify_items, GridAlign::Center);
}

#[test]
fn test_grid_align_items() {
    let grid = Grid::new().align_items(GridAlign::End);
    assert_eq!(grid.align_items, GridAlign::End);
}

#[test]
fn test_grid_auto_flow_row() {
    let grid = Grid::new().auto_flow_row();
    assert!(grid.auto_flow_row);
}

#[test]
fn test_grid_auto_flow_col() {
    let grid = Grid::new().auto_flow_col();
    assert!(!grid.auto_flow_row);
}

#[test]
fn test_grid_auto_cols() {
    let grid = Grid::new().auto_cols(TrackSize::Fixed(10));
    assert_eq!(grid.auto_cols, TrackSize::Fixed(10));
}

#[test]
fn test_grid_auto_rows() {
    let grid = Grid::new().auto_rows(TrackSize::Auto);
    assert_eq!(grid.auto_rows, TrackSize::Auto);
}

// =========================================================================
// Item builder tests
// =========================================================================

#[test]
fn test_grid_item() {
    let item = GridItem::new(revue::widget::Text::new("Test"));
    let grid = Grid::new().item(item);
    assert_eq!(grid.items.len(), 1);
}

#[test]
fn test_grid_child() {
    let grid = Grid::new().child(revue::widget::Text::new("Child"));
    assert_eq!(grid.items.len(), 1);
}

#[test]
fn test_grid_children() {
    let widgets: Vec<Box<dyn revue::widget::traits::View>> = vec![
        Box::new(revue::widget::Text::new("A")),
        Box::new(revue::widget::Text::new("B")),
        Box::new(revue::widget::Text::new("C")),
    ];
    let grid = Grid::new().children(widgets);
    assert_eq!(grid.items.len(), 3);
}

// =========================================================================
// Default trait tests
// =========================================================================

#[test]
fn test_grid_default() {
    let grid = Grid::default();
    assert!(grid.items.is_empty());
    assert!(grid.columns.is_empty());
    assert!(grid.rows.is_empty());
    assert_eq!(grid.col_gap, 0);
    assert_eq!(grid.row_gap, 0);
}

// =========================================================================
// Builder chain tests
// =========================================================================

#[test]
fn test_grid_builder_chain() {
    let grid = Grid::new()
        .cols(3)
        .rows_count(2)
        .gap(2)
        .justify_items(GridAlign::Center)
        .align_items(GridAlign::Start)
        .child(revue::widget::Text::new("A"))
        .child(revue::widget::Text::new("B"));

    assert_eq!(grid.columns.len(), 3);
    assert_eq!(grid.rows.len(), 2);
    assert_eq!(grid.col_gap, 2);
    assert_eq!(grid.row_gap, 2);
    assert_eq!(grid.justify_items, GridAlign::Center);
    assert_eq!(grid.align_items, GridAlign::Start);
    assert_eq!(grid.items.len(), 2);
}
