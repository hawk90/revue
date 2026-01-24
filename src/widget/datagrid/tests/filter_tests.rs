#![allow(unused_imports)]

use super::super::*;

#[test]
fn test_filtering() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(GridRow::new().cell("name", "Alice"))
        .row(GridRow::new().cell("name", "Bob"))
        .row(GridRow::new().cell("name", "Alex"));

    grid.set_filter("al");

    let filtered = grid.filtered_rows();
    assert_eq!(filtered.len(), 2);
}

#[test]
fn test_filter_cache() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(GridRow::new().cell("name", "Alice"))
        .row(GridRow::new().cell("name", "Bob"))
        .row(GridRow::new().cell("name", "Alex"))
        .row(GridRow::new().cell("name", "Charlie"));

    // Initial: all 4 rows
    assert_eq!(grid.filtered_count(), 4);

    // Multiple calls should use cache
    assert_eq!(grid.filtered_count(), 4);
    assert_eq!(grid.row_count(), 4);

    // Filter: only "al" matches
    grid.set_filter("al");
    assert_eq!(grid.filtered_count(), 2);

    // Cache should be invalidated and recomputed
    assert_eq!(grid.filtered_rows().len(), 2);

    // Clear filter
    grid.set_filter("");
    assert_eq!(grid.filtered_count(), 4);
}

#[test]
fn test_filter_specific_column() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .column(GridColumn::new("b", "B"))
        .row(GridRow::new().cell("a", "Alice").cell("b", "Smith"))
        .row(GridRow::new().cell("a", "Bob").cell("b", "Alice"));

    grid.filter_column = Some(0);
    grid.set_filter("alice");

    // Should only match first row (column A)
    assert_eq!(grid.filtered_count(), 1);
}

#[test]
fn test_filter_cancels_edit() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A").editable(true))
        .row(GridRow::new().cell("a", "test"));

    grid.start_edit();
    assert!(grid.is_editing());

    grid.set_filter("x");
    assert!(!grid.is_editing());
}
