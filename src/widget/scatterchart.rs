//! Scatter Chart widget for X-Y data visualization
//!
//! Supports scatter plots, bubble charts, multiple series, and trend lines.

use super::chart_common::{Axis, ChartGrid, ColorScheme, Legend, LegendPosition, Marker};
use super::chart_render::{
    fill_background, render_axis_title, render_grid, render_legend, render_title,
    render_x_axis_labels, render_y_axis_labels, LegendItem,
};
use super::traits::{RenderContext, View, WidgetProps};
use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::{impl_props_builders, impl_styled_view};

/// A data series for scatter chart
#[derive(Clone, Debug)]
pub struct ScatterSeries {
    /// Series name (for legend)
    pub name: String,
    /// Data points (x, y)
    pub data: Vec<(f64, f64)>,
    /// Optional sizes for bubble chart
    pub sizes: Option<Vec<f64>>,
    /// Custom color (uses palette if None)
    pub color: Option<Color>,
    /// Marker style
    pub marker: Marker,
}

impl ScatterSeries {
    /// Create a new scatter series
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            data: Vec::new(),
            sizes: None,
            color: None,
            marker: Marker::FilledCircle,
        }
    }

    /// Set data points
    pub fn data(mut self, data: Vec<(f64, f64)>) -> Self {
        self.data = data;
        self
    }

    /// Set data from slice
    pub fn points(mut self, points: &[(f64, f64)]) -> Self {
        self.data = points.to_vec();
        self
    }

    /// Set sizes for bubble chart
    pub fn sizes(mut self, sizes: Vec<f64>) -> Self {
        self.sizes = Some(sizes);
        self
    }

    /// Set custom color
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Set marker style
    pub fn marker(mut self, marker: Marker) -> Self {
        self.marker = marker;
        self
    }
}

/// Scatter chart widget
pub struct ScatterChart {
    /// Data series
    series: Vec<ScatterSeries>,
    /// X axis configuration
    x_axis: Axis,
    /// Y axis configuration
    y_axis: Axis,
    /// Legend configuration
    legend: Legend,
    /// Grid configuration
    grid: ChartGrid,
    /// Color palette
    colors: ColorScheme,
    /// Chart title
    title: Option<String>,
    /// Background color
    bg_color: Option<Color>,
    /// Border color
    border_color: Option<Color>,
    /// Widget properties
    props: WidgetProps,
}

impl Default for ScatterChart {
    fn default() -> Self {
        Self::new()
    }
}

impl ScatterChart {
    /// Create a new scatter chart
    pub fn new() -> Self {
        Self {
            series: Vec::new(),
            x_axis: Axis::default(),
            y_axis: Axis::default(),
            legend: Legend::new().position(LegendPosition::TopRight),
            grid: ChartGrid::new().x(true).y(true),
            colors: ColorScheme::default_palette(),
            title: None,
            bg_color: None,
            border_color: None,
            props: WidgetProps::new(),
        }
    }

    /// Add a series
    pub fn series(mut self, series: ScatterSeries) -> Self {
        self.series.push(series);
        self
    }

