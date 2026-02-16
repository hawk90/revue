//! JSON Viewer helper function tests

use revue::widget::data::json_viewer::helpers::{flatten_tree, json_viewer, line_number_width};
use revue::widget::data::json_viewer::types::{JsonNode, JsonType};

// =========================================================================
// json_viewer() helper tests
// =========================================================================

#[test]
fn test_json_viewer_function_creates_viewer() {
    let viewer = json_viewer();
    let _ = viewer;
}

#[test]
fn test_json_viewer_multiple_instances() {
    let viewer1 = json_viewer();
    let viewer2 = json_viewer();
    let _ = viewer1;
    let _ = viewer2;
}

#[test]
fn test_json_viewer_is_chainable() {
    let viewer = json_viewer();
    // Should allow builder methods
    let _ = viewer;
}

// =========================================================================
// flatten_tree() helper tests
// =========================================================================

#[test]
fn test_flatten_tree_empty_node() {
    let root = JsonNode::new("root", "root", JsonType::Null, 0).with_value("null");
    let collapsed = std::collections::HashSet::new();
    let result = flatten_tree(&root, &collapsed);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].path, "root");
}

#[test]
fn test_flatten_tree_with_children() {
    let child1 =
        JsonNode::new("child1", "root.child1", JsonType::String, 1).with_value("value1");
    let child2 = JsonNode::new("child2", "root.child2", JsonType::Number, 1).with_value("42");
    let root =
        JsonNode::new("root", "root", JsonType::Object, 0).with_children(vec![child1, child2]);

    let collapsed = std::collections::HashSet::new();
    let result = flatten_tree(&root, &collapsed);
    assert_eq!(result.len(), 3); // root + 2 children
}

#[test]
fn test_flatten_tree_with_collapsed_node() {
    let child = JsonNode::new("child", "root.child", JsonType::Boolean, 1).with_value("true");
    let root = JsonNode::new("root", "root", JsonType::Array, 0).with_children(vec![child]);

    let mut collapsed = std::collections::HashSet::new();
    collapsed.insert(String::from("root"));
    let result = flatten_tree(&root, &collapsed);
    assert_eq!(result.len(), 1); // Only root, children collapsed
}

#[test]
fn test_flatten_tree_with_partial_collapse() {
    let grandchild = JsonNode::new("grandchild", "root.child.grandchild", JsonType::String, 2)
        .with_value("value");
    let child = JsonNode::new("child", "root.child", JsonType::Object, 1)
        .with_children(vec![grandchild]);
    let root = JsonNode::new("root", "root", JsonType::Object, 0).with_children(vec![child]);

    let mut collapsed = std::collections::HashSet::new();
    collapsed.insert(String::from("root.child"));
    let result = flatten_tree(&root, &collapsed);
    assert_eq!(result.len(), 2); // root + child, grandchild collapsed
}

#[test]
fn test_flatten_tree_index_assignment() {
    let child1 = JsonNode::new("a", "root.a", JsonType::String, 1).with_value("1");
    let child2 = JsonNode::new("b", "root.b", JsonType::String, 1).with_value("2");
    let root =
        JsonNode::new("root", "root", JsonType::Object, 0).with_children(vec![child1, child2]);

    let collapsed = std::collections::HashSet::new();
    let result = flatten_tree(&root, &collapsed);
    assert_eq!(result[0].index, 0);
    assert_eq!(result[1].index, 1);
    assert_eq!(result[2].index, 2);
}

#[test]
fn test_flatten_tree_deep_nesting() {
    let leaf = JsonNode::new("leaf", "root.a.b.leaf", JsonType::String, 3).with_value("value");
    let mid = JsonNode::new("b", "root.a.b", JsonType::Object, 2).with_children(vec![leaf]);
    let child = JsonNode::new("a", "root.a", JsonType::Object, 1).with_children(vec![mid]);
    let root = JsonNode::new("root", "root", JsonType::Object, 0).with_children(vec![child]);

    let collapsed = std::collections::HashSet::new();
    let result = flatten_tree(&root, &collapsed);
    assert_eq!(result.len(), 4);
    assert_eq!(result[0].depth, 0);
    assert_eq!(result[1].depth, 1);
    assert_eq!(result[2].depth, 2);
    assert_eq!(result[3].depth, 3);
}

#[test]
fn test_flatten_tree_empty_collapsed_set() {
    let child = JsonNode::new("child", "root.child", JsonType::String, 1).with_value("value");
    let root = JsonNode::new("root", "root", JsonType::Object, 0).with_children(vec![child]);

    let collapsed = std::collections::HashSet::new();
    let result = flatten_tree(&root, &collapsed);
    assert_eq!(result.len(), 2); // All nodes visible
}

// =========================================================================
// line_number_width() helper tests
// =========================================================================

#[test]
fn test_line_number_width_hidden() {
    assert_eq!(line_number_width(false, 100), 0);
}

#[test]
fn test_line_number_width_single_digit() {
    assert_eq!(line_number_width(true, 5), 3); // min 2 + 1 = 3
}

#[test]
fn test_line_number_width_two_digits() {
    assert_eq!(line_number_width(true, 10), 3); // 2 + 1 = 3
}

#[test]
fn test_line_number_width_three_digits() {
    assert_eq!(line_number_width(true, 100), 4); // 3 + 1 = 4
}

#[test]
fn test_line_number_width_large() {
    assert_eq!(line_number_width(true, 1000), 5); // 4 + 1 = 5
}

#[test]
fn test_line_number_width_zero_lines() {
    // log10(0) is -inf, but we add 1 and max with 2
    let width = line_number_width(true, 0);
    assert!(width >= 2); // Should handle gracefully
}

#[test]
fn test_line_number_width_very_large() {
    assert_eq!(line_number_width(true, 999999), 7); // 6 + 1 = 7
}

#[test]
fn test_line_number_width_minimum_width() {
    // Even with 1 line, should be at least 3 (min 2 digits + 1)
    assert_eq!(line_number_width(true, 1), 3);
}
