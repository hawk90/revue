//! Common rendering functions for chart widgets
//!
//! Shared rendering utilities for title, legend, grid, and axis labels.

// Allow dead code for utility functions that are not yet used but available for future use
#![allow(dead_code)]

mod axes;
mod frame;
mod legend;

pub use axes::{render_axis_title, render_grid, render_x_axis_labels, render_y_axis_labels};
pub use frame::{calculate_chart_area, fill_background, render_title};
pub use legend::{calculate_legend_position, render_horizontal_legend, render_legend, LegendItem};
