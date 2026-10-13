//! ScatterChart mark tests: markers, colors and bubbles

use super::super::{count_char, render};
use super::bare;
use revue::style::Color;
use revue::widget::data::chart::{ColorScheme, ScatterSeries};

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
