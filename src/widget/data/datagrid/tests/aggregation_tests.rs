#![allow(unused_imports)]

use super::super::*;

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

    let sum = grid.compute_aggregation("value", AggregationType::Sum);
    assert_eq!(sum, Some(60.0));
}

#[test]
fn test_footer_average() {
    let grid = DataGrid::new()
        .column(GridColumn::new("value", "Value"))
        .row(GridRow::new().cell("value", "10"))
        .row(GridRow::new().cell("value", "20"))
        .row(GridRow::new().cell("value", "30"))
        .add_average("value");

    let avg = grid.compute_aggregation("value", AggregationType::Average);
    assert_eq!(avg, Some(20.0));
}

#[test]
fn test_footer_count() {
    let grid = DataGrid::new()
        .column(GridColumn::new("value", "Value"))
        .row(GridRow::new().cell("value", "10"))
        .row(GridRow::new().cell("value", "20"))
        .row(GridRow::new().cell("value", "30"));

    let count = grid.compute_aggregation("value", AggregationType::Count);
    assert_eq!(count, Some(3.0));
}

#[test]
fn test_footer_min_max() {
    let grid = DataGrid::new()
        .column(GridColumn::new("value", "Value"))
        .row(GridRow::new().cell("value", "5"))
        .row(GridRow::new().cell("value", "15"))
        .row(GridRow::new().cell("value", "10"));

    let min = grid.compute_aggregation("value", AggregationType::Min);
    assert_eq!(min, Some(5.0));

    let max = grid.compute_aggregation("value", AggregationType::Max);
    assert_eq!(max, Some(15.0));
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
        .row(GridRow::new().cell("name", "Cherry").cell("value", "30"));

    // Sum all
    let sum_all = grid.compute_aggregation("value", AggregationType::Sum);
    assert_eq!(sum_all, Some(60.0));

    // Filter to "Ap" items (only Apple matches)
    grid.set_filter("Ap");

    // Sum only filtered items (Apple=10)
    let sum_filtered = grid.compute_aggregation("value", AggregationType::Sum);
    assert_eq!(sum_filtered, Some(10.0));
}

#[test]
fn test_aggregation_non_numeric() {
    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(GridRow::new().cell("name", "Alice"))
        .row(GridRow::new().cell("name", "Bob"));

    // Non-numeric values should return None for sum/avg
    let sum = grid.compute_aggregation("name", AggregationType::Sum);
    assert!(sum.is_none());
}

#[test]
fn test_aggregation_type_labels() {
    assert_eq!(AggregationType::Sum.label(), "Sum");
    assert_eq!(AggregationType::Average.label(), "Avg");
    assert_eq!(AggregationType::Count.label(), "Count");
    assert_eq!(AggregationType::Min.label(), "Min");
    assert_eq!(AggregationType::Max.label(), "Max");
}

#[test]
fn test_footer_values() {
    let grid = DataGrid::new()
        .column(GridColumn::new("value", "Value"))
        .row(GridRow::new().cell("value", "10"))
        .row(GridRow::new().cell("value", "20"))
        .footer(FooterRow::new("Totals").sum("value"));

    let values = grid.get_footer_values(&grid.footer_rows[0]);
    assert_eq!(values.len(), 1);
    assert!(values[0].1.contains("30")); // Sum of 10+20
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

    let values = grid.get_footer_values(&grid.footer_rows[0]);
    assert!(values[0].1.contains("Total"));
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
    let grid = DataGrid::new().column(GridColumn::new("value", "Value"));

    // No rows, should return None
    let sum = grid.compute_aggregation("value", AggregationType::Sum);
    assert!(sum.is_none());
}
