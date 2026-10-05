//! Tree rendering tests, through `View::render`

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::data::tree::{Tree, TreeNode};
use revue::widget::traits::{RenderContext, View};

fn render(tree: &Tree, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let area = Rect::new(0, 0, width, height);
    let mut ctx = RenderContext::new(&mut buffer, area);
    tree.render(&mut ctx);
    buffer
}

/// Render into a `buf_w` x `buf_h` buffer, but give the tree only `area`.
fn render_in(tree: &Tree, buf_w: u16, buf_h: u16, area: Rect) -> Buffer {
    let mut buffer = Buffer::new(buf_w, buf_h);
    let mut ctx = RenderContext::new(&mut buffer, area);
    tree.render(&mut ctx);
    buffer
}

fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).unwrap().symbol)
        .collect::<String>()
        .trim_end()
        .to_string()
}

fn parent_with(children: Vec<TreeNode>) -> Tree {
    Tree::new().node(TreeNode::new("Parent").expanded(true).children(children))
}

// =========================================================================
// Area size
// =========================================================================

#[test]
fn test_tree_render_minimum_size() {
    // Width 3 and height 1 is the smallest area that is drawn
    let tree = Tree::new().node(TreeNode::new("Tree"));
    assert_eq!(row(&render(&tree, 3, 1), 0), " Tr");
}

#[test]
fn test_tree_render_too_narrow_draws_nothing() {
    let tree = Tree::new().node(TreeNode::new("Tree"));
    let buffer = render_in(&tree, 10, 1, Rect::new(0, 0, 2, 1));
    assert_eq!(row(&buffer, 0), "");
}

#[test]
fn test_tree_render_zero_width() {
    let tree = Tree::new().node(TreeNode::new("Test"));
    let buffer = render_in(&tree, 10, 3, Rect::new(0, 0, 0, 3));
    for y in 0..3 {
        assert_eq!(row(&buffer, y), "");
    }
}

#[test]
fn test_tree_render_zero_height() {
    let tree = Tree::new().node(TreeNode::new("Test"));
    let buffer = render_in(&tree, 10, 3, Rect::new(0, 0, 10, 0));
    for y in 0..3 {
        assert_eq!(row(&buffer, y), "");
    }
}

// =========================================================================
// Node rendering
// =========================================================================

#[test]
fn test_tree_render_leaf_node_indicator() {
    let tree = Tree::new().node(TreeNode::new("Leaf"));
    let buffer = render(&tree, 40, 10);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
    assert_eq!(row(&buffer, 0), " Leaf");
}

#[test]
fn test_tree_render_collapsed_node_indicator() {
    let tree = Tree::new().node(TreeNode::new("Parent").child(TreeNode::new("Child")));
    let buffer = render(&tree, 40, 10);
    assert_eq!(row(&buffer, 0), "▶Parent");
    // Collapsed: the child is not drawn
    assert_eq!(row(&buffer, 1), "");
}

#[test]
fn test_tree_render_expanded_node_indicator() {
    let tree = parent_with(vec![TreeNode::new("Child")]);
    let buffer = render(&tree, 40, 10);
    assert_eq!(row(&buffer, 0), "▼Parent");
}

#[test]
fn test_tree_render_node_icon() {
    let tree = Tree::new().node(TreeNode::new("src").icon('D'));
    assert_eq!(row(&render(&tree, 40, 10), 0), " Dsrc");
}

#[test]
fn test_tree_render_label_truncation() {
    // After the indicator, 9 columns are left for the label
    let tree = Tree::new().node(TreeNode::new("VeryLongLabelThatShouldBeTruncated"));
    let buffer = render_in(&tree, 20, 10, Rect::new(0, 0, 10, 10));
    assert_eq!(row(&buffer, 0), " VeryLongL");
}

// =========================================================================
// Tree structure
// =========================================================================

#[test]
fn test_tree_render_single_child() {
    let tree = parent_with(vec![TreeNode::new("Child")]);
    let buffer = render(&tree, 40, 10);
    assert_eq!(row(&buffer, 0), "▼Parent");
    assert_eq!(row(&buffer, 1), "  └─ Child");
}

#[test]
fn test_tree_render_multiple_children() {
    let tree = parent_with(vec![
        TreeNode::new("Child 1"),
        TreeNode::new("Child 2"),
        TreeNode::new("Child 3"),
    ]);
    let buffer = render(&tree, 40, 10);
    assert_eq!(row(&buffer, 0), "▼Parent");
    assert_eq!(row(&buffer, 1), "  ├─ Child 1");
    assert_eq!(row(&buffer, 2), "  ├─ Child 2");
    assert_eq!(row(&buffer, 3), "  └─ Child 3");
}

#[test]
fn test_tree_render_nested_levels() {
    let tree = Tree::new().node(
        TreeNode::new("L0").expanded(true).child(
            TreeNode::new("L1")
                .expanded(true)
                .child(TreeNode::new("L2")),
        ),
    );
    let buffer = render(&tree, 40, 10);
    assert_eq!(row(&buffer, 0), "▼L0");
    assert_eq!(row(&buffer, 1), "  └─▼L1");
    assert_eq!(row(&buffer, 2), "    └─ L2");
}

