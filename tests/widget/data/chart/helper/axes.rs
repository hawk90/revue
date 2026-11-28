//! Chart axis tests: tick labels, label formats, grid lines and axis titles

use super::super::{count_char, render, render_rows, rows};
use super::{diagonal, plain, sym};
use revue::layout::Rect;
use revue::widget::data::chart::{Axis, AxisFormat, Chart, Series};

#[test]
fn test_chart_negative_values_labels() {
    let s = Series::new("D").data(vec![(-5.0, -3.0), (-2.0, -1.0), (0.0, 0.0)]);
    let rows = render_rows(&plain().series(s), 40, 20);
    // Y -3..0 padded to -3.15..0.15: a 5-column label column, the plot at 6
    assert!(rows[17].starts_with("-3.15 "), "{:?}", rows[17]);
    assert!(rows[0].starts_with(" 0.15 "), "{:?}", rows[0]);
    // X labels are centered on their tick: "-5" starts one column left of the plot
    assert!(rows[18][5..].starts_with("-5"), "{:?}", rows[18]);
}

#[test]
fn test_chart_large_values_use_exponent_labels() {
    let s = Series::new("D").data(vec![(1000.0, 2000.0), (3000.0, 4000.0)]);
    let rows = render_rows(&plain().series(s), 40, 20);
    assert!(rows[17].starts_with("1.9e3 "), "{:?}", rows[17]);
    assert!(rows[0].starts_with("4.1e3 "), "{:?}", rows[0]);
}

#[test]
fn test_chart_small_values_use_exponent_labels() {
    let s = Series::new("D").data(vec![(0.001, 0.002), (0.003, 0.004)]);
    let rows = render_rows(&plain().series(s), 40, 20);
    assert!(rows[17].starts_with("1.9e-3 "), "{:?}", rows[17]);
    assert!(rows[0].starts_with("4.1e-3 "), "{:?}", rows[0]);
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
    // "v0.2" is the widest Y label: the plot starts at 5
    assert!(rows[17].starts_with("  v0 "), "{:?}", rows[17]);
    assert!(rows[0].starts_with("  v1 "), "{:?}", rows[0]);
    assert!(rows[18][4..].starts_with("0%"), "{:?}", rows[18]);
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
    assert!(rows[17].starts_with("0.000 "), "{:?}", rows[17]);

    let integer = plain()
        .series(Series::new("D").data(vec![(0.0, 0.0), (1.0, 1.0)]))
        .y_axis(
            Axis::new()
                .bounds(0.0, 5.67)
                .grid(false)
                .format(AxisFormat::Integer),
        );
    let rows = render_rows(&integer, 40, 20);
    // One-column labels "0".."6"
    assert!(rows[0].starts_with("6 "), "{:?}", rows[0]);
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
    // 6, 8 and 9, each label cut with `…` to the label column, a third of
    // the area (#1606), the plot blank after it
    for y in [2, 4, 6, 8, 10, 11] {
        let label = format!("{}…", "y".repeat(9));
        assert_eq!(rows[y], format!("   {label}{}", " ".repeat(22)), "row {y}");
    }
    // The X title from the plot's first column (area column 11)
    assert_eq!(rows[13], format!("{}{}  ", " ".repeat(14), "t".repeat(19)));
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
    // The X label row (area row 10): the title in column 0, then each
    // label cut with `…` to its slot, the 23 plot columns over 5 ticks
    // less one, 3 columns centered on ticks 6, 10, 15, 19, 24 and 29 (the
    // last one cut at the area's edge), so none overwrites the next
    // (#1606)
    assert_eq!(
        rows[12], "   v    qq… qq…  qq… qq…  qq…  qq  ",
        "{:?}",
        rows[12]
    );
}

// #1606: the Y label column fits the widest Y label, up to a third of the
// area, and the X labels are cut to their slots

/// A 30x12 chart whose two Y labels are `format` of 0 and 1 (in red), with
/// X labels "0" and "1" on row 10: "0" marks the plot's first column
fn y_labels(format: &str) -> revue::render::Buffer {
    use revue::style::Color;
    let chart = plain()
        .x_axis(Axis::new().bounds(0.0, 1.0).ticks(1).grid(false))
        .y_axis(
            Axis::new()
                .bounds(0.0, 1.0)
                .ticks(1)
                .grid(false)
                .color(Color::RED)
                .format(AxisFormat::Custom(format.to_string())),
        );
    render(&chart, 30, 12)
}

