//! Resizable widget types

use crate::layout::Rect;

/// Resize handle positions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResizeHandle {
    /// Top edge
    Top,
    /// Bottom edge
    Bottom,
    /// Left edge
    Left,
    /// Right edge
    Right,
    /// Top-left corner
    TopLeft,
    /// Top-right corner
    TopRight,
    /// Bottom-left corner
    BottomLeft,
    /// Bottom-right corner
    BottomRight,
}

impl ResizeHandle {
    /// All handles
    pub const ALL: &'static [ResizeHandle] = &[
        ResizeHandle::Top,
        ResizeHandle::Bottom,
        ResizeHandle::Left,
        ResizeHandle::Right,
        ResizeHandle::TopLeft,
        ResizeHandle::TopRight,
        ResizeHandle::BottomLeft,
        ResizeHandle::BottomRight,
    ];

    /// Edge handles only
    pub const EDGES: &'static [ResizeHandle] = &[
        ResizeHandle::Top,
        ResizeHandle::Bottom,
        ResizeHandle::Left,
        ResizeHandle::Right,
    ];

    /// Corner handles only
    pub const CORNERS: &'static [ResizeHandle] = &[
        ResizeHandle::TopLeft,
        ResizeHandle::TopRight,
        ResizeHandle::BottomLeft,
        ResizeHandle::BottomRight,
    ];

    /// Check if position is within handle area
    pub fn hit_test(&self, x: u16, y: u16, area: Rect, handle_size: u16) -> bool {
        match self {
            ResizeHandle::Top => {
                y == area.y && x > area.x + handle_size && x < area.x + area.width - handle_size
            }
            ResizeHandle::Bottom => {
                y == area.y + area.height.saturating_sub(1)
                    && x > area.x + handle_size
                    && x < area.x + area.width - handle_size
            }
            ResizeHandle::Left => {
                x == area.x && y > area.y + handle_size && y < area.y + area.height - handle_size
            }
            ResizeHandle::Right => {
                x == area.x + area.width.saturating_sub(1)
                    && y > area.y + handle_size
                    && y < area.y + area.height - handle_size
            }
            ResizeHandle::TopLeft => x <= area.x + handle_size && y <= area.y + handle_size,
            ResizeHandle::TopRight => {
                x >= area.x + area.width.saturating_sub(handle_size + 1)
                    && y <= area.y + handle_size
            }
            ResizeHandle::BottomLeft => {
                x <= area.x + handle_size
                    && y >= area.y + area.height.saturating_sub(handle_size + 1)
            }
            ResizeHandle::BottomRight => {
                x >= area.x + area.width.saturating_sub(handle_size + 1)
                    && y >= area.y + area.height.saturating_sub(handle_size + 1)
            }
        }
    }
}

/// Resize direction during drag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResizeDirection {
    /// Resize horizontally (-1 = left, 0 = none, 1 = right)
    pub horizontal: i8,
    /// Resize vertically (-1 = up, 0 = none, 1 = down)
    pub vertical: i8,
}

impl ResizeDirection {
    /// No resize
    pub const NONE: Self = Self {
        horizontal: 0,
        vertical: 0,
    };

    /// From handle
    pub fn from_handle(handle: ResizeHandle) -> Self {
        match handle {
            ResizeHandle::Top => Self {
                horizontal: 0,
                vertical: -1,
            },
            ResizeHandle::Bottom => Self {
                horizontal: 0,
                vertical: 1,
            },
            ResizeHandle::Left => Self {
                horizontal: -1,
                vertical: 0,
            },
            ResizeHandle::Right => Self {
                horizontal: 1,
                vertical: 0,
            },
            ResizeHandle::TopLeft => Self {
                horizontal: -1,
                vertical: -1,
            },
            ResizeHandle::TopRight => Self {
                horizontal: 1,
                vertical: -1,
            },
            ResizeHandle::BottomLeft => Self {
                horizontal: -1,
                vertical: 1,
            },
            ResizeHandle::BottomRight => Self {
                horizontal: 1,
                vertical: 1,
            },
        }
    }
}

