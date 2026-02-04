//! Waveline widget tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, StyledView};
use revue::widget::{
    area_wave, audio_waveform, sawtooth_wave, signal_wave, sine_wave, spectrum, square_wave,
    waveline, Interpolation, View, WaveStyle, Waveline,
};

// =============================================================================
// Builder Methods Tests (via rendering behavior)
// =============================================================================

#[test]
fn test_waveline_new() {
    let data = vec![0.0, 0.5, 1.0, 0.5, 0.0];
    let wave = Waveline::new(data);
    // Just ensure it compiles and can be rendered
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave.render(&mut ctx);
}

#[test]
fn test_waveline_default() {
    let wave = Waveline::default();
    // Should render without error even with empty data
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave.render(&mut ctx);
}

#[test]
fn test_waveline_data() {
    let data1 = vec![0.0, 0.5, 1.0];
    let data2 = vec![1.0, 0.5, 0.0];
    let wave = Waveline::new(data1).data(data2);
    // Should render with new data
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave.render(&mut ctx);
}

#[test]
fn test_waveline_style() {
    let styles = [
        WaveStyle::Line,
        WaveStyle::Filled,
        WaveStyle::Mirrored,
        WaveStyle::Bars,
        WaveStyle::Dots,
        WaveStyle::Smooth,
    ];

    for style in styles {
        let wave = waveline(vec![0.5; 20]).style(style);
        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);
        wave.render(&mut ctx);
    }
}

#[test]
fn test_waveline_interpolation() {
    let methods = [
        Interpolation::Linear,
        Interpolation::Bezier,
        Interpolation::CatmullRom,
        Interpolation::Step,
    ];

    for method in methods {
        let wave = waveline(vec![0.0, 0.5, 1.0].repeat(10)).interpolation(method);
        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);
        wave.render(&mut ctx);
    }
}

#[test]
fn test_waveline_color() {
    let wave = waveline(vec![0.5; 10]).color(Color::RED);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave.render(&mut ctx);

    // Verify color is applied
    let mut has_red = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.fg == Some(Color::RED) {
                    has_red = true;
                    break;
                }
            }
        }
    }
    assert!(has_red, "Color should be applied to rendered cells");
}

#[test]
fn test_waveline_gradient() {
    let wave = waveline(vec![0.0, 0.5, 1.0, 0.5, 0.0]).gradient(Color::BLUE, Color::GREEN);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave.render(&mut ctx);

    // Verify gradient colors are applied
    let mut has_color = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.fg.is_some() {
                    has_color = true;
                    break;
                }
            }
        }
    }
    assert!(has_color, "Gradient colors should be applied");
}

#[test]
fn test_waveline_baseline() {
    let wave = waveline(vec![0.5; 10]).baseline(0.75);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave.render(&mut ctx);
}

#[test]
fn test_waveline_baseline_clamping() {
    // Test clamping with extreme values
    let wave_high = waveline(vec![0.5; 10]).baseline(1.5);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave_high.render(&mut ctx);

    let wave_low = waveline(vec![0.5; 10]).baseline(-0.5);
    let mut ctx2 = RenderContext::new(&mut buffer, area);
    wave_low.render(&mut ctx2);
}

#[test]
fn test_waveline_amplitude() {
    let wave = waveline(vec![0.5; 10]).amplitude(2.5);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave.render(&mut ctx);
}

#[test]
fn test_waveline_show_baseline() {
    let wave = waveline(vec![0.5; 10]).show_baseline(true);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave.render(&mut ctx);

    // Check for baseline characters
    let mut has_baseline = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '─' {
                    has_baseline = true;
                    break;
                }
            }
        }
    }
    assert!(
        has_baseline,
        "Baseline should be rendered when show_baseline is true"
    );
}

#[test]
fn test_waveline_baseline_color() {
    let wave = waveline(vec![0.5; 10])
        .show_baseline(true)
        .baseline_color(Color::YELLOW);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave.render(&mut ctx);

    // Verify baseline color
    let mut has_yellow_baseline = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '─' && cell.fg == Some(Color::YELLOW) {
                    has_yellow_baseline = true;
                    break;
                }
            }
        }
    }
    assert!(
        has_yellow_baseline,
        "Baseline should have the specified color"
    );
}

