//! Layout computation orchestration
//!
//! Main entry point for computing layout across the tree.

use super::node::ComputedLayout;
use super::tree::LayoutTree;
use super::{block, flex, grid, position};
use crate::style::Display;

/// Compute layout for the entire tree starting from root
///
/// # Arguments
/// * `tree` - The layout tree to compute
/// * `root_id` - Root node ID
/// * `width` - Available width
/// * `height` - Available height
pub fn compute_layout(tree: &mut LayoutTree, root_id: u64, width: u16, height: u16) {
    // Set root node size and position
    if let Some(root) = tree.get_mut(root_id) {
        root.computed = ComputedLayout::new(0, 0, width, height);
    }

    // Recursively compute layout
    compute_node(tree, root_id, width, height, (width, height));
}

/// Compute layout for a single node and its descendants
fn compute_node(
    tree: &mut LayoutTree,
    node_id: u64,
    available_width: u16,
    available_height: u16,
    viewport: (u16, u16),
) {
    let node = match tree.get(node_id) {
        Some(n) => n,
        None => return,
    };

    // Skip Display::None nodes entirely
    if node.display == Display::None {
        // Set zero size for hidden nodes
        if let Some(node_mut) = tree.get_mut(node_id) {
            node_mut.computed = ComputedLayout::default();
        }
        return;
    }

    let display = node.display;
    let children: Vec<u64> = node.children.clone();

    // Compute this node's children layout based on display mode
    match display {
        Display::Flex => {
            flex::compute_flex(tree, node_id, available_width, available_height);
        }
        Display::Block => {
            block::compute_block(tree, node_id, available_width, available_height);
        }
        Display::Grid => {
            grid::compute_grid(tree, node_id, available_width, available_height);
        }
        Display::None => {
            // Already handled above, but included for completeness
            return;
        }
    }

    // Get parent layout for position calculations
    let parent_layout = tree.get(node_id).map(|n| n.computed).unwrap_or_default();

    // Recursively compute children, then apply position offsets
    for &child_id in &children {
        let child_layout = tree.get(child_id).map(|c| c.computed).unwrap_or_default();

        // Recursively compute grandchildren
        compute_node(
            tree,
            child_id,
            child_layout.width,
            child_layout.height,
            viewport,
        );

        // Apply position offsets after children are laid out
        if let Some(child_mut) = tree.get_mut(child_id) {
            position::apply_position_offsets(child_mut, parent_layout, viewport);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::node::LayoutNode;
    use crate::style::{FlexDirection, Size};

    fn setup_simple_tree() -> (LayoutTree, u64) {
        let mut tree = LayoutTree::new();

        let mut root = LayoutNode::default();
        root.id = 1;
        root.display = Display::Flex;

        let mut child1 = LayoutNode::default();
        child1.id = 2;
        child1.sizing.width = Size::Fixed(50);
        child1.sizing.height = Size::Fixed(30);

        let mut child2 = LayoutNode::default();
        child2.id = 3;
        child2.sizing.width = Size::Fixed(50);
        child2.sizing.height = Size::Fixed(30);

        root.children = vec![2, 3];

        tree.insert(root);
        tree.insert(child1);
        tree.insert(child2);
        tree.set_root(1);

        (tree, 1)
    }

    #[test]
    fn test_compute_layout_basic() {
        let (mut tree, root_id) = setup_simple_tree();

        compute_layout(&mut tree, root_id, 200, 100);

        let root = tree.get(root_id).unwrap();
        assert_eq!(root.computed.width, 200);
        assert_eq!(root.computed.height, 100);

        let child1 = tree.get(2).unwrap();
        assert_eq!(child1.computed.x, 0);
        assert_eq!(child1.computed.width, 50);

        let child2 = tree.get(3).unwrap();
        assert_eq!(child2.computed.x, 50);
        assert_eq!(child2.computed.width, 50);
    }

    #[test]
    fn test_compute_layout_nested() {
        let mut tree = LayoutTree::new();

        // Root (flex row)
        let mut root = LayoutNode::default();
        root.id = 1;
        root.display = Display::Flex;
        root.flex.direction = FlexDirection::Row;
        root.children = vec![2, 3];

        // Child 1 (flex column container)
        let mut child1 = LayoutNode::default();
        child1.id = 2;
        child1.display = Display::Flex;
        child1.flex.direction = FlexDirection::Column;
        child1.sizing.width = Size::Fixed(50);
        child1.sizing.height = Size::Auto;
        child1.children = vec![4, 5];

        // Child 2 (leaf)
        let mut child2 = LayoutNode::default();
        child2.id = 3;
        child2.sizing.width = Size::Auto;
        child2.sizing.height = Size::Auto;

        // Grandchildren
        let mut grandchild1 = LayoutNode::default();
        grandchild1.id = 4;
        grandchild1.sizing.height = Size::Fixed(20);

        let mut grandchild2 = LayoutNode::default();
        grandchild2.id = 5;
        grandchild2.sizing.height = Size::Fixed(20);

        tree.insert(root);
        tree.insert(child1);
        tree.insert(child2);
        tree.insert(grandchild1);
        tree.insert(grandchild2);
        tree.set_root(1);

        compute_layout(&mut tree, 1, 100, 100);

        // Check grandchildren are laid out
        let gc1 = tree.get(4).unwrap();
        assert_eq!(gc1.computed.height, 20);
        assert_eq!(gc1.computed.y, 0);

        let gc2 = tree.get(5).unwrap();
        assert_eq!(gc2.computed.height, 20);
        assert_eq!(gc2.computed.y, 20);
    }

    #[test]
    fn test_display_none_hidden() {
        let mut tree = LayoutTree::new();

        let mut root = LayoutNode::default();
        root.id = 1;
        root.display = Display::Flex;
        root.children = vec![2];

        let mut child = LayoutNode::default();
        child.id = 2;
        child.display = Display::None;
        child.sizing.width = Size::Fixed(100);
        child.sizing.height = Size::Fixed(100);

        tree.insert(root);
        tree.insert(child);
        tree.set_root(1);

        compute_layout(&mut tree, 1, 200, 200);

        let child = tree.get(2).unwrap();
        assert_eq!(child.computed.width, 0);
        assert_eq!(child.computed.height, 0);
    }

    #[test]
    fn test_mixed_display_modes() {
        let mut tree = LayoutTree::new();

        // Root (flex)
        let mut root = LayoutNode::default();
        root.id = 1;
        root.display = Display::Flex;
        root.children = vec![2];

        // Child (block container)
        let mut block_container = LayoutNode::default();
        block_container.id = 2;
        block_container.display = Display::Block;
        block_container.sizing.width = Size::Auto;
        block_container.sizing.height = Size::Auto;
        block_container.children = vec![3, 4];

        // Block children
        let mut block_child1 = LayoutNode::default();
        block_child1.id = 3;
        block_child1.sizing.height = Size::Fixed(20);

        let mut block_child2 = LayoutNode::default();
        block_child2.id = 4;
        block_child2.sizing.height = Size::Fixed(30);

        tree.insert(root);
        tree.insert(block_container);
        tree.insert(block_child1);
        tree.insert(block_child2);
        tree.set_root(1);

        compute_layout(&mut tree, 1, 100, 100);

        // Block children should stack vertically
        let bc1 = tree.get(3).unwrap();
        assert_eq!(bc1.computed.y, 0);

        let bc2 = tree.get(4).unwrap();
        assert_eq!(bc2.computed.y, 20);
    }

    #[test]
    fn test_deep_nesting_stress() {
        let mut tree = LayoutTree::new();

        // Create 10 levels of nested flex containers
        let depth = 10;
        for i in 1..=depth {
            let mut node = LayoutNode::default();
            node.id = i as u64;
            node.display = Display::Flex;
            node.flex.direction = if i % 2 == 0 {
                FlexDirection::Row
            } else {
                FlexDirection::Column
            };
            if i < depth {
                node.children = vec![(i + 1) as u64];
            }
            node.sizing.width = Size::Auto;
            node.sizing.height = Size::Auto;
            tree.insert(node);
        }
        tree.set_root(1);

        // Should not panic or stack overflow
        compute_layout(&mut tree, 1, 100, 100);

        // Deepest node should have valid layout
        let deepest = tree.get(depth as u64).unwrap();
        assert!(deepest.computed.width > 0 || deepest.computed.height > 0);
    }

    #[test]
    fn test_grid_in_flex() {
        let mut tree = LayoutTree::new();

        // Root (flex)
        let mut root = LayoutNode::default();
        root.id = 1;
        root.display = Display::Flex;
        root.children = vec![2];

        // Grid container
        let mut grid = LayoutNode::default();
        grid.id = 2;
        grid.display = Display::Grid;
        grid.sizing.width = Size::Fixed(80);
        grid.sizing.height = Size::Fixed(40);
        grid.children = vec![3, 4];

        // Grid items
        let mut item1 = LayoutNode::default();
        item1.id = 3;

        let mut item2 = LayoutNode::default();
        item2.id = 4;

        tree.insert(root);
        tree.insert(grid);
        tree.insert(item1);
        tree.insert(item2);
        tree.set_root(1);

        compute_layout(&mut tree, 1, 100, 100);

        // Grid container should be positioned
        let grid_node = tree.get(2).unwrap();
        assert_eq!(grid_node.computed.width, 80);

        // Grid items should be laid out
        let i1 = tree.get(3).unwrap();
        let i2 = tree.get(4).unwrap();
        assert!(i1.computed.width > 0);
        assert!(i2.computed.width > 0);
    }

    #[test]
    fn test_missing_node_graceful() {
        let mut tree = LayoutTree::new();

        let mut root = LayoutNode::default();
        root.id = 1;
        root.display = Display::Flex;
        root.children = vec![2, 999]; // 999 doesn't exist
        tree.insert(root);

        let mut child = LayoutNode::default();
        child.id = 2;
        child.sizing.width = Size::Fixed(50);
        tree.insert(child);

        tree.set_root(1);

        // Should not panic with missing child
        compute_layout(&mut tree, 1, 100, 100);

        let c = tree.get(2).unwrap();
        assert_eq!(c.computed.width, 50);
    }

    #[test]
    fn test_zero_available_space() {
        let (mut tree, _) = setup_simple_tree();

        // Should not panic with zero space
        compute_layout(&mut tree, 1, 0, 0);
    }
}
