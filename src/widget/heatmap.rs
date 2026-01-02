//! Heat map widget
//!
//! Displays 2D data as a color-coded grid, useful for visualizing matrices,
//! activity patterns, correlation data, and more.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{HeatMap, ColorScale, heatmap};
//!
//! // Simple heatmap
//! let data = vec![
//!     vec![0.1, 0.5, 0.9],
//!     vec![0.3, 0.7, 0.4],
//!     vec![0.8, 0.2, 0.6],
//! ];
//! let map = HeatMap::new(data);
//!
//! // GitHub-style contribution map
//! let contrib = HeatMap::contribution_map(&activity_data);
//!
//! // With custom color scale
//! let custom = heatmap(data)
//!     .color_scale(ColorScale::Viridis)
//!     .show_values(true);
//! ```

use crate::style::Color;
use crate::widget::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Color scale for heatmap
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ColorScale {
    /// Blue to Red (cold to hot)
    #[default]
    BlueRed,
    /// Green scale (GitHub style)
    Green,
    /// Viridis (perceptually uniform)
    Viridis,
    /// Plasma
    Plasma,
    /// Grayscale
    Gray,
    /// Red to Yellow to Green (traffic light)
    RedYellowGreen,
    /// Custom two colors
    Custom,
}

impl ColorScale {
    /// Get color for normalized value (0.0 to 1.0)
    pub fn color_at(&self, value: f64) -> Color {
        let v = value.clamp(0.0, 1.0);

        match self {
            ColorScale::BlueRed => {
                // Blue -> White -> Red
                if v < 0.5 {
                    let t = v * 2.0;
                    Color::rgb((t * 255.0) as u8, (t * 255.0) as u8, 255)
                } else {
                    let t = (v - 0.5) * 2.0;
                    Color::rgb(255, ((1.0 - t) * 255.0) as u8, ((1.0 - t) * 255.0) as u8)
                }
            }
            ColorScale::Green => {
                // GitHub contribution style
                if v < 0.01 {
                    Color::rgb(22, 27, 34) // Empty
                } else if v < 0.25 {
                    Color::rgb(14, 68, 41)
                } else if v < 0.50 {
                    Color::rgb(0, 109, 50)
                } else if v < 0.75 {
                    Color::rgb(38, 166, 65)
                } else {
                    Color::rgb(57, 211, 83)
                }
            }
            ColorScale::Viridis => {
                // Approximation of viridis colormap
                let r = (68.0 + v * (253.0 - 68.0) * (1.0 - v.powi(2))) as u8;
                let g = (1.0 + v * 230.0) as u8;
                let b = (84.0 + v * 50.0 - v.powi(2) * 100.0).max(30.0) as u8;
                Color::rgb(r, g, b)
            }
            ColorScale::Plasma => {
                // Approximation of plasma colormap
                let r = (13.0 + v * 230.0) as u8;
                let g = (8.0 + v * 90.0 + (1.0 - v) * 60.0) as u8;
                let b = (135.0 + v * 20.0 - v * 120.0).max(20.0) as u8;
                Color::rgb(r, g, b)
            }
            ColorScale::Gray => {
                let c = (v * 255.0) as u8;
                Color::rgb(c, c, c)
            }
            ColorScale::RedYellowGreen => {
                // Traffic light: Red -> Yellow -> Green
                if v < 0.5 {
                    let t = v * 2.0;
                    Color::rgb(255, (t * 255.0) as u8, 0)
                } else {
                    let t = (v - 0.5) * 2.0;
                    Color::rgb(((1.0 - t) * 255.0) as u8, 255, 0)
                }
            }
            ColorScale::Custom => Color::WHITE, // Override with custom_colors
        }
    }
}

/// Cell display mode
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CellDisplay {
    /// Block character (█)
    #[default]
    Block,
    /// Half block for higher resolution
    HalfBlock,
    /// Show numeric value
    Value,
    /// Custom character
    Custom,
}

/// Heat map widget
#[derive(Clone, Debug)]
pub struct HeatMap {
    /// Data grid (row-major)
    data: Vec<Vec<f64>>,
    /// Number of rows (cached for future use)
    _rows: usize,
    /// Number of columns
    cols: usize,
    /// Min value (for normalization)
    min_val: f64,
    /// Max value (for normalization)
    max_val: f64,
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
    fn normalize(&self, value: f64) -> f64 {
        if self.max_val == self.min_val {
            0.5
        } else {
            (value - self.min_val) / (self.max_val - self.min_val)
        }
    }

