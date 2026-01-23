//! Diagram node types and builders

use super::types::NodeShape;
use crate::style::Color;

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
