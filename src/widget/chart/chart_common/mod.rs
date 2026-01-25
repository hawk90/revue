//! Common types for chart widgets
//!
//! Shared axis, legend, tooltip, color scheme, and grid components
//! used across all chart widgets for consistent APIs.

mod axis;
mod color_scheme;
mod grid;
mod legend;
mod marker;
mod orientation;
mod tooltip;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;

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

// Re-exports
pub use axis::{Axis, AxisFormat};
pub use color_scheme::ColorScheme;
pub use grid::{ChartGrid, GridStyle};
pub use legend::{Legend, LegendOrientation, LegendPosition};
pub use marker::Marker;
pub use orientation::ChartOrientation;
// Tooltip types are exported for public API but not currently used internally
#[allow(unused_imports)]
pub use tooltip::{ChartTooltip, ChartTooltipFormat, ChartTooltipPosition};
