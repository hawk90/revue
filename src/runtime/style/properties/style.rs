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
        // Only override visible if child's value is not the default (true)
        if self.visual.visible != VisualStyle::default().visible {
            result.visual.visible = self.visual.visible;
        }
        if self.visual.z_index != 0 {
            result.visual.z_index = self.visual.z_index;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Layout accessor tests
    #[test]
    fn test_style_display_accessor() {
        let mut style = Style::default();
        style.layout.display = Display::Grid;
        assert_eq!(style.display(), Display::Grid);
    }

    #[test]
    fn test_style_position_accessor() {
        let mut style = Style::default();
        style.layout.position = Position::Absolute;
        assert_eq!(style.position(), Position::Absolute);
    }

    #[test]
    fn test_style_flex_direction_accessor() {
        let mut style = Style::default();
        style.layout.flex_direction = FlexDirection::Column;
        assert_eq!(style.flex_direction(), FlexDirection::Column);
    }

    #[test]
    fn test_style_justify_content_accessor() {
        let mut style = Style::default();
        style.layout.justify_content = JustifyContent::Center;
        assert_eq!(style.justify_content(), JustifyContent::Center);
    }

    #[test]
    fn test_style_align_items_accessor() {
        let mut style = Style::default();
        style.layout.align_items = AlignItems::Stretch;
        assert_eq!(style.align_items(), AlignItems::Stretch);
    }

    #[test]
    fn test_style_gap_accessor() {
        let mut style = Style::default();
        style.layout.gap = 10;
        assert_eq!(style.gap(), 10);
    }

    #[test]
    fn test_style_column_gap_accessor() {
        let mut style = Style::default();
        style.layout.column_gap = Some(5);
        assert_eq!(style.column_gap(), Some(5));
    }

    #[test]
    fn test_style_row_gap_accessor() {
        let mut style = Style::default();
        style.layout.row_gap = Some(5);
        assert_eq!(style.row_gap(), Some(5));
    }

    #[test]
    fn test_style_grid_template_columns_accessor() {
        let mut style = Style::default();
        let template = GridTemplate::fr(&[1.0, 2.0]);
        style.layout.grid_template_columns = template.clone();
        assert_eq!(style.grid_template_columns(), &template);
    }

    #[test]
    fn test_style_grid_template_rows_accessor() {
        let mut style = Style::default();
        let template = GridTemplate::fr(&[1.0, 2.0]);
        style.layout.grid_template_rows = template.clone();
        assert_eq!(style.grid_template_rows(), &template);
    }

    #[test]
    fn test_style_grid_column_accessor() {
        let mut style = Style::default();
        style.layout.grid_column = GridPlacement::span(2);
        assert_eq!(style.grid_column(), GridPlacement::span(2));
    }

    #[test]
    fn test_style_grid_row_accessor() {
        let mut style = Style::default();
        style.layout.grid_row = GridPlacement::span(2);
        assert_eq!(style.grid_row(), GridPlacement::span(2));
    }

    // Spacing accessor tests
    #[test]
    fn test_style_padding_accessor() {
        let mut style = Style::default();
        style.spacing.padding = Spacing::all(10);
        assert_eq!(style.padding(), Spacing::all(10));
    }

    #[test]
    fn test_style_margin_accessor() {
        let mut style = Style::default();
        style.spacing.margin = Spacing::all(10);
        assert_eq!(style.margin(), Spacing::all(10));
    }

    #[test]
    fn test_style_top_accessor() {
        let mut style = Style::default();
        style.spacing.top = Some(10);
        assert_eq!(style.top(), Some(10));
    }

    #[test]
    fn test_style_right_accessor() {
        let mut style = Style::default();
        style.spacing.right = Some(10);
        assert_eq!(style.right(), Some(10));
    }

    #[test]
    fn test_style_bottom_accessor() {
        let mut style = Style::default();
        style.spacing.bottom = Some(10);
        assert_eq!(style.bottom(), Some(10));
    }

    #[test]
    fn test_style_left_accessor() {
        let mut style = Style::default();
        style.spacing.left = Some(10);
        assert_eq!(style.left(), Some(10));
    }

    // Sizing accessor tests
    #[test]
    fn test_style_width_accessor() {
        let mut style = Style::default();
        style.sizing.width = Size::Fixed(100);
        assert_eq!(style.width(), Size::Fixed(100));
    }

    #[test]
    fn test_style_height_accessor() {
        let mut style = Style::default();
        style.sizing.height = Size::Fixed(100);
        assert_eq!(style.height(), Size::Fixed(100));
    }

    #[test]
    fn test_style_min_width_accessor() {
        let mut style = Style::default();
        style.sizing.min_width = Size::Fixed(50);
        assert_eq!(style.min_width(), Size::Fixed(50));
    }

    #[test]
    fn test_style_max_width_accessor() {
        let mut style = Style::default();
        style.sizing.max_width = Size::Fixed(200);
        assert_eq!(style.max_width(), Size::Fixed(200));
    }

    #[test]
    fn test_style_min_height_accessor() {
        let mut style = Style::default();
        style.sizing.min_height = Size::Fixed(50);
        assert_eq!(style.min_height(), Size::Fixed(50));
    }

    #[test]
    fn test_style_max_height_accessor() {
        let mut style = Style::default();
        style.sizing.max_height = Size::Fixed(200);
        assert_eq!(style.max_height(), Size::Fixed(200));
    }

    // Visual accessor tests
    #[test]
    fn test_style_border_style_accessor() {
        let mut style = Style::default();
        style.visual.border_style = BorderStyle::Solid;
        assert_eq!(style.border_style(), BorderStyle::Solid);
    }

    #[test]
    fn test_style_border_color_accessor() {
        let mut style = Style::default();
        style.visual.border_color = Color::RED;
        assert_eq!(style.border_color(), Color::RED);
    }

    #[test]
    fn test_style_color_accessor() {
        let mut style = Style::default();
        style.visual.color = Color::BLUE;
        assert_eq!(style.color(), Color::BLUE);
    }

    #[test]
    fn test_style_background_accessor() {
        let mut style = Style::default();
        style.visual.background = Color::WHITE;
        assert_eq!(style.background(), Color::WHITE);
    }

    #[test]
    fn test_style_opacity_accessor() {
        let mut style = Style::default();
        style.visual.opacity = 0.5;
        assert_eq!(style.opacity(), 0.5);
    }

    #[test]
    fn test_style_visible_accessor() {
        let mut style = Style::default();
        style.visual.visible = false;
        assert_eq!(style.visible(), false);
    }

    #[test]
    fn test_style_z_index_accessor() {
        let mut style = Style::default();
        style.visual.z_index = 10;
        assert_eq!(style.z_index(), 10);
    }

    // Inheritance tests
    #[test]
    fn test_style_inherit_creates_new_style() {
        let parent = Style::default();
        let child = Style::inherit(&parent);
        // Child should be a new style
        assert_eq!(child.layout.display, Display::default());
        assert_eq!(child.spacing.padding, Spacing::default());
    }

    #[test]
    fn test_style_inherit_copies_inherited_properties() {
        let mut parent = Style::default();
        parent.visual.color = Color::RED;
        parent.visual.opacity = 0.5;
        parent.visual.visible = false;

        let child = Style::inherit(&parent);
        assert_eq!(child.visual.color, Color::RED);
        assert_eq!(child.visual.opacity, 0.5);
        assert_eq!(child.visual.visible, false);
    }

    #[test]
    fn test_style_inherit_resets_non_inherited() {
        let mut parent = Style::default();
        parent.layout.gap = 10;
        parent.layout.display = Display::Grid;
        parent.spacing.padding = Spacing::all(5);

        let child = Style::inherit(&parent);
        assert_eq!(child.layout.gap, 0);
        assert_eq!(child.layout.display, Display::default());
        assert_eq!(child.spacing.padding, Spacing::default());
    }

    #[test]
    fn test_style_with_inheritance_merges_styles() {
        let mut parent = Style::default();
        parent.visual.color = Color::RED;

        let mut child = Style::default();
        child.layout.gap = 10;

        let result = child.with_inheritance(&parent);
        assert_eq!(result.visual.color, Color::RED);
        assert_eq!(result.layout.gap, 10);
    }

    #[test]
    fn test_style_with_inheritance_child_overrides_parent() {
        let mut parent = Style::default();
        parent.visual.color = Color::RED;

        let mut child = Style::default();
        child.visual.color = Color::BLUE;

        let result = child.with_inheritance(&parent);
        assert_eq!(result.visual.color, Color::BLUE);
    }

    #[test]
    fn test_style_with_inheritance_default_child_uses_parent() {
        let mut parent = Style::default();
        parent.visual.color = Color::RED;

        let child = Style::default();

        let result = child.with_inheritance(&parent);
        assert_eq!(result.visual.color, Color::RED);
    }

    #[test]
    fn test_style_with_inheritance_layout_not_inherited() {
        let mut parent = Style::default();
        parent.layout.gap = 10;
        parent.layout.display = Display::Grid;

        let child = Style::default();
        let result = child.with_inheritance(&parent);
        assert_eq!(result.layout.gap, 0);
        assert_eq!(result.layout.display, Display::default());
    }

    #[test]
    fn test_style_with_inheritance_explicit_layout_preserved() {
        let mut parent = Style::default();
        parent.layout.gap = 10;

        let mut child = Style::default();
        child.layout.gap = 20;

        let result = child.with_inheritance(&parent);
        assert_eq!(result.layout.gap, 20);
    }

    #[test]
    fn test_style_with_inheritance_opacity_inherited() {
        let mut parent = Style::default();
        parent.visual.opacity = 0.5;

        let child = Style::default();
        let result = child.with_inheritance(&parent);
        assert_eq!(result.visual.opacity, 0.5);
    }

    #[test]
    fn test_style_with_inheritance_opacity_override() {
        let mut parent = Style::default();
        parent.visual.opacity = 0.5;

        let mut child = Style::default();
        child.visual.opacity = 0.8;

        let result = child.with_inheritance(&parent);
        assert_eq!(result.visual.opacity, 0.8);
    }

    #[test]
    fn test_style_with_inheritance_visible_inherited() {
        let mut parent = Style::default();
        parent.visual.visible = false;

        let child = Style::default();
        let result = child.with_inheritance(&parent);
        assert_eq!(result.visual.visible, false);
    }

    #[test]
    fn test_style_with_inheritance_visible_override() {
        let mut parent = Style::default();
        parent.visual.visible = true; // Parent is visible

        let mut child = Style::default();
        child.visual.visible = false; // Child explicitly sets to hidden (not default)

        let result = child.with_inheritance(&parent);
        assert_eq!(result.visual.visible, false); // Child's non-default value should override
    }

    #[test]
    fn test_style_with_inheritance_spacing_not_inherited() {
        let mut parent = Style::default();
        parent.spacing.padding = Spacing::all(10);
        parent.spacing.margin = Spacing::all(5);

        let child = Style::default();
        let result = child.with_inheritance(&parent);
        assert_eq!(result.spacing.padding, Spacing::default());
        assert_eq!(result.spacing.margin, Spacing::default());
    }

    #[test]
    fn test_style_with_inheritance_sizing_not_inherited() {
        let mut parent = Style::default();
        parent.sizing.width = Size::Fixed(100);
        parent.sizing.height = Size::Fixed(50);

        let child = Style::default();
        let result = child.with_inheritance(&parent);
        assert_eq!(result.sizing.width, Size::default());
        assert_eq!(result.sizing.height, Size::default());
    }

    #[test]
    fn test_style_with_inheritance_background_not_inherited() {
        let mut parent = Style::default();
        parent.visual.background = Color::RED;

        let child = Style::default();
        let result = child.with_inheritance(&parent);
        assert_eq!(result.visual.background, Color::default());
    }

    #[test]
    fn test_style_with_inheritance_background_explicit() {
        let mut parent = Style::default();
        parent.visual.background = Color::RED;

        let mut child = Style::default();
        child.visual.background = Color::BLUE;

        let result = child.with_inheritance(&parent);
        assert_eq!(result.visual.background, Color::BLUE);
    }

    #[test]
    fn test_style_default() {
        let style = Style::default();
        assert_eq!(style.layout.display, Display::default());
        assert_eq!(style.spacing.padding, Spacing::default());
        assert_eq!(style.sizing.width, Size::default());
        assert_eq!(style.visual.border_style, BorderStyle::default());
    }

    #[test]
    fn test_style_clone() {
        let mut style = Style::default();
        style.layout.gap = 10;
        style.visual.color = Color::RED;

        let cloned = style.clone();
        assert_eq!(cloned.layout.gap, 10);
        assert_eq!(cloned.visual.color, Color::RED);
    }

    #[test]
    fn test_style_debug() {
        let style = Style::default();
        let debug_str = format!("{:?}", style);
        assert!(debug_str.contains("Style"));
    }
}
