//! Tests for DataGrid types public API
//!
//! Extracted from src/widget/data/datagrid/types/mod.rs

use revue::widget::data::datagrid::{
    GridColors, GridOptions, GridColumn, GridRow,
    ColumnType, SortDirection, Alignment,
    ExportFormat, ExportOptions,
    AggregationType, ColumnAggregation, FooterRow,
};
use revue::style::Color;

// =========================================================================
// GridColors Tests
// =========================================================================

#[test]
fn test_grid_colors_default() {
    let colors = GridColors::default();
    assert_eq!(colors.header_fg, Color::WHITE);
    assert_eq!(colors.selected_fg, Color::WHITE);
}

#[test]
fn test_grid_colors_new() {
    let colors = GridColors::new();
    assert_eq!(colors.header_fg, Color::WHITE);
}

#[test]
fn test_grid_colors_dark() {
    let colors = GridColors::dark();
    assert_eq!(colors.header_fg, Color::WHITE);
}

#[test]
fn test_grid_colors_light() {
    let colors = GridColors::light();
    assert_eq!(colors.header_fg, Color::BLACK);
}

// =========================================================================
// GridOptions Tests
// =========================================================================

#[test]
fn test_grid_options_default() {
    let opts = GridOptions::default();
    assert!(opts.show_header);
    assert!(!opts.show_row_numbers);
    assert!(!opts.multi_select);
    assert!(opts.zebra);
    assert!(opts.use_natural_sort);
    assert!(opts.virtual_scroll);
    assert_eq!(opts.row_height, 1);
    assert_eq!(opts.overscan, 5);
}

#[test]
fn test_grid_options_new() {
    let opts = GridOptions::new();
    assert!(opts.show_header);
}

// =========================================================================
// ColumnType Tests
// =========================================================================

#[test]
fn test_column_type_default() {
    assert_eq!(ColumnType::default(), ColumnType::Text);
}

#[test]
fn test_column_type_variants() {
    assert_eq!(ColumnType::Text, ColumnType::Text);
    assert_eq!(ColumnType::Number, ColumnType::Number);
    assert_eq!(ColumnType::Date, ColumnType::Date);
    assert_eq!(ColumnType::Boolean, ColumnType::Boolean);
    assert_eq!(ColumnType::Custom, ColumnType::Custom);
}

#[test]
fn test_column_type_inequality() {
    assert_ne!(ColumnType::Text, ColumnType::Number);
    assert_ne!(ColumnType::Date, ColumnType::Boolean);
}

// =========================================================================
// SortDirection Tests
// =========================================================================

#[test]
fn test_sort_direction_toggle() {
    let asc = SortDirection::Ascending;
    assert_eq!(asc.toggle(), SortDirection::Descending);

    let desc = SortDirection::Descending;
    assert_eq!(desc.toggle(), SortDirection::Ascending);
}

#[test]
fn test_sort_direction_icon() {
    assert_eq!(SortDirection::Ascending.icon(), '▲');
    assert_eq!(SortDirection::Descending.icon(), '▼');
}

#[test]
fn test_sort_direction_equality() {
    assert_eq!(SortDirection::Ascending, SortDirection::Ascending);
    assert_ne!(SortDirection::Ascending, SortDirection::Descending);
}

// =========================================================================
// Alignment Tests
// =========================================================================

#[test]
fn test_alignment_default() {
    assert_eq!(Alignment::default(), Alignment::Left);
}

#[test]
fn test_alignment_variants() {
    assert_eq!(Alignment::Left, Alignment::Left);
    assert_eq!(Alignment::Center, Alignment::Center);
    assert_eq!(Alignment::Right, Alignment::Right);
}

// =========================================================================
// GridColumn Tests
// =========================================================================

#[test]
fn test_grid_column_new() {
    let col = GridColumn::new("name", "Name");
    assert_eq!(col.key, "name");
    assert_eq!(col.title, "Name");
    assert_eq!(col.col_type, ColumnType::Text);
    assert_eq!(col.width, 0);
    assert_eq!(col.min_width, 5);
    assert_eq!(col.max_width, 50);
    assert!(col.sortable);
    assert!(col.filterable);
    assert!(!col.editable);
    assert!(col.visible);
    assert_eq!(col.align, Alignment::Left);
    assert!(col.resizable);
    assert!(!col.frozen);
}

#[test]
fn test_grid_column_col_type() {
    let col = GridColumn::new("age", "Age").col_type(ColumnType::Number);
    assert_eq!(col.col_type, ColumnType::Number);
}

#[test]
fn test_grid_column_width() {
    let col = GridColumn::new("name", "Name").width(20);
    assert_eq!(col.width, 20);
}

#[test]
fn test_grid_column_min_width() {
    let col = GridColumn::new("name", "Name").min_width(10);
    assert_eq!(col.min_width, 10);
}

#[test]
fn test_grid_column_max_width() {
    let col = GridColumn::new("name", "Name").max_width(100);
    assert_eq!(col.max_width, 100);
}

#[test]
fn test_grid_column_sortable() {
    let col = GridColumn::new("name", "Name").sortable(false);
    assert!(!col.sortable);
}

#[test]
fn test_grid_column_editable() {
    let col = GridColumn::new("name", "Name").editable(true);
    assert!(col.editable);
}

