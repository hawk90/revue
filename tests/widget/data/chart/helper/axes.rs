//! Chart axis tests: tick labels, label formats, grid lines and axis titles

use super::super::{count_char, render, render_rows, rows};
use super::{diagonal, plain, sym};
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
