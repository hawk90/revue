//! Heat map widget
//!
//! Displays 2D data as a color-coded grid, useful for visualizing matrices,
//! activity patterns, correlation data, and more.

mod types;
mod view;

pub use types::{CellDisplay, ColorScale};

use crate::style::Color;
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// Heat map widget
#[derive(Clone, Debug)]
pub struct HeatMap {
    /// Data grid (row-major)
    pub(crate) data: Vec<Vec<f64>>,
    /// Number of rows (cached for future use)
    pub(crate) _rows: usize,
    /// Number of columns
    pub(crate) cols: usize,
    /// Min value (for normalization)
    pub(crate) min_val: f64,
    /// Max value (for normalization)
    pub(crate) max_val: f64,
    /// Color scale
    color_scale: ColorScale,
    /// Custom colors (for ColorScale::Custom)
    custom_colors: Option<(Color, Color)>,
    /// Cell display mode
    cell_display: CellDisplay,
    /// Cell width
    cell_width: usize,
    /// Cell height
    cell_height: usize,
    /// Show values
    show_values: bool,
    /// Value format (decimal places)
    value_decimals: usize,
    /// Row labels
    row_labels: Option<Vec<String>>,
    /// Column labels
    col_labels: Option<Vec<String>>,
    /// Title
    title: Option<String>,
    /// Show legend
    show_legend: bool,
    /// Highlighted cell (row, col)
    highlighted: Option<(usize, usize)>,
    /// Widget properties
    props: WidgetProps,
}

impl HeatMap {
    /// Create new heatmap from 2D data
    pub fn new(data: Vec<Vec<f64>>) -> Self {
        let rows = data.len();
        let cols = data.first().map(|r| r.len()).unwrap_or(0);

        // Find min/max
        let mut min_val = f64::INFINITY;
        let mut max_val = f64::NEG_INFINITY;
        for row in &data {
            for &val in row {
                if val < min_val {
                    min_val = val;
                }
                if val > max_val {
                    max_val = val;
                }
            }
        }

        if min_val == f64::INFINITY {
            min_val = 0.0;
        }
        if max_val == f64::NEG_INFINITY {
            max_val = 1.0;
        }

        Self {
            data,
            _rows: rows,
            cols,
            min_val,
            max_val,
            color_scale: ColorScale::default(),
            custom_colors: None,
            cell_display: CellDisplay::default(),
            cell_width: 2,
            cell_height: 1,
            show_values: false,
            value_decimals: 1,
            row_labels: None,
            col_labels: None,
            title: None,
            show_legend: false,
            highlighted: None,
            props: WidgetProps::new(),
        }
    }

    /// Create from flat data with dimensions
    pub fn from_flat(data: &[f64], rows: usize, cols: usize) -> Self {
        let mut grid = Vec::with_capacity(rows);
        for r in 0..rows {
            let start = r * cols;
            let end = (start + cols).min(data.len());
            grid.push(data[start..end].to_vec());
        }
        Self::new(grid)
    }

    /// Create GitHub-style contribution map
    pub fn contribution_map(contributions: &[u32]) -> Self {
        // Assume 52 weeks x 7 days
        let weeks = 52;
        let days = 7;
        let mut data = vec![vec![0.0; weeks]; days];

        let max_contrib = *contributions.iter().max().unwrap_or(&1) as f64;

        for (i, &count) in contributions.iter().take(weeks * days).enumerate() {
            let week = i / days;
            let day = i % days;
            data[day][week] = count as f64 / max_contrib;
        }

        Self::new(data)
            .color_scale(ColorScale::Green)
            .cell_width(2)
            .cell_height(1)
    }

    /// Create correlation matrix
    pub fn correlation_matrix(correlations: &[Vec<f64>], labels: Vec<String>) -> Self {
        Self::new(correlations.to_vec())
            .color_scale(ColorScale::BlueRed)
            .bounds(-1.0, 1.0)
            .row_labels(labels.clone())
            .col_labels(labels)
            .show_values(true)
    }

