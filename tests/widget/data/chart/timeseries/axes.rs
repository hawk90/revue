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
    assert_eq!(formatted, "01:01:01");
}

fn format(format: TimeFormat, timestamp: u64) -> String {
    TimeSeries::new()
        .time_format(format)
        .format_time(timestamp, 0)
}

/// Days and Months are real UTC calendar dates: month lengths, leap days
/// and year ends (#1574)
#[test]
fn test_time_series_days_and_months_are_utc_dates() {
    for (ts, day, month) in [
        (0, "01/01", "1970/01"),
        // 2023-11-14 22:13:20 UTC
        (1_700_000_000, "11/14", "2023/11"),
        // 2024-02-29, a leap day, and the next day
        (1_709_164_800, "02/29", "2024/02"),
        (1_709_251_200, "03/01", "2024/03"),
        // The last second of 2023 and the first of 2024
        (1_704_067_199, "12/31", "2023/12"),
        (1_704_067_200, "01/01", "2024/01"),
        // 2100 is not a leap year: Feb 28 is followed by Mar 1
        (4_107_456_000, "02/28", "2100/02"),
        (4_107_542_400, "03/01", "2100/03"),
    ] {
        assert_eq!(format(TimeFormat::Days, ts), day, "{ts}");
        assert_eq!(format(TimeFormat::Months, ts), month, "{ts}");
    }
}

/// The last second the calendar can show, and past it the Unix seconds
#[test]
fn test_time_series_dates_past_the_calendar_range() {
    let last = 253_402_207_200; // 9999-12-30 22:00:00 UTC
    assert_eq!(format(TimeFormat::Days, last), "12/30");
    assert_eq!(format(TimeFormat::Months, last), "9999/12");
    for ts in [last + 1, i64::MAX as u64, u64::MAX] {
        assert_eq!(format(TimeFormat::Days, ts), ts.to_string());
        assert_eq!(format(TimeFormat::Months, ts), ts.to_string());
    }
    // Clock times need no calendar: u64::MAX is 07:00:15 UTC
    assert_eq!(format(TimeFormat::Seconds, u64::MAX), "07:00:15");
}

/// Auto picks the format from the axis span: under a minute HH:MM:SS,
/// under an hour HH:MM, under a day HH:00, under 30 days MM/DD, else
/// YYYY/MM (#1574)
#[test]
fn test_time_series_auto_format_follows_the_range() {
    let chart = TimeSeries::new();
    let ts = 1_700_000_000; // 2023-11-14 22:13:20 UTC
    for (range, want) in [
        (59, "22:13:20"),
        (60, "22:13"),
        (3599, "22:13"),
        (3600, "22:00"),
        (86_399, "22:00"),
        (86_400, "11/14"),
        (86_400 * 30 - 1, "11/14"),
        (86_400 * 30, "2023/11"),
    ] {
        assert_eq!(chart.format_time(ts, range), want, "range {range}");
    }
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
    assert_eq!(rows[11], "   2m                                0s ");
    // A minute ago is the middle of the plot
    assert_eq!(rows[5], "5.0 ┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄●┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄ ");
}

/// The value labels' column is as wide as the widest label, at most a
/// third of the area, with a gap column before the plot; labels are
/// right-aligned and a wider one is cut with `…` (#1614)
#[test]
fn test_time_series_value_label_column_fits_the_widest_label() {
    let chart = |min: f64, max: f64| {
        TimeSeries::new()
            .time_range(super::fixed_range())
            .y_range(min, max)
            .bg(Color::BLUE)
    };
    // "0.75", "0.50", ...: 4 columns, the plot at 5..19
    let buffer = render(&chart(0.0, 1.0), 20, 10);
    let rows = super::super::rows(&buffer);
    let grid = "┄".repeat(14);
    assert_eq!(rows[0], format!(" 1.0 {grid} "));
    assert_eq!(rows[2], format!("0.75 {grid} "));
    assert_eq!(rows[8], format!("0.00 {} ", "─".repeat(14)));
    // The labels keep the background
    for x in 0..4 {
        assert_eq!(buffer.get(x, 2).unwrap().bg, Some(Color::BLUE), "x={x}");
    }
    // "-123.5k" is 7 columns, past a third of 15: cut to 5
    let rows = super::super::rows(&render(&chart(-123_456.0, 123_456.0), 15, 10));
    let grid = "┄".repeat(8);
    assert_eq!(rows[0], format!("123.… {grid} "));
    assert_eq!(rows[2], format!("61.7k {grid} "));
    assert_eq!(rows[4], format!(" 0.00 {grid} "));
    assert_eq!(rows[6], format!("-61.… {grid} "));
    assert_eq!(rows[8], format!("-123… {} ", "─".repeat(8)));
}
