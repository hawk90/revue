//! Waveline tests extracted from src/widget/data/chart/waveline.rs
//!
//! This file contains tests for the Waveline chart widget:
//! - Waveline creation and configuration
//! - WaveStyle enum (Line, Filled, Mirrored, Bars, Dots, Smooth)
//! - Interpolation enum (Linear, Bezier, CatmullRom, Step)
//! - Wave generation functions (sine, square, sawtooth)
//! - Rendering tests for all styles

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::data::chart::waveline::{
    area_wave, audio_waveform, signal_wave, spectrum, sine_wave, square_wave, sawtooth_wave,
    waveline, Interpolation, WaveStyle, Waveline,
};
use revue::widget::traits::RenderContext;

// =========================================================================
// Waveline creation tests
// =========================================================================

#[test]
fn test_waveline_creation() {
    let data = vec![0.0, 0.5, 1.0, 0.5, 0.0];
    let wave = waveline(data.clone());

    assert_eq!(wave.data, data);
}

#[test]
fn test_waveline_styles() {
    let data = vec![0.5; 10];
    let wave = waveline(data)
        .style(WaveStyle::Mirrored)
        .color(Color::RED)
        .amplitude(0.8);

    assert_eq!(wave.style, WaveStyle::Mirrored);
    assert_eq!(wave.color, Color::RED);
    assert_eq!(wave.amplitude, 0.8);
}

#[test]
fn test_sine_wave_generation() {
    let data = sine_wave(100, 2.0, 1.0);
    assert_eq!(data.len(), 100);
    assert!(data.iter().all(|&v| v >= -1.0 && v <= 1.0));
}

#[test]
fn test_interpolation() {
    let wave = waveline(vec![0.0, 1.0, 0.0]).interpolation(Interpolation::CatmullRom);

    assert_eq!(wave.interpolation, Interpolation::CatmullRom);
}

// =========================================================================
// WaveStyle enum tests
// =========================================================================

#[test]
fn test_wave_style_default() {
    assert_eq!(WaveStyle::default(), WaveStyle::Line);
}

#[test]
fn test_wave_style_clone() {
    let style1 = WaveStyle::Filled;
    let style2 = style1.clone();
    assert_eq!(style1, style2);
}

#[test]
fn test_wave_style_copy() {
    let style1 = WaveStyle::Bars;
    let style2 = style1;
    assert_eq!(style2, WaveStyle::Bars);
}

#[test]
fn test_wave_style_partial_eq() {
    assert_eq!(WaveStyle::Line, WaveStyle::Line);
    assert_eq!(WaveStyle::Filled, WaveStyle::Filled);
    assert_ne!(WaveStyle::Line, WaveStyle::Mirrored);
}

#[test]
fn test_wave_style_debug() {
    let debug_str = format!("{:?}", WaveStyle::Line);
    assert!(debug_str.contains("Line"));
}

// =========================================================================
// Interpolation enum tests
// =========================================================================

#[test]
fn test_interpolation_default() {
    assert_eq!(Interpolation::default(), Interpolation::Linear);
}

#[test]
fn test_interpolation_clone() {
    let interp1 = Interpolation::Bezier;
    let interp2 = interp1.clone();
    assert_eq!(interp1, interp2);
}

#[test]
fn test_interpolation_copy() {
    let interp1 = Interpolation::CatmullRom;
    let interp2 = interp1;
    assert_eq!(interp2, Interpolation::CatmullRom);
}

#[test]
fn test_interpolation_partial_eq() {
    assert_eq!(Interpolation::Linear, Interpolation::Linear);
    assert_eq!(Interpolation::Bezier, Interpolation::Bezier);
    assert_ne!(Interpolation::Linear, Interpolation::Step);
}

#[test]
fn test_interpolation_debug() {
    let debug_str = format!("{:?}", Interpolation::Bezier);
    assert!(debug_str.contains("Bezier"));
}

// =========================================================================
// Waveline::new tests
// =========================================================================

#[test]
fn test_waveline_new_empty() {
    let wave = Waveline::new(vec![]);
    assert!(wave.data.is_empty());
    assert_eq!(wave.style, WaveStyle::Line);
    assert_eq!(wave.color, Color::CYAN);
}

#[test]
fn test_waveline_new_with_data() {
    let data = vec![0.5, 0.7, 0.3, 0.9];
    let wave = Waveline::new(data.clone());
    assert_eq!(wave.data, data);
}

// =========================================================================
// Waveline::data tests
// =========================================================================

