//! Flexbox layout algorithm
//!
//! Simplified flexbox implementation optimized for TUI.

use super::node::{ComputedLayout, LayoutNode};
use super::tree::LayoutTree;
use crate::style::{AlignItems, FlexDirection, JustifyContent, Size};

/// Compute flexbox layout for a node and its children
pub fn compute_flex(
    tree: &mut LayoutTree,
    node_id: u64,
    available_width: u16,
    available_height: u16,
) {
    let node = match tree.get(node_id) {
        Some(n) => n,
        None => return,
    };

    let direction = node.flex.direction;
    let justify = node.flex.justify_content;
    let align = node.flex.align_items;
    let gap = node.flex.main_gap();
    let padding = node.spacing.padding;
    let children: Vec<u64> = node.children.clone();

    if children.is_empty() {
        return;
    }

    // Calculate content area (subtract padding)
    let content_width = available_width
        .saturating_sub(padding.left)
        .saturating_sub(padding.right);
    let content_height = available_height
        .saturating_sub(padding.top)
        .saturating_sub(padding.bottom);

    // Determine main/cross axis dimensions
    let (main_size, cross_size) = match direction {
        FlexDirection::Row => (content_width, content_height),
        FlexDirection::Column => (content_height, content_width),
    };

    // Calculate total gaps
    let total_gaps = gap.saturating_mul(children.len().saturating_sub(1) as u16);
    let available_main = main_size.saturating_sub(total_gaps);

    // First pass: calculate fixed sizes and collect auto items
    let mut child_main_sizes: Vec<u16> = vec![0; children.len()];
    let mut total_fixed: u16 = 0;
    let mut auto_count: usize = 0;

    for (i, &child_id) in children.iter().enumerate() {
        let child = match tree.get(child_id) {
            Some(c) => c,
            None => continue,
        };

        let size_prop = match direction {
            FlexDirection::Row => child.sizing.width,
            FlexDirection::Column => child.sizing.height,
        };

        match size_prop {
            Size::Fixed(v) => {
                let v = apply_main_constraints(child, direction, v, available_main);
                child_main_sizes[i] = v;
                total_fixed = total_fixed.saturating_add(v);
            }
            Size::Percent(pct) => {
                let v = ((available_main as f32) * pct / 100.0) as u16;
                let v = apply_main_constraints(child, direction, v, available_main);
                child_main_sizes[i] = v;
                total_fixed = total_fixed.saturating_add(v);
            }
            Size::Auto => {
                auto_count += 1;
            }
        }
    }

    // Second pass: distribute remaining space to auto-sized children
    let remaining = available_main.saturating_sub(total_fixed);
    if auto_count > 0 && remaining > 0 {
        let per_auto = remaining / auto_count as u16;
        let extra = remaining % auto_count as u16;
        let mut extra_given = 0u16;

        for (i, &child_id) in children.iter().enumerate() {
            let child = match tree.get(child_id) {
                Some(c) => c,
                None => continue,
            };

            let size_prop = match direction {
                FlexDirection::Row => child.sizing.width,
                FlexDirection::Column => child.sizing.height,
            };

            if matches!(size_prop, Size::Auto) {
                let mut size = per_auto;
                // Distribute remaining pixels to first few items
                if extra_given < extra {
                    size += 1;
                    extra_given += 1;
                }
                let size = apply_main_constraints(child, direction, size, available_main);
                child_main_sizes[i] = size;
            }
        }
    }

    // Calculate total used space and free space
    let total_used: u16 = child_main_sizes
        .iter()
        .sum::<u16>()
        .saturating_add(total_gaps);
    let free_space = main_size.saturating_sub(total_used);

    // Calculate positions based on justify-content
    let (initial_offset, inter_gap) = compute_justify_offsets(justify, free_space, children.len());

    // Position children
    let mut main_pos = initial_offset;

    for (i, &child_id) in children.iter().enumerate() {
        let child_main = child_main_sizes[i];

        // Calculate cross axis size
        let child = match tree.get(child_id) {
            Some(c) => c,
            None => continue,
        };

        let cross_size_prop = match direction {
            FlexDirection::Row => child.sizing.height,
            FlexDirection::Column => child.sizing.width,
        };

        let child_cross = match cross_size_prop {
            Size::Fixed(v) => v,
            Size::Percent(pct) => ((cross_size as f32) * pct / 100.0) as u16,
            Size::Auto => {
                if align == AlignItems::Stretch {
                    cross_size
                } else {
                    // Default to 1 for auto in non-stretch
                    1
                }
            }
        };
        let child_cross = apply_cross_constraints(child, direction, child_cross, cross_size);

        // Calculate cross axis offset based on alignment
        let cross_offset = compute_align_offset(align, cross_size, child_cross);

        // Assign position based on direction
        let (x, y, w, h) = match direction {
            FlexDirection::Row => (
                padding.left.saturating_add(main_pos),
                padding.top.saturating_add(cross_offset),
                child_main,
                child_cross,
            ),
            FlexDirection::Column => (
                padding.left.saturating_add(cross_offset),
                padding.top.saturating_add(main_pos),
                child_cross,
                child_main,
            ),
        };

        // Update child's computed layout
        if let Some(child_mut) = tree.get_mut(child_id) {
            child_mut.computed = ComputedLayout::new(x, y, w, h);
        }

        // Advance position
        main_pos = main_pos.saturating_add(child_main).saturating_add(gap);
        if i < children.len() - 1 {
            main_pos = main_pos.saturating_add(inter_gap);
        }
    }
}

