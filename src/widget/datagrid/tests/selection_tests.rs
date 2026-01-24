#![allow(unused_imports)]

use super::super::*;

#[test]
fn test_toggle_selection() {
    let mut grid = DataGrid::new()
        .multi_select(true)
        .row(GridRow::new().cell("a", "1"))
        .row(GridRow::new().cell("a", "2"));

    assert!(!grid.rows[0].selected);
    grid.toggle_selection();
    assert!(grid.rows[0].selected);
    grid.toggle_selection();
    assert!(!grid.rows[0].selected);
}

#[test]
fn test_toggle_selection_without_multi_select() {
    let mut grid = DataGrid::new()
        .multi_select(false)
        .row(GridRow::new().cell("a", "1"));

    grid.toggle_selection();
    // Should not toggle when multi_select is disabled
    assert!(!grid.rows[0].selected);
}

#[test]
fn test_selected_rows() {
    let mut grid = DataGrid::new()
        .multi_select(true)
        .row(GridRow::new().cell("a", "1"))
        .row(GridRow::new().cell("a", "2"))
        .row(GridRow::new().cell("a", "3"));

    grid.rows[0].selected = true;
    grid.rows[2].selected = true;

    let selected = grid.selected_rows();
    assert_eq!(selected.len(), 2);
}
