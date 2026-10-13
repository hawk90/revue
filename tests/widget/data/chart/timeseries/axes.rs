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
