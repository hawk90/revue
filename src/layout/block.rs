//! Block layout algorithm
//!
//! Simple vertical stacking layout where each child takes full width.

use super::node::ComputedLayout;
use super::tree::LayoutTree;
use crate::style::Size;

/// Compute block layout (vertical stacking, full width)
pub fn compute_block(
    tree: &mut LayoutTree,
    node_id: u64,
    available_width: u16,
    available_height: u16,
) {
    let node = match tree.get(node_id) {
        Some(n) => n,
        None => return,
    };

    let padding = node.spacing.padding;
    let children: Vec<u64> = node.children.clone();

    if children.is_empty() {
        return;
    }

    // Calculate content area
    let content_width = available_width
        .saturating_sub(padding.left)
        .saturating_sub(padding.right);
    let content_height = available_height
        .saturating_sub(padding.top)
        .saturating_sub(padding.bottom);

    let mut y_pos = padding.top;

    for &child_id in &children {
        let child = match tree.get(child_id) {
            Some(c) => c,
            None => continue,
        };

        let margin = child.spacing.margin;

        // Block children take full width by default
        let child_width = match child.sizing.width {
            Size::Fixed(v) => v.min(content_width),
            Size::Percent(pct) => ((content_width as f32) * pct / 100.0) as u16,
            Size::Auto => content_width.saturating_sub(margin.horizontal()),
        };

        // Height defaults to 1 for auto
        let child_height = match child.sizing.height {
            Size::Fixed(v) => v,
            Size::Percent(pct) => ((content_height as f32) * pct / 100.0) as u16,
            Size::Auto => 1, // Minimum height for block items
        };

        // Apply min/max constraints
        let child_width = apply_constraints(
            child_width,
            child.sizing.min_width,
            child.sizing.max_width,
            content_width,
        );
        let child_height = apply_constraints(
            child_height,
            child.sizing.min_height,
            child.sizing.max_height,
            content_height,
        );

        // Position with margin
        let x = padding.left.saturating_add(margin.left);
        let y = y_pos.saturating_add(margin.top);

        // Update child's computed layout
        if let Some(child_mut) = tree.get_mut(child_id) {
            child_mut.computed = ComputedLayout::new(x, y, child_width, child_height);
        }

        // Advance y position
        y_pos = y_pos
            .saturating_add(margin.top)
            .saturating_add(child_height)
            .saturating_add(margin.bottom);
    }
}

/// Apply min/max size constraints
fn apply_constraints(size: u16, min: Size, max: Size, available: u16) -> u16 {
    let min_val = match min {
        Size::Auto => 0,
        Size::Fixed(v) => v,
        Size::Percent(pct) => ((available as f32) * pct / 100.0) as u16,
    };

    let max_val = match max {
        Size::Auto => u16::MAX,
        Size::Fixed(v) => v,
        Size::Percent(pct) => ((available as f32) * pct / 100.0) as u16,
    };

    size.clamp(min_val, max_val)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::node::{Edges, LayoutNode};

    fn setup_tree_with_children(heights: Vec<Size>) -> (LayoutTree, u64, Vec<u64>) {
        let mut tree = LayoutTree::new();

        let mut parent = LayoutNode::default();
        parent.id = 1;

        let mut child_ids = Vec::new();
        for (i, height) in heights.iter().enumerate() {
            let mut child = LayoutNode::default();
            child.id = (i + 2) as u64;
            child.sizing.height = *height;
            child.sizing.width = Size::Auto;
            child_ids.push(child.id);
            tree.insert(child);
        }

        parent.children = child_ids.clone();
        tree.insert(parent);
        tree.set_root(1);

        (tree, 1, child_ids)
    }

    #[test]
    fn test_block_stacking() {
        let (mut tree, parent_id, child_ids) =
            setup_tree_with_children(vec![Size::Fixed(10), Size::Fixed(20), Size::Fixed(15)]);

        compute_block(&mut tree, parent_id, 100, 100);

        let child1 = tree.get(child_ids[0]).unwrap();
        assert_eq!(child1.computed.y, 0);
        assert_eq!(child1.computed.height, 10);
        assert_eq!(child1.computed.width, 100); // Full width

        let child2 = tree.get(child_ids[1]).unwrap();
        assert_eq!(child2.computed.y, 10);
        assert_eq!(child2.computed.height, 20);

        let child3 = tree.get(child_ids[2]).unwrap();
        assert_eq!(child3.computed.y, 30);
        assert_eq!(child3.computed.height, 15);
    }

    #[test]
    fn test_block_with_padding() {
        let (mut tree, parent_id, child_ids) = setup_tree_with_children(vec![Size::Fixed(10)]);

        if let Some(parent) = tree.get_mut(parent_id) {
            parent.spacing.padding = Edges {
                top: 5,
                right: 10,
                bottom: 5,
                left: 10,
            };
        }

        compute_block(&mut tree, parent_id, 100, 100);

        let child = tree.get(child_ids[0]).unwrap();
        assert_eq!(child.computed.x, 10); // Left padding
        assert_eq!(child.computed.y, 5); // Top padding
        assert_eq!(child.computed.width, 80); // 100 - 20 padding
    }

    #[test]
    fn test_block_with_margin() {
        let (mut tree, parent_id, child_ids) =
            setup_tree_with_children(vec![Size::Fixed(10), Size::Fixed(10)]);

        // Add margin to first child
        if let Some(child) = tree.get_mut(child_ids[0]) {
            child.spacing.margin = Edges {
                top: 5,
                right: 0,
                bottom: 5,
                left: 0,
            };
        }

        compute_block(&mut tree, parent_id, 100, 100);

        let child1 = tree.get(child_ids[0]).unwrap();
        assert_eq!(child1.computed.y, 5); // Top margin

        let child2 = tree.get(child_ids[1]).unwrap();
        assert_eq!(child2.computed.y, 20); // 5 + 10 + 5
    }

    #[test]
    fn test_block_percent_width() {
        let (mut tree, parent_id, child_ids) = setup_tree_with_children(vec![Size::Fixed(10)]);

        if let Some(child) = tree.get_mut(child_ids[0]) {
            child.sizing.width = Size::Percent(50.0);
        }

        compute_block(&mut tree, parent_id, 100, 100);

        let child = tree.get(child_ids[0]).unwrap();
        assert_eq!(child.computed.width, 50);
    }
}
