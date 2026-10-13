//! Histogram bar tests: placement, colors, borders and both orientations

use super::super::{count_char, render, render_rows, rows};
use super::{bar_top, sym, three_bins};
use proptest::prelude::*;
use revue::style::Color;
use revue::widget::data::chart::histogram;

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

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// Bars stay inside the plot (columns 6..w-1, rows 0..h-2 without a
    /// title) for any finite data, size and orientation.
    #[test]
    fn test_histogram_bars_stay_in_the_plot(
        data in prop::collection::vec(-1e6f64..1e6, 1..40),
        w in 15u16..80,
        h in 5u16..30,
        horizontal in any::<bool>(),
    ) {
        let hist = if horizontal { histogram(&data).horizontal() } else { histogram(&data) };
        let buffer = render(&hist, w, h);
        for y in 0..h {
            for x in 0..w {
                if matches!(sym(&buffer, x, y), '█' | '▀' | '▌') {
                    prop_assert!((6..w - 1).contains(&x) && y < h - 2, "bar at ({x}, {y})");
                }
            }
        }
    }
}
