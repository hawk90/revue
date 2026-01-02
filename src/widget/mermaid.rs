//! Mermaid-style diagram rendering in ASCII
//!
//! Renders flowcharts, sequence diagrams, and other diagrams
//! using ASCII/Unicode art.

use super::traits::{RenderContext, View, WidgetProps};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::{impl_props_builders, impl_styled_view};
use std::collections::HashMap;

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
    /// Rectangle \[text\]
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
    title: String,
    /// Diagram type
    diagram_type: DiagramType,
    /// Nodes
    nodes: Vec<DiagramNode>,
    /// Edges
    edges: Vec<DiagramEdge>,
    /// Colors
    colors: DiagramColors,
    /// Direction (TD = top-down, LR = left-right)
    direction: Direction,
    /// Node positions (computed during layout)
    positions: HashMap<String, (u16, u16)>,
    /// Node sizes
    sizes: HashMap<String, (u16, u16)>,
    /// Widget properties
    props: WidgetProps,
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

impl Diagram {
    /// Create a new diagram
    pub fn new() -> Self {
        Self {
            title: String::new(),
            diagram_type: DiagramType::default(),
            nodes: Vec::new(),
            edges: Vec::new(),
            colors: DiagramColors::default(),
            direction: Direction::default(),
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
    pub fn direction(mut self, dir: Direction) -> Self {
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
    fn compute_layout(&mut self, width: u16, height: u16) {
        self.positions.clear();
        self.sizes.clear();

        if self.nodes.is_empty() {
            return;
        }

        // Simple grid layout
        let rows = ((self.nodes.len() as f32).sqrt().ceil() as u16).max(1);
        let cols = (self.nodes.len() as u16).div_ceil(rows).max(1);

        let cell_width = width / cols;
        let cell_height = height / rows;

        for (i, node) in self.nodes.iter().enumerate() {
            let row = i as u16 / cols;
            let col = i as u16 % cols;

            let node_width = (node.label.chars().count() as u16 + 4).min(cell_width - 2);
            let node_height = 3u16;

            let x = col * cell_width + (cell_width - node_width) / 2;
            let y = row * cell_height + (cell_height - node_height) / 2;

            self.positions.insert(node.id.clone(), (x, y));
            self.sizes
                .insert(node.id.clone(), (node_width, node_height));
        }
    }

    /// Render a node
    fn render_node(
        &self,
        ctx: &mut RenderContext,
        node: &DiagramNode,
        x: u16,
        y: u16,
        width: u16,
        _height: u16,
    ) {
        let area = ctx.area;
        let fg = node.color.unwrap_or(self.colors.node_fg);
        let bg = node.bg.or(Some(self.colors.node_bg));

        // Draw box based on shape
        match node.shape {
            NodeShape::Rectangle | NodeShape::Rounded => {
                let (tl, tr, bl, br, h, v) = if node.shape == NodeShape::Rounded {
                    ('╭', '╮', '╰', '╯', '─', '│')
                } else {
                    ('┌', '┐', '└', '┘', '─', '│')
                };

                // Top border
                let mut cell = Cell::new(tl);
                cell.fg = Some(fg);
                ctx.buffer.set(area.x + x, area.y + y, cell);

                for i in 1..width - 1 {
                    let mut cell = Cell::new(h);
                    cell.fg = Some(fg);
                    ctx.buffer.set(area.x + x + i, area.y + y, cell);
                }

                let mut cell = Cell::new(tr);
                cell.fg = Some(fg);
                ctx.buffer.set(area.x + x + width - 1, area.y + y, cell);

                // Middle (with label)
                let mut cell = Cell::new(v);
                cell.fg = Some(fg);
                ctx.buffer.set(area.x + x, area.y + y + 1, cell.clone());
                ctx.buffer.set(area.x + x + width - 1, area.y + y + 1, cell);

                // Label
                let label_start = (width as usize - node.label.chars().count()) / 2;
                for (i, ch) in node.label.chars().enumerate() {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(fg);
                    cell.bg = bg;
                    ctx.buffer.set(
                        area.x + x + label_start as u16 + i as u16,
                        area.y + y + 1,
                        cell,
                    );
                }

                // Bottom border
                let mut cell = Cell::new(bl);
                cell.fg = Some(fg);
                ctx.buffer.set(area.x + x, area.y + y + 2, cell);

                for i in 1..width - 1 {
                    let mut cell = Cell::new(h);
                    cell.fg = Some(fg);
                    ctx.buffer.set(area.x + x + i, area.y + y + 2, cell);
                }

                let mut cell = Cell::new(br);
                cell.fg = Some(fg);
                ctx.buffer.set(area.x + x + width - 1, area.y + y + 2, cell);
            }
            NodeShape::Diamond => {
                // Simplified diamond as <>
                let _mid = width / 2;

                let mut cell = Cell::new('<');
                cell.fg = Some(fg);
                ctx.buffer.set(area.x + x, area.y + y + 1, cell);

                for (i, ch) in node.label.chars().enumerate() {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(fg);
                    ctx.buffer
                        .set(area.x + x + 1 + i as u16, area.y + y + 1, cell);
                }

                let mut cell = Cell::new('>');
                cell.fg = Some(fg);
                ctx.buffer.set(area.x + x + width - 1, area.y + y + 1, cell);
            }
            _ => {
                // Default: just render label
                for (i, ch) in node.label.chars().enumerate() {
                    if x + i as u16 >= area.width {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(fg);
                    ctx.buffer.set(area.x + x + i as u16, area.y + y, cell);
                }
            }
        }
    }

    /// Render an edge/arrow
    fn render_edge(&self, ctx: &mut RenderContext, edge: &DiagramEdge) {
        let area = ctx.area;

        let Some(&(x1, y1)) = self.positions.get(&edge.from) else {
            return;
        };
        let Some(&(w1, h1)) = self.sizes.get(&edge.from) else {
            return;
        };
        let Some(&(x2, y2)) = self.positions.get(&edge.to) else {
            return;
        };
        let Some(&(w2, _h2)) = self.sizes.get(&edge.to) else {
            return;
        };

        // Simple arrow: draw from bottom of source to top of target
        let start_x = x1 + w1 / 2;
        let start_y = y1 + h1;
        let end_x = x2 + w2 / 2;
        let end_y = y2;

        let arrow_char = match edge.style {
            ArrowStyle::Solid => '│',
            ArrowStyle::Dashed => '┊',
            ArrowStyle::Thick => '┃',
            ArrowStyle::Line => '│',
        };

        // Vertical line
        if start_y < end_y {
            for y in start_y..end_y {
                if area.y + y < area.y + area.height {
                    let mut cell = Cell::new(arrow_char);
                    cell.fg = Some(self.colors.arrow);
                    ctx.buffer.set(area.x + start_x, area.y + y, cell);
                }
            }

            // Arrow head
            if area.y + end_y - 1 < area.y + area.height {
                let mut cell = Cell::new('▼');
                cell.fg = Some(self.colors.arrow);
                ctx.buffer.set(area.x + end_x, area.y + end_y - 1, cell);
            }
        }

        // Edge label
        if let Some(ref label) = edge.label {
            let label_y = (start_y + end_y) / 2;
            let label_x = start_x.saturating_sub(label.chars().count() as u16 / 2);
            for (i, ch) in label.chars().enumerate() {
                if area.x + label_x + i as u16 >= area.x + area.width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(self.colors.label);
                cell.modifier = Modifier::ITALIC;
                ctx.buffer
                    .set(area.x + label_x + i as u16, area.y + label_y, cell);
            }
        }
    }
}

impl Default for Diagram {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Diagram {
    crate::impl_view_meta!("Diagram");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 10 || area.height < 5 {
            return;
        }

        // Title
        let title_height = if !self.title.is_empty() {
            for (i, ch) in self.title.chars().enumerate() {
                if i as u16 >= area.width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(self.colors.title);
                cell.modifier = Modifier::BOLD;
                ctx.buffer.set(area.x + i as u16, area.y, cell);
            }
            2u16
        } else {
            0u16
        };

        // Create mutable copy for layout computation
        let mut diagram = Diagram {
            title: self.title.clone(),
            diagram_type: self.diagram_type,
            nodes: self.nodes.clone(),
            edges: self.edges.clone(),
            colors: self.colors.clone(),
            direction: self.direction,
            positions: HashMap::new(),
            sizes: HashMap::new(),
            props: WidgetProps::new(),
        };

        diagram.compute_layout(area.width, area.height - title_height);

        // Render edges first (behind nodes)
        for edge in &diagram.edges {
            diagram.render_edge(ctx, edge);
        }

        // Render nodes
        for node in &diagram.nodes {
            if let (Some(&(x, y)), Some(&(w, h))) =
                (diagram.positions.get(&node.id), diagram.sizes.get(&node.id))
            {
                diagram.render_node(ctx, node, x, y + title_height, w, h);
            }
        }
    }
}

impl_styled_view!(Diagram);
impl_props_builders!(Diagram);

/// Create a new diagram
pub fn diagram() -> Diagram {
    Diagram::new()
}

/// Create a flowchart from mermaid-like syntax
pub fn flowchart(source: &str) -> Diagram {
    Diagram::new()
        .diagram_type(DiagramType::Flowchart)
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
    use crate::layout::Rect;
    use crate::render::Buffer;

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

    #[test]
    fn test_parse_mermaid() {
        let diag = flowchart("A[Start] --> B[Process]\nB --> C{Decision}");
        assert_eq!(diag.nodes.len(), 3);
        assert_eq!(diag.edges.len(), 2);
    }

    #[test]
    fn test_node_shapes() {
        let n = node("A", "Test")
            .shape(NodeShape::Diamond)
            .color(Color::CYAN);
        assert_eq!(n.shape, NodeShape::Diamond);
    }

    #[test]
    fn test_diagram_render() {
        let diag = diagram()
            .node(node("A", "Hello"))
            .node(node("B", "World"))
            .edge(edge("A", "B"));

        let mut buffer = Buffer::new(60, 20);
        let area = Rect::new(0, 0, 60, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        diag.render(&mut ctx);
    }
}
