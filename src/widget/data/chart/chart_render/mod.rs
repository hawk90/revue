//! Common rendering functions for chart widgets
//!
//! Shared rendering utilities for title, legend, grid, and axis labels.

// Allow dead code for utility functions that are not yet used but available for future use
#![allow(dead_code)]

mod axes;
mod fit;
mod frame;
mod labels;
mod legend;

pub(crate) use axes::y_axis_label_columns;
pub use axes::{render_axis_title, render_grid, render_x_axis_labels, render_y_axis_labels};
pub(crate) use fit::fit_items;
pub use frame::{calculate_chart_area, fill_background, render_title};
pub(crate) use labels::{label_ticks, put_label, y_label_column};
pub use legend::{calculate_legend_position, render_horizontal_legend, render_legend, LegendItem};
