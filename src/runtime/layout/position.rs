//! Position offset handling
//!
//! Applies position offsets for relative, absolute, and fixed positioning.

use super::node::{ComputedLayout, LayoutNode};
use crate::style::Position;

/// Safely convert i32 to u16, clamping to valid range [0, u16::MAX]
#[inline]
fn clamp_to_u16(value: i32) -> u16 {
    value.clamp(0, u16::MAX as i32) as u16
}

/// Apply position offsets to a node's computed layout
///
/// # Arguments
/// * `node` - The node to apply offsets to (mutated)
/// * `parent_layout` - Parent's computed layout (for absolute positioning)
/// * `viewport` - Viewport dimensions (for fixed positioning)
pub fn apply_position_offsets(
    node: &mut LayoutNode,
    parent_layout: ComputedLayout,
    viewport: (u16, u16),
) {
    match node.position {
        Position::Static => {
            // No adjustment needed for static positioning
        }
        Position::Relative => {
            apply_relative_offset(node);
        }
        Position::Absolute => {
            apply_absolute_offset(node, parent_layout);
        }
        Position::Fixed => {
            apply_fixed_offset(node, viewport);
        }
        Position::Sticky => {
            // Sticky behaves like relative until scroll threshold is reached
            apply_relative_offset(node);
        }
    }
}

/// Apply relative position offset
///
/// Relative positioning offsets from the normal flow position.
/// Only one of top/bottom and one of left/right is used.
fn apply_relative_offset(node: &mut LayoutNode) {
    let inset = &node.spacing.inset;

    // Vertical: top takes precedence over bottom
    if let Some(top) = inset.top {
        node.computed.y = clamp_to_u16(node.computed.y as i32 + top as i32);
    } else if let Some(bottom) = inset.bottom {
        node.computed.y = clamp_to_u16(node.computed.y as i32 - bottom as i32);
    }

    // Horizontal: left takes precedence over right
    if let Some(left) = inset.left {
        node.computed.x = clamp_to_u16(node.computed.x as i32 + left as i32);
    } else if let Some(right) = inset.right {
        node.computed.x = clamp_to_u16(node.computed.x as i32 - right as i32);
    }
}

/// Apply absolute position offset
///
/// Absolute positioning is relative to the parent's content area.
fn apply_absolute_offset(node: &mut LayoutNode, parent_layout: ComputedLayout) {
    let inset = &node.spacing.inset;

    // Vertical positioning
    if let Some(top) = inset.top {
        node.computed.y = clamp_to_u16(top as i32);
    } else if let Some(bottom) = inset.bottom {
        node.computed.y = parent_layout
            .height
            .saturating_sub(node.computed.height)
            .saturating_sub(clamp_to_u16(bottom as i32));
    }

    // Horizontal positioning
    if let Some(left) = inset.left {
        node.computed.x = clamp_to_u16(left as i32);
    } else if let Some(right) = inset.right {
        node.computed.x = parent_layout
            .width
            .saturating_sub(node.computed.width)
            .saturating_sub(clamp_to_u16(right as i32));
    }
}

