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
    // border on the row above the x labels. (Column 0 of row 22 holds the
    // first glyph of the bottom Y label, "0.95", which the box does not
    // cover.)
    let chart = ScatterChart::new()
        .grid(ChartGrid::new())
        .series(ScatterSeries::new("Series A").points(&[(1.0, 1.0), (2.0, 2.0)]))
        .legend(Legend::bottom_left());
    let rows = render_rows(&chart, 50, 25);
    let from_1 = |y: usize| -> String { rows[y].chars().skip(1).collect() };
    assert!(from_1(21).starts_with("┌──────────┐ "), "{:?}", rows[21]);
    assert!(from_1(22).starts_with("│■ Series A│ "), "{:?}", rows[22]);
    assert!(from_1(23).starts_with("└──────────┘ "), "{:?}", rows[23]);
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
    // "-0.05" and "1.05": 5 columns, the plot (its grid lines) at 6, each
    // label on its grid line's row: the top and bottom of the plot (#1622)
    let rows = rows(&y_labeled("{}"));
    assert_eq!(rows[0], format!("1.05  ├{} ", "─".repeat(22)));
    for (y, row) in rows.iter().enumerate().take(9).skip(1) {
        assert_eq!(*row, " ".repeat(30), "row {y}");
    }
    assert_eq!(rows[9], format!("-0.05 ├{} ", "─".repeat(22)));
}

#[test]
fn test_scatter_y_labels_past_a_third_are_cut_with_an_ellipsis() {
    use revue::style::Color;
    // "1.05xxxxxxxxxx" is 14 columns: cut to the 10 of the column, the
    // plot at 11
    let buffer = y_labeled("{}xxxxxxxxxx");
    let lines = rows(&buffer);
    assert_eq!(lines[0], format!("1.05xxxxx… ├{} ", "─".repeat(17)));
    assert_eq!(lines[9], format!("-0.05xxxx… ├{} ", "─".repeat(17)));
    for x in 0..10 {
        assert_eq!(buffer.get(x, 0).unwrap().fg, Some(Color::RED), "x={x}");
        assert_eq!(buffer.get(x, 9).unwrap().fg, Some(Color::RED), "x={x}");
    }
    // Wide glyphs stay whole: "1.05가나다라" is 12 columns, cut to 9
    // (`rows` shows the right half of a wide glyph as '\0')
    let lines = rows(&y_labeled("{}가나다라"));
    assert_eq!(lines[0], format!("1.05가\0나\0…  ├{} ", "─".repeat(17)));
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
    // The plot is columns 2..29 of rows 0..10, a grid line and a Y label
    // on every row of it; X labels every other column, the last on the
    // plot's last column (#1622)
    for (y, row) in rows[..10].iter().enumerate() {
        assert_eq!(*row, format!("y ├{} ", "─".repeat(26)), "row {y}");
    }
    assert_eq!(rows[10], " ".repeat(30));
    assert_eq!(rows[11], format!("  {}", "x ".repeat(14)));
}

/// The two points (0, 0) and (10, 10) on the padded bounds -0.5..10.5,
/// with both grid lines, a red Y axis and a green X axis
fn two_points() -> ScatterChart {
    use revue::style::Color;
    use revue::widget::data::chart::Marker;
    ScatterChart::new()
        .no_legend()
        .grid(ChartGrid::both())
        .x_axis(Axis::new().color(Color::GREEN))
        .y_axis(Axis::new().color(Color::RED))
        .series(
            ScatterSeries::new("S")
                .points(&[(0.0, 0.0), (10.0, 10.0)])
                .marker(Marker::Star),
        )
}

