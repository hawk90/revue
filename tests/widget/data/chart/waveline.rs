//! Waveline widget tests
//!
//! With the default baseline (0.5) a value v is drawn at
//! y = (1 - (0.5 + v / 2)) * (height - 1): in a 5-row area 1.0 is row 0,
//! 0.0 is row 2 and -1.0 is row 4.

use super::{count_char, render, render_rows, rows};
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::data::chart::{
    audio_waveform, sawtooth_wave, signal_wave, sine_wave, square_wave, waveline, Interpolation,
    WaveStyle, Waveline,
};

fn sym(buffer: &Buffer, x: u16, y: u16) -> char {
    buffer.get(x, y).unwrap().symbol
}

/// Row of the '●' in each column (None if the column is empty).
fn dot_rows(buffer: &Buffer) -> Vec<Option<u16>> {
    (0..buffer.width())
        .map(|x| (0..buffer.height()).find(|&y| sym(buffer, x, y) == '●'))
        .collect()
}

fn blank(buffer: &Buffer) -> bool {
    rows(buffer).iter().all(|r| r.trim().is_empty())
}

// =========================================================================
// Enums
// =========================================================================

#[test]
fn test_enum_defaults() {
    assert_eq!(WaveStyle::default(), WaveStyle::Line);
    assert_eq!(Interpolation::default(), Interpolation::Linear);
}

// =========================================================================
// Data and layout
// =========================================================================

#[test]
fn test_waveline_line() {
    let buffer = render(&waveline(vec![-1.0, 0.0, 1.0]), 3, 5);
    assert_eq!(dot_rows(&buffer), vec![Some(4), Some(2), Some(0)]);
    assert_eq!(count_char(&buffer, '●'), 3);
    assert_eq!(buffer.get(0, 4).unwrap().fg, Some(Color::CYAN));
}

#[test]
fn test_waveline_new_and_data_agree() {
    let a = render_rows(&Waveline::new(vec![-1.0, 0.0, 1.0]), 3, 5);
    let b = render_rows(
        &Waveline::new(vec![1.0; 7]).data(vec![-1.0, 0.0, 1.0]),
        3,
        5,
    );
    assert_eq!(a, b);
}

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
fn test_waveline_clone() {
    let w = waveline(vec![0.0, 1.0]).color(Color::RED);
    assert_eq!(render_rows(&w.clone(), 4, 5), render_rows(&w, 4, 5));
}

#[test]
fn test_waveline_amplitude() {
    // 0.5 sits at row 1; doubled it reaches the top; values are clamped
    let base = render(&waveline(vec![0.5]), 3, 5);
    assert_eq!(dot_rows(&base), vec![Some(1); 3]);
    let doubled = render(&waveline(vec![0.5]).amplitude(2.0), 3, 5);
    assert_eq!(dot_rows(&doubled), vec![Some(0); 3]);
    let huge = render(&waveline(vec![0.5]).amplitude(100.0), 3, 5);
    assert_eq!(dot_rows(&huge), vec![Some(0); 3]);
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
fn test_waveline_color_and_gradient() {
    let buffer = render(&waveline(vec![1.0]).color(Color::RED), 2, 5);
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::RED));

    // The gradient runs from the start color at the bottom to the end at the top
    let start = Color::rgb(0, 0, 0);
    let end = Color::rgb(200, 100, 50);
    let wave = waveline(vec![-1.0, 1.0]).gradient(start, end);
    let buffer = render(&wave, 2, 5);
    assert_eq!(buffer.get(0, 4).unwrap().fg, Some(start));
    assert_eq!(buffer.get(1, 0).unwrap().fg, Some(end));
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

// =========================================================================
// Interpolation
// =========================================================================

#[test]
fn test_interpolation_linear_and_step() {
    let linear = render(&waveline(vec![-1.0, 1.0]), 5, 5);
    assert_eq!(
        dot_rows(&linear),
        vec![Some(4), Some(3), Some(2), Some(1), Some(0)]
    );
    let step = render(
        &waveline(vec![-1.0, 1.0]).interpolation(Interpolation::Step),
        5,
        5,
    );
    assert_eq!(
        dot_rows(&step),
        vec![Some(4), Some(4), Some(4), Some(4), Some(0)]
    );
}

#[test]
fn test_interpolation_curves_pass_through_points() {
    // 13 columns over 4 points: samples land exactly on x = 0, 4, 8, 12
    let data = vec![-1.0, 1.0, -1.0, 1.0];
    let linear = render(&waveline(data.clone()), 13, 9);
    for method in [Interpolation::CatmullRom, Interpolation::Bezier] {
        let curve = render(&waveline(data.clone()).interpolation(method), 13, 9);
        let dots = dot_rows(&curve);
        assert_eq!(dots[0], Some(8), "{method:?}");
        assert_eq!(dots[4], Some(0), "{method:?}");
        assert_eq!(dots[8], Some(8), "{method:?}");
        assert_eq!(dots[12], Some(0), "{method:?}");
        assert_ne!(dots, dot_rows(&linear), "{method:?}");
    }
}

// =========================================================================
// Styles
// =========================================================================

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

// =========================================================================
// Convenience constructors
// =========================================================================

#[test]
fn test_audio_waveform() {
    let data = vec![0.5, 0.7, 0.3];
    assert_eq!(
        render_rows(&audio_waveform(data.clone()), 6, 5),
        render_rows(
            &waveline(data)
                .style(WaveStyle::Mirrored)
                .gradient(Color::CYAN, Color::BLUE),
            6,
            5
        )
    );
}

#[test]
fn test_signal_wave() {
    let data = vec![0.0, 0.5, -0.5];
    let buffer = render(&signal_wave(data.clone()), 6, 5);
    assert_eq!(
        rows(&buffer),
        render_rows(
            &waveline(data)
                .interpolation(Interpolation::CatmullRom)
                .color(Color::GREEN)
                .show_baseline(true),
            6,
            5
        )
    );
    assert!(count_char(&buffer, '─') > 0);
}

// =========================================================================
// Wave generators
// =========================================================================

#[test]
fn test_sine_wave() {
    let data = sine_wave(4, 1.0, 1.0);
    let expected = [0.0, 1.0, 0.0, -1.0];
    assert_eq!(data.len(), 4);
    for (v, e) in data.iter().zip(expected) {
        assert!((v - e).abs() < 1e-9, "{data:?}");
    }
    // Frequency 2 completes a period every 4 of 8 samples
    let data = sine_wave(8, 2.0, 1.0);
    assert!((data[1] - 1.0).abs() < 1e-9 && (data[5] - 1.0).abs() < 1e-9);
    // Amplitude scales
    let data = sine_wave(100, 1.0, 0.5);
    assert!(data.iter().all(|&v| (-0.5..=0.5).contains(&v)));
    assert!(data.iter().any(|&v| (v - 0.5).abs() < 1e-9));
}

#[test]
fn test_square_wave() {
    assert_eq!(square_wave(4, 1.0, 1.0), vec![1.0, 1.0, -1.0, -1.0]);
    assert_eq!(square_wave(4, 2.0, 0.5), vec![0.5, -0.5, 0.5, -0.5]);
}

#[test]
fn test_sawtooth_wave() {
    assert_eq!(sawtooth_wave(4, 1.0, 1.0), vec![-1.0, -0.5, 0.0, 0.5]);
    assert_eq!(sawtooth_wave(2, 1.0, 2.0), vec![-2.0, 0.0]);
}