#[test]
fn test_waveline_bg() {
    let wave = waveline(vec![0.5; 10]).bg(Color::BLACK);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave.render(&mut ctx);

    // Check background color
    if let Some(cell) = buffer.get(0, 0) {
        assert_eq!(cell.bg, Some(Color::BLACK));
    }
}

#[test]
fn test_waveline_height() {
    let wave = waveline(vec![0.5; 10]).height(3);
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave.render(&mut ctx);

    // Check that area below height is not affected
    for y in 4..10 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                assert_eq!(cell.symbol, ' ');
            }
        }
    }
}

#[test]
fn test_waveline_max_points() {
    let data: Vec<f64> = (0..100).map(|i| i as f64 / 100.0).collect();
    let wave = waveline(data).max_points(10);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave.render(&mut ctx);

    // Should render successfully
    let mut has_content = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol != ' ' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_waveline_label() {
    let wave = waveline(vec![0.5; 10]).label("Audio Level");
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave.render(&mut ctx);

    // Check that label is rendered
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'A');
    assert_eq!(buffer.get(1, 0).unwrap().symbol, 'u');
}

#[test]
fn test_waveline_label_string() {
    let wave = waveline(vec![0.5; 10]).label(String::from("Test"));
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'T');
}

// =============================================================================
// Convenience Constructors Tests
// =============================================================================

#[test]
fn test_waveline_helper() {
    let data = vec![0.0, 0.5, 1.0];
    let wave = waveline(data);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave.render(&mut ctx);
}

#[test]
fn test_audio_waveform_helper() {
    let samples = vec![0.1, 0.2, 0.3, 0.2, 0.1];
    let wave = audio_waveform(samples);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave.render(&mut ctx);

    // Audio waveform uses Mirrored style - should render mirrored chars
    let mut has_mirrored = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' || cell.symbol == '▀' || cell.symbol == '▄' {
                    has_mirrored = true;
                    break;
                }
            }
        }
    }
    assert!(has_mirrored, "Audio waveform should render mirrored style");
}

#[test]
fn test_signal_wave_helper() {
    let data = vec![0.0, 0.5, 1.0, 0.5, 0.0];
    let wave = signal_wave(data);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave.render(&mut ctx);

    // Signal wave uses CatmullRom interpolation - should render smoothly
    let mut has_output = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol != ' ' && cell.symbol != '─' {
                    has_output = true;
                    break;
                }
            }
        }
    }
    assert!(has_output);
}

#[test]
fn test_area_wave_helper() {
    let data = vec![0.0, 0.5, 1.0, 0.5, 0.0];
    let wave = area_wave(data);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave.render(&mut ctx);

    // Area wave uses Filled style - should have filled characters
    let mut has_filled = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' || cell.symbol == '▓' {
                    has_filled = true;
                    break;
                }
            }
        }
    }
    assert!(has_filled, "Area wave should render filled style");
}

#[test]
fn test_spectrum_helper() {
    let data = vec![0.2, 0.5, 0.8, 0.6, 0.4];
    let wave = spectrum(data);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave.render(&mut ctx);

    // Spectrum uses Bars style - should have bar characters
    let bar_chars = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
    let mut has_bars = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if bar_chars.contains(&cell.symbol) {
                    has_bars = true;
                    break;
                }
            }
        }
    }
    assert!(has_bars, "Spectrum should render bar characters");
}

// =============================================================================
// Wave Generation Tests
// =============================================================================

#[test]
fn test_sine_wave_generation() {
    let data = sine_wave(100, 2.0, 1.0);
    assert_eq!(data.len(), 100);
    assert!(data.iter().all(|&v| v >= -1.0 && v <= 1.0));

    // Check that sine wave oscillates
    let max = data.iter().cloned().fold(f64::NAN, f64::max);
    let min = data.iter().cloned().fold(f64::NAN, f64::min);
    assert!(
        max > 0.9,
        "Sine wave should reach positive values close to 1.0"
    );
    assert!(
        min < -0.9,
        "Sine wave should reach negative values close to -1.0"
    );
}

