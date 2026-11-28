//! Chart axis tests: tick labels, label formats, grid lines and axis titles

use super::super::{count_char, render, render_rows, rows};
use super::{diagonal, plain, sym};
use revue::layout::Rect;
use revue::widget::data::chart::{Axis, AxisFormat, Chart, Series};

#[test]
fn test_chart_negative_values_labels() {
    let s = Series::new("D").data(vec![(-5.0, -3.0), (-2.0, -1.0), (0.0, 0.0)]);
    let rows = render_rows(&plain().series(s), 40, 20);
    // Y -3..0 padded to -3.15..0.15
    assert!(rows[17].starts_with("  -3.15"), "{:?}", rows[17]);
    assert!(rows[0].starts_with("   0.15"), "{:?}", rows[0]);
    // X labels are centered on their tick: "-5" starts one column left of the plot
    assert!(rows[18][7..].starts_with("-5"), "{:?}", rows[18]);
}

#[test]
fn test_chart_large_values_use_exponent_labels() {
    let s = Series::new("D").data(vec![(1000.0, 2000.0), (3000.0, 4000.0)]);
    let rows = render_rows(&plain().series(s), 40, 20);
    assert!(rows[17].starts_with("  1.9e3"), "{:?}", rows[17]);
    assert!(rows[0].starts_with("  4.1e3"), "{:?}", rows[0]);
}

#[test]
fn test_chart_small_values_use_exponent_labels() {
    let s = Series::new("D").data(vec![(0.001, 0.002), (0.003, 0.004)]);
    let rows = render_rows(&plain().series(s), 40, 20);
    assert!(rows[17].starts_with(" 1.9e-3"), "{:?}", rows[17]);
    assert!(rows[0].starts_with(" 4.1e-3"), "{:?}", rows[0]);
}

#[test]
fn test_chart_axis_formats() {
    let s = Series::new("D").data(vec![(0.0, 0.0), (1.0, 1.0)]);
    let chart = plain()
        .series(s)
        .x_axis(
            Axis::new()
                .bounds(0.0, 1.0)
                .grid(false)
                .format(AxisFormat::Percent),
        )
        .y_axis(
            Axis::new()
                .bounds(0.0, 1.0)
                .grid(false)
                .format(AxisFormat::Custom("v{}".to_string())),
        );
    let rows = render_rows(&chart, 40, 20);
    assert!(rows[17].starts_with("     v0"), "{:?}", rows[17]);
    assert!(rows[0].starts_with("     v1"), "{:?}", rows[0]);
    assert!(rows[18][7..].starts_with("0%"), "{:?}", rows[18]);
    // "100%" is centered on the last column, so its '%' falls off the edge
    assert!(rows[18].trim_end().ends_with("100"), "{:?}", rows[18]);

    let fixed = plain()
        .series(Series::new("D").data(vec![(0.0, 0.0), (1.0, 1.0)]))
        .y_axis(
            Axis::new()
                .bounds(0.0, 1.0)
                .grid(false)
                .format(AxisFormat::Fixed(3)),
        );
    let rows = render_rows(&fixed, 40, 20);
    assert!(rows[17].starts_with("  0.000"), "{:?}", rows[17]);

    let integer = plain()
        .series(Series::new("D").data(vec![(0.0, 0.0), (1.0, 1.0)]))
        .y_axis(
            Axis::new()
                .bounds(0.0, 5.67)
                .grid(false)
                .format(AxisFormat::Integer),
        );
    let rows = render_rows(&integer, 40, 20);
    assert!(rows[0].starts_with("      6"), "{:?}", rows[0]);
}

#[test]
fn test_chart_grid_lines() {
    let chart = Chart::new().no_legend().series(diagonal());
    let buffer = render(&chart, 40, 20);
    // Default axes draw 4 inner grid lines each way
    assert!(count_char(&buffer, '┄') > 0);
    assert!(count_char(&buffer, '┊') > 0);

    let buffer = render(&plain().series(diagonal()), 40, 20);
    assert_eq!(count_char(&buffer, '┄'), 0);
    assert_eq!(count_char(&buffer, '┊'), 0);
}

#[test]
fn test_chart_axis_titles() {
    let chart = plain()
        .x_axis(Axis::new().title("X Axis").grid(false))
        .y_axis(Axis::new().title("Y Axis").grid(false))
        .series(Series::new("Test").data(vec![(1.0, 2.0)]));
    let buffer = render(&chart, 40, 20);
    let rows = rows(&buffer);
    // X title centered under the plot on row 19
    assert_eq!(rows[19].trim(), "X Axis");
    // Y title written vertically in column 0, centered on the plot height
    let column: String = (6..12).map(|y| sym(&buffer, 0, y)).collect();
    assert_eq!(column, "Y Axis");
}

/// `ticks(0)` labels the axis once, not with NaN (#862)
#[test]
fn test_chart_zero_ticks_draws_no_nan() {
    let chart = Chart::new()
        .series(Series::new("s").data(vec![(0.0, 0.0), (10.0, 10.0)]))
        .x_axis(Axis::new().ticks(0))
        .y_axis(Axis::new().ticks(0));
    let text = render_rows(&chart, 40, 12).join("\n");
    assert!(!text.contains("NaN"), "{text}");
}

/// Y labels of 65536 columns are right-aligned by their real width, so
/// they start at the label column's left edge instead of at its last
/// column, and the X title of 65536 columns starts at the plot's left
/// edge instead of its middle; inside an area at an offset (#1596)
#[test]
fn test_chart_y_labels_and_x_title_wider_than_u16() {
    use super::super::render_at;
    let chart = plain()
        .y_axis(
            Axis::new()
                .grid(false)
                .format(AxisFormat::Custom("y".repeat(65_536))),
        )
        .x_axis(Axis::new().grid(false).title("t".repeat(65_536)));
    let rows = rows(&render_at(&chart, Rect::new(3, 2, 30, 12)));
    // The area is columns 3..33, rows 2..14; ticks on plot rows 0, 2, 4,
    // 6, 8 and 9, each label running to the area's edge
    for y in [2, 4, 6, 8, 10, 11] {
        assert_eq!(rows[y], format!("   {}  ", "y".repeat(30)), "row {y}");
    }
    // The X title from the plot's first column (area column 8)
    assert_eq!(rows[13], format!("{}{}  ", " ".repeat(11), "t".repeat(22)));
    assert!(rows[14].trim().is_empty() && rows[1].trim().is_empty());
}

/// X labels of 65536 columns are centered by their real width, so the
/// first one starts at the area's left edge instead of at its tick, and a
/// Y title of 65536 chars runs down the whole first column instead of
/// starting halfway and wrapping its row; inside an area at an offset
/// (#1596)
#[test]
fn test_chart_x_labels_and_y_title_wider_than_u16() {
    use super::super::render_at;
    let chart = plain()
        .x_axis(
            Axis::new()
                .grid(false)
                .format(AxisFormat::Custom("q".repeat(65_536))),
        )
        .y_axis(Axis::new().grid(false).title("v".repeat(65_536)));
    let buffer = render_at(&chart, Rect::new(3, 2, 30, 12));
    let rows = rows(&buffer);
    let column: String = (0..16).map(|y| sym(&buffer, 3, y)).collect();
    assert_eq!(column, format!("  {}  ", "v".repeat(12)));
    // The X label row (area row 10): the title in column 0, labels after
    assert_eq!(rows[12], format!("   v{}  ", "q".repeat(29)));
}