/// Y labels sit on the rows of the horizontal grid lines and X labels are
/// centered on the vertical ones, the points on the same scale: the
/// chart area is columns 6..39 and rows 0..18 of 40x20, y_max on its top
/// row and y_min on its bottom one, x_max on its last column, all drawn,
/// at an offset too (#1622)
#[test]
fn test_scatter_labels_grid_and_points_share_the_chart_cells() {
    use super::super::render_at;
    use revue::layout::Rect;
    use revue::style::Color;
    let plain = format!("      {} ", "│     │     │      │     │      │");
    let line = |label: &str| format!("{label:<6}├{} ", "─".repeat(32));
    let mut expected: Vec<String> = (0..20).map(|_| plain.clone()).collect();
    expected[0] = format!("10.5  ├{}★── ", "─".repeat(29));
    for (y, label) in [
        (3, "8.3"),
        (6, "6.1"),
        (10, "3.9"),
        (13, "1.7"),
        (17, "-0.50"),
    ] {
        expected[y] = line(label);
    }
    expected[16] = format!("      │★{}", &plain[10..]);
    expected[18] = " ".repeat(40);
    expected[19] = "    -0.50  1.7   3.9    6.1   8.3   10.5".to_string();
    assert_eq!(rows(&render(&two_points(), 40, 20)), expected);

    let buffer = render_at(&two_points(), Rect::new(3, 2, 40, 20));
    for y in 0..24u16 {
        for x in 0..45u16 {
            let cell = buffer.get(x, y).unwrap();
            let inside = (3..43).contains(&x) && (2..22).contains(&y);
            let want = if inside {
                expected[usize::from(y - 2)]
                    .chars()
                    .nth(usize::from(x - 3))
                    .unwrap()
            } else {
                ' '
            };
            assert_eq!(cell.symbol, want, "({x}, {y})");
            let label_fg = if y == 21 && cell.symbol != ' ' {
                Some(Color::GREEN)
            } else if x < 8 && cell.symbol != ' ' && inside {
                Some(Color::RED)
            } else {
                None
            };
            if label_fg.is_some() {
                assert_eq!(cell.fg, label_fg, "({x}, {y})");
            }
        }
    }
}

/// The X labels of `format` with `ticks` ticks under the points (0, 0)
/// and (10, 10) in a `width`x8 chart with vertical grid lines, and the
/// grid line row above them
fn x_labeled(format: &str, ticks: usize, width: u16) -> (String, String) {
    use revue::widget::data::chart::{AxisFormat, Marker};
    let format = match format {
        "fixed2" => AxisFormat::Fixed(2),
        custom => AxisFormat::Custom(custom.into()),
    };
    let chart = ScatterChart::new()
        .no_legend()
        .grid(ChartGrid::x_only())
        .x_axis(Axis::new().ticks(ticks).format(format))
        .series(
            ScatterSeries::new("S")
                .points(&[(0.0, 0.0), (10.0, 10.0)])
                .marker(Marker::Star),
        );
    let rows = rows(&render(&chart, width, 8));
    (rows[5].clone(), rows[7].clone())
}

/// Each X label is centered on its tick and cut with `…` to the columns
/// between labeled ticks less one, so neighbors never overwrite each
/// other; one past the right edge is moved in, whole (#1623)
#[test]
fn test_scatter_x_labels_are_cut_to_their_slot() {
    // Ticks on columns 6, 10, 14 and 18: 3 columns each
    let (grid, labels) = x_labeled("fixed2", 3, 20);
    assert_eq!(grid, "-0.50 ┴   ┴   ┴   ┴ ");
    assert_eq!(labels, "     -0… 3.… 6.… 10…");
    // Ticks 6 or 7 columns apart: "10.50" moves in to end on the last
    // column
    let (grid, labels) = x_labeled("fixed2", 5, 40);
    assert_eq!(grid, "-0.50 ┴     ┴     ┴      ┴     ┴      ┴ ");
    assert_eq!(labels, "    -0.50 1.70  3.90   6.10  8.30  10.50");
}

/// A label moved in onto the one before it starts a column after that
/// one and is cut with `…` to the columns left; wide glyphs stay whole
/// (`rows` shows the right half of a wide glyph as '\0')
#[test]
fn test_scatter_x_labels_moved_in_never_overwrite() {
    // Ticks on columns 6, 12, 18 and 24, each label cut to 5 columns;
    // "가10…" would start at 21, inside "가6.…" (16..20) and its gap
    let (grid, labels) = x_labeled("가{}", 3, 26);
    assert_eq!(grid, "-0.50 ┴     ┴     ┴     ┴ ");
    assert_eq!(labels, "    가\0-0… 가\x003.… 가\x006.… 가\x001…");
}
