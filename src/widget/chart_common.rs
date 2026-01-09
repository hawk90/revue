//! Common types for chart widgets
//!
//! Shared axis, legend, tooltip, color scheme, and grid components
//! used across all chart widgets for consistent APIs.

use crate::style::Color;

// ============================================================================
// Axis
// ============================================================================

/// Axis configuration for charts
#[derive(Clone, Debug)]
pub struct Axis {
    /// Axis title
    pub title: Option<String>,
    /// Minimum value (auto if None)
    pub min: Option<f64>,
    /// Maximum value (auto if None)
    pub max: Option<f64>,
    /// Number of tick marks
    pub ticks: usize,
    /// Show grid lines
    pub grid: bool,
    /// Axis color
    pub color: Color,
    /// Label formatter
    pub format: AxisFormat,
}

/// Axis label format
#[derive(Clone, Debug, Default)]
pub enum AxisFormat {
    /// Auto format based on value range
    #[default]
    Auto,
    /// Integer format (no decimals)
    Integer,
    /// Fixed decimal places
    Fixed(usize),
    /// Percentage format (value * 100%)
    Percent,
    /// Custom format string
    Custom(String),
}

impl Default for Axis {
    fn default() -> Self {
        Self {
            title: None,
            min: None,
            max: None,
            ticks: 5,
            grid: true,
            color: Color::rgb(100, 100, 100),
            format: AxisFormat::Auto,
        }
    }
}

impl Axis {
    /// Create a new axis with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Set axis title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set min and max bounds
    pub fn bounds(mut self, min: f64, max: f64) -> Self {
        self.min = Some(min);
        self.max = Some(max);
        self
    }

    /// Set minimum bound
    pub fn min(mut self, min: f64) -> Self {
        self.min = Some(min);
        self
    }

    /// Set maximum bound
    pub fn max(mut self, max: f64) -> Self {
        self.max = Some(max);
        self
    }

    /// Set number of tick marks
    pub fn ticks(mut self, ticks: usize) -> Self {
        self.ticks = ticks;
        self
    }

    /// Enable/disable grid lines
    pub fn grid(mut self, show: bool) -> Self {
        self.grid = show;
        self
    }

    /// Set axis color
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set label format
    pub fn format(mut self, format: AxisFormat) -> Self {
        self.format = format;
        self
    }

    /// Format a value using the axis format
    pub fn format_value(&self, value: f64) -> String {
        match &self.format {
            AxisFormat::Auto => {
                if value.abs() >= 1000.0 {
                    format!("{:.0}", value)
                } else if value.abs() >= 1.0 {
                    format!("{:.1}", value)
                } else {
                    format!("{:.2}", value)
                }
            }
            AxisFormat::Integer => format!("{:.0}", value),
            AxisFormat::Fixed(n) => format!("{:.1$}", value, *n),
            AxisFormat::Percent => format!("{:.0}%", value * 100.0),
            AxisFormat::Custom(fmt) => fmt.replace("{}", &value.to_string()),
        }
    }
}

// ============================================================================
// Legend
// ============================================================================

/// Legend configuration
#[derive(Clone, Debug, Default)]
pub struct Legend {
    /// Position of the legend
    pub position: LegendPosition,
    /// Orientation of legend items
    pub orientation: LegendOrientation,
    /// Whether legend items are interactive (click to toggle)
    pub interactive: bool,
}

impl Legend {
    /// Create a new legend with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Set legend position
    pub fn position(mut self, position: LegendPosition) -> Self {
        self.position = position;
        self
    }

