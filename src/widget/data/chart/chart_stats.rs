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
    // Safe: filter_finite() removes NaN, so partial_cmp always returns Some
    valid.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
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
    // Safe: filter_finite() removes NaN, so partial_cmp always returns Some
    valid.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

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
    // Safe: filter_finite() removes NaN, so partial_cmp always returns Some
    valid.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

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

// All tests extracted to tests/widget/data/chart/chart_stats.rs
