//! Tests for the mermaid helper functions and flowchart parsing

use revue::widget::{diagram, edge, flowchart, node, ArrowStyle, Diagram, DiagramType, NodeShape};

fn nodes(d: &Diagram) -> Vec<(&str, &str)> {
    d.get_nodes()
        .iter()
        .map(|n| (n.id.as_str(), n.label.as_str()))
        .collect()
}

fn edges(d: &Diagram) -> Vec<(&str, &str, Option<&str>)> {
    d.get_edges()
        .iter()
        .map(|e| (e.from.as_str(), e.to.as_str(), e.label.as_deref()))
        .collect()
}

// =========================================================================
// diagram()
// =========================================================================

#[test]
fn test_diagram_function() {
    let d = diagram();
    assert!(d.get_nodes().is_empty());
    assert!(d.get_edges().is_empty());
    assert_eq!(d.title, "");
    assert_eq!(d.diagram_type, DiagramType::Flowchart);
}

#[test]
fn test_diagram_builders() {
    let d = diagram()
        .title("Flow")
        .diagram_type(DiagramType::Tree)
        .node(node("A", "Start"))
        .node(node("B", "End"))
        .edge(edge("A", "B"));
    assert_eq!(d.title, "Flow");
    assert_eq!(d.diagram_type, DiagramType::Tree);
    assert_eq!(nodes(&d), vec![("A", "Start"), ("B", "End")]);
    assert_eq!(edges(&d), vec![("A", "B", None)]);
}

// =========================================================================
// flowchart()
// =========================================================================

#[test]
fn test_flowchart_with_simple_edge() {
    let d = flowchart("A --> B");
    assert_eq!(d.diagram_type, DiagramType::Flowchart);
    assert_eq!(nodes(&d), vec![("A", "A"), ("B", "B")]);
    assert_eq!(edges(&d), vec![("A", "B", None)]);
}

#[test]
fn test_flowchart_without_spaces() {
    let d = flowchart("A-->B");
    assert_eq!(edges(&d), vec![("A", "B", None)]);
}

#[test]
fn test_flowchart_single_dash_arrow_is_not_an_edge() {
    // Only `-->` is understood
    let d = flowchart("A -> B");
    assert!(d.get_nodes().is_empty());
    assert!(d.get_edges().is_empty());
}

#[test]
fn test_flowchart_with_empty_source() {
    let d = flowchart("");
    assert!(d.get_nodes().is_empty());
    assert!(d.get_edges().is_empty());
}

#[test]
fn test_flowchart_with_multiple_edges() {
    let d = flowchart("A --> B\nB --> C\nC --> A");
    // Nodes are added once, in order of first appearance
    assert_eq!(nodes(&d), vec![("A", "A"), ("B", "B"), ("C", "C")]);
    assert_eq!(
        edges(&d),
        vec![("A", "B", None), ("B", "C", None), ("C", "A", None)]
    );
}

#[test]
fn test_flowchart_node_labels() {
    let d = flowchart("A[Start] --> B{Decide}\nB --> C(Done)");
    assert_eq!(
        nodes(&d),
        vec![("A", "Start"), ("B", "Decide"), ("C", "Done")]
    );
}

#[test]
fn test_flowchart_edge_labels() {
    let d = flowchart("A[Start] --> B{Process}\nB -->|Yes| C[End]\nB -->|No| D[Retry]");
    assert_eq!(
        edges(&d),
        vec![
            ("A", "B", None),
            ("B", "C", Some("Yes")),
            ("B", "D", Some("No")),
        ]
    );
}

#[test]
fn test_flowchart_ignores_other_lines() {
    let d = flowchart(
        "graph TD\n%% a comment --> X\nsubgraph One\n    A-->B\nend\nstyle A fill:#f9f,stroke:#333",
    );
    assert_eq!(nodes(&d), vec![("A", "A"), ("B", "B")]);
    assert_eq!(edges(&d), vec![("A", "B", None)]);
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
    let d = flowchart(source);
    assert_eq!(
        nodes(&d),
        vec![
            ("A", "Start"),
            ("B", "Is it working?"),
            ("C", "Great!"),
            ("D", "Debug"),
        ]
    );
    assert_eq!(
        edges(&d),
        vec![
            ("A", "B", None),
            ("B", "C", Some("Yes")),
            ("B", "D", Some("No")),
            ("D", "B", None),
        ]
    );
}

#[test]
fn test_flowchart_keeps_first_label_of_a_node() {
    let d = flowchart("A[First] --> B\nA[Second] --> C");
    assert_eq!(nodes(&d)[0], ("A", "First"));
}

// =========================================================================
// node() / edge()
// =========================================================================

#[test]
fn test_node_function() {
    let n = node("id", "label");
    assert_eq!(n.id, "id");
    assert_eq!(n.label, "label");
    assert_eq!(n.shape, NodeShape::Rectangle);
    assert!(n.color.is_none());
    assert!(n.bg.is_none());
}

#[test]
fn test_node_with_owned_strings_and_unicode() {
    let n = node(String::from("ノード"), String::from("🎯 Target"));
    assert_eq!(n.id, "ノード");
    assert_eq!(n.label, "🎯 Target");
}

#[test]
fn test_edge_function() {
    let e = edge("A", "B");
    assert_eq!(e.from, "A");
    assert_eq!(e.to, "B");
    assert!(e.label.is_none());
    assert_eq!(e.style, ArrowStyle::Solid);
}

#[test]
fn test_edge_with_owned_strings_and_unicode() {
    let e = edge(String::from("開始"), String::from("終了"));
    assert_eq!(e.from, "開始");
    assert_eq!(e.to, "終了");
}
