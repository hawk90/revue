//! Main Style struct and inheritance implementation

use super::{
    layout::LayoutStyle, sizing::SizingStyle, spacing::SpacingStyle, types::*, visual::VisualStyle,
};

/// Style properties for a widget
///
/// Contains all CSS-like properties that can be applied to a widget.
/// Properties are organized into logical groups:
/// - `layout`: Display mode, flexbox, and grid properties
/// - `spacing`: Padding, margin, and position offsets
/// - `sizing`: Width, height, and min/max constraints
/// - `visual`: Colors, border, opacity, and visibility
///
/// For backward compatibility, individual properties can still be accessed
/// directly (e.g., `style.display` instead of `style.layout.display`).
#[derive(Debug, Clone, Default)]
pub struct Style {
    /// Layout properties (display, flex, grid)
    pub layout: LayoutStyle,
    /// Spacing properties (padding, margin, offsets)
    pub spacing: SpacingStyle,
    /// Sizing properties (width, height, min/max)
    pub sizing: SizingStyle,
    /// Visual properties (colors, border, opacity)
    pub visual: VisualStyle,
}

// Backward-compatible field accessors
impl Style {
    // Layout accessors
    /// Display mode (flex, block, grid, none) - non-inherited
    pub fn display(&self) -> Display {
        self.layout.display
    }
    /// Position mode (static, relative, absolute, fixed) - non-inherited
    pub fn position(&self) -> Position {
        self.layout.position
    }
    /// Flex direction (row, column) - non-inherited
    pub fn flex_direction(&self) -> FlexDirection {
        self.layout.flex_direction
    }
    /// Main axis alignment - non-inherited
    pub fn justify_content(&self) -> JustifyContent {
        self.layout.justify_content
    }
    /// Cross axis alignment - non-inherited
    pub fn align_items(&self) -> AlignItems {
        self.layout.align_items
    }
    /// Gap between flex/grid items - non-inherited
    pub fn gap(&self) -> u16 {
        self.layout.gap
    }
    /// Column gap for grid - non-inherited
    pub fn column_gap(&self) -> Option<u16> {
        self.layout.column_gap
    }
    /// Row gap for grid - non-inherited
    pub fn row_gap(&self) -> Option<u16> {
        self.layout.row_gap
    }
    /// Grid template columns - non-inherited
    pub fn grid_template_columns(&self) -> &GridTemplate {
        &self.layout.grid_template_columns
    }
    /// Grid template rows - non-inherited
    pub fn grid_template_rows(&self) -> &GridTemplate {
        &self.layout.grid_template_rows
    }
    /// Grid column placement - non-inherited
    pub fn grid_column(&self) -> GridPlacement {
        self.layout.grid_column
    }
    /// Grid row placement - non-inherited
    pub fn grid_row(&self) -> GridPlacement {
        self.layout.grid_row
    }

    // Spacing accessors
    /// Inner padding - non-inherited
    pub fn padding(&self) -> Spacing {
        self.spacing.padding
    }
    /// Outer margin - non-inherited
    pub fn margin(&self) -> Spacing {
        self.spacing.margin
    }
    /// Top offset - non-inherited
    pub fn top(&self) -> Option<i16> {
        self.spacing.top
    }
    /// Right offset - non-inherited
    pub fn right(&self) -> Option<i16> {
        self.spacing.right
    }
    /// Bottom offset - non-inherited
    pub fn bottom(&self) -> Option<i16> {
        self.spacing.bottom
    }
    /// Left offset - non-inherited
    pub fn left(&self) -> Option<i16> {
        self.spacing.left
    }

    // Sizing accessors
    /// Width constraint - non-inherited
    pub fn width(&self) -> Size {
        self.sizing.width
    }
    /// Height constraint - non-inherited
    pub fn height(&self) -> Size {
        self.sizing.height
    }
    /// Minimum width - non-inherited
    pub fn min_width(&self) -> Size {
        self.sizing.min_width
    }
    /// Maximum width - non-inherited
    pub fn max_width(&self) -> Size {
        self.sizing.max_width
    }
    /// Minimum height - non-inherited
    pub fn min_height(&self) -> Size {
        self.sizing.min_height
    }
    /// Maximum height - non-inherited
    pub fn max_height(&self) -> Size {
        self.sizing.max_height
    }

    // Visual accessors
    /// Border style - non-inherited
    pub fn border_style(&self) -> BorderStyle {
        self.visual.border_style
    }
    /// Border color - non-inherited
    pub fn border_color(&self) -> Color {
        self.visual.border_color
    }
    /// Text/foreground color - INHERITED
    pub fn color(&self) -> Color {
        self.visual.color
    }
    /// Background color - non-inherited
    pub fn background(&self) -> Color {
        self.visual.background
    }
    /// Opacity (0.0 to 1.0) - INHERITED
    pub fn opacity(&self) -> f32 {
        self.visual.opacity
    }
    /// Visibility flag - INHERITED
    pub fn visible(&self) -> bool {
        self.visual.visible
    }
    /// Z-index for stacking order - non-inherited
    pub fn z_index(&self) -> i16 {
        self.visual.z_index
    }
}