/// Apply fixed position offset
///
/// Fixed positioning is relative to the viewport.
fn apply_fixed_offset(node: &mut LayoutNode, viewport: (u16, u16)) {
    let (vw, vh) = viewport;
    let inset = &node.spacing.inset;

    // Vertical positioning
    if let Some(top) = inset.top {
        node.computed.y = clamp_to_u16(top as i32);
    } else if let Some(bottom) = inset.bottom {
        node.computed.y = vh
            .saturating_sub(node.computed.height)
            .saturating_sub(clamp_to_u16(bottom as i32));
    }

    // Horizontal positioning
    if let Some(left) = inset.left {
        node.computed.x = clamp_to_u16(left as i32);
    } else if let Some(right) = inset.right {
        node.computed.x = vw
            .saturating_sub(node.computed.width)
            .saturating_sub(clamp_to_u16(right as i32));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::node::Inset;

    fn make_node_at(x: u16, y: u16, w: u16, h: u16, position: Position) -> LayoutNode {
        let mut node = LayoutNode::default();
        node.position = position;
        node.computed = ComputedLayout::new(x, y, w, h);
        node
    }

    #[test]
    fn test_static_no_change() {
        let mut node = make_node_at(10, 20, 50, 30, Position::Static);
        node.spacing.inset = Inset {
            top: Some(5),
            left: Some(5),
            right: None,
            bottom: None,
        };

        let parent = ComputedLayout::new(0, 0, 100, 100);
        apply_position_offsets(&mut node, parent, (200, 200));

        assert_eq!(node.computed.x, 10);
        assert_eq!(node.computed.y, 20);
    }

    #[test]
    fn test_relative_top_left() {
        let mut node = make_node_at(10, 20, 50, 30, Position::Relative);
        node.spacing.inset = Inset {
            top: Some(5),
            left: Some(10),
            right: None,
            bottom: None,
        };

        let parent = ComputedLayout::new(0, 0, 100, 100);
        apply_position_offsets(&mut node, parent, (200, 200));

        assert_eq!(node.computed.x, 20); // 10 + 10
        assert_eq!(node.computed.y, 25); // 20 + 5
    }

    #[test]
    fn test_relative_bottom_right() {
        let mut node = make_node_at(10, 20, 50, 30, Position::Relative);
        node.spacing.inset = Inset {
            top: None,
            left: None,
            right: Some(5),
            bottom: Some(10),
        };

        let parent = ComputedLayout::new(0, 0, 100, 100);
        apply_position_offsets(&mut node, parent, (200, 200));

        assert_eq!(node.computed.x, 5); // 10 - 5
        assert_eq!(node.computed.y, 10); // 20 - 10
    }

    #[test]
    fn test_relative_negative_clamps_to_zero() {
        let mut node = make_node_at(5, 5, 50, 30, Position::Relative);
        node.spacing.inset = Inset {
            top: Some(-20),
            left: Some(-20),
            right: None,
            bottom: None,
        };

        let parent = ComputedLayout::new(0, 0, 100, 100);
        apply_position_offsets(&mut node, parent, (200, 200));

        assert_eq!(node.computed.x, 0);
        assert_eq!(node.computed.y, 0);
    }

    #[test]
    fn test_absolute_top_left() {
        let mut node = make_node_at(10, 20, 50, 30, Position::Absolute);
        node.spacing.inset = Inset {
            top: Some(5),
            left: Some(10),
            right: None,
            bottom: None,
        };

        let parent = ComputedLayout::new(0, 0, 100, 100);
        apply_position_offsets(&mut node, parent, (200, 200));

        // Absolute ignores normal flow position
        assert_eq!(node.computed.x, 10);
        assert_eq!(node.computed.y, 5);
    }

    #[test]
    fn test_absolute_bottom_right() {
        let mut node = make_node_at(10, 20, 50, 30, Position::Absolute);
        node.spacing.inset = Inset {
            top: None,
            left: None,
            right: Some(10),
            bottom: Some(5),
        };

        let parent = ComputedLayout::new(0, 0, 100, 100);
        apply_position_offsets(&mut node, parent, (200, 200));

        // x = 100 - 50 - 10 = 40
        // y = 100 - 30 - 5 = 65
        assert_eq!(node.computed.x, 40);
        assert_eq!(node.computed.y, 65);
    }

    #[test]
    fn test_fixed_top_left() {
        let mut node = make_node_at(10, 20, 50, 30, Position::Fixed);
        node.spacing.inset = Inset {
            top: Some(0),
            left: Some(0),
            right: None,
            bottom: None,
        };

        let parent = ComputedLayout::new(50, 50, 100, 100);
        apply_position_offsets(&mut node, parent, (200, 150));

        // Fixed uses viewport, not parent
        assert_eq!(node.computed.x, 0);
        assert_eq!(node.computed.y, 0);
    }

    #[test]
    fn test_fixed_bottom_right() {
        let mut node = make_node_at(10, 20, 50, 30, Position::Fixed);
        node.spacing.inset = Inset {
            top: None,
            left: None,
            right: Some(10),
            bottom: Some(5),
        };

        let parent = ComputedLayout::new(50, 50, 100, 100);
        apply_position_offsets(&mut node, parent, (200, 150));

        // x = 200 - 50 - 10 = 140
        // y = 150 - 30 - 5 = 115
        assert_eq!(node.computed.x, 140);
        assert_eq!(node.computed.y, 115);
    }

    #[test]
    fn test_top_precedence_over_bottom() {
        let mut node = make_node_at(10, 20, 50, 30, Position::Relative);
        node.spacing.inset = Inset {
            top: Some(5),
            left: None,
            right: None,
            bottom: Some(100), // Should be ignored
        };

        let parent = ComputedLayout::new(0, 0, 100, 100);
        apply_position_offsets(&mut node, parent, (200, 200));

        assert_eq!(node.computed.y, 25); // Uses top, not bottom
    }

    #[test]
    fn test_left_precedence_over_right() {
        let mut node = make_node_at(10, 20, 50, 30, Position::Relative);
        node.spacing.inset = Inset {
            top: None,
            left: Some(5),
            right: Some(100), // Should be ignored
            bottom: None,
        };

        let parent = ComputedLayout::new(0, 0, 100, 100);
        apply_position_offsets(&mut node, parent, (200, 200));

        assert_eq!(node.computed.x, 15); // Uses left, not right
    }

    #[test]
    fn test_clamp_to_u16() {
        // Test the helper function directly
        assert_eq!(clamp_to_u16(0), 0);
        assert_eq!(clamp_to_u16(100), 100);
        assert_eq!(clamp_to_u16(-100), 0);
        assert_eq!(clamp_to_u16(u16::MAX as i32), u16::MAX);
        assert_eq!(clamp_to_u16(u16::MAX as i32 + 1), u16::MAX);
        assert_eq!(clamp_to_u16(i32::MAX), u16::MAX);
        assert_eq!(clamp_to_u16(i32::MIN), 0);
    }

    #[test]
    fn test_relative_overflow_clamps_to_u16_max() {
        // Test that large offsets clamp to u16::MAX instead of wrapping
        // This is the fix for issue #150
        let mut node = make_node_at(60000, 60000, 50, 30, Position::Relative);
        node.spacing.inset = Inset {
            top: Some(10000),  // 60000 + 10000 = 70000 > u16::MAX
            left: Some(10000), // 60000 + 10000 = 70000 > u16::MAX
            right: None,
            bottom: None,
        };

        let parent = ComputedLayout::new(0, 0, 100, 100);
        apply_position_offsets(&mut node, parent, (200, 200));

        // Should clamp to u16::MAX (65535), not wrap to 4464
        assert_eq!(node.computed.x, u16::MAX);
        assert_eq!(node.computed.y, u16::MAX);
    }

    #[test]
    fn test_absolute_large_inset_clamps() {
        // Test that very large inset values clamp properly
        let mut node = make_node_at(10, 20, 50, 30, Position::Absolute);
        node.spacing.inset = Inset {
            top: Some(i16::MAX), // Large positive value
            left: Some(i16::MAX),
            right: None,
            bottom: None,
        };

        let parent = ComputedLayout::new(0, 0, 100, 100);
        apply_position_offsets(&mut node, parent, (200, 200));

        // i16::MAX (32767) fits in u16, should work correctly
        assert_eq!(node.computed.x, 32767);
        assert_eq!(node.computed.y, 32767);
    }

    #[test]
    fn test_fixed_large_inset_clamps() {
        let mut node = make_node_at(10, 20, 50, 30, Position::Fixed);
        node.spacing.inset = Inset {
            top: Some(i16::MAX),
            left: Some(i16::MAX),
            right: None,
            bottom: None,
        };

        let parent = ComputedLayout::new(0, 0, 100, 100);
        apply_position_offsets(&mut node, parent, (200, 200));

        assert_eq!(node.computed.x, 32767);
        assert_eq!(node.computed.y, 32767);
    }

    #[test]
    fn test_relative_negative_inset_clamps_to_zero() {
        // Large negative offset should clamp to 0
        let mut node = make_node_at(100, 100, 50, 30, Position::Relative);
        node.spacing.inset = Inset {
            top: None,
            left: None,
            right: Some(i16::MAX), // Large subtraction
            bottom: Some(i16::MAX),
        };

        let parent = ComputedLayout::new(0, 0, 100, 100);
        apply_position_offsets(&mut node, parent, (200, 200));

        // 100 - 32767 = negative, should clamp to 0
        assert_eq!(node.computed.x, 0);
        assert_eq!(node.computed.y, 0);
    }
}