#[test]
fn test_tree_render_vertical_line_for_open_ancestor() {
    // The first root is not the last one, so its subtree keeps a │ guide
    let tree = Tree::new()
        .node(
            TreeNode::new("A")
                .expanded(true)
                .child(TreeNode::new("B").expanded(true).child(TreeNode::new("C"))),
        )
        .node(TreeNode::new("D"));
    let buffer = render(&tree, 40, 10);
    assert_eq!(row(&buffer, 0), "▼A");
    assert_eq!(row(&buffer, 1), "│ └─▼B");
    assert_eq!(row(&buffer, 2), "│   └─ C");
    assert_eq!(row(&buffer, 3), " D");
}

#[test]
fn test_tree_render_multiple_roots() {
    let tree = Tree::new()
        .node(TreeNode::new("Root 1"))
        .node(TreeNode::new("Root 2"))
        .node(TreeNode::new("Root 3"));
    let buffer = render(&tree, 40, 10);
    assert_eq!(row(&buffer, 0), " Root 1");
    assert_eq!(row(&buffer, 1), " Root 2");
    assert_eq!(row(&buffer, 2), " Root 3");
}

// =========================================================================
// Selection
// =========================================================================

#[test]
fn test_tree_render_selected_node() {
    let tree = Tree::new()
        .nodes(vec![TreeNode::new("First"), TreeNode::new("Second")])
        .selected(1)
        .selected_style(Color::WHITE, Color::BLUE);
    let buffer = render(&tree, 40, 10);

    // The whole selected row is filled
    assert_eq!(buffer.get(0, 1).unwrap().bg, Some(Color::BLUE));
    assert_eq!(buffer.get(39, 1).unwrap().bg, Some(Color::BLUE));
    assert_eq!(buffer.get(0, 0).unwrap().bg, None);
}

#[test]
fn test_tree_render_selected_first() {
    let tree = Tree::new()
        .node(TreeNode::new("Test"))
        .selected(0)
        .selected_style(Color::WHITE, Color::BLUE);
    let buffer = render(&tree, 40, 10);
    assert_eq!(buffer.get(0, 0).unwrap().bg, Some(Color::BLUE));
}

#[test]
fn test_tree_render_selected_colors() {
    let tree = Tree::new()
        .node(TreeNode::new("Test"))
        .selected(0)
        .selected_style(Color::YELLOW, Color::GREEN);
    let cell = *render(&tree, 40, 10).get(1, 0).unwrap();
    assert_eq!(cell.fg, Some(Color::YELLOW));
    assert_eq!(cell.bg, Some(Color::GREEN));
}

// =========================================================================
// Colors
// =========================================================================

#[test]
fn test_tree_render_custom_fg() {
    // Row 0 is selected and uses the selection colours; row 1 uses fg
    let tree = Tree::new()
        .nodes(vec![TreeNode::new("Selected"), TreeNode::new("Test")])
        .fg(Color::RED);
    let buffer = render(&tree, 40, 10);
    assert_eq!(buffer.get(1, 1).unwrap().symbol, 'T');
    assert_eq!(buffer.get(1, 1).unwrap().fg, Some(Color::RED));
}

#[test]
fn test_tree_render_custom_bg() {
    let tree = Tree::new()
        .nodes(vec![TreeNode::new("Selected"), TreeNode::new("Test")])
        .bg(Color::BLACK);
    let buffer = render(&tree, 40, 10);
    assert_eq!(buffer.get(0, 1).unwrap().bg, Some(Color::BLACK));
    assert_eq!(buffer.get(1, 1).unwrap().bg, Some(Color::BLACK));
}

// =========================================================================
// Indent
// =========================================================================

#[test]
fn test_tree_render_default_indent() {
    let tree = parent_with(vec![TreeNode::new("Child")]);
    let buffer = render(&tree, 40, 10);
    // Default indent is 2: the connector starts at column 2
    assert_eq!(row(&buffer, 1), "  └─ Child");
}

#[test]
fn test_tree_render_custom_indent() {
    let tree = Tree::new().indent(4).node(
        TreeNode::new("Parent")
            .expanded(true)
            .child(TreeNode::new("Child")),
    );
    let buffer = render(&tree, 40, 10);
    assert_eq!(row(&buffer, 1), "    └─ Child");
}

#[test]
fn test_tree_render_zero_indent_uses_one_column() {
    let tree = Tree::new().indent(0).node(
        TreeNode::new("Parent")
            .expanded(true)
            .child(TreeNode::new("Child")),
    );
    let buffer = render(&tree, 40, 10);
    assert_eq!(row(&buffer, 1), " └─ Child");
}

// =========================================================================
// Connectors
// =========================================================================

#[test]
fn test_tree_render_lines_single_child() {
    let tree = parent_with(vec![TreeNode::new("Child")]);
    let buffer = render(&tree, 40, 10);
    assert_eq!(buffer.get(2, 1).unwrap().symbol, '└');
    assert_eq!(buffer.get(3, 1).unwrap().symbol, '─');
}

