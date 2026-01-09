//! Histogram widget for frequency distribution visualization
//!
//! Supports automatic binning, density normalization, cumulative histograms, and statistics overlay.

use super::chart_common::{Axis, ChartGrid, ChartOrientation, Legend};
use super::traits::{RenderContext, View, WidgetProps};
use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::{impl_props_builders, impl_styled_view};

/// Bin configuration for histogram
#[derive(Clone, Debug, Default)]
pub enum BinConfig {
    /// Automatic binning (Sturges' rule)
    #[default]
    Auto,
    /// Fixed number of bins
    Count(usize),
    /// Fixed bin width
    Width(f64),
    /// Custom bin edges
    Edges(Vec<f64>),
}

/// A single bin in the histogram
#[derive(Clone, Debug)]
pub struct HistogramBin {
    /// Bin start (inclusive)
    pub start: f64,
    /// Bin end (exclusive)
    pub end: f64,
    /// Count of values in bin
    pub count: usize,
    /// Frequency (count / total)
    pub frequency: f64,
    /// Density (frequency / bin_width)
    pub density: f64,
}

/// Histogram widget
pub struct Histogram {
    /// Raw data values
    data: Vec<f64>,
    /// Computed bins
    bins: Vec<HistogramBin>,
    /// Bin configuration
    bin_config: BinConfig,
    /// Orientation
    orientation: ChartOrientation,
    /// X axis configuration
    x_axis: Axis,
    /// Y axis configuration
    y_axis: Axis,
    /// Legend configuration
    legend: Legend,
    /// Grid configuration
    grid: ChartGrid,
    /// Fill color
    fill_color: Color,
    /// Border color for bars
    bar_border: Option<Color>,
    /// Show cumulative distribution
    cumulative: bool,
    /// Normalize to density
    density: bool,
    /// Show statistics (mean, median)
    show_stats: bool,
    /// Chart title
    title: Option<String>,
    /// Background color
    bg_color: Option<Color>,
    /// Widget properties
    props: WidgetProps,
}

impl Default for Histogram {
    fn default() -> Self {
        Self::new(&[])
    }
}

impl Histogram {
    /// Create a new histogram from data
    pub fn new(data: &[f64]) -> Self {
        let mut hist = Self {
            data: data.to_vec(),
            bins: Vec::new(),
            bin_config: BinConfig::Auto,
            orientation: ChartOrientation::Vertical,
            x_axis: Axis::default(),
            y_axis: Axis::default(),
            legend: Legend::none(),
            grid: ChartGrid::new().y(true),
            fill_color: Color::rgb(97, 175, 239),
            bar_border: None,
            cumulative: false,
            density: false,
            show_stats: false,
            title: None,
            bg_color: None,
            props: WidgetProps::new(),
        };
        hist.compute_bins();
        hist
    }

    /// Set data
    pub fn data(mut self, data: &[f64]) -> Self {
        self.data = data.to_vec();
        self.compute_bins();
        self
    }

    /// Set bin configuration
    pub fn bins(mut self, config: BinConfig) -> Self {
        self.bin_config = config;
        self.compute_bins();
        self
    }

    /// Set number of bins
    pub fn bin_count(mut self, count: usize) -> Self {
        self.bin_config = BinConfig::Count(count);
        self.compute_bins();
        self
    }

    /// Set bin width
    pub fn bin_width(mut self, width: f64) -> Self {
        self.bin_config = BinConfig::Width(width);
        self.compute_bins();
        self
    }

    /// Set orientation
    pub fn orientation(mut self, orientation: ChartOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set horizontal orientation
    pub fn horizontal(mut self) -> Self {
        self.orientation = ChartOrientation::Horizontal;
        self
    }

    /// Set vertical orientation
    pub fn vertical(mut self) -> Self {
        self.orientation = ChartOrientation::Vertical;
        self
    }

    /// Set X axis configuration
    pub fn x_axis(mut self, axis: Axis) -> Self {
        self.x_axis = axis;
        self
    }

    /// Set Y axis configuration
    pub fn y_axis(mut self, axis: Axis) -> Self {
        self.y_axis = axis;
        self
    }

    /// Set legend configuration
    pub fn legend(mut self, legend: Legend) -> Self {
        self.legend = legend;
        self
    }

    /// Set grid configuration
    pub fn grid(mut self, grid: ChartGrid) -> Self {
        self.grid = grid;
        self
    }

    /// Set fill color
    pub fn fill_color(mut self, color: Color) -> Self {
        self.fill_color = color;
        self
    }

    /// Alias for fill_color
    pub fn color(mut self, color: Color) -> Self {
        self.fill_color = color;
        self
    }

    /// Set bar border color
    pub fn bar_border(mut self, color: Color) -> Self {
        self.bar_border = Some(color);
        self
    }

    /// Enable cumulative distribution
    pub fn cumulative(mut self, enabled: bool) -> Self {
        self.cumulative = enabled;
        self
    }

    /// Enable density normalization
    pub fn density(mut self, enabled: bool) -> Self {
        self.density = enabled;
        self
    }

    /// Show statistics (mean, median lines)
    pub fn show_stats(mut self, enabled: bool) -> Self {
        self.show_stats = enabled;
        self
    }

    /// Set chart title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self
    }

