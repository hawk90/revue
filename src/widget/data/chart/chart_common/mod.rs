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

// KEEP HERE - Private implementation tests (accesses private fields: title, min, max, ticks, grid, color, format, position)

// Re-exports
pub use axis::{Axis, AxisFormat};
pub use color_scheme::ColorScheme;
pub use grid::{ChartGrid, GridStyle};
pub use legend::{Legend, LegendOrientation, LegendPosition};
pub use marker::Marker;
pub use orientation::ChartOrientation;
#[allow(deprecated)]
pub use tooltip::{ChartTooltip, ChartTooltipFormat, ChartTooltipPosition};

#[cfg(test)]
mod tests {
    use super::*;

    // ========== Axis Tests ==========

    #[test]
    fn test_axis_format_value() {
        let axis = Axis::new().format(AxisFormat::Fixed(2));
        assert_eq!(axis.format_value(1.23456), "1.23");

        let axis = Axis::new().format(AxisFormat::Percent);
        assert_eq!(axis.format_value(0.5), "50%");

        let axis = Axis::new().format(AxisFormat::Integer);
        assert_eq!(axis.format_value(42.7), "43");
    }

    // ========== Legend Tests ==========

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
    #[allow(deprecated)]
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
    #[allow(deprecated)]
    fn test_tooltip_default() {
        let tooltip = ChartTooltip::new();
        assert!(!tooltip.enabled);
        assert!(matches!(tooltip.format, ChartTooltipFormat::Auto));
        assert_eq!(tooltip.position, ChartTooltipPosition::Follow);
    }

    #[test]
    #[allow(deprecated)]
    fn test_tooltip_enabled() {
        let tooltip = ChartTooltip::enabled();
        assert!(tooltip.enabled);
    }

    #[test]
    #[allow(deprecated)]
    fn test_tooltip_format() {
        let tooltip = ChartTooltip::new().format(ChartTooltipFormat::Auto);
        assert!(matches!(tooltip.format, ChartTooltipFormat::Auto));
    }

    // ========== ColorScheme Tests ==========

    // ========== Grid Tests ==========

    // ========== Marker Tests ==========

    // ========== Orientation Tests ==========
}
