//! Box plot group data

use super::types::{BoxStats, WhiskerStyle};
use crate::style::Color;

/// A box plot group
#[derive(Clone, Debug)]
pub struct BoxGroup {
    /// Group label
    pub label: String,
    /// Raw data (stats will be computed)
    pub data: Vec<f64>,
    /// Pre-computed statistics (optional)
    pub stats: Option<BoxStats>,
    /// Custom color
    pub color: Option<Color>,
}

impl BoxGroup {
    /// Create a group from raw data
    pub fn new(label: impl Into<String>, data: &[f64]) -> Self {
        Self {
            label: label.into(),
            data: data.to_vec(),
            stats: None,
            color: None,
        }
    }

    /// Create a group with pre-computed stats
    pub fn from_stats(label: impl Into<String>, stats: BoxStats) -> Self {
        Self {
            label: label.into(),
            data: Vec::new(),
            stats: Some(stats),
            color: None,
        }
    }

    /// Set custom color
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Get statistics (compute if necessary)
    pub fn get_stats(&self, whisker_style: WhiskerStyle) -> Option<BoxStats> {
        self.stats
            .clone()
            .or_else(|| BoxStats::from_data(&self.data, whisker_style))
    }
}
