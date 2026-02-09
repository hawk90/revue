//! Layout-related style property structures

use super::types::{
    AlignItems, Display, FlexDirection, GridPlacement, GridTemplate, JustifyContent, Position,
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
    /// Gap between flex/grid items
    pub gap: u16,
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
        assert_eq!(layout.gap, 0);
        assert_eq!(layout.column_gap, None);
        assert_eq!(layout.row_gap, None);
    }

    #[test]
    fn test_layout_style_clone() {
        let mut layout = LayoutStyle::default();
        layout.gap = 10;
        let cloned = layout.clone();
        assert_eq!(cloned.gap, 10);
    }

    #[test]
    fn test_layout_style_partial_eq() {
        let layout1 = LayoutStyle::default();
        let layout2 = LayoutStyle::default();
        assert_eq!(layout1, layout2);
    }

    #[test]
    fn test_layout_style_not_equal() {
        let mut layout1 = LayoutStyle::default();
        layout1.gap = 10;
        let layout2 = LayoutStyle::default();
        assert_ne!(layout1, layout2);
    }

    #[test]
    fn test_layout_style_grid_template() {
        let mut layout = LayoutStyle::default();
        layout.grid_template_columns = GridTemplate::fr(&[1.0, 2.0]);
        assert_eq!(layout.grid_template_columns.tracks.len(), 2);
    }

    #[test]
    fn test_layout_style_grid_placement() {
        let mut layout = LayoutStyle::default();
        layout.grid_column = GridPlacement::span(2);
        layout.grid_row = GridPlacement::from_to(1, 3);
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
