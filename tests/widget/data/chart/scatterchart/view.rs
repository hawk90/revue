//! ScatterChart frame tests: title, legend, grid, constructors and small areas

use super::super::{count_char, render, render_rows, rows};
use super::bare;
use revue::widget::data::chart::{
    bubble_chart, scatter_chart, Axis, ChartGrid, Legend, ScatterChart, ScatterSeries,
};

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
fn test_scatter_legend_bottom_left() {
    // 50x25: the legend box sits one cell in from the left edge, its bottom
    // border on the row above the x labels
    let chart = ScatterChart::new()
        .grid(ChartGrid::new())
        .series(ScatterSeries::new("Series A").points(&[(1.0, 1.0), (2.0, 2.0)]))
        .legend(Legend::bottom_left());
    let rows = render_rows(&chart, 50, 25);
    assert!(rows[21].starts_with(" ┌──────────┐ "), "{:?}", rows[21]);
    assert!(rows[22].starts_with(" │■ Series A│ "), "{:?}", rows[22]);
    assert!(rows[23].starts_with(" └──────────┘ "), "{:?}", rows[23]);
}

#[test]
fn test_scatter_grid_y_only() {
    // Horizontal lines only: no vertical lines or tick marks
    let chart = ScatterChart::new()
        .no_legend()
        .series(ScatterSeries::new("D").points(&[(1.0, 1.0), (2.0, 2.0)]))
        .grid(ChartGrid::y_only());
    let buffer = render(&chart, 40, 20);
    assert!(count_char(&buffer, '─') > 0);
    assert_eq!(count_char(&buffer, '│'), 0);
    assert_eq!(count_char(&buffer, '┴'), 0);
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

/// Tick counts the axis takes as given: 0, and more ticks than columns
/// (#862)
#[test]
fn test_scatter_any_tick_count_draws() {
    for ticks in [0, 1, 1_000] {
        let chart = ScatterChart::new()
            .grid(ChartGrid::both())
            .x_axis(Axis::new().ticks(ticks))
            .y_axis(Axis::new().ticks(ticks))
            .series(ScatterSeries::new("s").points(&[(0.0, 0.0), (10.0, 10.0)]));
        let buffer = render(&chart, 40, 12);
        assert!(count_char(&buffer, '0') > 0, "ticks {ticks}: no labels");
    }
}
