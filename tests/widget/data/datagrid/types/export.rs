//! DataGrid export options tests

use revue::widget::data::datagrid::{ExportFormat, ExportOptions};

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
