//! Histogram axis label tests, both orientations

use super::super::render_rows;
use super::three_bins;

#[test]
fn test_histogram_axis_labels() {
    let rows = render_rows(&three_bins(), 40, 20);
    // Y: the counts 3, 2, 1 and 0, each once, on the top row of a bar of
    // that count (3 of 3 fills the 18 rows, 2 fills 12 and 1 fills 6) and
    // 0 on the bottom row (#1624)
    let heads: Vec<String> = rows.iter().map(|r| r.chars().take(2).collect()).collect();
    for (y, head) in heads.iter().enumerate().take(19) {
        let expected = match y {
            0 => "3 ",
            6 => "2 ",
            12 => "1 ",
            17 => "0 ",
            _ => "  ",
        };
        assert_eq!(head, expected, "row {y}");
    }
    // X: bin range 0..30 in five ticks over the chart's 37 columns from
    // column 2, 0 on its first and 30 on its last (36), each centered on
    // its tick (#1622, #1623)
    assert_eq!(rows[19], "0.00      7.5     15.0     22.5     30.0");
}

#[test]
fn test_histogram_horizontal_axis_labels() {
    let rows = render_rows(&three_bins().horizontal(), 40, 20);
    // Bin values down the left side, lowest first
    assert!(rows[0].starts_with("0.00 "), "{:?}", rows[0]);
    assert!(rows[4].starts_with("7.5 "), "{:?}", rows[4]);
    assert!(rows[17].starts_with("30.0 "), "{:?}", rows[17]);
    // Counts 0, 1, 2 and 3 along the bottom, each once, on the last
    // column of a bar of that count: the bin labels are at most four
    // columns wide, so the chart is columns 5..39, which 3 fills, 2 fills
    // 22 of and 1 fills 11 of (#1624)
    assert_eq!(rows[19], "     0         1          2           3 ");
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
    // at 6; each count on the top row of a bar of that count in the 8
    // rows, 0 on the bottom one
    let hist = Histogram::new(&vec![1.0; 123_456]).bins(BinConfig::Edges(vec![0.0, 10.0]));
    let rows = super::super::rows(&render(&hist, 15, 10));
    let head = |y: usize| -> String { rows[y].chars().take(7).collect() };
    assert_eq!(head(0), "1234… ▀");
    assert_eq!(head(1), "      █");
    assert_eq!(head(2), "92592 █");
    assert_eq!(head(3), "      █");
    assert_eq!(head(4), "61728 █");
    assert_eq!(head(5), "      █");
    assert_eq!(head(6), "30864 █");
    assert_eq!(head(7), "0     █");
}

/// The first glyphs of rows `0..rows` of `hist` drawn `width`x`height`,
/// `columns` wide
fn label_heads(
    hist: &revue::widget::data::chart::Histogram,
    width: u16,
    height: u16,
    columns: usize,
) -> Vec<String> {
    render_rows(hist, width, height)
        .iter()
        .map(|r| r.chars().take(columns).collect())
        .collect()
}

/// The counts are labeled at whole counts, each once, on the top row of a
/// bar of that count, 0 on the bottom row: the ticks 7, 5.25, 3.5, 1.75
/// and 0 read 7, 5, 4, 2 and 0 (#1624)
#[test]
fn test_histogram_count_labels_are_whole_counts_each_once() {
    use revue::widget::data::chart::{BinConfig, Histogram};
    // 7 and 1 values in two bins: the chart is rows 0..8, a bar of n
    // counts n * 8 / 7 rows tall
    let hist = Histogram::new(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 15.0])
        .bins(BinConfig::Edges(vec![0.0, 10.0, 20.0]));
    let heads = label_heads(&hist, 30, 10, 2);
    assert_eq!(heads[..8], ["7 ", "  ", "  ", "5 ", "4 ", "  ", "2 ", "0 "]);
    // One value: 0.75 and 0.5 read 1 again and 0.25 reads 0, so only 1
    // and 0
    let hist = Histogram::new(&[1.0]).bins(BinConfig::Edges(vec![0.0, 10.0]));
    let heads = label_heads(&hist, 20, 10, 2);
    assert_eq!(heads[..8], ["1 ", "  ", "  ", "  ", "  ", "  ", "  ", "0 "]);
}

/// A count whose bar top shares a row with the label above it is not
/// labeled, except 0, which takes the bottom row; the labels are the same
/// at an offset
#[test]
fn test_histogram_count_labels_one_per_row() {
    use super::super::assert_same_when_offset;
    // Three rows: 3 fills them all, 2 fills two and 1 one, the bottom row
    // where 0 goes
    let heads = label_heads(&three_bins(), 40, 5, 2);
    assert_eq!(heads[..3], ["3 ", "2 ", "0 "]);
    // 30 in one bin: 23 fills two rows, and 15 and 8 the bottom one,
    // which 0 takes from them whole, not drawn over them
    use revue::widget::data::chart::{BinConfig, Histogram};
    let hist = Histogram::new(&[1.0; 30]).bins(BinConfig::Edges(vec![0.0, 10.0]));
    let heads = label_heads(&hist, 40, 5, 3);
    assert_eq!(heads[..3], ["30 ", "23 ", "0  "]);
    assert_same_when_offset(three_bins(), 40, 5);
    assert_same_when_offset(three_bins(), 40, 10);
}

/// The bin value labels are centered on their ticks, the last on the
/// chart's last column, each cut with `…` to the columns between labeled
/// ticks less one so neighbors never overwrite each other (#1623)
#[test]
fn test_histogram_x_labels_are_cut_to_their_slot() {
    use revue::widget::data::chart::{Axis, AxisFormat, BinConfig, Histogram};
    // The chart is columns 2..18: ticks on 2, 6, 10, 14 and 18, 3 columns
    // each
    let hist = Histogram::new(&[1.0]).bins(BinConfig::Edges(vec![0.0, 10.0]));
    let rows = render_rows(&hist, 20, 10);
    assert_eq!(rows[9], " 0.… 2.5 5.0 7.5 10…");
    // "0가나" cut to 3 columns keeps no wide glyph: "0…", 2 columns,
    // centered on its tick
    let wide = hist.x_axis(Axis::new().format(AxisFormat::Custom("{}가나".into())));
    let rows = render_rows(&wide, 20, 10);
    assert_eq!(rows[9], " 0…  2.… 5…  7.… 10…");
}
