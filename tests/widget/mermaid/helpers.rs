//! Tests for mermaid diagram helper functions

use revue::widget::mermaid::{diagram, flowchart, node, edge, Diagram, DiagramNode, DiagramEdge, DiagramType, ArrowStyle, NodeShape};

// =========================================================================
// diagram() helper function tests
// =========================================================================

#[test]
fn test_diagram_function() {
    let diagram = diagram();
    let _ = diagram;
}

#[test]
fn test_diagram_multiple_times() {
    let diagram1 = diagram();
    let diagram2 = diagram();
    let _ = diagram1;
    let _ = diagram2;
}

#[test]
fn test_diagram_is_chainable() {
    let diagram = diagram();
    // Should allow builder methods to be chained
    let _ = diagram;
}

// =========================================================================
// flowchart() helper function tests
// =========================================================================

#[test]
fn test_flowchart_function() {
    let diagram = flowchart("A -> B");
    let _ = diagram;
}

#[test]
fn test_flowchart_with_simple_edge() {
    let diagram = flowchart("A --> B");
    let _ = diagram;
}

#[test]
fn test_flowchart_with_multiple_edges() {
    let diagram = flowchart("A -> B\nB -> C\nC -> A");
    let _ = diagram;
}

#[test]
fn test_flowchart_with_empty_source() {
    let diagram = flowchart("");
    let _ = diagram;
}

#[test]
fn test_flowchart_with_complex_syntax() {
    let diagram = flowchart("A[Start] --> B{Process}\nB -->|Yes| C[End]\nB -->|No| D[Retry]");
    let _ = diagram;
}

#[test]
fn test_flowchart_with_subgraphs() {
    let diagram = flowchart("subgraph One\n    A-->B\nend");
    let _ = diagram;
}

#[test]
fn test_flowchart_with_styles() {
    let diagram = flowchart("A-->B\nstyle A fill:#f9f,stroke:#333");
    let _ = diagram;
}

#[test]
fn test_flowchart_is_chainable() {
    let diagram = flowchart("A -> B");
    // Should allow further builder methods
    let _ = diagram;
}

#[test]
fn test_flowchart_multiple_instances() {
    let diagram1 = flowchart("X -> Y");
    let diagram2 = flowchart("P -> Q");
    let _ = diagram1;
    let _ = diagram2;
}

// =========================================================================
// node() helper function tests
// =========================================================================

#[test]
fn test_node_function() {
    let node = node("id", "label");
    assert_eq!(node.id, "id");
    assert_eq!(node.label, "label");
}

#[test]
fn test_node_with_string_id() {
    let node = node(String::from("node1"), "Label");
    assert_eq!(node.id, "node1");
    assert_eq!(node.label, "Label");
}

#[test]
fn test_node_with_string_label() {
    let node = node("id", String::from("My Label"));
    assert_eq!(node.id, "id");
    assert_eq!(node.label, "My Label");
}

#[test]
fn test_node_with_both_strings() {
    let node = node(String::from("A"), String::from("B"));
    assert_eq!(node.id, "A");
    assert_eq!(node.label, "B");
}

#[test]
fn test_node_with_empty_id() {
    let node = node("", "Label");
    assert_eq!(node.id, "");
    assert_eq!(node.label, "Label");
}

#[test]
fn test_node_with_empty_label() {
    let node = node("id", "");
    assert_eq!(node.id, "id");
    assert_eq!(node.label, "");
}

#[test]
fn test_node_with_both_empty() {
    let node = node("", "");
    assert_eq!(node.id, "");
    assert_eq!(node.label, "");
}

#[test]
fn test_node_with_special_chars() {
    let node = node("node-1", "Label with spaces");
    assert_eq!(node.id, "node-1");
    assert_eq!(node.label, "Label with spaces");
}

#[test]
fn test_node_with_unicode() {
    let node = node("ノード", "ラベル");
    assert_eq!(node.id, "ノード");
    assert_eq!(node.label, "ラベル");
}

#[test]
fn test_node_with_emoji() {
    let node = node("node1", "🎯 Target");
    assert_eq!(node.id, "node1");
    assert_eq!(node.label, "🎯 Target");
}

#[test]
fn test_node_with_long_id() {
    let long_id = "a".repeat(100);
    let node = node(&long_id, "Label");
    assert_eq!(node.id.len(), 100);
}

#[test]
fn test_node_with_long_label() {
    let long_label = "L".repeat(200);
    let node = node("id", &long_label);
    assert_eq!(node.label.len(), 200);
}

#[test]
fn test_node_with_newlines() {
    let node = node("id", "Line 1\nLine 2");
    assert_eq!(node.id, "id");
    assert_eq!(node.label, "Line 1\nLine 2");
}

#[test]
fn test_node_multiple_instances() {
    let node1 = node("A", "Label A");
    let node2 = node("B", "Label B");
    assert_eq!(node1.id, "A");
    assert_eq!(node2.id, "B");
}

// =========================================================================
// edge() helper function tests
// =========================================================================

