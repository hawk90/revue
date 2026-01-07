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

    // ==================== ColorScale Tests ====================

    #[test]
    fn test_color_scale_viridis() {
        let scale = ColorScale::Viridis;
        let low = scale.color_at(0.0);
        let high = scale.color_at(1.0);

        // Viridis starts purple-ish and ends yellow-ish
        assert!(low.b > low.r); // Low end has more blue
        assert!(high.g > high.b); // High end has more green
    }

    #[test]
    fn test_color_scale_plasma() {
        let scale = ColorScale::Plasma;
        let low = scale.color_at(0.0);
        let mid = scale.color_at(0.5);
        let high = scale.color_at(1.0);

        // Plasma goes from dark purple to bright yellow
        assert!(low.r < 50); // Starts dark
        assert!(high.r > 200); // Ends bright
        assert!(mid.r > low.r && mid.r < high.r); // Monotonic increase
    }

    #[test]
    fn test_color_scale_gray() {
        let scale = ColorScale::Gray;
        let black = scale.color_at(0.0);
        let white = scale.color_at(1.0);
        let mid = scale.color_at(0.5);

        assert_eq!(black.r, 0);
        assert_eq!(black.g, 0);
        assert_eq!(black.b, 0);

        assert_eq!(white.r, 255);
        assert_eq!(white.g, 255);
        assert_eq!(white.b, 255);

        // Grayscale should have equal RGB
        assert_eq!(mid.r, mid.g);
        assert_eq!(mid.g, mid.b);
    }

    #[test]
    fn test_color_scale_red_yellow_green() {
        let scale = ColorScale::RedYellowGreen;
        let red = scale.color_at(0.0);
        let yellow = scale.color_at(0.5);
        let green = scale.color_at(1.0);

        // At 0.0: Red
        assert_eq!(red.r, 255);
        assert_eq!(red.g, 0);
        assert_eq!(red.b, 0);

        // At 0.5: Yellow
        assert_eq!(yellow.r, 255);
        assert_eq!(yellow.g, 255);
        assert_eq!(yellow.b, 0);

        // At 1.0: Green
        assert_eq!(green.r, 0);
        assert_eq!(green.g, 255);
        assert_eq!(green.b, 0);
    }

    #[test]
    fn test_color_scale_custom_returns_white() {
        let scale = ColorScale::Custom;
        let color = scale.color_at(0.5);
        // Custom returns WHITE when used directly (override with custom_colors)
        assert_eq!(color, Color::WHITE);
    }

    #[test]
    fn test_color_scale_green_buckets() {
        let scale = ColorScale::Green;

        // Test each bucket
        let empty = scale.color_at(0.0); // < 0.01
        let low = scale.color_at(0.15); // 0.01-0.25
        let med_low = scale.color_at(0.35); // 0.25-0.50
        let med_high = scale.color_at(0.60); // 0.50-0.75
        let high = scale.color_at(0.90); // >= 0.75

        // Empty is darkest
        assert_eq!(empty, Color::rgb(22, 27, 34));
        // Low
        assert_eq!(low, Color::rgb(14, 68, 41));
        // Medium-low
        assert_eq!(med_low, Color::rgb(0, 109, 50));
        // Medium-high
        assert_eq!(med_high, Color::rgb(38, 166, 65));
        // High
        assert_eq!(high, Color::rgb(57, 211, 83));
    }

    #[test]
    fn test_color_scale_blue_red_gradient() {
        let scale = ColorScale::BlueRed;

        // Lower half: blue to white
        let low = scale.color_at(0.25);
        assert_eq!(low.b, 255); // Blue stays at 255
        assert!(low.r > 0 && low.r < 255); // Red increases

        // Upper half: white to red
        let high = scale.color_at(0.75);
        assert_eq!(high.r, 255); // Red stays at 255
        assert!(high.b < 255 && high.b > 0); // Blue decreases
    }

    #[test]
    fn test_color_scale_value_clamping() {
        let scale = ColorScale::Gray;

        // Values below 0 should clamp to 0
        let below = scale.color_at(-1.0);
        assert_eq!(below, Color::rgb(0, 0, 0));

        // Values above 1 should clamp to 1
        let above = scale.color_at(2.0);
        assert_eq!(above, Color::rgb(255, 255, 255));
    }

    #[test]
    fn test_color_scale_default() {
        assert_eq!(ColorScale::default(), ColorScale::BlueRed);
    }

    #[test]
    fn test_color_scale_debug_and_clone() {
        let scale = ColorScale::Viridis;
        let cloned = scale;
        assert_eq!(scale, cloned);
        let _ = format!("{:?}", scale);
    }

    // ==================== CellDisplay Tests ====================

    #[test]
    fn test_cell_display_default() {
        assert_eq!(CellDisplay::default(), CellDisplay::Block);
    }

    #[test]
    fn test_cell_display_debug_and_clone() {
        let display = CellDisplay::HalfBlock;
        let cloned = display;
        assert_eq!(display, cloned);
        let _ = format!("{:?}", display);
    }

    // ==================== HeatMap Builder Tests ====================

    #[test]
    fn test_heatmap_custom_colors() {
        let hm = HeatMap::new(vec![vec![0.0, 1.0]]).custom_colors(Color::BLUE, Color::RED);

        assert_eq!(hm.color_scale, ColorScale::Custom);
        assert!(hm.custom_colors.is_some());

        let (low, high) = hm.custom_colors.unwrap();
        assert_eq!(low, Color::BLUE);
        assert_eq!(high, Color::RED);
    }

    #[test]
    fn test_heatmap_cell_display() {
        let hm = HeatMap::new(vec![vec![0.5]]).cell_display(CellDisplay::HalfBlock);
        assert_eq!(hm.cell_display, CellDisplay::HalfBlock);
    }

    #[test]
    fn test_heatmap_cell_size() {
        let hm = HeatMap::new(vec![vec![0.5]]).cell_size(5, 3);
        assert_eq!(hm.cell_width, 5);
        assert_eq!(hm.cell_height, 3);
    }

    #[test]
    fn test_heatmap_cell_width() {
        let hm = HeatMap::new(vec![vec![0.5]]).cell_width(8);
        assert_eq!(hm.cell_width, 8);
    }

    #[test]
    fn test_heatmap_cell_height() {
        let hm = HeatMap::new(vec![vec![0.5]]).cell_height(4);
        assert_eq!(hm.cell_height, 4);
    }

    #[test]
    fn test_heatmap_show_values_increases_cell_width() {
        let hm = HeatMap::new(vec![vec![0.5]])
            .cell_width(2)
            .show_values(true);

        // show_values increases cell_width to at least 4
        assert!(hm.cell_width >= 4);
        assert!(hm.show_values);
    }

    #[test]
    fn test_heatmap_show_values_keeps_larger_width() {
        let hm = HeatMap::new(vec![vec![0.5]])
            .cell_width(10)
            .show_values(true);

        // Should keep the larger width
        assert_eq!(hm.cell_width, 10);
    }

    #[test]
    fn test_heatmap_value_decimals() {
        let hm = HeatMap::new(vec![vec![0.5]]).value_decimals(3);
        assert_eq!(hm.value_decimals, 3);
    }

    #[test]
    fn test_heatmap_title() {
        let hm = HeatMap::new(vec![vec![0.5]]).title("My Heatmap");
        assert_eq!(hm.title, Some("My Heatmap".to_string()));
    }

    #[test]
    fn test_heatmap_show_legend() {
        let hm = HeatMap::new(vec![vec![0.5]]).show_legend(true);
        assert!(hm.show_legend);
    }

    #[test]
    fn test_heatmap_highlight() {
        let hm = HeatMap::new(vec![vec![0.5, 0.6], vec![0.7, 0.8]]).highlight(1, 0);
        assert_eq!(hm.highlighted, Some((1, 0)));
    }

    // ==================== HeatMap Helper Functions ====================

    #[test]
    fn test_correlation_matrix() {
        let data = vec![
            vec![1.0, 0.5, -0.5],
            vec![0.5, 1.0, 0.3],
            vec![-0.5, 0.3, 1.0],
        ];
        let labels = vec!["A".into(), "B".into(), "C".into()];
        let hm = HeatMap::correlation_matrix(&data, labels);

        assert_eq!(hm.color_scale, ColorScale::BlueRed);
        assert_eq!(hm.min_val, -1.0);
        assert_eq!(hm.max_val, 1.0);
        assert!(hm.row_labels.is_some());
        assert!(hm.col_labels.is_some());
        assert!(hm.show_values);
    }

    #[test]
    fn test_contribution_map_helper() {
        let contributions: Vec<u32> = (0..100).collect();
        let hm = contribution_map(&contributions);
        assert_eq!(hm.color_scale, ColorScale::Green);
    }

    // ==================== Normalization Tests ====================

    #[test]
    fn test_normalize_same_min_max() {
        let hm = HeatMap::new(vec![vec![5.0, 5.0, 5.0]]);
        // When min == max, normalize returns 0.5
        assert_eq!(hm.normalize(5.0), 0.5);
    }

    #[test]
    fn test_normalize_range() {
        let hm = HeatMap::new(vec![vec![0.0, 100.0]]);
        assert_eq!(hm.normalize(0.0), 0.0);
        assert_eq!(hm.normalize(50.0), 0.5);
        assert_eq!(hm.normalize(100.0), 1.0);
        assert_eq!(hm.normalize(25.0), 0.25);
    }

    // ==================== Color For Value Tests ====================

    #[test]
    fn test_color_for_with_custom_colors() {
        let hm = HeatMap::new(vec![vec![0.0, 1.0]])
            .custom_colors(Color::rgb(0, 0, 0), Color::rgb(255, 255, 255));

        let low_color = hm.color_for(0.0);
        let high_color = hm.color_for(1.0);
        let mid_color = hm.color_for(0.5);

        assert_eq!(low_color, Color::rgb(0, 0, 0));
        assert_eq!(high_color, Color::rgb(255, 255, 255));
        // Mid should be approximately gray
        assert!(mid_color.r > 100 && mid_color.r < 150);
    }

    #[test]
    fn test_color_for_with_standard_scale() {
        let hm = HeatMap::new(vec![vec![0.0, 1.0]]).color_scale(ColorScale::Gray);

        let low = hm.color_for(0.0);
        let high = hm.color_for(1.0);

        assert_eq!(low, Color::rgb(0, 0, 0));
        assert_eq!(high, Color::rgb(255, 255, 255));
    }

    // ==================== Render Cell Tests ====================

    #[test]
    fn test_render_cell_block() {
        let hm = HeatMap::new(vec![vec![0.5]])
            .cell_display(CellDisplay::Block)
            .cell_width(3);

        let cell = hm.render_cell(0.5);
        assert_eq!(cell, "███");
    }

    #[test]
    fn test_render_cell_half_block() {
        let hm = HeatMap::new(vec![vec![0.5]])
            .cell_display(CellDisplay::HalfBlock)
            .cell_width(2);

        let cell = hm.render_cell(0.5);
        assert_eq!(cell, "▀▀");
    }

    #[test]
    fn test_render_cell_value_display() {
        let hm = HeatMap::new(vec![vec![0.5]])
            .cell_display(CellDisplay::Value)
            .cell_width(4)
            .value_decimals(1);

        let cell = hm.render_cell(0.5);
        assert!(cell.contains("0.5"));
    }

    #[test]
    fn test_render_cell_custom() {
        let hm = HeatMap::new(vec![vec![0.5]])
            .cell_display(CellDisplay::Custom)
            .cell_width(2);

        let cell = hm.render_cell(0.5);
        assert_eq!(cell, "■■");
    }

    #[test]
    fn test_render_cell_show_values_overrides_display() {
        let hm = HeatMap::new(vec![vec![0.5]])
            .cell_display(CellDisplay::Block)
            .show_values(true)
            .value_decimals(2);

        let cell = hm.render_cell(0.5);
        // show_values takes precedence over cell_display
        assert!(cell.contains("0.50"));
    }

    // ==================== Edge Cases ====================

    #[test]
    fn test_heatmap_empty_data() {
        let hm = HeatMap::new(vec![]);
        assert_eq!(hm._rows, 0);
        assert_eq!(hm.cols, 0);
        // When empty, defaults apply
        assert_eq!(hm.min_val, 0.0);
        assert_eq!(hm.max_val, 1.0);
    }

    #[test]
    fn test_heatmap_empty_rows() {
        let hm = HeatMap::new(vec![vec![], vec![]]);
        assert_eq!(hm._rows, 2);
        assert_eq!(hm.cols, 0);
    }

    #[test]
    fn test_from_flat_partial_data() {
        // Data smaller than rows * cols
        let data = vec![1.0, 2.0, 3.0];
        let hm = HeatMap::from_flat(&data, 2, 3);
        assert_eq!(hm._rows, 2);
        assert_eq!(hm.data[0].len(), 3);
        assert_eq!(hm.data[1].len(), 0); // Second row is empty since data ran out
    }

    #[test]
    fn test_heatmap_negative_values() {
        let hm = HeatMap::new(vec![vec![-10.0, 0.0, 10.0]]);
        assert_eq!(hm.min_val, -10.0);
        assert_eq!(hm.max_val, 10.0);
        assert_eq!(hm.normalize(-10.0), 0.0);
        assert_eq!(hm.normalize(0.0), 0.5);
        assert_eq!(hm.normalize(10.0), 1.0);
    }

    // ==================== Rendering Tests ====================

    #[test]
    fn test_heatmap_render_basic() {
        use crate::layout::Rect;
        use crate::render::Buffer;

        let hm = HeatMap::new(vec![vec![0.0, 0.5, 1.0]]).cell_width(1);

        let mut buffer = Buffer::new(20, 5);
        let rect = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        hm.render(&mut ctx);
        // Basic smoke test - just ensure no panic
    }

    #[test]
    fn test_heatmap_render_with_title() {
        use crate::layout::Rect;
        use crate::render::Buffer;

        let hm = HeatMap::new(vec![vec![0.5]]).title("Test Title");

        let mut buffer = Buffer::new(20, 5);
        let rect = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        hm.render(&mut ctx);

        // Check that first row contains title characters
        assert_eq!(buffer.get(0, 0).unwrap().symbol, 'T');
        assert_eq!(buffer.get(1, 0).unwrap().symbol, 'e');
        assert_eq!(buffer.get(2, 0).unwrap().symbol, 's');
        assert_eq!(buffer.get(3, 0).unwrap().symbol, 't');
    }

    #[test]
    fn test_heatmap_render_with_labels() {
        use crate::layout::Rect;
        use crate::render::Buffer;

        let hm = HeatMap::new(vec![vec![0.5, 0.8], vec![0.2, 0.9]])
            .row_labels(vec!["R1".into(), "R2".into()])
            .col_labels(vec!["C1".into(), "C2".into()]);

        let mut buffer = Buffer::new(30, 10);
        let rect = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        hm.render(&mut ctx);

        // Smoke test - render should complete without panic
        // Labels are rendered somewhere in the buffer
    }

    #[test]
    fn test_heatmap_render_with_legend() {
        use crate::layout::Rect;
        use crate::render::Buffer;

        let hm = HeatMap::new(vec![vec![0.0, 1.0]]).show_legend(true);

        let mut buffer = Buffer::new(40, 10);
        let rect = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        hm.render(&mut ctx);

        // Smoke test - legend should render without panic
        // The legend contains "Low" and "High" labels somewhere in the buffer
    }

    #[test]
    fn test_heatmap_render_with_values() {
        use crate::layout::Rect;
        use crate::render::Buffer;

        let hm = HeatMap::new(vec![vec![0.5]])
            .show_values(true)
            .value_decimals(1);

        let mut buffer = Buffer::new(20, 5);
        let rect = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        hm.render(&mut ctx);
        // Smoke test
    }

    #[test]
    fn test_heatmap_render_with_highlight() {
        use crate::layout::Rect;
        use crate::render::Buffer;

        let hm = HeatMap::new(vec![vec![0.5, 0.8], vec![0.2, 0.9]]).highlight(0, 1);

        let mut buffer = Buffer::new(20, 10);
        let rect = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        hm.render(&mut ctx);
        // Smoke test - highlight should apply bold
    }

    #[test]
    fn test_heatmap_render_multiline_cells() {
        use crate::layout::Rect;
        use crate::render::Buffer;

        let hm = HeatMap::new(vec![vec![0.5]]).cell_height(2);

        let mut buffer = Buffer::new(20, 10);
        let rect = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        hm.render(&mut ctx);
        // Smoke test
    }

    #[test]
    fn test_heatmap_clone() {
        let hm = HeatMap::new(vec![vec![0.5]])
            .title("Test")
            .color_scale(ColorScale::Viridis);

        let cloned = hm.clone();
        assert_eq!(cloned.title, Some("Test".to_string()));
        assert_eq!(cloned.color_scale, ColorScale::Viridis);
    }

    #[test]
    fn test_heatmap_debug() {
        let hm = HeatMap::new(vec![vec![0.5]]);
        let debug = format!("{:?}", hm);
        assert!(debug.contains("HeatMap"));
    }

    // ==================== Brightness Contrast Tests ====================

    #[test]
    fn test_render_value_brightness_contrast() {
        use crate::layout::Rect;
        use crate::render::Buffer;

        // Test that high brightness cells get black text, low brightness get white
        let hm = HeatMap::new(vec![vec![0.0, 1.0]])
            .color_scale(ColorScale::Gray)
            .show_values(true);

        let mut buffer = Buffer::new(20, 5);
        let rect = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        hm.render(&mut ctx);
        // This tests the brightness > 128 branch in render
    }

    // ==================== Label Truncation Tests ====================

    #[test]
    fn test_col_label_truncation() {
        use crate::layout::Rect;
        use crate::render::Buffer;

        let hm = HeatMap::new(vec![vec![0.5]])
            .cell_width(3)
            .col_labels(vec!["VeryLongLabel".into()]);

        let mut buffer = Buffer::new(30, 10);
        let rect = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        hm.render(&mut ctx);
        // Column label should be truncated to cell_width
    }

    #[test]
    fn test_row_label_truncation() {
        use crate::layout::Rect;
        use crate::render::Buffer;

        let hm = HeatMap::new(vec![vec![0.5]]).row_labels(vec!["VeryLongRowLabel".into()]);

        let mut buffer = Buffer::new(30, 10);
        let rect = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        hm.render(&mut ctx);
        // Row label should be truncated to 6 chars
    }
}
