//! Vertical BarChart tests: bars, values and labels

use super::super::{render, render_rows, rows};
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

/// A value wider than its bar is cut to the bar's width, like the label,
/// so it does not run into the next bars' columns (#1586)
#[test]
fn test_barchart_vertical_values_cut_to_the_bar() {
    // bar_width(1), gap 1: A at 0, B at 2; 6 rows = value, 4 bars, label
    let chart = BarChart::new().bar("A", 150.0).bar("B", 5.0).vertical();
    let rows = render_rows(&chart, 10, 6);
    assert_eq!(rows[0], "1         ");
    assert_eq!(rows[3], "█ 5       ");
    assert_eq!(rows[5], "A B       ");

    // A value that fits its bar is drawn whole, the last bar's value at
    // the area's last columns
    let chart = BarChart::new()
        .bar("A", 150.0)
        .bar("B", 150.0)
        .vertical()
        .bar_width(3)
        .max(150.0);
    let rows = render_rows(&chart, 7, 6);
    assert_eq!(rows[0], "150 150");
    let rows = render_rows(&chart.clone().bar_width(2).gap(0), 4, 6);
    assert_eq!(rows[0], "1515");
}

/// Three 3-wide bars with a 1-column gap, A at 0..3, B at 4..7, C at 8..11
fn abc() -> BarChart {
    BarChart::new()
        .bar("A", 1.0)
        .bar("B", 1.0)
        .bar("C", 1.0)
        .vertical()
        .show_values(false)
        .bar_width(3)
}

#[test]
fn test_barchart_vertical_bars_that_exactly_fit_have_no_marker() {
    let rows = render_rows(&abc(), 11, 3);
    assert_eq!(rows[0], "███ ███ ███");
    assert_eq!(rows[1], "███ ███ ███");
    assert_eq!(rows[2], "A   B   C  ");
}

#[test]
fn test_barchart_vertical_bars_beyond_width_are_marked() {
    // C would need 8..11: `…` takes its first column, the rest stays blank
    let buffer = render(&abc(), 10, 3);
    let rows = rows(&buffer);
    assert_eq!(rows[0], "███ ███   ");
    assert_eq!(rows[1], "███ ███   ");
    assert_eq!(rows[2], "A   B   … ");
    // Styled like the labels: no color of its own
    assert_eq!(buffer.get(8, 2).unwrap().fg, buffer.get(4, 2).unwrap().fg);
}

#[test]
fn test_barchart_vertical_marker_replaces_a_bar_at_the_edge() {
    // B fits exactly (4..7) but leaves no column for the `…`, so B gives
    // its slot to it
    let rows = render_rows(&abc(), 7, 3);
    assert_eq!(rows[0], "███    ");
    assert_eq!(rows[1], "███    ");
    assert_eq!(rows[2], "A   …  ");
}

#[test]
fn test_barchart_vertical_one_column_shows_only_the_marker() {
    assert_eq!(
        render_rows(&abc(), 1, 3),
        ["", "", "…"].map(|r| format!("{r:1}"))
    );
}

#[test]
fn test_barchart_vertical_far_too_many_bars() {
    // 1000 one-column bars with a gap in 20 columns: 9 bars at 0, 2, .., 16,
    // `…` at 18 and column 19 blank
    let chart = BarChart::new()
        .data((0..1000).map(|i| (((b'a' + (i % 26) as u8) as char).to_string(), 1.0)))
        .vertical()
        .show_values(false);
    let rows = render_rows(&chart, 20, 2);
    assert_eq!(rows[0], "█ █ █ █ █ █ █ █ █   ");
    assert_eq!(rows[1], "a b c d e f g h i … ");
}

#[test]
fn test_barchart_vertical_more_than_65535_bars() {
    let chart = BarChart::new()
        .data((0..70_000).map(|_| ("x", 1.0)))
        .vertical()
        .gap(0)
        .show_values(false);
    let rows = render_rows(&chart, 20, 2);
    assert_eq!(rows[0], format!("{}{}", "█".repeat(19), " "));
    assert_eq!(rows[1], format!("{}…", "x".repeat(19)));
}

#[test]
fn test_barchart_vertical_wide_labels_next_to_the_marker() {
    let chart = BarChart::new()
        .data([("한", 1.0), ("글", 1.0), ("자", 1.0)])
        .vertical()
        .gap(0)
        .bar_width(2)
        .show_values(false);
    // 6 columns needed, 5 given: two bars, `…` at 4
    let buffer = render(&chart, 5, 2);
    let label: Vec<_> = (0..5).map(|x| buffer.get(x, 1).unwrap().symbol).collect();
    assert_eq!(label, ['한', '\0', '글', '\0', '…']);
    assert_eq!(rows(&buffer)[0], "████ ");
}