/// Apply min/max constraints on main axis
fn apply_main_constraints(
    node: &LayoutNode,
    direction: FlexDirection,
    size: u16,
    available: u16,
) -> u16 {
    let (min_size, max_size) = match direction {
        FlexDirection::Row => (node.sizing.min_width, node.sizing.max_width),
        FlexDirection::Column => (node.sizing.min_height, node.sizing.max_height),
    };

    let min_val = resolve_constraint(min_size, available, 0);
    let max_val = resolve_constraint(max_size, available, u16::MAX);

    size.clamp(min_val, max_val)
}

/// Apply min/max constraints on cross axis
fn apply_cross_constraints(
    node: &LayoutNode,
    direction: FlexDirection,
    size: u16,
    available: u16,
) -> u16 {
    let (min_size, max_size) = match direction {
        FlexDirection::Row => (node.sizing.min_height, node.sizing.max_height),
        FlexDirection::Column => (node.sizing.min_width, node.sizing.max_width),
    };

    let min_val = resolve_constraint(min_size, available, 0);
    let max_val = resolve_constraint(max_size, available, u16::MAX);

    size.clamp(min_val, max_val)
}

/// Resolve a size constraint to a concrete value
fn resolve_constraint(size: Size, available: u16, default: u16) -> u16 {
    match size {
        Size::Auto => default,
        Size::Fixed(v) => v,
        Size::Percent(pct) => ((available as f32) * pct / 100.0) as u16,
    }
}

/// Compute initial offset and inter-item gap for justify-content
fn compute_justify_offsets(
    justify: JustifyContent,
    free_space: u16,
    child_count: usize,
) -> (u16, u16) {
    match justify {
        JustifyContent::Start => (0, 0),
        JustifyContent::End => (free_space, 0),
        JustifyContent::Center => (free_space / 2, 0),
        JustifyContent::SpaceBetween => {
            if child_count > 1 {
                (0, free_space / (child_count - 1) as u16)
            } else {
                (0, 0)
            }
        }
        JustifyContent::SpaceAround => {
            if child_count > 0 {
                let space = free_space / child_count as u16;
                (space / 2, space)
            } else {
                (0, 0)
            }
        }
    }
}

