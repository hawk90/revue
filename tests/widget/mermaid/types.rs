//! Tests for mermaid diagram types

use revue::widget::{
    ArrowStyle, Diagram, DiagramDirection, DiagramEdge, DiagramNode, DiagramType, NodeShape,
};

// =========================================================================
// DiagramType enum trait tests
// =========================================================================

#[test]
fn test_diagram_type_default() {
    assert_eq!(DiagramType::default(), DiagramType::Flowchart);
}

#[test]
fn test_diagram_type_copy() {
    let dt1 = DiagramType::Sequence;
    let dt2 = dt1;
    assert_eq!(dt1, DiagramType::Sequence);
    assert_eq!(dt2, DiagramType::Sequence);
}

#[test]
fn test_diagram_type_partial_eq() {
    assert_eq!(DiagramType::Flowchart, DiagramType::Flowchart);
    assert_eq!(DiagramType::Sequence, DiagramType::Sequence);
    assert_eq!(DiagramType::Tree, DiagramType::Tree);
    assert_eq!(DiagramType::Er, DiagramType::Er);

    assert_ne!(DiagramType::Flowchart, DiagramType::Sequence);
    assert_ne!(DiagramType::Tree, DiagramType::Er);
    assert_ne!(DiagramType::Flowchart, DiagramType::Tree);
}

#[test]
fn test_diagram_type_debug() {
    let debug_str = format!("{:?}", DiagramType::Flowchart);
    assert!(debug_str.contains("Flowchart"));
}

// =========================================================================
// NodeShape enum trait tests
// =========================================================================

#[test]
fn test_node_shape_default() {
    assert_eq!(NodeShape::default(), NodeShape::Rectangle);
}

#[test]
fn test_node_shape_copy() {
    let ns1 = NodeShape::Circle;
    let ns2 = ns1;
    assert_eq!(ns1, NodeShape::Circle);
    assert_eq!(ns2, NodeShape::Circle);
}

#[test]
fn test_node_shape_partial_eq() {
    assert_eq!(NodeShape::Rectangle, NodeShape::Rectangle);
    assert_eq!(NodeShape::Rounded, NodeShape::Rounded);
    assert_eq!(NodeShape::Diamond, NodeShape::Diamond);
    assert_eq!(NodeShape::Circle, NodeShape::Circle);
    assert_eq!(NodeShape::Parallelogram, NodeShape::Parallelogram);
    assert_eq!(NodeShape::Database, NodeShape::Database);

    assert_ne!(NodeShape::Rectangle, NodeShape::Rounded);
    assert_ne!(NodeShape::Diamond, NodeShape::Circle);
    assert_ne!(NodeShape::Parallelogram, NodeShape::Database);
}

#[test]
fn test_node_shape_debug() {
    let debug_str = format!("{:?}", NodeShape::Database);
    assert!(debug_str.contains("Database"));
}

// =========================================================================
// ArrowStyle enum trait tests
// =========================================================================

#[test]
fn test_arrow_style_default() {
    assert_eq!(ArrowStyle::default(), ArrowStyle::Solid);
}

#[test]
fn test_arrow_style_copy() {
    let as1 = ArrowStyle::Thick;
    let as2 = as1;
    assert_eq!(as1, ArrowStyle::Thick);
    assert_eq!(as2, ArrowStyle::Thick);
}

#[test]
fn test_arrow_style_partial_eq() {
    assert_eq!(ArrowStyle::Solid, ArrowStyle::Solid);
    assert_eq!(ArrowStyle::Dashed, ArrowStyle::Dashed);
    assert_eq!(ArrowStyle::Thick, ArrowStyle::Thick);
    assert_eq!(ArrowStyle::Line, ArrowStyle::Line);

    assert_ne!(ArrowStyle::Solid, ArrowStyle::Dashed);
    assert_ne!(ArrowStyle::Thick, ArrowStyle::Line);
    assert_ne!(ArrowStyle::Solid, ArrowStyle::Line);
}

#[test]
fn test_arrow_style_debug() {
    let debug_str = format!("{:?}", ArrowStyle::Dashed);
    assert!(debug_str.contains("Dashed"));
}

// =========================================================================
// DiagramNode struct tests
// =========================================================================

#[test]
fn test_diagram_node_new() {
    let node = DiagramNode::new("id123", "My Label");
    assert_eq!(node.id, "id123");
    assert_eq!(node.label, "My Label");
    assert_eq!(node.shape, NodeShape::default());
    assert!(node.color.is_none());
    assert!(node.bg.is_none());
}

#[test]
fn test_diagram_node_shape_builder() {
    let node = DiagramNode::new("A", "B").shape(NodeShape::Diamond);
    assert_eq!(node.shape, NodeShape::Diamond);
}

#[test]
fn test_diagram_node_color_builder() {
    use revue::style::Color;
    let node = DiagramNode::new("A", "B").color(Color::RED);
    assert_eq!(node.color, Some(Color::RED));
}

#[test]
fn test_diagram_node_bg_builder() {
    use revue::style::Color;
    let node = DiagramNode::new("A", "B").bg(Color::BLUE);
    assert_eq!(node.bg, Some(Color::BLUE));
}