#[test]
fn test_sine_wave_different_params() {
    let data1 = sine_wave(50, 1.0, 0.5);
    assert_eq!(data1.len(), 50);
    assert!(data1.iter().all(|&v| v >= -0.5 && v <= 0.5));

    let data2 = sine_wave(200, 5.0, 0.8);
    assert_eq!(data2.len(), 200);
    assert!(data2.iter().all(|&v| v >= -0.8 && v <= 0.8));
}

#[test]
fn test_sine_wave_rendering() {
    let data = sine_wave(50, 2.0, 0.8);
    let wave = waveline(data).style(WaveStyle::Line);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave.render(&mut ctx);

    // Should render sine wave
    let mut has_points = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '●' {
                    has_points = true;
                    break;
                }
            }
        }
    }
    assert!(has_points);
}

#[test]
fn test_square_wave_generation() {
    let data = square_wave(100, 2.0, 1.0);
    assert_eq!(data.len(), 100);
    assert!(data.iter().all(|&v| v.abs() <= 1.0));

    // Square wave should only have values at amplitude or -amplitude
    let all_at_extreme = data.iter().all(|&v| v.abs() == 1.0);
    assert!(
        all_at_extreme,
        "Square wave values should all be at +/- amplitude"
    );
}

#[test]
fn test_square_wave_rendering() {
    let data = square_wave(50, 2.0, 0.8);
    let wave = waveline(data).style(WaveStyle::Line);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave.render(&mut ctx);

    // Should render square wave
    let mut has_points = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol != ' ' {
                    has_points = true;
                    break;
                }
            }
        }
    }
    assert!(has_points);
}

#[test]
fn test_sawtooth_wave_generation() {
    let data = sawtooth_wave(100, 2.0, 1.0);
    assert_eq!(data.len(), 100);
    assert!(data.iter().all(|&v| v >= -1.0 && v <= 1.0));

    // Sawtooth should have smooth transitions
    let has_intermediate = data.iter().any(|&v| v > -0.9 && v < 0.9);
    assert!(has_intermediate);
}

#[test]
fn test_sawtooth_wave_different_params() {
    let data1 = sawtooth_wave(50, 1.0, 0.5);
    assert_eq!(data1.len(), 50);
    assert!(data1.iter().all(|&v| v >= -0.5 && v <= 0.5));

    let data2 = sawtooth_wave(200, 5.0, 0.8);
    assert_eq!(data2.len(), 200);
    assert!(data2.iter().all(|&v| v >= -0.8 && v <= 0.8));
}

#[test]
fn test_sawtooth_wave_rendering() {
    let data = sawtooth_wave(50, 2.0, 0.8);
    let wave = waveline(data).style(WaveStyle::Line);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave.render(&mut ctx);

    // Should render sawtooth wave
    let mut has_points = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol != ' ' {
                    has_points = true;
                    break;
                }
            }
        }
    }
    assert!(has_points);
}

// =============================================================================
// Render Tests - Different Styles
// =============================================================================

#[test]
fn test_waveline_render_line_style() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let data = vec![0.0, 0.5, 1.0, 0.5, 0.0];
    let wave = waveline(data).style(WaveStyle::Line);
    wave.render(&mut ctx);

    // Should render points with circle character
    let mut has_points = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '●' {
                    has_points = true;
                    break;
                }
            }
        }
    }
    assert!(has_points);
}

#[test]
fn test_waveline_render_smooth_style() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let data = sine_wave(50, 2.0, 0.8);
    let wave = waveline(data).style(WaveStyle::Smooth);
    wave.render(&mut ctx);

    // Should render points with circle character
    let mut has_points = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '●' {
                    has_points = true;
                    break;
                }
            }
        }
    }
    assert!(has_points);
}

#[test]
fn test_waveline_render_filled_style() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let data = vec![0.0, 0.3, 0.6, 0.3, 0.0];
    let wave = waveline(data).style(WaveStyle::Filled);
    wave.render(&mut ctx);

    // Should render filled characters
    let mut has_filled = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' || cell.symbol == '▓' {
                    has_filled = true;
                    break;
                }
            }
        }
    }
    assert!(has_filled);
}

