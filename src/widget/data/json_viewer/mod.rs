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
    //! JSON Viewer widget tests that access private fields
    //!
    //! Tests using only public APIs are in:
    //! /Users/hawk/Workspaces/revue/tests/widget/data/json_viewer.rs

    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::style::Color;
    use crate::widget::traits::{RenderContext, View};

    // KEEP HERE: accesses private fields (visible_count, selected_index)
    #[test]
    fn test_json_viewer_new() {
        let viewer = JsonViewer::new();
        assert!(!viewer.has_data());
        assert_eq!(viewer.visible_count(), 0);
        assert_eq!(viewer.selected_index(), 0);
    }

    // EXTRACTED: tests/widget/data/json_viewer.rs

    // KEEP HERE: accesses private fields (selected_index)
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

    // KEEP HERE: accesses private fields (selected_index, page_down, page_up)
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

    // KEEP HERE: accesses private fields (visible_count)
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

    // EXTRACTED: tests/widget/data/json_viewer.rs

    // KEEP HERE: accesses private fields (get_indent_size)
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

    // EXTRACTED: tests/widget/data/json_viewer.rs

    // KEEP HERE: accesses private fields (node.children)
    #[test]
    fn test_json_node_child_count() {
        let mut node = JsonNode::new("", "$", JsonType::Object, 0);
        assert_eq!(node.child_count(), 0);

        node.children
            .push(JsonNode::new("a", "$.a", JsonType::String, 1));
        assert_eq!(node.child_count(), 1);
    }

    // EXTRACTED: tests/widget/data/json_viewer.rs

    // KEEP HERE: accesses private types (RenderContext, Buffer, Rect)
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

// Test module requires private field access - keeping inline