/// Row `y` of `buffer` from column `from` to `to` (exclusive)
fn cells(buffer: &revue::render::Buffer, y: u16, from: u16, to: u16) -> String {
    (from..to).map(|x| sym(buffer, x, y)).collect()
}

#[test]
fn test_chart_y_label_column_is_as_wide_as_the_widest_label() {
    // One column: the plot starts after it and the gap, at 2
    let b = y_labels("{}");
    assert_eq!(cells(&b, 9, 0, 3), "0  ");
    assert_eq!(cells(&b, 0, 0, 3), "1  ");
    assert_eq!(cells(&b, 10, 0, 4), "  0 ");
    // Nine columns, one under the cap of 30 / 3
    let b = y_labels("{}xxxxxxxx");
    assert_eq!(cells(&b, 9, 0, 11), "0xxxxxxxx  ");
    assert_eq!(cells(&b, 0, 0, 11), "1xxxxxxxx  ");
    assert_eq!(cells(&b, 10, 9, 12), " 0 ");
    // Ten columns, exactly the cap: not cut
    let b = y_labels("{}xxxxxxxxx");
    assert_eq!(cells(&b, 9, 0, 12), "0xxxxxxxxx  ");
    assert_eq!(cells(&b, 10, 10, 13), " 0 ");
    // The X labels stay at the plot's ends
    assert_eq!(sym(&b, 29, 10), '1');
}

#[test]
fn test_chart_y_labels_past_the_cap_are_cut_with_an_ellipsis() {
    use revue::style::Color;
    // Eleven columns: cut to the ten of the column, `…` last, the plot
    // still at 11 and nothing drawn over it
    let b = y_labels("{}xxxxxxxxxx");
    assert_eq!(cells(&b, 9, 0, 12), "0xxxxxxxx…  ");
    assert_eq!(cells(&b, 0, 0, 12), "1xxxxxxxx…  ");
    assert_eq!(cells(&b, 10, 10, 13), " 0 ");
    for y in [0, 9] {
        for x in 0..10 {
            assert_eq!(b.get(x, y).unwrap().fg, Some(Color::RED), "({x}, {y})");
        }
        assert!(cells(&b, y, 10, 30).trim().is_empty(), "row {y}");
    }
    // Wide glyphs: "0가나다라마" is 11 columns, cut to "0가나다라…"
    let b = y_labels("{}가나다라마");
    for (x, ch) in [
        (0, '0'),
        (1, '가'),
        (3, '나'),
        (5, '다'),
        (7, '라'),
        (9, '…'),
    ] {
        assert_eq!(sym(&b, x, 9), ch, "x={x}");
    }
    assert_eq!(cells(&b, 9, 10, 12), "  ");
    assert_eq!(cells(&b, 10, 10, 13), " 0 ");
}

#[test]
fn test_chart_long_x_labels_are_cut_to_their_slots() {
    // Ticks 0, 5 and 10 on plot columns 6, 22 and 39 (the empty chart's
    // Y labels are 5 columns): slots of 33 / 2 - 1 = 15 columns. Each label
    // is cut to 15 with `…`; the first one, centered on 6, starts at the
    // area's edge, and the last is cut at the area's right edge
    let chart = plain().x_axis(
        Axis::new()
            .bounds(0.0, 10.0)
            .ticks(2)
            .grid(false)
            .format(AxisFormat::Custom("{}abcdefghijklmnopqrstuvwxyz".into())),
    );
    let rows = render_rows(&chart, 40, 20);
    assert_eq!(
        rows[18], "0abcdefghijklm…5abcdefghijklm…  10abcdef",
        "{:?}",
        rows[18]
    );
}

/// The label column is a third of the area at most, so a 10x5 chart (one
/// of 3 columns and the gap) has room for its 6-column plot; a 10x4 one
/// still has no room for 3 plot rows
#[test]
fn test_chart_a_narrow_label_column_leaves_room_for_the_plot() {
    let chart = plain()
        .x_axis(Axis::new().bounds(0.0, 1.0).ticks(1).grid(false))
        .y_axis(Axis::new().bounds(0.0, 1.0).ticks(1).grid(false));
    let rows = render_rows(&chart, 10, 5);
    assert_eq!(rows[0], "1         ");
    assert_eq!(rows[2], "0         ");
    assert_eq!(rows[3], "  0      1");
    assert!(render_rows(&chart, 10, 4)
        .iter()
        .all(|r| r.trim().is_empty()));
}
