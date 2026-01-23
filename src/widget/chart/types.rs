//! Chart types and enums

use super::super::chart_common::Marker;
use crate::style::Color;

/// Chart type
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChartType {
    /// Line chart connecting points
    #[default]
    Line,
    /// Scatter plot (points only)
    Scatter,
    /// Area chart (filled below line)
    Area,
    /// Step chart (horizontal then vertical)
    StepAfter,
    /// Step chart (vertical then horizontal)
    StepBefore,
}

/// Line style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LineStyle {
    /// Solid line
    #[default]
    Solid,
    /// Dashed line
    Dashed,
    /// Dotted line
    Dotted,
    /// No line (for scatter)
    None,
}

/// A data series
#[derive(Clone, Debug)]
pub struct Series {
    /// Series name (for legend)
    pub name: String,
    /// Data points (x, y)
    pub data: Vec<(f64, f64)>,
    /// Chart type for this series
    pub chart_type: ChartType,
    /// Line color
    pub color: Color,
    /// Line style
    pub line_style: LineStyle,
    /// Marker style
    pub marker: Marker,
    /// Fill color (for area charts)
    pub fill_color: Option<Color>,
}

impl Series {
    /// Create a new series
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            data: Vec::new(),
            chart_type: ChartType::Line,
            color: Color::WHITE,
            line_style: LineStyle::Solid,
            marker: Marker::None,
            fill_color: None,
        }
    }

    /// Set data points
    pub fn data(mut self, data: Vec<(f64, f64)>) -> Self {
        self.data = data;
        self
    }

    /// Set data from y values (x = index)
    pub fn data_y(mut self, ys: &[f64]) -> Self {
        self.data = ys.iter().enumerate().map(|(i, &y)| (i as f64, y)).collect();
        self
    }

    /// Set chart type
    pub fn chart_type(mut self, chart_type: ChartType) -> Self {
        self.chart_type = chart_type;
        self
    }

    /// Set color
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set line style
    pub fn line_style(mut self, style: LineStyle) -> Self {
        self.line_style = style;
        self
    }

    /// Set marker
    pub fn marker(mut self, marker: Marker) -> Self {
        self.marker = marker;
        self
    }

    /// Set fill color (for area charts)
    pub fn fill(mut self, color: Color) -> Self {
        self.fill_color = Some(color);
        self.chart_type = ChartType::Area;
        self
    }

    /// Make this a scatter plot
    pub fn scatter(mut self) -> Self {
        self.chart_type = ChartType::Scatter;
        self.line_style = LineStyle::None;
        if matches!(self.marker, Marker::None) {
            self.marker = Marker::Dot;
        }
        self
    }

    /// Make this a line chart
    pub fn line(mut self) -> Self {
        self.chart_type = ChartType::Line;
        self.line_style = LineStyle::Solid;
        self
    }

    /// Make this an area chart
    pub fn area(mut self, fill_color: Color) -> Self {
        self.chart_type = ChartType::Area;
        self.fill_color = Some(fill_color);
        self
    }

    /// Make this a step chart
    pub fn step(mut self) -> Self {
        self.chart_type = ChartType::StepAfter;
        self
    }
}
