#![allow(unused_imports)]

use super::super::*;

#[test]
fn test_freeze_columns_left() {
    let grid = DataGrid::new()
        .freeze_columns_left(2)
        .column(GridColumn::new("a", "A"))
        .column(GridColumn::new("b", "B"))
        .column(GridColumn::new("c", "C"));

    assert_eq!(grid.frozen_left(), 2);
}

#[test]
fn test_freeze_columns_right() {
    let grid = DataGrid::new()
        .freeze_columns_right(1)
        .column(GridColumn::new("a", "A"))
        .column(GridColumn::new("b", "B"))
        .column(GridColumn::new("c", "C"));

    assert_eq!(grid.frozen_right(), 1);
}

#[test]
fn test_horizontal_scroll() {
    let mut grid = DataGrid::new()
        .freeze_columns_left(1)
        .column(GridColumn::new("a", "A"))
        .column(GridColumn::new("b", "B"))
        .column(GridColumn::new("c", "C"))
        .column(GridColumn::new("d", "D"))
        .column(GridColumn::new("e", "E"))
        .row(GridRow::new());

    // Initial scroll position
    assert_eq!(grid.scroll_col, 0);

    // Scroll right
    grid.scroll_col_right();
    assert_eq!(grid.scroll_col, 1);

    grid.scroll_col_right();
    assert_eq!(grid.scroll_col, 2);

    // Scroll left
    grid.scroll_col_left();
    assert_eq!(grid.scroll_col, 1);

    // Can't scroll past 0
    grid.scroll_col_left();
    grid.scroll_col_left();
    assert_eq!(grid.scroll_col, 0);
}
