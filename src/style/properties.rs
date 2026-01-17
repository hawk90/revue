//! CSS property definitions
//!
//! This module defines all CSS-like properties used for styling widgets.
//! Properties are organized into logical groups for maintainability.

// ─────────────────────────────────────────────────────────────────────────────
// Layout Style (display, flex, grid properties)
// ─────────────────────────────────────────────────────────────────────────────

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

// ─────────────────────────────────────────────────────────────────────────────
// Spacing Style (padding, margin, position offsets)
// ─────────────────────────────────────────────────────────────────────────────

/// Spacing-related style properties
///
/// Contains padding, margin, and position offset properties.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SpacingStyle {
    /// Inner padding
    pub padding: Spacing,
    /// Outer margin
    pub margin: Spacing,
    /// Top offset (for absolute/fixed/relative)
    pub top: Option<i16>,
    /// Right offset
    pub right: Option<i16>,
    /// Bottom offset
    pub bottom: Option<i16>,
    /// Left offset
    pub left: Option<i16>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Sizing Style (width, height, min/max constraints)
// ─────────────────────────────────────────────────────────────────────────────

/// Size constraint style properties
///
/// Contains width, height, and min/max constraints.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SizingStyle {
    /// Width constraint
    pub width: Size,
    /// Height constraint
    pub height: Size,
    /// Minimum width
    pub min_width: Size,
    /// Maximum width
    pub max_width: Size,
    /// Minimum height
    pub min_height: Size,
    /// Maximum height
    pub max_height: Size,
}

// ─────────────────────────────────────────────────────────────────────────────
// Visual Style (colors, border, opacity)
// ─────────────────────────────────────────────────────────────────────────────

/// Visual style properties
///
/// Contains colors, border, opacity, and visibility properties.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VisualStyle {
    /// Border style
    pub border_style: BorderStyle,
    /// Border color
    pub border_color: Color,
    /// Text/foreground color (INHERITED)
    pub color: Color,
    /// Background color
    pub background: Color,
    /// Opacity (0.0 to 1.0, INHERITED)
    pub opacity: f32,
    /// Visibility flag (INHERITED)
    pub visible: bool,
    /// Z-index for stacking order
    pub z_index: i16,
}

