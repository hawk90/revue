//! Waveline widget tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
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
