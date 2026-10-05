//! BoxPlot widget tests (BoxPlot, BoxGroup, BoxStats, WhiskerStyle)

use super::{count_char, render, rows};
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::data::chart::{
    boxplot, Axis, BoxGroup, BoxPlot, BoxStats, ColorScheme, WhiskerStyle,
};

/// Stats spanning 0..100 with quartiles at 25/50/75 and no outliers.
fn even_stats() -> BoxStats {
    BoxStats {
        min: 0.0,
        q1: 25.0,
        median: 50.0,
        q3: 75.0,
        max: 100.0,
        outliers: vec![],
        whisker_low: 0.0,
        whisker_high: 100.0,
    }
}

/// Row `y` of `buffer` as a vector of chars.
fn row_chars(buffer: &Buffer, y: u16) -> Vec<char> {
    rows(buffer)[y as usize].chars().collect()
}

/// Number of cells with symbol `ch` and foreground `fg`.
fn count_colored(buffer: &Buffer, ch: char, fg: Color) -> usize {
    let mut n = 0;
    for y in 0..buffer.height() {
        for x in 0..buffer.width() {
            let cell = buffer.get(x, y).unwrap();
            if cell.symbol == ch && cell.fg == Some(fg) {
                n += 1;
            }
        }
    }
    n
}

// =========================================================================
// WhiskerStyle
// =========================================================================

#[test]
fn test_whisker_style_default() {
    assert_eq!(WhiskerStyle::default(), WhiskerStyle::IQR);
}

#[test]
fn test_whisker_style_variants_distinct() {
    assert_ne!(WhiskerStyle::IQR, WhiskerStyle::MinMax);
    assert_ne!(WhiskerStyle::MinMax, WhiskerStyle::Percentile);
    assert_ne!(WhiskerStyle::IQR, WhiskerStyle::Percentile);
}

// =========================================================================
// BoxStats
// =========================================================================

#[test]
fn test_box_stats_from_data_empty() {
    assert!(BoxStats::from_data(&[], WhiskerStyle::IQR).is_none());
    assert!(BoxStats::from_data(&[f64::NAN], WhiskerStyle::IQR).is_none());
}

#[test]
fn test_box_stats_from_data_single_value() {
    let stats = BoxStats::from_data(&[5.0], WhiskerStyle::IQR).unwrap();
    assert_eq!(stats.min, 5.0);
    assert_eq!(stats.q1, 5.0);
    assert_eq!(stats.median, 5.0);
    assert_eq!(stats.q3, 5.0);
    assert_eq!(stats.max, 5.0);
    assert!(stats.outliers.is_empty());
}

#[test]
fn test_box_stats_from_data_even_count() {
    let data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let stats = BoxStats::from_data(&data, WhiskerStyle::MinMax).unwrap();
    assert_eq!(stats.min, 1.0);
    assert_eq!(stats.max, 10.0);
    assert_eq!(stats.median, 5.5);
}

#[test]
fn test_box_stats_quartiles() {
    // Linear interpolation over 8 points: k = 1.75 and 5.25
    let data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let stats = BoxStats::from_data(&data, WhiskerStyle::MinMax).unwrap();
    assert_eq!(stats.q1, 2.75);
    assert_eq!(stats.q3, 6.25);
}

#[test]
fn test_box_stats_from_data_ignores_non_finite() {
    let data = vec![1.0, f64::NAN, 3.0, f64::INFINITY, 5.0];
    let stats = BoxStats::from_data(&data, WhiskerStyle::IQR).unwrap();
    assert_eq!(stats.min, 1.0);
    assert_eq!(stats.max, 5.0);
    assert_eq!(stats.median, 3.0);
}

#[test]
fn test_box_stats_iqr_whiskers_and_outliers() {
    // Q1 = 2, Q3 = 4, IQR = 2, fences at -1 and 7
    let data = [1.0, 2.0, 3.0, 4.0, 100.0];
    let stats = BoxStats::from_data(&data, WhiskerStyle::IQR).unwrap();
    assert_eq!(stats.min, 1.0);
    assert_eq!(stats.max, 100.0);
    assert_eq!(stats.median, 3.0);
    assert_eq!(stats.outliers, vec![100.0]);
    assert_eq!(stats.whisker_low, 1.0);
    assert_eq!(stats.whisker_high, 4.0);
}

#[test]
fn test_box_stats_iqr_no_outliers() {
    let stats = BoxStats::from_data(&[1.0, 2.0, 3.0, 4.0, 5.0], WhiskerStyle::IQR).unwrap();
    assert!(stats.outliers.is_empty());
    assert_eq!(stats.whisker_low, 1.0);
    assert_eq!(stats.whisker_high, 5.0);
}

#[test]
fn test_box_stats_minmax_whiskers() {
    let stats = BoxStats::from_data(&[1.0, 2.0, 3.0, 4.0, 100.0], WhiskerStyle::MinMax).unwrap();
    assert_eq!(stats.whisker_low, 1.0);
    assert_eq!(stats.whisker_high, 100.0);
    assert!(stats.outliers.is_empty());
}

#[test]
fn test_box_stats_percentile_whiskers() {
    // 5th / 95th percentile of [1..5]: k = 0.2 and 3.8
    let stats = BoxStats::from_data(&[1.0, 2.0, 3.0, 4.0, 5.0], WhiskerStyle::Percentile).unwrap();
    assert!((stats.whisker_low - 1.2).abs() < 1e-9);
    assert!((stats.whisker_high - 4.8).abs() < 1e-9);
    assert_eq!(stats.outliers, vec![1.0, 5.0]);
}

// =========================================================================
// BoxGroup
// =========================================================================