#[test]
fn test_waveline_render_mirrored_style() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let data = vec![0.1, 0.3, 0.5, 0.3, 0.1];
    let wave = waveline(data).style(WaveStyle::Mirrored);
    wave.render(&mut ctx);

    // Should render mirrored characters
    let mut has_mirrored = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' || cell.symbol == '▀' || cell.symbol == '▄' {
                    has_mirrored = true;
                    break;
                }
            }
        }
    }
    assert!(has_mirrored);
}

#[test]
fn test_waveline_render_bars_style() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let data = vec![0.1, 0.3, 0.5, 0.7, 0.5, 0.3, 0.1];
    let wave = waveline(data).style(WaveStyle::Bars);
    wave.render(&mut ctx);

    // Should render bar characters
    let bar_chars = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
    let mut has_bars = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if bar_chars.contains(&cell.symbol) {
                    has_bars = true;
                    break;
                }
            }
        }
    }
    assert!(has_bars);
}

#[test]
fn test_waveline_render_dots_style() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let data = vec![0.0, 0.5, 1.0, 0.5, 0.0];
    let wave = waveline(data).style(WaveStyle::Dots);
    wave.render(&mut ctx);

    // Should render dots
    let mut has_dots = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '⣿' {
                    has_dots = true;
                    break;
                }
            }
        }
    }
    assert!(has_dots);
}

// =============================================================================
// Render Tests - Edge Cases
// =============================================================================

#[test]
fn test_waveline_render_empty_data() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let wave = waveline(vec![]);
    wave.render(&mut ctx);

    // Should not crash, buffer should remain mostly empty
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                // Only spaces expected for empty data
                assert_eq!(cell.symbol, ' ');
            }
        }
    }
}

#[test]
fn test_waveline_render_small_area() {
    let mut buffer = Buffer::new(2, 1);
    let area = Rect::new(0, 0, 2, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let data = vec![0.5; 5];
    let wave = waveline(data);
    wave.render(&mut ctx);

    // Should render without error
}

#[test]
fn test_waveline_render_too_small_area() {
    let mut buffer = Buffer::new(1, 1);
    let area = Rect::new(0, 0, 1, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let data = vec![0.5; 5];
    let wave = waveline(data);
    wave.render(&mut ctx);

    // Should handle gracefully without rendering
}

#[test]
fn test_waveline_render_with_negative_values() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let data = vec![-0.5, -0.25, 0.0, 0.25, 0.5];
    let wave = waveline(data);
    wave.render(&mut ctx);

    // Should handle negative values
    let mut has_points = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol != ' ' {
                    has_points = true;
                    break;
                }
            }
        }
    }
    assert!(has_points);
}

#[test]
fn test_waveline_amplitude_clamping() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let data = vec![0.5, 1.5, 2.0]; // Values > 1.0
    let wave = waveline(data).amplitude(1.0);
    wave.render(&mut ctx);

    // Should clamp values and render
    let mut has_points = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol != ' ' {
                    has_points = true;
                    break;
                }
            }
        }
    }
    assert!(has_points);
}

// =============================================================================
// Interpolation Render Tests
// =============================================================================

#[test]
fn test_waveline_interpolation_linear_render() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let data = vec![0.0, 1.0];
    let wave = waveline(data).interpolation(Interpolation::Linear);
    wave.render(&mut ctx);

    // Should render
    let mut has_points = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol != ' ' {
                    has_points = true;
                    break;
                }
            }
        }
    }
    assert!(has_points);
}

#[test]
fn test_waveline_interpolation_step_render() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let data = vec![0.0, 0.5, 1.0];
    let wave = waveline(data).interpolation(Interpolation::Step);
    wave.render(&mut ctx);

    // Should render
    let mut has_points = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol != ' ' {
                    has_points = true;
                    break;
                }
            }
        }
    }
    assert!(has_points);
}

#[test]
fn test_waveline_interpolation_bezier_render() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let data = vec![0.0, 0.5, 1.0, 0.5, 0.0];
    let wave = waveline(data).interpolation(Interpolation::Bezier);
    wave.render(&mut ctx);

    // Should render
    let mut has_points = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol != ' ' {
                    has_points = true;
                    break;
                }
            }
        }
    }
    assert!(has_points);
}

