//! Waveline widget tests - basic tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::RenderContext;
use revue::widget::{waveline, Interpolation, View, Waveline};

#[test]
fn test_waveline_new() {
    let data = vec![0.0, 0.5, 1.0, 0.5, 0.0];
    let wave = Waveline::new(data);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave.render(&mut ctx);
}

#[test]
fn test_waveline_default() {
    let wave = Waveline::default();
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
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    wave.render(&mut ctx);
}
