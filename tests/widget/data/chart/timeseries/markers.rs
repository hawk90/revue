//! TimeSeries marker tests: line and point markers with labels

use super::super::{render, render_at, render_rows, rows};
use super::two_points;
use revue::layout::Rect;
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

/// A marker label of 65536 columns is centered on its marker by its real
/// width, so it starts at the area's left edge instead of at the marker,
/// and runs to the area's edge under the time labels drawn after it;
/// inside an area at an offset (#1596)
#[test]
fn test_time_series_marker_label_wider_than_u16() {
    let chart = TimeSeries::new()
        .time_range(super::fixed_range())
        .marker(TimeMarker::new(1800, "m".repeat(65_536)).color(Color::GREEN));
    let buffer = render_at(&chart, Rect::new(3, 2, 30, 12));
    let rows = rows(&buffer);
    assert_eq!(rows[13], "   mmmmmm00:00mmmmmmmmmmmmmmm01:0  ");
    for x in 0..35 {
        let cell = buffer.get(x, 13).unwrap();
        if cell.symbol == 'm' {
            assert_eq!(cell.fg, Some(Color::GREEN), "{x}");
        }
    }
    // The marker line is in the plot's middle column
    assert_eq!(buffer.get(21, 3).unwrap().symbol, '│');
}
