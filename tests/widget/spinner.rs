//! Spinner widget tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::RenderContext;
use revue::widget::{Spinner, SpinnerStyle, View};

#[test]
fn test_spinner_tick() {
    let mut s = Spinner::new();
    assert_eq!(s.frame(), 0);
    s.tick();
    assert_eq!(s.frame(), 1);
    s.tick();
    assert_eq!(s.frame(), 2);
}

#[test]
fn test_spinner_wrap() {
    let mut s = Spinner::new().style(SpinnerStyle::Line);
    s.set_frame(3);
    s.tick();
    assert_eq!(s.frame(), 0);
}

#[test]
fn test_spinner_render() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Spinner::new();
    s.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '⠋');
}

#[test]
fn test_spinner_with_label() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Spinner::new().label("Loading...");
    s.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '⠋');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, 'L');
    assert_eq!(buffer.get(3, 0).unwrap().symbol, 'o');
}

#[test]
fn test_spinner_style_line() {
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Spinner::new().style(SpinnerStyle::Line);
    s.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '|');
}

#[test]
fn test_spinner_style_circle() {
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Spinner::new().style(SpinnerStyle::Circle);
    s.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '◐');
}

#[test]
fn test_spinner_reset() {
    let mut s = Spinner::new();
    s.tick();
    s.tick();
    assert_eq!(s.frame(), 2);
    s.reset();
    assert_eq!(s.frame(), 0);
}

// =============================================================================
