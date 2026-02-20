//! Tree widget rendering

use super::types::TreeNode;
use super::Tree;
use crate::render::Cell;
use crate::widget::traits::RenderContext;

impl Tree {
    /// Internal render implementation
    pub fn render_internal(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 3 || area.height < 1 || self.root.is_empty() {
            return;
        }

        let mut y = area.y;
        let mut visible_index = 0;

        fn render_nodes(
            tree: &Tree,
            nodes: &[TreeNode],
            ctx: &mut RenderContext,
            y: &mut u16,
            visible_index: &mut usize,
            depth: usize,
            is_last_stack: &[bool],
        ) {
            let area = ctx.area;

            for (i, node) in nodes.iter().enumerate() {
                if *y >= area.y + area.height {
                    return;
                }

                let is_selected = *visible_index == tree.selection.index;
                let is_last = i == nodes.len() - 1;

                let (fg, bg) = if is_selected {
                    (tree.selected_fg, tree.selected_bg)
                } else {
                    (tree.fg, tree.bg)
                };

                // Draw background if selected
                if is_selected {
                    for x in area.x..area.x + area.width {
                        let mut cell = Cell::new(' ');
                        cell.bg = bg;
                        ctx.buffer.set(x, *y, cell);
                    }
                }

                let mut x = area.x;

                // Draw tree lines for depth
                for (d, &parent_is_last) in is_last_stack.iter().enumerate() {
                    if d < depth {
                        let ch = if parent_is_last { ' ' } else { '│' };
                        let mut cell = Cell::new(ch);
                        cell.fg = fg;
                        cell.bg = bg;
                        ctx.buffer.set(x, *y, cell);
                        x += 1;
                        // Add spacing
                        for _ in 1..tree.indent {
                            let mut cell = Cell::new(' ');
                            cell.bg = bg;
                            ctx.buffer.set(x, *y, cell);
                            x += 1;
                        }
                    }
                }

                // Draw current node connector
                if depth > 0 {
                    let connector = if is_last { '└' } else { '├' };
                    let mut cell = Cell::new(connector);
                    cell.fg = fg;
                    cell.bg = bg;
                    ctx.buffer.set(x, *y, cell);
                    x += 1;

                    // Draw horizontal line
                    let mut cell = Cell::new('─');
                    cell.fg = fg;
                    cell.bg = bg;
                    ctx.buffer.set(x, *y, cell);
                    x += 1;
                }

                // Draw expand/collapse indicator
                let indicator = if node.has_children() {
                    if node.expanded {
                        '▼'
                    } else {
                        '▶'
                    }
                } else {
                    ' '
                };
                let mut cell = Cell::new(indicator);
                cell.fg = fg;
                cell.bg = bg;
                ctx.buffer.set(x, *y, cell);
                x += 1;

                // Draw icon if present
                if let Some(icon_ch) = node.icon {
                    let mut icon_cell = Cell::new(icon_ch);
                    icon_cell.fg = fg;
                    icon_cell.bg = bg;
                    ctx.buffer.set(x, *y, icon_cell);
                    x += 1;
                }

                // Draw label with optional highlighting
                let available_width = (area.x + area.width).saturating_sub(x) as usize;
                let truncated: String = node.label.chars().take(available_width).collect();

                // Get match indices for highlighting
                let match_indices: Vec<usize> = tree
                    .get_match(&node.label)
                    .map(|m| m.indices)
                    .unwrap_or_default();

                for (idx, ch) in truncated.chars().enumerate() {
                    let mut cell = Cell::new(ch);
                    cell.bg = bg;

                    // Highlight matched characters
                    if match_indices.contains(&idx) {
                        cell.fg = tree.highlight_fg;
                    } else {
                        cell.fg = fg;
                    }

                    ctx.buffer.set(x, *y, cell);
                    x += 1;
                }

                *y += 1;
                *visible_index += 1;

                // Render children if expanded
                if node.expanded && !node.children.is_empty() {
                    let mut new_stack = is_last_stack.to_vec();
                    new_stack.push(is_last);
                    render_nodes(
                        tree,
                        &node.children,
                        ctx,
                        y,
                        visible_index,
                        depth + 1,
                        &new_stack,
                    );
                }
            }
        }

        render_nodes(self, &self.root, ctx, &mut y, &mut visible_index, 0, &[]);
    }
}

// Tests extracted to:
// - tests/widget/data/tree_view.rs (public API tests)
// - tests/widget/data/tree_search.rs (search functionality tests)

// Private tests that need access to internal details would go here
// KEEP HERE: Tests for Tree::render_internal() and other private methods
// Most tests have been moved to separate test files in tests/widget/data/
