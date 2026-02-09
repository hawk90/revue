//! Types for the Mermaid diagram widget

use crate::style::Color;

/// Diagram type
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DiagramType {
    /// Flowchart (boxes and arrows)
    #[default]
    Flowchart,
    /// Sequence diagram
    Sequence,
    /// Simple tree structure
    Tree,
    /// Entity relationship
    Er,
}

/// Node shape
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NodeShape {
    /// Rectangle `[text]` (Mermaid syntax)
    #[default]
    Rectangle,
    /// Rounded rectangle `(text)` (Mermaid syntax)
    Rounded,
    /// Diamond `{text}` (Mermaid syntax)
    Diamond,
    /// Circle `((text))` (Mermaid syntax)
    Circle,
    /// Parallelogram `/text/` (Mermaid syntax)
    Parallelogram,
    /// Database `[(text)]` (Mermaid syntax)
    Database,
}

/// Arrow style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ArrowStyle {
    /// Solid arrow -->
    #[default]
    Solid,
    /// Dashed arrow -.>
    Dashed,
    /// Thick arrow ==>
    Thick,
    /// No arrow ---
    Line,
}

/// Layout direction
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Direction {
    /// Top to bottom
    #[default]
    TopDown,
    /// Left to right
    LeftRight,
    /// Bottom to top
    BottomUp,
    /// Right to left
    RightLeft,
}

/// A node in the diagram
#[derive(Clone, Debug)]
pub struct DiagramNode {
    /// Node ID
    pub id: String,
    /// Display label
    pub label: String,
    /// Node shape
    pub shape: NodeShape,
    /// Node color
    pub color: Option<Color>,
    /// Background color
    pub bg: Option<Color>,
}

impl DiagramNode {
    /// Create a new node
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            shape: NodeShape::default(),
            color: None,
            bg: None,
        }
    }

    /// Set shape
    pub fn shape(mut self, shape: NodeShape) -> Self {
        self.shape = shape;
        self
    }

    /// Set color
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Set background
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }
}

/// An edge (connection) in the diagram
#[derive(Clone, Debug)]
pub struct DiagramEdge {
    /// Source node ID
    pub from: String,
    /// Target node ID
    pub to: String,
    /// Edge label
    pub label: Option<String>,
    /// Arrow style
    pub style: ArrowStyle,
}

impl DiagramEdge {
    /// Create a new edge
    pub fn new(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            label: None,
            style: ArrowStyle::default(),
        }
    }

    /// Set label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set arrow style
    pub fn style(mut self, style: ArrowStyle) -> Self {
        self.style = style;
        self
    }
}

/// Diagram color scheme
#[derive(Clone, Debug)]
pub struct DiagramColors {
    /// Default node color
    pub node_fg: Color,
    /// Default node background
    pub node_bg: Color,
    /// Arrow color
    pub arrow: Color,
    /// Label color
    pub label: Color,
    /// Title color
    pub title: Color,
}