/// Visual style for resize handles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ResizeStyle {
    /// Border with corner markers
    #[default]
    Border,
    /// Only show handles on hover
    Subtle,
    /// Invisible handles
    Hidden,
    /// Dot indicators at corners
    Dots,
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // ResizeHandle tests
    // =========================================================================

    #[test]
    fn test_resize_handle_all_const() {
        let all = ResizeHandle::ALL;
        assert_eq!(all.len(), 8);
        assert!(all.contains(&ResizeHandle::Top));
        assert!(all.contains(&ResizeHandle::Bottom));
        assert!(all.contains(&ResizeHandle::Left));
        assert!(all.contains(&ResizeHandle::Right));
        assert!(all.contains(&ResizeHandle::TopLeft));
        assert!(all.contains(&ResizeHandle::TopRight));
        assert!(all.contains(&ResizeHandle::BottomLeft));
        assert!(all.contains(&ResizeHandle::BottomRight));
    }

    #[test]
    fn test_resize_handle_edges_const() {
        let edges = ResizeHandle::EDGES;
        assert_eq!(edges.len(), 4);
        assert!(edges.contains(&ResizeHandle::Top));
        assert!(edges.contains(&ResizeHandle::Bottom));
        assert!(edges.contains(&ResizeHandle::Left));
        assert!(edges.contains(&ResizeHandle::Right));
    }

    #[test]
    fn test_resize_handle_corners_const() {
        let corners = ResizeHandle::CORNERS;
        assert_eq!(corners.len(), 4);
        assert!(corners.contains(&ResizeHandle::TopLeft));
        assert!(corners.contains(&ResizeHandle::TopRight));
        assert!(corners.contains(&ResizeHandle::BottomLeft));
        assert!(corners.contains(&ResizeHandle::BottomRight));
    }

    #[test]
    fn test_resize_handle_hit_test_top() {
        let area = Rect::new(10, 10, 100, 50);
        let handle_size = 5;
        assert!(ResizeHandle::Top.hit_test(50, 10, area, handle_size));
        assert!(!ResizeHandle::Top.hit_test(10, 10, area, handle_size)); // Too close to left
        assert!(!ResizeHandle::Top.hit_test(109, 10, area, handle_size)); // Too close to right
    }

    #[test]
    fn test_resize_handle_hit_test_bottom() {
        let area = Rect::new(10, 10, 100, 50);
        let handle_size = 5;
        assert!(ResizeHandle::Bottom.hit_test(50, 59, area, handle_size));
        assert!(!ResizeHandle::Bottom.hit_test(10, 59, area, handle_size)); // Too close to left
    }

    #[test]
    fn test_resize_handle_hit_test_left() {
        let area = Rect::new(10, 10, 100, 50);
        let handle_size = 5;
        assert!(ResizeHandle::Left.hit_test(10, 30, area, handle_size));
        assert!(!ResizeHandle::Left.hit_test(10, 10, area, handle_size)); // Too close to top
        assert!(!ResizeHandle::Left.hit_test(10, 59, area, handle_size)); // Too close to bottom
    }

    #[test]
    fn test_resize_handle_hit_test_right() {
        let area = Rect::new(10, 10, 100, 50);
        let handle_size = 5;
        assert!(ResizeHandle::Right.hit_test(109, 30, area, handle_size));
        assert!(!ResizeHandle::Right.hit_test(109, 10, area, handle_size)); // Too close to top
    }

    #[test]
    fn test_resize_handle_hit_test_top_left() {
        let area = Rect::new(10, 10, 100, 50);
        let handle_size = 5;
        assert!(ResizeHandle::TopLeft.hit_test(10, 10, area, handle_size));
        assert!(ResizeHandle::TopLeft.hit_test(15, 15, area, handle_size));
        assert!(!ResizeHandle::TopLeft.hit_test(20, 20, area, handle_size));
    }

    #[test]
    fn test_resize_handle_hit_test_top_right() {
        let area = Rect::new(10, 10, 100, 50);
        let handle_size = 5;
        assert!(ResizeHandle::TopRight.hit_test(109, 10, area, handle_size));
        assert!(ResizeHandle::TopRight.hit_test(105, 15, area, handle_size));
    }

    #[test]
    fn test_resize_handle_hit_test_bottom_left() {
        let area = Rect::new(10, 10, 100, 50);
        let handle_size = 5;
        assert!(ResizeHandle::BottomLeft.hit_test(10, 59, area, handle_size));
        assert!(ResizeHandle::BottomLeft.hit_test(15, 55, area, handle_size));
    }

    #[test]
    fn test_resize_handle_hit_test_bottom_right() {
        let area = Rect::new(10, 10, 100, 50);
        let handle_size = 5;
        assert!(ResizeHandle::BottomRight.hit_test(109, 59, area, handle_size));
        assert!(ResizeHandle::BottomRight.hit_test(105, 55, area, handle_size));
    }

    #[test]
    fn test_resize_handle_equality() {
        assert_eq!(ResizeHandle::Top, ResizeHandle::Top);
        assert_eq!(ResizeHandle::Bottom, ResizeHandle::Bottom);
        assert_eq!(ResizeHandle::Left, ResizeHandle::Left);
        assert_eq!(ResizeHandle::Right, ResizeHandle::Right);
        assert_eq!(ResizeHandle::TopLeft, ResizeHandle::TopLeft);
        assert_eq!(ResizeHandle::TopRight, ResizeHandle::TopRight);
        assert_eq!(ResizeHandle::BottomLeft, ResizeHandle::BottomLeft);
        assert_eq!(ResizeHandle::BottomRight, ResizeHandle::BottomRight);
    }

    #[test]
    fn test_resize_handle_inequality() {
        assert_ne!(ResizeHandle::Top, ResizeHandle::Bottom);
        assert_ne!(ResizeHandle::Left, ResizeHandle::Right);
        assert_ne!(ResizeHandle::TopLeft, ResizeHandle::BottomRight);
        assert_ne!(ResizeHandle::Top, ResizeHandle::TopLeft);
    }

    // =========================================================================
    // ResizeDirection tests
    // =========================================================================

    #[test]
    fn test_resize_direction_none_const() {
        let dir = ResizeDirection::NONE;
        assert_eq!(dir.horizontal, 0);
        assert_eq!(dir.vertical, 0);
    }

    #[test]
    fn test_resize_direction_from_handle_top() {
        let dir = ResizeDirection::from_handle(ResizeHandle::Top);
        assert_eq!(dir.horizontal, 0);
        assert_eq!(dir.vertical, -1);
    }

    #[test]
    fn test_resize_direction_from_handle_bottom() {
        let dir = ResizeDirection::from_handle(ResizeHandle::Bottom);
        assert_eq!(dir.horizontal, 0);
        assert_eq!(dir.vertical, 1);
    }

    #[test]
    fn test_resize_direction_from_handle_left() {
        let dir = ResizeDirection::from_handle(ResizeHandle::Left);
        assert_eq!(dir.horizontal, -1);
        assert_eq!(dir.vertical, 0);
    }

    #[test]
    fn test_resize_direction_from_handle_right() {
        let dir = ResizeDirection::from_handle(ResizeHandle::Right);
        assert_eq!(dir.horizontal, 1);
        assert_eq!(dir.vertical, 0);
    }

    #[test]
    fn test_resize_direction_from_handle_top_left() {
        let dir = ResizeDirection::from_handle(ResizeHandle::TopLeft);
        assert_eq!(dir.horizontal, -1);
        assert_eq!(dir.vertical, -1);
    }

    #[test]
    fn test_resize_direction_from_handle_top_right() {
        let dir = ResizeDirection::from_handle(ResizeHandle::TopRight);
        assert_eq!(dir.horizontal, 1);
        assert_eq!(dir.vertical, -1);
    }

    #[test]
    fn test_resize_direction_from_handle_bottom_left() {
        let dir = ResizeDirection::from_handle(ResizeHandle::BottomLeft);
        assert_eq!(dir.horizontal, -1);
        assert_eq!(dir.vertical, 1);
    }

    #[test]
    fn test_resize_direction_from_handle_bottom_right() {
        let dir = ResizeDirection::from_handle(ResizeHandle::BottomRight);
        assert_eq!(dir.horizontal, 1);
        assert_eq!(dir.vertical, 1);
    }

    #[test]
    fn test_resize_direction_clone() {
        let dir1 = ResizeDirection {
            horizontal: 1,
            vertical: -1,
        };
        let dir2 = dir1;
        assert_eq!(dir1.horizontal, dir2.horizontal);
        assert_eq!(dir1.vertical, dir2.vertical);
    }

    #[test]
    fn test_resize_direction_copy() {
        let dir1 = ResizeDirection {
            horizontal: -1,
            vertical: 1,
        };
        let dir2 = dir1;
        assert_eq!(dir1, dir2);
    }

    #[test]
    fn test_resize_direction_equality() {
        let dir1 = ResizeDirection {
            horizontal: 1,
            vertical: -1,
        };
        let dir2 = ResizeDirection {
            horizontal: 1,
            vertical: -1,
        };
        assert_eq!(dir1.horizontal, dir2.horizontal);
        assert_eq!(dir1.vertical, dir2.vertical);
    }

    // =========================================================================
    // ResizeStyle tests
    // =========================================================================

    #[test]
    fn test_resize_style_default() {
        let style = ResizeStyle::default();
        assert_eq!(style, ResizeStyle::Border);
    }

    #[test]
    fn test_resize_style_all_variants() {
        let _ = ResizeStyle::Border;
        let _ = ResizeStyle::Subtle;
        let _ = ResizeStyle::Hidden;
        let _ = ResizeStyle::Dots;
    }

    #[test]
    fn test_resize_style_equality() {
        assert_eq!(ResizeStyle::Border, ResizeStyle::Border);
        assert_eq!(ResizeStyle::Subtle, ResizeStyle::Subtle);
        assert_eq!(ResizeStyle::Hidden, ResizeStyle::Hidden);
        assert_eq!(ResizeStyle::Dots, ResizeStyle::Dots);
    }

    #[test]
    fn test_resize_style_inequality() {
        assert_ne!(ResizeStyle::Border, ResizeStyle::Subtle);
        assert_ne!(ResizeStyle::Hidden, ResizeStyle::Dots);
        assert_ne!(ResizeStyle::Border, ResizeStyle::Hidden);
    }

    #[test]
    fn test_resize_style_clone() {
        let style = ResizeStyle::Dots;
        let cloned = style;
        assert_eq!(style, cloned);
    }

    #[test]
    fn test_resize_style_copy() {
        let style1 = ResizeStyle::Subtle;
        let style2 = style1;
        assert_eq!(style1, style2);
    }
}
