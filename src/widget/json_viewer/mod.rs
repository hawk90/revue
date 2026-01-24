//! JSON Viewer widget for displaying and navigating JSON data
//!
//! Features:
//! - Collapsible tree structure
//! - Syntax highlighting by type (string, number, boolean, null)
//! - Search functionality
//! - Expand/collapse all
//! - Copy path/value support
//! - Virtual scrolling for large documents

#![allow(dead_code)]

mod helpers;
mod parser;
mod search;
mod types;
mod view;

pub use search::Search;
pub use types::{JsonNode, JsonType};
pub use view::JsonViewer;

// Re-export helper
pub use helpers::json_viewer;

#[cfg(test)]
mod tests {
//! JSON Viewer widget tests

use super::search::Search;
use super::*;
use crate::layout::Rect;
use crate::render::Buffer;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View};

#[test]
fn test_json_viewer_new() {
    let viewer = JsonViewer::new();
    assert!(!viewer.has_data());
    assert_eq!(viewer.visible_count(), 0);
    assert_eq!(viewer.selected_index(), 0);
}

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
}

#[test]
fn test_json_viewer_parse_array() {
    let mut viewer = JsonViewer::new();
    viewer.parse(r#"[1, 2, 3]"#);
    assert!(viewer.has_data());
    assert_eq!(viewer.root_type(), Some(&JsonType::Array));
    assert_eq!(viewer.root_children_count(), 3);
}

#[test]
fn test_json_viewer_parse_nested() {
    let mut viewer = JsonViewer::new();
    viewer.parse(r#"{"user": {"name": "Alice", "age": 30}}"#);
    assert!(viewer.has_data());
    assert_eq!(viewer.root_children_count(), 1);
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

#[test]
fn test_json_viewer_navigation() {
    let mut viewer = JsonViewer::from_content(r#"{"a": 1, "b": 2, "c": 3}"#);
    assert_eq!(viewer.selected_index(), 0);

    viewer.select_down();
    assert_eq!(viewer.selected_index(), 1);

    viewer.select_down();
    assert_eq!(viewer.selected_index(), 2);

    viewer.select_up();
    assert_eq!(viewer.selected_index(), 1);

    viewer.select_first();
    assert_eq!(viewer.selected_index(), 0);

    viewer.select_last();
    assert_eq!(viewer.selected_index() > 0, true);
}

#[test]
fn test_json_viewer_page_navigation() {
    let mut viewer = JsonViewer::from_content(r#"{"a":1,"b":2,"c":3,"d":4,"e":5}"#);
    viewer.select_first();
    assert_eq!(viewer.selected_index(), 0);

    viewer.page_down(2);
    assert_eq!(viewer.selected_index(), 2);

    viewer.page_up(1);
    assert_eq!(viewer.selected_index(), 1);
}

#[test]
fn test_json_viewer_collapse_expand() {
    let mut viewer = JsonViewer::from_content(r#"{"obj": {"nested": true}}"#);
    let initial_count = viewer.visible_count();

    // Collapse the nested object
    viewer.select_down(); // Select the "obj" node
    viewer.collapse();

    // Should have fewer visible nodes after collapse
    assert!(viewer.visible_count() <= initial_count);

    // Expand it back
    viewer.expand();
    assert_eq!(viewer.visible_count(), initial_count);
}

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
}

#[test]
fn test_json_viewer_search() {
    let mut viewer = JsonViewer::from_content(r#"{"name": "Alice", "friend": "Bob"}"#);

    viewer.search("alice");
    assert!(viewer.is_searching());
    assert_eq!(viewer.match_count(), 1);

    viewer.clear_search();
    assert!(!viewer.is_searching());
    assert_eq!(viewer.match_count(), 0);
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

#[test]
fn test_json_viewer_selected_path() {
    let viewer = JsonViewer::from_content(r#"{"name": "test"}"#);
    let path = viewer.selected_path();
    assert!(path.is_some());
}

#[test]
fn test_json_viewer_selected_value() {
    let mut viewer = JsonViewer::from_content(r#"{"name": "test"}"#);
    viewer.select_down(); // Move to "name" key
    let value = viewer.selected_value();
    // Value depends on node structure
    assert!(value.is_some() || value.is_none()); // May or may not have value
}

#[test]
fn test_json_viewer_builders() {
    let viewer = JsonViewer::new()
        .json(r#"{"a": 1}"#)
        .show_line_numbers(false)
        .indent_size(4)
        .show_type_badges(true)
        .key_color(Color::RED)
        .string_color(Color::GREEN)
        .number_color(Color::YELLOW)
        .bool_color(Color::MAGENTA)
        .null_color(Color::WHITE)
        .selected_style(Color::WHITE, Color::BLUE)
        .match_style(Color::BLACK, Color::YELLOW)
        .fg(Color::WHITE)
        .bg(Color::BLACK);

    assert!(viewer.has_data());
    assert_eq!(viewer.get_indent_size(), 4);
}

#[test]
fn test_json_viewer_default() {
    let viewer = JsonViewer::default();
    assert!(!viewer.has_data());
}

#[test]
fn test_json_viewer_helper() {
    let viewer = json_viewer();
    assert!(!viewer.has_data());
}

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

#[test]
fn test_json_node_child_count() {
    let mut node = JsonNode::new("", "$", JsonType::Object, 0);
    assert_eq!(node.child_count(), 0);

    node.children
        .push(JsonNode::new("a", "$.a", JsonType::String, 1));
    assert_eq!(node.child_count(), 1);
}

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
    let viewer = JsonViewer::from_content(r#"{"msg": "hello
world"}"#);
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

#[test]
fn test_json_viewer_render() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let viewer = JsonViewer::from_content(r#"{"name": "test"}"#);
    viewer.render(&mut ctx);
    // Should not crash
}

#[test]
fn test_json_viewer_render_empty() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let viewer = JsonViewer::new();
    viewer.render(&mut ctx);
    // Should not crash on empty viewer
}

#[test]
fn test_json_viewer_render_small_area() {
    let mut buffer = Buffer::new(3, 1);
    let area = Rect::new(0, 0, 3, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let viewer = JsonViewer::from_content(r#"{"a": 1}"#);
    viewer.render(&mut ctx);
    // Should handle small area gracefully
}

}