impl Default for DiagramColors {
    fn default() -> Self {
        Self {
            node_fg: Color::WHITE,
            node_bg: Color::rgb(40, 60, 80),
            arrow: Color::rgb(100, 150, 200),
            label: Color::rgb(180, 180, 180),
            title: Color::CYAN,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // DiagramType enum trait tests
    // =========================================================================

    #[test]
    fn test_diagram_type_default() {
        assert_eq!(DiagramType::default(), DiagramType::Flowchart);
    }

    #[test]
    fn test_diagram_type_clone() {
        let dt1 = DiagramType::Flowchart;
        let dt2 = dt1.clone();
        assert_eq!(dt1, dt2);
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

    #[test]
    fn test_diagram_type_all_variants() {
        let _ = DiagramType::Flowchart;
        let _ = DiagramType::Sequence;
        let _ = DiagramType::Tree;
        let _ = DiagramType::Er;
    }

    // =========================================================================
    // NodeShape enum trait tests
    // =========================================================================

    #[test]
    fn test_node_shape_default() {
        assert_eq!(NodeShape::default(), NodeShape::Rectangle);
    }

    #[test]
    fn test_node_shape_clone() {
        let ns1 = NodeShape::Diamond;
        let ns2 = ns1.clone();
        assert_eq!(ns1, ns2);
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

    #[test]
    fn test_node_shape_all_variants() {
        let _ = NodeShape::Rectangle;
        let _ = NodeShape::Rounded;
        let _ = NodeShape::Diamond;
        let _ = NodeShape::Circle;
        let _ = NodeShape::Parallelogram;
        let _ = NodeShape::Database;
    }

    // =========================================================================
    // ArrowStyle enum trait tests
    // =========================================================================

    #[test]
    fn test_arrow_style_default() {
        assert_eq!(ArrowStyle::default(), ArrowStyle::Solid);
    }

    #[test]
    fn test_arrow_style_clone() {
        let as1 = ArrowStyle::Dashed;
        let as2 = as1.clone();
        assert_eq!(as1, as2);
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

    #[test]
    fn test_arrow_style_all_variants() {
        let _ = ArrowStyle::Solid;
        let _ = ArrowStyle::Dashed;
        let _ = ArrowStyle::Thick;
        let _ = ArrowStyle::Line;
    }

    // =========================================================================
    // Direction enum trait tests
    // =========================================================================

    #[test]
    fn test_direction_default() {
        assert_eq!(Direction::default(), Direction::TopDown);
    }

    #[test]
    fn test_direction_clone() {
        let d1 = Direction::LeftRight;
        let d2 = d1.clone();
        assert_eq!(d1, d2);
    }

    #[test]
    fn test_direction_copy() {
        let d1 = Direction::BottomUp;
        let d2 = d1;
        assert_eq!(d1, Direction::BottomUp);
        assert_eq!(d2, Direction::BottomUp);
    }

    #[test]
    fn test_direction_partial_eq() {
        assert_eq!(Direction::TopDown, Direction::TopDown);
        assert_eq!(Direction::LeftRight, Direction::LeftRight);
        assert_eq!(Direction::BottomUp, Direction::BottomUp);
        assert_eq!(Direction::RightLeft, Direction::RightLeft);

        assert_ne!(Direction::TopDown, Direction::LeftRight);
        assert_ne!(Direction::BottomUp, Direction::RightLeft);
        assert_ne!(Direction::TopDown, Direction::BottomUp);
    }

    #[test]
    fn test_direction_debug() {
        let debug_str = format!("{:?}", Direction::RightLeft);
        assert!(debug_str.contains("RightLeft"));
    }

    #[test]
    fn test_direction_all_variants() {
        let _ = Direction::TopDown;
        let _ = Direction::LeftRight;
        let _ = Direction::BottomUp;
        let _ = Direction::RightLeft;
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
    fn test_diagram_node_new_with_string_types() {
        let id = String::from("test_id");
        let label = String::from("test_label");
        let node = DiagramNode::new(id.clone(), label.clone());
        assert_eq!(node.id, id);
        assert_eq!(node.label, label);
    }

    #[test]
    fn test_diagram_node_shape_builder() {
        let node = DiagramNode::new("A", "B").shape(NodeShape::Diamond);
        assert_eq!(node.shape, NodeShape::Diamond);
    }

    #[test]
    fn test_diagram_node_color_builder() {
        let node = DiagramNode::new("A", "B").color(Color::RED);
        assert_eq!(node.color, Some(Color::RED));
    }

    #[test]
    fn test_diagram_node_bg_builder() {
        let node = DiagramNode::new("A", "B").bg(Color::BLUE);
        assert_eq!(node.bg, Some(Color::BLUE));
    }

    #[test]
    fn test_diagram_node_builder_chain() {
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

    #[test]
    fn test_diagram_node_with_unicode() {
        let node = DiagramNode::new("测试", "标签🏷️");
        assert_eq!(node.id, "测试");
        assert_eq!(node.label, "标签🏷️");
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
    fn test_diagram_edge_new_with_string_types() {
        let from = String::from("node1");
        let to = String::from("node2");
        let edge = DiagramEdge::new(from.clone(), to.clone());
        assert_eq!(edge.from, from);
        assert_eq!(edge.to, to);
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
        let colors = DiagramColors::default();
        assert_eq!(colors.node_fg, Color::WHITE);
        assert_eq!(colors.node_bg, Color::rgb(40, 60, 80));
        assert_eq!(colors.arrow, Color::rgb(100, 150, 200));
        assert_eq!(colors.label, Color::rgb(180, 180, 180));
        assert_eq!(colors.title, Color::CYAN);
    }

    #[test]
    fn test_diagram_colors_clone() {
        let colors1 = DiagramColors::default();
        let colors2 = colors1.clone();

        assert_eq!(colors1.node_fg, colors2.node_fg);
        assert_eq!(colors1.node_bg, colors2.node_bg);
        assert_eq!(colors1.arrow, colors2.arrow);
        assert_eq!(colors1.label, colors2.label);
        assert_eq!(colors1.title, colors2.title);
    }

    #[test]
    fn test_diagram_colors_debug() {
        let colors = DiagramColors::default();
        let debug_str = format!("{:?}", colors);
        // Should be able to format for debug
        assert!(!debug_str.is_empty());
    }
}
