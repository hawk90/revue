//! Scatter Chart widget for X-Y data visualization
//!
//! Supports scatter plots, bubble charts, multiple series, and trend lines.

mod marks;
mod scale;
mod series;
mod view;

pub use series::ScatterSeries;

use super::chart_common::{Axis, ChartGrid, ColorScheme, Legend, LegendPosition};
use crate::style::Color;
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// Scatter chart widget
#[derive(Clone)]
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
    pub(super) fn series_color(&self, index: usize) -> Color {
        self.series
            .get(index)
            .and_then(|s| s.color)
            .unwrap_or_else(|| self.colors.get(index))
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

// KEEP HERE - Private implementation tests (all tests access private fields: series, data, color, marker, etc.)

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widget::data::chart::chart_common::Marker;

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
}
