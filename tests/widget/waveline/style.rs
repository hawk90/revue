//! Waveline widget tests - style & interpolation tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::RenderContext;
use revue::widget::{Interpolation, WaveStyle, waveline};

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
