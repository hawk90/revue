/// Orientation for charts (bar, histogram, box plot)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChartOrientation {
    /// Vertical orientation (default for most charts)
    #[default]
    Vertical,
    /// Horizontal orientation
    Horizontal,
}

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
