//! ScatterChart value range and scale tests: padded bounds, axis overrides, non-finite points

use super::super::{count_char, render};
use super::bare;
use revue::widget::data::chart::{Axis, Marker, ScatterSeries};

#[test]
fn test_scatter_points_are_placed_on_the_padded_scale() {
    // 40x20: chart area x=8, y=0, 31x18. Bounds pad the data by 5%.
    let chart = bare().series(
        ScatterSeries::new("S")
            .points(&[(0.0, 0.0), (10.0, 10.0)])
            .marker(Marker::Star),
    );
    let buffer = render(&chart, 40, 20);
    assert_eq!(count_char(&buffer, '★'), 2);
    assert_eq!(buffer.get(9, 17).unwrap().symbol, '★');
    assert_eq!(buffer.get(36, 1).unwrap().symbol, '★');
}

#[test]
fn test_scatter_axis_bounds_override() {
    // x: 0..100 -> -5..105, y: 0..50 -> -2.5..52.5
    let chart = bare()
        .series(
            ScatterSeries::new("S")
                .points(&[(50.0, 25.0)])
                .marker(Marker::Star),
        )
        .x_axis(Axis::new().bounds(0.0, 100.0))
        .y_axis(Axis::new().bounds(0.0, 50.0));
    let buffer = render(&chart, 40, 20);
    assert_eq!(buffer.get(23, 9).unwrap().symbol, '★');
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
