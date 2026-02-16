//! Helper functions for JsonViewer

use super::types::JsonNode;

/// Helper function to create a JSON viewer
pub fn json_viewer() -> super::JsonViewer {
    super::JsonViewer::new()
}

/// Get flattened list of visible nodes
pub fn flatten_tree(
    root: &JsonNode,
    collapsed: &std::collections::HashSet<String>,
) -> Vec<JsonNode> {
    let mut nodes = Vec::new();
    flatten_node(root, &mut nodes, collapsed);
    nodes
}

fn flatten_node(
    node: &JsonNode,
    nodes: &mut Vec<JsonNode>,
    collapsed: &std::collections::HashSet<String>,
) {
    let mut node_clone = node.clone();
    node_clone.index = nodes.len();
    nodes.push(node_clone);

    if node.is_container() && !collapsed.contains(&node.path) {
        for child in &node.children {
            flatten_node(child, nodes, collapsed);
        }
    }
}

/// Get line number width
pub fn line_number_width(show_line_numbers: bool, total_lines: usize) -> u16 {
    if show_line_numbers {
        let digits = (total_lines as f64).log10().floor() as u16 + 1;
        digits.max(2) + 1
    } else {
        0
    }
}

// Tests moved to tests/widget/data/json_viewer_helpers.rs
