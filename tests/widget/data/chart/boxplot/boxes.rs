//! BoxPlot mark tests: boxes, whiskers, medians, outliers, notches and colors, both orientations

use super::super::{count_char, render, rows};
use super::{count_colored, even_stats, row_chars};
use proptest::prelude::*;
use revue::style::Color;
use revue::widget::data::chart::{BoxGroup, BoxPlot, ColorScheme, WhiskerStyle};

#[test]
fn test_boxplot_box_layout() {
    // 40x20, no title: chart area x=6, y=0, 33x18. Bounds are the whisker
    // range padded by 10% (-10..110), mapped onto 18 rows.
    let bp = BoxPlot::new().group_stats("Data", even_stats());
    let buffer = render(&bp, 40, 20);

    // Box is 60% of the 33-wide group (19), centered on column 22
    let (left, right, center) = (13usize, 32usize, 22usize);

    let cap_high = row_chars(&buffer, 2);
    let q3 = row_chars(&buffer, 5);
    let median = row_chars(&buffer, 9);
    let q1 = row_chars(&buffer, 13);
    let cap_low = row_chars(&buffer, 16);

    assert!(cap_high[left..=right].iter().all(|&c| c == '─'));
    assert!(cap_low[left..=right].iter().all(|&c| c == '─'));
    for y in [3, 4, 14, 15] {
        assert_eq!(row_chars(&buffer, y)[center], '│', "whisker row {y}");
    }

    assert_eq!(q3[left], '┌');
    assert_eq!(q3[right], '┐');
    assert_eq!(q1[left], '└');
    assert_eq!(q1[right], '┘');
    assert_eq!(median[left], '├');
    assert_eq!(median[right], '┤');
    assert!(median[left + 1..right].iter().all(|&c| c == '─'));

    // Box sides; the inside is cleared over the whisker
    for y in [6, 7, 8, 10, 11, 12] {
        let row = row_chars(&buffer, y);
        assert_eq!(row[left], '│', "row {y}");
        assert_eq!(row[right], '│', "row {y}");
        assert_eq!(row[center], ' ', "row {y}");
    }

    // Nothing is drawn above the upper whisker or below the lower one
    assert!(row_chars(&buffer, 1)[6..].iter().all(|&c| c == ' '));
    assert!(row_chars(&buffer, 17)[6..].iter().all(|&c| c == ' '));
}

#[test]
fn test_boxplot_multiple_groups() {
    let bp = BoxPlot::new()
        .group("Group A", &[1.0, 2.0, 3.0, 4.0, 5.0])
        .group("Group B", &[3.0, 4.0, 5.0, 6.0, 7.0])
        .group("Group C", &[5.0, 6.0, 7.0, 8.0, 9.0]);
    let buffer = render(&bp, 60, 25);
    assert_eq!(count_char(&buffer, '┌'), 3);
    assert_eq!(count_char(&buffer, '├'), 3);
}

#[test]
fn test_boxplot_outliers_shown_by_default() {
    let mut data: Vec<f64> = (0..20).map(|x| x as f64).collect();
    data.push(100.0);
    data.push(-50.0);
    let buffer = render(&BoxPlot::new().group("Data", &data), 40, 20);
    assert_eq!(count_char(&buffer, '○'), 2);
}

#[test]
fn test_boxplot_show_outliers_false() {
    let mut data: Vec<f64> = (0..20).map(|x| x as f64).collect();
    data.push(100.0);
    let bp = BoxPlot::new().group("Data", &data).show_outliers(false);
    let buffer = render(&bp, 40, 20);
    assert_eq!(count_char(&buffer, '○'), 0);
    // The box itself is still drawn
    assert_eq!(count_char(&buffer, '┌'), 1);
}

#[test]
fn test_boxplot_minmax_whiskers_have_no_outliers() {
    let mut data: Vec<f64> = (0..20).map(|x| x as f64).collect();
    data.push(100.0);
    let bp = BoxPlot::new()
        .group("Data", &data)
        .whisker_style(WhiskerStyle::MinMax);
    let buffer = render(&bp, 40, 20);
    assert_eq!(count_char(&buffer, '○'), 0);
    assert_eq!(count_char(&buffer, '┌'), 1);
}

#[test]
fn test_boxplot_box_width() {
    let width_of = |bp: BoxPlot| {
        let buffer = render(&bp, 40, 20);
        let rows = rows(&buffer);
        let row = rows.iter().find(|r| r.contains('┌')).unwrap();
        let chars: Vec<char> = row.chars().collect();
        let l = chars.iter().position(|&c| c == '┌').unwrap();
        let r = chars.iter().position(|&c| c == '┐').unwrap();
        r - l
    };
    let base = || BoxPlot::new().group_stats("A", even_stats());
    // Default 0.6 of 33 columns = 19, 0.8 of 33 = 26
    assert_eq!(width_of(base()), 19);
    assert_eq!(width_of(base().box_width(0.8)), 26);
}

