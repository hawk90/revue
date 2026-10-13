//! TimeSeries bounds tests: the time range and the value range with y_min, y_max and y_range

use revue::widget::data::chart::timeseries::{TimeRange, TimeSeries, TimeSeriesData};

#[test]
fn test_time_series_series() {
    // Verify series are stored by checking get_value_bounds works with data
    let chart = TimeSeries::new()
        .series(TimeSeriesData::new("Data1").point(100, 10.0))
        .series(TimeSeriesData::new("Data2").point(200, 20.0));
    // Both series have data, so value bounds should reflect both
    let (min, max) = chart.get_value_bounds();
    assert!(min <= 10.0);
    assert!(max >= 20.0);
}

#[test]
fn test_time_series_time_range() {
    // Verify time_range is applied by checking get_time_bounds
    let chart = TimeSeries::new().time_range(TimeRange::LastMinutes(30));
    let (start, end) = chart.get_time_bounds();
    // end - start should be approximately 30 * 60 = 1800 seconds
    let diff = end.saturating_sub(start);
    assert!((diff as i64 - 1800).abs() <= 2);
}

#[test]
fn test_time_series_y_min_max() {
    let chart = TimeSeries::new().y_min(0.0).y_max(100.0);
    let (min, max) = chart.get_value_bounds();
    assert_eq!(min, 0.0);
    assert_eq!(max, 100.0);
}

#[test]
fn test_time_series_y_range() {
    let chart = TimeSeries::new().y_range(-50.0, 50.0);
    let (min, max) = chart.get_value_bounds();
    assert_eq!(min, -50.0);
    assert_eq!(max, 50.0);
}
