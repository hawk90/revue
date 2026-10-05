//! ScatterChart rendering tests
//!
//! Builder/bounds tests live in the in-source tests of scatterchart.rs;
//! these check what the chart draws.

use super::{count_char, render, render_rows, rows};
use revue::style::Color;
use revue::widget::data::chart::{
    bubble_chart, scatter_chart, Axis, ChartGrid, ColorScheme, Legend, Marker, ScatterChart,
    ScatterSeries,
};

/// A chart with no grid and no legend, so only points and labels are drawn.
fn bare() -> ScatterChart {
    ScatterChart::new().grid(ChartGrid::new()).no_legend()
}

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
fn test_scatter_default_marker_is_filled_circle() {
    let chart = bare().series(ScatterSeries::new("S").points(&[(0.0, 0.0), (10.0, 10.0)]));
    let buffer = render(&chart, 40, 20);
    assert_eq!(count_char(&buffer, '●'), 2);
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
fn test_scatter_title() {
    let chart = bare()
        .title("My Scatter")
        .series(ScatterSeries::new("D").points(&[(1.0, 1.0)]));
    let rows = render_rows(&chart, 40, 20);
    // Centered: (40 - 10) / 2 = 15
    assert_eq!(rows[0].trim_end(), format!("{}My Scatter", " ".repeat(15)));
}

#[test]
fn test_scatter_legend_lists_series() {
    let chart = ScatterChart::new()
        .grid(ChartGrid::new())
        .series(ScatterSeries::new("Series A").points(&[(1.0, 1.0), (2.0, 2.0)]))
        .series(ScatterSeries::new("Series B").points(&[(3.0, 3.0), (4.0, 4.0)]))
        .legend(Legend::top_right());
    let text = render_rows(&chart, 50, 25).join("\n");
    assert!(text.contains("■ Series A"), "{text}");
    assert!(text.contains("■ Series B"), "{text}");
}

#[test]
fn test_scatter_no_legend() {
    let chart = ScatterChart::new()
        .series(ScatterSeries::new("Series A").points(&[(1.0, 1.0)]))
        .no_legend();
    let text = render_rows(&chart, 50, 25).join("\n");
    assert!(!text.contains("Series A"));
    assert!(!text.contains('■'));
}

#[test]
fn test_scatter_grid() {
    let series = || ScatterSeries::new("D").points(&[(5.0, 5.0)]);
    let with_grid = render(
        &ScatterChart::new()
            .no_legend()
            .series(series())
            .grid(ChartGrid::both()),
        40,
        20,
    );
    let without = render(&bare().series(series()), 40, 20);
    assert!(count_char(&with_grid, '│') > 0);
    assert!(count_char(&with_grid, '─') > 0);
    assert_eq!(count_char(&without, '│'), 0);
    assert_eq!(count_char(&without, '┴'), 0);
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
fn test_scatter_helpers_match_new() {
    let series = || ScatterSeries::new("D").points(&[(1.0, 2.0), (3.0, 4.0)]);
    let base = render_rows(&ScatterChart::new().series(series()), 40, 20);
    assert_eq!(render_rows(&scatter_chart().series(series()), 40, 20), base);
    assert_eq!(render_rows(&bubble_chart().series(series()), 40, 20), base);
}

#[test]
fn test_scatter_small_area_renders_nothing() {
    let chart = ScatterChart::new().series(ScatterSeries::new("D").points(&[(1.0, 1.0)]));
    let buffer = render(&chart, 14, 20);
    assert!(rows(&buffer).iter().all(|r| r.trim().is_empty()));
    let buffer = render(&chart, 40, 7);
    assert!(rows(&buffer).iter().all(|r| r.trim().is_empty()));
}

#[test]
fn test_scatter_empty_draws_no_points() {
    let buffer = render(&bare(), 30, 15);
    assert_eq!(count_char(&buffer, '●'), 0);
}
