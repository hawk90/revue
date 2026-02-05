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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagram_function() {
        let diagram = diagram();
        let _ = diagram;
    }

    #[test]
    fn test_flowchart_function() {
        let diagram = flowchart("A -> B");
        let _ = diagram;
    }

    #[test]
    fn test_node_function() {
        let node = node("id", "label");
        assert_eq!(node.id, "id");
        assert_eq!(node.label, "label");
    }

    #[test]
    fn test_edge_function() {
        let edge = edge("A", "B");
        assert_eq!(edge.from, "A");
        assert_eq!(edge.to, "B");
    }
}
