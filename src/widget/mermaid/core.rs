//! Core Diagram implementation

use super::types::{DiagramColors, DiagramDirection, DiagramEdge, DiagramNode, DiagramType};
use crate::widget::traits::WidgetProps;
use std::collections::HashMap;

/// Mermaid-style diagram widget
///
/// # Example
///
/// ```rust,ignore
/// use revue::prelude::*;
///
/// let diagram = Diagram::new()
///     .title("User Flow")
///     .node(DiagramNode::new("A", "Start"))
///     .node(DiagramNode::new("B", "Process").shape(NodeShape::Rectangle))
///     .node(DiagramNode::new("C", "Decision").shape(NodeShape::Diamond))
///     .edge(DiagramEdge::new("A", "B"))
///     .edge(DiagramEdge::new("B", "C").label("check"));
/// ```
pub struct Diagram {
    /// Diagram title
    pub title: String,
    /// Diagram type
    pub diagram_type: DiagramType,
    /// Nodes
    pub nodes: Vec<DiagramNode>,
    /// Edges
    pub edges: Vec<DiagramEdge>,
    /// Colors
    pub colors: DiagramColors,
    /// Direction the flow runs in (TD = top-down, LR = left-right,
    /// BT = bottom-up, RL = right-left)
    pub direction: DiagramDirection,
    /// Node positions (computed during layout)
    pub positions: HashMap<String, (u16, u16)>,
    /// Node sizes
    pub sizes: HashMap<String, (u16, u16)>,
    /// Widget properties
    pub props: WidgetProps,
}

impl Default for Diagram {
    fn default() -> Self {
        Self::new()
    }
}

impl Diagram {
    /// Create a new diagram
    pub fn new() -> Self {
        Self {
            title: String::new(),
            diagram_type: DiagramType::default(),
            nodes: Vec::new(),
            edges: Vec::new(),
            colors: DiagramColors::default(),
            direction: DiagramDirection::default(),
            positions: HashMap::new(),
            sizes: HashMap::new(),
            props: WidgetProps::new(),
        }
    }

    /// Set title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set diagram type
    pub fn diagram_type(mut self, dt: DiagramType) -> Self {
        self.diagram_type = dt;
        self
    }

    /// Set direction
    pub fn direction(mut self, dir: DiagramDirection) -> Self {
        self.direction = dir;
        self
    }

    /// Set colors
    pub fn colors(mut self, colors: DiagramColors) -> Self {
        self.colors = colors;
        self
    }

    /// Add a node
    pub fn node(mut self, node: DiagramNode) -> Self {
        self.nodes.push(node);
        self
    }

    /// Add an edge
    pub fn edge(mut self, edge: DiagramEdge) -> Self {
        self.edges.push(edge);
        self
    }

    /// Parse mermaid-like syntax
    pub fn parse(mut self, source: &str) -> Self {
        for line in source.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("%%") {
                continue;
            }

            // Parse arrows: A --> B or A -->|label| B
            if let Some((left, right)) = line.split_once("-->") {
                let from = left.trim();
                let (label, to) = if right.contains('|') {
                    let parts: Vec<&str> = right.split('|').collect();
                    if parts.len() >= 3 {
                        (Some(parts[1].to_string()), parts[2].trim())
                    } else {
                        (None, right.trim())
                    }
                } else {
                    (None, right.trim())
                };

                // Extract node IDs and labels
                let (from_id, from_label) = Self::parse_node_def(from);
                let (to_id, to_label) = Self::parse_node_def(to);

                // Add nodes if not exists
                if !self.nodes.iter().any(|n| n.id == from_id) {
                    self.nodes.push(DiagramNode::new(
                        &from_id,
                        from_label.unwrap_or_else(|| from_id.clone()),
                    ));
                }
                if !self.nodes.iter().any(|n| n.id == to_id) {
                    self.nodes.push(DiagramNode::new(
                        &to_id,
                        to_label.unwrap_or_else(|| to_id.clone()),
                    ));
                }

                let mut edge = DiagramEdge::new(from_id, to_id);
                if let Some(l) = label {
                    edge = edge.label(l);
                }
                self.edges.push(edge);
            }
        }
        self
    }

    /// Getter for nodes (for testing)
    #[doc(hidden)]
    pub fn get_nodes(&self) -> &Vec<DiagramNode> {
        &self.nodes
    }

    /// Getter for edges (for testing)
    #[doc(hidden)]
    pub fn get_edges(&self) -> &Vec<DiagramEdge> {
        &self.edges
    }

    /// Parse node definition like `A[Label]` or `B{Decision}`
    fn parse_node_def(s: &str) -> (String, Option<String>) {
        let s = s.trim();

        // `[Label]`
        if let Some(bracket_start) = s.find('[') {
            if let Some(bracket_end) = s.find(']') {
                let id = s[..bracket_start].trim().to_string();
                let label = s[bracket_start + 1..bracket_end].to_string();
                return (id, Some(label));
            }
        }

        // {Label}
        if let Some(brace_start) = s.find('{') {
            if let Some(brace_end) = s.find('}') {
                let id = s[..brace_start].trim().to_string();
                let label = s[brace_start + 1..brace_end].to_string();
                return (id, Some(label));
            }
        }

        // (Label)
        if let Some(paren_start) = s.find('(') {
            if let Some(paren_end) = s.rfind(')') {
                let id = s[..paren_start].trim().to_string();
                let label = s[paren_start + 1..paren_end].to_string();
                return (id, Some(label));
            }
        }

        (s.to_string(), None)
    }

    /// Compute layout
    pub fn compute_layout(&mut self, width: u16, height: u16) {
        self.positions.clear();
        self.sizes.clear();

        if self.nodes.is_empty() {
            return;
        }

        // Simple grid layout. The flow runs along the direction's axis: nodes
        // fill the grid in reading order, top-down by rows and left-right by
        // columns, and the grid is mirrored for bottom-up and right-left.
        let count = self.nodes.len() as u16;
        let along = ((self.nodes.len() as f32).sqrt().ceil() as u16).max(1);
        let across = count.div_ceil(along).max(1);
        let horizontal = matches!(
            self.direction,
            DiagramDirection::LeftRight | DiagramDirection::RightLeft
        );
        let (rows, cols) = if horizontal {
            (across, along)
        } else {
            (along, across)
        };

        let cell_width = width / cols;
        let cell_height = height / rows;

        for (i, node) in self.nodes.iter().enumerate() {
            let i = i as u16;
            let (row, col) = match self.direction {
                DiagramDirection::TopDown => (i / cols, i % cols),
                DiagramDirection::BottomUp => (rows - 1 - i / cols, i % cols),
                DiagramDirection::LeftRight => (i % rows, i / rows),
                DiagramDirection::RightLeft => (i % rows, cols - 1 - i / rows),
            };

            // A crowded area can leave a grid cell narrower or shorter than a
            // node; shrink the node or let it overhang instead of
            // underflowing.
            let label_width = crate::utils::unicode::display_width(&node.label) as u16;
            let node_width = (label_width + 4).min(cell_width.saturating_sub(2));
            let node_height = 3u16;

            let x = col * cell_width + cell_width.saturating_sub(node_width) / 2;
            let y = row * cell_height + cell_height.saturating_sub(node_height) / 2;

            self.positions.insert(node.id.clone(), (x, y));
            self.sizes
                .insert(node.id.clone(), (node_width, node_height));
        }
    }
}