    /// Add multiple series
    pub fn series_vec(mut self, series: Vec<ScatterSeries>) -> Self {
        self.series.extend(series);
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

    /// Hide the legend
    pub fn no_legend(mut self) -> Self {
        self.legend = Legend::none();
        self
    }

    /// Set grid configuration
    pub fn grid(mut self, grid: ChartGrid) -> Self {
        self.grid = grid;
        self
    }

    /// Set color palette
    pub fn colors(mut self, colors: ColorScheme) -> Self {
        self.colors = colors;
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

    /// Set border color
    pub fn border(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }

    /// Get color for series at index
    fn series_color(&self, index: usize) -> Color {
        self.series
            .get(index)
            .and_then(|s| s.color)
            .unwrap_or_else(|| self.colors.get(index))
    }

    /// Compute data bounds
    fn compute_bounds(&self) -> (f64, f64, f64, f64) {
        let mut x_min = f64::INFINITY;
        let mut x_max = f64::NEG_INFINITY;
        let mut y_min = f64::INFINITY;
        let mut y_max = f64::NEG_INFINITY;

        for series in &self.series {
            for &(x, y) in &series.data {
                if x.is_finite() {
                    x_min = x_min.min(x);
                    x_max = x_max.max(x);
                }
                if y.is_finite() {
                    y_min = y_min.min(y);
                    y_max = y_max.max(y);
                }
            }
        }

        // Apply axis overrides
        let x_min = self.x_axis.min.unwrap_or(x_min);
        let x_max = self.x_axis.max.unwrap_or(x_max);
        let y_min = self.y_axis.min.unwrap_or(y_min);
        let y_max = self.y_axis.max.unwrap_or(y_max);

        // Add padding
        let x_range = (x_max - x_min).max(1.0);
        let y_range = (y_max - y_min).max(1.0);
        let x_padding = x_range * 0.05;
        let y_padding = y_range * 0.05;

        (
            x_min - x_padding,
            x_max + x_padding,
            y_min - y_padding,
            y_max + y_padding,
        )
    }

    /// Map data coordinates to screen coordinates
    fn map_to_screen(
        &self,
        x: f64,
        y: f64,
        bounds: (f64, f64, f64, f64),
        chart_area: Rect,
    ) -> (u16, u16) {
        let (x_min, x_max, y_min, y_max) = bounds;

        let x_range = (x_max - x_min).max(1.0);
        let y_range = (y_max - y_min).max(1.0);

        let screen_x =
            chart_area.x + ((x - x_min) / x_range * (chart_area.width as f64 - 1.0)) as u16;
        let screen_y = chart_area.y + chart_area.height
            - 1
            - ((y - y_min) / y_range * (chart_area.height as f64 - 1.0)) as u16;

        (
            screen_x.clamp(chart_area.x, chart_area.x + chart_area.width - 1),
            screen_y.clamp(chart_area.y, chart_area.y + chart_area.height - 1),
        )
    }

    /// Render data points
    fn render_points(
        &self,
        ctx: &mut RenderContext,
        chart_area: Rect,
        bounds: (f64, f64, f64, f64),
    ) {
        for (series_idx, series) in self.series.iter().enumerate() {
            let color = self.series_color(series_idx);
            let marker_char = series.marker.char();

            for (point_idx, &(x, y)) in series.data.iter().enumerate() {
                if !x.is_finite() || !y.is_finite() {
                    continue;
                }

                let (screen_x, screen_y) = self.map_to_screen(x, y, bounds, chart_area);

                // For bubble chart, draw larger markers based on size
                if let Some(ref sizes) = series.sizes {
                    if let Some(&size) = sizes.get(point_idx) {
                        // Draw a circle proportional to size
                        let radius = ((size / 100.0).sqrt() * 2.0).max(1.0) as u16;
                        for dy in 0..radius {
                            for dx in 0..radius {
                                let bx = screen_x.saturating_sub(radius / 2) + dx;
                                let by = screen_y.saturating_sub(radius / 2) + dy;
                                if bx >= chart_area.x
                                    && bx < chart_area.x + chart_area.width
                                    && by >= chart_area.y
                                    && by < chart_area.y + chart_area.height
                                {
                                    let mut cell = Cell::new('●');
                                    cell.fg = Some(color);
                                    ctx.buffer.set(bx, by, cell);
                                }
                            }
                        }
                        continue;
                    }
                }

                // Regular marker
                if screen_x >= chart_area.x
                    && screen_x < chart_area.x + chart_area.width
                    && screen_y >= chart_area.y
                    && screen_y < chart_area.y + chart_area.height
                {
                    let mut cell = Cell::new(marker_char);
                    cell.fg = Some(color);
                    ctx.buffer.set(screen_x, screen_y, cell);
                }
            }
        }
    }
}

impl View for ScatterChart {
    crate::impl_view_meta!("ScatterChart");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;

        if area.width < 15 || area.height < 8 {
            return;
        }

        // Fill background if set
        if let Some(bg) = self.bg_color {
            fill_background(ctx, area, bg);
        }

        // Draw title using shared function
        let title_offset = render_title(ctx, area, self.title.as_deref(), Color::WHITE);

        // Calculate chart area (leave room for axes)
        let y_label_width = 8u16;
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

        let bounds = self.compute_bounds();
        let (x_min, x_max, y_min, y_max) = bounds;

        // Render components using shared functions
        render_grid(
            ctx,
            chart_area,
            &self.grid,
            self.x_axis.ticks,
            self.y_axis.ticks,
        );
        self.render_points(ctx, chart_area, bounds);

        // Render axes
        render_y_axis_labels(ctx, area, &self.y_axis, y_min, y_max, y_label_width);
        render_x_axis_labels(
            ctx,
            area,
            &self.x_axis,
            x_min,
            x_max,
            title_offset,
            y_label_width,
        );
        render_axis_title(
            ctx,
            area,
            self.x_axis.title.as_deref(),
            self.x_axis.color,
            true,
        );

        // Render legend using shared function
        let legend_items: Vec<LegendItem<'_>> = self
            .series
            .iter()
            .enumerate()
            .map(|(i, s)| LegendItem {
                label: &s.name,
                color: self.series_color(i),
            })
            .collect();
        render_legend(ctx, area, &self.legend, &legend_items);
    }
}

impl_styled_view!(ScatterChart);
impl_props_builders!(ScatterChart);

/// Create a new scatter chart
pub fn scatter_chart() -> ScatterChart {
    ScatterChart::new()
}

/// Create a bubble chart (scatter chart with sizes)
pub fn bubble_chart() -> ScatterChart {
    ScatterChart::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scatter_chart_new() {
        let chart = ScatterChart::new();
        assert!(chart.series.is_empty());
    }

    #[test]
    fn test_scatter_series() {
        let series = ScatterSeries::new("Test")
            .points(&[(1.0, 2.0), (3.0, 4.0), (5.0, 6.0)])
            .color(Color::RED)
            .marker(Marker::Star);

        assert_eq!(series.name, "Test");
        assert_eq!(series.data.len(), 3);
        assert_eq!(series.color, Some(Color::RED));
        assert_eq!(series.marker, Marker::Star);
    }

    #[test]
    fn test_scatter_chart_series() {
        let chart = ScatterChart::new()
            .series(ScatterSeries::new("A").points(&[(1.0, 1.0)]))
            .series(ScatterSeries::new("B").points(&[(2.0, 2.0)]));

        assert_eq!(chart.series.len(), 2);
    }

    #[test]
    fn test_scatter_chart_bounds() {
        let chart = ScatterChart::new()
            .series(ScatterSeries::new("Test").points(&[(0.0, 0.0), (10.0, 20.0)]));

        let (x_min, x_max, y_min, y_max) = chart.compute_bounds();
        assert!(x_min < 0.0);
        assert!(x_max > 10.0);
        assert!(y_min < 0.0);
        assert!(y_max > 20.0);
    }

    #[test]
    fn test_scatter_chart_axis_override() {
        let chart = ScatterChart::new()
            .series(ScatterSeries::new("Test").points(&[(5.0, 5.0)]))
            .x_axis(Axis::new().bounds(0.0, 100.0))
            .y_axis(Axis::new().bounds(0.0, 50.0));

        let (x_min, x_max, y_min, y_max) = chart.compute_bounds();
        assert!(x_min <= 0.0);
        assert!(x_max >= 100.0);
        assert!(y_min <= 0.0);
        assert!(y_max >= 50.0);
    }

    #[test]
    fn test_bubble_chart() {
        let series = ScatterSeries::new("Bubbles")
            .points(&[(1.0, 1.0), (2.0, 2.0)])
            .sizes(vec![10.0, 50.0]);

        assert!(series.sizes.is_some());
        assert_eq!(series.sizes.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_scatter_chart_legend() {
        let chart = ScatterChart::new().legend(Legend::bottom_left());
        assert_eq!(chart.legend.position, LegendPosition::BottomLeft);

        let chart = ScatterChart::new().no_legend();
        assert!(!chart.legend.is_visible());
    }

    #[test]
    fn test_scatter_chart_grid() {
        let chart = ScatterChart::new().grid(ChartGrid::y_only());
        assert!(!chart.grid.x);
        assert!(chart.grid.y);
    }

    #[test]
    fn test_scatter_chart_builder() {
        let chart = ScatterChart::new()
            .title("Scatter Plot")
            .series(ScatterSeries::new("Data").points(&[(1.0, 1.0)]))
            .x_axis(Axis::new().title("X"))
            .y_axis(Axis::new().title("Y"))
            .legend(Legend::top_right())
            .grid(ChartGrid::both());

        assert_eq!(chart.title, Some("Scatter Plot".to_string()));
        assert_eq!(chart.series.len(), 1);
        assert!(chart.grid.x);
        assert!(chart.grid.y);
    }

    #[test]
    fn test_scatter_helpers() {
        let chart = scatter_chart();
        assert!(chart.series.is_empty());

        let chart = bubble_chart();
        assert!(chart.series.is_empty());
    }

    // ========== Render Tests ==========

    #[test]
    fn test_scatter_chart_render_basic() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let chart = ScatterChart::new().series(ScatterSeries::new("Data").points(&[
            (1.0, 2.0),
            (3.0, 4.0),
            (5.0, 3.0),
            (7.0, 6.0),
        ]));

        chart.render(&mut ctx);

        // Verify something was rendered
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
    fn test_scatter_chart_render_with_title() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let chart = ScatterChart::new()
            .title("My Scatter")
            .series(ScatterSeries::new("D").points(&[(1.0, 1.0)]));

        chart.render(&mut ctx);

        // Title should be rendered
        let mut title_found = false;
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, 0) {
                if cell.symbol == 'M' {
                    title_found = true;
                    break;
                }
            }
        }
        assert!(title_found);
    }

