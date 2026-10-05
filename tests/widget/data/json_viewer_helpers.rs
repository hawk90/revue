//! JsonViewer visible-node and line-number tests
//!
//! The visible-node list (`flatten_tree`) and the gutter width
//! (`line_number_width`) are private helpers. These tests drive them
//! through `visible_count()`, `selected_path()`, collapse/expand and the
//! rendered output.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::data::json_viewer::JsonViewer;
use revue::widget::traits::{RenderContext, View};

fn render(viewer: &JsonViewer, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let area = Rect::new(0, 0, width, height);
    let mut ctx = RenderContext::new(&mut buffer, area);
    viewer.render(&mut ctx);
    buffer
}

fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).unwrap().symbol)
        .collect::<String>()
        .trim_end()
        .to_string()
}

/// Paths of every visible line, top to bottom
fn visible_paths(viewer: &mut JsonViewer) -> Vec<String> {
    viewer.select_first();
    let mut paths = Vec::new();
    for _ in 0..viewer.visible_count() {
        paths.push(viewer.selected_path().unwrap());
        viewer.select_down();
    }
    paths
}

/// A JSON array with `n` elements renders as n + 1 lines
fn array_of(n: usize) -> String {
    format!("[{}]", vec!["0"; n].join(","))
}

// =========================================================================
// Visible nodes
// =========================================================================

#[test]
fn test_visible_scalar_root() {
    let viewer = JsonViewer::from_content("null");
    assert_eq!(viewer.visible_count(), 1);
}

#[test]
fn test_visible_with_children() {
    let viewer = JsonViewer::from_content(r#"{"child1": "value1", "child2": 42}"#);
    assert_eq!(viewer.visible_count(), 3); // root + 2 children
}

#[test]
fn test_visible_collapsed_root() {
    let mut viewer = JsonViewer::from_content("[true]");
    assert_eq!(viewer.visible_count(), 2);

    viewer.collapse();
    assert!(viewer.is_collapsed("$"));
    assert_eq!(viewer.visible_count(), 1); // Only the root, children hidden
}

#[test]
fn test_visible_partial_collapse() {
    let mut viewer = JsonViewer::from_content(r#"{"child": {"grandchild": "value"}}"#);
    assert_eq!(viewer.visible_count(), 3);

    viewer.select_down();
    assert_eq!(viewer.selected_path().as_deref(), Some("$.child"));
    viewer.collapse();
    assert_eq!(viewer.visible_count(), 2); // root + child, grandchild hidden
    assert!(!viewer.is_collapsed("$"));
}

#[test]
fn test_visible_order_is_depth_first() {
    let mut viewer = JsonViewer::from_content(r#"{"a": {"x": 1}, "b": 2}"#);
    assert_eq!(visible_paths(&mut viewer), ["$", "$.a", "$.a.x", "$.b"]);
}

#[test]
fn test_visible_deep_nesting_indents_each_level() {
    let viewer = JsonViewer::from_content(r#"{"a": {"b": {"leaf": "value"}}}"#)
        .show_line_numbers(false)
        .indent_size(2);
    assert_eq!(viewer.visible_count(), 4);

    let buffer = render(&viewer, 40, 6);
    assert_eq!(row(&buffer, 0), "▼ {");
    assert_eq!(row(&buffer, 1), "  ▼ \"a\": {");
    assert_eq!(row(&buffer, 2), "    ▼ \"b\": {");
    assert_eq!(row(&buffer, 3), "      \"leaf\": \"value\"");
}

// =========================================================================
// Line-number gutter
// =========================================================================

/// Column where the first line's content starts
fn content_start(viewer: &JsonViewer) -> u16 {
    let buffer = render(viewer, 40, 3);
    (0..40)
        .find(|&x| buffer.get(x, 0).unwrap().symbol == '▼')
        .unwrap()
}

#[test]
fn test_line_numbers_hidden() {
    let viewer = JsonViewer::from_content(&array_of(99)).show_line_numbers(false);
    assert_eq!(content_start(&viewer), 0);
}

#[test]
fn test_line_numbers_minimum_two_digits() {
    // 1 and 9 lines both get a two-digit gutter plus a space
    let viewer = JsonViewer::from_content("[]");
    assert_eq!(viewer.visible_count(), 1);
    assert_eq!(content_start(&viewer), 3);

    let viewer = JsonViewer::from_content(&array_of(4));
    assert_eq!(viewer.visible_count(), 5);
    assert_eq!(content_start(&viewer), 3);
}

#[test]
fn test_line_numbers_two_digits() {
    let viewer = JsonViewer::from_content(&array_of(9)); // 10 lines
    assert_eq!(content_start(&viewer), 3);
}

#[test]
fn test_line_numbers_three_digits() {
    let viewer = JsonViewer::from_content(&array_of(99)); // 100 lines
    assert_eq!(content_start(&viewer), 4);
    assert_eq!(&row(&render(&viewer, 40, 3), 0)[..4], "  1 ");
}

#[test]
fn test_line_numbers_four_digits() {
    let viewer = JsonViewer::from_content(&array_of(999)); // 1000 lines
    assert_eq!(content_start(&viewer), 5);
}
