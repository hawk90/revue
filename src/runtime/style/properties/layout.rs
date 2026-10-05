//! Layout-related style property structures

use super::types::{
    AlignItems, AlignSelf, Display, FlexDirection, FlexWrap, GridPlacement, GridTemplate,
    JustifyContent, Position,
};

/// Layout-related style properties
///
/// Contains display mode, flexbox, and grid layout properties.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LayoutStyle {
    /// Display mode (flex, block, grid, none)
    pub display: Display,
    /// Position mode (static, relative, absolute, fixed)
    pub position: Position,
    /// Flex direction (row, column)
    pub flex_direction: FlexDirection,
    /// Main axis alignment
    pub justify_content: JustifyContent,
    /// Cross axis alignment
    pub align_items: AlignItems,
    /// Flex grow factor (distributes remaining space proportionally)
    pub flex_grow: f32,
    /// Flex wrap behavior
    pub flex_wrap: FlexWrap,
    /// Individual item cross-axis alignment
    pub align_self: AlignSelf,
    /// Item order (lower values rendered first)
    pub order: i16,
    /// Gap between flex/grid items
    ///
    /// `None` is "not specified", which is not the same as `Some(0)`. A plain
    /// `u16` could not tell those apart, so a stylesheet saying `gap: 0` read
    /// as saying nothing and the builder's own gap survived. `column_gap` and
    /// `row_gap` were already `Option` and did not have that problem.
    pub gap: Option<u16>,
    /// Column gap for grid
    pub column_gap: Option<u16>,
    /// Row gap for grid
    pub row_gap: Option<u16>,
    /// Grid template columns
    pub grid_template_columns: GridTemplate,
    /// Grid template rows
    pub grid_template_rows: GridTemplate,
    /// Grid column placement
    pub grid_column: GridPlacement,
    /// Grid row placement
    pub grid_row: GridPlacement,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_style_default() {
        let layout = LayoutStyle::default();
        assert_eq!(layout.display, Display::default());
        assert_eq!(layout.position, Position::default());
        assert_eq!(layout.flex_direction, FlexDirection::default());
        assert_eq!(layout.justify_content, JustifyContent::default());
        assert_eq!(layout.align_items, AlignItems::default());
        assert_eq!(layout.gap, None);
        assert_eq!(layout.column_gap, None);
        assert_eq!(layout.row_gap, None);
    }

    #[test]
    fn test_layout_style_clone() {
        let layout = LayoutStyle {
            gap: Some(10),
            ..Default::default()
        };
        let cloned = layout.clone();
        assert_eq!(cloned.gap, Some(10));
    }

    #[test]
    fn test_layout_style_partial_eq() {
        let layout1 = LayoutStyle::default();
        let layout2 = LayoutStyle::default();
        assert_eq!(layout1, layout2);
    }

    #[test]
    fn test_layout_style_not_equal() {
        let layout1 = LayoutStyle {
            gap: Some(10),
            ..Default::default()
        };
        let layout2 = LayoutStyle::default();
        assert_ne!(layout1, layout2);
    }

    #[test]
    fn test_layout_style_grid_template() {
        let layout = LayoutStyle {
            grid_template_columns: GridTemplate::fr(&[1.0, 2.0]),
            ..Default::default()
        };
        assert_eq!(layout.grid_template_columns.tracks.len(), 2);
    }

    #[test]
    fn test_layout_style_grid_placement() {
        let layout = LayoutStyle {
            grid_column: GridPlacement::span(2),
            grid_row: GridPlacement::from_to(1, 3),
            ..Default::default()
        };
        assert_eq!(layout.grid_column, GridPlacement::span(2));
        assert_eq!(layout.grid_row, GridPlacement::from_to(1, 3));
    }

    #[test]
    fn test_layout_style_debug() {
        let layout = LayoutStyle::default();
        let debug_str = format!("{:?}", layout);
        assert!(debug_str.contains("LayoutStyle"));
    }
}
