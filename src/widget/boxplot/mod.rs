//! Box Plot widget for statistical distribution visualization
//!
//! Displays median, quartiles, and outliers for comparing distributions across categories.

use super::chart_common::{Axis, ChartGrid, ChartOrientation, ColorScheme, Legend};
use super::chart_render::{fill_background, render_title};
use super::traits::{RenderContext, View, WidgetProps};
use crate::layout::Rect;
use crate::style::Color;
use crate::{impl_props_builders, impl_styled_view};

mod group;
mod render;
mod types;

#[cfg(test)]
mod tests;

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
