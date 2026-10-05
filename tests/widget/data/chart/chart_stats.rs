//! Statistical functions used by the Histogram and BoxPlot widgets

use revue::widget::data::chart::chart_stats::{
    compute_bins, filter_finite, iqr, mean, median, min_max, outliers_iqr, percentile, quartiles,
    BinConfig, HistogramBin,
};

#[test]
fn test_filter_finite() {
    let data = vec![1.0, f64::NAN, 2.0, f64::INFINITY, 3.0, f64::NEG_INFINITY];
    let filtered = filter_finite(&data);
    assert_eq!(filtered, vec![1.0, 2.0, 3.0]);
}

#[test]
fn test_percentile() {
    let sorted = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    assert_eq!(percentile(&sorted, 0.0), 1.0);
    // k = 0.25 * (5 - 1) = 1.0 -> sorted[1]
    assert_eq!(percentile(&sorted, 25.0), 2.0);
    assert_eq!(percentile(&sorted, 50.0), 3.0);
    assert_eq!(percentile(&sorted, 100.0), 5.0);
}

#[test]
fn test_percentile_empty() {
    let sorted: Vec<f64> = vec![];
    assert_eq!(percentile(&sorted, 50.0), 0.0);
}

#[test]
fn test_percentile_single_value() {
    let data = vec![5.0];
    assert_eq!(percentile(&data, 0.0), 5.0);
    assert_eq!(percentile(&data, 50.0), 5.0);
    assert_eq!(percentile(&data, 100.0), 5.0);
}

#[test]
fn test_percentile_two_values() {
    // Linear interpolation: k = p/100 * (n - 1) = p/100
    let data = vec![1.0, 3.0];
    assert_eq!(percentile(&data, 0.0), 1.0);
    assert_eq!(percentile(&data, 25.0), 1.5);
    assert_eq!(percentile(&data, 50.0), 2.0);
    assert_eq!(percentile(&data, 75.0), 2.5);
    assert_eq!(percentile(&data, 100.0), 3.0);
}

#[test]
fn test_mean() {
    assert_eq!(mean(&[1.0, 2.0, 3.0, 4.0, 5.0]), Some(3.0));
    assert_eq!(mean(&[1.0, f64::NAN, 3.0]), Some(2.0));
    assert_eq!(mean(&[]), None);
    assert_eq!(mean(&[f64::NAN]), None);
}

#[test]
fn test_mean_with_nan_only() {
    let data = vec![f64::NAN, f64::NAN, f64::NAN];
    assert_eq!(mean(&data), None);
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
fn test_median_with_nan() {
    assert_eq!(median(&[1.0, f64::NAN, 3.0]), Some(2.0));
    assert_eq!(median(&[1.0, 2.0, f64::NAN, 3.0, 4.0]), Some(2.5));
}

#[test]
fn test_quartiles_odd() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    // Linear interpolation: k = p/100 * (n-1), where n=9
    // Q1: k = 2.0 -> 3.0, median: k = 4.0 -> 5.0, Q3: k = 6.0 -> 7.0
    assert_eq!(quartiles(&data), Some((3.0, 5.0, 7.0)));
}

#[test]
fn test_quartiles_even() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    // k = 1.75, 3.5, 5.25
    assert_eq!(quartiles(&data), Some((2.75, 4.5, 6.25)));
    assert_eq!(quartiles(&[]), None);
}

#[test]
fn test_quartiles_with_nan() {
    // NaN is dropped: same as [1, 2, 3, 4, 5]
    let data = vec![1.0, 2.0, f64::NAN, 3.0, 4.0, 5.0];
    assert_eq!(quartiles(&data), Some((2.0, 3.0, 4.0)));
}

#[test]
fn test_min_max() {
    assert_eq!(min_max(&[3.0, 1.0, 4.0, 1.0, 5.0]), Some((1.0, 5.0)));
    assert_eq!(min_max(&[]), None);
}

#[test]
fn test_min_max_with_nan() {
    let data = vec![1.0, f64::NAN, 3.0, f64::NAN, 5.0];
    assert_eq!(min_max(&data), Some((1.0, 5.0)));
}

#[test]
fn test_iqr() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    // Q3 - Q1 = 7 - 3
    assert_eq!(iqr(&data), Some(4.0));
    assert_eq!(iqr(&[]), None);
}

#[test]
fn test_iqr_with_nan() {
    // Same as [1, 2, 4, 5]: Q1 = 1.75, Q3 = 4.25
    let data = vec![1.0, 2.0, f64::NAN, 4.0, 5.0];
    assert_eq!(iqr(&data), Some(2.5));
}

#[test]
fn test_outliers_iqr() {
    let mut data: Vec<f64> = (0..20).map(|x| x as f64).collect();
    data.push(100.0);
    data.push(-50.0);

    let outliers = outliers_iqr(&data);
    assert_eq!(outliers, vec![-50.0, 100.0]);
}

#[test]
fn test_outliers_iqr_no_outliers() {
    let data: Vec<f64> = (10..20).map(|x| x as f64).collect();
    assert!(outliers_iqr(&data).is_empty());
    assert!(outliers_iqr(&[]).is_empty());
}

