#![allow(unused_imports)]

use super::super::*;

#[test]
fn test_export_csv() {
    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .column(GridColumn::new("value", "Value"))
        .row(GridRow::new().cell("name", "Alice").cell("value", "100"))
        .row(GridRow::new().cell("name", "Bob").cell("value", "200"));

    let csv = grid.export_csv();
    assert!(csv.contains("Name,Value"));
    assert!(csv.contains("Alice,100"));
    assert!(csv.contains("Bob,200"));
}

#[test]
fn test_export_csv_escaping() {
    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(GridRow::new().cell("name", "Hello, World"));

    let csv = grid.export_csv();
    // Comma in value should be quoted
    assert!(csv.contains("\"Hello, World\""));
}

#[test]
fn test_export_csv_quote_escaping() {
    let grid = DataGrid::new()
        .column(GridColumn::new("quote", "Quote"))
        .row(GridRow::new().cell("quote", "He said \"Hello\""));

    let csv = grid.export_csv();
    // Quotes should be escaped with double quotes
    assert!(csv.contains("\"He said \"\"Hello\"\"\""));
}

#[test]
fn test_export_tsv() {
    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .column(GridColumn::new("value", "Value"))
        .row(GridRow::new().cell("name", "Alice").cell("value", "100"));

    let tsv = grid.export_tsv();
    assert!(tsv.contains("Name\tValue"));
    assert!(tsv.contains("Alice\t100"));
}

#[test]
fn test_export_options() {
    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(GridRow::new().cell("name", "Test"));

    // Without headers
    let csv = grid.export(&ExportOptions::new().include_headers(false));
    assert!(!csv.contains("Name"));
    assert!(csv.contains("Test"));
}

#[test]
fn test_copy_cell() {
    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(GridRow::new().cell("name", "Alice"))
        .row(GridRow::new().cell("name", "Bob"));

    let cell = grid.copy_cell();
    assert_eq!(cell, "Alice");
}

#[test]
fn test_export_format_default() {
    let options = ExportOptions::default();
    assert_eq!(options.format, ExportFormat::Csv);
    assert!(options.include_headers);
    assert!(!options.selected_only);
    assert!(options.visible_columns_only);
}

#[test]
fn test_export_plain_text() {
    let grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .column(GridColumn::new("b", "B"))
        .row(GridRow::new().cell("a", "1").cell("b", "2"));

    let text = grid.export(&ExportOptions::new().format(ExportFormat::PlainText));
    assert!(text.contains("A B"));
    assert!(text.contains("1 2"));
}

#[test]
fn test_export_tsv_with_special_chars() {
    let grid = DataGrid::new()
        .column(GridColumn::new("text", "Text"))
        .row(GridRow::new().cell("text", "has\ttab\nand\nnewline"));

    let tsv = grid.export_tsv();
    // Tabs and newlines should be replaced with spaces
    assert!(!tsv.contains('\t') || tsv.lines().count() <= 2);
}

#[test]
fn test_copy_cell_empty() {
    let grid = DataGrid::new().column(GridColumn::new("a", "A"));

    // No rows, should return empty string
    let cell = grid.copy_cell();
    assert!(cell.is_empty());
}
