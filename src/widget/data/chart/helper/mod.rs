//! Helper functions for chart widget

mod geometry;
mod render;

use super::chart_common::{Axis, LegendPosition};
use super::types::Series;
use crate::style::Color;
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// Chart widget
#[derive(Debug, Clone)]
pub struct Chart {
    /// Chart title
    title: Option<String>,
    /// Data series
    series: Vec<Series>,
    /// X axis
    x_axis: Axis,
    /// Y axis
    y_axis: Axis,
    /// Legend position
    legend: LegendPosition,
    /// Background color
    bg_color: Option<Color>,
    /// Border color
    border_color: Option<Color>,
    /// Use Braille for higher resolution
    braille_mode: bool,
    /// Tooltip configuration
    tooltip: Option<super::chart_common::ChartTooltip>,
    /// Widget properties
    props: WidgetProps,
}

impl Chart {
    /// Create a new chart
    pub fn new() -> Self {
        Self {
            title: None,
            series: Vec::new(),
            x_axis: Axis::default(),
            y_axis: Axis::default(),
            legend: LegendPosition::TopRight,
            bg_color: None,
            border_color: None,
            braille_mode: false,
            tooltip: None,
            props: WidgetProps::new(),
        }
    }

    /// Set chart title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Add a series
    pub fn series(mut self, series: Series) -> Self {
        self.series.push(series);
        self
    }

    /// Add multiple series
    pub fn series_vec(mut self, series: Vec<Series>) -> Self {
        self.series.extend(series);
        self
    }

    /// Set X axis
    pub fn x_axis(mut self, axis: Axis) -> Self {
        self.x_axis = axis;
        self
    }

    /// Set Y axis
    pub fn y_axis(mut self, axis: Axis) -> Self {
        self.y_axis = axis;
        self
    }

    /// Set legend position
    pub fn legend(mut self, position: LegendPosition) -> Self {
        self.legend = position;
        self
    }

    /// Hide legend
    pub fn no_legend(mut self) -> Self {
        self.legend = LegendPosition::None;
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

    /// Enable Braille mode for higher resolution
    ///
    /// Series lines (line, area outline and step charts) are drawn with
    /// Braille dots, 2x4 per cell, instead of box-drawing characters.
    /// Markers, area fills, axes and labels still use whole cells.
    pub fn braille(mut self) -> Self {
        self.braille_mode = true;
        self
    }

    /// Set tooltip configuration
    pub fn tooltip(mut self, tooltip: super::chart_common::ChartTooltip) -> Self {
        self.tooltip = Some(tooltip);
        self
    }

    /// Enable tooltips with default settings
    pub fn with_tooltip(mut self) -> Self {
        self.tooltip = Some(super::chart_common::ChartTooltip::enabled());
        self
    }
}

impl Default for Chart {
    fn default() -> Self {
        Self::new()
    }
}

impl_styled_view!(Chart);
impl_props_builders!(Chart);

/// Helper function to create a chart
pub fn chart() -> Chart {
    Chart::new()
}

/// Quick line chart from data
pub fn line_chart(data: &[f64]) -> Chart {
    Chart::new().series(Series::new("Data").data_y(data).line())
}

/// Quick scatter plot from data
pub fn scatter_plot(data: &[(f64, f64)]) -> Chart {
    Chart::new().series(Series::new("Data").data(data.to_vec()).scatter())
}