#[test]
fn test_grid_column_align() {
    let col = GridColumn::new("name", "Name").align(Alignment::Center);
    assert_eq!(col.align, Alignment::Center);
}

#[test]
fn test_grid_column_right() {
    let col = GridColumn::new("amount", "Amount").right();
    assert_eq!(col.align, Alignment::Right);
}

#[test]
fn test_grid_column_center() {
    let col = GridColumn::new("status", "Status").center();
    assert_eq!(col.align, Alignment::Center);
}

#[test]
fn test_grid_column_resizable() {
    let col = GridColumn::new("name", "Name").resizable(false);
    assert!(!col.resizable);
}

#[test]
fn test_grid_column_frozen() {
    let col = GridColumn::new("id", "ID").frozen(true);
    assert!(col.frozen);
}

#[test]
fn test_grid_column_builder_chain() {
    let col = GridColumn::new("price", "Price")
        .col_type(ColumnType::Number)
        .width(15)
        .min_width(10)
        .max_width(20)
        .right()
        .sortable(true)
        .editable(true)
        .resizable(true)
        .frozen(false);

    assert_eq!(col.col_type, ColumnType::Number);
    assert_eq!(col.width, 15);
    assert_eq!(col.min_width, 10);
    assert_eq!(col.max_width, 20);
    assert_eq!(col.align, Alignment::Right);
    assert!(col.sortable);
    assert!(col.editable);
    assert!(col.resizable);
    assert!(!col.frozen);
}

// =========================================================================
// GridRow Tests
// =========================================================================

#[test]
fn test_grid_row_new() {
    let row = GridRow::new();
    assert!(row.data.is_empty());
    assert!(!row.selected);
    assert!(!row.expanded);
    assert!(row.children.is_empty());
}

#[test]
fn test_grid_row_default() {
    let row = GridRow::default();
    assert!(row.data.is_empty());
}

#[test]
fn test_grid_row_cell() {
    let row = GridRow::new().cell("name", "Alice").cell("age", "30");
    assert_eq!(row.data.len(), 2);
    assert_eq!(row.get("name"), Some("Alice"));
    assert_eq!(row.get("age"), Some("30"));
}

#[test]
fn test_grid_row_get_not_found() {
    let row = GridRow::new().cell("name", "Alice");
    assert!(row.get("email").is_none());
}

#[test]
fn test_grid_row_child() {
    let parent = GridRow::new()
        .cell("name", "Parent")
        .child(GridRow::new().cell("name", "Child"));
    assert_eq!(parent.children.len(), 1);
    assert!(parent.has_children());
}

#[test]
fn test_grid_row_children() {
    let parent = GridRow::new().cell("name", "Parent").children(vec![
        GridRow::new().cell("name", "Child1"),
        GridRow::new().cell("name", "Child2"),
    ]);
    assert_eq!(parent.children.len(), 2);
}

#[test]
fn test_grid_row_expanded() {
    let row = GridRow::new().expanded(true);
    assert!(row.expanded);
}

#[test]
fn test_grid_row_has_children() {
    let row = GridRow::new();
    assert!(!row.has_children());

    let parent = GridRow::new().child(GridRow::new());
    assert!(parent.has_children());
}

// =========================================================================
// ExportFormat Tests
// =========================================================================

#[test]
fn test_export_format_default() {
    assert_eq!(ExportFormat::default(), ExportFormat::Csv);
}

#[test]
fn test_export_format_variants() {
    assert_eq!(ExportFormat::Csv, ExportFormat::Csv);
    assert_eq!(ExportFormat::Tsv, ExportFormat::Tsv);
    assert_eq!(ExportFormat::PlainText, ExportFormat::PlainText);
}

// =========================================================================
// ExportOptions Tests
// =========================================================================

#[test]
fn test_export_options_default() {
    let opts = ExportOptions::default();
    assert_eq!(opts.format, ExportFormat::Csv);
    assert!(opts.include_headers);
    assert!(!opts.selected_only);
    assert!(opts.visible_columns_only);
}

#[test]
fn test_export_options_new() {
    let opts = ExportOptions::new();
    assert_eq!(opts.format, ExportFormat::Csv);
}

#[test]
fn test_export_options_format() {
    let opts = ExportOptions::new().format(ExportFormat::Tsv);
    assert_eq!(opts.format, ExportFormat::Tsv);
}

#[test]
fn test_export_options_include_headers() {
    let opts = ExportOptions::new().include_headers(false);
    assert!(!opts.include_headers);
}

#[test]
fn test_export_options_selected_only() {
    let opts = ExportOptions::new().selected_only(true);
    assert!(opts.selected_only);
}

#[test]
fn test_export_options_builder_chain() {
    let opts = ExportOptions::new()
        .format(ExportFormat::PlainText)
        .include_headers(false)
        .selected_only(true);

    assert_eq!(opts.format, ExportFormat::PlainText);
    assert!(!opts.include_headers);
    assert!(opts.selected_only);
}

// =========================================================================
// AggregationType Tests
// =========================================================================

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

// =========================================================================
// ColumnAggregation Tests
// =========================================================================

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

// =========================================================================
// FooterRow Tests
// =========================================================================

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
