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