    /// Compute bins from data
    fn compute_bins(&mut self) {
        if self.data.is_empty() {
            self.bins.clear();
            return;
        }

        // Filter valid values
        let valid_data: Vec<f64> = self
            .data
            .iter()
            .filter(|x| x.is_finite())
            .copied()
            .collect();
        if valid_data.is_empty() {
            self.bins.clear();
            return;
        }

        let min = valid_data.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = valid_data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let range = (max - min).max(1.0);

        // Determine bin edges
        let edges = match &self.bin_config {
            BinConfig::Auto => {
                // Sturges' rule
                let n = valid_data.len();
                let bin_count = ((n as f64).log2() + 1.0).ceil() as usize;
                let bin_count = bin_count.clamp(1, 100);
                let bin_width = range / bin_count as f64;
                (0..=bin_count)
                    .map(|i| min + i as f64 * bin_width)
                    .collect::<Vec<_>>()
            }
            BinConfig::Count(n) => {
                let bin_count = (*n).max(1);
                let bin_width = range / bin_count as f64;
                (0..=bin_count)
                    .map(|i| min + i as f64 * bin_width)
                    .collect::<Vec<_>>()
            }
            BinConfig::Width(w) => {
                let bin_width = (*w).max(0.001);
                let bin_count = (range / bin_width).ceil() as usize;
                (0..=bin_count)
                    .map(|i| min + i as f64 * bin_width)
                    .collect::<Vec<_>>()
            }
            BinConfig::Edges(edges) => edges.clone(),
        };

        // Count values in each bin
        let total = valid_data.len();
        let mut bins = Vec::new();

        for i in 0..edges.len().saturating_sub(1) {
            let start = edges[i];
            let end = edges[i + 1];
            let count = valid_data
                .iter()
                .filter(|&&x| {
                    if i == edges.len() - 2 {
                        x >= start && x <= end // Include last edge
                    } else {
                        x >= start && x < end
                    }
                })
                .count();

            let frequency = count as f64 / total as f64;
            let bin_width = end - start;
            let density = if bin_width > 0.0 {
                frequency / bin_width
            } else {
                0.0
            };

            bins.push(HistogramBin {
                start,
                end,
                count,
                frequency,
                density,
            });
        }

        self.bins = bins;
    }

    /// Get the mean of the data
    fn mean(&self) -> Option<f64> {
        let valid: Vec<f64> = self
            .data
            .iter()
            .filter(|x| x.is_finite())
            .copied()
            .collect();
        if valid.is_empty() {
            return None;
        }
        Some(valid.iter().sum::<f64>() / valid.len() as f64)
    }

    /// Get the median of the data
    fn median(&self) -> Option<f64> {
        let mut valid: Vec<f64> = self
            .data
            .iter()
            .filter(|x| x.is_finite())
            .copied()
            .collect();
        if valid.is_empty() {
            return None;
        }
        valid.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mid = valid.len() / 2;
        if valid.len().is_multiple_of(2) {
            Some((valid[mid - 1] + valid[mid]) / 2.0)
        } else {
            Some(valid[mid])
        }
    }

    /// Get max value for y-axis
    fn max_value(&self) -> f64 {
        if self.cumulative {
            1.0
        } else if self.density {
            self.bins
                .iter()
                .map(|b| b.density)
                .fold(0.0, f64::max)
                .max(0.001)
        } else {
            self.bins.iter().map(|b| b.count).max().unwrap_or(1) as f64
        }
    }

    /// Get bin value based on settings
    fn bin_value(&self, bin: &HistogramBin, cumulative_sum: f64) -> f64 {
        if self.cumulative {
            cumulative_sum
        } else if self.density {
            bin.density
        } else {
            bin.count as f64
        }
    }

