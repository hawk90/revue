//! TimeSeries marker tests: line and point markers with labels

use super::super::{render, render_rows};
use super::two_points;
use revue::style::Color;
use revue::widget::data::chart::timeseries::{MarkerStyle, TimeMarker, TimeSeries, TimeSeriesData};

#[test]
fn test_time_series_marker() {
    // Markers at the first and last timestamp: lines in the plot's
    // first and last column
    let chart = TimeSeries::new()
        .series(
            TimeSeriesData::new("Data")
                .point(1000, 50.0)
                .point(2000, 75.0),
        )
        .marker(TimeMarker::new(1000, "Event1"))
        .marker(TimeMarker::new(2000, "Event2"));
    let buffer = render(&chart, 60, 20);
    // The plot spans columns 8..59, rows 1..18
    for x in [8, 58] {
        let marked = (1..18)
            .filter(|&y| buffer.get(x, y).unwrap().fg == Some(Color::YELLOW))
            .count();
        assert!(marked >= 15, "column {x}: {marked}");
    }
}

#[test]
fn test_time_series_render_with_markers() {
    let chart = TimeSeries::new()
        .series(two_points())
        .marker(TimeMarker::new(1500, "Event"))
        .marker(TimeMarker::new(1200, "P").style(MarkerStyle::Point));
    let rows = render_rows(&chart, 40, 12);
    // A line marker halfway through the plot (columns 8..38), a point
    // marker on its top row, and both labels under the plot
    let line_x = 8 + 15;
    let column: String = rows[1..10]
        .iter()
        .map(|r| r.chars().nth(line_x).unwrap())
        .collect();
    assert!(
        column.chars().filter(|&c| c == '│').count() >= 7,
        "{rows:#?}"
    );
    assert!(rows[1].contains('▼'), "{rows:#?}");
    assert!(
        rows[11].contains("Event") && rows[11].contains('P'),
        "{rows:#?}"
    );
}
