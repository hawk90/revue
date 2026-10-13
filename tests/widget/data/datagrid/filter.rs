//! DataGrid filter + selection interaction tests
//!
//! Verifies that filtering resets selection to prevent OOB access.

use revue::widget::data::datagrid::{DataGrid, GridColumn, GridRow};

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

#[test]
fn non_filterable_column_does_not_match_the_filter() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .column(GridColumn::new("b", "B").filterable(false))
        .row(GridRow::new().cell("a", "Alice").cell("b", "x"))
        .row(GridRow::new().cell("a", "Bob").cell("b", "Alice"));

    grid.set_filter("alice");
    assert_eq!(grid.filtered_count(), 1);
    assert_eq!(grid.filtered_rows()[0].get("a"), Some("Alice"));

    // Filtering on the non-filterable column directly matches nothing.
    grid.filter_column = Some(1);
    grid.set_filter("alice");
    assert_eq!(grid.filtered_count(), 0);
}

#[test]
fn test_datagrid_recompute_cache_initializes() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .row(GridRow::new().cell("a", "1"))
        .row(GridRow::new().cell("a", "2"));

    grid.recompute_cache();
    assert_eq!(grid.filtered_cache, vec![0, 1]);
}

#[test]
fn test_datagrid_filtered_indices() {
    let grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .row(GridRow::new().cell("a", "1"))
        .row(GridRow::new().cell("a", "2"));

    assert_eq!(grid.filtered_indices(), &[0, 1]);
}

#[test]
fn test_datagrid_filtered_count() {
    let grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .row(GridRow::new().cell("a", "1"))
        .row(GridRow::new().cell("a", "2"))
        .row(GridRow::new().cell("a", "3"));

    assert_eq!(grid.filtered_count(), 3);
}

#[test]
fn test_datagrid_filtered_rows() {
    let grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .row(GridRow::new().cell("a", "x"))
        .row(GridRow::new().cell("a", "y"));

    let rows = grid.filtered_rows();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].get("a"), Some("x"));
    assert_eq!(rows[1].get("a"), Some("y"));
}