    /// Render histogram bars
    fn render_bars(&self, ctx: &mut RenderContext, chart_area: Rect) {
        if self.bins.is_empty() {
            return;
        }

        let x_min = self.bins.first().map(|b| b.start).unwrap_or(0.0);
        let x_max = self.bins.last().map(|b| b.end).unwrap_or(1.0);
        let x_range = (x_max - x_min).max(1.0);
        let y_max = self.max_value();

        let mut cumulative_sum = 0.0;

        for bin in &self.bins {
            cumulative_sum += bin.frequency;
            let value = self.bin_value(bin, cumulative_sum);

            // Calculate bar position
            let bar_x_start = ((bin.start - x_min) / x_range * chart_area.width as f64) as u16;
            let bar_x_end = ((bin.end - x_min) / x_range * chart_area.width as f64) as u16;
            let bar_width = (bar_x_end - bar_x_start).max(1);

            let bar_height = ((value / y_max) * chart_area.height as f64) as u16;
            let bar_height = bar_height.min(chart_area.height);

            // Draw bar
            for dx in 0..bar_width {
                for dy in 0..bar_height {
                    let x = chart_area.x + bar_x_start + dx;
                    let y = chart_area.y + chart_area.height - 1 - dy;

                    if x < chart_area.x + chart_area.width && y >= chart_area.y {
                        let ch = if dy == bar_height - 1 {
                            '▀'
                        } else if self.bar_border.is_some() && (dx == 0 || dx == bar_width - 1) {
                            '│'
                        } else {
                            '█'
                        };

                        let mut cell = Cell::new(ch);
                        if self.bar_border.is_some() && (dx == 0 || dx == bar_width - 1) {
                            cell.fg = self.bar_border;
                        } else {
                            cell.fg = Some(self.fill_color);
                        }
                        ctx.buffer.set(x, y, cell);
                    }
                }
            }
        }
    }

    /// Render statistics lines
    fn render_stats(&self, ctx: &mut RenderContext, chart_area: Rect) {
        if !self.show_stats || self.bins.is_empty() {
            return;
        }

        let x_min = self.bins.first().map(|b| b.start).unwrap_or(0.0);
        let x_max = self.bins.last().map(|b| b.end).unwrap_or(1.0);
        let x_range = (x_max - x_min).max(1.0);

        // Draw mean line
        if let Some(mean) = self.mean() {
            let x = chart_area.x + ((mean - x_min) / x_range * chart_area.width as f64) as u16;
            if x >= chart_area.x && x < chart_area.x + chart_area.width {
                for y in chart_area.y..chart_area.y + chart_area.height {
                    let mut cell = Cell::new('│');
                    cell.fg = Some(Color::rgb(224, 108, 117)); // Red
                    ctx.buffer.set(x, y, cell);
                }
                // Label
                if x + 1 < chart_area.x + chart_area.width {
                    let mut cell = Cell::new('μ');
                    cell.fg = Some(Color::rgb(224, 108, 117));
                    ctx.buffer.set(x + 1, chart_area.y, cell);
                }
            }
        }

        // Draw median line
        if let Some(median) = self.median() {
            let x = chart_area.x + ((median - x_min) / x_range * chart_area.width as f64) as u16;
            if x >= chart_area.x && x < chart_area.x + chart_area.width {
                for y in chart_area.y..chart_area.y + chart_area.height {
                    let mut cell = Cell::new('┊');
                    cell.fg = Some(Color::rgb(152, 195, 121)); // Green
                    ctx.buffer.set(x, y, cell);
                }
                // Label
                if x + 1 < chart_area.x + chart_area.width {
                    let mut cell = Cell::new('M');
                    cell.fg = Some(Color::rgb(152, 195, 121));
                    ctx.buffer.set(x + 1, chart_area.y, cell);
                }
            }
        }
    }

    /// Render axis labels
    fn render_axes(&self, ctx: &mut RenderContext, area: Rect) {
        if self.bins.is_empty() {
            return;
        }

        let x_min = self.bins.first().map(|b| b.start).unwrap_or(0.0);
        let x_max = self.bins.last().map(|b| b.end).unwrap_or(1.0);
        let y_max = self.max_value();

        // Y axis labels
        let y_label_width = 6u16;
        for i in 0..=4 {
            let value = y_max * (1.0 - i as f64 / 4.0);
            let label = if self.density || self.cumulative {
                format!("{:.2}", value)
            } else {
                format!("{:.0}", value)
            };
            let y = area.y + 1 + (i as u16 * (area.height - 3) / 4);

            for (j, ch) in label.chars().take(y_label_width as usize - 1).enumerate() {
                let x = area.x + j as u16;
                if x < area.x + y_label_width && y < area.y + area.height {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(self.y_axis.color);
                    ctx.buffer.set(x, y, cell);
                }
            }
        }

        // X axis labels
        let chart_width = area.width.saturating_sub(y_label_width);
        for i in 0..=4 {
            let value = x_min + (x_max - x_min) * i as f64 / 4.0;
            let label = self.x_axis.format_value(value);
            let x = area.x + y_label_width + (i as u16 * chart_width / 4);
            let y = area.y + area.height - 1;

            for (j, ch) in label.chars().take(6).enumerate() {
                let label_x = x + j as u16;
                if label_x < area.x + area.width {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(self.x_axis.color);
                    ctx.buffer.set(label_x, y, cell);
                }
            }
        }
    }
}

