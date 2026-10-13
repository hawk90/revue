//! DataGrid footer functionality tests

use super::{footer_cells, render_to, row_text};
use revue::widget::data::datagrid::{
    AggregationType, ColumnAggregation, DataGrid, FooterRow, GridColumn, GridRow,
};

#[test]
fn test_footer_adds_footer_row() {
    let grid = DataGrid::new().footer(FooterRow::new("Totals"));

    assert_eq!(grid.footer_rows.len(), 1);
    assert_eq!(grid.footer_rows[0].label, "Totals");
}

#[test]
fn test_footer_enables_show_footer() {
    let grid = DataGrid::new().footer(FooterRow::new("Summary"));

    assert!(grid.show_footer);
}

#[test]
fn test_footer_multiple_footers() {
    let grid = DataGrid::new()
        .footer(FooterRow::new("Totals"))
        .footer(FooterRow::new("Averages"));

    assert_eq!(grid.footer_rows.len(), 2);
    assert!(grid.show_footer);
}

#[test]
fn test_footer_preserves_existing_footers() {
    let grid = DataGrid::new()
        .footer(FooterRow::new("First").sum("a"))
        .footer(FooterRow::new("Second").average("b"));

    assert_eq!(grid.footer_rows.len(), 2);
    assert_eq!(grid.footer_rows[0].label, "First");
    assert_eq!(grid.footer_rows[1].label, "Second");
}

#[test]
fn test_footer_with_aggregations() {
    let footer_row = FooterRow::new("Stats")
        .sum("value")
        .average("score")
        .count("items");

    let grid = DataGrid::new().footer(footer_row);

    assert_eq!(grid.footer_rows[0].aggregations.len(), 3);
}

#[test]
fn test_show_footer_true() {
    let grid = DataGrid::new().show_footer(true);

    assert!(grid.show_footer);
}

#[test]
fn test_show_footer_false() {
    let grid = DataGrid::new().show_footer(false);

    assert!(!grid.show_footer);
}

#[test]
fn test_show_footer_override() {
    let grid = DataGrid::new()
        .footer(FooterRow::new("Totals")) // Enables footer
        .show_footer(false); // Disable it

    assert!(!grid.show_footer);
    assert_eq!(grid.footer_rows.len(), 1); // Footer row still exists
}

#[test]
fn test_show_footer_then_enable() {
    let grid = DataGrid::new().show_footer(false).show_footer(true);

    assert!(grid.show_footer);
}

#[test]
fn test_add_sum_creates_footer_if_empty() {
    let grid = DataGrid::new().add_sum("value");

    assert_eq!(grid.footer_rows.len(), 1);
    assert_eq!(grid.footer_rows[0].label, "Total");
}

#[test]
fn test_add_sum_adds_aggregation() {
    let grid = DataGrid::new().add_sum("price");

    assert_eq!(grid.footer_rows[0].aggregations.len(), 1);
    assert_eq!(grid.footer_rows[0].aggregations[0].column_key, "price");
    assert_eq!(
        grid.footer_rows[0].aggregations[0].agg_type,
        AggregationType::Sum
    );
}

#[test]
fn test_add_sum_enables_footer() {
    let grid = DataGrid::new().add_sum("value");

    assert!(grid.show_footer);
}

#[test]
fn test_add_sum_reuses_existing_footer() {
    let grid = DataGrid::new().add_sum("price").add_sum("quantity");

    assert_eq!(grid.footer_rows.len(), 1);
    assert_eq!(grid.footer_rows[0].aggregations.len(), 2);
}

#[test]
fn test_add_sum_with_string() {
    let grid = DataGrid::new().add_sum(String::from("total"));

    assert_eq!(grid.footer_rows[0].aggregations[0].column_key, "total");
}

#[test]
fn test_add_sum_chainable() {
    let grid = DataGrid::new().add_sum("a").add_sum("b").add_sum("c");

    assert_eq!(grid.footer_rows[0].aggregations.len(), 3);
}

#[test]
fn test_add_average_creates_footer_if_empty() {
    let grid = DataGrid::new().add_average("value");

    assert_eq!(grid.footer_rows.len(), 1);
    assert_eq!(grid.footer_rows[0].label, "Average");
}

#[test]
fn test_add_average_adds_aggregation() {
    let grid = DataGrid::new().add_average("score");

    assert_eq!(grid.footer_rows[0].aggregations.len(), 1);
    assert_eq!(grid.footer_rows[0].aggregations[0].column_key, "score");
    assert_eq!(
        grid.footer_rows[0].aggregations[0].agg_type,
        AggregationType::Average
    );
}

#[test]
fn test_add_average_enables_footer() {
    let grid = DataGrid::new().add_average("value");

    assert!(grid.show_footer);
}

#[test]
fn test_add_average_reuses_existing_footer() {
    let grid = DataGrid::new().add_average("price").add_average("quantity");

    assert_eq!(grid.footer_rows.len(), 1);
    assert_eq!(grid.footer_rows[0].aggregations.len(), 2);
}

#[test]
fn test_add_average_with_string() {
    let grid = DataGrid::new().add_average(String::from("rating"));

    assert_eq!(grid.footer_rows[0].aggregations[0].column_key, "rating");
}

#[test]
fn test_add_average_chainable() {
    let grid = DataGrid::new()
        .add_average("x")
        .add_average("y")
        .add_average("z");

    assert_eq!(grid.footer_rows[0].aggregations.len(), 3);
}

#[test]
fn test_footer_combines_add_sum_and_add_average() {
    let grid = DataGrid::new().add_sum("total").add_average("average");

    assert_eq!(grid.footer_rows.len(), 1);
    assert_eq!(grid.footer_rows[0].aggregations.len(), 2);
    assert_eq!(
        grid.footer_rows[0].aggregations[0].agg_type,
        AggregationType::Sum
    );
    assert_eq!(
        grid.footer_rows[0].aggregations[1].agg_type,
        AggregationType::Average
    );
}

#[test]
fn test_footer_with_explicit_footer_and_add_methods() {
    let grid = DataGrid::new()
        .footer(FooterRow::new("Custom"))
        .add_sum("value");

    // add_sum should reuse the first footer
    assert_eq!(grid.footer_rows.len(), 1);
    assert_eq!(grid.footer_rows[0].label, "Custom");
    assert_eq!(grid.footer_rows[0].aggregations.len(), 1);
}

#[test]
fn test_footer_multiple_aggregations_same_column() {
    let grid = DataGrid::new()
        .add_sum("value")
        .add_average("value")
        .add_average("value");

    assert_eq!(grid.footer_rows[0].aggregations.len(), 3);
}

#[test]
fn test_footer_builder_pattern() {
    let grid = DataGrid::new()
        .column(GridColumn::new("value", "Value"))
        .row(GridRow::new().cell("value", "10"))
        .row(GridRow::new().cell("value", "20"))
        .add_sum("value")
        .show_footer(true);

    assert_eq!(grid.footer_rows.len(), 1);
    assert!(grid.show_footer);
}

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
