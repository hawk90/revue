//! Waveline layout tests: empty and narrow areas, baseline, background, height, label and the data window

use super::super::{count_char, render, render_rows, rows};
use super::{blank, dot_rows};
use revue::style::Color;
use revue::widget::data::chart::{sine_wave, waveline, WaveStyle, Waveline};

#[test]
fn test_waveline_empty_renders_nothing() {
    assert!(blank(&render(&Waveline::default(), 20, 5)));
    assert!(blank(&render(&Waveline::new(vec![]), 20, 5)));
}

#[test]
fn test_waveline_too_narrow_renders_nothing() {
    assert!(blank(&render(&waveline(vec![1.0]), 1, 5)));
}

#[test]
fn test_waveline_baseline() {
    // Baseline at the bottom: 0 is the bottom row, 1 the top
    let buffer = render(&waveline(vec![0.0, 1.0]).baseline(0.0), 2, 5);
    assert_eq!(dot_rows(&buffer), vec![Some(4), Some(0)]);
}

#[test]
fn test_waveline_baseline_is_clamped() {
    let data = vec![0.0, 0.5, 1.0];
    assert_eq!(
        render_rows(&waveline(data.clone()).baseline(-1.0), 3, 5),
        render_rows(&waveline(data.clone()).baseline(0.0), 3, 5)
    );
    assert_eq!(
        render_rows(&waveline(data.clone()).baseline(2.0), 3, 5),
        render_rows(&waveline(data).baseline(1.0), 3, 5)
    );
}

#[test]
fn test_waveline_show_baseline() {
    let hidden = render(&waveline(vec![1.0]), 4, 5);
    assert_eq!(count_char(&hidden, '─'), 0);

    let wave = waveline(vec![1.0])
        .show_baseline(true)
        .baseline_color(Color::RED);
    let buffer = render(&wave, 4, 5);
    assert_eq!(rows(&buffer)[2], "────");
    assert_eq!(buffer.get(0, 2).unwrap().fg, Some(Color::RED));
}

#[test]
fn test_waveline_bg() {
    let buffer = render(&waveline(vec![0.0]).bg(Color::BLUE), 3, 5);
    for y in 0..5 {
        for x in 0..3 {
            assert_eq!(
                buffer.get(x, y).unwrap().bg,
                Some(Color::BLUE),
                "({x}, {y})"
            );
        }
    }
}

#[test]
fn test_waveline_height() {
    // Only the first 3 rows are used: 1 -> row 0, -1 -> row 2
    let buffer = render(&waveline(vec![-1.0, 1.0]).height(3), 2, 5);
    assert_eq!(dot_rows(&buffer), vec![Some(2), Some(0)]);
    // A height larger than the area is capped
    let tall = render(&waveline(vec![-1.0, 1.0]).height(50), 2, 5);
    assert_eq!(dot_rows(&tall), vec![Some(4), Some(0)]);
}

#[test]
fn test_waveline_max_points_keeps_latest() {
    let wave = waveline(vec![-1.0, -1.0, 0.0, 1.0]).max_points(2);
    let buffer = render(&wave, 2, 5);
    assert_eq!(dot_rows(&buffer), vec![Some(2), Some(0)]);
}

#[test]
fn test_waveline_label() {
    let rows = render_rows(&waveline(vec![-1.0, 1.0]).label("Signal"), 8, 5);
    assert_eq!(rows[0], "Signal  ");
    // The plot uses the 4 rows below the label
    let buffer = render(&waveline(vec![-1.0, 1.0]).label(String::from("S")), 2, 5);
    assert_eq!(dot_rows(&buffer), vec![Some(4), Some(1)]);
}

#[test]
fn test_waveline_zero_area_draws_nothing() {
    for style in [
        WaveStyle::Line,
        WaveStyle::Filled,
        WaveStyle::Mirrored,
        WaveStyle::Bars,
        WaveStyle::Dots,
    ] {
        let wave = waveline(sine_wave(20, 1.0, 1.0))
            .style(style)
            .label("sig")
            .bg(Color::BLACK);
        super::super::assert_zero_areas_draw_nothing(&wave);
    }
}

#[test]
fn test_waveline_draws_the_same_when_offset() {
    for style in [
        WaveStyle::Line,
        WaveStyle::Filled,
        WaveStyle::Mirrored,
        WaveStyle::Bars,
        WaveStyle::Dots,
    ] {
        let wave = waveline(sine_wave(30, 2.0, 1.0))
            .style(style)
            .label("sig")
            .show_baseline(true)
            .gradient(Color::BLUE, Color::RED);
        super::super::assert_same_when_offset(wave, 24, 8);
    }
}
