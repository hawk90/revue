//! Histogram widget tests
//!
//! Layout used below: a 40x20 histogram without title has its plot area at
//! x=6..39, y=0..18, Y labels in columns 0..6 and X labels on row 19.

use super::{count_char, render, render_rows, rows};
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::data::chart::{histogram, BinConfig, Histogram};

/// Bins [0,10) [10,20) [20,30] holding 3, 1 and 2 values.
fn three_bins() -> Histogram {
    Histogram::new(&[1.0, 2.0, 3.0, 11.0, 21.0, 22.0])
        .bins(BinConfig::Edges(vec![0.0, 10.0, 20.0, 30.0]))
}

fn sym(buffer: &Buffer, x: u16, y: u16) -> char {
    buffer.get(x, y).unwrap().symbol
}

/// Row on which the bar starting at column `x` has its top cell.
fn bar_top(buffer: &Buffer, x: u16) -> Option<u16> {
    (0..18).find(|&y| sym(buffer, x, y) == '▀')
}

/// Number of bars, counted by their border cells on the bottom plot row.
fn bar_count(hist: Histogram) -> usize {
    let buffer = render(&hist.bar_border(Color::WHITE), 40, 20);
    let bottom: String = (6..39).map(|x| sym(&buffer, x, 17)).collect();
    bottom.matches('│').count() / 2
}

#[test]
fn test_histogram_bars() {
    // 33 columns over 0..30: bars at 6..17, 17..28, 28..39; heights 18, 6, 12
    let buffer = render(&three_bins(), 40, 20);
    assert_eq!(bar_top(&buffer, 6), Some(0));
    assert_eq!(bar_top(&buffer, 17), Some(12));
    assert_eq!(bar_top(&buffer, 28), Some(6));
    for y in 1..18 {
        assert_eq!(sym(&buffer, 6, y), '█', "y={y}");
    }
    assert_eq!(count_char(&buffer, '▀'), 33);
}

#[test]
fn test_histogram_axis_labels() {
    let rows = render_rows(&three_bins(), 40, 20);
    // Y: count from max (3) down to 0
    assert!(rows[1].starts_with("3 "), "{:?}", rows[1]);
    assert!(rows[18].starts_with("0 "), "{:?}", rows[18]);
    // X: bin range 0..30 in five ticks
    assert!(
        rows[19].starts_with("      0.00    7.5      15.0    22.5"),
        "{:?}",
        rows[19]
    );
}

#[test]
fn test_histogram_density() {
    // Same shape as counts, scaled to densities; labels get two decimals
    let buffer = render(&three_bins().density(true), 40, 20);
    assert_eq!(bar_top(&buffer, 6), Some(0));
    assert_eq!(bar_top(&buffer, 17), Some(12));
    assert_eq!(bar_top(&buffer, 28), Some(6));
    assert!(rows(&buffer)[1].starts_with("0.05"));
}

#[test]
fn test_histogram_cumulative() {
    // Cumulative frequency 0.5, 0.67, 1.0 of 18 rows: heights 9, 12, 18
    let buffer = render(&three_bins().cumulative(true), 40, 20);
    assert_eq!(bar_top(&buffer, 6), Some(9));
    assert_eq!(bar_top(&buffer, 17), Some(6));
    assert_eq!(bar_top(&buffer, 28), Some(0));
    assert!(rows(&buffer)[1].starts_with("1.00"));
}

#[test]
fn test_histogram_show_stats() {
    // mean 10 -> column 17, median 7 -> column 13
    let plain = render(&three_bins(), 40, 20);
    assert_eq!(count_char(&plain, 'μ'), 0);

    let buffer = render(&three_bins().show_stats(true), 40, 20);
    for y in 1..18 {
        assert_eq!(sym(&buffer, 17, y), '│', "mean y={y}");
        assert_eq!(sym(&buffer, 13, y), '┊', "median y={y}");
    }
    assert_eq!(sym(&buffer, 18, 0), 'μ');
    assert_eq!(sym(&buffer, 14, 0), 'M');
}

#[test]
fn test_histogram_mean_median() {
    let hist = Histogram::new(&[1.0, 2.0, 3.0, 4.0, 5.0]);
    assert_eq!(hist.mean(), Some(3.0));
    assert_eq!(hist.median(), Some(3.0));
    let hist = Histogram::new(&[1.0, 2.0, 3.0, 4.0]);
    assert_eq!(hist.median(), Some(2.5));
    let empty = Histogram::new(&[]);
    assert_eq!(empty.mean(), None);
    assert_eq!(empty.median(), None);
}

#[test]
fn test_histogram_bin_configs() {
    let data: Vec<f64> = (0..100).map(|x| x as f64).collect();
    // Sturges' rule: 8 bins for 100 values
    assert_eq!(bar_count(Histogram::new(&data)), 8);
    assert_eq!(bar_count(Histogram::new(&data).bin_count(10)), 10);
    assert_eq!(bar_count(Histogram::new(&data).bin_width(10.0)), 10);
    assert_eq!(
        bar_count(Histogram::new(&data).bins(BinConfig::Count(15))),
        15
    );
}

