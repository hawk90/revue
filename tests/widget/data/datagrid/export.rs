//! DataGrid export and copy tests

use proptest::prelude::*;
use revue::widget::data::datagrid::{DataGrid, ExportFormat, ExportOptions, GridColumn, GridRow};

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

/// Split CSV text into records, undoing the quoting the export applies
fn parse_csv(text: &str) -> Vec<Vec<String>> {
    let mut records = Vec::new();
    let mut record = Vec::new();
    let mut field = String::new();
    let mut quoted = false;
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match (quoted, c) {
            (true, '"') if chars.peek() == Some(&'"') => {
                field.push('"');
                chars.next();
            }
            (true, '"') => quoted = false,
            (false, '"') => quoted = true,
            (false, ',') => record.push(std::mem::take(&mut field)),
            (false, '\n') => {
                record.push(std::mem::take(&mut field));
                records.push(std::mem::take(&mut record));
            }
            (_, c) => field.push(c),
        }
    }
    records
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// CSV export with headers reads back as the titles and the cell
    /// values, whatever commas, quotes and newlines they hold
    #[test]
    fn csv_export_reads_back(rows in prop::collection::vec(("[a-z,\" \n]{0,6}", "[a-z,\" \n]{0,6}"), 0..12)) {
        let grid = DataGrid::new()
            .column(GridColumn::new("a", "A"))
            .column(GridColumn::new("b", "B"))
            .rows(rows.iter().map(|(a, b)| GridRow::new().cell("a", a.clone()).cell("b", b.clone())).collect());

        let mut expected = vec![vec!["A".to_string(), "B".to_string()]];
        expected.extend(rows.iter().map(|(a, b)| vec![a.clone(), b.clone()]));
        prop_assert_eq!(parse_csv(&grid.export_csv()), expected);
    }
}
