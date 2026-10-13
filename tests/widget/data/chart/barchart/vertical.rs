//! Vertical BarChart tests: bars, values and labels

use super::super::render_rows;
use revue::widget::data::chart::BarChart;

#[test]
fn test_barchart_render_vertical() {
    // 10 rows: 1 value row, 8 bar rows, 1 label row
    let chart = BarChart::new()
        .bar("A", 50.0)
        .bar("B", 100.0)
        .vertical()
        .max(100.0)
        .bar_width(3);
    let rows = render_rows(&chart, 20, 10);
    let expected = [
        "    100", // value above the full-height bar
        "    ███",
        "    ███",
        "    ███",
        "50  ███", // value directly above the half-height bar
        "███ ███",
        "███ ███",
        "███ ███",
        "███ ███",
        "A   B",
    ];
    for (y, want) in expected.iter().enumerate() {
        assert_eq!(rows[y].trim_end(), *want, "row {y}: {rows:#?}");
    }
}

#[test]
fn test_barchart_vertical_without_values() {
    let chart = BarChart::new()
        .bar("A", 100.0)
        .vertical()
        .show_values(false)
        .bar_width(2);
    let rows = render_rows(&chart, 10, 5);
    // 4 bar rows, 1 label row
    for row in &rows[..4] {
        assert_eq!(row.trim_end(), "██");
    }
    assert_eq!(rows[4].trim_end(), "A");
}

#[test]
fn test_barchart_vertical_negative_values_use_magnitude() {
    let chart = BarChart::new()
        .bar("A", -100.0)
        .bar("B", -50.0)
        .vertical()
        .show_values(false);
    let rows = render_rows(&chart, 10, 5);
    // 4 bar rows: A full height, B half height
    assert_eq!(rows[0].trim_end(), "█");
    assert_eq!(rows[1].trim_end(), "█");
    assert_eq!(rows[2].trim_end(), "█ █");
    assert_eq!(rows[3].trim_end(), "█ █");
    assert_eq!(rows[4].trim_end(), "A B");
}

#[test]
fn test_barchart_vertical_long_labels_truncated() {
    let chart = BarChart::new()
        .bar("VeryLongLabel", 50.0)
        .vertical()
        .bar_width(5)
        .max(100.0);
    let rows = render_rows(&chart, 10, 10);
    assert_eq!(rows[9].trim_end(), "VeryL");
}

#[test]
fn test_barchart_vertical_bars_beyond_width_are_dropped() {
    let chart = BarChart::new()
        .bar("A", 1.0)
        .bar("B", 1.0)
        .bar("C", 1.0)
        .vertical()
        .bar_width(3);
    // A at 0..3, B at 4..7, C would need 8..11
    let rows = render_rows(&chart, 10, 5);
    assert_eq!(rows[4].trim_end(), "A   B");
}