#[test]
fn test_waveline_interpolation_catmull_rom_render() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let data = vec![0.0, 0.5, 1.0, 0.5, 0.0];
    let wave = waveline(data).interpolation(Interpolation::CatmullRom);
    wave.render(&mut ctx);

    // Should render
    let mut has_points = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol != ' ' {
                    has_points = true;
                    break;
                }
            }
        }
    }
    assert!(has_points);
}

// =============================================================================
// Chained Builders Tests
// =============================================================================

#[test]
fn test_waveline_chained_builders() {
    let data = vec![0.0, 0.5, 1.0, 0.5, 0.0];
    let wave = waveline(data)
        .style(WaveStyle::Filled)
        .gradient(Color::CYAN, Color::BLUE)
        .baseline(0.75)
        .amplitude(1.5)
        .show_baseline(true)
        .baseline_color(Color::YELLOW)
        .bg(Color::BLACK)
        .height(8)
        .max_points(50)
        .label("Test Wave")
        .interpolation(Interpolation::CatmullRom);

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave.render(&mut ctx);

    // Verify label was rendered
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'T');

    // Verify background is set (label area has background)
    if let Some(cell) = buffer.get(0, 0) {
        assert_eq!(cell.bg, Some(Color::BLACK));
    }

    // Verify chart area has background
    if let Some(cell) = buffer.get(0, 1) {
        assert_eq!(cell.bg, Some(Color::BLACK));
    }

    // Verify some content was rendered (filled style should produce █ or ▓)
    let mut has_chart_content = false;
    for y in 0..10 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' || cell.symbol == '▓' {
                    has_chart_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_chart_content, "Chart should render filled content");
}

#[test]
fn test_waveline_render_with_all_styles() {
    let styles = [
        WaveStyle::Line,
        WaveStyle::Filled,
        WaveStyle::Mirrored,
        WaveStyle::Bars,
        WaveStyle::Dots,
        WaveStyle::Smooth,
    ];

    let data = sine_wave(50, 2.0, 0.8);

    for style in styles {
        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let wave = waveline(data.clone()).style(style);
        wave.render(&mut ctx);

        // Each style should produce some output
        let mut has_output = false;
        for y in 0..5 {
            for x in 0..20 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol != ' ' {
                        has_output = true;
                        break;
                    }
                }
            }
        }
        assert!(has_output, "Style {:?} should produce output", style);
    }
}

// =============================================================================
// Data Update Tests
// =============================================================================

#[test]
fn test_waveline_update_data_via_render() {
    let data1 = vec![0.0, 0.2, 0.4, 0.6, 0.8];
    let data2 = vec![1.0, 0.8, 0.6, 0.4, 0.2];

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);

    // Render with first dataset
    let wave1 = waveline(data1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave1.render(&mut ctx);

    // Clear buffer
    buffer.clear();

    // Render with second dataset
    let wave2 = waveline(data2);
    let mut ctx2 = RenderContext::new(&mut buffer, area);
    wave2.render(&mut ctx2);

    // Both should render successfully
    let mut has_content = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol != ' ' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_waveline_single_point() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let wave = waveline(vec![0.5]);
    wave.render(&mut ctx);

    // Should render a single point
    let mut has_point = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol != ' ' {
                    has_point = true;
                    break;
                }
            }
        }
    }
    assert!(has_point);
}

#[test]
fn test_waveline_two_points() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let wave = waveline(vec![0.0, 1.0]);
    wave.render(&mut ctx);

    // Should render line between points
    let mut has_content = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol != ' ' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_waveline_constant_values() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let wave = waveline(vec![0.5; 20]);
    wave.render(&mut ctx);

    // Should render flat line
    let mut has_content = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol != ' ' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_waveline_extreme_values() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Mix of extreme values
    let data = vec![-10.0, -1.0, 0.0, 1.0, 10.0];
    let wave = waveline(data).amplitude(0.1);
    wave.render(&mut ctx);

    // Should render with clamped values
    let mut has_content = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol != ' ' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

// =============================================================================
// WaveStyle Enum Tests
// =============================================================================

#[test]
fn test_wave_style_default() {
    let style = WaveStyle::default();
    assert_eq!(style, WaveStyle::Line);
}

