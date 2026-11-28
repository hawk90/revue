//! BoxPlot axis label tests: values and group names

use super::super::{assert_same_when_offset, assert_zero_areas_draw_nothing, render, rows};
use super::{even_stats, row_chars};
use revue::render::Buffer;
use revue::widget::data::chart::BoxPlot;

#[test]
fn test_boxplot_value_axis_labels() {
    let bp = BoxPlot::new().group_stats("Data", even_stats());
    let rows = rows(&render(&bp, 40, 20));
    // Five labels from the padded max (110) down to the padded min (-10)
    assert!(rows[1].starts_with("110.0"));
    assert!(rows[5].starts_with("80.0"));
    assert!(rows[9].starts_with("50.0"));
    assert!(rows[13].starts_with("20.0"));
    assert!(rows[18].starts_with("-10.0"));
}

#[test]
fn test_boxplot_category_labels() {
    let bp = BoxPlot::new()
        .group_stats("A", even_stats())
        .group_stats("B", even_stats());
    let rows = rows(&render(&bp, 40, 20));
    // Labels sit under their boxes: the groups split the 33 chart columns
    // (x = 6..39) into bands of 16, centered on 14 and 30
    let last: Vec<char> = rows[19].chars().collect();
    assert_eq!(last[14], 'A');
    assert_eq!(last[30], 'B');
    // The whiskers run down the same columns
    let whisker: Vec<char> = rows[3].chars().collect();
    assert_eq!((whisker[14], whisker[30]), ('│', '│'));
    assert_eq!(whisker.iter().filter(|&&c| c != ' ').count(), 2);
}

#[test]
fn test_boxplot_long_category_label_clipped() {
    let bp = BoxPlot::new().group("VeryLongGroupName", &[1.0, 2.0, 3.0]);
    let rows = rows(&render(&bp, 20, 10));
    // Label is centered on column 12 (the box's center), so it starts at
    // column 4; cells left of the chart area (the value-label column,
    // x < 6) are not drawn
    assert_eq!(rows[9].trim_end(), "      ryLongGroupNam");
}

/// A label of 65536+ columns is centered by its real width, not the width
/// cut to u16 (which would be 0 and start it at the box's center): it
/// covers every chart column of the label row (#1587)
#[test]
fn test_boxplot_category_label_wider_than_u16() {
    for n in [65_536, 131_072] {
        let bp = BoxPlot::new().group("x".repeat(n), &[1.0, 2.0, 3.0]);
        let rows = rows(&render(&bp, 20, 10));
        assert_eq!(rows[9], format!("      {}", "x".repeat(14)), "{n}");
    }
}

/// A horizontal plot's value label of 65536+ columns is measured without
/// wrapping to a small width: like any label wider than the chart it is
/// skipped, where it used to be drawn and overflow its column (#1587)
#[test]
fn test_boxplot_horizontal_value_label_wider_than_u16() {
    use revue::widget::data::chart::{Axis, AxisFormat};
    let axis = Axis::new().format(AxisFormat::Custom(format!("{}{{}}", "x".repeat(65_540))));
    let bp = BoxPlot::new()
        .group_stats("A", even_stats())
        .value_axis(axis)
        .horizontal();
    let buffer = render(&bp, 40, 20);
    assert_eq!(cols(&buffer, 19, 0, 40), " ".repeat(40));
}

/// A horizontal plot's label column is as wide as the longest label up to
/// a third of the width; a label of 65536+ columns takes the third instead
/// of wrapping to a narrow column (#1587)
#[test]
fn test_boxplot_horizontal_label_column_for_a_label_wider_than_u16() {
    for n in [65_536, 65_537] {
        let bp = BoxPlot::new()
            .group_stats("x".repeat(n), even_stats())
            .horizontal();
        let buffer = render(&bp, 30, 10);
        // A 10-column label column, the label cut one column before the
        // chart, which starts at 11
        let row = (0..10u16)
            .find(|&y| row_chars(&buffer, y)[0] == 'x')
            .unwrap();
        assert_eq!(
            cols(&buffer, row, 0, 11),
            format!("{} ", "x".repeat(10)),
            "{n}"
        );
    }
}

/// `n` groups with the same stats, labeled by `label(i)`
fn many(n: usize, label: impl Fn(usize) -> String) -> BoxPlot {
    (0..n).fold(BoxPlot::new(), |bp, i| {
        bp.group_stats(label(i), even_stats())
    })
}

/// A one-character label per group: `!`, `"`, `#`, ...
fn ascii(i: usize) -> String {
    char::from(b'!' + (i % 90) as u8).to_string()
}

/// Columns `from..to` of row `y`
fn cols(buffer: &Buffer, y: u16, from: usize, to: usize) -> String {
    row_chars(buffer, y)[from..to].iter().collect()
}

// 40x20 vertical: chart columns 6..39 (33), median row 9, label row 19.
// Each group needs a column; with one each, a box is a single `├` on the
// median row.

#[test]
fn test_boxplot_groups_that_exactly_fit_have_no_marker() {
    let buffer = render(&many(33, ascii), 40, 20);
    assert_eq!(cols(&buffer, 9, 6, 40), format!("{} ", "├".repeat(33)));
    let labels: String = (0..33).map(ascii).collect();
    assert_eq!(cols(&buffer, 19, 6, 40), format!("{labels} "));
}

