//! Statistical functions for chart widgets
//!
//! Common statistical calculations used by Histogram and BoxPlot widgets.

// Allow dead code for utility functions that are not yet used but available for future use
#![allow(dead_code)]

/// Filter non-finite values from data
pub fn filter_finite(data: &[f64]) -> Vec<f64> {
    data.iter().filter(|x| x.is_finite()).copied().collect()
}

/// Calculate percentile from sorted data
///
/// Uses linear interpolation between data points.
/// `p` should be in range 0-100.
pub fn percentile(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let k = (p / 100.0) * (sorted.len() - 1) as f64;
    let lower = k.floor() as usize;
    let upper = k.ceil() as usize;
    let weight = k - lower as f64;

    if upper >= sorted.len() {
        sorted[sorted.len() - 1]
    } else {
        sorted[lower] * (1.0 - weight) + sorted[upper] * weight
    }
}

/// Calculate mean of data
pub fn mean(data: &[f64]) -> Option<f64> {
    let valid = filter_finite(data);
    if valid.is_empty() {
        return None;
    }
    Some(valid.iter().sum::<f64>() / valid.len() as f64)
}

/// Calculate median of data
pub fn median(data: &[f64]) -> Option<f64> {
    let mut valid = filter_finite(data);
    if valid.is_empty() {
        return None;
    }
    valid.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = valid.len() / 2;
    if valid.len().is_multiple_of(2) {
        Some((valid[mid - 1] + valid[mid]) / 2.0)
    } else {
        Some(valid[mid])
    }
}

/// Calculate quartiles (Q1, median, Q3) from data
pub fn quartiles(data: &[f64]) -> Option<(f64, f64, f64)> {
    let mut valid = filter_finite(data);
    if valid.is_empty() {
        return None;
    }
    valid.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let q1 = percentile(&valid, 25.0);
    let med = percentile(&valid, 50.0);
    let q3 = percentile(&valid, 75.0);

    Some((q1, med, q3))
}

/// Calculate min and max of data
pub fn min_max(data: &[f64]) -> Option<(f64, f64)> {
    let valid = filter_finite(data);
    if valid.is_empty() {
        return None;
    }
    let min = valid.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = valid.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    Some((min, max))
}

/// Calculate interquartile range (IQR = Q3 - Q1)
pub fn iqr(data: &[f64]) -> Option<f64> {
    quartiles(data).map(|(q1, _, q3)| q3 - q1)
}

/// Detect outliers using IQR method (1.5 * IQR)
pub fn outliers_iqr(data: &[f64]) -> Vec<f64> {
    let mut valid = filter_finite(data);
    if valid.is_empty() {
        return Vec::new();
    }
    valid.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let q1 = percentile(&valid, 25.0);
    let q3 = percentile(&valid, 75.0);
    let iqr = q3 - q1;
    let lower_fence = q1 - 1.5 * iqr;
    let upper_fence = q3 + 1.5 * iqr;

    valid
        .into_iter()
        .filter(|&x| x < lower_fence || x > upper_fence)
        .collect()
}

/// Bin configuration for histograms
#[derive(Clone, Debug, Default)]
pub enum BinConfig {
    /// Automatic binning (Sturges' rule)
    #[default]
    Auto,
    /// Fixed number of bins
    Count(usize),
    /// Fixed bin width
    Width(f64),
    /// Custom bin edges
    Edges(Vec<f64>),
}

/// A single bin in a histogram
#[derive(Clone, Debug)]
pub struct HistogramBin {
    /// Start value (inclusive)
    pub start: f64,
    /// End value (exclusive, except for last bin)
    pub end: f64,
    /// Count of values in this bin
    pub count: usize,
    /// Frequency (count / total)
    pub frequency: f64,
    /// Density (frequency / bin_width)
    pub density: f64,
}

