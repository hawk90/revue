//! Button widget tests

use revue::event::{MouseButton, MouseEvent, MouseEventKind};
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{Button, StyledView, View};

#[test]
fn test_button_render() {
    let btn = Button::new("OK").width(6);
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(1, 1, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    btn.render(&mut ctx);
}

#[test]
fn test_button_focused_render() {
    let btn = Button::new("Submit").focused(true);
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(2, 1, 15, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    btn.render(&mut ctx);
}

#[test]
fn test_button_disabled() {
    let btn = Button::new("Disabled").disabled(true);
    assert!(btn.is_disabled());
    assert!(!btn.is_focused());
}

#[test]
fn test_button_handle_mouse_click() {
    let mut btn = Button::new("Test");
    let area = Rect::new(10, 5, 10, 1);

    let down = MouseEvent::new(15, 5, MouseEventKind::Down(MouseButton::Left));
    let (needs_render, clicked) = btn.handle_mouse(&down, area);
    assert!(needs_render);
    assert!(!clicked);
    assert!(btn.is_pressed());

    let up = MouseEvent::new(15, 5, MouseEventKind::Up(MouseButton::Left));
    let (needs_render, clicked) = btn.handle_mouse(&up, area);
    assert!(needs_render);
    assert!(clicked);
    assert!(!btn.is_pressed());
}

#[test]
fn test_button_handle_mouse_click_outside() {
    let mut btn = Button::new("Test");
    let area = Rect::new(10, 5, 10, 1);

    let down = MouseEvent::new(15, 5, MouseEventKind::Down(MouseButton::Left));
    btn.handle_mouse(&down, area);
    assert!(btn.is_pressed());

    let up = MouseEvent::new(0, 0, MouseEventKind::Up(MouseButton::Left));
    let (needs_render, clicked) = btn.handle_mouse(&up, area);
    assert!(needs_render);
    assert!(!clicked);
}

#[test]
fn test_button_handle_mouse_hover() {
    let mut btn = Button::new("Test");
    let area = Rect::new(10, 5, 10, 1);

    let enter = MouseEvent::new(15, 5, MouseEventKind::Move);
    let (needs_render, _) = btn.handle_mouse(&enter, area);
    assert!(needs_render);
    assert!(btn.is_hovered());

    let inside = MouseEvent::new(16, 5, MouseEventKind::Move);
    let (needs_render, _) = btn.handle_mouse(&inside, area);
    assert!(!needs_render);
    assert!(btn.is_hovered());

    let leave = MouseEvent::new(0, 0, MouseEventKind::Move);
    let (needs_render, _) = btn.handle_mouse(&leave, area);
    assert!(needs_render);
    assert!(!btn.is_hovered());
}

#[test]
fn test_button_handle_mouse_disabled() {
    let mut btn = Button::new("Test").disabled(true);
    let area = Rect::new(10, 5, 10, 1);

    let down = MouseEvent::new(15, 5, MouseEventKind::Down(MouseButton::Left));
    let (needs_render, clicked) = btn.handle_mouse(&down, area);
    assert!(!needs_render);
    assert!(!clicked);
    assert!(!btn.is_pressed());
}

#[test]
fn test_button_css_id() {
    let btn = Button::new("Submit").element_id("submit-btn");
    assert_eq!(View::id(&btn), Some("submit-btn"));

    let meta = btn.meta();
    assert_eq!(meta.id, Some("submit-btn".to_string()));
}

#[test]
fn test_button_css_classes() {
    let btn = Button::new("Action").class("primary").class("large");

    assert!(btn.has_class("primary"));
    assert!(btn.has_class("large"));
    assert!(!btn.has_class("small"));

    let meta = btn.meta();
    assert!(meta.classes.contains("primary"));
    assert!(meta.classes.contains("large"));
}

#[test]
fn test_button_styled_view() {
    let mut btn = Button::new("Test");

    btn.set_id("test-id");
    assert_eq!(View::id(&btn), Some("test-id"));

    btn.add_class("active");
    assert!(btn.has_class("active"));

    btn.remove_class("active");
    assert!(!btn.has_class("active"));

    btn.toggle_class("selected");
    assert!(btn.has_class("selected"));

    btn.toggle_class("selected");
    assert!(!btn.has_class("selected"));
}

#[test]
fn test_button_css_colors_from_context() {
    use revue::style::{Style, VisualStyle};

    let btn = Button::new("CSS");
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(1, 1, 15, 1);

    let mut style = Style::default();
    style.visual = VisualStyle {
        color: Color::RED,
        background: Color::BLUE,
        ..VisualStyle::default()
    };

    let mut ctx = RenderContext::with_style(&mut buffer, area, &style);
    btn.render(&mut ctx);
}

#[test]
fn test_button_inline_override_css() {
    use revue::style::{Style, VisualStyle};

    let btn = Button::new("Override").fg(Color::GREEN).bg(Color::YELLOW);

    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(1, 1, 15, 1);

    let mut style = Style::default();
    style.visual = VisualStyle {
        color: Color::RED,
        background: Color::BLUE,
        ..VisualStyle::default()
    };

    let mut ctx = RenderContext::with_style(&mut buffer, area, &style);
    btn.render(&mut ctx);
}
