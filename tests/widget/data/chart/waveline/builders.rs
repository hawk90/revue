//! Waveline builder tests: data, Clone and the preset constructors

use super::super::{count_char, render, render_rows, rows};
use super::sym;
use revue::style::Color;
use revue::widget::data::chart::{
    area_wave, audio_waveform, signal_wave, spectrum, waveline, Interpolation, WaveStyle, Waveline,
};

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
fn test_waveline_clone() {
    let w = waveline(vec![0.0, 1.0]).color(Color::RED);
    assert_eq!(render_rows(&w.clone(), 4, 5), render_rows(&w, 4, 5));
}

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

#[test]
fn test_area_wave_fills_up_from_the_bottom() {
    let buffer = render(&area_wave(vec![0.0, 0.5, 1.0]), 3, 5);
    assert_eq!(rows(&buffer), vec!["  █", "  ▓", " █▓", " ▓▓", "█▓▓"]);
    assert_eq!(buffer.get(2, 0).unwrap().fg, Some(Color::MAGENTA));
}

#[test]
fn test_spectrum_bars_rise_from_the_bottom() {
    let buffer = render(&spectrum(vec![0.0, 0.5, 1.0]), 3, 5);
    let heights: Vec<usize> = (0..3)
        .map(|x| (0..5).filter(|&y| sym(&buffer, x, y) != ' ').count())
        .collect();
    assert_eq!(heights, vec![1, 3, 5]);
    assert_eq!(buffer.get(2, 4).unwrap().fg, Some(Color::YELLOW));
}
