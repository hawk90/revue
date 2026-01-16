//! Layout node data structures
//!
//! Internal representation of layout nodes used by the custom layout engine.

use crate::style::{
    AlignItems, Display, FlexDirection, GridPlacement, GridTrack, JustifyContent, Position, Size,
    Spacing,
};

/// A node in the layout tree
#[derive(Debug, Clone, Default)]
pub struct LayoutNode {
    /// Node identifier (matches DomId)
    pub id: u64,

    /// Display mode
    pub display: Display,

    /// Position mode
    pub position: Position,

    /// Flexbox properties
    pub flex: FlexProps,

    /// Grid properties (only for grid containers/items)
    pub grid: GridProps,

    /// Spacing (padding, margin, insets)
    pub spacing: LayoutSpacing,

    /// Size constraints
    pub sizing: SizeConstraints,

    /// Child node IDs (in order)
    pub children: Vec<u64>,

    /// Parent node ID (None for root)
    pub parent: Option<u64>,

    /// Computed layout result (filled after compute)
    pub computed: ComputedLayout,
}

/// Flexbox layout properties
#[derive(Debug, Clone, Default)]
pub struct FlexProps {
    /// Main axis direction
    pub direction: FlexDirection,
    /// Main axis alignment
    pub justify_content: JustifyContent,
    /// Cross axis alignment
    pub align_items: AlignItems,
    /// Gap between items
    pub gap: u16,
    /// Column gap (overrides gap for horizontal)
    pub column_gap: Option<u16>,
    /// Row gap (overrides gap for vertical)
    pub row_gap: Option<u16>,
}

impl FlexProps {
    /// Get effective gap for main axis
    pub fn main_gap(&self) -> u16 {
        match self.direction {
            FlexDirection::Row => self.column_gap.unwrap_or(self.gap),
            FlexDirection::Column => self.row_gap.unwrap_or(self.gap),
        }
    }

    /// Get effective gap for cross axis
    #[allow(dead_code)]
    pub fn cross_gap(&self) -> u16 {
        match self.direction {
            FlexDirection::Row => self.row_gap.unwrap_or(self.gap),
            FlexDirection::Column => self.column_gap.unwrap_or(self.gap),
        }
    }
}

/// Grid layout properties
#[derive(Debug, Clone, Default)]
pub struct GridProps {
    /// Column track definitions
    pub template_columns: Vec<GridTrack>,
    /// Row track definitions
    pub template_rows: Vec<GridTrack>,
    /// Column placement for grid items
    pub column: GridPlacement,
    /// Row placement for grid items
    pub row: GridPlacement,
}

/// Spacing for padding, margin, and position insets
#[derive(Debug, Clone, Copy, Default)]
pub struct LayoutSpacing {
    /// Inner padding
    pub padding: Edges,
    /// Outer margin
    pub margin: Edges,
    /// Position insets (for absolute/relative/fixed)
    pub inset: Inset,
}

/// Edge values (top, right, bottom, left)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Edges {
    pub top: u16,
    pub right: u16,
    pub bottom: u16,
    pub left: u16,
}

impl Edges {
    /// Total horizontal spacing
    pub fn horizontal(&self) -> u16 {
        self.left.saturating_add(self.right)
    }

    /// Total vertical spacing
    #[allow(dead_code)]
    pub fn vertical(&self) -> u16 {
        self.top.saturating_add(self.bottom)
    }
}

impl From<Spacing> for Edges {
    fn from(s: Spacing) -> Self {
        Self {
            top: s.top,
            right: s.right,
            bottom: s.bottom,
            left: s.left,
        }
    }
}

/// Position insets (optional, for positioned elements)
#[derive(Debug, Clone, Copy, Default)]
pub struct Inset {
    pub top: Option<i16>,
    pub right: Option<i16>,
    pub bottom: Option<i16>,
    pub left: Option<i16>,
}

/// Size constraints for a node
#[derive(Debug, Clone, Copy, Default)]
pub struct SizeConstraints {
    pub width: Size,
    pub height: Size,
    pub min_width: Size,
    pub max_width: Size,
    pub min_height: Size,
    pub max_height: Size,
}

/// Computed layout result for a node
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ComputedLayout {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl ComputedLayout {
    /// Create a new computed layout
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Content width (inner area excluding padding)
    #[allow(dead_code)]
    pub fn content_width(&self, padding: &Edges) -> u16 {
        self.width
            .saturating_sub(padding.left)
            .saturating_sub(padding.right)
    }

    /// Content height (inner area excluding padding)
    #[allow(dead_code)]
    pub fn content_height(&self, padding: &Edges) -> u16 {
        self.height
            .saturating_sub(padding.top)
            .saturating_sub(padding.bottom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edges_from_spacing() {
        let spacing = Spacing::new(1, 2, 3, 4);
        let edges: Edges = spacing.into();
        assert_eq!(edges.top, 1);
        assert_eq!(edges.right, 2);
        assert_eq!(edges.bottom, 3);
        assert_eq!(edges.left, 4);
    }

    #[test]
    fn test_edges_horizontal_vertical() {
        let edges = Edges {
            top: 5,
            right: 10,
            bottom: 15,
            left: 20,
        };
        assert_eq!(edges.horizontal(), 30);
        assert_eq!(edges.vertical(), 20);
    }

    #[test]
    fn test_flex_props_gap() {
        let mut props = FlexProps::default();
        props.gap = 5;
        props.direction = FlexDirection::Row;

        // Default uses gap
        assert_eq!(props.main_gap(), 5);
        assert_eq!(props.cross_gap(), 5);

        // Override column gap
        props.column_gap = Some(10);
        assert_eq!(props.main_gap(), 10); // Row direction uses column_gap
        assert_eq!(props.cross_gap(), 5);

        // Switch to column direction
        props.direction = FlexDirection::Column;
        assert_eq!(props.main_gap(), 5); // Column direction uses row_gap
        assert_eq!(props.cross_gap(), 10);
    }

    #[test]
    fn test_computed_layout_content() {
        let layout = ComputedLayout::new(0, 0, 100, 50);
        let padding = Edges {
            top: 5,
            right: 10,
            bottom: 5,
            left: 10,
        };
        assert_eq!(layout.content_width(&padding), 80);
        assert_eq!(layout.content_height(&padding), 40);
    }
}