#[test]
fn test_histogram_data_recomputes_bins() {
    let replaced = Histogram::new(&[100.0, 200.0])
        .bins(BinConfig::Edges(vec![0.0, 10.0, 20.0, 30.0]))
        .data(&[1.0, 2.0, 3.0, 11.0, 21.0, 22.0]);
    assert_eq!(
        render_rows(&replaced, 40, 20),
        render_rows(&three_bins(), 40, 20)
    );
}

#[test]
fn test_histogram_fill_color_and_alias() {
    let buffer = render(&three_bins().fill_color(Color::YELLOW), 40, 20);
    assert_eq!(buffer.get(6, 17).unwrap().fg, Some(Color::YELLOW));
    let buffer = render(&three_bins().color(Color::CYAN), 40, 20);
    assert_eq!(buffer.get(6, 17).unwrap().fg, Some(Color::CYAN));
    // Default fill
    let buffer = render(&three_bins(), 40, 20);
    assert_eq!(
        buffer.get(6, 17).unwrap().fg,
        Some(Color::rgb(97, 175, 239))
    );
}

#[test]
fn test_histogram_bar_border() {
    let buffer = render(&three_bins().bar_border(Color::WHITE), 40, 20);
    // First bar spans columns 6..17: border on its first and last column
    assert_eq!(sym(&buffer, 6, 10), '│');
    assert_eq!(sym(&buffer, 16, 10), '│');
    assert_eq!(sym(&buffer, 7, 10), '█');
    assert_eq!(buffer.get(6, 10).unwrap().fg, Some(Color::WHITE));
}

#[test]
fn test_histogram_title() {
    let rows = render_rows(&three_bins().title("Distribution"), 40, 20);
    // Centered: (40 - 12) / 2 = 14; the plot moves down one row
    assert_eq!(
        rows[0].trim_end(),
        format!("{}Distribution", " ".repeat(14))
    );
}

#[test]
fn test_histogram_bg() {
    let buffer = render(&three_bins().bg(Color::BLUE), 40, 20);
    assert_eq!(buffer.get(39, 0).unwrap().bg, Some(Color::BLUE));
}

#[test]
fn test_histogram_helper_matches_new() {
    let data = [1.0, 2.0, 3.0, 5.0, 8.0];
    assert_eq!(
        render_rows(&histogram(&data), 40, 20),
        render_rows(&Histogram::new(&data), 40, 20)
    );
}

#[test]
fn test_histogram_empty_renders_nothing() {
    let buffer = render(&Histogram::new(&[]), 40, 20);
    assert!(rows(&buffer).iter().all(|r| r.trim().is_empty()));
}

#[test]
fn test_histogram_too_small_renders_nothing() {
    for (w, h) in [(14, 20), (40, 4)] {
        let buffer = render(&three_bins(), w, h);
        assert!(rows(&buffer).iter().all(|r| r.trim().is_empty()), "{w}x{h}");
    }
}

#[test]
fn test_histogram_horizontal_differs_from_vertical() {
    let v = render_rows(&three_bins().vertical(), 40, 20);
    let h = render_rows(&three_bins().horizontal(), 40, 20);
    assert_ne!(v, h);
}

#[test]
fn test_histogram_horizontal_bars() {
    // Same plot area; the 18 rows cover 0..30, so the bins take rows 0..6,
    // 6..12 and 12..18, and bars of 3, 1 and 2 grow right over 33 columns
    // to lengths 33, 11 and 22, each ending in a half block
    let buffer = render(&three_bins().horizontal(), 40, 20);
    let rows = rows(&buffer);
    let bar = |y: usize| -> String { rows[y].chars().skip(6).collect::<String>() };
    for (ys, len) in [(0..6, 33), (6..12, 11), (12..18, 22)] {
        for y in ys {
            let expected = format!("{}▌", "█".repeat(len - 1));
            assert_eq!(bar(y).trim_end(), expected, "row {y}");
        }
    }
    // No vertical bar tops
    assert_eq!(count_char(&buffer, '▀'), 0);
}

#[test]
fn test_histogram_horizontal_axis_labels() {
    let rows = render_rows(&three_bins().horizontal(), 40, 20);
    // Bin values down the left side, lowest first
    assert!(rows[0].starts_with("0.00 "), "{:?}", rows[0]);
    assert!(rows[4].starts_with("7.5 "), "{:?}", rows[4]);
    assert!(rows[17].starts_with("30.0 "), "{:?}", rows[17]);
    // Counts along the bottom, from 0 on the left to the maximum on the right
    assert!(rows[19].starts_with("      0 "), "{:?}", rows[19]);
    assert!(rows[19].trim_end().ends_with('3'), "{:?}", rows[19]);
}

#[test]
fn test_histogram_horizontal_show_stats() {
    // mean 10 -> row 6, median 7 -> row 4, each across the whole plot
    let buffer = render(&three_bins().horizontal().show_stats(true), 40, 20);
    for x in 6..39 {
        assert_eq!(sym(&buffer, x, 6), '─', "mean x={x}");
        assert_eq!(sym(&buffer, x, 4), '┄', "median x={x}");
    }
    assert_eq!(sym(&buffer, 38, 7), 'μ');
    assert_eq!(sym(&buffer, 38, 5), 'M');
}
