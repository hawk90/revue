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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_group_new() {
        let group = BoxGroup::new("test", &[1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(group.label, "test");
        assert_eq!(group.data, vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        assert!(group.stats.is_none());
        assert!(group.color.is_none());
    }

    #[test]
    fn test_box_group_from_stats() {
        let stats = BoxStats {
            min: 1.0,
            q1: 2.0,
            median: 3.0,
            q3: 4.0,
            max: 5.0,
            outliers: vec![],
            whisker_low: 1.0,
            whisker_high: 5.0,
        };
        let group = BoxGroup::from_stats("test", stats.clone());
        assert_eq!(group.label, "test");
        assert!(group.data.is_empty());
        assert!(group.stats.is_some());
        assert!(group.color.is_none());
        // Can't compare stats directly due to no PartialEq, but we verified Some(stats)
    }

    #[test]
    fn test_box_group_color() {
        let group = BoxGroup::new("test", &[1.0, 2.0]).color(crate::style::Color::RED);
        assert_eq!(group.color, Some(crate::style::Color::RED));
    }

    #[test]
    fn test_box_group_get_stats_with_precomputed() {
        let stats = BoxStats {
            min: 1.0,
            q1: 2.0,
            median: 3.0,
            q3: 4.0,
            max: 5.0,
            outliers: vec![],
            whisker_low: 1.0,
            whisker_high: 5.0,
        };
        let group = BoxGroup::from_stats("test", stats.clone());
        let result = group.get_stats(WhiskerStyle::MinMax);
        assert!(result.is_some());
        // Can't compare BoxStats directly as it doesn't derive PartialEq
        let result_stats = result.unwrap();
        assert_eq!(result_stats.min, stats.min);
        assert_eq!(result_stats.median, stats.median);
    }

    #[test]
    fn test_box_group_get_stats_compute() {
        let group = BoxGroup::new("test", &[1.0, 2.0, 3.0, 4.0, 5.0]);
        let result = group.get_stats(WhiskerStyle::MinMax);
        assert!(result.is_some());
        let stats = result.unwrap();
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 5.0);
        assert_eq!(stats.median, 3.0);
    }

    #[test]
    fn test_box_group_public_fields() {
        let mut group = BoxGroup::new("test", &[1.0, 2.0]);
        group.label = "modified".to_string();
        group.data.push(3.0);
        group.stats = None;
        group.color = Some(crate::style::Color::BLUE);

        assert_eq!(group.label, "modified");
        assert_eq!(group.data, vec![1.0, 2.0, 3.0]);
        assert!(group.stats.is_none());
        assert_eq!(group.color, Some(crate::style::Color::BLUE));
    }

    #[test]
    fn test_box_group_empty_data() {
        let group = BoxGroup::new("test", &[]);
        assert_eq!(group.data, Vec::<f64>::new());
        assert!(group.stats.is_none());
    }

    #[test]
    fn test_box_group_new_from_slice() {
        let data = vec![10.0, 20.0, 30.0];
        let group = BoxGroup::new("slice", &data);
        assert_eq!(group.data, data);
    }
}
