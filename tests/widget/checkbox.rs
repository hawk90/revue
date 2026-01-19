//! Checkbox widget tests

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{Checkbox, StyledView, View};

#[test]
fn test_checkbox_toggle() {
    let mut cb = Checkbox::new("Toggle");
    assert!(!cb.is_checked());

    cb.toggle();
    assert!(cb.is_checked());

    cb.toggle();
    assert!(!cb.is_checked());
}

#[test]
fn test_checkbox_disabled_toggle() {
    let mut cb = Checkbox::new("Disabled").disabled(true);
    assert!(!cb.is_checked());

    cb.toggle();
    assert!(!cb.is_checked()); // Should not change
}

#[test]
fn test_checkbox_handle_key() {
    let mut cb = Checkbox::new("Test");

    assert!(cb.handle_key(&Key::Enter));
    assert!(cb.is_checked());

    assert!(cb.handle_key(&Key::Char(' ')));
    assert!(!cb.is_checked());

    assert!(!cb.handle_key(&Key::Char('a')));
}

#[test]
fn test_checkbox_disabled_handle_key() {
    let mut cb = Checkbox::new("Test").disabled(true);

    assert!(!cb.handle_key(&Key::Enter));
    assert!(!cb.is_checked());
}

#[test]
fn test_checkbox_render() {
    let cb = Checkbox::new("Option").checked(true);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(1, 1, 25, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cb.render(&mut ctx);
}

#[test]
fn test_checkbox_css_id() {
    let cb = Checkbox::new("Accept").element_id("accept-checkbox");
    assert_eq!(View::id(&cb), Some("accept-checkbox"));

    let meta = cb.meta();
    assert_eq!(meta.id, Some("accept-checkbox".to_string()));
}

#[test]
fn test_checkbox_css_classes() {
    let cb = Checkbox::new("Option")
        .class("required")
        .class("form-control");

    assert!(cb.has_class("required"));
    assert!(cb.has_class("form-control"));
    assert!(!cb.has_class("optional"));

    let meta = cb.meta();
    assert!(meta.classes.contains("required"));
    assert!(meta.classes.contains("form-control"));
}

#[test]
fn test_checkbox_styled_view() {
    let mut cb = Checkbox::new("Test");

    cb.set_id("test-cb");
    assert_eq!(View::id(&cb), Some("test-cb"));

    cb.add_class("active");
    assert!(cb.has_class("active"));

    cb.toggle_class("active");
    assert!(!cb.has_class("active"));

    cb.toggle_class("selected");
    assert!(cb.has_class("selected"));

    cb.remove_class("selected");
    assert!(!cb.has_class("selected"));
}

#[test]
fn test_checkbox_css_colors_from_context() {
    use revue::style::{Style, VisualStyle};

    let cb = Checkbox::new("CSS Test");
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(1, 1, 25, 1);

    let mut style = Style::default();
    style.visual = VisualStyle {
        color: Color::YELLOW,
        ..VisualStyle::default()
    };

    let mut ctx = RenderContext::with_style(&mut buffer, area, &style);
    cb.render(&mut ctx);
}

// =============================================================================
