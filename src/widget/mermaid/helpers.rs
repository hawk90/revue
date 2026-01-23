//! Helper functions for creating diagrams

use super::{core::Diagram, types::DiagramEdge, DiagramNode};

/// Create a new diagram
pub fn diagram() -> Diagram {
    Diagram::new()
}

/// Create a flowchart from mermaid-like syntax
pub fn flowchart(source: &str) -> Diagram {
    Diagram::new()
        .diagram_type(super::types::DiagramType::Flowchart)
        .parse(source)
}

/// Create a node
pub fn node(id: impl Into<String>, label: impl Into<String>) -> DiagramNode {
    DiagramNode::new(id, label)
}

/// Create an edge
pub fn edge(from: impl Into<String>, to: impl Into<String>) -> DiagramEdge {
    DiagramEdge::new(from, to)
}
