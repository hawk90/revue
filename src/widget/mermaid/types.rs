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
    /// Rectangle [text]
    #[default]
    Rectangle,
    /// Rounded rectangle (text)
    Rounded,
    /// Diamond {text}
    Diamond,
    /// Circle ((text))
    Circle,
    /// Parallelogram /text/
    Parallelogram,
    /// Database [(text)]
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