impl Style {
    /// Inherit inheritable properties from parent style
    ///
    /// CSS Inherited Properties:
    /// - `color` - text color
    /// - `opacity` - visual opacity
    /// - `visible` - visibility
    ///
    /// Non-inherited properties are reset to their defaults.
    pub fn inherit(parent: &Style) -> Self {
        Self {
            layout: LayoutStyle::default(),
            spacing: SpacingStyle::default(),
            sizing: SizingStyle::default(),
            visual: VisualStyle {
                // Inherited properties from parent
                color: parent.visual.color,
                opacity: parent.visual.opacity,
                visible: parent.visual.visible,
                // Non-inherited - use defaults
                ..VisualStyle::default()
            },
        }
    }

    /// Inherit from parent, then apply own rules
    ///
    /// This is the main method for computing inherited styles:
    /// 1. Start with inherited values from parent
    /// 2. Apply this style's explicitly set values
    pub fn with_inheritance(&self, parent: &Style) -> Self {
        let mut result = Self::inherit(parent);

        // Apply own values (only if explicitly set)
        // Layout
        if self.layout.display != Display::default() {
            result.layout.display = self.layout.display;
        }
        if self.layout.position != Position::default() {
            result.layout.position = self.layout.position;
        }
        if self.layout.flex_direction != FlexDirection::default() {
            result.layout.flex_direction = self.layout.flex_direction;
        }
        if self.layout.justify_content != JustifyContent::default() {
            result.layout.justify_content = self.layout.justify_content;
        }
        if self.layout.align_items != AlignItems::default() {
            result.layout.align_items = self.layout.align_items;
        }
        if self.layout.gap != 0 {
            result.layout.gap = self.layout.gap;
        }
        if self.layout.column_gap.is_some() {
            result.layout.column_gap = self.layout.column_gap;
        }
        if self.layout.row_gap.is_some() {
            result.layout.row_gap = self.layout.row_gap;
        }

        // Grid
        if !self.layout.grid_template_columns.tracks.is_empty() {
            result.layout.grid_template_columns = self.layout.grid_template_columns.clone();
        }
        if !self.layout.grid_template_rows.tracks.is_empty() {
            result.layout.grid_template_rows = self.layout.grid_template_rows.clone();
        }
        if self.layout.grid_column != GridPlacement::default() {
            result.layout.grid_column = self.layout.grid_column;
        }
        if self.layout.grid_row != GridPlacement::default() {
            result.layout.grid_row = self.layout.grid_row;
        }

        // Position offsets
        if self.spacing.top.is_some() {
            result.spacing.top = self.spacing.top;
        }
        if self.spacing.right.is_some() {
            result.spacing.right = self.spacing.right;
        }
        if self.spacing.bottom.is_some() {
            result.spacing.bottom = self.spacing.bottom;
        }
        if self.spacing.left.is_some() {
            result.spacing.left = self.spacing.left;
        }

        // Spacing
        if self.spacing.padding != Spacing::default() {
            result.spacing.padding = self.spacing.padding;
        }
        if self.spacing.margin != Spacing::default() {
            result.spacing.margin = self.spacing.margin;
        }

        // Size
        if self.sizing.width != Size::default() {
            result.sizing.width = self.sizing.width;
        }
        if self.sizing.height != Size::default() {
            result.sizing.height = self.sizing.height;
        }
        if self.sizing.min_width != Size::default() {
            result.sizing.min_width = self.sizing.min_width;
        }
        if self.sizing.min_height != Size::default() {
            result.sizing.min_height = self.sizing.min_height;
        }
        if self.sizing.max_width != Size::default() {
            result.sizing.max_width = self.sizing.max_width;
        }
        if self.sizing.max_height != Size::default() {
            result.sizing.max_height = self.sizing.max_height;
        }

        // Border
        if self.visual.border_style != BorderStyle::default() {
            result.visual.border_style = self.visual.border_style;
        }
        if self.visual.border_color != Color::default() {
            result.visual.border_color = self.visual.border_color;
        }

        // Colors - color inherits, background doesn't
        if self.visual.color != Color::default() {
            result.visual.color = self.visual.color;
        }
        if self.visual.background != Color::default() {
            result.visual.background = self.visual.background;
        }

        // Visual - these inherit but can be overridden
        if self.visual.opacity != 1.0 {
            result.visual.opacity = self.visual.opacity;
        }
        if !self.visual.visible {
            result.visual.visible = self.visual.visible;
        }
        if self.visual.z_index != 0 {
            result.visual.z_index = self.visual.z_index;
        }

        result
    }
}