/// Compute cross axis offset based on align-items
fn compute_align_offset(align: AlignItems, cross_size: u16, child_cross: u16) -> u16 {
    match align {
        AlignItems::Start => 0,
        AlignItems::End => cross_size.saturating_sub(child_cross),
        AlignItems::Center => cross_size.saturating_sub(child_cross) / 2,
        AlignItems::Stretch => 0, // Child already stretched to cross_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::node::Edges;

    fn setup_tree_with_parent_and_children(
        parent_width: u16,
        parent_height: u16,
        child_widths: Vec<Size>,
        direction: FlexDirection,
    ) -> (LayoutTree, u64, Vec<u64>) {
        let mut tree = LayoutTree::new();

        // Create parent
        let mut parent = LayoutNode::default();
        parent.id = 1;
        parent.flex.direction = direction;
        parent.sizing.width = Size::Fixed(parent_width);
        parent.sizing.height = Size::Fixed(parent_height);

        let mut child_ids = Vec::new();
        for (i, width) in child_widths.iter().enumerate() {
            let mut child = LayoutNode::default();
            child.id = (i + 2) as u64;
            child.sizing.width = *width;
            child.sizing.height = Size::Auto;
            child_ids.push(child.id);
            tree.insert(child);
        }

        parent.children = child_ids.clone();
        tree.insert(parent);
        tree.set_root(1);

        (tree, 1, child_ids)
    }

    #[test]
    fn test_flex_row_auto_distribution() {
        let (mut tree, parent_id, child_ids) = setup_tree_with_parent_and_children(
            100,
            50,
            vec![Size::Auto, Size::Auto],
            FlexDirection::Row,
        );

        compute_flex(&mut tree, parent_id, 100, 50);

        // Each auto child should get 50 width
        let child1 = tree.get(child_ids[0]).unwrap();
        assert_eq!(child1.computed.width, 50);
        assert_eq!(child1.computed.x, 0);

        let child2 = tree.get(child_ids[1]).unwrap();
        assert_eq!(child2.computed.width, 50);
        assert_eq!(child2.computed.x, 50);
    }

    #[test]
    fn test_flex_row_fixed_sizes() {
        let (mut tree, parent_id, child_ids) = setup_tree_with_parent_and_children(
            100,
            50,
            vec![Size::Fixed(30), Size::Fixed(40)],
            FlexDirection::Row,
        );

        compute_flex(&mut tree, parent_id, 100, 50);

        let child1 = tree.get(child_ids[0]).unwrap();
        assert_eq!(child1.computed.width, 30);
        assert_eq!(child1.computed.x, 0);

        let child2 = tree.get(child_ids[1]).unwrap();
        assert_eq!(child2.computed.width, 40);
        assert_eq!(child2.computed.x, 30);
    }

    #[test]
    fn test_flex_column() {
        let (mut tree, parent_id, child_ids) = setup_tree_with_parent_and_children(
            50,
            100,
            vec![Size::Auto, Size::Auto],
            FlexDirection::Column,
        );

        // For column, we need to set heights
        for &id in &child_ids {
            if let Some(node) = tree.get_mut(id) {
                node.sizing.height = Size::Auto;
                node.sizing.width = Size::Auto;
            }
        }

        compute_flex(&mut tree, parent_id, 50, 100);

        let child1 = tree.get(child_ids[0]).unwrap();
        assert_eq!(child1.computed.height, 50);
        assert_eq!(child1.computed.y, 0);

        let child2 = tree.get(child_ids[1]).unwrap();
        assert_eq!(child2.computed.height, 50);
        assert_eq!(child2.computed.y, 50);
    }

    #[test]
    fn test_flex_with_gap() {
        let (mut tree, parent_id, child_ids) = setup_tree_with_parent_and_children(
            100,
            50,
            vec![Size::Auto, Size::Auto],
            FlexDirection::Row,
        );

        // Add gap
        if let Some(parent) = tree.get_mut(parent_id) {
            parent.flex.gap = 10;
        }

        compute_flex(&mut tree, parent_id, 100, 50);

        // 100 - 10 (gap) = 90 / 2 = 45 each
        let child1 = tree.get(child_ids[0]).unwrap();
        assert_eq!(child1.computed.width, 45);
        assert_eq!(child1.computed.x, 0);

        let child2 = tree.get(child_ids[1]).unwrap();
        assert_eq!(child2.computed.width, 45);
        assert_eq!(child2.computed.x, 55); // 45 + 10 gap
    }

    #[test]
    fn test_flex_justify_center() {
        let (mut tree, parent_id, child_ids) = setup_tree_with_parent_and_children(
            100,
            50,
            vec![Size::Fixed(20), Size::Fixed(20)],
            FlexDirection::Row,
        );

        if let Some(parent) = tree.get_mut(parent_id) {
            parent.flex.justify_content = JustifyContent::Center;
        }

        compute_flex(&mut tree, parent_id, 100, 50);

        // Free space = 100 - 40 = 60, offset = 30
        let child1 = tree.get(child_ids[0]).unwrap();
        assert_eq!(child1.computed.x, 30);

        let child2 = tree.get(child_ids[1]).unwrap();
        assert_eq!(child2.computed.x, 50);
    }

    #[test]
    fn test_flex_justify_space_between() {
        let (mut tree, parent_id, child_ids) = setup_tree_with_parent_and_children(
            100,
            50,
            vec![Size::Fixed(20), Size::Fixed(20)],
            FlexDirection::Row,
        );

        if let Some(parent) = tree.get_mut(parent_id) {
            parent.flex.justify_content = JustifyContent::SpaceBetween;
        }

        compute_flex(&mut tree, parent_id, 100, 50);

        // First at 0, second at 100 - 20 = 80
        let child1 = tree.get(child_ids[0]).unwrap();
        assert_eq!(child1.computed.x, 0);

        let child2 = tree.get(child_ids[1]).unwrap();
        assert_eq!(child2.computed.x, 80);
    }

    #[test]
    fn test_flex_align_center() {
        let (mut tree, parent_id, child_ids) =
            setup_tree_with_parent_and_children(100, 50, vec![Size::Fixed(30)], FlexDirection::Row);

        if let Some(parent) = tree.get_mut(parent_id) {
            parent.flex.align_items = AlignItems::Center;
        }
        if let Some(child) = tree.get_mut(child_ids[0]) {
            child.sizing.height = Size::Fixed(20);
        }

        compute_flex(&mut tree, parent_id, 100, 50);

        let child = tree.get(child_ids[0]).unwrap();
        assert_eq!(child.computed.height, 20);
        assert_eq!(child.computed.y, 15); // (50 - 20) / 2
    }

    #[test]
    fn test_flex_with_padding() {
        let (mut tree, parent_id, child_ids) =
            setup_tree_with_parent_and_children(100, 50, vec![Size::Auto], FlexDirection::Row);

        if let Some(parent) = tree.get_mut(parent_id) {
            parent.spacing.padding = Edges {
                top: 5,
                right: 10,
                bottom: 5,
                left: 10,
            };
        }

        compute_flex(&mut tree, parent_id, 100, 50);

        let child = tree.get(child_ids[0]).unwrap();
        // Content area = 100 - 20 = 80
        assert_eq!(child.computed.width, 80);
        assert_eq!(child.computed.x, 10); // Left padding
        assert_eq!(child.computed.y, 5); // Top padding
    }
}
