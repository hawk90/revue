//! Core types and enums for mermaid diagrams

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
    /// Dashed arrow -.->
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
