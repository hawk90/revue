//! Types for the streamline chart widget

use crate::style::Color;

/// A single layer in the stream graph
#[derive(Debug, Clone)]
pub struct StreamLayer {
    /// Name of this layer
    pub name: String,
    /// Data values for each time point
    pub values: Vec<f64>,
    /// Layer color (auto-assigned if None)
    pub color: Option<Color>,
}

impl StreamLayer {
    /// Create a new stream layer
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            values: Vec::new(),
            color: None,
        }
    }

    /// Set data values
    pub fn data(mut self, values: Vec<f64>) -> Self {
        self.values = values;
        self
    }

    /// Set layer color
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}

/// Baseline calculation mode for the stream graph
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StreamBaseline {
    /// Stack from zero (traditional stacked area)
    Zero,
    /// Symmetric around center (classic stream graph)
    #[default]
    Symmetric,
    /// Weighted wiggle minimization
    Wiggle,
    /// Expand to fill height (100% stacked)
    Expand,
}

/// Stream sort order
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StreamOrder {
    /// Keep original order
    #[default]
    None,
    /// Sort by total value (ascending)
    Ascending,
    /// Sort by total value (descending)
    Descending,
    /// Inside-out ordering (largest in middle)
    InsideOut,
}
