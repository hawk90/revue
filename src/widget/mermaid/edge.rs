//! Diagram edge types and builders

use super::types::ArrowStyle;

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
