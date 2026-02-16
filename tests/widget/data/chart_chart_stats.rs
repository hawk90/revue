//! Tests for chart stats public API
//!
//! Extracted from src/widget/data/chart/chart_stats.rs

use revue::widget::data::chart::{
    filter_finite, percentile, mean, median, quartiles, min_max, iqr,
    outliers_iqr, BinConfig, compute_bins, HistogramBin,
};

#[test]
fn test_filter_finite() {
    let data = vec![1.0, f64::NAN, 2.0, f64::INFINITY, 3.0, f64::NEG_INFINITY];
    let filtered = filter_finite(&data);

    assert_eq!(filtered, vec![1.0, 2.0, 3.0]);
}

#[test]
fn test_percentile() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];

    // 0th percentile is minimum
    assert_eq!(percentile(&data, 0.0), 1.0);

    // 50th percentile is median
    assert_eq!(percentile(&data, 50.0), 3.0);

    // 100th percentile is maximum
    assert_eq!(percentile(&data, 100.0), 5.0);

    // 25th percentile (linear interpolation)
    assert_eq!(percentile(&data, 25.0), 1.75);
}

#[test]
fn test_mean() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    assert_eq!(mean(&data), Some(3.0));

    let data_with_nan = vec![1.0, f64::NAN, 3.0];
    assert_eq!(mean(&data_with_nan), Some(2.0));

    let empty_data: Vec<f64> = vec![];
    assert_eq!(mean(&empty_data), None);
}

#[test]
fn test_median() {
    let even_data = vec![1.0, 2.0, 3.0, 4.0];
    assert_eq!(median(&even_data), Some(2.5));

    let odd_data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    assert_eq!(median(&odd_data), Some(3.0));

    let data_with_nan = vec![1.0, f64::NAN, 3.0];
    assert_eq!(median(&data_with_nan), Some(2.0));

    let empty_data: Vec<f64> = vec![];
    assert_eq!(median(&empty_data), None);
}

#[test]
fn test_quartiles() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let (q1, median, q3) = quartiles(&data).unwrap();

    assert_eq!(q1, 2.5);  // 25th percentile
    assert_eq!(median, 4.5);  // 50th percentile
    assert_eq!(q3, 6.5);  // 75th percentile

    let empty_data: Vec<f64> = vec![];
    assert_eq!(quartiles(&empty_data), None);
}

#[test]
fn test_min_max() {
    let data = vec![1.0, 5.0, 3.0, 9.0, 2.0];
    let (min, max) = min_max(&data).unwrap();

    assert_eq!(min, 1.0);
    assert_eq!(max, 9.0);

    let data_with_nan = vec![1.0, f64::NAN, 3.0];
    let (min, max) = min_max(&data_with_nan).unwrap();
    assert_eq!(min, 1.0);
    assert_eq!(max, 3.0);

    let empty_data: Vec<f64> = vec![];
    assert_eq!(min_max(&empty_data), None);
}

#[test]
fn test_iqr() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let iqr_value = iqr(&data);

    assert_eq!(iqr_value.unwrap(), 4.0);  // Q3 - Q1 = 6.5 - 2.5 = 4.0

    let empty_data: Vec<f64> = vec![];
    assert_eq!(iqr(&empty_data), None);
}

#[test]
fn test_outliers_iqr() {
    let data = vec![
        // Normal range
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0,
        // Lower outlier
        -10.0,
        // Upper outlier
        20.0,
    ];

    let outliers = outliers_iqr(&data);
    assert_eq!(outliers.len(), 2);
    assert!(outliers.contains(&-10.0));
    assert!(outliers.contains(&20.0));

    let no_outliers = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let outliers = outliers_iqr(&no_outliers);
    assert!(outliers.is_empty());
}

#[test]
fn test_histogram_bin() {
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
fn test_compute_bins_auto() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let bins = compute_bins(&data, &BinConfig::Auto);

    assert!(!bins.is_empty());
    // Should use Sturges' rule - log2(10) + 1 ≈ 4.32, rounded up to 5 bins
    assert!(bins.len() >= 4);  // At least 4 bins
    assert!(bins.len() <= 10);  // At most 10 bins

    // Check that all data is distributed across bins
    let total_count: usize = bins.iter().map(|bin| bin.count).sum();
    assert_eq!(total_count, 10);
}

#[test]
fn test_compute_bins_count() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let bins = compute_bins(&data, &BinConfig::Count(5));

    assert_eq!(bins.len(), 5);
    let total_count: usize = bins.iter().map(|bin| bin.count).sum();
    assert_eq!(total_count, 10);
}

#[test]
fn test_compute_bins_width() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let bins = compute_bins(&data, &BinConfig::Width(2.0));

    assert!(!bins.is_empty());
    let total_count: usize = bins.iter().map(|bin| bin.count).sum();
    assert_eq!(total_count, 10);

    // Check that each bin has the correct width (except possibly last bin)
    for i in 0..bins.len() - 1 {
        let bin_width = bins[i].end - bins[i].start;
        assert_eq!(bin_width, 2.0);
    }
}

#[test]
fn test_compute_bins_edges() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let edges = vec![0.0, 5.0, 10.0, 15.0];
    let bins = compute_bins(&data, &BinConfig::Edges(edges.clone()));

    assert_eq!(bins.len(), edges.len() - 1);
    let total_count: usize = bins.iter().map(|bin| bin.count).sum();
    assert_eq!(total_count, 10);
}

#[test]
fn test_compute_bins_empty_data() {
    let empty_data: Vec<f64> = vec![];
    let bins = compute_bins(&empty_data, &BinConfig::Auto);
    assert!(bins.is_empty());
}

#[test]
fn test_bin_config_default() {
    let default_config = BinConfig::default();
    assert!(matches!(default_config, BinConfig::Auto));
}

#[test]
fn test_bin_config_clone() {
    let config = BinConfig::Count(10);
    let cloned = config.clone();
    assert!(matches!(cloned, BinConfig::Count(10)));
}