#[test]
fn test_wave_style_partial_eq() {
    assert_eq!(WaveStyle::Line, WaveStyle::Line);
    assert_eq!(WaveStyle::Filled, WaveStyle::Filled);
    assert_eq!(WaveStyle::Mirrored, WaveStyle::Mirrored);
    assert_eq!(WaveStyle::Bars, WaveStyle::Bars);
    assert_eq!(WaveStyle::Dots, WaveStyle::Dots);
    assert_eq!(WaveStyle::Smooth, WaveStyle::Smooth);
    assert_ne!(WaveStyle::Line, WaveStyle::Filled);
}

#[test]
fn test_wave_style_all_variants() {
    let _ = WaveStyle::Line;
    let _ = WaveStyle::Filled;
    let _ = WaveStyle::Mirrored;
    let _ = WaveStyle::Bars;
    let _ = WaveStyle::Dots;
    let _ = WaveStyle::Smooth;
}

// =============================================================================
// Interpolation Enum Tests
// =============================================================================

#[test]
fn test_interpolation_default() {
    let interp = Interpolation::default();
    assert_eq!(interp, Interpolation::Linear);
}

#[test]
fn test_interpolation_partial_eq() {
    assert_eq!(Interpolation::Linear, Interpolation::Linear);
    assert_eq!(Interpolation::Bezier, Interpolation::Bezier);
    assert_eq!(Interpolation::CatmullRom, Interpolation::CatmullRom);
    assert_eq!(Interpolation::Step, Interpolation::Step);
    assert_ne!(Interpolation::Linear, Interpolation::Step);
}

#[test]
fn test_interpolation_all_variants() {
    let _ = Interpolation::Linear;
    let _ = Interpolation::Bezier;
    let _ = Interpolation::CatmullRom;
    let _ = Interpolation::Step;
}

// =============================================================================
// CSS Integration Tests
// =============================================================================

#[test]
fn test_waveline_element_id() {
    let wave = waveline(vec![0.5; 10]).element_id("test-wave");
    assert_eq!(View::id(&wave), Some("test-wave"));
}

#[test]
fn test_waveline_classes() {
    let wave = waveline(vec![0.5; 10]).class("chart").class("interactive");
    assert!(wave.has_class("chart"));
    assert!(wave.has_class("interactive"));
    assert!(!wave.has_class("hidden"));
}

#[test]
fn test_waveline_styled_view_methods() {
    let mut wave = waveline(vec![0.5; 10]);

    wave.set_id("my-wave");
    assert_eq!(View::id(&wave), Some("my-wave"));

    wave.add_class("active");
    assert!(wave.has_class("active"));

    wave.remove_class("active");
    assert!(!wave.has_class("active"));

    wave.toggle_class("visible");
    assert!(wave.has_class("visible"));

    wave.toggle_class("visible");
    assert!(!wave.has_class("visible"));
}

#[test]
fn test_waveline_meta() {
    let wave = waveline(vec![0.5; 10])
        .element_id("test")
        .class("class1")
        .class("class2");

    let meta = wave.meta();
    assert_eq!(meta.id, Some("test".to_string()));
    assert_eq!(meta.classes.len(), 2);
}

// =============================================================================
// Additional Edge Cases
// =============================================================================

#[test]
fn test_waveline_zero_area() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 0, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let wave = waveline(vec![0.5; 10]);
    wave.render(&mut ctx);
    // Should handle gracefully
}

#[test]
fn test_waveline_very_long_data() {
    let data: Vec<f64> = (0..1000).map(|i| i as f64 / 1000.0).collect();
    let wave = waveline(data).max_points(100);

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave.render(&mut ctx);
}

#[test]
fn test_waveline_all_zeros() {
    let data = vec![0.0; 20];
    let wave = waveline(data);

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave.render(&mut ctx);

    // Should render flat line at baseline
    let mut has_content = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol != ' ' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_waveline_all_ones() {
    let data = vec![1.0; 20];
    let wave = waveline(data);

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave.render(&mut ctx);

    // Should render at max height
    let mut has_content = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol != ' ' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_waveline_alternating_values() {
    let data = vec![0.0, 1.0, 0.0, 1.0, 0.0, 1.0];
    let wave = waveline(data);

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave.render(&mut ctx);

    // Should render alternating pattern
    let mut has_content = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol != ' ' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_waveline_single_point_negative() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let wave = waveline(vec![-0.5]);
    wave.render(&mut ctx);

    // Should handle single negative point
    let mut has_content = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol != ' ' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_waveline_gradient_same_colors() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let wave = waveline(vec![0.0, 0.5, 1.0]).gradient(Color::RED, Color::RED);
    wave.render(&mut ctx);

    // Should render with single color
    let mut has_color = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.fg == Some(Color::RED) {
                    has_color = true;
                    break;
                }
            }
        }
    }
    assert!(has_color);
}

