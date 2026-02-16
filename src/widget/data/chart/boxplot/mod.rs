//! Box Plot widget for statistical distribution visualization
//!
//! Displays median, quartiles, and outliers for comparing distributions across categories.

use super::chart_common::{Axis, ChartGrid, ChartOrientation, ColorScheme, Legend};
use super::chart_render::{fill_background, render_title};
use crate::layout::Rect;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

mod group;
mod render;
mod types;

// Public API tests extracted to tests/widget/data/chart_boxplot.rs
// KEEP HERE - Render tests require access to private RenderContext
#[cfg(test)]
mod tests {
    //! Tests for boxplot module

    use super::super::chart_stats::percentile;
    use super::*;

    #[test]
    fn test_percentile() {
        let sorted = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(percentile(&sorted, 0.0), 1.0);
        assert_eq!(percentile(&sorted, 50.0), 3.0);
        assert_eq!(percentile(&sorted, 100.0), 5.0);
    }

    // ========== Render Tests - KEEP HERE (access private RenderContext) ==========

    #[test]
    fn test_boxplot_render_basic() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let data: Vec<f64> = (0..50).map(|x| x as f64).collect();
        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let bp = BoxPlot::new().group("Data", &data);
        bp.render(&mut ctx);

        // Verify box elements are rendered
        let mut has_box = false;
        for y in 0..20 {
            for x in 0..40 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol == '┌'
                        || cell.symbol == '┐'
                        || cell.symbol == '└'
                        || cell.symbol == '┘'
                        || cell.symbol == '│'
                        || cell.symbol == '─'
                    {
                        has_box = true;
                        break;
                    }
                }
            }
        }
        assert!(has_box);
    }

    #[test]
    fn test_boxplot_render_multiple_groups() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(60, 25);
        let area = Rect::new(0, 0, 60, 25);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let bp = BoxPlot::new()
            .group("Group A", &[1.0, 2.0, 3.0, 4.0, 5.0])
            .group("Group B", &[3.0, 4.0, 5.0, 6.0, 7.0])
            .group("Group C", &[5.0, 6.0, 7.0, 8.0, 9.0]);

        bp.render(&mut ctx);

        // Should render without panic
        let mut has_content = false;
        for y in 0..25 {
            for x in 0..60 {
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
    fn test_boxplot_render_with_title() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let bp = BoxPlot::new()
            .title("Test BoxPlot")
            .group("D", &[1.0, 2.0, 3.0, 4.0, 5.0]);

        bp.render(&mut ctx);

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
    fn test_boxplot_render_with_outliers() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Data with outliers
        let mut data: Vec<f64> = (0..20).map(|x| x as f64).collect();
        data.push(100.0); // Outlier
        data.push(-50.0); // Outlier

        let bp = BoxPlot::new()
            .group("Data", &data)
            .show_outliers(true)
            .whisker_style(WhiskerStyle::IQR);

        bp.render(&mut ctx);

        // Should render without panic and show outliers
        let mut has_outliers = false;
        for y in 0..20 {
            for x in 0..40 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol == '○' {
                        has_outliers = true;
                        break;
                    }
                }
            }
        }
        assert!(has_outliers);
    }

    #[test]
    fn test_boxplot_render_horizontal() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let bp = BoxPlot::new()
            .group("Data", &[1.0, 2.0, 3.0, 4.0, 5.0])
            .horizontal();

        bp.render(&mut ctx);

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
    fn test_boxplot_render_small_area() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let bp = BoxPlot::new().group("D", &[1.0, 2.0, 3.0]);

        // Should not panic on small area
        bp.render(&mut ctx);
    }

    #[test]
    fn test_boxplot_render_empty() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(30, 15);
        let area = Rect::new(0, 0, 30, 15);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Empty boxplot
        let bp = BoxPlot::new();
        bp.render(&mut ctx);
    }

    #[test]
    fn test_boxplot_render_notched() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let data: Vec<f64> = (0..30).map(|x| x as f64).collect();
        let bp = BoxPlot::new().group("Data", &data).notched(true);

        bp.render(&mut ctx);

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
    fn test_boxplot_render_minmax_whiskers() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let data: Vec<f64> = (0..20).map(|x| x as f64).collect();
        let bp = BoxPlot::new()
            .group("Data", &data)
            .whisker_style(WhiskerStyle::MinMax);

        bp.render(&mut ctx);

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

pub use group::BoxGroup;
pub use types::{BoxStats, WhiskerStyle};

