//! Waveline style tests: line, dots, smooth, filled, mirrored and bars

use super::super::{count_char, render, render_rows};
use super::{dot_rows, sym};
use proptest::prelude::*;
use revue::style::Color;
use revue::widget::data::chart::{waveline, WaveStyle};

#[test]
fn test_waveline_line() {
    let buffer = render(&waveline(vec![-1.0, 0.0, 1.0]), 3, 5);
    assert_eq!(dot_rows(&buffer), vec![Some(4), Some(2), Some(0)]);
    assert_eq!(count_char(&buffer, '●'), 3);
    assert_eq!(buffer.get(0, 4).unwrap().fg, Some(Color::CYAN));
}

#[test]
fn test_style_dots_and_smooth() {
    let data = vec![-1.0, 0.0, 1.0];
    let dots = render(&waveline(data.clone()).style(WaveStyle::Dots), 3, 5);
    assert_eq!(sym(&dots, 0, 4), '⣿');
    assert_eq!(sym(&dots, 1, 2), '⣿');
    assert_eq!(sym(&dots, 2, 0), '⣿');
    assert_eq!(count_char(&dots, '●'), 0);

    // Smooth draws like Line (smoothing comes from the interpolation)
    assert_eq!(
        render_rows(&waveline(data.clone()).style(WaveStyle::Smooth), 3, 5),
        render_rows(&waveline(data), 3, 5)
    );
}

#[test]
fn test_style_filled() {
    // Positive: from the value down to the baseline row
    let up = render_rows(&waveline(vec![1.0]).style(WaveStyle::Filled), 2, 5);
    assert_eq!(up, vec!["██", "▓▓", "▓▓", "  ", "  "]);
    // Negative: from the baseline row down to the value
    let down = render_rows(&waveline(vec![-1.0]).style(WaveStyle::Filled), 2, 5);
    assert_eq!(down, vec!["  ", "  ", "▓▓", "▓▓", "██"]);
}

#[test]
fn test_style_mirrored() {
    let full = render_rows(&waveline(vec![1.0]).style(WaveStyle::Mirrored), 2, 5);
    assert_eq!(full, vec!["▀▀", "██", "██", "██", "▄▄"]);
    // Uses the magnitude
    let negative = render_rows(&waveline(vec![-1.0]).style(WaveStyle::Mirrored), 2, 5);
    assert_eq!(negative, full);
    // Zero is a single cell on the center row
    let zero = render_rows(&waveline(vec![0.0]).style(WaveStyle::Mirrored), 2, 5);
    assert_eq!(zero, vec!["  ", "  ", "▄▄", "  ", "  "]);
}

#[test]
fn test_style_bars() {
    // Baseline at the bottom: a 0.5 bar covers rows 2..4 of 5
    let wave = waveline(vec![0.5]).style(WaveStyle::Bars).baseline(0.0);
    let buffer = render(&wave, 2, 5);
    let column: Vec<char> = (0..5).map(|y| sym(&buffer, 0, y)).collect();
    assert_eq!(&column[..2], &[' ', ' ']);
    assert_ne!(column[2], ' ');
    assert_eq!(&column[3..], &['█', '█']);
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// For any finite data, every style draws inside the area below the
    /// label row.
    #[test]
    fn test_waveline_stays_below_the_label(
        data in prop::collection::vec(-10f64..10.0, 1..60),
        style in 0usize..5,
        baseline in 0.0f64..1.0,
        w in 2u16..40,
        h in 2u16..12,
    ) {
        let style = [WaveStyle::Line, WaveStyle::Filled, WaveStyle::Mirrored, WaveStyle::Bars, WaveStyle::Dots][style];
        let wave = waveline(data).style(style).baseline(baseline).label("L");
        let buffer = render(&wave, w, h);
        // Row 0 holds only the label
        let top: String = (0..w).map(|x| sym(&buffer, x, 0)).collect();
        prop_assert_eq!(top.trim_end(), "L");
    }
}
