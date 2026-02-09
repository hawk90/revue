//! Box plot types and statistics

use super::super::chart_stats::percentile;

/// Whisker calculation style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WhiskerStyle {
    /// 1.5 * IQR (default)
    #[default]
    IQR,
    /// Min and max values
    MinMax,
    /// Percentile (5th and 95th)
    Percentile,
}

/// Pre-computed box plot statistics
#[derive(Clone, Debug)]
pub struct BoxStats {
    /// Minimum value
    pub min: f64,
    /// First quartile (25th percentile)
    pub q1: f64,
    /// Median (50th percentile)
    pub median: f64,
    /// Third quartile (75th percentile)
    pub q3: f64,
    /// Maximum value
    pub max: f64,
    /// Outlier values
    pub outliers: Vec<f64>,
    /// Lower whisker value
    pub whisker_low: f64,
    /// Upper whisker value
    pub whisker_high: f64,
}

impl BoxStats {
    /// Compute statistics from raw data
    pub fn from_data(data: &[f64], whisker_style: WhiskerStyle) -> Option<Self> {
        let mut valid: Vec<f64> = data.iter().filter(|x| x.is_finite()).copied().collect();
        if valid.is_empty() {
            return None;
        }

        valid.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let n = valid.len();
        let min = valid[0];
        let max = valid[n - 1];

        let median = if n.is_multiple_of(2) {
            (valid[n / 2 - 1] + valid[n / 2]) / 2.0
        } else {
            valid[n / 2]
        };

        let q1 = percentile(&valid, 25.0);
        let q3 = percentile(&valid, 75.0);
        let iqr = q3 - q1;

        let (whisker_low, whisker_high, outliers) = match whisker_style {
            WhiskerStyle::IQR => {
                let lower_fence = q1 - 1.5 * iqr;
                let upper_fence = q3 + 1.5 * iqr;
                let whisker_low = valid
                    .iter()
                    .find(|&&x| x >= lower_fence)
                    .copied()
                    .unwrap_or(min);
                let whisker_high = valid
                    .iter()
                    .rev()
                    .find(|&&x| x <= upper_fence)
                    .copied()
                    .unwrap_or(max);
                let outliers: Vec<f64> = valid
                    .iter()
                    .filter(|&&x| x < lower_fence || x > upper_fence)
                    .copied()
                    .collect();
                (whisker_low, whisker_high, outliers)
            }
            WhiskerStyle::MinMax => (min, max, Vec::new()),
            WhiskerStyle::Percentile => {
                let p5 = percentile(&valid, 5.0);
                let p95 = percentile(&valid, 95.0);
                let outliers: Vec<f64> = valid
                    .iter()
                    .filter(|&&x| x < p5 || x > p95)
                    .copied()
                    .collect();
                (p5, p95, outliers)
            }
        };

        Some(BoxStats {
            min,
            q1,
            median,
            q3,
            max,
            outliers,
            whisker_low,
            whisker_high,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whisker_style_default() {
        let style = WhiskerStyle::default();
        assert_eq!(style, WhiskerStyle::IQR);
    }

    #[test]
    fn test_whisker_style_variants() {
        assert_eq!(WhiskerStyle::IQR, WhiskerStyle::IQR);
        assert_eq!(WhiskerStyle::MinMax, WhiskerStyle::MinMax);
        assert_eq!(WhiskerStyle::Percentile, WhiskerStyle::Percentile);
    }

    #[test]
    fn test_box_stats_from_data_empty() {
        let result = BoxStats::from_data(&[], WhiskerStyle::IQR);
        assert!(result.is_none());
    }

    #[test]
    fn test_box_stats_from_data_single_value() {
        let data = vec![5.0];
        let result = BoxStats::from_data(&data, WhiskerStyle::IQR);
        assert!(result.is_some());
        let stats = result.unwrap();
        assert_eq!(stats.min, 5.0);
        assert_eq!(stats.max, 5.0);
        assert_eq!(stats.median, 5.0);
    }

    #[test]
    fn test_box_stats_from_data_multiple_values() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = BoxStats::from_data(&data, WhiskerStyle::IQR);
        assert!(result.is_some());
        let stats = result.unwrap();
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 5.0);
        assert_eq!(stats.median, 3.0);
    }

    #[test]
    fn test_box_stats_from_data_with_nan() {
        let data = vec![1.0, f64::NAN, 3.0, f64::INFINITY, 5.0];
        let result = BoxStats::from_data(&data, WhiskerStyle::IQR);
        assert!(result.is_some());
        let stats = result.unwrap();
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 5.0);
        assert_eq!(stats.median, 3.0);
    }

    #[test]
    fn test_box_stats_public_fields() {
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
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.q1, 2.0);
        assert_eq!(stats.median, 3.0);
        assert_eq!(stats.q3, 4.0);
        assert_eq!(stats.max, 5.0);
        assert!(stats.outliers.is_empty());
        assert_eq!(stats.whisker_low, 1.0);
        assert_eq!(stats.whisker_high, 5.0);
    }

    #[test]
    fn test_box_stats_from_data_whisker_style_iqr() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = BoxStats::from_data(&data, WhiskerStyle::IQR);
        assert!(result.is_some());
        let stats = result.unwrap();
        assert!(stats.outliers.is_empty());
    }

    #[test]
    fn test_box_stats_from_data_whisker_style_minmax() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = BoxStats::from_data(&data, WhiskerStyle::MinMax);
        assert!(result.is_some());
        let stats = result.unwrap();
        assert_eq!(stats.whisker_low, 1.0);
        assert_eq!(stats.whisker_high, 5.0);
        assert!(stats.outliers.is_empty());
    }

    #[test]
    fn test_box_stats_from_data_whisker_style_percentile() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = BoxStats::from_data(&data, WhiskerStyle::Percentile);
        assert!(result.is_some());
        let stats = result.unwrap();
        // Percentile style may or may not have outliers depending on the data
        // Just verify we got some values
        assert!(stats.min <= stats.whisker_low);
        assert!(stats.max >= stats.whisker_high);
    }

    #[test]
    fn test_box_stats_from_data_with_outliers() {
        // Data with potential outliers
        let data = vec![1.0, 2.0, 3.0, 4.0, 100.0];
        let result = BoxStats::from_data(&data, WhiskerStyle::IQR);
        assert!(result.is_some());
        let stats = result.unwrap();
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 100.0);
        assert_eq!(stats.median, 3.0);
    }
}
