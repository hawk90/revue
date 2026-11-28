//! TimeSeries bounds tests: the time range and the value range with y_min, y_max and y_range

use super::{wall_at, NOW};
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

#[test]
fn the_last_ranges_end_at_the_mocked_time_and_move_with_it() {
    let (time, _clock) = wall_at(NOW);
    for (range, span) in [
        (TimeRange::LastSeconds(1), 1),
        (TimeRange::LastMinutes(30), 1800),
        (TimeRange::LastHours(2), 7200),
        (TimeRange::LastDays(7), 7 * 86_400),
    ] {
        let chart = TimeSeries::new().time_range(range);
        assert_eq!(chart.get_time_bounds(), (NOW - span, NOW), "{range:?}");
    }
    time.advance_secs(10);
    let chart = TimeSeries::new().time_range(TimeRange::LastMinutes(1));
    assert_eq!(chart.get_time_bounds(), (NOW - 50, NOW + 10));
}

#[test]
fn an_empty_chart_shows_the_hour_before_the_mocked_time() {
    let (_time, _clock) = wall_at(NOW);
    assert_eq!(TimeSeries::new().get_time_bounds(), (NOW - 3600, NOW));
}

#[test]
fn ranges_near_1970_stop_at_zero() {
    let (_time, _clock) = wall_at(100);
    assert_eq!(TimeSeries::new().get_time_bounds(), (0, 100), "no data");
    let chart = TimeSeries::new().time_range(TimeRange::LastMinutes(5));
    assert_eq!(chart.get_time_bounds(), (0, 100));
    let (_time, _clock) = wall_at(0);
    assert_eq!(TimeSeries::new().get_time_bounds(), (0, 0));
}

#[test]
fn value_bounds_keep_only_points_in_the_mocked_range() {
    let (time, _clock) = wall_at(NOW);
    let chart = TimeSeries::new()
        .time_range(TimeRange::LastSeconds(60))
        .series(
            TimeSeriesData::new("cpu")
                .point(NOW - 61, 1000.0)
                .point(NOW - 60, 10.0)
                .point(NOW, 20.0),
        );
    assert_eq!(chart.get_value_bounds(), (9.0, 21.0), "61 s ago is out");
    time.advance_secs(1);
    assert_eq!(
        chart.get_value_bounds(),
        (19.0, 21.0),
        "a second later, 60 s ago is out too"
    );
}