    #[test]
    fn test_scatter_chart_render_multiple_series() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(50, 25);
        let area = Rect::new(0, 0, 50, 25);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let chart = ScatterChart::new()
            .series(ScatterSeries::new("Series A").points(&[(1.0, 1.0), (2.0, 2.0)]))
            .series(ScatterSeries::new("Series B").points(&[(3.0, 3.0), (4.0, 4.0)]))
            .legend(Legend::top_right());

        chart.render(&mut ctx);

        // Should render without panic
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
    fn test_scatter_chart_render_with_grid() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let chart = ScatterChart::new()
            .series(ScatterSeries::new("D").points(&[(5.0, 5.0)]))
            .grid(ChartGrid::both());

        chart.render(&mut ctx);

        // Look for grid characters
        let mut grid_found = false;
        for y in 0..20 {
            for x in 0..40 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol == '─' || cell.symbol == '│' || cell.symbol == '┼' {
                        grid_found = true;
                        break;
                    }
                }
            }
        }
        assert!(grid_found);
    }

    #[test]
    fn test_scatter_chart_render_small_area() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let chart = ScatterChart::new().series(ScatterSeries::new("D").points(&[(1.0, 1.0)]));

        // Should not panic on small area
        chart.render(&mut ctx);
    }

    #[test]
    fn test_scatter_chart_render_empty() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(30, 15);
        let area = Rect::new(0, 0, 30, 15);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Empty chart
        let chart = ScatterChart::new();
        chart.render(&mut ctx);
    }

    #[test]
    fn test_scatter_chart_render_with_markers() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let chart = ScatterChart::new().series(
            ScatterSeries::new("Stars")
                .points(&[(5.0, 5.0), (10.0, 10.0)])
                .marker(Marker::Star),
        );

        chart.render(&mut ctx);

        // Look for star markers
        let mut marker_found = false;
        for y in 0..20 {
            for x in 0..40 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol == '★' {
                        marker_found = true;
                        break;
                    }
                }
            }
        }
        assert!(marker_found);
    }

    #[test]
    fn test_scatter_chart_render_bubble() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let chart = bubble_chart().series(
            ScatterSeries::new("Bubbles")
                .points(&[(5.0, 5.0), (10.0, 10.0)])
                .sizes(vec![1.0, 5.0]),
        );

        chart.render(&mut ctx);

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
