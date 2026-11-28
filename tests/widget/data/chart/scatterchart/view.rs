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

/// A 30x12 chart with horizontal grid lines and Y labels of `format` (in
/// red) on the bounds 0..1, padded to -0.05..1.05
fn y_labeled(format: &str) -> revue::render::Buffer {
    use revue::style::Color;
    use revue::widget::data::chart::AxisFormat;
    let chart = bare()
        .grid(ChartGrid::y_only())
        .x_axis(Axis::new().ticks(1))
        .y_axis(
            Axis::new()
                .bounds(0.0, 1.0)
                .ticks(1)
                .color(Color::RED)
                .format(AxisFormat::Custom(format.into())),
        );
    render(&chart, 30, 12)
}

/// The Y label column is as wide as the widest label, at most a third of
/// the area, and the plot starts after it and a gap column (#1614)
#[test]
fn test_scatter_y_label_column_fits_the_widest_label() {
    // "-0.05" and "1.05": 5 columns, the plot (its first grid line) at 6
    let rows = rows(&y_labeled("{}"));
    assert_eq!(rows[0], format!("{}├{} ", " ".repeat(6), "─".repeat(22)));
    assert_eq!(rows[1], format!("1.05{}", " ".repeat(26)));
}

#[test]
fn test_scatter_y_labels_past_a_third_are_cut_with_an_ellipsis() {
    use revue::style::Color;
    // "1.05xxxxxxxxxx" is 14 columns: cut to the 10 of the column, the
    // plot at 11
    let buffer = y_labeled("{}xxxxxxxxxx");
    let lines = rows(&buffer);
    assert_eq!(lines[0], format!("{}├{} ", " ".repeat(11), "─".repeat(17)));
    assert_eq!(lines[1], format!("1.05xxxxx…{}", " ".repeat(20)));
    for x in 0..10 {
        assert_eq!(buffer.get(x, 1).unwrap().fg, Some(Color::RED), "x={x}");
    }
    // Wide glyphs stay whole: "1.05가나다라" is 12 columns, cut to 9
    // (`rows` shows the right half of a wide glyph as '\0')
    let lines = rows(&y_labeled("{}가나다라"));
    assert_eq!(lines[0], format!("{}├{} ", " ".repeat(11), "─".repeat(17)));
    assert_eq!(lines[1], format!("1.05가\0나\0…{}", " ".repeat(21)));
}

/// A huge tick count draws one grid line and Y label per row and an X
/// label every other column, instead of looping once per tick (#1615,
/// #1616)
#[test]
fn test_scatter_huge_tick_counts_are_capped_at_the_cells() {
    use revue::widget::data::chart::AxisFormat;
    let chart = bare()
        .grid(ChartGrid::both())
        .x_axis(
            Axis::new()
                .ticks(usize::MAX)
                .format(AxisFormat::Custom("x".into())),
        )
        .y_axis(
            Axis::new()
                .ticks(usize::MAX)
                .format(AxisFormat::Custom("y".into())),
        );
    let rows = rows(&render(&chart, 30, 12));
    // Rows 1..=10 hold a Y label each; the plot is columns 2..29 of rows
    // 0..10, a grid line on every row of it
    assert_eq!(rows[0], format!("  ├{} ", "─".repeat(26)));
    for (y, row) in rows[1..10].iter().enumerate() {
        assert_eq!(*row, format!("y ├{} ", "─".repeat(26)), "row {}", y + 1);
    }
    assert_eq!(rows[10], format!("y{}", " ".repeat(29)));
    assert_eq!(rows[11], format!("y {}", "x ".repeat(14)));
}
