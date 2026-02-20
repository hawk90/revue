//! Layout node data structures
//!
//! Internal representation of layout nodes used by the custom layout engine.

use crate::style::{
    AlignItems, Display, FlexDirection, GridPlacement, GridTrack, JustifyContent, Position, Size,
    Spacing,
};

/// A node in the layout tree
#[derive(Debug, Clone)]
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

    /// Dirty flag for incremental layout updates
    /// When true, this node needs layout recalculation
    pub dirty: bool,
}

impl Default for LayoutNode {
    fn default() -> Self {
        Self {
            id: 0,
            display: Display::default(),
            position: Position::default(),
            flex: FlexProps::default(),
            grid: GridProps::default(),
            spacing: LayoutSpacing::default(),
            sizing: SizeConstraints::default(),
            children: Vec::new(),
            parent: None,
            computed: ComputedLayout::default(),
            dirty: true, // New nodes are always dirty
        }
    }
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
    /// Flex grow factor (distributes remaining space proportionally)
    pub flex_grow: f32,
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

    // =========================================================================
    // LayoutNode tests
    // =========================================================================

    #[test]
    fn test_layout_node_default() {
        let node = LayoutNode::default();
        assert_eq!(node.id, 0);
        assert!(node.children.is_empty());
        assert!(node.parent.is_none());
    }

    #[test]
    fn test_layout_node_with_children() {
        let mut node = LayoutNode::default();
        node.id = 1;
        node.children = vec![2, 3, 4];

        assert_eq!(node.children.len(), 3);
        assert!(node.children.contains(&2));
    }

    #[test]
    fn test_layout_node_with_parent() {
        let mut node = LayoutNode::default();
        node.id = 5;
        node.parent = Some(1);

        assert_eq!(node.parent, Some(1));
    }

    #[test]
    fn test_layout_node_clone() {
        let mut node = LayoutNode::default();
        node.id = 10;
        node.children = vec![20, 30];

        let cloned = node.clone();
        assert_eq!(cloned.id, 10);
        assert_eq!(cloned.children, vec![20, 30]);
    }

    // =========================================================================
    // FlexProps tests
    // =========================================================================

    #[test]
    fn test_flex_props_default() {
        let props = FlexProps::default();
        assert_eq!(props.gap, 0);
        assert_eq!(props.column_gap, None);
        assert_eq!(props.row_gap, None);
    }

    #[test]
    fn test_flex_props_main_gap_row() {
        let mut props = FlexProps::default();
        props.direction = FlexDirection::Row;
        props.gap = 8;

        assert_eq!(props.main_gap(), 8);

        props.column_gap = Some(16);
        assert_eq!(props.main_gap(), 16);
    }

    #[test]
    fn test_flex_props_main_gap_column() {
        let mut props = FlexProps::default();
        props.direction = FlexDirection::Column;
        props.gap = 8;

        assert_eq!(props.main_gap(), 8);

        props.row_gap = Some(12);
        assert_eq!(props.main_gap(), 12);
    }

    #[test]
    fn test_flex_props_cross_gap_row() {
        let mut props = FlexProps::default();
        props.direction = FlexDirection::Row;
        props.gap = 8;

        assert_eq!(props.cross_gap(), 8);

        props.row_gap = Some(4);
        assert_eq!(props.cross_gap(), 4);
    }

    #[test]
    fn test_flex_props_cross_gap_column() {
        let mut props = FlexProps::default();
        props.direction = FlexDirection::Column;
        props.gap = 8;

        assert_eq!(props.cross_gap(), 8);

        props.column_gap = Some(20);
        assert_eq!(props.cross_gap(), 20);
    }

    // =========================================================================
    // GridProps tests
    // =========================================================================

    #[test]
    fn test_grid_props_default() {
        let props = GridProps::default();
        assert!(props.template_columns.is_empty());
        assert!(props.template_rows.is_empty());
    }

    #[test]
    fn test_grid_props_clone() {
        let props = GridProps::default();
        let cloned = props.clone();
        assert!(cloned.template_columns.is_empty());
    }

    // =========================================================================
    // LayoutSpacing tests
    // =========================================================================

    #[test]
    fn test_layout_spacing_default() {
        let spacing = LayoutSpacing::default();
        assert_eq!(spacing.padding.top, 0);
        assert_eq!(spacing.margin.left, 0);
    }

    #[test]
    fn test_layout_spacing_copy() {
        let spacing = LayoutSpacing {
            padding: Edges {
                top: 1,
                right: 2,
                bottom: 3,
                left: 4,
            },
            margin: Edges::default(),
            inset: Inset::default(),
        };
        let copied = spacing;
        assert_eq!(copied.padding.top, 1);
    }

    // =========================================================================
    // Edges tests
    // =========================================================================

    #[test]
    fn test_edges_default() {
        let edges = Edges::default();
        assert_eq!(edges.top, 0);
        assert_eq!(edges.right, 0);
        assert_eq!(edges.bottom, 0);
        assert_eq!(edges.left, 0);
    }

