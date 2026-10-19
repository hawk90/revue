//! DataGrid sorting tests

use proptest::prelude::*;
use revue::widget::data::datagrid::{ColumnType, DataGrid, GridColumn, GridRow, SortDirection};

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

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// Sorting a number column orders it ascending, sorting it again
    /// descending, and the rows are the same rows reordered
    #[test]
    fn sort_orders_a_number_column(values in prop::collection::vec(-1000i32..1000, 0..40)) {
        let mut grid = DataGrid::new()
            .column(GridColumn::new("n", "N").col_type(ColumnType::Number))
            .column(GridColumn::new("id", "Id"))
            .rows(
                values
                    .iter()
                    .enumerate()
                    .map(|(i, v)| GridRow::new().cell("n", v.to_string()).cell("id", i.to_string()))
                    .collect(),
            );
        let column = |grid: &DataGrid| -> Vec<i32> {
            grid.rows.iter().map(|r| r.get("n").unwrap().parse().unwrap()).collect()
        };
        let mut ascending = values.clone();
        ascending.sort();

        grid.sort(0);
        prop_assert_eq!(column(&grid), ascending.clone());
        grid.sort(0);
        let mut descending = ascending;
        descending.reverse();
        prop_assert_eq!(column(&grid), descending);

        let mut ids: Vec<usize> = grid.rows.iter().map(|r| r.get("id").unwrap().parse().unwrap()).collect();
        ids.sort();
        prop_assert_eq!(ids, (0..values.len()).collect::<Vec<_>>());
    }

    /// A multi-column sort orders by the first key and breaks ties with the
    /// second (here descending, after a second add_sort)
    #[test]
    fn multi_sort_orders_by_the_keys_in_priority(rows in prop::collection::vec((0u8..4, 0i32..50), 0..40)) {
        let mut grid = DataGrid::new()
            .column(GridColumn::new("k", "K"))
            .column(GridColumn::new("n", "N").col_type(ColumnType::Number))
            .rows(
                rows.iter()
                    .map(|(k, n)| GridRow::new().cell("k", format!("k{k}")).cell("n", n.to_string()))
                    .collect(),
            );
        grid.add_sort(0);
        grid.add_sort(1);
        grid.add_sort(1);

        let got: Vec<(String, i32)> = grid
            .rows
            .iter()
            .map(|r| (r.get("k").unwrap().to_string(), r.get("n").unwrap().parse().unwrap()))
            .collect();
        let mut expected: Vec<(String, i32)> = rows.iter().map(|(k, n)| (format!("k{k}"), *n)).collect();
        expected.sort_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));
        prop_assert_eq!(got, expected);
    }
}

/// A "NaN" cell in a number column sorts to one end instead of panicking
/// the sort (#986)
#[test]
fn test_datagrid_sort_number_column_with_nan() {
    let rows = (0..40)
        .map(|i| {
            let v = if i % 3 == 0 {
                "NaN".to_string()
            } else {
                (i % 11).to_string()
            };
            GridRow::new().cell("n", v)
        })
        .collect();
    let mut grid = DataGrid::new()
        .column(GridColumn::new("n", "N").col_type(ColumnType::Number))
        .rows(rows);
    grid.sort(0);
    assert_eq!(grid.row_count(), 40);
}