#[test]
fn test_boxplot_groups_beyond_width_are_marked() {
    // 34 groups in 33 columns: 32 are drawn and `…` takes the last column
    let buffer = render(&many(34, ascii), 40, 20);
    assert_eq!(cols(&buffer, 9, 6, 40), format!("{}  ", "├".repeat(32)));
    let labels: String = (0..32).map(ascii).collect();
    assert_eq!(cols(&buffer, 19, 6, 40), format!("{labels}… "));
    // The `…` is styled like the labels
    let (label, marker) = (buffer.get(6, 19).unwrap(), buffer.get(38, 19).unwrap());
    assert_eq!(marker.fg, label.fg);
    assert!(marker.fg.is_some());
    // Nothing else is drawn in the reserved column
    assert!((0..19).all(|y| buffer.get(38, y).unwrap().symbol == ' '));
}

#[test]
fn test_boxplot_far_too_many_groups() {
    // 1000 groups in 20 chart columns (6..26): 19 drawn, `…` at 25. The
    // labels overlap; the last one stops before the `…`.
    // "grp18" is centered on column 24: its "18" would reach past it.
    let buffer = render(&many(1000, |i| format!("grp{i}")), 27, 20);
    assert_eq!(cols(&buffer, 9, 6, 27), format!("{}  ", "├".repeat(19)));
    assert_eq!(cols(&buffer, 19, 22, 27), "grp… ");
}

#[test]
fn test_boxplot_more_than_65535_groups() {
    // Group 65536 would wrap onto column 6
    let buffer = render(&many(70_000, ascii), 40, 20);
    assert_eq!(cols(&buffer, 9, 6, 40), format!("{}  ", "├".repeat(32)));
    let labels: String = (0..32).map(ascii).collect();
    assert_eq!(cols(&buffer, 19, 6, 40), format!("{labels}… "));
}

#[test]
fn test_boxplot_wide_label_next_to_the_marker() {
    // 32 of 34 groups drawn, `…` at 38. "한" is 2 wide, on its group's
    // column and the one before: the last drawn group's (31, at 36..38)
    // fits.
    let buffer = render(&many(34, |_| "한".into()), 40, 20);
    let at = |x| buffer.get(x, 19).unwrap().symbol;
    assert_eq!([at(36), at(37), at(38), at(39)], ['한', '\0', '…', ' ']);

    // "a한" on group 31 puts '한' on 37..39, over the `…` column: only the
    // 'a' is drawn, and column 37 stays untouched
    let buffer = render(
        &many(34, |i| if i == 31 { "a한".into() } else { "x".into() }),
        40,
        20,
    );
    let at = |x| buffer.get(x, 19).unwrap().symbol;
    assert_eq!(
        [at(35), at(36), at(37), at(38), at(39)],
        ['x', 'a', ' ', '…', ' ']
    );
    assert_eq!(buffer.get(37, 19).unwrap().fg, None);
}

#[test]
fn test_boxplot_horizontal_groups_that_exactly_fit_have_no_marker() {
    // 40x20 horizontal, one-character labels: label column 0, chart columns
    // 2..39, rows 0..19 (19), value labels on row 19. A one-row box is a
    // single `┬` on its median column.
    let buffer = render(&many(19, ascii).horizontal(), 40, 20);
    let median = row_chars(&buffer, 0)
        .iter()
        .position(|&c| c == '┬')
        .unwrap();
    for y in 0..19u16 {
        assert_eq!(row_chars(&buffer, y)[0].to_string(), ascii(y as usize));
        assert_eq!(row_chars(&buffer, y)[median], '┬', "row {y}");
    }
}

#[test]
fn test_boxplot_horizontal_groups_beyond_height_are_marked() {
    for n in [20, 1000, 70_000] {
        let buffer = render(&many(n, ascii).horizontal(), 40, 20);
        let median = row_chars(&buffer, 0)
            .iter()
            .position(|&c| c == '┬')
            .unwrap();
        for y in 0..18u16 {
            assert_eq!(
                row_chars(&buffer, y)[0].to_string(),
                ascii(y as usize),
                "{n}"
            );
            assert_eq!(row_chars(&buffer, y)[median], '┬', "{n} row {y}");
        }
        // `…` takes the last chart row in the label column; nothing else
        // is drawn on that row
        assert_eq!(
            cols(&buffer, 18, 0, 40),
            format!("…{}", " ".repeat(39)),
            "{n}"
        );
        assert_eq!(buffer.get(0, 18).unwrap().fg, buffer.get(0, 0).unwrap().fg);
    }
}

#[test]
fn test_boxplot_horizontal_wide_labels_above_the_marker() {
    let buffer = render(&many(20, |_| "한글".into()).horizontal(), 40, 20);
    let at = |x, y| buffer.get(x, y).unwrap().symbol;
    assert_eq!(
        (0..5).map(|x| at(x, 17)).collect::<Vec<_>>(),
        ['한', '\0', '글', '\0', ' ']
    );
    assert_eq!(
        (0..5).map(|x| at(x, 18)).collect::<Vec<_>>(),
        ['…', ' ', ' ', ' ', ' ']
    );
}

#[test]
fn test_boxplot_groups_left_out_zero_area_and_offset() {
    let bp = many(100, |i| format!("{i}"));
    assert_zero_areas_draw_nothing(&bp);
    assert_zero_areas_draw_nothing(&bp.clone().horizontal());
    assert_same_when_offset(bp.clone(), 40, 20);
    assert_same_when_offset(bp.horizontal(), 40, 20);
}
