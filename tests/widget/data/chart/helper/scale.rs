//! Chart value range and scale tests: bounds, non-finite points, clipping to the plot

use super::super::{count_char, render, rows};
use super::{flat, plain, plot_line_cells, sym};
use revue::style::Color;
use revue::widget::data::chart::{Axis, Marker, Series};

#[test]
fn test_chart_empty_uses_unit_bounds() {
    let buffer = render(&plain(), 40, 20);
    // X runs 0..1; Y runs 0..1 padded by 5%
    assert_eq!(sym(&buffer, 8, 18), '0');
    assert_eq!(sym(&buffer, 39, 18), '1');
    let rows = rows(&buffer);
    assert!(rows[17].starts_with("  -0.05"), "{:?}", rows[17]);
    assert!(rows[0].starts_with("   1.05"), "{:?}", rows[0]);
}

#[test]
fn test_chart_identical_values_draw_flat_line() {
    // A zero Y range is widened to +-0.5 around the value: row 9
    let buffer = render(&plain().series(flat()), 40, 20);
    for x in 8..40 {
        assert_eq!(sym(&buffer, x, 9), '─', "x={x}");
    }
    assert_eq!(plot_line_cells(&buffer), 32);
}

#[test]
fn test_chart_single_point_marker_is_centered() {
    let s = Series::new("P")
        .data(vec![(5.0, 10.0)])
        .marker(Marker::Star);
    let buffer = render(&plain().series(s), 40, 20);
    assert_eq!(count_char(&buffer, '★'), 1);
    assert_eq!(sym(&buffer, 23, 9), '★');
    assert_eq!(plot_line_cells(&buffer), 0);
}

#[test]
fn test_chart_nan_points_are_skipped() {
    let s = Series::new("Data")
        .data(vec![
            (1.0, 2.0),
            (f64::NAN, 3.0),
            (3.0, f64::NAN),
            (4.0, 5.0),
        ])
        .marker(Marker::Star);
    let buffer = render(&plain().series(s), 40, 20);
    // Only the two finite points get markers: (1,2) -> (8,17), (4,5) -> (39,1)
    assert_eq!(count_char(&buffer, '★'), 2);
    assert_eq!(sym(&buffer, 8, 17), '★');
    assert_eq!(sym(&buffer, 39, 1), '★');
    // ... joined by a single rising line
    assert!(rows(&buffer).iter().all(|r| !r.contains('╲')));
}

#[test]
fn test_chart_infinite_points_are_skipped() {
    let s = Series::new("Data")
        .data(vec![
            (1.0, 2.0),
            (f64::INFINITY, 3.0),
            (3.0, f64::NEG_INFINITY),
            (4.0, 5.0),
        ])
        .marker(Marker::Star);
    let buffer = render(&plain().series(s), 40, 20);
    assert_eq!(count_char(&buffer, '★'), 2);
    assert_eq!(sym(&buffer, 8, 17), '★');
    assert_eq!(sym(&buffer, 39, 1), '★');
}

#[test]
fn test_chart_points_outside_axis_bounds_are_clipped() {
    // Fixed bounds 0..10 with data reaching past both ends
    let s = Series::new("Data")
        .data(vec![(-5.0, -5.0), (5.0, 5.0), (20.0, 20.0)])
        .marker(Marker::Star);
    let chart = plain()
        .series(s)
        .x_axis(Axis::new().bounds(0.0, 10.0).grid(false))
        .y_axis(Axis::new().bounds(0.0, 10.0).grid(false));
    let buffer = render(&chart, 40, 20);
    // Only (5, 5) is inside the plot: x = 8 + 5/10*31, y = 17 - 5/10*17
    assert_eq!(count_char(&buffer, '★'), 1);
    assert_eq!(sym(&buffer, 23, 9), '★');
    // The line through it is cut off at the plot edges, not dropped: it
    // runs from the bottom-left corner to the top-right corner
    assert_eq!(sym(&buffer, 8, 17), '╱');
    assert_eq!(sym(&buffer, 39, 0), '╱');
}

#[test]
fn test_chart_segment_crossing_the_plot_is_clipped() {
    // Both ends outside the bounds; the segment still crosses the plot
    let s = Series::new("Data").data(vec![(-10.0, 5.0), (20.0, 5.0)]);
    let chart = plain()
        .series(s)
        .x_axis(Axis::new().bounds(0.0, 10.0).grid(false))
        .y_axis(Axis::new().bounds(0.0, 10.0).grid(false));
    let buffer = render(&chart, 40, 20);
    let row: String = rows(&buffer)[9].chars().skip(8).collect();
    assert_eq!(row, "─".repeat(32));
}

#[test]
fn test_chart_segment_missing_the_plot_draws_nothing() {
    // Entirely above the plot, with an area fill and step series too
    let chart = plain()
        .series(Series::new("L").data(vec![(0.0, 50.0), (10.0, 60.0)]))
        .series(
            Series::new("A")
                .data(vec![(0.0, 50.0), (10.0, 60.0)])
                .area(Color::BLUE),
        )
        .series(
            Series::new("S")
                .data(vec![(0.0, 50.0), (10.0, 60.0)])
                .step(),
        )
        .x_axis(Axis::new().bounds(0.0, 10.0).grid(false))
        .y_axis(Axis::new().bounds(0.0, 10.0).grid(false));
    let buffer = render(&chart, 40, 20);
    for y in 0..18 {
        for x in 8..40 {
            assert_eq!(sym(&buffer, x, y), ' ', "({x}, {y})");
        }
    }
}

#[test]
fn test_chart_axis_bounds_override() {
    // Both bounds fixed, so no padding: (5, 10) -> x = 8 + 5/20*31, y = 17 - 10/30*17
    let chart = plain()
        .series(
            Series::new("P")
                .data(vec![(5.0, 10.0)])
                .marker(Marker::Star),
        )
        .x_axis(Axis::new().bounds(0.0, 20.0).grid(false))
        .y_axis(Axis::new().bounds(0.0, 30.0).grid(false));
    let buffer = render(&chart, 40, 20);
    assert_eq!(sym(&buffer, 15, 12), '★');
    let rows = rows(&buffer);
    assert!(rows[17].starts_with("      0"), "{:?}", rows[17]);
    assert!(rows[0].starts_with("     30"), "{:?}", rows[0]);
    assert!(rows[18].trim_end().ends_with("20"), "{:?}", rows[18]);
}