#[test]
fn test_waveline_data_builder() {
    let wave = Waveline::new(vec![0.5]).data(vec![0.1, 0.2, 0.3]);
    assert_eq!(wave.data, vec![0.1, 0.2, 0.3]);
}

// =========================================================================
// Waveline::style tests
// =========================================================================

#[test]
fn test_waveline_style_filled() {
    let wave = Waveline::new(vec![0.5]).style(WaveStyle::Filled);
    assert_eq!(wave.style, WaveStyle::Filled);
}

#[test]
fn test_waveline_style_mirrored() {
    let wave = Waveline::new(vec![0.5]).style(WaveStyle::Mirrored);
    assert_eq!(wave.style, WaveStyle::Mirrored);
}

#[test]
fn test_waveline_style_bars() {
    let wave = Waveline::new(vec![0.5]).style(WaveStyle::Bars);
    assert_eq!(wave.style, WaveStyle::Bars);
}

#[test]
fn test_waveline_style_dots() {
    let wave = Waveline::new(vec![0.5]).style(WaveStyle::Dots);
    assert_eq!(wave.style, WaveStyle::Dots);
}

#[test]
fn test_waveline_style_smooth() {
    let wave = Waveline::new(vec![0.5]).style(WaveStyle::Smooth);
    assert_eq!(wave.style, WaveStyle::Smooth);
}

// =========================================================================
// Waveline::interpolation tests
// =========================================================================

#[test]
fn test_waveline_interpolation_linear() {
    let wave = Waveline::new(vec![0.5]).interpolation(Interpolation::Linear);
    assert_eq!(wave.interpolation, Interpolation::Linear);
}

#[test]
fn test_waveline_interpolation_bezier() {
    let wave = Waveline::new(vec![0.5]).interpolation(Interpolation::Bezier);
    assert_eq!(wave.interpolation, Interpolation::Bezier);
}

#[test]
fn test_waveline_interpolation_step() {
    let wave = Waveline::new(vec![0.5]).interpolation(Interpolation::Step);
    assert_eq!(wave.interpolation, Interpolation::Step);
}

// =========================================================================
// Waveline::color tests
// =========================================================================

#[test]
fn test_waveline_color() {
    let wave = Waveline::new(vec![0.5]).color(Color::RED);
    assert_eq!(wave.color, Color::RED);
}

// =========================================================================
// Waveline::gradient tests
// =========================================================================

#[test]
fn test_waveline_gradient() {
    let wave = Waveline::new(vec![0.5]).gradient(Color::BLUE, Color::GREEN);
    assert_eq!(wave.color, Color::BLUE);
    assert_eq!(wave.gradient_color, Some(Color::GREEN));
}

// =========================================================================
// Waveline::baseline tests
// =========================================================================

#[test]
fn test_waveline_baseline() {
    let wave = Waveline::new(vec![0.5]).baseline(0.7);
    assert_eq!(wave.baseline, 0.7);
}

#[test]
fn test_waveline_baseline_clamp_low() {
    let wave = Waveline::new(vec![0.5]).baseline(-0.5);
    assert_eq!(wave.baseline, 0.0);
}

#[test]
fn test_waveline_baseline_clamp_high() {
    let wave = Waveline::new(vec![0.5]).baseline(1.5);
    assert_eq!(wave.baseline, 1.0);
}

// =========================================================================
// Waveline::amplitude tests
// =========================================================================

#[test]
fn test_waveline_amplitude() {
    let wave = Waveline::new(vec![0.5]).amplitude(2.0);
    assert_eq!(wave.amplitude, 2.0);
}

// =========================================================================
// Waveline::show_baseline tests
// =========================================================================

#[test]
fn test_waveline_show_baseline_true() {
    let wave = Waveline::new(vec![0.5]).show_baseline(true);
    assert!(wave.show_baseline);
}

#[test]
fn test_waveline_show_baseline_false() {
    let wave = Waveline::new(vec![0.5]).show_baseline(false);
    assert!(!wave.show_baseline);
}

// =========================================================================
// Waveline::baseline_color tests
// =========================================================================

#[test]
fn test_waveline_baseline_color() {
    let wave = Waveline::new(vec![0.5]).baseline_color(Color::YELLOW);
    assert_eq!(wave.baseline_color, Color::YELLOW);
}

// =========================================================================
// Waveline::bg tests
// =========================================================================

