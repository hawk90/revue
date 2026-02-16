//! Statistical functions for chart widgets public API tests extracted from chart_stats.rs

use crate::widget::data::chart::chart_stats::{filter_finite, percentile, mean, median, quartiles, min_max, iqr, outliers_iqr, BinConfig, HistogramBin, compute_bins};

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

// =========================================================================
// BinConfig enum tests (derived traits)
// =========================================================================

#[test]
fn test_bin_config_default() {
    assert_eq!(BinConfig::default(), BinConfig::Auto);
}

#[test]
fn test_bin_config_clone() {
    let config1 = BinConfig::Count(5);
    let config2 = config1.clone();
    assert_eq!(config1, config2);
}

#[test]
fn test_bin_config_copy() {
    let config1 = BinConfig::Width(10.0);
    let config2 = config1;
    assert_eq!(config2, BinConfig::Width(10.0));
}

#[test]
fn test_bin_config_partial_eq() {
    assert_eq!(BinConfig::Auto, BinConfig::Auto);
    assert_eq!(BinConfig::Count(5), BinConfig::Count(5));
    assert_ne!(BinConfig::Auto, BinConfig::Count(5));
}

// =========================================================================
// HistogramBin struct tests
// =========================================================================

#[test]
fn test_histogram_bin_creation() {
    let bin = HistogramBin {
        start: 0.0,
        end: 10.0,
        count: 5,
        frequency: 0.5,
        density: 0.05,
    };
    assert_eq!(bin.start, 0.0);
    assert_eq!(bin.end, 10.0);
    assert_eq!(bin.count, 5);
    assert_eq!(bin.frequency, 0.5);
    assert_eq!(bin.density, 0.05);
}

#[test]
fn test_histogram_bin_clone() {
    let bin1 = HistogramBin {
        start: 0.0,
        end: 10.0,
        count: 5,
        frequency: 0.5,
        density: 0.05,
    };
    let bin2 = bin1.clone();
    assert_eq!(bin1.start, bin2.start);
    assert_eq!(bin1.frequency, bin2.frequency);
}

// =========================================================================
// Statistical function edge cases
// =========================================================================

#[test]
fn test_mean_with_nan_only() {
    let data = vec![f64::NAN, f64::NAN, f64::NAN];
    assert_eq!(mean(&data), None);
}

#[test]
fn test_median_with_nan() {
    let data = vec![1.0, 2.0, f64::NAN, 3.0, 4.0];
    assert_eq!(median(&data), Some(2.5));
}

#[test]
fn test_quartiles_with_nan() {
    let data = vec![1.0, 2.0, f64::NAN, 3.0, 4.0, 5.0];
    let result = quartiles(&data);
    assert!(result.is_some());
    assert!(result.unwrap().0 > 0.0);
}

#[test]
fn test_min_max_with_nan() {
    let data = vec![1.0, f64::NAN, 3.0, f64::NAN, 5.0];
    assert_eq!(min_max(&data), Some((1.0, 5.0)));
}

#[test]
fn test_iqr_with_nan() {
    let data = vec![1.0, 2.0, f64::NAN, 4.0, 5.0];
    assert!(iqr(&data).unwrap() > 0.0);
}

#[test]
fn test_outliers_iqr_no_outliers() {
    let data: Vec<f64> = (10..20).map(|x| x as f64).collect();
    let outliers = outliers_iqr(&data);
    assert!(outliers.is_empty());
}

// =========================================================================
// Bin configuration edge cases
// =========================================================================

#[test]
fn test_compute_bins_count_zero() {
    let data: Vec<f64> = (0..100).map(|x| x as f64).collect();
    let bins = compute_bins(&data, &BinConfig::Count(0));
    // Should use at least 1 bin
    assert!(!bins.is_empty());
}

#[test]
fn test_compute_bins_width_zero() {
    let data: Vec<f64> = (0..100).map(|x| x as f64).collect();
    let bins = compute_bins(&data, &BinConfig::Width(0.0));
    // Should use a small positive width
    assert!(!bins.is_empty());
}

#[test]
fn test_compute_bins_with_negative_values() {
    let data: Vec<f64> = (-10..10).map(|x| x as f64).collect();
    let bins = compute_bins(&data, &BinConfig::Auto);
    assert!(!bins.is_empty());
    assert!(bins[0].start <= -10.0);
    assert!(bins[bins.len() - 1].end >= 10.0);
}

// =========================================================================
// Percentile edge cases
// =========================================================================

#[test]
fn test_percentile_single_value() {
    let data = vec![5.0];
    assert_eq!(percentile(&data, 0.0), 5.0);
    assert_eq!(percentile(&data, 50.0), 5.0);
    assert_eq!(percentile(&data, 100.0), 5.0);
}

#[test]
fn test_percentile_two_values() {
    let data = vec![1.0, 3.0];
    assert_eq!(percentile(&data, 0.0), 1.0);
    assert_eq!(percentile(&data, 25.0), 1.75); // Interpolated
    assert_eq!(percentile(&data, 50.0), 2.0);  // Interpolated
    assert_eq!(percentile(&data, 75.0), 2.25); // Interpolated
    assert_eq!(percentile(&data, 100.0), 3.0);
}

// =========================================================================
// Histogram bin properties
// =========================================================================

#[test]
fn test_histogram_bin_density() {
    let bin = HistogramBin {
        start: 0.0,
        end: 10.0,
        count: 5,
        frequency: 0.5,
        density: 0.05,
    };
    // density = frequency / width
    let expected_density = 0.5 / 10.0;
    assert!((bin.density - expected_density).abs() < 0.0001);
}

#[test]
fn test_histogram_bin_zero_width() {
    let bin = HistogramBin {
        start: 0.0,
        end: 0.0,
        count: 5,
        frequency: 0.5,
        density: 0.0, // Should be zero for zero width
    };
    assert_eq!(bin.density, 0.0);
}