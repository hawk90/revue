//! Histogram widget for frequency distribution visualization
//!
//! Supports automatic binning, density normalization, cumulative histograms, and statistics overlay.

use super::chart_common::{Axis, ChartGrid, ChartOrientation, Legend};
use super::chart_render::{fill_background, render_title};
use super::chart_stats::{self, mean, median};
use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

// Re-export from chart_stats for public API compatibility
pub use super::chart_stats::{BinConfig, HistogramBin};

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

    /// Compute bins from data using shared stats module
    fn compute_bins(&mut self) {
        self.bins = chart_stats::compute_bins(&self.data, &self.bin_config);
    }

    /// Get the mean of the data
    pub fn mean(&self) -> Option<f64> {
        mean(&self.data)
    }

    /// Get the median of the data
    pub fn median(&self) -> Option<f64> {
        median(&self.data)
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

        // Draw mean line using shared stats function
        if let Some(mean_val) = mean(&self.data) {
            let x = chart_area.x + ((mean_val - x_min) / x_range * chart_area.width as f64) as u16;
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

        // Draw median line using shared stats function
        if let Some(median_val) = median(&self.data) {
            let x =
                chart_area.x + ((median_val - x_min) / x_range * chart_area.width as f64) as u16;
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

        // Fill background using shared function
        if let Some(bg) = self.bg_color {
            fill_background(ctx, area, bg);
        }

        // Draw title using shared function
        let title_offset = render_title(ctx, area, self.title.as_deref(), Color::WHITE);

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

// Public API tests extracted to tests/widget/data/chart_histogram.rs
// KEEP HERE - Render tests require access to private RenderContext
#[cfg(test)]
mod tests {
    use super::*;

    // ========== Render Tests - KEEP HERE (access private RenderContext) ==========

    #[test]
    fn test_histogram_render_basic() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let data: Vec<f64> = (0..50)
            .map(|x| (x as f64) + (x as f64).sin() * 5.0)
            .collect();
        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let hist = Histogram::new(&data).bin_count(10);
        hist.render(&mut ctx);

        // Verify bars are rendered (look for block characters)
        let mut has_bars = false;
        for y in 0..20 {
            for x in 0..40 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol == '█' || cell.symbol == '▓' || cell.symbol == '▒' {
                        has_bars = true;
                        break;
                    }
                }
            }
        }
        assert!(has_bars);
    }

    #[test]
    fn test_histogram_render_with_title() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let hist = Histogram::new(&[1.0, 2.0, 3.0, 4.0, 5.0]).title("Test Distribution");
        hist.render(&mut ctx);

        // Title should be rendered
        let mut title_found = false;
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, 0) {
                if cell.symbol == 'T' {
                    title_found = true;
                    break;
                }
            }
        }
        assert!(title_found);
    }

    #[test]
    fn test_histogram_render_with_stats() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(50, 25);
        let area = Rect::new(0, 0, 50, 25);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let data: Vec<f64> = (0..100).map(|x| x as f64).collect();
        let hist = Histogram::new(&data).show_stats(true).bin_count(10);
        hist.render(&mut ctx);

        // Should render without panic and have content
        let mut has_content = false;
        for y in 0..25 {
            for x in 0..50 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol != ' ' {
                        has_content = true;
                        break;
                    }
                }
            }
        }
        assert!(has_content);
    }

    #[test]
    fn test_histogram_render_density() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let data: Vec<f64> = (0..50).map(|x| x as f64).collect();
        let hist = Histogram::new(&data).density(true);
        hist.render(&mut ctx);

        // Should render without panic
        let mut has_content = false;
        for y in 0..20 {
            for x in 0..40 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol != ' ' {
                        has_content = true;
                        break;
                    }
                }
            }
        }
        assert!(has_content);
    }

    #[test]
    fn test_histogram_render_cumulative() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let data: Vec<f64> = (0..50).map(|x| x as f64).collect();
        let hist = Histogram::new(&data).cumulative(true);
        hist.render(&mut ctx);

        // Should render without panic
        let mut has_content = false;
        for y in 0..20 {
            for x in 0..40 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol != ' ' {
                        has_content = true;
                        break;
                    }
                }
            }
        }
        assert!(has_content);
    }

    #[test]
    fn test_histogram_render_small_area() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(10, 3);
        let area = Rect::new(0, 0, 10, 3);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let hist = Histogram::new(&[1.0, 2.0, 3.0]);
        // Should not panic on small area
        hist.render(&mut ctx);
    }

    #[test]
    fn test_histogram_render_empty() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(30, 15);
        let area = Rect::new(0, 0, 30, 15);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Empty data
        let hist = Histogram::new(&[]);
        hist.render(&mut ctx);
    }

    #[test]
    fn test_histogram_render_with_grid() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let data: Vec<f64> = (0..50).map(|x| x as f64).collect();
        let hist = Histogram::new(&data).grid(ChartGrid::both());
        hist.render(&mut ctx);

        // Should have grid lines
        let mut has_content = false;
        for y in 0..20 {
            for x in 0..40 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol != ' ' {
                        has_content = true;
                        break;
                    }
                }
            }
        }
        assert!(has_content);
    }

    #[test]
    fn test_histogram_render_custom_bins() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let data: Vec<f64> = (0..100).map(|x| x as f64).collect();
        let hist = Histogram::new(&data).bins(BinConfig::Edges(vec![0.0, 25.0, 50.0, 75.0, 100.0]));
        hist.render(&mut ctx);

        // Should render without panic
        let mut has_content = false;
        for y in 0..20 {
            for x in 0..40 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol != ' ' {
                        has_content = true;
                        break;
                    }
                }
            }
        }
        assert!(has_content);
    }
}