    /// Set legend orientation
    pub fn orientation(mut self, orientation: LegendOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Enable interactive mode
    pub fn interactive(mut self, interactive: bool) -> Self {
        self.interactive = interactive;
        self
    }

    /// Create legend at top left
    pub fn top_left() -> Self {
        Self::new().position(LegendPosition::TopLeft)
    }

    /// Create legend at top center
    pub fn top_center() -> Self {
        Self::new().position(LegendPosition::TopCenter)
    }

    /// Create legend at top right
    pub fn top_right() -> Self {
        Self::new().position(LegendPosition::TopRight)
    }

    /// Create legend at bottom left
    pub fn bottom_left() -> Self {
        Self::new().position(LegendPosition::BottomLeft)
    }

    /// Create legend at bottom center
    pub fn bottom_center() -> Self {
        Self::new().position(LegendPosition::BottomCenter)
    }

    /// Create legend at bottom right
    pub fn bottom_right() -> Self {
        Self::new().position(LegendPosition::BottomRight)
    }

    /// Create legend on the left side
    pub fn left() -> Self {
        Self::new()
            .position(LegendPosition::Left)
            .orientation(LegendOrientation::Vertical)
    }

    /// Create legend on the right side
    pub fn right() -> Self {
        Self::new()
            .position(LegendPosition::Right)
            .orientation(LegendOrientation::Vertical)
    }

    /// Create a hidden legend
    pub fn none() -> Self {
        Self::new().position(LegendPosition::None)
    }

    /// Create a hidden legend (alias)
    pub fn hidden() -> Self {
        Self::none()
    }

    /// Check if legend is visible
    pub fn is_visible(&self) -> bool {
        self.position != LegendPosition::None
    }
}

/// Legend position
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LegendPosition {
    /// Top left corner
    TopLeft,
    /// Top center
    TopCenter,
    /// Top right corner
    #[default]
    TopRight,
    /// Bottom left corner
    BottomLeft,
    /// Bottom center
    BottomCenter,
    /// Bottom right corner
    BottomRight,
    /// Left side (vertical)
    Left,
    /// Right side (vertical)
    Right,
    /// Hidden
    None,
}

/// Legend item orientation
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LegendOrientation {
    /// Items arranged horizontally
    #[default]
    Horizontal,
    /// Items arranged vertically
    Vertical,
}

// ============================================================================
// Tooltip
// ============================================================================

/// Tooltip configuration
#[derive(Clone, Debug, Default)]
pub struct ChartTooltip {
    /// Whether tooltip is enabled
    pub enabled: bool,
    /// Tooltip format
    pub format: ChartTooltipFormat,
    /// Tooltip position mode
    pub position: ChartTooltipPosition,
}

impl ChartTooltip {
    /// Create a new tooltip (disabled by default)
    pub fn new() -> Self {
        Self::default()
    }

    /// Create an enabled tooltip
    pub fn enabled() -> Self {
        Self {
            enabled: true,
            ..Default::default()
        }
    }

    /// Enable/disable tooltip
    pub fn enable(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set tooltip format
    pub fn format(mut self, format: ChartTooltipFormat) -> Self {
        self.format = format;
        self
    }

    /// Set custom format string
    pub fn custom_format(mut self, format: impl Into<String>) -> Self {
        self.format = ChartTooltipFormat::Custom(format.into());
        self
    }

    /// Set tooltip position mode
    pub fn position(mut self, position: ChartTooltipPosition) -> Self {
        self.position = position;
        self
    }
}

/// Tooltip format
#[derive(Clone, Debug, Default)]
pub enum ChartTooltipFormat {
    /// Automatic format based on data
    #[default]
    Auto,
    /// Custom format string (e.g., "{label}: {value}")
    Custom(String),
}

/// Tooltip position mode
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChartTooltipPosition {
    /// Follow cursor position
    #[default]
    Follow,
    /// Fixed position near data point
    Fixed,
}

// ============================================================================
// ColorScheme
// ============================================================================

/// Color scheme for chart series
#[derive(Clone, Debug)]
pub struct ColorScheme {
    /// Color palette
    pub palette: Vec<Color>,
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self::default_palette()
    }
}

impl ColorScheme {
    /// Create a color scheme with custom colors
    pub fn new(colors: Vec<Color>) -> Self {
        Self { palette: colors }
    }

