//! ScatterChart mark tests: markers, colors and bubbles

use super::super::{count_char, render};
use super::bare;
use proptest::prelude::*;
use revue::style::Color;
use revue::widget::data::chart::{ColorScheme, Marker, ScatterSeries};

#[test]
fn test_scatter_default_marker_is_filled_circle() {
    let chart = bare().series(ScatterSeries::new("S").points(&[(0.0, 0.0), (10.0, 10.0)]));
    let buffer = render(&chart, 40, 20);
    assert_eq!(count_char(&buffer, '●'), 2);
}

#[test]
fn test_scatter_series_colors() {
    let chart = bare()
        .series(
            ScatterSeries::new("A")
                .points(&[(0.0, 0.0)])
                .color(Color::RED),
        )
        .series(ScatterSeries::new("B").points(&[(10.0, 10.0)]))
        .colors(ColorScheme::new(vec![Color::GREEN, Color::BLUE]));
    let buffer = render(&chart, 40, 20);
    assert_eq!(buffer.get(9, 17).unwrap().fg, Some(Color::RED));
    assert_eq!(buffer.get(36, 1).unwrap().fg, Some(Color::BLUE));
}

#[test]
fn test_scatter_bubbles_scale_with_size() {
    // radius = sqrt(size / 100) * 2: 100 -> 2x2 cells, 400 -> 4x4 cells
    let chart = bare().series(
        ScatterSeries::new("Bubbles")
            .points(&[(0.0, 0.0), (10.0, 10.0)])
            .sizes(vec![100.0, 400.0]),
    );
    let buffer = render(&chart, 40, 20);
    assert_eq!(count_char(&buffer, '●'), 4 + 16);
}

#[test]
fn test_scatter_small_bubbles_still_draw_one_cell() {
    let chart = bare().series(
        ScatterSeries::new("Bubbles")
            .points(&[(5.0, 5.0), (10.0, 10.0)])
            .sizes(vec![1.0, 5.0]),
    );
    assert_eq!(count_char(&render(&chart, 40, 20), '●'), 2);
}

#[test]
fn test_scatter_empty_draws_no_points() {
    let buffer = render(&bare(), 30, 15);
    assert_eq!(count_char(&buffer, '●'), 0);
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// Any finite points with auto bounds: at least one marker is drawn and
    /// every marker sits inside the chart area (columns 8..w-1, rows ..h-2)
    #[test]
    fn test_scatter_points_land_inside_the_chart_area(
        points in prop::collection::vec((-1e6f64..1e6, -1e6f64..1e6), 1..20),
        w in 15u16..80,
        h in 8u16..30,
    ) {
        let chart = bare().series(ScatterSeries::new("s").data(points).marker(Marker::Star));
        let buffer = render(&chart, w, h);
        let mut stars = 0;
        for y in 0..h {
            for x in 0..w {
                if buffer.get(x, y).unwrap().symbol == '★' {
                    prop_assert!(x >= 8 && x < w - 1 && y < h - 2, "point at ({}, {}) in {}x{}", x, y, w, h);
                    stars += 1;
                }
            }
        }
        prop_assert!(stars >= 1);
    }
}