#[test]
fn test_waveline_bg() {
    let wave = Waveline::new(vec![0.5]).bg(Color::BLACK);
    assert_eq!(wave.bg_color, Some(Color::BLACK));
}

// =========================================================================
// Waveline::height tests
// =========================================================================

#[test]
fn test_waveline_height() {
    let wave = Waveline::new(vec![0.5]).height(10);
    assert_eq!(wave.height, Some(10));
}

// =========================================================================
// Waveline::max_points tests
// =========================================================================

#[test]
fn test_waveline_max_points() {
    let wave = Waveline::new(vec![0.5]).max_points(100);
    assert_eq!(wave.max_points, Some(100));
}

// =========================================================================
// Waveline::label tests
// =========================================================================

#[test]
fn test_waveline_label_str() {
    let wave = Waveline::new(vec![0.5]).label("Audio");
    assert_eq!(wave.label, Some("Audio".to_string()));
}

#[test]
fn test_waveline_label_string() {
    let wave = Waveline::new(vec![0.5]).label(String::from("Wave"));
    assert_eq!(wave.label, Some("Wave".to_string()));
}

// =========================================================================
// Waveline Default trait
// =========================================================================

#[test]
fn test_waveline_default() {
    let wave = Waveline::default();
    assert!(wave.data.is_empty());
    assert_eq!(wave.style, WaveStyle::Line);
}

// =========================================================================
// Waveline Clone trait
// =========================================================================

#[test]
fn test_waveline_clone() {
    let wave1 = Waveline::new(vec![0.5, 0.7]).color(Color::RED);
    let wave2 = wave1.clone();
    assert_eq!(wave1.data, wave2.data);
    assert_eq!(wave1.color, wave2.color);
}

// =========================================================================
// Waveline Debug trait
// =========================================================================

#[test]
fn test_waveline_debug() {
    let wave = Waveline::new(vec![0.5]);
    let debug_str = format!("{:?}", wave);
    assert!(debug_str.contains("Waveline"));
}

// =========================================================================
// Wave generation helper tests
// =========================================================================

#[test]
fn test_sine_wave_frequency() {
    let data = sine_wave(100, 1.0, 1.0);
    assert_eq!(data.len(), 100);
}

#[test]
fn test_sine_wave_amplitude() {
    let data = sine_wave(100, 1.0, 0.5);
    // All values should be within amplitude
    assert!(data.iter().all(|&v| v >= -0.5 && v <= 0.5));
}

#[test]
fn test_square_wave_generation() {
    let data = square_wave(100, 1.0, 1.0);
    assert_eq!(data.len(), 100);
    // Square wave should only have two values
    let unique_vals: std::collections::HashSet<_> =
        data.iter().map(|&v| (v * 10.0).round() as i32).collect();
    assert_eq!(unique_vals.len(), 2);
}

#[test]
fn test_sawtooth_wave_generation() {
    let data = sawtooth_wave(100, 1.0, 1.0);
    assert_eq!(data.len(), 100);
    assert!(data.iter().all(|&v| v >= -1.0 && v <= 1.0));
}

// =========================================================================
// Convenience constructor tests
// =========================================================================

#[test]
fn test_audio_waveform() {
    let wave = audio_waveform(vec![0.5, 0.7, 0.3]);
    assert_eq!(wave.style, WaveStyle::Mirrored);
    assert_eq!(wave.color, Color::CYAN);
    assert!(wave.gradient_color.is_some());
}

#[test]
fn test_signal_wave() {
    let wave = signal_wave(vec![0.5, 0.7, 0.3]);
    assert_eq!(wave.style, WaveStyle::Line);
    assert_eq!(wave.interpolation, Interpolation::CatmullRom);
    assert_eq!(wave.color, Color::GREEN);
    assert!(wave.show_baseline);
}

#[test]
fn test_area_wave() {
    let wave = area_wave(vec![0.5, 0.7, 0.3]);
    assert_eq!(wave.style, WaveStyle::Filled);
    assert_eq!(wave.color, Color::MAGENTA);
    assert_eq!(wave.baseline, 1.0);
}

#[test]
fn test_spectrum() {
    let wave = spectrum(vec![0.5, 0.7, 0.3]);
    assert_eq!(wave.style, WaveStyle::Bars);
    assert_eq!(wave.color, Color::YELLOW);
    assert_eq!(wave.baseline, 1.0);
}

// =========================================================================
// Builder chain tests
// =========================================================================

