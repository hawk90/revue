//! Chart legend tests

use super::super::{count_char, render, render_rows, rows};
use super::{plain, sym};
use revue::widget::data::chart::{Chart, LegendPosition, Series};

#[test]
fn test_chart_legend_lists_series_in_order() {
    let chart = Chart::new()
        .series(Series::new("Data 1").data(vec![(1.0, 2.0)]))
        .series_vec(vec![
            Series::new("Data 2").data(vec![(1.0, 3.0)]),
            Series::new("Data 3").data(vec![(1.0, 4.0)]),
        ]);
    let rows = render_rows(&chart, 40, 20);
    // Default TopRight: box starts on row 1, entries on rows 2..5
    assert!(rows[2].contains("■ Data 1"), "{:?}", rows[2]);
    assert!(rows[3].contains("■ Data 2"), "{:?}", rows[3]);
    assert!(rows[4].contains("■ Data 3"), "{:?}", rows[4]);
}

#[test]
fn test_chart_legend_positions() {
    // Legend for "Data" is 8 wide and 3 tall
    let corner = |pos: LegendPosition| {
        let chart = plain()
            .legend(pos)
            .series(Series::new("Data").data(vec![(1.0, 2.0)]));
        let buffer = render(&chart, 40, 20);
        let mut found = None;
        for y in 0..20 {
            for x in 0..40 {
                if sym(&buffer, x, y) == '┌' {
                    found = Some((x, y));
                }
            }
        }
        found
    };
    assert_eq!(corner(LegendPosition::TopLeft), Some((9, 1)));
    assert_eq!(corner(LegendPosition::TopRight), Some((31, 1)));
    assert_eq!(corner(LegendPosition::BottomLeft), Some((9, 14)));
    assert_eq!(corner(LegendPosition::BottomRight), Some((31, 14)));
    assert_eq!(corner(LegendPosition::None), None);
}

#[test]
fn test_chart_no_legend() {
    let chart = Chart::new()
        .legend(LegendPosition::TopLeft)
        .no_legend()
        .series(Series::new("Data").data(vec![(1.0, 2.0)]));
    let buffer = render(&chart, 40, 20);
    assert_eq!(count_char(&buffer, '■'), 0);
    assert!(!rows(&buffer).join("").contains("Data"));
}

#[test]
fn test_chart_legend_wider_than_plot() {
    let chart = Chart::new()
        .series(Series::new("A very long series name that does not fit").data(vec![(1.0, 2.0)]));
    let buffer = render(&chart, 20, 10);
    assert_eq!(count_char(&buffer, '■'), 1);
}

#[test]
fn test_chart_legend_taller_than_plot() {
    let mut chart = Chart::new();
    for i in 0..10 {
        chart = chart.series(Series::new(format!("S{i}")).data(vec![(1.0, i as f64)]));
    }
    let buffer = render(&chart, 30, 8);
    assert!(count_char(&buffer, '■') >= 1);
}

/// A legend of more series than u16 holds is clamped to it and clipped to
/// the plot instead of overflowing (#1552)
#[test]
fn test_chart_legend_series_count_near_u16_max() {
    let series: Vec<Series> = (0..65_534).map(|_| Series::new("s")).collect();
    let chart = plain().legend(LegendPosition::TopLeft).series_vec(series);
    let buffer = render(&chart, 40, 10);
    let rows = rows(&buffer);
    // The box is cut at the plot's bottom, and the entries fill every row
    // down to it (#1565)
    assert_eq!(&rows[1][9..], "┌───┐                          ");
    for row in &rows[2..8] {
        assert_eq!(&row[9..], "│■ s│                          ", "{row:?}");
    }
    assert_eq!(&rows[8][8..], "0   0.20  0.40  0.60  0.80     1");
    for y in 2..8 {
        assert_eq!(
            buffer.get(12, y).unwrap().fg,
            Some(revue::style::Color::WHITE),
            "y={y}"
        );
    }
}

/// The box is sized by the names' terminal columns, not their bytes
/// (#1564): "매출" is 6 bytes but 4 columns, so the box is 8 wide, its
/// glyphs and continuation cells styled like the rest of the name
#[test]
fn test_chart_legend_sizes_wide_names_by_columns() {
    let chart = plain()
        .legend(LegendPosition::TopLeft)
        .series(Series::new("매출"))
        .series(Series::new("abcd"));
    let buffer = render(&chart, 30, 8);
    let rows = rows(&buffer);
    assert_eq!(&rows[1][9..], "┌──────┐             ");
    assert_eq!(&rows[2][9..], "│■ 매\0출\0│             ");
    assert_eq!(&rows[3][9..], "│■ abcd│             ");
    assert_eq!(&rows[4][9..], "└──────┘             ");
    let bg = Some(revue::style::Color::rgb(20, 20, 20));
    for x in 12..16 {
        let cell = buffer.get(x, 2).unwrap();
        assert_eq!(cell.fg, buffer.get(12, 3).unwrap().fg, "x={x}");
        assert_eq!(cell.bg, bg, "x={x}");
    }
}

/// A legend cut at the plot's right edge stops its names there: a name
/// does not cross the chart's border, and a wide glyph that would end on
/// it is not drawn (#1564)
#[test]
fn test_chart_legend_names_stop_at_the_plot_edge() {
    use revue::style::Color;
    // The plot ends at column 16, the chart's border; "ab" ends at 14 and
    // "매" would take 15 and 16
    let chart = plain()
        .border(Color::BLUE)
        .legend(LegendPosition::TopLeft)
        .series(Series::new("ab매출"));
    let buffer = render(&chart, 17, 8);
    let rows = rows(&buffer);
    assert_eq!(rows[2], "│   0.83  ┌─────│");
    assert_eq!(rows[3], "│   0.61  │■ ab │");
    assert_eq!(rows[4], "│  -0.17  └─────│");
    assert_eq!(buffer.get(16, 3).unwrap().fg, Some(Color::BLUE));
    // An ASCII name is cut at the border the same way
    let chart = plain()
        .border(Color::BLUE)
        .legend(LegendPosition::TopLeft)
        .series(Series::new("A very long series name"));
    let rows = render_rows(&chart, 20, 8);
    assert_eq!(rows[3], "│   0.61  │■ A very│");
}
