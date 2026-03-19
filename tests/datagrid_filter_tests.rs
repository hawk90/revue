//! DataGrid filter + selection interaction tests
//!
//! Verifies that filtering resets selection to prevent OOB access.

use revue::widget::datagrid::{DataGrid, GridColumn, GridRow};

fn make_grid() -> DataGrid {
    DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .column(GridColumn::new("value", "Value"))
        .row(GridRow::new().cell("name", "Apple").cell("value", "100"))
        .row(GridRow::new().cell("name", "Banana").cell("value", "200"))
        .row(GridRow::new().cell("name", "Cherry").cell("value", "300"))
        .row(GridRow::new().cell("name", "Date").cell("value", "400"))
        .row(
            GridRow::new()
                .cell("name", "Elderberry")
                .cell("value", "500"),
        )
}

#[test]
fn test_filter_resets_selection_to_zero() {
    let mut grid = make_grid();

    grid.select_next();
    grid.select_next();
    grid.select_next();
    assert_eq!(grid.selected_row, 3);

    grid.set_filter("a");
    assert_eq!(grid.selected_row, 0);
}

#[test]
fn test_filter_resets_scroll_to_zero() {
    let mut grid = make_grid();
    grid.scroll_row = 3;

    grid.set_filter("cherry");
    assert_eq!(grid.scroll_row, 0);
}

#[test]
fn test_filter_then_navigate_works() {
    let mut grid = make_grid();
    grid.set_filter("a");

    let count = grid.filtered_count();
    assert!(count > 0);

    grid.select_next();
    assert_eq!(grid.selected_row, 1);
}

#[test]
fn test_filter_empty_result() {
    let mut grid = make_grid();
    grid.set_filter("zzzzz");

    assert_eq!(grid.filtered_count(), 0);
    assert_eq!(grid.selected_row, 0);
}
