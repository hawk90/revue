//! TimeSeries series tests: points, solid and step lines, fills and several series

use super::super::{render, render_rows};
use proptest::prelude::*;
use revue::style::Color;
use revue::widget::data::chart::timeseries::{TimeLineStyle, TimeSeries, TimeSeriesData};

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
    assert_eq!(super::super::count_char(&render(&chart, 60, 20), '●'), 2);
    assert!(rows[2..18].iter().any(|r| r.contains("──")), "{rows:#?}");
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
    assert_eq!(
        super::super::rows(&buffer)[0].trim(),
        "─ Series1   ─ Series2"
    );
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

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// Every point lands in the plot: right of the 8-column value labels,
    /// below the legend row and above the time labels.
    #[test]
    fn test_time_series_points_stay_in_the_plot(
        points in prop::collection::vec((0u64..1_000_000, -1e6f64..1e6), 1..30),
        w in 15u16..80,
        h in 7u16..30,
    ) {
        let chart = TimeSeries::new().series(TimeSeriesData::new("s").points(points));
        let buffer = render(&chart, w, h);
        for y in 0..h {
            for x in 0..w {
                if buffer.get(x, y).unwrap().symbol == '●' {
                    prop_assert!((8..w - 1).contains(&x) && (1..h - 2).contains(&y), "point at ({x}, {y})");
                }
            }
        }
    }
}
