//! Waveline widget tests - advanced features

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{waveline, Waveline};

#[test]
fn test_waveline_color() {
    let wave = waveline(vec![0.0, 0.5, 1.0]).color(Color::Red);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave.render(&mut ctx);
}

#[test]
fn test_waveline_gradient() {
    let wave = waveline(vec![0.0, 0.5, 1.0])
        .gradient(Color::Blue, Color::Green);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave.render(&mut ctx);
}

#[test]
fn test_waveline_baseline() {
    let wave = waveline(vec![0.0, 0.5, 1.0]).baseline(0.5);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave.render(&mut ctx);
}

#[test]
fn test_waveline_amplitude() {
    let wave = waveline(vec![0.0, 0.5, 1.0]).amplitude(2.0);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave.render(&mut ctx);
}

#[test]
fn test_waveline_show_baseline() {
    let wave = waveline(vec![0.0, 0.5, 1.0]).show_baseline(true);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave.render(&mut ctx);
}