#[test]
fn test_waveline_zero_height() {
    let wave = waveline(vec![0.5; 10]).height(0);
    // Zero height should be handled
}

#[test]
fn test_waveline_very_large_height() {
    let wave = waveline(vec![0.5; 10]).height(1000);
    // Very large height should be handled
}

#[test]
fn test_waveline_baseline_at_zero() {
    let wave = waveline(vec![0.5; 10]).baseline(0.0);
    // Just verify it compiles
}

#[test]
fn test_waveline_baseline_at_one() {
    let wave = waveline(vec![0.5; 10]).baseline(1.0);
    // Just verify it compiles
}

#[test]
fn test_waveline_zero_amplitude() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let wave = waveline(vec![0.0, 0.5, 1.0]).amplitude(0.0);
    wave.render(&mut ctx);
    // Should render all at baseline
}

#[test]
fn test_waveline_negative_amplitude() {
    let wave = waveline(vec![0.5; 10]).amplitude(-1.0);
    // Negative amplitude should be handled
}

#[test]
fn test_waveline_very_large_amplitude() {
    let wave = waveline(vec![0.5; 10]).amplitude(1000.0);
    // Very large amplitude should be handled
}

#[test]
fn test_waveline_max_points_zero() {
    let wave = waveline(vec![0.5; 10]).max_points(0);
    // Zero max_points should be handled
}

#[test]
fn test_waveline_max_points_larger_than_data() {
    let wave = waveline(vec![0.5; 5]).max_points(100);
    // Just verify it compiles
}

#[test]
fn test_waveline_empty_label() {
    let _wave = waveline(vec![0.5; 10]).label("");
    // Empty label should be handled
}

#[test]
fn test_waveline_very_long_label() {
    let long_label = "This is a very long label that exceeds the normal width";
    let _wave = waveline(vec![0.5; 10]).label(long_label);
    // Long label should be handled
}

#[test]
fn test_waveline_unicode_label() {
    let _wave = waveline(vec![0.5; 10]).label("📊 波形");
    // Unicode label should be handled
}

#[test]
fn test_waveline_multiple_render_calls() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);

    let wave = waveline(vec![0.5; 10]);

    for _ in 0..5 {
        buffer.clear();
        let mut ctx = RenderContext::new(&mut buffer, area);
        wave.render(&mut ctx);
    }
}

#[test]
fn test_waveline_sine_wave_with_frequency_zero() {
    let data = sine_wave(50, 0.0, 1.0);
    // Zero frequency should produce constant value
    assert!(data.iter().all(|&v| v.abs() < 0.01));
}

#[test]
fn test_waveline_sine_wave_with_zero_amplitude() {
    let data = sine_wave(50, 2.0, 0.0);
    // Zero amplitude should produce all zeros
    assert!(data.iter().all(|&v| v.abs() < 0.01));
}

#[test]
fn test_waveline_square_wave_with_zero_amplitude() {
    let data = square_wave(50, 2.0, 0.0);
    // Zero amplitude should produce all zeros
    assert!(data.iter().all(|&v| v.abs() < 0.01));
}

#[test]
fn test_waveline_sawtooth_wave_with_zero_amplitude() {
    let data = sawtooth_wave(50, 2.0, 0.0);
    // Zero amplitude should produce all zeros
    assert!(data.iter().all(|&v| v.abs() < 0.01));
}

#[test]
fn test_waveline_render_with_all_interpolations() {
    let interps = [
        Interpolation::Linear,
        Interpolation::Bezier,
        Interpolation::CatmullRom,
        Interpolation::Step,
    ];

    let data = sine_wave(30, 2.0, 0.8);

    for interp in interps {
        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let wave = waveline(data.clone()).interpolation(interp);
        wave.render(&mut ctx);

        // Each interpolation should produce output
        let mut has_output = false;
        for y in 0..5 {
            for x in 0..20 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol != ' ' {
                        has_output = true;
                        break;
                    }
                }
            }
        }
        assert!(
            has_output,
            "Interpolation {:?} should produce output",
            interp
        );
    }
}

