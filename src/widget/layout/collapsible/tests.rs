use super::*;
use crate::layout::Rect;
use crate::render::Buffer;
use crate::widget::traits::{RenderContext, View};

#[test]
fn test_collapsible_new() {
    let c = Collapsible::new("Info");
    assert_eq!(c.title, "Info");
    assert!(!c.is_expanded());
    assert_eq!(c.height(), 1);
}

#[test]
fn test_collapsible_expand_collapse() {
    let mut c = Collapsible::new("Info").content("Line 1\nLine 2");
    assert!(!c.is_expanded());
    assert_eq!(c.height(), 1);

    c.expand();
    assert!(c.is_expanded());
    assert!(c.height() > 1);

    c.collapse();
    assert!(!c.is_expanded());
    assert_eq!(c.height(), 1);
}

#[test]
fn test_collapsible_toggle() {
    let mut c = Collapsible::new("Info");
    assert!(!c.is_expanded());
    c.toggle();
    assert!(c.is_expanded());
    c.toggle();
    assert!(!c.is_expanded());
}

#[test]
fn test_collapsible_content() {
    let c = Collapsible::new("Info")
        .content("Line 1\nLine 2\nLine 3")
        .expanded(true);
    assert!(c.is_expanded());
    assert_eq!(c.content.len(), 3);
    // header(1) + 3 content lines + border(1) = 5
    assert_eq!(c.height(), 5);
}

#[test]
fn test_collapsible_content_no_border() {
    let c = Collapsible::new("Info")
        .content("Line 1\nLine 2")
        .border(false)
        .expanded(true);
    // header(1) + 2 content lines = 3 (no border)
    assert_eq!(c.height(), 3);
}

#[test]
fn test_collapsible_handle_key() {
    let mut c = Collapsible::new("Info").content("Text");

    assert!(c.handle_key(&Key::Enter));
    assert!(c.is_expanded());

    assert!(c.handle_key(&Key::Char(' ')));
    assert!(!c.is_expanded());

    assert!(c.handle_key(&Key::Right));
    assert!(c.is_expanded());

    assert!(c.handle_key(&Key::Left));
    assert!(!c.is_expanded());

    assert!(!c.handle_key(&Key::Char('x')));
}

#[test]
fn test_collapsible_handle_key_disabled() {
    let mut c = Collapsible::new("Info").content("Text");
    c.state.disabled = true;
    assert!(!c.handle_key(&Key::Enter));
    assert!(!c.is_expanded());
}

#[test]
fn test_collapsible_icons() {
    let c = Collapsible::new("Info").icons('+', '-');
    assert_eq!(c.icon(), '+');

    let c = Collapsible::new("Info").icons('+', '-').expanded(true);
    assert_eq!(c.icon(), '-');
}

#[test]
fn test_collapsible_render_collapsed() {
    let mut buf = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buf, area);

    let c = Collapsible::new("Details");
    c.render(&mut ctx);
    // Collapsed icon should be rendered
    assert_eq!(buf.get(0, 0).unwrap().symbol, '▶');
}

#[test]
fn test_collapsible_render_expanded() {
    let mut buf = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buf, area);

    let c = Collapsible::new("Details").content("Hello").expanded(true);
    c.render(&mut ctx);
    assert_eq!(buf.get(0, 0).unwrap().symbol, '▼');
}

#[test]
fn test_collapsible_render_small_area_no_panic() {
    let mut buf = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 3, 1);
    let mut ctx = RenderContext::new(&mut buf, area);
    let c = Collapsible::new("Long Title").expanded(true);
    c.render(&mut ctx); // Width < 4, should return early
}

#[test]
fn test_collapsible_default() {
    let c = Collapsible::default();
    assert_eq!(c.title, "Details");
}

#[test]
fn test_collapsible_helper_fn() {
    let c = collapsible("Test");
    assert_eq!(c.title, "Test");
}

#[test]
fn test_collapsible_line_and_lines() {
    let c = Collapsible::new("Info")
        .line("First")
        .lines(&["Second", "Third"]);
    assert_eq!(c.content.len(), 3);
}