impl View for Histogram {
    crate::impl_view_meta!("Histogram");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;

        if area.width < 15 || area.height < 5 {
            return;
        }

        // Fill background
        if let Some(bg) = self.bg_color {
            for y in area.y..area.y + area.height {
                for x in area.x..area.x + area.width {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(bg);
                    ctx.buffer.set(x, y, cell);
                }
            }
        }

        // Draw title
        let title_offset = if let Some(ref title) = self.title {
            let title_x = area.x + (area.width.saturating_sub(title.len() as u16)) / 2;
            for (i, ch) in title.chars().enumerate() {
                let x = title_x + i as u16;
                if x < area.x + area.width {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(Color::WHITE);
                    ctx.buffer.set(x, area.y, cell);
                }
            }
            1
        } else {
            0
        };

        // Calculate chart area
        let y_label_width = 6u16;
        let x_label_height = 1u16;

        let chart_area = Rect {
            x: area.x + y_label_width,
            y: area.y + title_offset,
            width: area.width.saturating_sub(y_label_width + 1),
            height: area
                .height
                .saturating_sub(title_offset + x_label_height + 1),
        };

        if chart_area.width < 5 || chart_area.height < 3 {
            return;
        }

        // Render components
        self.render_bars(ctx, chart_area);
        self.render_stats(ctx, chart_area);
        self.render_axes(ctx, area);
    }
}

impl_styled_view!(Histogram);
impl_props_builders!(Histogram);

/// Create a new histogram
pub fn histogram(data: &[f64]) -> Histogram {
    Histogram::new(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_histogram_new() {
        let hist = Histogram::new(&[1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(hist.data.len(), 5);
        assert!(!hist.bins.is_empty());
    }

    #[test]
    fn test_histogram_empty() {
        let hist = Histogram::new(&[]);
        assert!(hist.data.is_empty());
        assert!(hist.bins.is_empty());
    }

    #[test]
    fn test_histogram_bins_auto() {
        let data: Vec<f64> = (0..100).map(|x| x as f64).collect();
        let hist = Histogram::new(&data);
        assert!(!hist.bins.is_empty());
    }

    #[test]
    fn test_histogram_bins_count() {
        let data: Vec<f64> = (0..100).map(|x| x as f64).collect();
        let hist = Histogram::new(&data).bin_count(10);
        assert_eq!(hist.bins.len(), 10);
    }

    #[test]
    fn test_histogram_bins_width() {
        let data: Vec<f64> = (0..100).map(|x| x as f64).collect();
        let hist = Histogram::new(&data).bin_width(10.0);
        assert_eq!(hist.bins.len(), 10);
    }

    #[test]
    fn test_histogram_mean() {
        let hist = Histogram::new(&[1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(hist.mean(), Some(3.0));
    }

    #[test]
    fn test_histogram_median() {
        let hist = Histogram::new(&[1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(hist.median(), Some(3.0));

        let hist = Histogram::new(&[1.0, 2.0, 3.0, 4.0]);
        assert_eq!(hist.median(), Some(2.5));
    }

    #[test]
    fn test_histogram_density() {
        let hist = Histogram::new(&[1.0, 2.0, 3.0]).density(true);
        assert!(hist.density);
    }

    #[test]
    fn test_histogram_cumulative() {
        let hist = Histogram::new(&[1.0, 2.0, 3.0]).cumulative(true);
        assert!(hist.cumulative);
    }

    #[test]
    fn test_histogram_show_stats() {
        let hist = Histogram::new(&[1.0, 2.0, 3.0]).show_stats(true);
        assert!(hist.show_stats);
    }

    #[test]
    fn test_histogram_builder() {
        let hist = Histogram::new(&[1.0, 2.0, 3.0])
            .title("Distribution")
            .bin_count(5)
            .color(Color::GREEN)
            .density(true)
            .show_stats(true)
            .x_axis(Axis::new().title("Value"))
            .y_axis(Axis::new().title("Density"));

        assert_eq!(hist.title, Some("Distribution".to_string()));
        assert!(hist.density);
        assert!(hist.show_stats);
    }

    #[test]
    fn test_histogram_helper() {
        let hist = histogram(&[1.0, 2.0, 3.0]);
        assert_eq!(hist.data.len(), 3);
    }

    #[test]
    fn test_histogram_orientation() {
        let hist = Histogram::new(&[1.0]).horizontal();
        assert_eq!(hist.orientation, ChartOrientation::Horizontal);

        let hist = Histogram::new(&[1.0]).vertical();
        assert_eq!(hist.orientation, ChartOrientation::Vertical);
    }
}