#[test]
fn test_waveline_render_with_negative_baseline() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let wave = waveline(vec![0.5; 10]).baseline(-0.5);
    wave.render(&mut ctx);
    // Should handle negative baseline
}

#[test]
fn test_waveline_render_without_baseline() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let wave = waveline(vec![0.5; 10]).show_baseline(false);
    wave.render(&mut ctx);

    // Should not have baseline character
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '─' {
                    panic!("Should not have baseline when show_baseline is false");
                }
            }
        }
    }
}

#[test]
fn test_waveline_gradient_with_baseline() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let wave = waveline(vec![0.0, 0.5, 1.0])
        .gradient(Color::RED, Color::BLUE)
        .show_baseline(true)
        .baseline(0.5);

    wave.render(&mut ctx);
    // Should render with both gradient and baseline
}

#[test]
fn test_waveline_all_zeros_with_baseline() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let wave = waveline(vec![0.0; 10]).baseline(0.5).show_baseline(true);

    wave.render(&mut ctx);
    // Should show baseline even with flat data
}

#[test]
fn test_waveline_data_mutation() {
    let data = vec![0.0, 0.5, 1.0];
    let wave = waveline(data.clone());

    // Mutate original data - shouldn't affect wave
    let mut data_mut = data;
    data_mut[0] = 1.0;

    // Data should be cloned internally
    // Just verify it compiles and renders
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave.render(&mut ctx);
}

#[test]
fn test_waveline_clone() {
    let wave1 = waveline(vec![0.0, 0.5, 1.0]).style(WaveStyle::Filled);
    let wave2 = wave1.clone();

    // Both should render identically
    let mut buffer1 = Buffer::new(20, 5);
    let mut buffer2 = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx1 = RenderContext::new(&mut buffer1, area);
    let mut ctx2 = RenderContext::new(&mut buffer2, area);

    wave1.render(&mut ctx1);
    wave2.render(&mut ctx2);

    // Compare rendered output
    for y in 0..5 {
        for x in 0..20 {
            let c1 = buffer1.get(x, y).unwrap();
            let c2 = buffer2.get(x, y).unwrap();
            assert_eq!(c1.symbol, c2.symbol);
        }
    }
}

#[test]
fn test_waveline_offset_rendering() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(10, 3, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let wave = waveline(vec![0.0, 0.5, 1.0]);
    wave.render(&mut ctx);

    // Should render at offset position
    // First content should be at y=3 + relative_position
    let mut has_content = false;
    for y in 3..8 {
        for x in 10..30 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol != ' ' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_waveline_very_narrow_area() {
    let mut buffer = Buffer::new(3, 5);
    let area = Rect::new(0, 0, 3, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let wave = waveline(vec![0.5; 5]);
    wave.render(&mut ctx);
    // Should handle very narrow area
}

#[test]
fn test_waveline_inconsistent_data_spacing() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Wide variance in values
    let data = vec![0.0, 0.99, 0.01, 0.98, 0.02, 0.97];
    let wave = waveline(data);
    wave.render(&mut ctx);

    // Should handle inconsistent data
    let mut has_content = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol != ' ' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_waveline_nan_values() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let data = vec![0.0, f64::NAN, 1.0, f64::NAN, 0.5];
    let wave = waveline(data);
    wave.render(&mut ctx);
    // Should handle NaN values gracefully
}

#[test]
fn test_waveline_infinity_values() {
    let data = vec![0.0, f64::INFINITY, 1.0, f64::NEG_INFINITY, 0.5];
    let wave = waveline(data);
    // Should handle infinity values gracefully - just verify it compiles
}

#[test]
fn test_waveline_builder_chain_consumes() {
    let wave1 = waveline(vec![0.5; 5]).color(Color::RED);
    let wave2 = wave1.gradient(Color::BLUE, Color::GREEN);

    // Builder pattern consumes self - just verify it compiles
}
