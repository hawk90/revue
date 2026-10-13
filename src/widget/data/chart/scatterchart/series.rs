//! ScatterChart data: one named series of points

use crate::style::Color;
use crate::widget::data::chart::chart_common::Marker;

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