impl Default for VisualStyle {
    fn default() -> Self {
        Self {
            border_style: BorderStyle::default(),
            border_color: Color::default(),
            color: Color::default(),
            background: Color::default(),
            opacity: 1.0,
            visible: true,
            z_index: 0,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Main Style struct (composed of sub-styles)
// ─────────────────────────────────────────────────────────────────────────────

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

/// CSS property enum for dynamic property handling
#[derive(Debug, Clone)]
pub enum Property {
    /// Display property value
    Display(Display),
    /// Flex direction property value
    FlexDirection(FlexDirection),
    /// Padding property value
    Padding(Spacing),
    /// Margin property value
    Margin(Spacing),
    /// Color property value
    Color(Color),
    /// Background property value
    Background(Color),
    /// Border style property value
    BorderStyle(BorderStyle),
    /// Border color property value
    BorderColor(Color),
}

/// Display mode for layout
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Display {
    /// Flexbox layout (default)
    #[default]
    Flex,
    /// Block layout
    Block,
    /// CSS Grid layout
    Grid,
    /// Hidden - element takes no space
    None,
}

/// Position mode for layout
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Position {
    /// Static positioning (default, normal flow)
    #[default]
    Static,
    /// Relative to normal position
    Relative,
    /// Positioned relative to nearest positioned ancestor
    Absolute,
    /// Positioned relative to viewport
    Fixed,
}

/// Grid track sizing
#[derive(Debug, Clone, Default, PartialEq)]
pub enum GridTrack {
    /// Fixed size in cells
    Fixed(u16),
    /// Fractional unit (fr)
    Fr(f32),
    /// Automatic sizing
    #[default]
    Auto,
    /// Minimum content
    MinContent,
    /// Maximum content
    MaxContent,
}

/// Grid template (columns or rows)
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GridTemplate {
    /// Track definitions
    pub tracks: Vec<GridTrack>,
}

impl GridTemplate {
    /// Create a new grid template
    pub fn new(tracks: Vec<GridTrack>) -> Self {
        Self { tracks }
    }

    /// Create a template with repeated tracks
    pub fn repeat(count: usize, track: GridTrack) -> Self {
        Self {
            tracks: vec![track; count],
        }
    }

    /// Create from fr values
    pub fn fr(values: &[f32]) -> Self {
        Self {
            tracks: values.iter().map(|&v| GridTrack::Fr(v)).collect(),
        }
    }

    /// Create from fixed values
    pub fn fixed(values: &[u16]) -> Self {
        Self {
            tracks: values.iter().map(|&v| GridTrack::Fixed(v)).collect(),
        }
    }
}

/// Grid placement for a single axis
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GridPlacement {
    /// Start line (1-based, 0 = auto)
    pub start: i16,
    /// End line (1-based, 0 = auto)
    pub end: i16,
}

impl GridPlacement {
    /// Create auto placement
    pub fn auto() -> Self {
        Self { start: 0, end: 0 }
    }

    /// Place at a specific line
    pub fn line(line: i16) -> Self {
        Self {
            start: line,
            end: 0,
        }
    }

    /// Span a number of tracks
    pub fn span(count: i16) -> Self {
        Self {
            start: 0,
            end: -count,
        } // Negative end means span
    }

    /// Place from start to end line
    pub fn from_to(start: i16, end: i16) -> Self {
        Self { start, end }
    }
}

/// Flex direction for flexbox layout
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FlexDirection {
    /// Horizontal layout (default)
    #[default]
    Row,
    /// Vertical layout
    Column,
}

/// Main axis alignment for flexbox
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum JustifyContent {
    /// Align to start (default)
    #[default]
    Start,
    /// Center alignment
    Center,
    /// Align to end
    End,
    /// Space between items
    SpaceBetween,
    /// Space around items
    SpaceAround,
}

/// Cross axis alignment for flexbox
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum AlignItems {
    /// Align to start (default)
    #[default]
    Start,
    /// Center alignment
    Center,
    /// Align to end
    End,
    /// Stretch to fill
    Stretch,
}

/// Spacing values for padding and margin
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Spacing {
    /// Top spacing
    pub top: u16,
    /// Right spacing
    pub right: u16,
    /// Bottom spacing
    pub bottom: u16,
    /// Left spacing
    pub left: u16,
}

impl Spacing {
    /// Create spacing with same value on all sides
    pub fn all(value: u16) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }

    /// Create spacing with vertical (top/bottom) values only
    pub fn vertical(value: u16) -> Self {
        Self {
            top: value,
            right: 0,
            bottom: value,
            left: 0,
        }
    }

    /// Create spacing with horizontal (left/right) values only
    pub fn horizontal(value: u16) -> Self {
        Self {
            top: 0,
            right: value,
            bottom: 0,
            left: value,
        }
    }

    /// Create spacing with individual values
    pub fn new(top: u16, right: u16, bottom: u16, left: u16) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }
}

/// Size constraint for width/height
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Size {
    /// Automatic sizing (default)
    #[default]
    Auto,
    /// Fixed size in cells
    Fixed(u16),
    /// Percentage of parent
    Percent(f32),
}

/// Border style options
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum BorderStyle {
    /// No border (default)
    #[default]
    None,
    /// Solid line border
    Solid,
    /// Dashed line border
    Dashed,
    /// Double line border
    Double,
    /// Rounded corner border
    Rounded,
}

/// RGBA color value with alpha channel
///
/// Alpha channel: 0 = fully transparent, 255 = fully opaque.
/// Default color has alpha=0 to indicate "unset" state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    /// Red component (0-255)
    pub r: u8,
    /// Green component (0-255)
    pub g: u8,
    /// Blue component (0-255)
    pub b: u8,
    /// Alpha component (0=transparent, 255=opaque)
    pub a: u8,
}

