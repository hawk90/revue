//! DataGrid footer aggregation over the shown rows

use super::super::footer_cells;
use revue::widget::data::datagrid::{
    AggregationType, ColumnAggregation, DataGrid, FooterRow, GridColumn, GridRow,
};

#[test]
fn test_footer_sum() {
    let grid = DataGrid::new()
        .column(GridColumn::new("value", "Value"))
        .row(GridRow::new().cell("value", "10"))
        .row(GridRow::new().cell("value", "20"))
        .row(GridRow::new().cell("value", "30"))
        .add_sum("value");

    assert!(grid.show_footer);
    assert_eq!(grid.footer_rows.len(), 1);

    assert_eq!(footer_cells(&grid, 0)[0], "Sum: 60");
}

#[test]
fn test_footer_average() {
    let grid = DataGrid::new()
        .column(GridColumn::new("value", "Value"))
        .row(GridRow::new().cell("value", "10"))
        .row(GridRow::new().cell("value", "20"))
        .row(GridRow::new().cell("value", "30"))
        .add_average("value");

    assert_eq!(footer_cells(&grid, 0)[0], "Avg: 20");
}

#[test]
fn test_footer_count() {
    let grid = DataGrid::new()
        .column(GridColumn::new("value", "Value"))
        .row(GridRow::new().cell("value", "10"))
        .row(GridRow::new().cell("value", "20"))
        .row(GridRow::new().cell("value", "30"))
        .footer(FooterRow::new("").count("value"));

    assert_eq!(footer_cells(&grid, 0)[0], "Count: 3");
}

#[test]
fn test_footer_min_max() {
    let grid = DataGrid::new()
        .column(GridColumn::new("value", "Value"))
        .row(GridRow::new().cell("value", "5"))
        .row(GridRow::new().cell("value", "15"))
        .row(GridRow::new().cell("value", "10"))
        .footer(FooterRow::new("").min("value"))
        .footer(FooterRow::new("").max("value"));

    assert_eq!(footer_cells(&grid, 1)[0], "Min: 5");

    assert_eq!(footer_cells(&grid, 0)[0], "Max: 15");
}

#[test]
fn test_footer_with_filter() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .column(GridColumn::new("value", "Value"))
        .row(GridRow::new().cell("name", "Apple").cell("value", "10"))
        .row(GridRow::new().cell("name", "Banana").cell("value", "20"))
        .row(GridRow::new().cell("name", "Cherry").cell("value", "30"))
        .add_sum("value");

    // Sum all
    assert_eq!(footer_cells(&grid, 0)[1], "Sum: 60");

    // Filter to "Ap" items (only Apple matches)
    grid.set_filter("Ap");

    // Sum only filtered items (Apple=10)
    assert_eq!(footer_cells(&grid, 0)[1], "Sum: 10");
}

#[test]
fn test_aggregation_non_numeric() {
    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(GridRow::new().cell("name", "Alice"))
        .row(GridRow::new().cell("name", "Bob"))
        .add_sum("name");

    // Non-numeric values should return None for sum/avg
    assert_eq!(footer_cells(&grid, 0)[0], "Sum: —");
}

#[test]
fn test_footer_values() {
    let grid = DataGrid::new()
        .column(GridColumn::new("value", "Value"))
        .row(GridRow::new().cell("value", "10"))
        .row(GridRow::new().cell("value", "20"))
        .footer(FooterRow::new("Totals").sum("value"));

    assert_eq!(footer_cells(&grid, 0)[0], "Sum: 30"); // Sum of 10+20
}

#[test]
fn test_footer_values_with_label() {
    let grid = DataGrid::new()
        .column(GridColumn::new("value", "Value"))
        .row(GridRow::new().cell("value", "100"))
        .footer(
            FooterRow::new("Stats")
                .aggregation(ColumnAggregation::new("value", AggregationType::Sum).label("Total")),
        );

    assert_eq!(footer_cells(&grid, 0)[0], "Total: 100");
}

#[test]
fn test_aggregation_empty_data() {
    let grid = DataGrid::new()
        .column(GridColumn::new("value", "Value"))
        .add_sum("value");

    // No rows, should return None
    assert_eq!(footer_cells(&grid, 0)[0], "Sum: —");
}

/// Count is the number of shown rows, whatever the column holds: text,
/// empty and missing cells count too, and no rows is 0, not "—" (#1538)
#[test]
fn test_footer_count_is_the_shown_rows() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .column(GridColumn::new("value", "Value"))
        .row(GridRow::new().cell("name", "Apple").cell("value", "10"))
        .row(GridRow::new().cell("name", "Banana").cell("value", "n/a"))
        .row(GridRow::new().cell("name", "Cherry").cell("value", ""))
        .row(GridRow::new().cell("name", "Apricot"))
        .footer(FooterRow::new("").count("name").count("value"));

    assert_eq!(footer_cells(&grid, 0)[0], "Count: 4", "a text column");
    assert_eq!(
        footer_cells(&grid, 0)[1],
        "Count: 4",
        "number, text, empty, missing"
    );

    grid.set_filter("Ap");
    assert_eq!(footer_cells(&grid, 0)[0], "Count: 2", "only the shown rows");

    grid.set_filter("zzz");
    assert_eq!(footer_cells(&grid, 0)[0], "Count: 0", "no rows shown");
}

/// The other aggregations still use the cells that parse as numbers, and show
/// "—" when there are none
#[test]
fn test_footer_sum_skips_non_numbers_and_has_no_value_without_numbers() {
    let grid = DataGrid::new()
        .column(GridColumn::new("value", "Value"))
        .row(GridRow::new().cell("value", "10"))
        .row(GridRow::new().cell("value", "n/a"))
        .row(GridRow::new().cell("value", ""))
        .add_sum("value");
    assert_eq!(footer_cells(&grid, 0)[0], "Sum: 10");

    let grid = DataGrid::new()
        .column(GridColumn::new("value", "Value"))
        .row(GridRow::new().cell("value", "n/a"))
        .add_sum("value");
    assert_eq!(footer_cells(&grid, 0)[0], "Sum: —");
}
