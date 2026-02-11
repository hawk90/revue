//! Tests for RenderContext progress bar drawing methods
//!
//! Extracted from src/widget/traits/render_context/progress.rs

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::render_context::{ProgressBarConfig, RenderContext};
use revue::widget::View;

// Test widget to create a render context
#[allow(dead_code)]
struct TestWidget;
impl View for TestWidget {
    fn render(&self, _ctx: &mut RenderContext) {}
}

#[test]
fn test_draw_progress_bar() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let config = ProgressBarConfig {
        x: 10,
        y: 5,
        width: 20,
        progress: 0.5,
        filled_char: '█',
        empty_char: '░',
        fg: Color::rgb(255, 255, 255),
    };

    // Should not panic
    ctx.draw_progress_bar(&config);
}

#[test]
fn test_draw_progress_bar_zero() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let config = ProgressBarConfig {
        x: 10,
        y: 5,
        width: 20,
        progress: 0.0,
        filled_char: '█',
        empty_char: '░',
        fg: Color::rgb(255, 255, 255),
    };

    ctx.draw_progress_bar(&config);
}

#[test]
fn test_draw_progress_bar_full() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let config = ProgressBarConfig {
        x: 10,
        y: 5,
        width: 20,
        progress: 1.0,
        filled_char: '█',
        empty_char: '░',
        fg: Color::rgb(255, 255, 255),
    };

    ctx.draw_progress_bar(&config);
}

#[test]
fn test_draw_progress_bar_clamped() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Progress values outside 0.0-1.0 should be clamped
    let config = ProgressBarConfig {
        x: 10,
        y: 5,
        width: 20,
        progress: 1.5, // Over 1.0
        filled_char: '█',
        empty_char: '░',
        fg: Color::rgb(255, 255, 255),
    };

    ctx.draw_progress_bar(&config);
}

#[test]
fn test_draw_progress_bar_negative() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let config = ProgressBarConfig {
        x: 10,
        y: 5,
        width: 20,
        progress: -0.5, // Negative
        filled_char: '█',
        empty_char: '░',
        fg: Color::rgb(255, 255, 255),
    };

    ctx.draw_progress_bar(&config);
}

#[test]
fn test_draw_progress_bar_labeled() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Should not panic
    ctx.draw_progress_bar_labeled(10, 5, 20, 0.5, Color::rgb(255, 255, 255));
}

#[test]
fn test_draw_progress_bar_labeled_zero() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    ctx.draw_progress_bar_labeled(10, 5, 20, 0.0, Color::rgb(255, 255, 255));
}

#[test]
fn test_draw_progress_bar_labeled_full() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    ctx.draw_progress_bar_labeled(10, 5, 20, 1.0, Color::rgb(255, 255, 255));
}

#[test]
fn test_progress_bar_config_public_fields() {
    let config = ProgressBarConfig {
        x: 5,
        y: 10,
        width: 30,
        progress: 0.75,
        filled_char: '=',
        empty_char: '-',
        fg: Color::rgb(100, 100, 100),
    };

    assert_eq!(config.x, 5);
    assert_eq!(config.y, 10);
    assert_eq!(config.width, 30);
    assert_eq!(config.progress, 0.75);
    assert_eq!(config.filled_char, '=');
    assert_eq!(config.empty_char, '-');
}