#[test]
fn test_waveline_builder_chain() {
    let wave = Waveline::new(vec![0.5, 0.7, 0.3])
        .style(WaveStyle::Mirrored)
        .interpolation(Interpolation::Bezier)
        .color(Color::RED)
        .gradient(Color::RED, Color::YELLOW)
        .baseline(0.3)
        .amplitude(1.5)
        .show_baseline(true)
        .baseline_color(Color::rgb(128, 128, 128))
        .bg(Color::BLACK)
        .height(15)
        .max_points(200)
        .label("Test Wave");

    assert_eq!(wave.style, WaveStyle::Mirrored);
    assert_eq!(wave.interpolation, Interpolation::Bezier);
    assert_eq!(wave.color, Color::RED);
    assert_eq!(wave.baseline, 0.3);
    assert_eq!(wave.amplitude, 1.5);
    assert!(wave.show_baseline);
    assert_eq!(wave.height, Some(15));
    assert_eq!(wave.max_points, Some(200));
    assert_eq!(wave.label, Some("Test Wave".to_string()));
}

// =========================================================================
// Render tests
// =========================================================================

#[test]
fn test_waveline_render_basic() {
    let data = vec![0.2, 0.5, 0.8, 0.5, 0.2];
    let wave = Waveline::new(data);

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    wave.render(&mut ctx);
    // Should render without panic
}

#[test]
fn test_waveline_render_with_label() {
    let data = vec![0.5, 0.7, 0.3];
    let wave = Waveline::new(data).label("Audio");

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    wave.render(&mut ctx);

    // Label should be rendered
    let mut label_found = false;
    for x in 0..40 {
        if let Some(cell) = buffer.get(x, 0) {
            if cell.symbol == 'A' {
                label_found = true;
                break;
            }
        }
    }
    assert!(label_found);
}

#[test]
fn test_waveline_render_filled() {
    let data = vec![0.3, 0.7, 0.5, 0.9, 0.4];
    let wave = Waveline::new(data).style(WaveStyle::Filled);

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    wave.render(&mut ctx);
    // Should render filled area
}

#[test]
fn test_waveline_render_mirrored() {
    let data = vec![0.3, 0.7, 0.5, 0.9, 0.4];
    let wave = Waveline::new(data).style(WaveStyle::Mirrored);

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    wave.render(&mut ctx);
    // Should render mirrored wave
}

#[test]
fn test_waveline_render_bars() {
    let data = vec![0.3, 0.7, 0.5];
    let wave = Waveline::new(data).style(WaveStyle::Bars);

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    wave.render(&mut ctx);
    // Should render bars
}

#[test]
fn test_waveline_render_dots() {
    let data = vec![0.3, 0.7, 0.5];
    let wave = Waveline::new(data).style(WaveStyle::Dots);

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    wave.render(&mut ctx);
    // Should render dots
}

#[test]
fn test_waveline_render_smooth() {
    let data = vec![0.2, 0.5, 0.8, 0.5, 0.2];
    let wave = Waveline::new(data).style(WaveStyle::Smooth);

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    wave.render(&mut ctx);
    // Should render smooth curve
}

#[test]
fn test_waveline_render_with_background() {
    let data = vec![0.5, 0.7, 0.3];
    let wave = Waveline::new(data).bg(Color::BLUE);

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    wave.render(&mut ctx);
    // Should render with background
}

#[test]
fn test_waveline_render_with_baseline() {
    let data = vec![0.5, 0.7, 0.3];
    let wave = Waveline::new(data).show_baseline(true);

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    wave.render(&mut ctx);
    // Should render baseline
}

#[test]
fn test_waveline_render_empty() {
    let wave = Waveline::new(vec![]);

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    wave.render(&mut ctx);
    // Should handle empty data gracefully
}

#[test]
fn test_waveline_render_small_area() {
    let data = vec![0.5, 0.7, 0.3];
    let wave = Waveline::new(data);

    let mut buffer = Buffer::new(5, 3);
    let area = Rect::new(0, 0, 5, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    wave.render(&mut ctx);
    // Should handle small area
}

#[test]
fn test_waveline_render_negative_values() {
    let data = vec![-0.5, 0.0, 0.5, 0.0, -0.5];
    let wave = Waveline::new(data);

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    wave.render(&mut ctx);
    // Should handle negative values
}

#[test]
fn test_waveline_render_max_points() {
    let data: Vec<f64> = (0..100).map(|i| i as f64 / 100.0).collect();
    let wave = Waveline::new(data).max_points(20);

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    wave.render(&mut ctx);
    // Should only render last 20 points
}
