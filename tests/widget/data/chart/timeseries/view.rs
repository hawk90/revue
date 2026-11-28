//! TimeSeries layout tests: title, legend, unit label, background, height and small areas

use super::super::{render, render_at, render_rows, rows};
use super::{fixed_range, two_points};
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::data::chart::timeseries::{
    TimeLineStyle, TimeMarker, TimeSeries, TimeSeriesData,
};
use revue::widget::traits::{RenderContext, View};

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
fn test_time_series_show_legend() {
    let rows = render_rows(&TimeSeries::new().series(two_points()), 40, 12);
    assert_eq!(rows[0].trim(), "─ Data");

    let chart = TimeSeries::new().series(two_points()).show_legend(false);
    let rows = render_rows(&chart, 40, 12);
    assert!(!rows.iter().any(|r| r.contains("Data")), "{rows:#?}");
}

/// A name near 65535 columns does not overflow the legend's column: the
/// legend stops at the area's edge and later series are left out (#1571)
#[test]
fn test_time_series_legend_name_near_u16_max() {
    let long = TimeSeriesData::new("x".repeat(65_540));
    let chart = TimeSeries::new()
        .time_range(fixed_range())
        .series(long)
        .series(TimeSeriesData::new("B").line_style(TimeLineStyle::Dashed));
    let rows = render_rows(&chart, 40, 12);
    assert_eq!(rows[0], format!("  ─ {}", "x".repeat(36)));

    // Two short entries, the second cut at the last column
    let chart = TimeSeries::new()
        .time_range(fixed_range())
        .series(TimeSeriesData::new("Alpha"))
        .series(TimeSeriesData::new("Beta").line_style(TimeLineStyle::Dashed));
    let rows = render_rows(&chart, 16, 12);
    assert_eq!(rows[0], "  ─ Alpha   ╌ Be");
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
fn test_time_series_height() {
    let chart = TimeSeries::new().series(two_points()).height(8);
    let rows = render_rows(&chart, 40, 12);
    // The time labels end the 8 rows; nothing below
    assert!(rows[7].contains(':'), "{rows:#?}");
    assert!(rows[8..].iter().all(|r| r.trim().is_empty()), "{rows:#?}");
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
fn test_time_series_zero_area_draws_nothing() {
    let chart = TimeSeries::new().title("T").series(two_points());
    super::super::assert_zero_areas_draw_nothing(&chart);
}

#[test]
fn test_time_series_draws_the_same_when_offset() {
    let chart = TimeSeries::new()
        .title("T")
        .y_label("%")
        .series(two_points().filled())
        .series(
            TimeSeriesData::new("Step")
                .point(1200, 60.0)
                .point(1800, 55.0)
                .line_style(TimeLineStyle::Step),
        )
        .marker(TimeMarker::new(1500, "Event"))
        .bg(Color::BLACK);
    super::super::assert_same_when_offset(chart, 40, 16);
}

/// A title of 65536 columns is not measured as 0 and centered at the
/// middle: it fills the first row from the left edge, inside an area at an
/// offset (#1596)
#[test]
fn test_time_series_title_wider_than_u16_fills_the_first_row() {
    let chart = TimeSeries::new()
        .time_range(fixed_range())
        .title("x".repeat(65_536))
        .bg(Color::BLUE);
    let buffer = render_at(&chart, Rect::new(3, 2, 30, 12));
    let rows = rows(&buffer);
    assert_eq!(rows[2], format!("   {}  ", "x".repeat(30)));
    for x in 3..33 {
        assert_eq!(buffer.get(x, 2).unwrap().bg, Some(Color::BLUE), "{x}");
    }
    assert!(rows[..2].iter().all(|r| r.trim().is_empty()), "{rows:#?}");
}
