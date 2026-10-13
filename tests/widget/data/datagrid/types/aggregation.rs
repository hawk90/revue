//! DataGrid aggregation types tests

use revue::widget::data::datagrid::{AggregationType, ColumnAggregation, FooterRow};

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
fn test_aggregation_type_default() {
    assert_eq!(AggregationType::default(), AggregationType::Sum);
}

#[test]
fn test_aggregation_type_label() {
    assert_eq!(AggregationType::Sum.label(), "Sum");
    assert_eq!(AggregationType::Average.label(), "Avg");
    assert_eq!(AggregationType::Count.label(), "Count");
    assert_eq!(AggregationType::Min.label(), "Min");
    assert_eq!(AggregationType::Max.label(), "Max");
}

#[test]
fn test_aggregation_type_equality() {
    assert_eq!(AggregationType::Sum, AggregationType::Sum);
    assert_ne!(AggregationType::Sum, AggregationType::Average);
}

#[test]
fn test_column_aggregation_new() {
    let agg = ColumnAggregation::new("amount", AggregationType::Sum);
    assert_eq!(agg.column_key, "amount");
    assert_eq!(agg.agg_type, AggregationType::Sum);
    assert!(agg.label.is_none());
}

#[test]
fn test_column_aggregation_label() {
    let agg = ColumnAggregation::new("amount", AggregationType::Sum).label("Total");
    assert_eq!(agg.label, Some("Total".to_string()));
}

#[test]
fn test_footer_row_new() {
    let footer = FooterRow::new("Totals");
    assert_eq!(footer.label, "Totals");
    assert!(footer.aggregations.is_empty());
}

#[test]
fn test_footer_row_default() {
    let footer = FooterRow::default();
    assert!(footer.label.is_empty());
    assert!(footer.aggregations.is_empty());
}

#[test]
fn test_footer_row_aggregation() {
    let agg = ColumnAggregation::new("amount", AggregationType::Sum);
    let footer = FooterRow::new("Totals").aggregation(agg);
    assert_eq!(footer.aggregations.len(), 1);
}

#[test]
fn test_footer_row_sum() {
    let footer = FooterRow::new("Totals").sum("amount");
    assert_eq!(footer.aggregations.len(), 1);
    assert_eq!(footer.aggregations[0].agg_type, AggregationType::Sum);
    assert_eq!(footer.aggregations[0].column_key, "amount");
}

#[test]
fn test_footer_row_average() {
    let footer = FooterRow::new("Averages").average("score");
    assert_eq!(footer.aggregations[0].agg_type, AggregationType::Average);
}

#[test]
fn test_footer_row_count() {
    let footer = FooterRow::new("Count").count("id");
    assert_eq!(footer.aggregations[0].agg_type, AggregationType::Count);
}

#[test]
fn test_footer_row_min() {
    let footer = FooterRow::new("Minimum").min("price");
    assert_eq!(footer.aggregations[0].agg_type, AggregationType::Min);
}

#[test]
fn test_footer_row_max() {
    let footer = FooterRow::new("Maximum").max("price");
    assert_eq!(footer.aggregations[0].agg_type, AggregationType::Max);
}

#[test]
fn test_footer_row_multiple_aggregations() {
    let footer = FooterRow::new("Summary")
        .sum("quantity")
        .average("price")
        .count("id")
        .min("date")
        .max("amount");

    assert_eq!(footer.aggregations.len(), 5);
}