/// Compute histogram bins from data
pub fn compute_bins(data: &[f64], config: &BinConfig) -> Vec<HistogramBin> {
    let valid_data = filter_finite(data);
    if valid_data.is_empty() {
        return Vec::new();
    }

    let min = valid_data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = valid_data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = (max - min).max(1.0);

    // Determine bin edges
    let edges = match config {
        BinConfig::Auto => {
            // Sturges' rule
            let n = valid_data.len();
            let bin_count = ((n as f64).log2() + 1.0).ceil() as usize;
            let bin_count = bin_count.clamp(1, 100);
            let bin_width = range / bin_count as f64;
            (0..=bin_count)
                .map(|i| min + i as f64 * bin_width)
                .collect::<Vec<_>>()
        }
        BinConfig::Count(n) => {
            let bin_count = (*n).max(1);
            let bin_width = range / bin_count as f64;
            (0..=bin_count)
                .map(|i| min + i as f64 * bin_width)
                .collect::<Vec<_>>()
        }
        BinConfig::Width(w) => {
            let bin_width = (*w).max(0.001);
            let bin_count = (range / bin_width).ceil() as usize;
            (0..=bin_count)
                .map(|i| min + i as f64 * bin_width)
                .collect::<Vec<_>>()
        }
        BinConfig::Edges(edges) => edges.clone(),
    };

    // Count values in each bin
    let total = valid_data.len();
    let mut bins = Vec::new();

    for i in 0..edges.len().saturating_sub(1) {
        let start = edges[i];
        let end = edges[i + 1];
        let count = valid_data
            .iter()
            .filter(|&&x| {
                if i == edges.len() - 2 {
                    x >= start && x <= end // Include last edge
                } else {
                    x >= start && x < end
                }
            })
            .count();

        let frequency = count as f64 / total as f64;
        let bin_width = end - start;
        let density = if bin_width > 0.0 {
            frequency / bin_width
        } else {
            0.0
        };

        bins.push(HistogramBin {
            start,
            end,
            count,
            frequency,
            density,
        });
    }

    bins
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_finite() {
        let data = vec![1.0, f64::NAN, 2.0, f64::INFINITY, 3.0];
        let filtered = filter_finite(&data);
        assert_eq!(filtered, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_percentile() {
        let sorted = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(percentile(&sorted, 0.0), 1.0);
        assert_eq!(percentile(&sorted, 50.0), 3.0);
        assert_eq!(percentile(&sorted, 100.0), 5.0);
    }

    #[test]
    fn test_percentile_empty() {
        let sorted: Vec<f64> = vec![];
        assert_eq!(percentile(&sorted, 50.0), 0.0);
    }

    #[test]
    fn test_mean() {
        assert_eq!(mean(&[1.0, 2.0, 3.0, 4.0, 5.0]), Some(3.0));
        assert_eq!(mean(&[]), None);
        assert_eq!(mean(&[f64::NAN]), None);
    }

    #[test]
    fn test_median_odd() {
        assert_eq!(median(&[1.0, 2.0, 3.0, 4.0, 5.0]), Some(3.0));
    }

    #[test]
    fn test_median_even() {
        assert_eq!(median(&[1.0, 2.0, 3.0, 4.0]), Some(2.5));
    }

    #[test]
    fn test_median_empty() {
        assert_eq!(median(&[]), None);
    }

    #[test]
    fn test_quartiles() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let (q1, med, q3) = quartiles(&data).unwrap();
        // Linear interpolation: k = p/100 * (n-1), where n=9
        // Q1: k = 0.25 * 8 = 2.0 -> sorted[2] = 3.0
        // Median: k = 0.5 * 8 = 4.0 -> sorted[4] = 5.0
        // Q3: k = 0.75 * 8 = 6.0 -> sorted[6] = 7.0
        assert!((q1 - 3.0).abs() < 0.1);
        assert!((med - 5.0).abs() < 0.1);
        assert!((q3 - 7.0).abs() < 0.1);
    }

    #[test]
    fn test_min_max() {
        assert_eq!(min_max(&[3.0, 1.0, 4.0, 1.0, 5.0]), Some((1.0, 5.0)));
        assert_eq!(min_max(&[]), None);
    }

    #[test]
    fn test_iqr() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let iqr_val = iqr(&data).unwrap();
        assert!(iqr_val > 0.0);
    }

    #[test]
    fn test_outliers_iqr() {
        let mut data: Vec<f64> = (0..20).map(|x| x as f64).collect();
        data.push(100.0); // Outlier
        data.push(-50.0); // Outlier

        let outliers = outliers_iqr(&data);
        assert!(outliers.contains(&100.0));
        assert!(outliers.contains(&-50.0));
    }

    #[test]
    fn test_compute_bins_auto() {
        let data: Vec<f64> = (0..100).map(|x| x as f64).collect();
        let bins = compute_bins(&data, &BinConfig::Auto);
        assert!(!bins.is_empty());
        assert!(bins.len() <= 100);
    }

    #[test]
    fn test_compute_bins_count() {
        let data: Vec<f64> = (0..100).map(|x| x as f64).collect();
        let bins = compute_bins(&data, &BinConfig::Count(10));
        assert_eq!(bins.len(), 10);
    }

    #[test]
    fn test_compute_bins_width() {
        let data: Vec<f64> = (0..100).map(|x| x as f64).collect();
        let bins = compute_bins(&data, &BinConfig::Width(10.0));
        assert_eq!(bins.len(), 10);
    }

    #[test]
    fn test_compute_bins_edges() {
        let data: Vec<f64> = (0..100).map(|x| x as f64).collect();
        let bins = compute_bins(&data, &BinConfig::Edges(vec![0.0, 25.0, 50.0, 75.0, 100.0]));
        assert_eq!(bins.len(), 4);
    }

    #[test]
    fn test_compute_bins_empty() {
        let bins = compute_bins(&[], &BinConfig::Auto);
        assert!(bins.is_empty());
    }

    #[test]
    fn test_histogram_bin_frequency() {
        let data: Vec<f64> = (0..100).map(|x| x as f64).collect();
        let bins = compute_bins(&data, &BinConfig::Count(10));

        // Sum of frequencies should be approximately 1.0
        let total_freq: f64 = bins.iter().map(|b| b.frequency).sum();
        assert!((total_freq - 1.0).abs() < 0.01);
    }
}