    /// Default color palette (10 distinct colors)
    pub fn default_palette() -> Self {
        Self {
            palette: vec![
                Color::rgb(97, 175, 239),  // Blue
                Color::rgb(152, 195, 121), // Green
                Color::rgb(224, 108, 117), // Red
                Color::rgb(229, 192, 123), // Yellow
                Color::rgb(198, 120, 221), // Purple
                Color::rgb(86, 182, 194),  // Cyan
                Color::rgb(209, 154, 102), // Orange
                Color::rgb(190, 80, 70),   // Dark Red
                Color::rgb(152, 104, 1),   // Brown
                Color::rgb(171, 178, 191), // Gray
            ],
        }
    }

    /// Monochrome palette with shades of a base color
    pub fn monochrome(base: Color) -> Self {
        let (r, g, b) = (base.r, base.g, base.b);
        Self {
            palette: (1..=5)
                .map(|i| {
                    let factor = 0.5 + (i as f32 * 0.1);
                    Color::rgb(
                        (r as f32 * factor).min(255.0) as u8,
                        (g as f32 * factor).min(255.0) as u8,
                        (b as f32 * factor).min(255.0) as u8,
                    )
                })
                .collect(),
        }
    }

    /// Categorical palette (high contrast)
    pub fn categorical() -> Self {
        Self {
            palette: vec![
                Color::rgb(31, 119, 180),  // Blue
                Color::rgb(255, 127, 14),  // Orange
                Color::rgb(44, 160, 44),   // Green
                Color::rgb(214, 39, 40),   // Red
                Color::rgb(148, 103, 189), // Purple
                Color::rgb(140, 86, 75),   // Brown
                Color::rgb(227, 119, 194), // Pink
                Color::rgb(127, 127, 127), // Gray
                Color::rgb(188, 189, 34),  // Olive
                Color::rgb(23, 190, 207),  // Cyan
            ],
        }
    }

    /// Get color at index (cycles through palette)
    pub fn get(&self, index: usize) -> Color {
        if self.palette.is_empty() {
            Color::WHITE
        } else {
            self.palette[index % self.palette.len()]
        }
    }

    /// Number of colors in palette
    pub fn len(&self) -> usize {
        self.palette.len()
    }

    /// Check if palette is empty
    pub fn is_empty(&self) -> bool {
        self.palette.is_empty()
    }
}

// ============================================================================
// Grid
// ============================================================================

/// Grid configuration
#[derive(Clone, Debug, Default)]
pub struct ChartGrid {
    /// Show vertical grid lines (X axis)
    pub x: bool,
    /// Show horizontal grid lines (Y axis)
    pub y: bool,
    /// Grid line color
    pub color: Option<Color>,
    /// Grid line style
    pub style: GridStyle,
}

impl ChartGrid {
    /// Create a new grid (hidden by default)
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a grid with both X and Y lines
    pub fn both() -> Self {
        Self {
            x: true,
            y: true,
            ..Default::default()
        }
    }

    /// Create a grid with only X lines
    pub fn x_only() -> Self {
        Self {
            x: true,
            y: false,
            ..Default::default()
        }
    }

    /// Create a grid with only Y lines
    pub fn y_only() -> Self {
        Self {
            x: false,
            y: true,
            ..Default::default()
        }
    }

    /// Enable/disable X grid lines
    pub fn x(mut self, show: bool) -> Self {
        self.x = show;
        self
    }

    /// Enable/disable Y grid lines
    pub fn y(mut self, show: bool) -> Self {
        self.y = show;
        self
    }

    /// Set grid line color
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Set grid line style
    pub fn style(mut self, style: GridStyle) -> Self {
        self.style = style;
        self
    }

    /// Get the grid character for rendering
    pub fn char(&self) -> char {
        match self.style {
            GridStyle::Solid => '─',
            GridStyle::Dashed => '╌',
            GridStyle::Dotted => '·',
        }
    }