#[test]
fn test_boxplot_group_colors() {
    let bp = BoxPlot::new()
        .add_group(BoxGroup::new("A", &[1.0, 2.0, 3.0, 4.0, 5.0]).color(Color::RED))
        .group("B", &[1.0, 2.0, 3.0, 4.0, 5.0])
        .colors(ColorScheme::new(vec![Color::GREEN, Color::BLUE]));
    let buffer = render(&bp, 40, 20);
    // A uses its own color; B takes the scheme color for index 1
    assert_eq!(count_colored(&buffer, '┌', Color::RED), 1);
    assert_eq!(count_colored(&buffer, '┌', Color::BLUE), 1);
    assert_eq!(count_colored(&buffer, '┌', Color::GREEN), 0);
}

#[test]
fn test_boxplot_default_palette_colors() {
    let bp = BoxPlot::new()
        .group("A", &[1.0, 2.0, 3.0, 4.0, 5.0])
        .group("B", &[1.0, 2.0, 3.0, 4.0, 5.0]);
    let buffer = render(&bp, 40, 20);
    let palette = ColorScheme::default_palette();
    assert_eq!(count_colored(&buffer, '┌', palette.get(0)), 1);
    assert_eq!(count_colored(&buffer, '┌', palette.get(1)), 1);
}

#[test]
fn test_boxplot_horizontal_layout() {
    // 40x20, no title: the label column is as wide as "Data" plus a gap, so
    // the chart area is x=5, y=0, 34x19 with value labels on row 19. Bounds
    // -10..110 map onto 34 columns; the box is 60% of 19 rows (11), centered
    // on row 9.
    let bp = BoxPlot::new()
        .group_stats("Data", even_stats())
        .horizontal();
    let buffer = render(&bp, 40, 20);
    let (top, bottom, center) = (4u16, 15u16, 9u16);
    let (cap_low, q1, median, q3, cap_high) = (7usize, 14usize, 21usize, 28usize, 35usize);

    // Group label on the left of its center row
    assert!(rows(&buffer)[center as usize].starts_with("Data   │"));

    // Whisker caps are vertical, the whisker itself horizontal
    for y in top..=bottom {
        let row = row_chars(&buffer, y);
        assert_eq!(row[cap_low], '│', "row {y}");
        assert_eq!(row[cap_high], '│', "row {y}");
    }
    let mid = row_chars(&buffer, center);
    assert!(mid[cap_low + 1..q1].iter().all(|&c| c == '─'));
    assert!(mid[q3 + 1..cap_high].iter().all(|&c| c == '─'));

    // Box spans Q1..Q3 left to right with the median as a vertical line
    let top_row = row_chars(&buffer, top);
    let bottom_row = row_chars(&buffer, bottom);
    assert_eq!(top_row[q1], '┌');
    assert_eq!(top_row[q3], '┐');
    assert_eq!(bottom_row[q1], '└');
    assert_eq!(bottom_row[q3], '┘');
    assert_eq!(top_row[median], '┬');
    assert_eq!(bottom_row[median], '┴');
    for y in top + 1..bottom {
        let row = row_chars(&buffer, y);
        assert_eq!(row[q1], '│', "row {y}");
        assert_eq!(row[median], '│', "row {y}");
        assert_eq!(row[q3], '│', "row {y}");
    }
    // The inside is cleared over the whisker
    assert_eq!(mid[q1 + 1], ' ');

    // No vertical-orientation median caps
    assert_eq!(count_char(&buffer, '├'), 0);

    // Value labels run from the padded min to the padded max along the bottom
    let last = &rows(&buffer)[19];
    assert!(last.trim_start().starts_with("-10.0"), "{last:?}");
    assert!(last.trim_end().ends_with("110.0"), "{last:?}");
    assert!(last.contains("50.0"), "{last:?}");
}

#[test]
fn test_boxplot_horizontal_groups_stack_top_to_bottom() {
    let bp = BoxPlot::new()
        .group("A", &[1.0, 2.0, 3.0, 4.0, 5.0])
        .group("Bee", &[3.0, 4.0, 5.0, 6.0, 7.0, 20.0])
        .horizontal();
    let buffer = render(&bp, 40, 20);
    let rows = rows(&buffer);
    // Two bands of 9 rows: centers at rows 4 and 13
    assert!(rows[4].starts_with("A   "), "{rows:?}");
    assert!(rows[13].starts_with("Bee "), "{rows:?}");
    assert_eq!(count_char(&buffer, '┌'), 2);
    assert_eq!(count_char(&buffer, '┬'), 2);
    // Bee's outlier sits on its center row, right of its box
    assert!(rows[13].contains('○'), "{rows:?}");
}

