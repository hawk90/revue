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
    // The box is cut at the plot's bottom; the entries stop a row above it
    assert_eq!(&rows[1][9..], "┌───┐                          ");
    for row in &rows[2..7] {
        assert_eq!(&row[9..], "│■ s│                          ", "{row:?}");
    }
    assert_eq!(&rows[7][9..], "│   │                          ");
    for y in 2..7 {
        assert_eq!(
            buffer.get(12, y).unwrap().fg,
            Some(revue::style::Color::WHITE),
            "y={y}"
        );
    }
}