    /// Set color scale
    pub fn color_scale(mut self, scale: ColorScale) -> Self {
        self.color_scale = scale;
        self
    }

    /// Set custom colors (for gradient)
    pub fn custom_colors(mut self, low: Color, high: Color) -> Self {
        self.color_scale = ColorScale::Custom;
        self.custom_colors = Some((low, high));
        self
    }

    /// Set value bounds
    pub fn bounds(mut self, min: f64, max: f64) -> Self {
        self.min_val = min;
        self.max_val = max;
        self
    }

    /// Set cell display mode
    pub fn cell_display(mut self, display: CellDisplay) -> Self {
        self.cell_display = display;
        self
    }

    /// Set cell dimensions
    pub fn cell_size(mut self, width: usize, height: usize) -> Self {
        self.cell_width = width;
        self.cell_height = height;
        self
    }

    /// Shorthand for cell width
    pub fn cell_width(mut self, width: usize) -> Self {
        self.cell_width = width;
        self
    }

    /// Shorthand for cell height
    pub fn cell_height(mut self, height: usize) -> Self {
        self.cell_height = height;
        self
    }

    /// Show cell values
    pub fn show_values(mut self, show: bool) -> Self {
        self.show_values = show;
        if show && self.cell_width < 4 {
            self.cell_width = 4;
        }
        self
    }

    /// Set value decimal places
    pub fn value_decimals(mut self, decimals: usize) -> Self {
        self.value_decimals = decimals;
        self
    }

    /// Set row labels
    pub fn row_labels(mut self, labels: Vec<String>) -> Self {
        self.row_labels = Some(labels);
        self
    }

    /// Set column labels
    pub fn col_labels(mut self, labels: Vec<String>) -> Self {
        self.col_labels = Some(labels);
        self
    }

    /// Set title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Show legend
    pub fn show_legend(mut self, show: bool) -> Self {
        self.show_legend = show;
        self
    }

    /// Highlight cell
    pub fn highlight(mut self, row: usize, col: usize) -> Self {
        self.highlighted = Some((row, col));
        self
    }

    /// Normalize value to 0.0-1.0
    pub fn normalize(&self, value: f64) -> f64 {
        if self.max_val == self.min_val {
            0.5
        } else {
            (value - self.min_val) / (self.max_val - self.min_val)
        }
    }

    /// Get color for value
    pub fn color_for(&self, value: f64) -> Color {
        let normalized = self.normalize(value);

        if let (ColorScale::Custom, Some((low, high))) = (self.color_scale, &self.custom_colors) {
            // Linear interpolation
            Color::rgb(
                (low.r as f64 + normalized * (high.r as f64 - low.r as f64)) as u8,
                (low.g as f64 + normalized * (high.g as f64 - low.g as f64)) as u8,
                (low.b as f64 + normalized * (high.b as f64 - low.b as f64)) as u8,
            )
        } else {
            self.color_scale.color_at(normalized)
        }
    }

    /// Render cell content
    pub fn render_cell(&self, value: f64) -> String {
        if self.show_values {
            format!(
                "{:>width$.prec$}",
                value,
                width = self.cell_width,
                prec = self.value_decimals
            )
        } else {
            match self.cell_display {
                CellDisplay::Block => "█".repeat(self.cell_width),
                CellDisplay::HalfBlock => "▀".repeat(self.cell_width),
                CellDisplay::Value => {
                    format!(
                        "{:>width$.prec$}",
                        value,
                        width = self.cell_width,
                        prec = self.value_decimals
                    )
                }
                CellDisplay::Custom => "■".repeat(self.cell_width),
            }
        }
    }
}

impl_styled_view!(HeatMap);
impl_props_builders!(HeatMap);

/// Create a heatmap
pub fn heatmap(data: Vec<Vec<f64>>) -> HeatMap {
    HeatMap::new(data)
}

/// Create a contribution map
pub fn contribution_map(contributions: &[u32]) -> HeatMap {
    HeatMap::contribution_map(contributions)
}