#[test]
fn test_boxplot_notched_differs_from_plain() {
    let data: Vec<f64> = (0..30).map(|x| x as f64).collect();
    let plain = rows(&render(&BoxPlot::new().group("Data", &data), 40, 20));
    let notched = rows(&render(
        &BoxPlot::new().group("Data", &data).notched(true),
        40,
        20,
    ));
    assert_ne!(plain, notched);
}

#[test]
fn test_boxplot_notch_layout() {
    // 0..30: n = 30, Q1 = 7.25, median = 14.5, Q3 = 21.75, so the notch is
    // 14.5 ± 1.57 * 14.5 / sqrt(30) = 10.34..18.66. On the padded -2.9..31.9
    // scale over 18 rows that is rows 7..11 inside the box (rows 5..13),
    // with the median on row 9. The box spans columns 13..32.
    let data: Vec<f64> = (0..30).map(|x| x as f64).collect();
    let buffer = render(&BoxPlot::new().group("Data", &data).notched(true), 40, 20);
    let (left, right) = (13usize, 32usize);

    let start = row_chars(&buffer, 7);
    assert_eq!((start[left], start[right]), ('╲', '╱'));
    let end = row_chars(&buffer, 11);
    assert_eq!((end[left], end[right]), ('╱', '╲'));
    for y in [8, 10] {
        let row = row_chars(&buffer, y);
        assert_eq!((row[left], row[right]), (' ', ' '), "row {y}");
        assert_eq!((row[left + 1], row[right - 1]), ('│', '│'), "row {y}");
    }
    // The median line is pinched in with the sides
    let median = row_chars(&buffer, 9);
    assert_eq!(median[left], ' ');
    assert_eq!(median[left + 1], '├');
    assert_eq!(median[right - 1], '┤');
    assert_eq!(median[right], ' ');
    // Outside the notch the box keeps its full width
    for y in [6, 12] {
        let row = row_chars(&buffer, y);
        assert_eq!((row[left], row[right]), ('│', '│'), "row {y}");
    }

    // The plain box has no notch
    let plain = render(&BoxPlot::new().group("Data", &data), 40, 20);
    assert_eq!(count_char(&plain, '╲') + count_char(&plain, '╱'), 0);
}

#[test]
fn test_boxplot_notch_horizontal() {
    // Same data laid out horizontally: the box is rows 4..15 and columns
    // 14..28; the notch pinches the top and bottom edges in by one row.
    let data: Vec<f64> = (0..30).map(|x| x as f64).collect();
    let bp = BoxPlot::new()
        .group("Data", &data)
        .notched(true)
        .horizontal();
    let buffer = render(&bp, 40, 20);
    let top = rows(&buffer)[4].clone();
    let bottom = rows(&buffer)[15].clone();
    assert_eq!(top.matches('╲').count(), 1, "{top:?}");
    assert_eq!(top.matches('╱').count(), 1, "{top:?}");
    assert_eq!(bottom.matches('╱').count(), 1, "{bottom:?}");
    assert_eq!(bottom.matches('╲').count(), 1, "{bottom:?}");
    // The median line starts and ends inside the notch
    assert!(!top.contains('┬'), "{top:?}");
    assert!(rows(&buffer)[5].contains('┬'));
    assert!(rows(&buffer)[14].contains('┴'));
}

#[test]
fn test_boxplot_notch_needs_sample_size() {
    // Precomputed stats carry no sample size, so there is no interval to draw
    let plain = rows(&render(
        &BoxPlot::new().group_stats("Data", even_stats()),
        40,
        20,
    ));
    let notched = rows(&render(
        &BoxPlot::new()
            .group_stats("Data", even_stats())
            .notched(true),
        40,
        20,
    ));
    assert_eq!(plain, notched);
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// Boxes, medians and outliers stay inside the plot for any finite data,
    /// group count, size and orientation. Vertical: columns 6..w-1, rows
    /// 0..h-2. Horizontal: right of the group-label column and its gap,
    /// rows 0..h-1.
    #[test]
    fn test_boxplot_marks_stay_in_the_plot(
        groups in prop::collection::vec(prop::collection::vec(-1e6f64..1e6, 1..20), 1..4),
        w in 10u16..80,
        h in 8u16..30,
        horizontal in any::<bool>(),
    ) {
        let mut bp = BoxPlot::new();
        for (i, data) in groups.iter().enumerate() {
            bp = bp.group(format!("g{i}"), data);
        }
        if horizontal {
            bp = bp.horizontal();
        }
        let buffer = render(&bp, w, h);
        let (left, bottom) = if horizontal { (2u16.min(w / 3).max(1) + 1, h - 1) } else { (6, h - 2) };
        for y in 0..h {
            for x in 0..w {
                if matches!(
                    buffer.get(x, y).unwrap().symbol,
                    '┌' | '┐' | '└' | '┘' | '├' | '┤' | '┬' | '┴' | '○' | '╲' | '╱'
                ) {
                    prop_assert!((left..w - 1).contains(&x) && y < bottom, "mark at ({x}, {y})");
                }
            }
        }
    }
}
