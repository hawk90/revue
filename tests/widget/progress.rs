//! Progress widget tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::RenderContext;
use revue::widget::{Progress, ProgressStyle, View};

#[test]
fn test_progress_new() {
    let p = Progress::new(0.5);
    assert!((p.value() - 0.5).abs() < f32::EPSILON);
}

#[test]
fn test_progress_clamp() {
    let p1 = Progress::new(-0.5);
    assert!((p1.value() - 0.0).abs() < f32::EPSILON);

    let p2 = Progress::new(1.5);
    assert!((p2.value() - 1.0).abs() < f32::EPSILON);
}

#[test]
fn test_progress_render_half() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(0.5);
    p.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '█');
    assert_eq!(buffer.get(4, 0).unwrap().symbol, '█');
    assert_eq!(buffer.get(5, 0).unwrap().symbol, '░');
    assert_eq!(buffer.get(9, 0).unwrap().symbol, '░');
}

#[test]
fn test_progress_render_full() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(1.0);
    p.render(&mut ctx);

    for x in 0..10 {
        assert_eq!(buffer.get(x, 0).unwrap().symbol, '█');
    }
}

#[test]
fn test_progress_render_empty() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(0.0);
    p.render(&mut ctx);

    for x in 0..10 {
        assert_eq!(buffer.get(x, 0).unwrap().symbol, '░');
    }
}

#[test]
fn test_progress_ascii_style() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(0.5).style(ProgressStyle::Ascii);
    p.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '#');
    assert_eq!(buffer.get(5, 0).unwrap().symbol, '-');
}

#[test]
fn test_progress_with_percentage() {
    let mut buffer = Buffer::new(15, 1);
    let area = Rect::new(0, 0, 15, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(0.5).show_percentage(true);
    p.render(&mut ctx);

    assert_eq!(buffer.get(11, 0).unwrap().symbol, ' ');
    assert_eq!(buffer.get(12, 0).unwrap().symbol, '5');
    assert_eq!(buffer.get(13, 0).unwrap().symbol, '0');
    assert_eq!(buffer.get(14, 0).unwrap().symbol, '%');
}

#[test]
fn test_progress_set() {
    let mut p = Progress::new(0.0);
    p.set_progress(0.75);
    assert!((p.value() - 0.75).abs() < f32::EPSILON);
}

// =============================================================================
