//! Histogram axis label tests, both orientations

use super::super::render_rows;
use super::three_bins;

#[test]
fn test_histogram_axis_labels() {
    let rows = render_rows(&three_bins(), 40, 20);
    // Y: count from max (3) down to 0
    assert!(rows[1].starts_with("3 "), "{:?}", rows[1]);
    assert!(rows[18].starts_with("0 "), "{:?}", rows[18]);
    // X: bin range 0..30 in five ticks, from the chart's first column: the
    // count labels are one column wide, so the chart starts at column 2
    assert_eq!(rows[19], "  0.00     7.5       15.0     22.5      ");
}

#[test]
fn test_histogram_horizontal_axis_labels() {
    let rows = render_rows(&three_bins().horizontal(), 40, 20);
    // Bin values down the left side, lowest first
    assert!(rows[0].starts_with("0.00 "), "{:?}", rows[0]);
    assert!(rows[4].starts_with("7.5 "), "{:?}", rows[4]);
    assert!(rows[17].starts_with("30.0 "), "{:?}", rows[17]);
    // Counts along the bottom, from 0 on the left to the maximum on the
    // right, from the chart's first column: the bin labels are at most four
    // columns wide, so the chart starts at column 5
    assert_eq!(rows[19], "     0       1       2       2        3 ");
}

/// The label column left of the chart is as wide as the widest of its
/// labels, at most a third of the area, with a gap column before the
/// chart; a wider label is cut with `…` (#1614)
#[test]
fn test_histogram_horizontal_bin_labels_past_a_third_are_cut_with_an_ellipsis() {
    use super::super::render;
    use revue::style::Color;
    use revue::widget::data::chart::{Axis, AxisFormat};
    let axis = Axis::new()
        .color(Color::RED)
        .format(AxisFormat::Custom("{}abcdefghij".into()));
    let buffer = render(&three_bins().horizontal().x_axis(axis), 30, 12);
    let rows = super::super::rows(&buffer);
    let head = |y: usize| -> String { rows[y].chars().take(12).collect() };
    // Bin values 0, 7.5, 15, 22.5 and 30 on rows 0, 2, 4, 6 and 9, cut to
    // the 10 columns of a third of 30; the bars start at 11
    assert_eq!(head(0), "0abcdefgh… █");
    assert_eq!(head(2), "7.5abcdef… █");
    assert_eq!(head(4), "15abcdefg… █");
    assert_eq!(head(6), "22.5abcde… █");
    assert_eq!(head(9), "30abcdefg… █");
    for x in 0..10 {
        assert_eq!(buffer.get(x, 0).unwrap().fg, Some(Color::RED), "x={x}");
    }
}

#[test]
fn test_histogram_count_labels_past_a_third_are_cut_with_an_ellipsis() {
    use super::super::render;
    use revue::widget::data::chart::{BinConfig, Histogram};
    // 123456 values in one bin: "123456" is 6 columns, past a third of 15,
    // so it is cut to 5, the 5-column counts are whole, and the bar starts
    // at 6
    let hist = Histogram::new(&vec![1.0; 123_456]).bins(BinConfig::Edges(vec![0.0, 10.0]));
    let rows = super::super::rows(&render(&hist, 15, 10));
    let head = |y: usize| -> String { rows[y].chars().take(7).collect() };
    assert_eq!(head(1), "1234… █");
    assert_eq!(head(2), "92592 █");
    assert_eq!(head(4), "61728 █");
    assert_eq!(head(6), "30864 █");
    assert_eq!(head(8), "0      ");
}
