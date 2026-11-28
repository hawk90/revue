//! TimeSeries axes tests: time formats, the grid and its color

use super::super::{render, render_rows};
use super::two_points;
use revue::style::Color;
use revue::widget::data::chart::timeseries::{TimeFormat, TimeSeries};

#[test]
fn test_time_series_time_format() {
    // Verify time_format affects format_time output
    let chart = TimeSeries::new().time_format(TimeFormat::Seconds);
    // With Seconds format, output should include colons (HH:MM:SS)
    let formatted = chart.format_time(3661, 100);
    assert!(formatted.contains(':'));
}

#[test]
fn test_time_series_show_grid() {
    let rows = render_rows(&TimeSeries::new().series(two_points()), 40, 12);
    assert!(rows.iter().any(|r| r.contains('┄')));

    let chart = TimeSeries::new().series(two_points()).show_grid(false);
    let rows = render_rows(&chart, 40, 12);
    assert!(!rows.iter().any(|r| r.contains('┄')), "{rows:#?}");
    assert_eq!(super::super::count_char(&render(&chart, 40, 12), '●'), 2);
}

#[test]
fn test_time_series_grid_color() {
    let chart = TimeSeries::new()
        .series(two_points())
        .grid_color(Color::MAGENTA);
    let buffer = render(&chart, 40, 12);
    let grid: Vec<_> = (0..12)
        .flat_map(|y| (0..40).map(move |x| (x, y)))
        .filter_map(|(x, y)| buffer.get(x, y))
        .filter(|c| c.symbol == '┄')
        .collect();
    assert!(!grid.is_empty());
    assert!(grid.iter().all(|c| c.fg == Some(Color::MAGENTA)));
}

#[test]
fn relative_labels_count_from_the_mocked_time() {
    let (time, _clock) = super::wall_at(super::NOW);
    let chart = TimeSeries::new().time_format(TimeFormat::Relative);
    let now = super::NOW;
    assert_eq!(chart.format_time(now, 60), "0s");
    assert_eq!(chart.format_time(now - 59, 60), "59s");
    assert_eq!(chart.format_time(now - 60, 60), "1m");
    assert_eq!(chart.format_time(now - 3600, 60), "1h");
    assert_eq!(chart.format_time(now - 86_400, 60), "1d");
    assert_eq!(chart.format_time(now + 5, 60), "0s", "a point after now");
    time.advance_secs(30);
    assert_eq!(chart.format_time(now, 60), "30s");
}

#[test]
fn relative_time_labels_draw_from_the_mocked_time() {
    use revue::widget::data::chart::timeseries::{TimeRange, TimeSeriesData};
    let (_time, _clock) = super::wall_at(super::NOW);
    let chart = TimeSeries::new()
        .time_format(TimeFormat::Relative)
        .time_range(TimeRange::LastMinutes(2))
        .series(TimeSeriesData::new("cpu").point(super::NOW - 60, 5.0));
    let rows = render_rows(&chart, 40, 12);
    // Two minutes ago on the left, now on the right
    assert_eq!(rows[11], "       2m                            0s ");
    // A minute ago is the middle of the plot
    assert_eq!(rows[5], "    5.0 ┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄●┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄ ");
}
