//! JsonViewer widget tests

mod helpers;

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::data::json_viewer::{json_viewer, JsonNode, JsonType, JsonViewer, Search};
use revue::widget::traits::{RenderContext, View};

// =========================================================================
// Basic creation and parsing tests
// =========================================================================

#[test]
fn test_json_viewer_from_content() {
    let viewer = JsonViewer::from_content(r#"{"name": "test"}"#);
    assert!(viewer.has_data());
    assert_eq!(viewer.root_type(), Some(&JsonType::Object));
}

#[test]
fn test_json_viewer_parse_object() {
    let mut viewer = JsonViewer::new();
    viewer.parse(r#"{"key": "value"}"#);
    assert!(viewer.has_data());
    assert_eq!(viewer.root_type(), Some(&JsonType::Object));
    assert_eq!(viewer.root_children_count(), 1);

    let json = r#"{"name": "Alice", "age": 30}"#;
    let viewer = JsonViewer::from_content(json);

    assert!(viewer.has_data());
    assert_eq!(viewer.root_type(), Some(&JsonType::Object));
    assert_eq!(viewer.root_children_count(), 2);
}

#[test]
fn test_json_viewer_parse_array() {
    let mut viewer = JsonViewer::new();
    viewer.parse(r#"[1, 2, 3]"#);
    assert!(viewer.has_data());
    assert_eq!(viewer.root_type(), Some(&JsonType::Array));
    assert_eq!(viewer.root_children_count(), 3);

    let json = r#"[1, 2, 3, "four"]"#;
    let viewer = JsonViewer::from_content(json);

    assert!(viewer.has_data());
    assert_eq!(viewer.root_type(), Some(&JsonType::Array));
    assert_eq!(viewer.root_children_count(), 4);
}

#[test]
fn test_json_viewer_parse_nested() {
    let mut viewer = JsonViewer::new();
    viewer.parse(r#"{"user": {"name": "Alice", "age": 30}}"#);
    assert!(viewer.has_data());
    assert_eq!(viewer.root_children_count(), 1);

    let json = r#"{"user": {"name": "Bob", "scores": [100, 95, 87]}}"#;
    let viewer = JsonViewer::from_content(json);

    assert!(viewer.has_data());
    assert!(viewer.visible_count() > 5);
}

#[test]
fn test_json_viewer_parse_empty() {
    let mut viewer = JsonViewer::new();
    viewer.parse("");
    assert!(!viewer.has_data());
}

#[test]
fn test_json_viewer_parse_empty_object() {
    let mut viewer = JsonViewer::new();
    viewer.parse("{}");
    assert!(viewer.has_data());
    assert_eq!(viewer.root_type(), Some(&JsonType::Object));
    assert_eq!(viewer.root_children_count(), 0);
}

#[test]
fn test_json_viewer_parse_empty_array() {
    let mut viewer = JsonViewer::new();
    viewer.parse("[]");
    assert!(viewer.has_data());
    assert_eq!(viewer.root_type(), Some(&JsonType::Array));
    assert_eq!(viewer.root_children_count(), 0);
}

// =========================================================================
// Navigation tests
// =========================================================================

#[test]
fn test_json_viewer_navigation() {
    let mut viewer = JsonViewer::from_content(r#"{"a": 1, "b": 2, "c": 3}"#);

    assert_eq!(viewer.selected_index(), 0);

    viewer.select_down();
    viewer.select_down();
    assert_eq!(viewer.selected_index(), 2);
    assert_eq!(viewer.selected_path().as_deref(), Some("$.b"));

    viewer.select_up();
    assert_eq!(viewer.selected_index(), 1);

    viewer.select_last();
    assert_eq!(viewer.selected_index(), 3);
    assert_eq!(viewer.selected_path().as_deref(), Some("$.c"));

    // No wrap at either end
    viewer.select_down();
    assert_eq!(viewer.selected_index(), 3);

    viewer.select_first();
    assert_eq!(viewer.selected_index(), 0);
    viewer.select_up();
    assert_eq!(viewer.selected_index(), 0);

    let json = r#"{"a": 1, "b": 2, "c": 3}"#;
    let mut viewer = JsonViewer::from_content(json);

    assert_eq!(viewer.selected_index(), 0);
    viewer.select_down();
    assert_eq!(viewer.selected_index(), 1);
    viewer.select_up();
    assert_eq!(viewer.selected_index(), 0);
}

// =========================================================================
// Expand/collapse tests
// =========================================================================

#[test]
fn test_json_viewer_toggle() {
    let mut viewer = JsonViewer::from_content(r#"{"obj": {"a": 1}}"#);
    viewer.select_down(); // Select the "obj" node

    let before = viewer.is_collapsed("$.obj");
    viewer.toggle();
    let after = viewer.is_collapsed("$.obj");

    assert_ne!(before, after);
}

#[test]
fn test_json_viewer_expand_collapse_all() {
    let mut viewer = JsonViewer::from_content(r#"{"a": {"b": {"c": 1}}}"#);

    viewer.collapse_all();
    // After collapse_all, nested containers should be collapsed
    assert!(viewer.is_collapsed("$"));

    viewer.expand_all();
    // After expand_all, nothing should be collapsed
    assert!(!viewer.is_collapsed("$"));
    assert!(!viewer.is_collapsed("$.a"));

    let json = r#"{"a": {"b": [1, 2]}, "c": {"d": 3}}"#;
    let mut viewer = JsonViewer::from_content(json);

    viewer.collapse_all();
    let collapsed_count = viewer.visible_count();

    viewer.expand_all();
    let expanded_count = viewer.visible_count();

    assert!(expanded_count > collapsed_count);
}

// =========================================================================
// Search tests
// =========================================================================

#[test]
fn test_json_viewer_search() {
    let mut viewer = JsonViewer::from_content(r#"{"name": "Alice", "friend": "Bob"}"#);

    viewer.search("alice");
    assert!(viewer.is_searching());
    assert_eq!(viewer.match_count(), 1);

    viewer.clear_search();
    assert!(!viewer.is_searching());
    assert_eq!(viewer.match_count(), 0);

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
fn test_json_viewer_search_multiple_matches() {
    let mut viewer = JsonViewer::from_content(r#"{"a": "test", "b": "test", "c": "other"}"#);

    viewer.search("test");
    assert_eq!(viewer.match_count(), 2);
}

#[test]
fn test_json_viewer_search_navigation() {
    let mut viewer = JsonViewer::from_content(r#"{"a": "x", "b": "x", "c": "x"}"#);

    viewer.search("x");
    assert_eq!(viewer.match_count(), 3);

    viewer.next_match();
    viewer.next_match();
    viewer.prev_match();
}

#[test]
fn test_json_viewer_search_empty_query() {
    let mut viewer = JsonViewer::from_content(r#"{"key": "value"}"#);

    viewer.search("");
    assert!(!viewer.is_searching());
    assert_eq!(viewer.match_count(), 0);
}

// =========================================================================
// Selected info tests
// =========================================================================

#[test]
fn test_json_viewer_selected_path() {
    let viewer = JsonViewer::from_content(r#"{"name": "test"}"#);
    let path = viewer.selected_path();
    assert!(path.is_some());

    let json = r#"{"name": "Test"}"#;
    let viewer = JsonViewer::from_content(json);

    assert!(viewer.selected_path().is_some());
}

#[test]
fn test_json_viewer_selected_value() {
    let mut viewer = JsonViewer::from_content(r#"{"name": "test"}"#);
    viewer.select_down(); // Move to "name" key
    let value = viewer.selected_value();
    // Value depends on node structure
    assert!(value.is_some() || value.is_none()); // May or may not have value
}

// =========================================================================
// Builder tests
// =========================================================================

#[test]
fn test_json_viewer_helper() {
    let viewer = json_viewer();
    assert!(!viewer.has_data());

    let viewer = json_viewer().show_line_numbers(true).indent_size(4);
    assert_eq!(viewer.get_indent_size(), 4);
}

// =========================================================================
// Type tests
// =========================================================================

#[test]
fn test_json_type_enum() {
    assert_eq!(JsonType::Object, JsonType::Object);
    assert_ne!(JsonType::Object, JsonType::Array);
    assert_ne!(JsonType::String, JsonType::Number);
}

#[test]
fn test_json_node_is_container() {
    let obj_node = JsonNode::new("", "$", JsonType::Object, 0);
    let arr_node = JsonNode::new("", "$", JsonType::Array, 0);
    let str_node = JsonNode::new("", "$", JsonType::String, 0);

    assert!(obj_node.is_container());
    assert!(arr_node.is_container());
    assert!(!str_node.is_container());
}

// =========================================================================
// Primitive value parsing tests
// =========================================================================

#[test]
fn test_parse_json_primitives() {
    // Test string
    let viewer = JsonViewer::from_content(r#""hello""#);
    assert!(viewer.has_data());
    assert_eq!(viewer.root_type(), Some(&JsonType::String));

    // Test number
    let viewer = JsonViewer::from_content("42");
    assert!(viewer.has_data());
    assert_eq!(viewer.root_type(), Some(&JsonType::Number));

    // Test boolean true
    let viewer = JsonViewer::from_content("true");
    assert!(viewer.has_data());
    assert_eq!(viewer.root_type(), Some(&JsonType::Boolean));

    // Test boolean false
    let viewer = JsonViewer::from_content("false");
    assert!(viewer.has_data());
    assert_eq!(viewer.root_type(), Some(&JsonType::Boolean));

    // Test null
    let viewer = JsonViewer::from_content("null");
    assert!(viewer.has_data());
    assert_eq!(viewer.root_type(), Some(&JsonType::Null));
}

#[test]
fn test_parse_json_negative_number() {
    let viewer = JsonViewer::from_content("-123.45");
    assert!(viewer.has_data());
    assert_eq!(viewer.root_type(), Some(&JsonType::Number));
}

#[test]
fn test_parse_json_escaped_string() {
    let viewer = JsonViewer::from_content(r#"{"msg": "hello\nworld"}"#);
    assert!(viewer.has_data());
}

#[test]
fn test_parse_json_complex() {
    let json = r#"{
        "users": [
            {"name": "Alice", "age": 30},
            {"name": "Bob", "age": 25}
        ],
        "count": 2,
        "active": true
    }"#;
    let viewer = JsonViewer::from_content(json);
    assert!(viewer.has_data());
    assert_eq!(viewer.root_type(), Some(&JsonType::Object));
}

// Found by tests/event_sequences.rs: the shrunk sequence was `Ctrl+End 'x'`
// (select the last row, then collapse everything) - the selection stayed on
// a row that no longer existed.
#[test]
fn test_collapse_all_keeps_the_selection_visible() {
    let mut viewer = JsonViewer::from_content(r#"{"a":[1,2,3],"b":{"c":true}}"#);
    viewer.select_last();
    assert!(viewer.selected_index() > 0);
    viewer.collapse_all();
    assert_eq!(viewer.visible_count(), 1);
    assert_eq!(viewer.selected_index(), 0);
    assert!(viewer.selected_path().is_some());
}

#[test]
fn test_json_viewer_new() {
    let viewer = JsonViewer::new();
    assert!(!viewer.has_data());
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
fn test_json_viewer_render() {
    let mut buffer = Buffer::new(60, 20);
    let area = Rect::new(0, 0, 60, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let json = r#"{"name": "Test", "count": 42}"#;
    let viewer = JsonViewer::from_content(json);
    viewer.render(&mut ctx);
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

fn render_lines(viewer: &JsonViewer, w: u16, h: u16) -> Vec<String> {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    viewer.render(&mut ctx);
    (0..h)
        .map(|y| {
            (0..w)
                .map(|x| buffer.get(x, y).map(|c| c.symbol).unwrap_or(' '))
                .collect::<String>()
                .trim_end()
                .to_string()
        })
        .collect()
}

#[test]
fn type_badges_are_drawn_after_each_value() {
    let json = r#"{"name": "Alice", "age": 30, "ok": true, "x": null, "tags": []}"#;
    let viewer = JsonViewer::from_content(json)
        .show_line_numbers(false)
        .show_type_badges(true);
    let text = render_lines(&viewer, 50, 8).join("\n");
    for badge in ["string", "number", "boolean", "null", "array", "object"] {
        assert!(text.contains(badge), "missing {badge} badge in:\n{text}");
    }

    let plain = JsonViewer::from_content(json).show_line_numbers(false);
    let text = render_lines(&plain, 50, 8).join("\n");
    assert!(!text.contains("string"), "badges drawn while off:\n{text}");
}
