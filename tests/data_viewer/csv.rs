//! CSV Viewer tests

#![allow(unused_imports)]

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{csv_viewer, json_viewer, CsvViewer, Delimiter, JsonType, JsonViewer};

#[test]
fn test_csv_viewer_new() {
    let viewer = CsvViewer::new();
    assert_eq!(viewer.row_count(), 0);
    assert_eq!(viewer.column_count(), 0);
}

#[test]
fn test_csv_viewer_from_str() {
    let csv = "Name,Age,City\nAlice,30,NYC\nBob,25,LA";
    let viewer = CsvViewer::from_content(csv);

    assert_eq!(viewer.row_count(), 2);
    assert_eq!(viewer.column_count(), 3);
    assert_eq!(viewer.get_header(0), Some("Name"));
    assert_eq!(viewer.get_cell(0, 0), Some("Alice"));
    assert_eq!(viewer.get_cell(1, 1), Some("25"));
}

#[test]
fn test_csv_viewer_no_header() {
    let csv = "Alice,30,NYC\nBob,25,LA";
    let mut viewer = CsvViewer::new().has_header(false);
    viewer.parse(csv);

    assert_eq!(viewer.row_count(), 2);
    assert_eq!(viewer.get_cell(0, 0), Some("Alice"));
}

#[test]
fn test_csv_viewer_navigation() {
    let csv = "A,B\n1,2\n3,4\n5,6";
    let mut viewer = CsvViewer::from_content(csv);

    assert_eq!(viewer.selected_row(), 0);
    viewer.select_down();
    assert_eq!(viewer.selected_row(), 1);
    viewer.select_down();
    assert_eq!(viewer.selected_row(), 2);
    viewer.select_up();
    assert_eq!(viewer.selected_row(), 1);

    viewer.select_first_row();
    assert_eq!(viewer.selected_row(), 0);
    viewer.select_last_row();
    assert_eq!(viewer.selected_row(), 2);
}

#[test]
fn test_csv_viewer_column_navigation() {
    let csv = "A,B,C\n1,2,3";
    let mut viewer = CsvViewer::from_content(csv);

    assert_eq!(viewer.selected_col(), 0);
    viewer.select_right();
    assert_eq!(viewer.selected_col(), 1);
    viewer.select_right();
    assert_eq!(viewer.selected_col(), 2);
    viewer.select_right(); // Should stay at max
    assert_eq!(viewer.selected_col(), 2);
    viewer.select_left();
    assert_eq!(viewer.selected_col(), 1);
}

#[test]
fn test_csv_viewer_sorting() {
    let csv = "Name,Value\nBob,20\nAlice,10\nCharlie,30";
    let mut viewer = CsvViewer::from_content(csv);

    viewer.sort_by(0); // Sort by Name ascending
                       // We can verify sorting works by checking selected value after sorting

    viewer.sort_by(0); // Toggle to descending
    viewer.sort_by(0); // Toggle to none
    viewer.reset_sort();
}

#[test]
fn test_csv_viewer_search() {
    let csv = "Name,City\nAlice,NYC\nBob,LA\nAlice,Chicago";
    let mut viewer = CsvViewer::from_content(csv);

    viewer.search("alice");
    assert_eq!(viewer.match_count(), 2);
    assert!(viewer.is_searching());

    viewer.next_match();
    viewer.next_match();
    viewer.prev_match();

    viewer.clear_search();
    assert_eq!(viewer.match_count(), 0);
    assert!(!viewer.is_searching());
}

#[test]
fn test_csv_viewer_delimiter_detection() {
    // Tab-separated
    let tsv = "A\tB\tC\n1\t2\t3";
    let viewer = CsvViewer::from_content(tsv);
    assert_eq!(viewer.column_count(), 3);

    // Semicolon-separated
    let ssv = "A;B;C\n1;2;3";
    let viewer = CsvViewer::from_content(ssv);
    assert_eq!(viewer.column_count(), 3);
}

#[test]
fn test_csv_viewer_quoted_fields() {
    let csv = r#"Name,Description
"Alice","She said ""Hello"""
Bob,"Simple, text""#;
    let viewer = CsvViewer::from_content(csv);

    assert_eq!(viewer.row_count(), 2);
    assert_eq!(viewer.get_cell(0, 1), Some(r#"She said "Hello""#));
}

#[test]
fn test_csv_viewer_render() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let csv = "Name,Age\nAlice,30\nBob,25";
    let viewer = CsvViewer::from_content(csv);
    viewer.render(&mut ctx);
}

#[test]
fn test_csv_viewer_helper() {
    let viewer = csv_viewer().has_header(true).show_row_numbers(true);
    assert_eq!(viewer.row_count(), 0);
}

#[test]
fn test_csv_viewer_default() {
    let viewer = CsvViewer::default();
    assert_eq!(viewer.row_count(), 0);
}

#[test]
fn test_delimiter_enum() {
    assert_eq!(Delimiter::Comma.char(), Some(','));
    assert_eq!(Delimiter::Tab.char(), Some('\t'));
    assert_eq!(Delimiter::Auto.char(), None);
    assert_eq!(Delimiter::Custom('|').char(), Some('|'));
    assert_eq!(Delimiter::Semicolon.char(), Some(';'));
    assert_eq!(Delimiter::Pipe.char(), Some('|'));
}

#[test]
fn test_csv_viewer_page_navigation() {
    let csv = "A\n1\n2\n3\n4\n5\n6\n7\n8\n9\n10";
    let mut viewer = CsvViewer::from_content(csv);

    viewer.page_down(5);
    assert_eq!(viewer.selected_row(), 5);

    viewer.page_up(3);
    assert_eq!(viewer.selected_row(), 2);
}

#[test]
fn test_csv_viewer_selected_value() {
    let csv = "A,B\n1,2\n3,4";
    let mut viewer = CsvViewer::from_content(csv);

    assert_eq!(viewer.selected_value(), Some("1"));
    viewer.select_right();
    assert_eq!(viewer.selected_value(), Some("2"));
    viewer.select_down();
    assert_eq!(viewer.selected_value(), Some("4"));
}

#[test]
fn test_csv_viewer_data_builder() {
    let data = vec![
        vec!["Header1".to_string(), "Header2".to_string()],
        vec!["Value1".to_string(), "Value2".to_string()],
    ];
    let viewer = CsvViewer::new().data(data).has_header(true);

    assert_eq!(viewer.row_count(), 1);
    assert_eq!(viewer.get_header(0), Some("Header1"));
}

#[test]
fn test_csv_viewer_styling() {
    let viewer = CsvViewer::new()
        .header_style(Color::WHITE, Color::BLUE)
        .selected_style(Color::BLACK, Color::YELLOW)
        .match_style(Color::WHITE, Color::RED)
        .fg(Color::WHITE)
        .bg(Color::BLACK);

    // Verify the viewer was created successfully
    assert_eq!(viewer.row_count(), 0);
}

#[test]
fn test_csv_viewer_delimiter_builder() {
    let viewer = CsvViewer::new()
        .delimiter(Delimiter::Tab)
        .show_separators(false);

    assert_eq!(viewer.row_count(), 0);
}