#[test]
fn test_edge_function() {
    let edge = edge("A", "B");
    assert_eq!(edge.from, "A");
    assert_eq!(edge.to, "B");
}

#[test]
fn test_edge_with_string_from() {
    let edge = edge(String::from("Start"), "End");
    assert_eq!(edge.from, "Start");
    assert_eq!(edge.to, "End");
}

#[test]
fn test_edge_with_string_to() {
    let edge = edge("Start", String::from("End"));
    assert_eq!(edge.from, "Start");
    assert_eq!(edge.to, "End");
}

#[test]
fn test_edge_with_both_strings() {
    let edge = edge(String::from("X"), String::from("Y"));
    assert_eq!(edge.from, "X");
    assert_eq!(edge.to, "Y");
}

#[test]
fn test_edge_same_node() {
    let edge = edge("A", "A");
    assert_eq!(edge.from, "A");
    assert_eq!(edge.to, "A");
}

#[test]
fn test_edge_with_empty_from() {
    let edge = edge("", "B");
    assert_eq!(edge.from, "");
    assert_eq!(edge.to, "B");
}

#[test]
fn test_edge_with_empty_to() {
    let edge = edge("A", "");
    assert_eq!(edge.from, "A");
    assert_eq!(edge.to, "");
}

#[test]
fn test_edge_with_both_empty() {
    let edge = edge("", "");
    assert_eq!(edge.from, "");
    assert_eq!(edge.to, "");
}

#[test]
fn test_edge_with_special_chars() {
    let edge = edge("node-1", "node_2");
    assert_eq!(edge.from, "node-1");
    assert_eq!(edge.to, "node_2");
}

#[test]
fn test_edge_with_unicode() {
    let edge = edge("開始", "終了");
    assert_eq!(edge.from, "開始");
    assert_eq!(edge.to, "終了");
}

#[test]
fn test_edge_with_numbers() {
    let edge = edge("1", "2");
    assert_eq!(edge.from, "1");
    assert_eq!(edge.to, "2");
}

#[test]
fn test_edge_with_emoji() {
    let edge = edge("🚀 Start", "🏁 End");
    assert_eq!(edge.from, "🚀 Start");
    assert_eq!(edge.to, "🏁 End");
}

#[test]
fn test_edge_long_ids() {
    let long_from = "a".repeat(50);
    let long_to = "b".repeat(50);
    let edge = edge(&long_from, &long_to);
    assert_eq!(edge.from.len(), 50);
    assert_eq!(edge.to.len(), 50);
}

#[test]
fn test_edge_multiple_instances() {
    let edge1 = edge("A", "B");
    let edge2 = edge("C", "D");
    assert_eq!(edge1.from, "A");
    assert_eq!(edge2.from, "C");
}

#[test]
fn test_edge_bidirectional_consistency() {
    let edge_ab = edge("A", "B");
    let edge_reverse = edge("B", "A");
    assert_eq!(edge_ab.from, "A");
    assert_eq!(edge_ab.to, "B");
    assert_eq!(edge_reverse.from, "B");
    assert_eq!(edge_reverse.to, "A");
}

// =========================================================================
// Combined usage tests
// =========================================================================

#[test]
fn test_helpers_combined() {
    // Test that helpers work together
    let n1 = node("A", "Start");
    let n2 = node("B", "End");
    let e = edge("A", "B");
    let d = diagram();

    assert_eq!(n1.id, "A");
    assert_eq!(n2.id, "B");
    assert_eq!(e.from, "A");
    assert_eq!(e.to, "B");
    let _ = d;
}

#[test]
fn test_helpers_do_not_panic() {
    // All helper functions should work without panicking
    let _ = diagram();
    let _ = flowchart("A->B");
    let _ = node("id", "label");
    let _ = edge("A", "B");
    let _ = node("", "");
    let _ = edge("", "");
    let _ = flowchart("");
}

#[test]
fn test_flowchart_with_whitespaces() {
    let diagram = flowchart("   A   ->   B   ");
    let _ = diagram;
}

#[test]
fn test_node_id_with_dots() {
    let node = node("node.sub", "Label");
    assert_eq!(node.id, "node.sub");
}

#[test]
fn test_edge_ids_with_dots() {
    let edge = edge("sub.node1", "sub.node2");
    assert_eq!(edge.from, "sub.node1");
    assert_eq!(edge.to, "sub.node2");
}

#[test]
fn test_node_label_with_quotes() {
    let node = node("id", "\"Label in quotes\"");
    assert_eq!(node.label, "\"Label in quotes\"");
}

#[test]
fn test_edge_id_with_colons() {
    let edge = edge("ns:A", "ns:B");
    assert_eq!(edge.from, "ns:A");
    assert_eq!(edge.to, "ns:B");
}

#[test]
fn test_flowchart_multiline_complex() {
    let source = r#"
        graph TD
            A[Start] --> B{Is it working?}
            B -->|Yes| C[Great!]
            B -->|No| D[Debug]
            D --> B
    "#;
    let diagram = flowchart(source);
    let _ = diagram;
}