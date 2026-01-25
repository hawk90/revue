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
