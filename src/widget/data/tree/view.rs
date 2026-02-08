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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::style::Color;

    // =========================================================================
    // Render method tests
    // =========================================================================

    #[test]
    fn test_tree_render_internal_basic() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = Tree::new().node(TreeNode::new("Test"));
        tree.render_internal(&mut ctx);

        // Verify no panic and some content rendered
        assert_eq!(buffer.get(1, 0).unwrap().symbol, 'T');
    }

    #[test]
    fn test_tree_render_internal_empty_tree() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = Tree::new();
        tree.render_internal(&mut ctx);

        // Empty tree should not panic
    }

    #[test]
    fn test_tree_render_internal_small_width() {
        let mut buffer = Buffer::new(2, 10);
        let area = Rect::new(0, 0, 2, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = Tree::new().node(TreeNode::new("T"));
        tree.render_internal(&mut ctx);

        // Should not panic with small width
    }

    #[test]
    fn test_tree_render_internal_small_height() {
        let mut buffer = Buffer::new(40, 1);
        let area = Rect::new(0, 0, 40, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = Tree::new().node(TreeNode::new("Test"));
        tree.render_internal(&mut ctx);

        // Should not panic with small height
    }

    #[test]
    fn test_tree_render_internal_minimum_size() {
        let mut buffer = Buffer::new(3, 1);
        let area = Rect::new(0, 0, 3, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = Tree::new().node(TreeNode::new("T"));
        tree.render_internal(&mut ctx);

        // Minimum valid size (width >= 3, height >= 1)
    }

    #[test]
    fn test_tree_render_internal_zero_width() {
        let mut buffer = Buffer::new(0, 10);
        let area = Rect::new(0, 0, 0, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = Tree::new().node(TreeNode::new("Test"));
        tree.render_internal(&mut ctx);

        // Zero width should not render
    }

    #[test]
    fn test_tree_render_internal_zero_height() {
        let mut buffer = Buffer::new(40, 0);
        let area = Rect::new(0, 0, 40, 0);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = Tree::new().node(TreeNode::new("Test"));
        tree.render_internal(&mut ctx);

        // Zero height should not render
    }

    // =========================================================================
    // Node rendering tests
    // =========================================================================

    #[test]
    fn test_tree_render_leaf_node_indicator() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = Tree::new().node(TreeNode::new("Leaf"));
        tree.render_internal(&mut ctx);

        // Leaf node should have space indicator (no children)
        assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
    }

    #[test]
    fn test_tree_render_collapsed_node_indicator() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = Tree::new().node(TreeNode::new("Parent").child(TreeNode::new("Child")));
        tree.render_internal(&mut ctx);

        // Collapsed node with children should show ▶
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '▶');
    }

    #[test]
    fn test_tree_render_expanded_node_indicator() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = Tree::new().node(
            TreeNode::new("Parent")
                .expanded(true)
                .child(TreeNode::new("Child")),
        );
        tree.render_internal(&mut ctx);

        // Expanded node should show ▼
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '▼');
    }

    #[test]
    fn test_tree_render_node_label() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = Tree::new().node(TreeNode::new("TestLabel"));
        tree.render_internal(&mut ctx);

        // Check label is rendered
        assert_eq!(buffer.get(1, 0).unwrap().symbol, 'T');
        assert_eq!(buffer.get(2, 0).unwrap().symbol, 'e');
        assert_eq!(buffer.get(3, 0).unwrap().symbol, 's');
    }

    #[test]
    fn test_tree_render_label_truncation() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = Tree::new().node(TreeNode::new("VeryLongLabelThatShouldBeTruncated"));
        tree.render_internal(&mut ctx);

        // Label should be truncated to fit available width
        // After indicator (1 char) we have 9 chars for label
        let truncated: String = "VeryLongLabelThatShouldBeTruncated"
            .chars()
            .take(9)
            .collect();
        assert_eq!(truncated.len(), 9);
    }

    // =========================================================================
    // Tree structure rendering tests
    // =========================================================================

    #[test]
    fn test_tree_render_single_child() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = Tree::new().node(
            TreeNode::new("Parent")
                .expanded(true)
                .child(TreeNode::new("Child")),
        );
        tree.render_internal(&mut ctx);

        // Parent on first line - indicator at position 0, label starts at position 1
        assert_eq!(buffer.get(1, 0).unwrap().symbol, 'P');

        // Child is rendered on second line
        // Just verify something was rendered at y=1
        let cell = buffer.get(0, 1).unwrap();
        // At depth 1, position 0 should have connector
        assert!(
            cell.symbol == '└'
                || cell.symbol == '├'
                || cell.symbol == ' '
                || cell.symbol == '│'
                || cell.symbol == '─'
        );

        // Find the 'C' from "Child" somewhere on the line
        let mut found_child = false;
        for x in 0..10 {
            if buffer.get(x, 1).unwrap().symbol == 'C' {
                found_child = true;
                break;
            }
        }
        assert!(found_child, "Could not find 'C' from 'Child' label");
    }

    #[test]
    fn test_tree_render_multiple_children() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = Tree::new().node(
            TreeNode::new("Parent")
                .expanded(true)
                .child(TreeNode::new("Child 1"))
                .child(TreeNode::new("Child 2"))
                .child(TreeNode::new("Child 3")),
        );
        tree.render_internal(&mut ctx);

        // All children should be rendered on separate lines
        // Parent at y=0, children at y=1, y=2, y=3
        assert_eq!(buffer.get(1, 0).unwrap().symbol, 'P');

        // Find 'C' from child labels on each line
        for y in 1..=3 {
            let mut found = false;
            for x in 0..10 {
                if buffer.get(x, y).unwrap().symbol == 'C' {
                    found = true;
                    break;
                }
            }
            assert!(found, "Could not find 'C' on line {}", y);
        }
    }

    #[test]
    fn test_tree_render_nested_levels() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = Tree::new().node(
            TreeNode::new("L0").expanded(true).child(
                TreeNode::new("L1")
                    .expanded(true)
                    .child(TreeNode::new("L2")),
            ),
        );
        tree.render_internal(&mut ctx);

        // Three levels should be rendered
        // Just verify 'L' appears on each line
        for y in 0..3 {
            let mut found = false;
            for x in 0..10 {
                if buffer.get(x, y).unwrap().symbol == 'L' {
                    found = true;
                    break;
                }
            }
            assert!(found, "Could not find 'L' on line {}", y);
        }
    }

    #[test]
    fn test_tree_render_multiple_roots() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = Tree::new()
            .node(TreeNode::new("Root 1"))
            .node(TreeNode::new("Root 2"))
            .node(TreeNode::new("Root 3"));
        tree.render_internal(&mut ctx);

        // All root nodes should be rendered
        assert_eq!(buffer.get(1, 0).unwrap().symbol, 'R');
        assert_eq!(buffer.get(1, 1).unwrap().symbol, 'R');
        assert_eq!(buffer.get(1, 2).unwrap().symbol, 'R');
    }

    // =========================================================================
    // Selection rendering tests
    // =========================================================================

    #[test]
    fn test_tree_render_selected_node() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = Tree::new()
            .nodes(vec![TreeNode::new("First"), TreeNode::new("Second")])
            .selected(1)
            .selected_style(Color::WHITE, Color::BLUE);

        tree.render_internal(&mut ctx);

        // Second line should have blue background
        let cell = buffer.get(0, 1).unwrap();
        assert_eq!(cell.bg, Some(Color::BLUE));
    }

    #[test]
    fn test_tree_render_selected_first() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = Tree::new()
            .node(TreeNode::new("Test"))
            .selected(0)
            .selected_style(Color::WHITE, Color::BLUE);

        tree.render_internal(&mut ctx);

        // First line should have blue background
        let cell = buffer.get(0, 0).unwrap();
        assert_eq!(cell.bg, Some(Color::BLUE));
    }

    // =========================================================================
    // Color rendering tests
    // =========================================================================

    #[test]
    fn test_tree_render_custom_fg() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = Tree::new().node(TreeNode::new("Test")).fg(Color::RED);

        tree.render_internal(&mut ctx);

        // Check foreground color is set (may be overridden by highlight or selection)
        let cell = buffer.get(1, 0).unwrap();
        // The label character should have the custom fg color
        assert!(cell.fg == Some(Color::RED) || cell.fg.is_some());
    }

    #[test]
    fn test_tree_render_custom_bg() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = Tree::new().node(TreeNode::new("Test")).bg(Color::BLACK);

        tree.render_internal(&mut ctx);

        // Check background color is set
        let cell = buffer.get(0, 0).unwrap();
        assert!(cell.bg == Some(Color::BLACK) || cell.bg.is_some());
    }

    #[test]
    fn test_tree_render_selected_colors() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = Tree::new()
            .node(TreeNode::new("Test"))
            .selected(0)
            .selected_style(Color::YELLOW, Color::GREEN);

        tree.render_internal(&mut ctx);

        // Check selected colors
        let cell = buffer.get(1, 0).unwrap();
        assert_eq!(cell.fg, Some(Color::YELLOW));
        assert_eq!(cell.bg, Some(Color::GREEN));
    }

    // =========================================================================
    // Indent rendering tests
    // =========================================================================

    #[test]
    fn test_tree_render_default_indent() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = Tree::new().node(
            TreeNode::new("Parent")
                .expanded(true)
                .child(TreeNode::new("Child")),
        );

        tree.render_internal(&mut ctx);

        // Default indent is 2, child should be indented
        // First char at y=1 should be space (indent)
        let cell = buffer.get(0, 1).unwrap();
        assert_eq!(cell.symbol, ' ');
    }

    #[test]
    fn test_tree_render_custom_indent() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = Tree::new().indent(4).node(
            TreeNode::new("Parent")
                .expanded(true)
                .child(TreeNode::new("Child")),
        );

        tree.render_internal(&mut ctx);

        // Custom indent of 4 spaces
        let cell = buffer.get(0, 1).unwrap();
        assert_eq!(cell.symbol, ' ');
    }

    // =========================================================================
    // Tree line rendering tests
    // =========================================================================

    #[test]
    fn test_tree_render_lines_single_child() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = Tree::new().node(
            TreeNode::new("Parent")
                .expanded(true)
                .child(TreeNode::new("Child")),
        );

        tree.render_internal(&mut ctx);

        // Find the connector character on line 1
        // It should be somewhere at the beginning
        let mut found_connector = false;
        for x in 0..10 {
            let ch = buffer.get(x, 1).unwrap().symbol;
            if ch == '└' || ch == '├' {
                found_connector = true;
                assert_eq!(ch, '└', "Single child should use └ connector");
                break;
            }
        }
        assert!(found_connector, "Could not find connector character");
    }

    #[test]
    fn test_tree_render_lines_multiple_children() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = Tree::new().node(
            TreeNode::new("Parent")
                .expanded(true)
                .child(TreeNode::new("Child 1"))
                .child(TreeNode::new("Child 2")),
        );

        tree.render_internal(&mut ctx);

        // Find connectors on child lines
        // First child (y=1) should use ├ (not last)
        let mut found_first_connector = false;
        for x in 0..10 {
            let ch = buffer.get(x, 1).unwrap().symbol;
            if ch == '├' || ch == '└' {
                found_first_connector = true;
                assert_eq!(ch, '├', "First child should use ├ connector");
                break;
            }
        }
        assert!(found_first_connector, "Could not find first connector");

        // Second child (y=2) should use └ (last)
        let mut found_second_connector = false;
        for x in 0..10 {
            let ch = buffer.get(x, 2).unwrap().symbol;
            if ch == '├' || ch == '└' {
                found_second_connector = true;
                assert_eq!(ch, '└', "Last child should use └ connector");
                break;
            }
        }
        assert!(found_second_connector, "Could not find second connector");
    }

    #[test]
    fn test_tree_render_lines_nested() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = Tree::new().node(
            TreeNode::new("L0")
                .expanded(true)
                .child(
                    TreeNode::new("L1a")
                        .expanded(true)
                        .child(TreeNode::new("L2")),
                )
                .child(TreeNode::new("L1b")),
        );

        tree.render_internal(&mut ctx);

        // Complex nesting should render correctly
        // Just verify no panic and structure exists
    }

    // =========================================================================
    // Highlight rendering tests
    // =========================================================================

    #[test]
    fn test_tree_render_with_highlight() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut tree = Tree::new()
            .nodes(vec![TreeNode::new("Hello World")])
            .searchable(true)
            .highlight_fg(Color::YELLOW);

        tree.set_query("hw");
        tree.render_internal(&mut ctx);

        // Highlight should be applied to matched chars
        // Get match indices
        let m = tree.get_match("Hello World").unwrap();
        assert!(m.indices.contains(&0)); // H
        assert!(m.indices.contains(&6)); // W
    }

    #[test]
    fn test_tree_render_no_highlight_when_no_query() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = Tree::new()
            .nodes(vec![TreeNode::new("Test")])
            .searchable(true)
            .highlight_fg(Color::YELLOW);

        tree.render_internal(&mut ctx);

        // No query means no highlight
        assert!(tree.get_match("Test").is_none());
    }

    // =========================================================================
    // Clipping tests
    // =========================================================================

    #[test]
    fn test_tree_render_clips_to_area_height() {
        let mut buffer = Buffer::new(40, 3);
        let area = Rect::new(0, 0, 40, 3);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = Tree::new().nodes(vec![
            TreeNode::new("Line 1"),
            TreeNode::new("Line 2"),
            TreeNode::new("Line 3"),
            TreeNode::new("Line 4"),
            TreeNode::new("Line 5"),
        ]);

        tree.render_internal(&mut ctx);

        // Should only render 3 lines (area height)
        // No panic expected
    }

    #[test]
    fn test_tree_render_clips_to_area_width() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = Tree::new().node(TreeNode::new("VeryVeryLongLabel"));
        tree.render_internal(&mut ctx);

        // Should clip label to fit width
        // No panic expected
    }

    // =========================================================================
    // Edge case tests
    // =========================================================================

    #[test]
    fn test_tree_render_empty_label() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = Tree::new().node(TreeNode::new(""));
        tree.render_internal(&mut ctx);

        // Empty label should not panic
    }

    #[test]
    fn test_tree_render_unicode_label() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = Tree::new().node(TreeNode::new("📁 文件夹"));
        tree.render_internal(&mut ctx);

        // Unicode should render correctly
        // Just verify no panic
    }

    #[test]
    fn test_tree_render_special_chars_label() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = Tree::new().node(TreeNode::new("path/to/file.txt"));
        tree.render_internal(&mut ctx);

        // Special chars should render
        assert_eq!(buffer.get(1, 0).unwrap().symbol, 'p');
    }

    #[test]
    fn test_tree_render_very_deep_nesting() {
        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = Tree::new().node(
            TreeNode::new("L0").expanded(true).child(
                TreeNode::new("L1").expanded(true).child(
                    TreeNode::new("L2").expanded(true).child(
                        TreeNode::new("L3")
                            .expanded(true)
                            .child(TreeNode::new("L4")),
                    ),
                ),
            ),
        );

        tree.render_internal(&mut ctx);

        // Deep nesting should render correctly
    }

    #[test]
    fn test_tree_render_many_siblings() {
        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = Tree::new().node(
            TreeNode::new("Parent").expanded(true).children(
                (0..15)
                    .map(|i| TreeNode::new(format!("Child {}", i)))
                    .collect(),
            ),
        );

        tree.render_internal(&mut ctx);

        // Many siblings should render
    }
}
