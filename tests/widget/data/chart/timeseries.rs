//! TimeSeries widget tests

use super::{render, render_rows};
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::data::chart::timeseries::{
    cpu_chart, memory_chart, network_chart, time_series, time_series_with_data, MarkerStyle,
    TimeFormat, TimeLineStyle, TimeMarker, TimePoint, TimeRange, TimeSeries, TimeSeriesData,
};
use revue::widget::traits::{RenderContext, View};

// ========================================================================
// TimePoint tests
// ========================================================================

#[test]
fn test_time_point_new() {
    let point = TimePoint::new(1000, 42.0);
    assert_eq!(point.timestamp, 1000);
    assert_eq!(point.value, 42.0);
}

#[test]
fn test_time_point_now() {
    let point = TimePoint::now(50.0);
    assert!(point.timestamp > 0);
    assert_eq!(point.value, 50.0);
}

// ========================================================================
// TimeSeriesData tests
// ========================================================================

#[test]
fn test_time_series_data_new() {
    let data = TimeSeriesData::new("Test");
    assert_eq!(data.name, "Test");
    assert!(data.points.is_empty());
    assert_eq!(data.color, Color::CYAN);
    assert!(!data.fill);
}

#[test]
fn test_time_series_data_point() {
    let data = TimeSeriesData::new("Test")
        .point(100, 10.0)
        .point(200, 20.0);

    assert_eq!(data.points.len(), 2);
    assert_eq!(data.points[0].timestamp, 100);
    assert_eq!(data.points[0].value, 10.0);
}

#[test]
fn test_time_series_data_points() {
    let data = TimeSeriesData::new("Test").points(vec![(100, 10.0), (200, 20.0), (300, 30.0)]);

    assert_eq!(data.points.len(), 3);
}

#[test]
fn test_time_series_data_color() {
    let data = TimeSeriesData::new("Test").color(Color::RED);
    assert_eq!(data.color, Color::RED);
}

#[test]
fn test_time_series_data_line_style() {
    let data = TimeSeriesData::new("Test").line_style(TimeLineStyle::Dashed);
    assert_eq!(data.line_style, TimeLineStyle::Dashed);
}

#[test]
fn test_time_series_data_filled() {
    let data = TimeSeriesData::new("Test").filled();
    assert!(data.fill);
}

// ========================================================================
// TimeLineStyle tests
// ========================================================================

#[test]
fn test_time_line_style_default() {
    assert_eq!(TimeLineStyle::default(), TimeLineStyle::Solid);
}

#[test]
fn test_time_line_style_variants() {
    assert_eq!(TimeLineStyle::Solid, TimeLineStyle::Solid);
    assert_ne!(TimeLineStyle::Solid, TimeLineStyle::Dashed);
    assert_ne!(TimeLineStyle::Dashed, TimeLineStyle::Dotted);
    assert_ne!(TimeLineStyle::Dotted, TimeLineStyle::Step);
}

// ========================================================================
// TimeFormat tests
// ========================================================================

#[test]
fn test_time_format_default() {
    assert_eq!(TimeFormat::default(), TimeFormat::Auto);
}

#[test]
fn test_time_format_variants() {
    assert_ne!(TimeFormat::Seconds, TimeFormat::Minutes);
    assert_ne!(TimeFormat::Hours, TimeFormat::Days);
    assert_ne!(TimeFormat::Months, TimeFormat::Unix);
}

// ========================================================================
// TimeMarker tests
// ========================================================================

#[test]
fn test_time_marker_new() {
    let marker = TimeMarker::new(1000, "Event");
    assert_eq!(marker.timestamp, 1000);
    assert_eq!(marker.label, "Event");
    assert_eq!(marker.color, Color::YELLOW);
}

#[test]
fn test_time_marker_color() {
    let marker = TimeMarker::new(1000, "Event").color(Color::RED);
    assert_eq!(marker.color, Color::RED);
}

#[test]
fn test_time_marker_style() {
    let marker = TimeMarker::new(1000, "Event").style(MarkerStyle::Point);
    assert_eq!(marker.style, MarkerStyle::Point);
}

