//! Rendering functions for the Diagram widget

use super::core::Diagram;
use super::types::{ArrowStyle, DiagramEdge, NodeShape};
use crate::render::Cell;
use crate::style::Color;
use crate::utils::unicode::{display_width, truncate_to_width};
use crate::widget::traits::{RenderContext, View};

impl View for Diagram {
    crate::impl_view_meta!("Diagram");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 10 || area.height < 5 {
            return;
        }

        // Title
        let title_height = if !self.title.is_empty() {
            let title_fg = self.colors.title;
            ctx.put_str_with(0, 0, &self.title, area.width, |ch| {
                Cell::new(ch).fg(title_fg).bold()
            });
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
            positions: std::collections::HashMap::new(),
            sizes: std::collections::HashMap::new(),
            props: crate::widget::traits::WidgetProps::new(),
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

impl Diagram {
    /// Render a node
    pub(super) fn render_node(
        &self,
        ctx: &mut RenderContext,
        node: &super::types::DiagramNode,
        x: u16,
        y: u16,
        width: u16,
        _height: u16,
    ) {
        let area = ctx.area;
        // A node that names its own color keeps it - that is how a diagram
        // marks one box out from the rest. The arrow color, the label and the
        // title each say something a `color` rule cannot say separately, so the
        // rule gets the ordinary node.
        let fg = node
            .color
            .unwrap_or_else(|| ctx.color_or(self.colors.node_fg, Color::WHITE));
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
                ctx.set(x, y, cell);

                for i in 1..width - 1 {
                    let mut cell = Cell::new(h);
                    cell.fg = Some(fg);
                    ctx.set(x + i, y, cell);
                }

                let mut cell = Cell::new(tr);
                cell.fg = Some(fg);
                ctx.set(x + width - 1, y, cell);

                // Middle (with label)
                let mut cell = Cell::new(v);
                cell.fg = Some(fg);
                ctx.set(x, y + 1, cell);
                ctx.set(x + width - 1, y + 1, cell);

                // Label
                // Centered in columns, and kept inside the side borders: a
                // label wider than its box is cut, not drawn over them.
                let inner = width.saturating_sub(2);
                let label = truncate_to_width(&node.label, inner as usize);
                let label_start = 1 + (inner - display_width(label) as u16) / 2;
                ctx.put_str_with(x + label_start, y + 1, label, x + width - 1, |ch| {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(fg);
                    cell.bg = bg;
                    cell
                });

                // Bottom border
                let mut cell = Cell::new(bl);
                cell.fg = Some(fg);
                ctx.set(x, y + 2, cell);

                for i in 1..width - 1 {
                    let mut cell = Cell::new(h);
                    cell.fg = Some(fg);
                    ctx.set(x + i, y + 2, cell);
                }

                let mut cell = Cell::new(br);
                cell.fg = Some(fg);
                ctx.set(x + width - 1, y + 2, cell);
            }
            NodeShape::Diamond => {
                // Simplified diamond as <>
                let _mid = width / 2;

                let mut cell = Cell::new('<');
                cell.fg = Some(fg);
                ctx.set(x, y + 1, cell);

                ctx.put_str_with(x + 1, y + 1, &node.label, x + width - 1, |ch| {
                    Cell::new(ch).fg(fg)
                });

                let mut cell = Cell::new('>');
                cell.fg = Some(fg);
                ctx.set(x + width - 1, y + 1, cell);
            }
            _ => {
                // Default: just render label
                ctx.put_str_with(x, y, &node.label, area.width, |ch| Cell::new(ch).fg(fg));
            }
        }
    }

    /// Render an edge/arrow
    pub(super) fn render_edge(&self, ctx: &mut RenderContext, edge: &DiagramEdge) {
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
                if y < area.height {
                    let mut cell = Cell::new(arrow_char);
                    cell.fg = Some(self.colors.arrow);
                    ctx.set(start_x, y, cell);
                }
            }

            // Arrow head
            if end_y - 1 < area.height {
                let mut cell = Cell::new('▼');
                cell.fg = Some(self.colors.arrow);
                ctx.set(end_x, end_y - 1, cell);
            }
        }

        // Edge label
        if let Some(ref label) = edge.label {
            let label_y = (start_y + end_y) / 2;
            let label_str = label.as_str();
            let label_x = start_x.saturating_sub(display_width(label_str) as u16 / 2);
            let label_fg = self.colors.label;
            ctx.put_str_with(label_x, label_y, label_str, area.width, |ch| {
                Cell::new(ch).fg(label_fg).italic()
            });
        }
    }
}