#[test]
fn test_diagram_node_builder_chain() {
    use revue::style::Color;
    let node = DiagramNode::new("A", "B")
        .shape(NodeShape::Circle)
        .color(Color::GREEN)
        .bg(Color::BLACK);

    assert_eq!(node.shape, NodeShape::Circle);
    assert_eq!(node.color, Some(Color::GREEN));
    assert_eq!(node.bg, Some(Color::BLACK));
}

#[test]
fn test_diagram_node_clone() {
    use revue::style::Color;
    let node1 = DiagramNode::new("A", "B")
        .shape(NodeShape::Rounded)
        .color(Color::CYAN)
        .bg(Color::rgb(20, 20, 30));

    let node2 = node1.clone();

    assert_eq!(node1.id, node2.id);
    assert_eq!(node1.label, node2.label);
    assert_eq!(node1.shape, node2.shape);
    assert_eq!(node1.color, node2.color);
    assert_eq!(node1.bg, node2.bg);
}

#[test]
fn test_diagram_node_debug() {
    let node = DiagramNode::new("test_id", "Test Label");
    let debug_str = format!("{:?}", node);
    assert!(debug_str.contains("test_id") || debug_str.contains("Test Label"));
}

#[test]
fn test_diagram_node_with_empty_strings() {
    let node = DiagramNode::new("", "");
    assert_eq!(node.id, "");
    assert_eq!(node.label, "");
}

// =========================================================================
// DiagramEdge struct tests
// =========================================================================

#[test]
fn test_diagram_edge_new() {
    let edge = DiagramEdge::new("A", "B");
    assert_eq!(edge.from, "A");
    assert_eq!(edge.to, "B");
    assert!(edge.label.is_none());
    assert_eq!(edge.style, ArrowStyle::default());
}

#[test]
fn test_diagram_edge_label_builder() {
    let edge = DiagramEdge::new("A", "B").label("connects");
    assert_eq!(edge.label, Some("connects".to_string()));
}

#[test]
fn test_diagram_edge_label_builder_with_string() {
    let label = String::from("test label");
    let edge = DiagramEdge::new("A", "B").label(label.clone());
    assert_eq!(edge.label, Some(label));
}

#[test]
fn test_diagram_edge_style_builder() {
    let edge = DiagramEdge::new("A", "B").style(ArrowStyle::Dashed);
    assert_eq!(edge.style, ArrowStyle::Dashed);
}

#[test]
fn test_diagram_edge_builder_chain() {
    let edge = DiagramEdge::new("A", "B")
        .label("test label")
        .style(ArrowStyle::Thick);

    assert_eq!(edge.from, "A");
    assert_eq!(edge.to, "B");
    assert_eq!(edge.label, Some("test label".to_string()));
    assert_eq!(edge.style, ArrowStyle::Thick);
}

#[test]
fn test_diagram_edge_clone() {
    let edge1 = DiagramEdge::new("X", "Y")
        .label("test")
        .style(ArrowStyle::Line);

    let edge2 = edge1.clone();

    assert_eq!(edge1.from, edge2.from);
    assert_eq!(edge1.to, edge2.to);
    assert_eq!(edge1.label, edge2.label);
    assert_eq!(edge1.style, edge2.style);
}

#[test]
fn test_diagram_edge_debug() {
    let edge = DiagramEdge::new("from_id", "to_id");
    let debug_str = format!("{:?}", edge);
    assert!(debug_str.contains("from_id") || debug_str.contains("to_id"));
}

#[test]
fn test_diagram_edge_with_unicode_label() {
    let edge = DiagramEdge::new("A", "B").label("测试标签");
    assert_eq!(edge.label, Some("测试标签".to_string()));
}

#[test]
fn test_diagram_edge_with_empty_label() {
    let edge = DiagramEdge::new("A", "B").label("");
    assert_eq!(edge.label, Some("".to_string()));
}

// =========================================================================
// DiagramColors struct tests
// =========================================================================

#[test]
fn test_diagram_colors_default() {
    let colors = Diagram::new().colors;
    assert_eq!(colors.node_fg, None);
    assert_eq!(colors.node_bg, revue::style::Color::rgb(40, 60, 80));
    assert_eq!(colors.arrow, revue::style::Color::rgb(100, 150, 200));
    assert_eq!(colors.label, revue::style::Color::rgb(180, 180, 180));
    assert_eq!(colors.title, revue::style::Color::CYAN);
}

#[test]
fn test_diagram_colors_clone() {
    let colors1 = Diagram::new().colors;
    let colors2 = colors1.clone();

    assert_eq!(colors1.node_fg, colors2.node_fg);
    assert_eq!(colors1.node_bg, colors2.node_bg);
    assert_eq!(colors1.arrow, colors2.arrow);
    assert_eq!(colors1.label, colors2.label);
    assert_eq!(colors1.title, colors2.title);
}

// =========================================================================
// DiagramDirection
// =========================================================================

#[test]
fn test_diagram_direction_default_is_top_down() {
    assert_eq!(DiagramDirection::default(), DiagramDirection::TopDown);
    assert_eq!(Diagram::new().direction, DiagramDirection::TopDown);
}

#[test]
fn test_diagram_direction_builder() {
    let d = Diagram::new().direction(DiagramDirection::LeftRight);
    assert_eq!(d.direction, DiagramDirection::LeftRight);
}
