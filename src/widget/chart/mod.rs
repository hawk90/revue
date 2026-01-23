//! Chart widget for data visualization
//!
//! Supports line charts, scatter plots, area charts, and step charts
//! with multiple series, axes, legends, and grid lines.

pub use helper::{chart, line_chart, scatter_plot, Chart};
pub use types::{ChartType, LineStyle, Series};

mod helper;
mod tests;
mod types;
