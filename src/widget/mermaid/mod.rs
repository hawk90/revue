//! Mermaid-style diagram rendering in ASCII
//!
//! Renders flowcharts, sequence diagrams, and other diagrams
//! using ASCII/Unicode art.

mod core;
mod helpers;
mod render;
mod types;

pub use core::Diagram;
pub use helpers::{diagram, edge, flowchart, node};
pub use types::{ArrowStyle, DiagramEdge, DiagramNode, DiagramType, NodeShape};

crate::impl_styled_view!(Diagram);
crate::impl_props_builders!(Diagram);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::mermaid::types::{DiagramColors, Direction};

    // ========================================================================
    // DiagramType tests
    // ========================================================================

    #[test]
    fn test_diagram_type_default() {
        assert_eq!(DiagramType::default(), DiagramType::Flowchart);
    }

    #[test]
    fn test_diagram_type_variants() {
        assert_ne!(DiagramType::Flowchart, DiagramType::Sequence);
        assert_ne!(DiagramType::Tree, DiagramType::Er);
    }

    // ========================================================================
    // NodeShape tests
    // ========================================================================

    #[test]
    fn test_node_shape_default() {
        assert_eq!(NodeShape::default(), NodeShape::Rectangle);
    }

    #[test]
    fn test_node_shape_variants() {
        assert_ne!(NodeShape::Rectangle, NodeShape::Rounded);
        assert_ne!(NodeShape::Diamond, NodeShape::Circle);
        assert_ne!(NodeShape::Parallelogram, NodeShape::Database);
    }

    // ========================================================================
    // ArrowStyle tests
    // ========================================================================

    #[test]
    fn test_arrow_style_default() {
        assert_eq!(ArrowStyle::default(), ArrowStyle::Solid);
    }

    #[test]
    fn test_arrow_style_variants() {
        assert_ne!(ArrowStyle::Solid, ArrowStyle::Dashed);
        assert_ne!(ArrowStyle::Thick, ArrowStyle::Line);
    }

    // ========================================================================
    // DiagramNode tests
    // ========================================================================

    #[test]
    fn test_diagram_node_new() {
        let node = DiagramNode::new("A", "Start");
        assert_eq!(node.id, "A");
        assert_eq!(node.label, "Start");
        assert_eq!(node.shape, NodeShape::Rectangle);
        assert!(node.color.is_none());
        assert!(node.bg.is_none());
    }

    #[test]
    fn test_diagram_node_shape() {
        let node = DiagramNode::new("A", "Test").shape(NodeShape::Diamond);
        assert_eq!(node.shape, NodeShape::Diamond);
    }

    #[test]
    fn test_diagram_node_color() {
        let node = DiagramNode::new("A", "Test").color(crate::style::Color::RED);
        assert_eq!(node.color, Some(crate::style::Color::RED));
    }

    #[test]
    fn test_diagram_node_bg() {
        let node = DiagramNode::new("A", "Test").bg(crate::style::Color::BLUE);
        assert_eq!(node.bg, Some(crate::style::Color::BLUE));
    }

    #[test]
    fn test_diagram_node_builder_chain() {
        let node = DiagramNode::new("A", "Test")
            .shape(NodeShape::Rounded)
            .color(crate::style::Color::WHITE)
            .bg(crate::style::Color::BLACK);

        assert_eq!(node.shape, NodeShape::Rounded);
        assert_eq!(node.color, Some(crate::style::Color::WHITE));
        assert_eq!(node.bg, Some(crate::style::Color::BLACK));
    }

    // ========================================================================
    // DiagramEdge tests
    // ========================================================================

    #[test]
    fn test_diagram_edge_new() {
        let edge = DiagramEdge::new("A", "B");
        assert_eq!(edge.from, "A");
        assert_eq!(edge.to, "B");
        assert!(edge.label.is_none());
        assert_eq!(edge.style, ArrowStyle::Solid);
    }

    #[test]
    fn test_diagram_edge_label() {
        let edge = DiagramEdge::new("A", "B").label("connects");
        assert_eq!(edge.label, Some("connects".to_string()));
    }

    #[test]
    fn test_diagram_edge_style() {
        let edge = DiagramEdge::new("A", "B").style(ArrowStyle::Dashed);
        assert_eq!(edge.style, ArrowStyle::Dashed);
    }

    // ========================================================================
    // DiagramColors tests
    // ========================================================================

    #[test]
    fn test_diagram_colors_default() {
        let colors = DiagramColors::default();
        assert_eq!(colors.node_fg, crate::style::Color::WHITE);
        assert_eq!(colors.title, crate::style::Color::CYAN);
    }

    // ========================================================================
    // Direction tests
    // ========================================================================

    #[test]
    fn test_direction_default() {
        assert_eq!(Direction::default(), Direction::TopDown);
    }

    #[test]
    fn test_direction_variants() {
        assert_ne!(Direction::TopDown, Direction::LeftRight);
        assert_ne!(Direction::BottomUp, Direction::RightLeft);
    }

    // ========================================================================
    // Diagram tests
    // ========================================================================

    #[test]
    fn test_diagram_new() {
        let diag = Diagram::new();
        assert!(diag.title.is_empty());
        assert!(diag.nodes.is_empty());
        assert!(diag.edges.is_empty());
    }

    #[test]
    fn test_diagram_default() {
        let diag = Diagram::default();
        assert!(diag.nodes.is_empty());
    }

    #[test]
    fn test_diagram_title() {
        let diag = Diagram::new().title("My Diagram");
        assert_eq!(diag.title, "My Diagram");
    }

    #[test]
    fn test_diagram_diagram_type() {
        let diag = Diagram::new().diagram_type(DiagramType::Sequence);
        assert_eq!(diag.diagram_type, DiagramType::Sequence);
    }

    #[test]
    fn test_diagram_direction() {
        let diag = Diagram::new().direction(Direction::LeftRight);
        assert_eq!(diag.direction, Direction::LeftRight);
    }

    #[test]
    fn test_diagram_colors() {
        let mut colors = DiagramColors::default();
        colors.title = crate::style::Color::RED;
        let diag = Diagram::new().colors(colors);
        assert_eq!(diag.colors.title, crate::style::Color::RED);
    }

    #[test]
    fn test_diagram_node() {
        let diag = Diagram::new()
            .node(DiagramNode::new("A", "Start"))
            .node(DiagramNode::new("B", "End"));
        assert_eq!(diag.nodes.len(), 2);
    }

    #[test]
    fn test_diagram_edge() {
        let diag = Diagram::new()
            .node(DiagramNode::new("A", "Start"))
            .node(DiagramNode::new("B", "End"))
            .edge(DiagramEdge::new("A", "B"));
        assert_eq!(diag.edges.len(), 1);
    }

    #[test]
    fn test_diagram_creation() {
        let diag = Diagram::new()
            .title("Test")
            .node(node("A", "Start"))
            .node(node("B", "End"))
            .edge(edge("A", "B"));

        assert_eq!(diag.nodes.len(), 2);
        assert_eq!(diag.edges.len(), 1);
    }

    // ========================================================================
    // Parse tests
    // ========================================================================

    #[test]
    fn test_parse_mermaid() {
        let diag = flowchart("A[Start] --> B[Process]\nB --> C{Decision}");
        assert_eq!(diag.nodes.len(), 3);
        assert_eq!(diag.edges.len(), 2);
    }

    #[test]
    fn test_parse_simple_arrow() {
        let diag = Diagram::new().parse("A --> B");
        assert_eq!(diag.nodes.len(), 2);
        assert_eq!(diag.edges.len(), 1);
    }

    #[test]
    fn test_parse_with_labels() {
        let diag = Diagram::new().parse("A[Label A] --> B[Label B]");
        assert_eq!(diag.nodes.len(), 2);
        assert_eq!(diag.nodes[0].label, "Label A");
        assert_eq!(diag.nodes[1].label, "Label B");
    }

    #[test]
    fn test_parse_edge_label() {
        let diag = Diagram::new().parse("A -->|yes| B");
        assert_eq!(diag.edges.len(), 1);
        assert_eq!(diag.edges[0].label, Some("yes".to_string()));
    }

    #[test]
    fn test_parse_comments() {
        let diag = Diagram::new().parse("A --> B\n%% comment\nB --> C");
        assert_eq!(diag.nodes.len(), 3);
        assert_eq!(diag.edges.len(), 2);
    }

    #[test]
    fn test_parse_empty_lines() {
        let diag = Diagram::new().parse("A --> B\n\n\nB --> C");
        assert_eq!(diag.edges.len(), 2);
    }

    #[test]
    fn test_parse_brace_label() {
        let diag = Diagram::new().parse("A{Decision} --> B");
        assert_eq!(diag.nodes[0].label, "Decision");
    }

    #[test]
    fn test_parse_paren_label() {
        let diag = Diagram::new().parse("A(Rounded) --> B");
        assert_eq!(diag.nodes[0].label, "Rounded");
    }

    // ========================================================================
    // Helper function tests
    // ========================================================================

    #[test]
    fn test_node_shapes() {
        let n = node("A", "Test")
            .shape(NodeShape::Diamond)
            .color(crate::style::Color::CYAN);
        assert_eq!(n.shape, NodeShape::Diamond);
    }

    #[test]
    fn test_diagram_helper() {
        let diag = diagram();
        assert!(diag.nodes.is_empty());
    }

    #[test]
    fn test_flowchart_helper() {
        let diag = flowchart("A --> B");
        assert_eq!(diag.diagram_type, DiagramType::Flowchart);
        assert_eq!(diag.nodes.len(), 2);
    }

    #[test]
    fn test_node_helper() {
        let n = node("id", "label");
        assert_eq!(n.id, "id");
        assert_eq!(n.label, "label");
    }

    #[test]
    fn test_edge_helper() {
        let e = edge("A", "B");
        assert_eq!(e.from, "A");
        assert_eq!(e.to, "B");
    }

    // ========================================================================
    // Render tests
    // ========================================================================

    #[test]
    fn test_diagram_render() {
        let diag = diagram()
            .node(node("A", "Hello"))
            .node(node("B", "World"))
            .edge(edge("A", "B"));

        let mut buffer = Buffer::new(60, 20);
        let area = Rect::new(0, 0, 60, 20);
        let mut ctx = crate::widget::traits::RenderContext::new(&mut buffer, area);

        diag.render(&mut ctx);
    }

    #[test]
    fn test_diagram_render_with_title() {
        let mut buffer = Buffer::new(60, 20);
        let area = Rect::new(0, 0, 60, 20);
        let mut ctx = crate::widget::traits::RenderContext::new(&mut buffer, area);

        let diag = diagram()
            .title("My Flowchart")
            .node(node("A", "Start"))
            .node(node("B", "End"))
            .edge(edge("A", "B"));

        diag.render(&mut ctx);
    }

    #[test]
    fn test_diagram_render_empty() {
        let mut buffer = Buffer::new(60, 20);
        let area = Rect::new(0, 0, 60, 20);
        let mut ctx = crate::widget::traits::RenderContext::new(&mut buffer, area);

        let diag = Diagram::new();
        diag.render(&mut ctx);
    }

    #[test]
    fn test_diagram_render_small_area() {
        let mut buffer = Buffer::new(5, 3);
        let area = Rect::new(0, 0, 5, 3);
        let mut ctx = crate::widget::traits::RenderContext::new(&mut buffer, area);

        let diag = diagram().node(node("A", "Start"));
        diag.render(&mut ctx);
        // Should handle small area gracefully
    }

    #[test]
    fn test_diagram_render_diamond_shape() {
        let mut buffer = Buffer::new(60, 20);
        let area = Rect::new(0, 0, 60, 20);
        let mut ctx = crate::widget::traits::RenderContext::new(&mut buffer, area);

        let diag = diagram().node(node("A", "Decision").shape(NodeShape::Diamond));
        diag.render(&mut ctx);
    }

    #[test]
    fn test_diagram_render_rounded_shape() {
        let mut buffer = Buffer::new(60, 20);
        let area = Rect::new(0, 0, 60, 20);
        let mut ctx = crate::widget::traits::RenderContext::new(&mut buffer, area);

        let diag = diagram().node(node("A", "Rounded").shape(NodeShape::Rounded));
        diag.render(&mut ctx);
    }

    #[test]
    fn test_diagram_render_with_edge_label() {
        let mut buffer = Buffer::new(60, 20);
        let area = Rect::new(0, 0, 60, 20);
        let mut ctx = crate::widget::traits::RenderContext::new(&mut buffer, area);

        let diag = diagram()
            .node(node("A", "Start"))
            .node(node("B", "End"))
            .edge(edge("A", "B").label("transition"));

        diag.render(&mut ctx);
    }

    #[test]
    fn test_diagram_render_multiple_nodes() {
        let mut buffer = Buffer::new(80, 30);
        let area = Rect::new(0, 0, 80, 30);
        let mut ctx = crate::widget::traits::RenderContext::new(&mut buffer, area);

        let diag = diagram()
            .node(node("A", "Start"))
            .node(node("B", "Process 1"))
            .node(node("C", "Process 2"))
            .node(node("D", "End"))
            .edge(edge("A", "B"))
            .edge(edge("B", "C"))
            .edge(edge("C", "D"));

        diag.render(&mut ctx);
    }
}