    #[test]
    fn test_edges_horizontal() {
        let edges = Edges {
            top: 0,
            right: 15,
            bottom: 0,
            left: 5,
        };
        assert_eq!(edges.horizontal(), 20);
    }

    #[test]
    fn test_edges_vertical() {
        let edges = Edges {
            top: 10,
            right: 0,
            bottom: 20,
            left: 0,
        };
        assert_eq!(edges.vertical(), 30);
    }

    #[test]
    fn test_edges_horizontal_saturating() {
        let edges = Edges {
            top: 0,
            right: u16::MAX,
            bottom: 0,
            left: 100,
        };
        // Should saturate instead of overflow
        assert_eq!(edges.horizontal(), u16::MAX);
    }

    #[test]
    fn test_edges_vertical_saturating() {
        let edges = Edges {
            top: u16::MAX,
            right: 0,
            bottom: 50,
            left: 0,
        };
        assert_eq!(edges.vertical(), u16::MAX);
    }

    #[test]
    fn test_edges_equality() {
        let e1 = Edges {
            top: 1,
            right: 2,
            bottom: 3,
            left: 4,
        };
        let e2 = Edges {
            top: 1,
            right: 2,
            bottom: 3,
            left: 4,
        };
        let e3 = Edges {
            top: 0,
            right: 0,
            bottom: 0,
            left: 0,
        };

        assert_eq!(e1, e2);
        assert_ne!(e1, e3);
    }

    // =========================================================================
    // Inset tests
    // =========================================================================

    #[test]
    fn test_inset_default() {
        let inset = Inset::default();
        assert_eq!(inset.top, None);
        assert_eq!(inset.right, None);
        assert_eq!(inset.bottom, None);
        assert_eq!(inset.left, None);
    }

    #[test]
    fn test_inset_with_values() {
        let inset = Inset {
            top: Some(10),
            right: Some(-5),
            bottom: None,
            left: Some(20),
        };
        assert_eq!(inset.top, Some(10));
        assert_eq!(inset.right, Some(-5));
        assert_eq!(inset.bottom, None);
        assert_eq!(inset.left, Some(20));
    }

    #[test]
    fn test_inset_copy() {
        let inset = Inset {
            top: Some(5),
            right: None,
            bottom: None,
            left: None,
        };
        let copied = inset;
        assert_eq!(copied.top, Some(5));
    }

    // =========================================================================
    // SizeConstraints tests
    // =========================================================================

    #[test]
    fn test_size_constraints_default() {
        let constraints = SizeConstraints::default();
        assert_eq!(constraints.width, Size::Auto);
        assert_eq!(constraints.height, Size::Auto);
    }

    #[test]
    fn test_size_constraints_copy() {
        let constraints = SizeConstraints {
            width: Size::Fixed(100),
            height: Size::Fixed(50),
            ..Default::default()
        };
        let copied = constraints;
        assert_eq!(copied.width, Size::Fixed(100));
    }

    // =========================================================================
    // ComputedLayout tests
    // =========================================================================

    #[test]
    fn test_computed_layout_default() {
        let layout = ComputedLayout::default();
        assert_eq!(layout.x, 0);
        assert_eq!(layout.y, 0);
        assert_eq!(layout.width, 0);
        assert_eq!(layout.height, 0);
    }

    #[test]
    fn test_computed_layout_new() {
        let layout = ComputedLayout::new(10, 20, 100, 50);
        assert_eq!(layout.x, 10);
        assert_eq!(layout.y, 20);
        assert_eq!(layout.width, 100);
        assert_eq!(layout.height, 50);
    }

    #[test]
    fn test_computed_layout_content_width() {
        let layout = ComputedLayout::new(0, 0, 80, 40);
        let padding = Edges {
            top: 0,
            right: 10,
            bottom: 0,
            left: 10,
        };
        assert_eq!(layout.content_width(&padding), 60);
    }

    #[test]
    fn test_computed_layout_content_height() {
        let layout = ComputedLayout::new(0, 0, 80, 40);
        let padding = Edges {
            top: 5,
            right: 0,
            bottom: 5,
            left: 0,
        };
        assert_eq!(layout.content_height(&padding), 30);
    }

    #[test]
    fn test_computed_layout_content_saturating() {
        let layout = ComputedLayout::new(0, 0, 10, 10);
        let padding = Edges {
            top: 20,
            right: 30,
            bottom: 20,
            left: 30,
        };
        // Should not underflow
        assert_eq!(layout.content_width(&padding), 0);
        assert_eq!(layout.content_height(&padding), 0);
    }

    #[test]
    fn test_computed_layout_equality() {
        let l1 = ComputedLayout::new(0, 0, 100, 50);
        let l2 = ComputedLayout::new(0, 0, 100, 50);
        let l3 = ComputedLayout::new(10, 10, 100, 50);

        assert_eq!(l1, l2);
        assert_ne!(l1, l3);
    }

    #[test]
    fn test_computed_layout_copy() {
        let layout = ComputedLayout::new(5, 10, 200, 100);
        let copied = layout;
        assert_eq!(copied.x, 5);
        assert_eq!(copied.y, 10);
    }
}