impl Default for Color {
    /// Default color is transparent black (unset state)
    fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }
}

impl Color {
    /// Check if this is the default (unset) color
    pub fn is_default(&self) -> bool {
        self.a == 0 && self.r == 0 && self.g == 0 && self.b == 0
    }

    /// Check if color is fully transparent
    pub const fn is_transparent(&self) -> bool {
        self.a == 0
    }

    /// Check if color is fully opaque
    pub const fn is_opaque(&self) -> bool {
        self.a == 255
    }

    /// Create color from RGB components (fully opaque)
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Create color from RGBA components
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Create color from hex value (0xRRGGBB, fully opaque)
    pub const fn hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
            a: 255,
        }
    }

    /// Create color from hex value with alpha (0xRRGGBBAA)
    pub const fn hexa(hex: u32) -> Self {
        Self {
            r: ((hex >> 24) & 0xFF) as u8,
            g: ((hex >> 16) & 0xFF) as u8,
            b: ((hex >> 8) & 0xFF) as u8,
            a: (hex & 0xFF) as u8,
        }
    }

    /// Create a new color with modified alpha
    pub const fn with_alpha(self, a: u8) -> Self {
        Self { a, ..self }
    }

    /// Create a semi-transparent version (alpha = 128)
    pub const fn semi_transparent(self) -> Self {
        self.with_alpha(128)
    }

    /// Get alpha as float (0.0 = transparent, 1.0 = opaque)
    pub fn alpha_f32(&self) -> f32 {
        self.a as f32 / 255.0
    }

    /// Create color with alpha from float (0.0 = transparent, 1.0 = opaque)
    pub fn with_alpha_f32(self, alpha: f32) -> Self {
        self.with_alpha((alpha.clamp(0.0, 1.0) * 255.0) as u8)
    }

    /// White color (#FFFFFF)
    pub const WHITE: Color = Color::rgb(255, 255, 255);
    /// Black color (#000000)
    pub const BLACK: Color = Color::rgb(0, 0, 0);
    /// Red color (#FF0000)
    pub const RED: Color = Color::rgb(255, 0, 0);
    /// Green color (#00FF00)
    pub const GREEN: Color = Color::rgb(0, 255, 0);
    /// Blue color (#0000FF)
    pub const BLUE: Color = Color::rgb(0, 0, 255);
    /// Cyan color (#00FFFF)
    pub const CYAN: Color = Color::rgb(0, 255, 255);
    /// Yellow color (#FFFF00)
    pub const YELLOW: Color = Color::rgb(255, 255, 0);
    /// Magenta color (#FF00FF)
    pub const MAGENTA: Color = Color::rgb(255, 0, 255);
    /// Transparent (fully transparent black)
    pub const TRANSPARENT: Color = Color::rgba(0, 0, 0, 0);

    // ─────────────────────────────────────────────────────────────────────────
    // Interaction Color Helpers
    // ─────────────────────────────────────────────────────────────────────────

    /// Darken the color by subtracting from RGB components
    ///
    /// # Arguments
    /// * `amount` - Value to subtract from each RGB component (0-255)
    ///
    /// # Example
    /// ```rust,ignore
    /// let base = Color::rgb(100, 150, 200);
    /// let darker = base.darken(30);  // rgb(70, 120, 170)
    /// ```
    #[must_use]
    pub const fn darken(self, amount: u8) -> Self {
        Self {
            r: self.r.saturating_sub(amount),
            g: self.g.saturating_sub(amount),
            b: self.b.saturating_sub(amount),
            a: self.a,
        }
    }

    /// Lighten the color by adding to RGB components
    ///
    /// # Arguments
    /// * `amount` - Value to add to each RGB component (0-255)
    ///
    /// # Example
    /// ```rust,ignore
    /// let base = Color::rgb(100, 150, 200);
    /// let lighter = base.lighten(30);  // rgb(130, 180, 230)
    /// ```
    #[must_use]
    pub const fn lighten(self, amount: u8) -> Self {
        Self {
            r: self.r.saturating_add(amount),
            g: self.g.saturating_add(amount),
            b: self.b.saturating_add(amount),
            a: self.a,
        }
    }

    /// Darken by percentage (0.0 to 1.0)
    ///
    /// # Example
    /// ```rust,ignore
    /// let base = Color::rgb(100, 100, 100);
    /// let darker = base.darken_pct(0.2);  // 20% darker
    /// ```
    #[must_use]
    pub fn darken_pct(self, pct: f32) -> Self {
        let factor = 1.0 - pct.clamp(0.0, 1.0);
        Self {
            r: (self.r as f32 * factor) as u8,
            g: (self.g as f32 * factor) as u8,
            b: (self.b as f32 * factor) as u8,
            a: self.a,
        }
    }

    /// Lighten by percentage (0.0 to 1.0)
    ///
    /// # Example
    /// ```rust,ignore
    /// let base = Color::rgb(100, 100, 100);
    /// let lighter = base.lighten_pct(0.2);  // 20% lighter
    /// ```
    #[must_use]
    pub fn lighten_pct(self, pct: f32) -> Self {
        let factor = pct.clamp(0.0, 1.0);
        Self {
            r: (self.r as f32 + (255.0 - self.r as f32) * factor) as u8,
            g: (self.g as f32 + (255.0 - self.g as f32) * factor) as u8,
            b: (self.b as f32 + (255.0 - self.b as f32) * factor) as u8,
            a: self.a,
        }
    }

    /// Get pressed state color (standard darkening)
    ///
    /// # Example
    /// ```rust,ignore
    /// let button_bg = Color::rgb(37, 99, 235);
    /// let pressed_bg = button_bg.pressed();  // Darker by 30
    /// ```
    #[must_use]
    pub const fn pressed(self) -> Self {
        self.darken(30)
    }

    /// Get hover state color (standard lightening)
    ///
    /// # Example
    /// ```rust,ignore
    /// let button_bg = Color::rgb(37, 99, 235);
    /// let hover_bg = button_bg.hover();  // Lighter by 40
    /// ```
    #[must_use]
    pub const fn hover(self) -> Self {
        self.lighten(40)
    }

    /// Get focus state color (same as hover)
    ///
    /// # Example
    /// ```rust,ignore
    /// let button_bg = Color::rgb(37, 99, 235);
    /// let focus_bg = button_bg.focus();  // Lighter by 40
    /// ```
    #[must_use]
    pub const fn focus(self) -> Self {
        self.lighten(40)
    }

    /// Blend this color with another color
    ///
    /// # Arguments
    /// * `other` - The other color to blend with
    /// * `ratio` - Blend ratio (0.0 = self, 1.0 = other)
    ///
    /// # Example
    /// ```rust,ignore
    /// let red = Color::RED;
    /// let blue = Color::BLUE;
    /// let purple = red.blend(blue, 0.5);  // 50% mix
    /// ```
    #[must_use]
    pub fn blend(self, other: Color, ratio: f32) -> Self {
        let ratio = ratio.clamp(0.0, 1.0);
        let inv = 1.0 - ratio;
        Self {
            r: (self.r as f32 * inv + other.r as f32 * ratio) as u8,
            g: (self.g as f32 * inv + other.g as f32 * ratio) as u8,
            b: (self.b as f32 * inv + other.b as f32 * ratio) as u8,
            a: (self.a as f32 * inv + other.a as f32 * ratio) as u8,
        }
    }

    /// Apply interaction state to color
    ///
    /// Convenience method that applies the appropriate color effect
    /// based on pressed/hovered/focused state.
    ///
    /// # Example
    /// ```rust,ignore
    /// let base = Color::rgb(37, 99, 235);
    /// let final_color = base.with_interaction(pressed, hovered, focused);
    /// ```
    #[must_use]
    pub const fn with_interaction(self, pressed: bool, hovered: bool, focused: bool) -> Self {
        if pressed {
            self.pressed()
        } else if hovered || focused {
            self.hover()
        } else {
            self
        }
    }
}

// Tests moved to tests/style_tests.rs
