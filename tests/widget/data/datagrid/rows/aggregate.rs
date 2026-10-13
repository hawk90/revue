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