/// Box plot widget
pub struct BoxPlot {
    /// Box groups
    groups: Vec<BoxGroup>,
    /// Orientation
    orientation: ChartOrientation,
    /// Value axis configuration
    value_axis: Axis,
    /// Category axis configuration
    category_axis: Axis,
    /// Legend configuration
    legend: Legend,
    /// Grid configuration
    grid: ChartGrid,
    /// Color palette
    colors: ColorScheme,
    /// Show outliers
    show_outliers: bool,
    /// Notched box plot
    notched: bool,
    /// Whisker calculation style
    whisker_style: WhiskerStyle,
    /// Box width (0.0-1.0)
    box_width: f64,
    /// Chart title
    title: Option<String>,
    /// Background color
    bg_color: Option<Color>,
    /// Widget properties
    props: WidgetProps,
}

impl Default for BoxPlot {
    fn default() -> Self {
        Self::new()
    }
}

impl BoxPlot {
    /// Create a new box plot
    pub fn new() -> Self {
        Self {
            groups: Vec::new(),
            orientation: ChartOrientation::Vertical,
            value_axis: Axis::default(),
            category_axis: Axis::default(),
            legend: Legend::none(),
            grid: ChartGrid::new().y(true),
            colors: ColorScheme::default_palette(),
            show_outliers: true,
            notched: false,
            whisker_style: WhiskerStyle::IQR,
            box_width: 0.6,
            title: None,
            bg_color: None,
            props: WidgetProps::new(),
        }
    }

    /// Add a group from raw data
    pub fn group(mut self, label: impl Into<String>, data: &[f64]) -> Self {
        self.groups.push(BoxGroup::new(label, data));
        self
    }

    /// Add a group with pre-computed stats
    pub fn group_stats(mut self, label: impl Into<String>, stats: BoxStats) -> Self {
        self.groups.push(BoxGroup::from_stats(label, stats));
        self
    }

    /// Add a box group
    pub fn add_group(mut self, group: BoxGroup) -> Self {
        self.groups.push(group);
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

    /// Set value axis configuration
    pub fn value_axis(mut self, axis: Axis) -> Self {
        self.value_axis = axis;
        self
    }

    /// Set category axis configuration
    pub fn category_axis(mut self, axis: Axis) -> Self {
        self.category_axis = axis;
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

    /// Set color palette
    pub fn colors(mut self, colors: ColorScheme) -> Self {
        self.colors = colors;
        self
    }

    /// Show/hide outliers
    pub fn show_outliers(mut self, show: bool) -> Self {
        self.show_outliers = show;
        self
    }

    /// Enable notched box plot
    pub fn notched(mut self, enabled: bool) -> Self {
        self.notched = enabled;
        self
    }

    /// Set whisker style
    pub fn whisker_style(mut self, style: WhiskerStyle) -> Self {
        self.whisker_style = style;
        self
    }

    /// Set box width (0.0-1.0)
    pub fn box_width(mut self, width: f64) -> Self {
        self.box_width = width.clamp(0.1, 1.0);
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

    /// Compute value bounds
    fn compute_bounds(&self) -> (f64, f64) {
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;

        for group in &self.groups {
            if let Some(stats) = group.get_stats(self.whisker_style) {
                min = min.min(stats.whisker_low);
                max = max.max(stats.whisker_high);
                for &outlier in &stats.outliers {
                    min = min.min(outlier);
                    max = max.max(outlier);
                }
            }
        }

        // Apply axis overrides
        let min = self.value_axis.min.unwrap_or(min);
        let max = self.value_axis.max.unwrap_or(max);

        // Add padding
        let range = (max - min).max(1.0);
        let padding = range * 0.1;

        (min - padding, max + padding)
    }
}

impl View for BoxPlot {
    crate::impl_view_meta!("BoxPlot");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;

        if area.width < 10 || area.height < 8 {
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

        if chart_area.width < 5 || chart_area.height < 5 {
            return;
        }

        let bounds = self.compute_bounds();

        // Render using render module
        let render_state = render::BoxPlotRender::new(
            &self.groups,
            bounds,
            chart_area,
            self.box_width,
            self.whisker_style,
            self.show_outliers,
        );

        render_state.render_boxes(ctx, &self.colors);
        render_state.render_axes(ctx, area, &self.value_axis, &self.category_axis);
    }
}

impl_styled_view!(BoxPlot);
impl_props_builders!(BoxPlot);

/// Create a new box plot
pub fn boxplot() -> BoxPlot {
    BoxPlot::new()
}
