#![allow(unused_imports)]

use super::super::*;

#[test]
fn test_select_next_col() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .column(GridColumn::new("b", "B"))
        .column(GridColumn::new("c", "C"));

    assert_eq!(grid.selected_col, 0);
    grid.select_next_col();
    assert_eq!(grid.selected_col, 1);
    grid.select_next_col();
    assert_eq!(grid.selected_col, 2);
    grid.select_next_col();
    assert_eq!(grid.selected_col, 2); // Can't go past last
}

#[test]
fn test_select_prev_col() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .column(GridColumn::new("b", "B"));

    grid.selected_col = 1;
    grid.select_prev_col();
    assert_eq!(grid.selected_col, 0);
    grid.select_prev_col();
    assert_eq!(grid.selected_col, 0); // Can't go before first
}

#[test]
fn test_page_up() {
    let mut grid = DataGrid::new().column(GridColumn::new("a", "A"));

    let rows: Vec<_> = (0..50)
        .map(|i| GridRow::new().cell("a", i.to_string()))
        .collect();
    grid = grid.rows(rows);

    grid.selected_row = 25;
    grid.page_up(10);
    assert_eq!(grid.selected_row, 15);

    grid.page_up(20);
    assert_eq!(grid.selected_row, 0); // Clamped to 0
}

#[test]
fn test_ensure_visible_with_height() {
    let mut grid = DataGrid::new().column(GridColumn::new("a", "A"));

    let rows: Vec<_> = (0..100)
        .map(|i| GridRow::new().cell("a", i.to_string()))
        .collect();
    grid = grid.rows(rows);

    grid.selected_row = 50;
    grid.scroll_row = 0;
    grid.ensure_visible_with_height(10);

    // Scroll should adjust to show selected row
    assert!(grid.scroll_row > 0);
}

#[test]
fn test_set_viewport_height() {
    let mut grid = DataGrid::new().column(GridColumn::new("a", "A"));

    let rows: Vec<_> = (0..50)
        .map(|i| GridRow::new().cell("a", i.to_string()))
        .collect();
    grid = grid.rows(rows);

    grid.selected_row = 30;
    grid.scroll_row = 0;
    grid.set_viewport_height(10);

    assert!(grid.scroll_row > 0);
}

#[test]
fn test_scroll_info() {
    let grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .row(GridRow::new().cell("a", "1"))
        .row(GridRow::new().cell("a", "2"));

    let (scroll, total, _viewport) = grid.scroll_info();
    assert_eq!(scroll, 0);
    assert_eq!(total, 2);
}

#[test]
fn test_visible_row_count() {
    let grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .row(GridRow::new().cell("a", "1"))
        .row(GridRow::new().cell("a", "2"));

    assert_eq!(grid.visible_row_count(), 2);
}