#[test]
fn test_compute_bins_auto() {
    // Sturges' rule: ceil(log2(100) + 1) = 8 bins
    let data: Vec<f64> = (0..100).map(|x| x as f64).collect();
    let bins = compute_bins(&data, &BinConfig::Auto);
    assert_eq!(bins.len(), 8);
    let total: usize = bins.iter().map(|b| b.count).sum();
    assert_eq!(total, 100);
}

#[test]
fn test_compute_bins_count() {
    let data: Vec<f64> = (0..100).map(|x| x as f64).collect();
    let bins = compute_bins(&data, &BinConfig::Count(10));
    assert_eq!(bins.len(), 10);
    let total: usize = bins.iter().map(|b| b.count).sum();
    assert_eq!(total, 100);
}

#[test]
fn test_compute_bins_width() {
    let data: Vec<f64> = (0..100).map(|x| x as f64).collect();
    let bins = compute_bins(&data, &BinConfig::Width(10.0));
    assert_eq!(bins.len(), 10);
    for bin in &bins {
        assert_eq!(bin.end - bin.start, 10.0);
    }
    let total: usize = bins.iter().map(|b| b.count).sum();
    assert_eq!(total, 100);
}

#[test]
fn test_compute_bins_edges() {
    let data: Vec<f64> = (0..100).map(|x| x as f64).collect();
    let bins = compute_bins(&data, &BinConfig::Edges(vec![0.0, 25.0, 50.0, 75.0, 100.0]));
    assert_eq!(bins.len(), 4);
    let counts: Vec<usize> = bins.iter().map(|b| b.count).collect();
    assert_eq!(counts, vec![25, 25, 25, 25]);
}

#[test]
fn test_compute_bins_last_edge_inclusive() {
    let data = vec![0.0, 5.0, 10.0];
    let bins = compute_bins(&data, &BinConfig::Edges(vec![0.0, 5.0, 10.0]));
    let counts: Vec<usize> = bins.iter().map(|b| b.count).collect();
    // 5.0 goes to the second bin (start inclusive), 10.0 is in the last bin
    assert_eq!(counts, vec![1, 2]);
}

#[test]
fn test_compute_bins_empty() {
    assert!(compute_bins(&[], &BinConfig::Auto).is_empty());
    assert!(compute_bins(&[f64::NAN], &BinConfig::Auto).is_empty());
}

#[test]
fn test_compute_bins_count_zero() {
    let data: Vec<f64> = (0..100).map(|x| x as f64).collect();
    let bins = compute_bins(&data, &BinConfig::Count(0));
    // Clamped to at least one bin
    assert_eq!(bins.len(), 1);
    assert_eq!(bins[0].count, 100);
}

#[test]
fn test_compute_bins_width_zero() {
    let data: Vec<f64> = (0..10).map(|x| x as f64).collect();
    let bins = compute_bins(&data, &BinConfig::Width(0.0));
    // Clamped to a small positive width
    assert!(!bins.is_empty());
    let total: usize = bins.iter().map(|b| b.count).sum();
    assert_eq!(total, 10);
}

#[test]
fn test_compute_bins_with_negative_values() {
    let data: Vec<f64> = (-10..10).map(|x| x as f64).collect();
    let bins = compute_bins(&data, &BinConfig::Auto);
    assert!(!bins.is_empty());
    assert_eq!(bins[0].start, -10.0);
    assert!((bins[bins.len() - 1].end - 9.0).abs() < 1e-9);
}

#[test]
fn test_histogram_bin_frequency_and_density() {
    let data: Vec<f64> = (0..100).map(|x| x as f64).collect();
    let bins = compute_bins(&data, &BinConfig::Count(10));

    let total_freq: f64 = bins.iter().map(|b| b.frequency).sum();
    assert!((total_freq - 1.0).abs() < 1e-9);

    for bin in &bins {
        assert_eq!(bin.frequency, bin.count as f64 / 100.0);
        let width = bin.end - bin.start;
        assert!((bin.density - bin.frequency / width).abs() < 1e-12);
    }
}

#[test]
fn test_histogram_bin_zero_width_has_zero_density() {
    let bins = compute_bins(&[1.0, 1.0], &BinConfig::Edges(vec![1.0, 1.0]));
    assert_eq!(bins.len(), 1);
    assert_eq!(bins[0].count, 2);
    assert_eq!(bins[0].density, 0.0);
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
    assert_eq!(bin1.end, bin2.end);
    assert_eq!(bin1.count, bin2.count);
    assert_eq!(bin1.frequency, bin2.frequency);
    assert_eq!(bin1.density, bin2.density);
}

#[test]
fn test_bin_config_default() {
    assert!(matches!(BinConfig::default(), BinConfig::Auto));
}

#[test]
fn test_bin_config_clone() {
    let config = BinConfig::Edges(vec![0.0, 1.0]);
    let cloned = config.clone();
    assert!(matches!(cloned, BinConfig::Edges(ref e) if *e == vec![0.0, 1.0]));
}
