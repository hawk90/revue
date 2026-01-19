//! Alert widget tests

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{Alert, AlertLevel, AlertVariant, View};

#[test]
fn test_alert_dismiss() {
    let mut a = Alert::new("Test").dismissible(true);
    assert!(!a.is_dismissed());

    a.dismiss();
    assert!(a.is_dismissed());

    a.reset();
    assert!(!a.is_dismissed());
}

#[test]
fn test_alert_handle_key() {
    let mut a = Alert::new("Test").dismissible(true);

    assert!(a.handle_key(&Key::Char('x')));
    assert!(a.is_dismissed());

    a.reset();
    assert!(a.handle_key(&Key::Escape));
    assert!(a.is_dismissed());
}

#[test]
fn test_alert_handle_key_not_dismissible() {
    let mut a = Alert::new("Test").dismissible(false);
    assert!(!a.handle_key(&Key::Char('x')));
    assert!(!a.is_dismissed());
}

#[test]
fn test_alert_height() {
    let minimal = Alert::new("msg").variant(AlertVariant::Minimal);
    assert_eq!(minimal.height(), 1);

    let minimal_title = Alert::new("msg")
        .title("Title")
        .variant(AlertVariant::Minimal);
    assert_eq!(minimal_title.height(), 2);

    let filled = Alert::new("msg").variant(AlertVariant::Filled);
    assert_eq!(filled.height(), 3);

    let filled_title = Alert::new("msg")
        .title("Title")
        .variant(AlertVariant::Filled);
    assert_eq!(filled_title.height(), 4);

    let mut dismissed = Alert::new("msg").dismissible(true);
    dismissed.dismiss();
    assert_eq!(dismissed.height(), 0);
}

#[test]
fn test_alert_level_colors() {
    assert_eq!(AlertLevel::Info.color(), Color::CYAN);
    assert_eq!(AlertLevel::Success.color(), Color::GREEN);
    assert_eq!(AlertLevel::Warning.color(), Color::YELLOW);
    assert_eq!(AlertLevel::Error.color(), Color::RED);
}

#[test]
fn test_alert_level_icons() {
    assert_eq!(AlertLevel::Info.icon(), 'ℹ');
    assert_eq!(AlertLevel::Success.icon(), '✓');
    assert_eq!(AlertLevel::Warning.icon(), '⚠');
    assert_eq!(AlertLevel::Error.icon(), '✗');
}

#[test]
fn test_alert_render_filled() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = Alert::new("Test message").variant(AlertVariant::Filled);
    a.render(&mut ctx);

    // Check border corners
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '╭');
    assert_eq!(buffer.get(39, 0).unwrap().symbol, '╮');
}

#[test]
fn test_alert_render_outlined() {
    let mut buffer = Buffer::new(40, 3);
    let area = Rect::new(0, 0, 40, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = Alert::new("Test").variant(AlertVariant::Outlined);
    a.render(&mut ctx);

    // Check left accent border
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┃');
    assert_eq!(buffer.get(0, 1).unwrap().symbol, '┃');
}

#[test]
fn test_alert_render_minimal() {
    let mut buffer = Buffer::new(40, 2);
    let area = Rect::new(0, 0, 40, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = Alert::new("Test").variant(AlertVariant::Minimal);
    a.render(&mut ctx);

    // Check icon
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'ℹ');
}

#[test]
fn test_alert_render_dismissed() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut a = Alert::new("Test");
    a.dismiss();
    a.render(&mut ctx);

    // Should not render anything (buffer should be default)
    assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
}

// =============================================================================
