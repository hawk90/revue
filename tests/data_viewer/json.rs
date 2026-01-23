//! JSON Viewer tests

#![allow(unused_imports)]

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, View};
use revue::widget::Search;
use revue::widget::{csv_viewer, json_viewer, CsvViewer, Delimiter, JsonType, JsonViewer};

#[test]
fn test_json_viewer_new() {
    let viewer = JsonViewer::new();
    assert!(!viewer.has_data());
}

#[test]
fn test_json_viewer_parse_object() {
    let json = r#"{"name": "Alice", "age": 30}"#;
    let viewer = JsonViewer::from_content(json);

    assert!(viewer.has_data());
    assert_eq!(viewer.root_type(), Some(&JsonType::Object));
    assert_eq!(viewer.root_children_count(), 2);
}

#[test]
fn test_json_viewer_parse_array() {
    let json = r#"[1, 2, 3, "four"]"#;
    let viewer = JsonViewer::from_content(json);

    assert!(viewer.has_data());
    assert_eq!(viewer.root_type(), Some(&JsonType::Array));
    assert_eq!(viewer.root_children_count(), 4);
}

#[test]
fn test_json_viewer_parse_nested() {
    let json = r#"{"user": {"name": "Bob", "scores": [100, 95, 87]}}"#;
    let viewer = JsonViewer::from_content(json);

    assert!(viewer.has_data());
    assert!(viewer.visible_count() > 5);
}

#[test]
fn test_json_viewer_navigation() {
    let json = r#"{"a": 1, "b": 2, "c": 3}"#;
    let mut viewer = JsonViewer::from_content(json);

    assert_eq!(viewer.selected_index(), 0);
    viewer.select_down();
    assert_eq!(viewer.selected_index(), 1);
    viewer.select_up();
    assert_eq!(viewer.selected_index(), 0);
}

#[test]
fn test_json_viewer_expand_collapse() {
    let json = r#"{"items": [1, 2, 3]}"#;
    let mut viewer = JsonViewer::from_content(json);

    // Initially expanded
    let initial_count = viewer.visible_count();

    // Collapse root
    viewer.toggle();
    let collapsed_count = viewer.visible_count();
    assert!(collapsed_count < initial_count);

    // Expand again
    viewer.toggle();
    let expanded_count = viewer.visible_count();
    assert_eq!(expanded_count, initial_count);
}

#[test]
fn test_json_viewer_expand_collapse_all() {
    let json = r#"{"a": {"b": [1, 2]}, "c": {"d": 3}}"#;
    let mut viewer = JsonViewer::from_content(json);

    viewer.collapse_all();
    let collapsed_count = viewer.visible_count();

    viewer.expand_all();
    let expanded_count = viewer.visible_count();

    assert!(expanded_count > collapsed_count);
}

#[test]
fn test_json_viewer_search() {
    let json = r#"{"name": "Alice", "city": "NYC", "friend": "Alice too"}"#;
    let mut viewer = JsonViewer::from_content(json);

    viewer.search("alice");
    assert_eq!(viewer.match_count(), 2);
    assert!(viewer.is_searching());

    viewer.next_match();
    viewer.prev_match();

    viewer.clear_search();
    assert_eq!(viewer.match_count(), 0);
    assert!(!viewer.is_searching());
}

#[test]
fn test_json_viewer_render() {
    let mut buffer = Buffer::new(60, 20);
    let area = Rect::new(0, 0, 60, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let json = r#"{"name": "Test", "count": 42}"#;
    let viewer = JsonViewer::from_content(json);
    viewer.render(&mut ctx);
}

#[test]
fn test_json_viewer_helper() {
    let viewer = json_viewer().show_line_numbers(true).indent_size(4);
    assert_eq!(viewer.get_indent_size(), 4);
}

#[test]
fn test_json_viewer_default() {
    let viewer = JsonViewer::default();
    assert!(!viewer.has_data());
}

#[test]
fn test_json_type_parsing() {
    // String
    let json = r#""hello""#;
    let viewer = JsonViewer::from_content(json);
    assert_eq!(viewer.root_type(), Some(&JsonType::String));

    // Number
    let json = "42.5";
    let viewer = JsonViewer::from_content(json);
    assert_eq!(viewer.root_type(), Some(&JsonType::Number));

    // Boolean
    let json = "true";
    let viewer = JsonViewer::from_content(json);
    assert_eq!(viewer.root_type(), Some(&JsonType::Boolean));

    // Null
    let json = "null";
    let viewer = JsonViewer::from_content(json);
    assert_eq!(viewer.root_type(), Some(&JsonType::Null));
}

#[test]
fn test_json_viewer_styling() {
    let viewer = JsonViewer::new()
        .key_color(Color::CYAN)
        .string_color(Color::GREEN)
        .number_color(Color::YELLOW)
        .bool_color(Color::MAGENTA)
        .null_color(Color::RED)
        .selected_style(Color::WHITE, Color::BLUE)
        .match_style(Color::BLACK, Color::YELLOW)
        .fg(Color::WHITE)
        .bg(Color::BLACK);

    // Verify widget was created
    assert!(!viewer.has_data());
}

#[test]
fn test_json_viewer_page_navigation() {
    let json = r#"{"a": 1, "b": 2, "c": 3, "d": 4, "e": 5}"#;
    let mut viewer = JsonViewer::from_content(json);

    viewer.page_down(3);
    assert_eq!(viewer.selected_index(), 3);

    viewer.page_up(2);
    assert_eq!(viewer.selected_index(), 1);

    viewer.select_first();
    assert_eq!(viewer.selected_index(), 0);

    viewer.select_last();
    let max = viewer.visible_count() - 1;
    assert_eq!(viewer.selected_index(), max);
}

#[test]
fn test_json_viewer_selected_path() {
    let json = r#"{"name": "Test"}"#;
    let viewer = JsonViewer::from_content(json);

    assert!(viewer.selected_path().is_some());
}

#[test]
fn test_json_viewer_is_collapsed() {
    let json = r#"{"items": [1, 2, 3]}"#;
    let mut viewer = JsonViewer::from_content(json);

    assert!(!viewer.is_collapsed("$"));
    viewer.collapse();
    assert!(viewer.is_collapsed("$"));
}

#[test]
fn test_json_viewer_json_builder() {
    let viewer = JsonViewer::new().json(r#"{"x": 1}"#);
    assert!(viewer.has_data());
}

#[test]
fn test_json_viewer_empty_containers() {
    let json = r#"{"empty_obj": {}, "empty_arr": []}"#;
    let viewer = JsonViewer::from_content(json);

    assert_eq!(viewer.root_children_count(), 2);
}

#[test]
fn test_json_viewer_escaped_strings() {
    let json = r#"{"text": "Hello\nWorld\t\"quoted\""}"#;
    let viewer = JsonViewer::from_content(json);

    assert!(viewer.has_data());
}

#[test]
fn test_json_viewer_negative_numbers() {
    let json = r#"{"value": -42.5}"#;
    let viewer = JsonViewer::from_content(json);

    assert_eq!(viewer.root_children_count(), 1);
}

#[test]
fn test_json_viewer_expand_single() {
    let json = r#"{"items": [1, 2, 3]}"#;
    let mut viewer = JsonViewer::from_content(json);

    viewer.collapse();
    assert!(viewer.is_collapsed("$"));

    viewer.expand();
    assert!(!viewer.is_collapsed("$"));
}

#[test]
fn test_json_viewer_show_type_badges() {
    let viewer = JsonViewer::new().show_type_badges(true);
    assert!(!viewer.has_data());
}
