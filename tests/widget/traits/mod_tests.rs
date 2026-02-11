//! Tests for widget traits module
//!
//! Extracted from src/widget/traits/mod.rs

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::Text;
use revue::widget::traits::{EventResult, View, WidgetState, DISABLED_FG, RenderContext};

#[test]
fn test_event_result_default() {
    let result = EventResult::default();
    assert!(!result.is_consumed());
    assert!(!result.needs_render());
}

#[test]
fn test_event_result_consumed() {
    let consumed = EventResult::Consumed;
    assert!(consumed.is_consumed());
    assert!(!consumed.needs_render());
}

#[test]
fn test_event_result_consumed_and_render() {
    let result = EventResult::ConsumedAndRender;
    assert!(result.is_consumed());
    assert!(result.needs_render());
}

#[test]
fn test_event_result_from_bool() {
    let handled: EventResult = true.into();
    assert_eq!(handled, EventResult::ConsumedAndRender);

    let ignored: EventResult = false.into();
    assert_eq!(ignored, EventResult::Ignored);
}

#[test]
fn test_event_result_or() {
    assert_eq!(
        EventResult::Ignored.or(EventResult::ConsumedAndRender),
        EventResult::ConsumedAndRender
    );
    assert_eq!(
        EventResult::ConsumedAndRender.or(EventResult::Ignored),
        EventResult::ConsumedAndRender
    );
    assert_eq!(
        EventResult::Ignored.or(EventResult::Consumed),
        EventResult::Consumed
    );
    assert_eq!(
        EventResult::Ignored.or(EventResult::Ignored),
        EventResult::Ignored
    );
}

#[test]
fn test_widget_state_new() {
    let state = WidgetState::new();
    assert!(!state.is_focused());
    assert!(!state.is_disabled());
    assert!(!state.is_pressed());
    assert!(!state.is_hovered());
    assert!(!state.is_interactive());
}

#[test]
fn test_widget_state_builder() {
    let state = WidgetState::new()
        .focused(true)
        .disabled(false)
        .fg(Color::RED)
        .bg(Color::BLUE);

    assert!(state.is_focused());
    assert!(!state.is_disabled());
    assert_eq!(state.fg, Some(Color::RED));
    assert_eq!(state.bg, Some(Color::BLUE));
}

#[test]
fn test_widget_state_effective_colors() {
    let default_color = Color::rgb(128, 128, 128);

    let normal = WidgetState::new().fg(Color::WHITE);
    assert_eq!(normal.effective_fg(default_color), Color::WHITE);

    let disabled = WidgetState::new().fg(Color::WHITE).disabled(true);
    assert_eq!(disabled.effective_fg(default_color), DISABLED_FG);
}

#[test]
fn test_widget_state_reset_transient() {
    let mut state = WidgetState::new()
        .focused(true)
        .disabled(false)
        .pressed(true)
        .hovered(true);

    state.reset_transient();

    assert!(state.focused);
    assert!(!state.disabled);
    assert!(!state.pressed);
    assert!(!state.hovered);
}

#[test]
fn test_widget_classes_exposure() {
    let widget = Text::new("Test").class("btn").class("primary");

    let classes = View::classes(&widget);
    assert_eq!(classes.len(), 2);
    assert!(classes.contains(&"btn".to_string()));
    assert!(classes.contains(&"primary".to_string()));

    let meta = widget.meta();
    assert!(meta.classes.contains("btn"));
    assert!(meta.classes.contains("primary"));
}

// Wide character tests
#[test]
fn test_draw_text_wide_chars() {
    let mut buf = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buf, area);

    ctx.draw_text(0, 0, "한글", Color::WHITE);

    assert_eq!(buf.get(0, 0).unwrap().symbol, '한');
    assert!(buf.get(1, 0).unwrap().is_continuation());
    assert_eq!(buf.get(2, 0).unwrap().symbol, '글');
    assert!(buf.get(3, 0).unwrap().is_continuation());
    assert_eq!(buf.get(4, 0).unwrap().symbol, ' ');
}

#[test]
fn test_draw_text_mixed_width() {
    let mut buf = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buf, area);

    ctx.draw_text(0, 0, "A한B", Color::WHITE);

    assert_eq!(buf.get(0, 0).unwrap().symbol, 'A');
    assert_eq!(buf.get(1, 0).unwrap().symbol, '한');
    assert!(buf.get(2, 0).unwrap().is_continuation());
    assert_eq!(buf.get(3, 0).unwrap().symbol, 'B');
}

#[test]
fn test_draw_text_centered_wide_chars() {
    let mut buf = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buf, area);

    ctx.draw_text_centered(0, 0, 10, "한글", Color::WHITE);

    assert_eq!(buf.get(3, 0).unwrap().symbol, '한');
    assert!(buf.get(4, 0).unwrap().is_continuation());
    assert_eq!(buf.get(5, 0).unwrap().symbol, '글');
    assert!(buf.get(6, 0).unwrap().is_continuation());
}

#[test]
fn test_draw_text_right_wide_chars() {
    let mut buf = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buf, area);

    ctx.draw_text_right(0, 0, 10, "한글", Color::WHITE);

    assert_eq!(buf.get(6, 0).unwrap().symbol, '한');
    assert!(buf.get(7, 0).unwrap().is_continuation());
    assert_eq!(buf.get(8, 0).unwrap().symbol, '글');
    assert!(buf.get(9, 0).unwrap().is_continuation());
}
