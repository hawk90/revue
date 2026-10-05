//! Checkbox construction and rendering tests

use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{checkbox, Checkbox, CheckboxStyle, StyledView, View};

fn render(c: &Checkbox, width: u16) -> Buffer {
    let mut buffer = Buffer::new(width, 1);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, 1));
    c.render(&mut ctx);
    buffer
}

fn line(c: &Checkbox, width: u16) -> String {
    let buffer = render(c, width);
    (0..width)
        .map(|x| buffer.get(x, 0).unwrap().symbol)
        .collect::<String>()
        .trim_end()
        .to_string()
}

#[test]
fn test_checkbox_new() {
    let c = checkbox("Accept");
    assert!(!c.is_checked());
    assert!(!c.is_disabled());
    assert_eq!(line(&c, 20), "[ ] Accept");
}

#[test]
fn test_checkbox_default() {
    let c = Checkbox::default();
    assert!(!c.is_checked());
    assert_eq!(line(&c, 10), "[ ]");
}

#[test]
fn test_checkbox_checked() {
    let c = checkbox("Accept").checked(true);
    assert!(c.is_checked());
    assert_eq!(line(&c, 20), "[x] Accept");
}

#[test]
fn test_checkbox_styles() {
    let cases = [
        (CheckboxStyle::Square, "[x] A", "[ ] A"),
        (CheckboxStyle::Unicode, "☑ A", "☐ A"),
        (CheckboxStyle::Filled, "■ A", "□ A"),
        (CheckboxStyle::Circle, "● A", "○ A"),
    ];
    for (style, on, off) in cases {
        let c = checkbox("A").style(style);
        assert_eq!(line(&c, 10), off, "{style:?}");
        assert_eq!(line(&c.checked(true), 10), on, "{style:?}");
    }
}

#[test]
fn test_checkbox_default_style_is_square() {
    assert_eq!(CheckboxStyle::default(), CheckboxStyle::Square);
}

#[test]
fn test_checkbox_check_colors() {
    let buffer = render(&checkbox("A").checked(true), 10);
    assert_eq!(buffer.get(1, 0).unwrap().fg, Some(Color::GREEN));
    let buffer = render(&checkbox("A").checked(true).check_fg(Color::MAGENTA), 10);
    assert_eq!(buffer.get(1, 0).unwrap().fg, Some(Color::MAGENTA));
    // check_fg colors the mark only when checked
    let buffer = render(&checkbox("A").check_fg(Color::MAGENTA), 10);
    assert_ne!(buffer.get(1, 0).unwrap().fg, Some(Color::MAGENTA));
}

#[test]
fn test_checkbox_focused_render() {
    let c = checkbox("Go").focused(true);
    assert_eq!(line(&c, 10), "> [ ] Go");
    let buffer = render(&c, 10);
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::CYAN));
    assert!(buffer.get(6, 0).unwrap().modifier.contains(Modifier::BOLD));
}

#[test]
fn test_checkbox_disabled_render() {
    // Disabled: no focus marker, grayed mark
    let c = checkbox("Go").focused(true).disabled(true).checked(true);
    assert_eq!(line(&c, 10), "[x] Go");
    let buffer = render(&c, 10);
    assert_ne!(buffer.get(1, 0).unwrap().fg, Some(Color::GREEN));
    assert!(!buffer.get(4, 0).unwrap().modifier.contains(Modifier::BOLD));
}

#[test]
fn test_checkbox_label_clipped() {
    assert_eq!(line(&checkbox("Long label"), 8), "[ ] Long");
}

#[test]
fn test_checkbox_render_zero_area() {
    let mut buffer = Buffer::new(5, 1);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 0, 1));
    checkbox("A").checked(true).render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
}

#[test]
fn test_checkbox_css_meta() {
    let c = checkbox("A").element_id("terms").class("required");
    assert_eq!(View::id(&c), Some("terms"));
    assert!(c.has_class("required"));
    assert_eq!(c.meta().widget_type, "Checkbox");
}