    /// Get color for value
    fn color_for(&self, value: f64) -> Color {
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
    fn render_cell(&self, value: f64) -> String {
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

impl View for HeatMap {
    crate::impl_view_meta!("HeatMap");

    fn render(&self, ctx: &mut RenderContext) {
        use crate::widget::stack::{hstack, vstack};
        use crate::widget::Text;

        let mut content = vstack();

        // Title
        if let Some(title) = &self.title {
            content = content.child(Text::new(title).bold());
        }

        // Column labels
        if let Some(labels) = &self.col_labels {
            let label_offset = if self.row_labels.is_some() { 8 } else { 0 };
            let mut col_header = hstack();
            col_header = col_header.child(Text::new(" ".repeat(label_offset)));

            for label in labels.iter().take(self.cols) {
                let truncated = if label.len() > self.cell_width {
                    &label[..self.cell_width]
                } else {
                    label
                };
                col_header = col_header.child(
                    Text::new(format!("{:^width$}", truncated, width = self.cell_width))
                        .fg(Color::rgb(150, 150, 150)),
                );
            }
            content = content.child(col_header);
        }

        // Data rows
        for (row_idx, row) in self.data.iter().enumerate() {
            for _ in 0..self.cell_height {
                let mut row_view = hstack();

                // Row label
                if let Some(labels) = &self.row_labels {
                    if let Some(label) = labels.get(row_idx) {
                        let truncated = if label.len() > 6 { &label[..6] } else { label };
                        row_view = row_view.child(
                            Text::new(format!("{:>6} ", truncated)).fg(Color::rgb(150, 150, 150)),
                        );
                    }
                }

                // Cells
                for (col_idx, &value) in row.iter().enumerate() {
                    let color = self.color_for(value);
                    let cell_str = self.render_cell(value);

                    let is_highlighted = self.highlighted == Some((row_idx, col_idx));

                    let mut cell_text = Text::new(&cell_str);

                    if self.show_values {
                        // Show value with colored background
                        cell_text = cell_text.bg(color);
                        // Contrast text color
                        let brightness = (color.r as u32 + color.g as u32 + color.b as u32) / 3;
                        if brightness > 128 {
                            cell_text = cell_text.fg(Color::BLACK);
                        } else {
                            cell_text = cell_text.fg(Color::WHITE);
                        }
                    } else {
                        cell_text = cell_text.fg(color);
                    }

                    if is_highlighted {
                        cell_text = cell_text.bold();
                    }

                    row_view = row_view.child(cell_text);
                }

                content = content.child(row_view);
            }
        }

        // Legend
        if self.show_legend {
            let mut legend = hstack();
            legend = legend.child(Text::new("Low ").fg(Color::rgb(128, 128, 128)));

            for i in 0..10 {
                let v = i as f64 / 9.0;
                let color = self.color_scale.color_at(v);
                legend = legend.child(Text::new("█").fg(color));
            }

            legend = legend.child(Text::new(" High").fg(Color::rgb(128, 128, 128)));
            legend = legend.child(
                Text::new(format!("  ({:.1} - {:.1})", self.min_val, self.max_val))
                    .fg(Color::rgb(100, 100, 100)),
            );

            content = content.child(legend);
        }

        content.render(ctx);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heatmap_new() {
        let data = vec![vec![0.0, 0.5, 1.0], vec![0.2, 0.4, 0.8]];
        let hm = HeatMap::new(data);
        assert_eq!(hm._rows, 2);
        assert_eq!(hm.cols, 3);
        assert_eq!(hm.min_val, 0.0);
        assert_eq!(hm.max_val, 1.0);
    }

    #[test]
    fn test_heatmap_from_flat() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let hm = HeatMap::from_flat(&data, 2, 3);
        assert_eq!(hm._rows, 2);
        assert_eq!(hm.cols, 3);
        assert_eq!(hm.data[0][0], 1.0);
        assert_eq!(hm.data[1][2], 6.0);
    }

    #[test]
    fn test_normalization() {
        let hm = HeatMap::new(vec![vec![10.0, 20.0, 30.0]]);
        assert!((hm.normalize(10.0) - 0.0).abs() < 0.001);
        assert!((hm.normalize(20.0) - 0.5).abs() < 0.001);
        assert!((hm.normalize(30.0) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_color_scale_blue_red() {
        let scale = ColorScale::BlueRed;
        let low = scale.color_at(0.0);
        let _mid = scale.color_at(0.5);
        let high = scale.color_at(1.0);

        assert_eq!(low.b, 255); // Blue
        assert_eq!(high.r, 255); // Red
    }

    #[test]
    fn test_color_scale_green() {
        let scale = ColorScale::Green;
        let empty = scale.color_at(0.0);
        let full = scale.color_at(1.0);

        // Empty should be dark
        assert!(empty.r < 50);
        // Full should be bright green
        assert!(full.g > 200);
    }

    #[test]
    fn test_custom_bounds() {
        let hm = HeatMap::new(vec![vec![5.0]]).bounds(-10.0, 10.0);
        assert_eq!(hm.min_val, -10.0);
        assert_eq!(hm.max_val, 10.0);
        assert!((hm.normalize(5.0) - 0.75).abs() < 0.001);
    }

    #[test]
    fn test_contribution_map() {
        let contributions: Vec<u32> = (0..364).map(|i| i % 10).collect();
        let hm = HeatMap::contribution_map(&contributions);
        assert_eq!(hm.color_scale, ColorScale::Green);
    }

    #[test]
    fn test_labels() {
        let hm = HeatMap::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]])
            .row_labels(vec!["A".into(), "B".into()])
            .col_labels(vec!["X".into(), "Y".into()]);

        assert!(hm.row_labels.is_some());
        assert!(hm.col_labels.is_some());
    }

    #[test]
    fn test_helper_functions() {
        let data = vec![vec![0.5]];
        let hm = heatmap(data);
        assert_eq!(hm._rows, 1);
    }
}