// ========================================================================
// MarkerStyle tests
// ========================================================================

#[test]
fn test_marker_style_default() {
    assert_eq!(MarkerStyle::default(), MarkerStyle::Line);
}

// ========================================================================
// TimeSeries tests
// ========================================================================

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
fn test_time_series_time_format() {
    // Verify time_format affects format_time output
    let chart = TimeSeries::new().time_format(TimeFormat::Seconds);
    // With Seconds format, output should include colons (HH:MM:SS)
    let formatted = chart.format_time(3661, 100);
    assert!(formatted.contains(':'));
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
fn y_label_is_drawn_above_the_value_axis() {
    let chart = TimeSeries::new()
        .series(TimeSeriesData::new("Mem").point(0, 1.0).point(10, 4.0))
        .show_legend(false)
        .y_label("GB");
    let mut buffer = Buffer::new(40, 12);
    let area = Rect::new(0, 0, 40, 12);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);

    let top: String = (0..40)
        .map(|x| buffer.get(x, 0).map(|c| c.symbol).unwrap_or(' '))
        .collect();
    assert!(top.starts_with("GB"), "y label not drawn: {top:?}");
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

// ========================================================================
// Helper function tests
// ========================================================================

#[test]
fn test_time_series_with_data_helper() {
    let chart = time_series_with_data(TimeSeriesData::new("Data").point(100, 50.0));
    // One series with a data point: value bounds should reflect it
    let (min, max) = chart.get_value_bounds();
    assert!(min <= 50.0);
    assert!(max >= 50.0);
}

// ========================================================================
// Builders, checked on the screen
// ========================================================================

/// A series from (1000, 50) to (2000, 75)
fn two_points() -> TimeSeriesData {
    TimeSeriesData::new("Data")
        .point(1000, 50.0)
        .point(2000, 75.0)
}

/// A fixed hour, so charts without data do not read the clock
fn fixed_range() -> TimeRange {
    TimeRange::Range {
        start: 0,
        end: 3600,
    }
}

#[test]
fn test_time_range_default() {
    assert!(matches!(TimeRange::default(), TimeRange::All));
}

#[test]
fn test_time_range_variants() {
    let range = TimeRange::LastSeconds(60);
    assert!(matches!(range, TimeRange::LastSeconds(60)));

    let range = TimeRange::Range {
        start: 1000,
        end: 2000,
    };
    assert!(matches!(
        range,
        TimeRange::Range {
            start: 1000,
            end: 2000
        }
    ));
}

#[test]
fn test_time_series_new() {
    // No series: no legend and no points, but the grid and the value
    // labels of the default range 0..100, padded by 10%
    let chart = TimeSeries::new();
    let rows = render_rows(&chart, 60, 20);
    assert_eq!(
        rows[0].split_whitespace().next(),
        Some("110.0"),
        "{rows:#?}"
    );
    assert!(rows[0].contains('┄'));
    assert!(!rows.iter().any(|r| r.contains('●')));
}

#[test]
fn test_time_series_default() {
    assert_eq!(
        render_rows(&TimeSeries::default().time_range(fixed_range()), 60, 20),
        render_rows(&TimeSeries::new().time_range(fixed_range()), 60, 20)
    );
}

#[test]
fn test_time_series_title() {
    let chart = TimeSeries::new().title("CPU Usage");
    let rows = render_rows(&chart, 40, 20);
    // Centered: (40 - 9) / 2
    assert_eq!(rows[0].find("CPU Usage"), Some(15), "{:?}", rows[0]);
}

#[test]
fn test_time_series_show_grid() {
    let rows = render_rows(&TimeSeries::new().series(two_points()), 40, 12);
    assert!(rows.iter().any(|r| r.contains('┄')));

    let chart = TimeSeries::new().series(two_points()).show_grid(false);
    let rows = render_rows(&chart, 40, 12);
    assert!(!rows.iter().any(|r| r.contains('┄')), "{rows:#?}");
    assert_eq!(super::count_char(&render(&chart, 40, 12), '●'), 2);
}

#[test]
fn test_time_series_show_legend() {
    let rows = render_rows(&TimeSeries::new().series(two_points()), 40, 12);
    assert_eq!(rows[0].trim(), "─ Data");

    let chart = TimeSeries::new().series(two_points()).show_legend(false);
    let rows = render_rows(&chart, 40, 12);
    assert!(!rows.iter().any(|r| r.contains("Data")), "{rows:#?}");
}

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
fn test_time_series_bg() {
    let chart = TimeSeries::new().series(two_points()).bg(Color::BLACK);
    let buffer = render(&chart, 40, 12);
    // The background covers the whole chart, under the text too
    for y in [0, 11] {
        for x in [0, 20, 39] {
            assert_eq!(
                buffer.get(x, y).unwrap().bg,
                Some(Color::BLACK),
                "({x}, {y})"
            );
        }
    }
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
fn test_time_series_height() {
    let chart = TimeSeries::new().series(two_points()).height(8);
    let rows = render_rows(&chart, 40, 12);
    // The time labels end the 8 rows; nothing below
    assert!(rows[7].contains(':'), "{rows:#?}");
    assert!(rows[8..].iter().all(|r| r.trim().is_empty()), "{rows:#?}");
}

#[test]
fn test_time_series_helper() {
    assert_eq!(
        render_rows(&time_series().time_range(fixed_range()), 60, 20),
        render_rows(&TimeSeries::new().time_range(fixed_range()), 60, 20)
    );
}

#[test]
fn test_cpu_chart_helper() {
    let chart = cpu_chart(vec![(1000, 50.0), (2000, 60.0)]);
    let rows = render_rows(&chart, 40, 12);
    assert_eq!(rows[0].trim(), "CPU Usage");
    assert_eq!(rows[1].trim(), "─ CPU");
    assert_eq!(rows[2].trim(), "%");
    // y_range is set to 0..100
    let (min, max) = chart.get_value_bounds();
    assert_eq!(min, 0.0);
    assert_eq!(max, 100.0);
}

#[test]
fn test_memory_chart_helper() {
    let chart = memory_chart(vec![(1000, 4.0), (2000, 5.0)]);
    let rows = render_rows(&chart, 40, 12);
    assert_eq!(rows[0].trim(), "Memory Usage");
    assert_eq!(rows[1].trim(), "─ Memory");
    assert_eq!(rows[2].trim(), "GB");
}

#[test]
fn test_network_chart_helper() {
    let chart = network_chart(vec![(1000, 100.0)], vec![(1000, 50.0)]);
    let rows = render_rows(&chart, 40, 12);
    assert_eq!(rows[0].trim(), "Network Traffic");
    assert_eq!(rows[1].trim(), "─ RX   ─ TX");
    assert_eq!(rows[2].trim(), "Mbps");
    // Two series: RX and TX
    let (min, max) = chart.get_value_bounds();
    assert!(min <= 50.0);
    assert!(max >= 100.0);
}

// ========================================================================
// Render tests
// ========================================================================

#[test]
fn test_time_series_render() {
    let chart = TimeSeries::new().title("Test").series(
        TimeSeriesData::new("Data")
            .point(1000, 50.0)
            .point(2000, 75.0),
    );
    let rows = render_rows(&chart, 60, 20);
    assert_eq!(rows[0].trim(), "Test");
    assert_eq!(rows[1].trim(), "─ Data");
    // The two points, joined by a line
    assert_eq!(super::count_char(&render(&chart, 60, 20), '●'), 2);
    assert!(rows[2..18].iter().any(|r| r.contains("──")), "{rows:#?}");
}

#[test]
fn test_time_series_render_small_area() {
    let chart = TimeSeries::new().series(TimeSeriesData::new("Data").point(1000, 50.0));
    assert!(render_rows(&chart, 5, 3)
        .iter()
        .all(|r| r.trim().is_empty()));
    // The smallest area that draws: 10 columns, 5 rows
    assert!(render_rows(&chart, 9, 5)
        .iter()
        .all(|r| r.trim().is_empty()));
    assert!(render_rows(&chart, 10, 4)
        .iter()
        .all(|r| r.trim().is_empty()));
    assert!(render_rows(&chart, 40, 12).iter().any(|r| r.contains('●')));
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

#[test]
fn test_time_series_render_multiple_series() {
    let chart = TimeSeries::new()
        .series(TimeSeriesData::new("Series1").point(1000, 50.0))
        .series(
            TimeSeriesData::new("Series2")
                .point(1000, 75.0)
                .color(Color::RED),
        );
    let buffer = render(&chart, 60, 20);
    assert_eq!(super::rows(&buffer)[0].trim(), "─ Series1   ─ Series2");
    let points: Vec<_> = (0..20)
        .flat_map(|y| (0..60).map(move |x| (x, y)))
        .filter_map(|(x, y)| buffer.get(x, y))
        .filter(|c| c.symbol == '●')
        .map(|c| c.fg)
        .collect();
    assert_eq!(points, [Some(Color::RED), Some(Color::CYAN)]);
}

#[test]
fn test_time_series_render_step_line() {
    let chart = TimeSeries::new().series(
        TimeSeriesData::new("Data")
            .line_style(TimeLineStyle::Step)
            .point(1000, 50.0)
            .point(2000, 75.0),
    );
    let rows = render_rows(&chart, 40, 12);
    assert_eq!(rows[0].trim(), "┐ Data");
    // Flat at the first value, then straight up at the second timestamp
    assert!(rows[9].contains("●────"), "{rows:#?}");
    let riser: String = rows[3..10]
        .iter()
        .map(|r| r.chars().nth(38).unwrap())
        .collect();
    assert_eq!(riser, "│││││││");
}

#[test]
fn test_time_series_render_filled() {
    let chart = TimeSeries::new().series(
        TimeSeriesData::new("Data")
            .filled()
            .point(1000, 50.0)
            .point(2000, 75.0),
    );
    let buffer = render(&chart, 40, 12);
    // The area under the line takes a dark shade of the series color
    let shade = Some(Color::rgb(0, 76, 76));
    assert_eq!(buffer.get(37, 9).unwrap().bg, shade);
    assert_eq!(buffer.get(9, 9).unwrap().bg, shade);
    assert_eq!(buffer.get(9, 2).unwrap().bg, None);
}

mod snapshots {

    use revue::testing::{Pilot, TestApp, TestConfig};

    #[test]
    fn test_timeseries_basic() {
        use revue::style::Color;
        use revue::widget::{TimeSeries, TimeSeriesData};

        let data = TimeSeriesData::new("CPU")
            .point(0, 25.0)
            .point(1, 45.0)
            .point(2, 30.0)
            .point(3, 60.0)
            .point(4, 55.0)
            .color(Color::CYAN);

        let view = TimeSeries::new()
            .title("CPU Usage")
            .series(data)
            .show_legend(true);

        let config = TestConfig::with_size(60, 15);
        let mut app = TestApp::with_config(view, config);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("timeseries_basic");
    }

    #[test]
    fn test_timeseries_multiple_series() {
        use revue::style::Color;
        use revue::widget::{TimeSeries, TimeSeriesData};

        let cpu = TimeSeriesData::new("CPU")
            .points(vec![(0, 20.0), (1, 40.0), (2, 35.0), (3, 50.0)])
            .color(Color::CYAN);

        let memory = TimeSeriesData::new("Memory")
            .points(vec![(0, 60.0), (1, 65.0), (2, 70.0), (3, 68.0)])
            .color(Color::MAGENTA);

        let view = TimeSeries::new()
            .title("System Metrics")
            .series(cpu)
            .series(memory)
            .y_label("Usage %")
            .show_grid(true);

        let config = TestConfig::with_size(70, 18);
        let mut app = TestApp::with_config(view, config);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("timeseries_multi");
    }
}