#[test]
fn test_box_group_new() {
    let group = BoxGroup::new("test", &[1.0, 2.0, 3.0, 4.0, 5.0]);
    assert_eq!(group.label, "test");
    assert_eq!(group.data, vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    assert!(group.stats.is_none());
    assert!(group.color.is_none());
}

#[test]
fn test_box_group_empty_data() {
    let group = BoxGroup::new("test", &[]);
    assert!(group.data.is_empty());
    assert!(group.get_stats(WhiskerStyle::IQR).is_none());
}

#[test]
fn test_box_group_from_stats() {
    let group = BoxGroup::from_stats("test", even_stats());
    assert_eq!(group.label, "test");
    assert!(group.data.is_empty());
    assert_eq!(group.stats.as_ref().unwrap().median, 50.0);
    assert!(group.color.is_none());
}

#[test]
fn test_box_group_color() {
    let group = BoxGroup::new("test", &[1.0, 2.0]).color(Color::RED);
    assert_eq!(group.color, Some(Color::RED));
}

#[test]
fn test_box_group_get_stats_prefers_precomputed() {
    let group = BoxGroup::from_stats("test", even_stats());
    // Precomputed stats win regardless of whisker style
    let stats = group.get_stats(WhiskerStyle::MinMax).unwrap();
    assert_eq!(stats.min, 0.0);
    assert_eq!(stats.q1, 25.0);
    assert_eq!(stats.median, 50.0);
    assert_eq!(stats.q3, 75.0);
    assert_eq!(stats.max, 100.0);
}

#[test]
fn test_box_group_get_stats_computes_from_data() {
    let group = BoxGroup::new("test", &[1.0, 2.0, 3.0, 4.0, 5.0]);
    let stats = group.get_stats(WhiskerStyle::MinMax).unwrap();
    assert_eq!(stats.min, 1.0);
    assert_eq!(stats.max, 5.0);
    assert_eq!(stats.median, 3.0);
}

// =========================================================================
// BoxPlot rendering
// =========================================================================

#[test]
fn test_boxplot_new_renders_nothing() {
    let buffer = render(&BoxPlot::new(), 40, 20);
    assert!(rows(&buffer).iter().all(|r| r.trim().is_empty()));
    let buffer = render(&boxplot(), 40, 20);
    assert!(rows(&buffer).iter().all(|r| r.trim().is_empty()));
}

#[test]
fn test_boxplot_too_small_area_renders_nothing() {
    let bp = BoxPlot::new().group("D", &[1.0, 2.0, 3.0]);
    let buffer = render(&bp, 10, 5);
    assert!(rows(&buffer).iter().all(|r| r.trim().is_empty()));
}

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
fn test_boxplot_value_axis_bounds_override() {
    let bp = BoxPlot::new()
        .group_stats("Data", even_stats())
        .value_axis(Axis::new().bounds(0.0, 1000.0));
    let rows = rows(&render(&bp, 40, 20));
    // Padding still applies to the overridden range (-100..1100)
    assert!(rows[1].starts_with("1100"));
    assert!(rows[18].starts_with("-100."));
}

#[test]
fn test_boxplot_category_labels() {
    let bp = BoxPlot::new()
        .group("A", &[1.0, 2.0, 3.0])
        .group("B", &[4.0, 5.0, 6.0]);
    let rows = rows(&render(&bp, 40, 20));
    // Groups split the 34 columns right of the value labels: centers 14 and 31
    let last: Vec<char> = rows[19].chars().collect();
    assert_eq!(last[14], 'A');
    assert_eq!(last[31], 'B');
}

#[test]
fn test_boxplot_long_category_label_clipped() {
    let bp = BoxPlot::new().group("VeryLongGroupName", &[1.0, 2.0, 3.0]);
    let rows = rows(&render(&bp, 20, 10));
    // Label is centered on column 13, so it starts at column 5; cells left
    // of the chart area (the value-label column, x < 6) are not drawn
    assert_eq!(rows[9].trim_end(), "      eryLongGroupNa");
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
fn test_boxplot_add_group_and_group_stats() {
    let bp = BoxPlot::new()
        .add_group(BoxGroup::new("A", &[1.0, 2.0, 3.0]))
        .group_stats("B", even_stats());
    let buffer = render(&bp, 40, 20);
    // A's box is a single row on B's 0..100 scale, so count the median caps
    assert_eq!(count_char(&buffer, '├'), 2);
}

#[test]
fn test_boxplot_title() {
    let bp = BoxPlot::new()
        .title("Distribution")
        .group("A", &[1.0, 2.0, 3.0])
        .group("B", &[4.0, 5.0, 6.0]);
    let rows = rows(&render(&bp, 40, 20));
    // Centered on the first row: (40 - 12) / 2 = 14
    assert_eq!(
        rows[0].trim_end(),
        format!("{}Distribution", " ".repeat(14))
    );
    assert_eq!(
        rows.iter().map(|r| r.matches('┌').count()).sum::<usize>(),
        2
    );
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
fn test_boxplot_bg() {
    let bp = BoxPlot::new().group("A", &[1.0, 2.0, 3.0]).bg(Color::BLUE);
    let buffer = render(&bp, 40, 20);
    assert_eq!(buffer.get(0, 0).unwrap().bg, Some(Color::BLUE));
    assert_eq!(buffer.get(39, 19).unwrap().bg, Some(Color::BLUE));
}

#[test]
fn test_boxplot_horizontal_differs_from_vertical() {
    let data = [1.0, 2.0, 3.0, 4.0, 5.0];
    let v = rows(&render(
        &BoxPlot::new().group("Data", &data).vertical(),
        40,
        20,
    ));
    let h = rows(&render(
        &BoxPlot::new().group("Data", &data).horizontal(),
        40,
        20,
    ));
    assert_ne!(v, h);
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
