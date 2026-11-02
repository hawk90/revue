//! Helper functions for JsonViewer

use super::types::JsonNode;

/// Helper function to create a JSON viewer
pub fn json_viewer() -> super::JsonViewer {
    super::JsonViewer::new()
}

/// Get flattened list of visible nodes
///
/// Borrows them: a clone of a node is a clone of its whole subtree, and this
/// runs on every render.
pub fn flatten_tree<'a>(
    root: &'a JsonNode,
    collapsed: &std::collections::HashSet<String>,
) -> Vec<&'a JsonNode> {
    let mut nodes = Vec::new();
    flatten_node(root, &mut nodes, collapsed);
    nodes
}

fn flatten_node<'a>(
    node: &'a JsonNode,
    nodes: &mut Vec<&'a JsonNode>,
    collapsed: &std::collections::HashSet<String>,
) {
    nodes.push(node);

    if node.is_container() && !collapsed.contains(&node.path) {
        for child in &node.children {
            flatten_node(child, nodes, collapsed);
        }
    }
}

/// Get line number width
pub fn line_number_width(show_line_numbers: bool, total_lines: usize) -> u16 {
    if show_line_numbers {
        let digits = (total_lines.max(1) as f64).log10().floor() as u16 + 1;
        digits.max(2) + 1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widget::data::json_viewer::parser::parse_json;
    use std::collections::HashSet;

    #[test]
    fn flatten_borrows_the_nodes_of_the_tree_in_document_order() {
        let root = parse_json(r#"{"a": [1, 2], "b": 3}"#).unwrap();
        let flat = flatten_tree(&root, &HashSet::new());

        let expected = [
            &root,
            &root.children[0],
            &root.children[0].children[0],
            &root.children[0].children[1],
            &root.children[1],
        ];
        assert_eq!(flat.len(), expected.len());
        for (got, want) in flat.iter().zip(expected) {
            assert!(std::ptr::eq(*got, want), "{} is a copy", got.path);
        }
    }

    #[test]
    fn flatten_keeps_a_collapsed_node_and_leaves_out_what_is_below_it() {
        let root = parse_json(r#"{"a": [1, 2], "b": 3}"#).unwrap();
        let collapsed = HashSet::from(["$.a".to_string()]);
        let paths: Vec<_> = flatten_tree(&root, &collapsed)
            .iter()
            .map(|n| n.path.as_str())
            .collect();
        assert_eq!(paths, ["$", "$.a", "$.b"]);
    }

    #[test]
    fn a_collapsed_root_is_the_only_row() {
        let root = parse_json(r#"{"a": 1}"#).unwrap();
        let collapsed = HashSet::from(["$".to_string()]);
        assert_eq!(flatten_tree(&root, &collapsed).len(), 1);
    }
}
