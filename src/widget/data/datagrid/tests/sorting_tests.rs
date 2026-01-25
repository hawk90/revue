#![allow(unused_imports)]

use super::super::*;

#[test]
fn test_sorting() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(GridRow::new().cell("name", "Charlie"))
        .row(GridRow::new().cell("name", "Alice"))
        .row(GridRow::new().cell("name", "Bob"));

    grid.sort(0);

    assert_eq!(grid.rows[0].get("name"), Some("Alice"));
    assert_eq!(grid.rows[1].get("name"), Some("Bob"));
    assert_eq!(grid.rows[2].get("name"), Some("Charlie"));
}

#[test]
fn test_natural_sorting() {
    let mut grid = DataGrid::new()
        .natural_sort(true)
        .column(GridColumn::new("file", "File"))
        .row(GridRow::new().cell("file", "file10.txt"))
        .row(GridRow::new().cell("file", "file2.txt"))
        .row(GridRow::new().cell("file", "file1.txt"));

    grid.sort(0);

    assert_eq!(grid.rows[0].get("file"), Some("file1.txt"));
    assert_eq!(grid.rows[1].get("file"), Some("file2.txt"));
    assert_eq!(grid.rows[2].get("file"), Some("file10.txt"));
}

#[test]
fn test_ascii_sorting() {
    let mut grid = DataGrid::new()
        .natural_sort(false)
        .column(GridColumn::new("file", "File"))
        .row(GridRow::new().cell("file", "file10.txt"))
        .row(GridRow::new().cell("file", "file2.txt"))
        .row(GridRow::new().cell("file", "file1.txt"));

    grid.sort(0);

    // ASCII sort: "file1" < "file10" < "file2"
    assert_eq!(grid.rows[0].get("file"), Some("file1.txt"));
    assert_eq!(grid.rows[1].get("file"), Some("file10.txt"));
    assert_eq!(grid.rows[2].get("file"), Some("file2.txt"));
}

#[test]
fn test_cache_invalidation_on_sort() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(GridRow::new().cell("name", "Charlie"))
        .row(GridRow::new().cell("name", "Alice"))
        .row(GridRow::new().cell("name", "Bob"));

    // Access cache
    assert_eq!(grid.filtered_count(), 3);

    // Sort should invalidate cache
    grid.sort(0);

    // Cache should still work correctly after sort
    assert_eq!(grid.filtered_count(), 3);
    let rows = grid.filtered_rows();
    assert_eq!(rows[0].get("name"), Some("Alice"));
}

#[test]
fn test_sort_invalid_column() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .row(GridRow::new().cell("a", "1"));

    // Sorting invalid column should be no-op
    grid.sort(99);
    assert!(grid.sort_column.is_none());
}

#[test]
fn test_sort_unsortable_column() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A").sortable(false))
        .row(GridRow::new().cell("a", "1"));

    grid.sort(0);
    assert!(grid.sort_column.is_none());
}

#[test]
fn test_sort_toggle_direction() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .row(GridRow::new().cell("a", "B"))
        .row(GridRow::new().cell("a", "A"));

    grid.sort(0); // Ascending
    assert_eq!(grid.sort_direction, SortDirection::Ascending);
    assert_eq!(grid.rows[0].get("a"), Some("A"));

    grid.sort(0); // Toggle to descending
    assert_eq!(grid.sort_direction, SortDirection::Descending);
    assert_eq!(grid.rows[0].get("a"), Some("B"));
}

#[test]
fn test_sort_number_column() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("num", "Number").col_type(ColumnType::Number))
        .row(GridRow::new().cell("num", "10"))
        .row(GridRow::new().cell("num", "2"))
        .row(GridRow::new().cell("num", "100"));

    grid.sort(0);

    assert_eq!(grid.rows[0].get("num"), Some("2"));
    assert_eq!(grid.rows[1].get("num"), Some("10"));
    assert_eq!(grid.rows[2].get("num"), Some("100"));
}

#[test]
fn test_sort_cancels_edit() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A").editable(true))
        .row(GridRow::new().cell("a", "B"))
        .row(GridRow::new().cell("a", "A"));

    grid.start_edit();
    assert!(grid.is_editing());

    grid.sort(0);
    assert!(!grid.is_editing());
}
