#![allow(unused_imports)]

use super::super::*;
use super::{footer_cells, render_to, row_text};

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
fn test_footer_row_builder() {
    let footer = FooterRow::new("Totals")
        .sum("price")
        .average("quantity")
        .count("items");

    assert_eq!(footer.label, "Totals");
    assert_eq!(footer.aggregations.len(), 3);
    assert_eq!(footer.aggregations[0].agg_type, AggregationType::Sum);
    assert_eq!(footer.aggregations[1].agg_type, AggregationType::Average);
    assert_eq!(footer.aggregations[2].agg_type, AggregationType::Count);
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
fn test_column_aggregation_builder() {
    let agg = ColumnAggregation::new("price", AggregationType::Sum).label("Total Price");

    assert_eq!(agg.column_key, "price");
    assert_eq!(agg.agg_type, AggregationType::Sum);
    assert_eq!(agg.label, Some("Total Price".to_string()));
}

#[test]
fn test_footer_row_min_max() {
    let footer = FooterRow::new("Stats").min("value").max("value");

    assert_eq!(footer.aggregations.len(), 2);
    assert_eq!(footer.aggregations[0].agg_type, AggregationType::Min);
    assert_eq!(footer.aggregations[1].agg_type, AggregationType::Max);
}

#[test]
fn test_show_footer_toggle() {
    let grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .add_sum("a")
        .show_footer(false);

    assert!(!grid.show_footer);
}

#[test]
fn test_aggregation_empty_data() {
    let grid = DataGrid::new()
        .column(GridColumn::new("value", "Value"))
        .add_sum("value");

    // No rows, should return None
    assert_eq!(footer_cells(&grid, 0)[0], "Sum: —");
}

// --- Rendering: the footer row ---

fn numbered_grid(rows: usize) -> DataGrid {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name").width(10))
        .column(GridColumn::new("value", "Value").width(12));
    for i in 0..rows {
        grid = grid.row(
            GridRow::new()
                .cell("name", format!("row{i}"))
                .cell("value", (i + 1).to_string()),
        );
    }
    grid
}

#[test]
fn footer_row_is_painted_below_the_rows() {
    let grid = numbered_grid(3).add_sum("value");
    let buf = render_to(&grid, 40, 8);

    let footer = row_text(&buf, 7, 40);
    assert!(footer.contains("Total"), "footer label missing: {footer:?}");
    assert!(footer.contains("Sum: 6"), "footer sum missing: {footer:?}");
    // The data rows are still drawn above it.
    assert!(row_text(&buf, 1, 40).contains("row0"));
}

#[test]
fn hidden_footer_is_not_painted() {
    let grid = numbered_grid(3).add_sum("value").show_footer(false);
    let buf = render_to(&grid, 40, 8);
    for y in 0..8 {
        assert!(!row_text(&buf, y, 40).contains("Sum:"));
    }
}

#[test]
fn footer_takes_a_row_from_the_viewport_and_last_row_stays_reachable() {
    // Height 7 = header + 5 data rows + footer.
    let mut grid = numbered_grid(20).add_sum("value");
    render_to(&grid, 40, 7);
    grid.select_last();
    let buf = render_to(&grid, 40, 7);

    let last_data_line = row_text(&buf, 5, 40);
    assert!(
        last_data_line.contains("row19"),
        "last row not visible above the footer: {last_data_line:?}"
    );
    let footer = row_text(&buf, 6, 40);
    assert!(footer.contains("Sum: 210"), "footer missing: {footer:?}");
    assert!(
        !footer.contains("row"),
        "a data row drew over the footer: {footer:?}"
    );
}
