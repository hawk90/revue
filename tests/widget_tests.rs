//! Widget integration tests
//!
//! Tests for widgets that use only public API.
//! Tests accessing private fields remain inline in src/widget/*.rs.

use revue::event::{MouseButton, MouseEvent, MouseEventKind};
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{
    badge, button, dot_badge, Alignment, Badge, BadgeVariant, Button, ButtonVariant, StyledView,
    Text, View,
};

// =============================================================================
// Button Tests
// =============================================================================

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

// =============================================================================
// Text Tests
// =============================================================================

#[test]
fn test_text_new() {
    let text = Text::new("Hello");
    assert_eq!(text.content(), "Hello");
}

#[test]
fn test_text_render() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let text = Text::new("Hello");
    text.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'H');
    assert_eq!(buffer.get(1, 0).unwrap().symbol, 'e');
    assert_eq!(buffer.get(4, 0).unwrap().symbol, 'o');
}

#[test]
fn test_text_render_centered() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let text = Text::new("Hi").align(Alignment::Center);
    text.render(&mut ctx);

    assert_eq!(buffer.get(9, 0).unwrap().symbol, 'H');
    assert_eq!(buffer.get(10, 0).unwrap().symbol, 'i');
}

#[test]
fn test_text_css_id() {
    let text = Text::new("Title").element_id("page-title");
    assert_eq!(View::id(&text), Some("page-title"));

    let meta = text.meta();
    assert_eq!(meta.id, Some("page-title".to_string()));
}

#[test]
fn test_text_css_classes() {
    let text = Text::new("Warning").class("alert").class("bold");

    assert!(text.has_class("alert"));
    assert!(text.has_class("bold"));
    assert!(!text.has_class("hidden"));

    let meta = text.meta();
    assert!(meta.classes.contains("alert"));
    assert!(meta.classes.contains("bold"));
}

#[test]
fn test_text_styled_view() {
    let mut text = Text::new("Test");

    text.set_id("test-text");
    assert_eq!(View::id(&text), Some("test-text"));

    text.add_class("highlight");
    assert!(text.has_class("highlight"));

    text.toggle_class("highlight");
    assert!(!text.has_class("highlight"));

    text.toggle_class("active");
    assert!(text.has_class("active"));

    text.remove_class("active");
    assert!(!text.has_class("active"));
}

#[test]
fn test_text_css_colors_from_context() {
    use revue::style::{Style, VisualStyle};

    let text = Text::new("CSS Text");
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 1);

    let mut style = Style::default();
    style.visual = VisualStyle {
        color: Color::MAGENTA,
        ..VisualStyle::default()
    };

    let mut ctx = RenderContext::with_style(&mut buffer, area, &style);
    text.render(&mut ctx);
}

#[test]
fn test_text_justify_alignment() {
    let text = Text::new("Hello World").align(Alignment::Justify);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    text.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'H');
    assert_eq!(buffer.get(4, 0).unwrap().symbol, 'o');
    assert_eq!(buffer.get(19, 0).unwrap().symbol, 'd');
    assert_eq!(buffer.get(15, 0).unwrap().symbol, 'W');
}

#[test]
fn test_text_justify_single_word() {
    let text = Text::new("Hello").align(Alignment::Justify);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    text.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'H');
    assert_eq!(buffer.get(4, 0).unwrap().symbol, 'o');
}

#[test]
fn test_text_justify_multiple_words() {
    let text = Text::new("A B C").align(Alignment::Justify);
    let mut buffer = Buffer::new(11, 1);
    let area = Rect::new(0, 0, 11, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    text.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'A');
    assert_eq!(buffer.get(5, 0).unwrap().symbol, 'B');
    assert_eq!(buffer.get(10, 0).unwrap().symbol, 'C');
}

// =============================================================================
// Badge Tests
// =============================================================================

#[test]
fn test_badge_render() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = badge("NEW").primary();
    b.render(&mut ctx);

    let text: String = (0..20)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(text.contains("NEW"));
}

#[test]
fn test_badge_dot_render() {
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = dot_badge().success();
    b.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('●'));
}

#[test]
fn test_variant_colors() {
    let (bg, fg) = BadgeVariant::Success.colors();
    assert_eq!(fg, Color::WHITE);
    assert_ne!(bg, Color::WHITE);
}

// =============================================================================
// Checkbox Tests
// =============================================================================

use revue::event::Key;
use revue::widget::{checkbox, Checkbox};

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
// Progress Tests
// =============================================================================

use revue::widget::{progress, Progress, ProgressStyle};

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
// Spinner Tests
// =============================================================================

use revue::widget::{spinner, Spinner, SpinnerStyle};

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
// Divider Tests
// =============================================================================

use revue::widget::{divider, vdivider, Divider};

#[test]
fn test_divider_render_horizontal() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let d = divider();
    d.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('─'));
}

#[test]
fn test_divider_render_vertical() {
    let mut buffer = Buffer::new(1, 10);
    let area = Rect::new(0, 0, 1, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let d = vdivider();
    d.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('│'));
}

#[test]
fn test_divider_with_label() {
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let d = divider().label("Section");
    d.render(&mut ctx);

    let text: String = (0..30)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(text.contains("Section"));
}

#[test]
fn test_divider_render_uses_helpers() {
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let d = divider().label("Test");
    d.render(&mut ctx);

    let text: String = (0..30)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(text.contains("Test"));
    assert!(text.contains("─"));
}

#[test]
fn test_divider_label_clipping() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let d = divider().label("VeryLongLabelThatWontFit");
    d.render(&mut ctx);
}

#[test]
fn test_divider_vertical_uses_vline() {
    let mut buffer = Buffer::new(1, 5);
    let area = Rect::new(0, 0, 1, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let d = vdivider();
    d.render(&mut ctx);

    for y in 0..5 {
        assert_eq!(buffer.get(0, y).map(|c| c.symbol), Some('│'));
    }
}
