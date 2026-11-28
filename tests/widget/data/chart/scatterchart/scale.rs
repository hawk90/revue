//! ScatterChart value range and scale tests: padded bounds, axis overrides, non-finite points

use super::super::{count_char, render};
use super::bare;
use revue::widget::data::chart::{Axis, Marker, ScatterSeries};

#[test]
fn test_scatter_points_are_placed_on_the_padded_scale() {
    // 40x20: chart area x=6 (after the Y labels, "-0.50" the widest, and a
    // gap column), y=0, 33x18. Bounds pad the data by 5%: -0.5..10.5, so 0
    // is 1.45 of the 32 columns past the first and 16.2 of the 17 rows
    // down from the top (#1622)
    let chart = bare().series(
        ScatterSeries::new("S")
            .points(&[(0.0, 0.0), (10.0, 10.0)])
            .marker(Marker::Star),
    );
    let buffer = render(&chart, 40, 20);
    assert_eq!(count_char(&buffer, '★'), 2);
    assert_eq!(buffer.get(7, 16).unwrap().symbol, '★');
    assert_eq!(buffer.get(36, 0).unwrap().symbol, '★');
}

#[test]
fn test_scatter_axis_bounds_override() {
    // x: 0..100 -> -5..105, y: 0..50 -> -2.5..52.5; the plot starts at
    // column 5, after the 4-column Y labels: 50 and 25 are halfway, 16.5
    // of the 33 columns and 8.5 of the 17 rows
    let chart = bare()
        .series(
            ScatterSeries::new("S")
                .points(&[(50.0, 25.0)])
                .marker(Marker::Star),
        )
        .x_axis(Axis::new().bounds(0.0, 100.0))
        .y_axis(Axis::new().bounds(0.0, 50.0));
    let buffer = render(&chart, 40, 20);
    assert_eq!(buffer.get(21, 8).unwrap().symbol, '★');
}

#[test]
fn test_scatter_non_finite_points_are_skipped() {
    let chart = bare().series(ScatterSeries::new("S").data(vec![
        (1.0, 1.0),
        (f64::NAN, 2.0),
        (3.0, f64::INFINITY),
        (4.0, 4.0),
    ]));
    assert_eq!(count_char(&render(&chart, 40, 20), '●'), 2);
}