#[test]
fn test_tree_render_lines_multiple_children() {
    let tree = parent_with(vec![TreeNode::new("Child 1"), TreeNode::new("Child 2")]);
    let buffer = render(&tree, 40, 10);
    assert_eq!(buffer.get(2, 1).unwrap().symbol, '├');
    assert_eq!(buffer.get(2, 2).unwrap().symbol, '└');
}

// =========================================================================
// Search highlight
// =========================================================================

#[test]
fn test_tree_render_with_highlight() {
    let mut tree = Tree::new()
        .nodes(vec![TreeNode::new("Hello World")])
        .searchable(true)
        .highlight_fg(Color::YELLOW);
    tree.set_query("hw");

    let m = tree.get_match("Hello World").unwrap();
    assert!(m.indices.contains(&0)); // H
    assert!(m.indices.contains(&6)); // W

    let buffer = render(&tree, 40, 10);
    // Label starts at column 1
    assert_eq!(buffer.get(1, 0).unwrap().symbol, 'H');
    assert_eq!(buffer.get(1, 0).unwrap().fg, Some(Color::YELLOW));
    assert_eq!(buffer.get(7, 0).unwrap().symbol, 'W');
    assert_eq!(buffer.get(7, 0).unwrap().fg, Some(Color::YELLOW));
    assert_ne!(buffer.get(2, 0).unwrap().fg, Some(Color::YELLOW));
}

#[test]
fn test_tree_render_no_highlight_when_no_query() {
    let tree = Tree::new()
        .nodes(vec![TreeNode::new("Test")])
        .searchable(true)
        .highlight_fg(Color::YELLOW);

    assert!(tree.get_match("Test").is_none());
    let buffer = render(&tree, 40, 10);
    for x in 0..40 {
        assert_ne!(buffer.get(x, 0).unwrap().fg, Some(Color::YELLOW), "x {x}");
    }
}

// =========================================================================
// Clipping
// =========================================================================

#[test]
fn test_tree_render_clips_to_area_height() {
    let tree = Tree::new().nodes(
        (1..=5)
            .map(|i| TreeNode::new(format!("Line {i}")))
            .collect(),
    );
    let buffer = render_in(&tree, 40, 10, Rect::new(0, 0, 40, 3));

    assert_eq!(row(&buffer, 0), " Line 1");
    assert_eq!(row(&buffer, 2), " Line 3");
    assert_eq!(row(&buffer, 3), "");
    assert_eq!(row(&buffer, 4), "");
}

#[test]
fn test_tree_render_clips_to_area_width() {
    let tree = Tree::new().node(TreeNode::new("VeryVeryLongLabel"));
    let buffer = render_in(&tree, 40, 10, Rect::new(0, 0, 10, 10));
    assert_eq!(row(&buffer, 0), " VeryVeryL");
}

// =========================================================================
// Edge cases
// =========================================================================

#[test]
fn test_tree_render_empty_tree() {
    let buffer = render(&Tree::new(), 10, 3);
    for y in 0..3 {
        assert_eq!(row(&buffer, y), "");
    }
}

#[test]
fn test_tree_render_empty_label() {
    let tree = Tree::new()
        .node(TreeNode::new(""))
        .node(TreeNode::new("next"));
    let buffer = render(&tree, 40, 10);
    assert_eq!(row(&buffer, 0), "");
    assert_eq!(row(&buffer, 1), " next");
}

#[test]
fn test_tree_render_unicode_label() {
    let tree = Tree::new().node(TreeNode::new("📁 文件夹"));
    let buffer = render(&tree, 40, 10);
    assert_eq!(buffer.get(1, 0).unwrap().symbol, '📁');
    assert_eq!(buffer.get(3, 0).unwrap().symbol, ' ');
    assert_eq!(buffer.get(4, 0).unwrap().symbol, '文');
    assert_eq!(buffer.get(6, 0).unwrap().symbol, '件');
    assert_eq!(buffer.get(8, 0).unwrap().symbol, '夹');
}

#[test]
fn test_tree_render_special_chars_label() {
    let tree = Tree::new().node(TreeNode::new("path/to/file.txt"));
    assert_eq!(row(&render(&tree, 40, 10), 0), " path/to/file.txt");
}

#[test]
fn test_tree_render_very_deep_nesting() {
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
    let buffer = render(&tree, 40, 20);
    assert_eq!(row(&buffer, 4), "        └─ L4");
    assert_eq!(row(&buffer, 5), "");
}

#[test]
fn test_tree_render_many_siblings() {
    let tree = parent_with(
        (0..15)
            .map(|i| TreeNode::new(format!("Child {i}")))
            .collect(),
    );
    let buffer = render(&tree, 40, 20);
    for y in 1..15 {
        assert_eq!(row(&buffer, y), format!("  ├─ Child {}", y - 1));
    }
    assert_eq!(row(&buffer, 15), "  └─ Child 14");
}
