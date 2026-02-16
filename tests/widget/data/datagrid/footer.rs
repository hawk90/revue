//! DataGrid footer functionality tests

use revue::widget::data::datagrid::DataGrid;
use revue::widget::data::datagrid::types::{FooterRow, GridColumn, GridRow};
use revue::widget::data::datagrid::core::AggregationType;

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