    /// Get the effective color (default if not set)
    pub fn effective_color(&self) -> Color {
        self.color.unwrap_or(Color::rgb(60, 60, 60))
    }
}

/// Grid line style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GridStyle {
    /// Solid lines
    #[default]
    Solid,
    /// Dashed lines
    Dashed,
    /// Dotted lines
    Dotted,
}

// ============================================================================
// Orientation
// ============================================================================

/// Orientation for charts (bar, histogram, box plot)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChartOrientation {
    /// Vertical orientation (default for most charts)
    #[default]
    Vertical,
    /// Horizontal orientation
    Horizontal,
}

// ============================================================================
// Marker (shared across chart types)
// ============================================================================

/// Marker style for data points
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Marker {
    /// No marker
    #[default]
    None,
    /// Dot marker (•)
    Dot,
    /// Circle marker (○)
    Circle,
    /// Filled circle (●)
    FilledCircle,
    /// Square marker (□)
    Square,
    /// Filled square (■)
    FilledSquare,
    /// Diamond marker (◇)
    Diamond,
    /// Filled diamond (◆)
    FilledDiamond,
    /// Triangle marker (△)
    Triangle,
    /// Filled triangle (▲)
    FilledTriangle,
    /// Cross marker (+)
    Cross,
    /// X marker (×)
    X,
    /// Star marker (★) - filled for backward compatibility
    Star,
    /// Outline star (☆)
    StarOutline,
    /// Braille dots for high resolution
    Braille,
}

