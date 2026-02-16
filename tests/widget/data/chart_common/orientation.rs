//! Tests for chart common orientation public APIs

use revue::widget::data::chart::chart_common::ChartOrientation;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chart_orientation_default() {
        let orientation = ChartOrientation::default();
        assert_eq!(orientation, ChartOrientation::Vertical);
    }

    #[test]
    fn test_chart_orientation_all_variants() {
        let _ = ChartOrientation::Vertical;
        let _ = ChartOrientation::Horizontal;
    }

    #[test]
    fn test_chart_orientation_clone() {
        let orientation = ChartOrientation::Horizontal;
        let cloned = orientation;
        assert_eq!(orientation, cloned);
    }

    #[test]
    fn test_chart_orientation_equality() {
        assert_eq!(ChartOrientation::Vertical, ChartOrientation::Vertical);
        assert_eq!(ChartOrientation::Horizontal, ChartOrientation::Horizontal);
        assert_ne!(ChartOrientation::Vertical, ChartOrientation::Horizontal);
    }

    #[test]
    fn test_chart_orientation_copy() {
        let orientation1 = ChartOrientation::Vertical;
        let orientation2 = orientation1;
        assert_eq!(orientation1, orientation2);
    }
}