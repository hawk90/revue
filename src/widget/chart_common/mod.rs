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
mod tests;

// Re-exports
pub use axis::{Axis, AxisFormat};
pub use color_scheme::ColorScheme;
pub use grid::{ChartGrid, GridStyle};
pub use legend::{Legend, LegendOrientation, LegendPosition};
pub use marker::Marker;
pub use orientation::ChartOrientation;
pub use tooltip::{ChartTooltip, ChartTooltipFormat, ChartTooltipPosition};