impl Marker {
    /// Get the character for this marker
    pub fn char(&self) -> char {
        match self {
            Marker::None => ' ',
            Marker::Dot => '•',
            Marker::Circle => '○',
            Marker::FilledCircle => '●',
            Marker::Square => '□',
            Marker::FilledSquare => '■',
            Marker::Diamond => '◇',
            Marker::FilledDiamond => '◆',
            Marker::Triangle => '△',
            Marker::FilledTriangle => '▲',
            Marker::Cross => '+',
            Marker::X => '×',
            Marker::Star => '★',
            Marker::StarOutline => '☆',
            Marker::Braille => '⣿',
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ========== Axis Tests ==========

    #[test]
    fn test_axis_default() {
        let axis = Axis::new();
        assert!(axis.title.is_none());
        assert!(axis.min.is_none());
        assert!(axis.max.is_none());
        assert_eq!(axis.ticks, 5);
        assert!(axis.grid);
    }

    #[test]
    fn test_axis_builder() {
        let axis = Axis::new()
            .title("X Axis")
            .bounds(0.0, 100.0)
            .ticks(10)
            .grid(false);

        assert_eq!(axis.title, Some("X Axis".to_string()));
        assert_eq!(axis.min, Some(0.0));
        assert_eq!(axis.max, Some(100.0));
        assert_eq!(axis.ticks, 10);
        assert!(!axis.grid);
    }

    #[test]
    fn test_axis_min_max_separate() {
        let axis = Axis::new().min(10.0).max(200.0).color(Color::RED);
        assert_eq!(axis.min, Some(10.0));
        assert_eq!(axis.max, Some(200.0));
        assert_eq!(axis.color.r, 255);
    }

    #[test]
    fn test_axis_format_value() {
        let axis = Axis::new().format(AxisFormat::Fixed(2));
        assert_eq!(axis.format_value(3.14159), "3.14");

        let axis = Axis::new().format(AxisFormat::Percent);
        assert_eq!(axis.format_value(0.5), "50%");

        let axis = Axis::new().format(AxisFormat::Integer);
        assert_eq!(axis.format_value(42.7), "43");
    }

    #[test]
    fn test_axis_format_auto() {
        let axis = Axis::new().format(AxisFormat::Auto);

        // Large values - no decimals
        assert_eq!(axis.format_value(1500.0), "1500");

        // Medium values - 1 decimal
        assert_eq!(axis.format_value(42.5), "42.5");

        // Small values - 2 decimals
        assert_eq!(axis.format_value(0.123), "0.12");
    }

    #[test]
    fn test_axis_format_custom() {
        let axis = Axis::new().format(AxisFormat::Custom("Value: {}".to_string()));
        assert_eq!(axis.format_value(42.0), "Value: 42");
    }

    // ========== Legend Tests ==========

    #[test]
    fn test_legend_positions() {
        assert_eq!(Legend::top_left().position, LegendPosition::TopLeft);
        assert_eq!(Legend::top_center().position, LegendPosition::TopCenter);
        assert_eq!(Legend::top_right().position, LegendPosition::TopRight);
        assert_eq!(Legend::bottom_left().position, LegendPosition::BottomLeft);
        assert_eq!(
            Legend::bottom_center().position,
            LegendPosition::BottomCenter
        );
        assert_eq!(Legend::bottom_right().position, LegendPosition::BottomRight);
        assert_eq!(Legend::left().position, LegendPosition::Left);
        assert_eq!(Legend::right().position, LegendPosition::Right);
        assert_eq!(Legend::none().position, LegendPosition::None);
        assert_eq!(Legend::hidden().position, LegendPosition::None);
    }

    #[test]
    fn test_legend_visibility() {
        assert!(!Legend::none().is_visible());
        assert!(!Legend::hidden().is_visible());
        assert!(Legend::top_left().is_visible());
        assert!(Legend::bottom_right().is_visible());
    }

    #[test]
    fn test_legend_orientation() {
        let legend = Legend::left();
        assert_eq!(legend.position, LegendPosition::Left);
        assert_eq!(legend.orientation, LegendOrientation::Vertical);

        let legend = Legend::right();
        assert_eq!(legend.orientation, LegendOrientation::Vertical);

        let legend = Legend::top_left();
        assert_eq!(legend.orientation, LegendOrientation::Horizontal);
    }

    #[test]
    fn test_legend_builder() {
        let legend = Legend::new()
            .position(LegendPosition::BottomCenter)
            .orientation(LegendOrientation::Vertical)
            .interactive(true);

        assert_eq!(legend.position, LegendPosition::BottomCenter);
        assert_eq!(legend.orientation, LegendOrientation::Vertical);
        assert!(legend.interactive);
    }

    // ========== Tooltip Tests ==========

    #[test]
    fn test_tooltip_default() {
        let tooltip = ChartTooltip::new();
        assert!(!tooltip.enabled);
        assert!(matches!(tooltip.format, ChartTooltipFormat::Auto));
        assert_eq!(tooltip.position, ChartTooltipPosition::Follow);
    }

    #[test]
    fn test_tooltip_enabled() {
        let tooltip = ChartTooltip::enabled();
        assert!(tooltip.enabled);
    }

    #[test]
    fn test_tooltip_builder() {
        let tooltip = ChartTooltip::new()
            .enable(true)
            .position(ChartTooltipPosition::Fixed)
            .custom_format("{label}: {value}");

        assert!(tooltip.enabled);
        assert_eq!(tooltip.position, ChartTooltipPosition::Fixed);
        assert!(matches!(tooltip.format, ChartTooltipFormat::Custom(_)));
    }

    #[test]
    fn test_tooltip_format() {
        let tooltip = ChartTooltip::new().format(ChartTooltipFormat::Auto);
        assert!(matches!(tooltip.format, ChartTooltipFormat::Auto));
    }

    // ========== ColorScheme Tests ==========

    #[test]
    fn test_color_scheme_default() {
        let scheme = ColorScheme::default_palette();
        assert_eq!(scheme.len(), 10);
        assert!(!scheme.is_empty());
    }

    #[test]
    fn test_color_scheme_cycling() {
        let scheme = ColorScheme::default_palette();
        let color0 = scheme.get(0);
        let color10 = scheme.get(10);
        assert_eq!(color0.r, color10.r);
        assert_eq!(color0.g, color10.g);
        assert_eq!(color0.b, color10.b);
    }

    #[test]
    fn test_color_scheme_custom() {
        let scheme = ColorScheme::new(vec![Color::RED, Color::GREEN, Color::BLUE]);
        assert_eq!(scheme.len(), 3);
        assert_eq!(scheme.get(0).r, 255); // RED
        assert_eq!(scheme.get(1).g, 255); // GREEN
        assert_eq!(scheme.get(2).b, 255); // BLUE
    }

    #[test]
    fn test_color_scheme_monochrome() {
        let scheme = ColorScheme::monochrome(Color::rgb(100, 100, 100));
        assert_eq!(scheme.len(), 5);
        // Verify shades are different
        let color1 = scheme.get(0);
        let color2 = scheme.get(4);
        assert_ne!(color1.r, color2.r);
    }

    #[test]
    fn test_color_scheme_categorical() {
        let scheme = ColorScheme::categorical();
        assert_eq!(scheme.len(), 10);
    }

    #[test]
    fn test_color_scheme_empty() {
        let scheme = ColorScheme::new(vec![]);
        assert!(scheme.is_empty());
        assert_eq!(scheme.len(), 0);
        // Empty scheme returns white
        let color = scheme.get(0);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 255);
        assert_eq!(color.b, 255);
    }

    // ========== Grid Tests ==========

    #[test]
    fn test_grid_default() {
        let grid = ChartGrid::new();
        assert!(!grid.x);
        assert!(!grid.y);
        assert!(grid.color.is_none());
    }

    #[test]
    fn test_grid_both() {
        let grid = ChartGrid::both();
        assert!(grid.x);
        assert!(grid.y);
    }

    #[test]
    fn test_grid_x_only() {
        let grid = ChartGrid::x_only();
        assert!(grid.x);
        assert!(!grid.y);
    }

    #[test]
    fn test_grid_y_only() {
        let grid = ChartGrid::y_only();
        assert!(!grid.x);
        assert!(grid.y);
    }

    #[test]
    fn test_grid_builder() {
        let grid = ChartGrid::new()
            .x(true)
            .y(true)
            .color(Color::CYAN)
            .style(GridStyle::Dashed);

        assert!(grid.x);
        assert!(grid.y);
        assert!(grid.color.is_some());
        assert_eq!(grid.style, GridStyle::Dashed);
    }

    #[test]
    fn test_grid_char() {
        assert_eq!(ChartGrid::new().style(GridStyle::Solid).char(), '─');
        assert_eq!(ChartGrid::new().style(GridStyle::Dashed).char(), '╌');
        assert_eq!(ChartGrid::new().style(GridStyle::Dotted).char(), '·');
    }

    #[test]
    fn test_grid_effective_color() {
        let grid = ChartGrid::new();
        let default_color = grid.effective_color();
        assert_eq!(default_color.r, 60);

        let grid = ChartGrid::new().color(Color::RED);
        let custom_color = grid.effective_color();
        assert_eq!(custom_color.r, 255);
    }

    // ========== Marker Tests ==========

    #[test]
    fn test_marker_char() {
        assert_eq!(Marker::None.char(), ' ');
        assert_eq!(Marker::Dot.char(), '•');
        assert_eq!(Marker::Circle.char(), '○');
        assert_eq!(Marker::FilledCircle.char(), '●');
        assert_eq!(Marker::Square.char(), '□');
        assert_eq!(Marker::FilledSquare.char(), '■');
        assert_eq!(Marker::Diamond.char(), '◇');
        assert_eq!(Marker::FilledDiamond.char(), '◆');
        assert_eq!(Marker::Triangle.char(), '△');
        assert_eq!(Marker::FilledTriangle.char(), '▲');
        assert_eq!(Marker::Cross.char(), '+');
        assert_eq!(Marker::X.char(), '×');
        assert_eq!(Marker::Star.char(), '★');
        assert_eq!(Marker::StarOutline.char(), '☆');
        assert_eq!(Marker::Braille.char(), '⣿');
    }

    // ========== Orientation Tests ==========

    #[test]
    fn test_orientation_default() {
        let orientation = ChartOrientation::default();
        assert_eq!(orientation, ChartOrientation::Vertical);
    }
}
