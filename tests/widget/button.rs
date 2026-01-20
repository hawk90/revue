//! Button widget tests

use revue::event::{Key, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{EventResult, Interactive, RenderContext, StyledView, View};
use revue::widget::{button, Button, ButtonVariant};

// =============================================================================
// Constructor Tests
// =============================================================================

#[test]
fn test_button_new() {
    let btn = Button::new("Click Me");
    assert!(!btn.is_focused());
    assert!(!btn.is_disabled());
    assert!(!btn.is_pressed());
    assert!(!btn.is_hovered());
}

#[test]
fn test_button_default() {
    let btn = Button::default();
    assert!(!btn.is_focused());
    assert!(!btn.is_disabled());
}

#[test]
fn test_button_helper() {
    let btn = button("Helper");
    assert!(!btn.is_focused());
    assert!(!btn.is_disabled());
}

// =============================================================================
// ButtonVariant Tests
// =============================================================================

#[test]
fn test_button_default_variant() {
    let btn = Button::new("Default");
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    btn.render(&mut ctx);
}

#[test]
fn test_button_primary_constructor() {
    let btn = Button::primary("Submit");
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    btn.render(&mut ctx);
}

#[test]
fn test_button_danger_constructor() {
    let btn = Button::danger("Delete");
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    btn.render(&mut ctx);
}

#[test]
fn test_button_ghost_constructor() {
    let btn = Button::ghost("Cancel");
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    btn.render(&mut ctx);
}

#[test]
fn test_button_success_constructor() {
    let btn = Button::success("Confirm");
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    btn.render(&mut ctx);
}

#[test]
fn test_button_variant_builder() {
    let btn = Button::new("Custom").variant(ButtonVariant::Primary);
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    btn.render(&mut ctx);
}

#[test]
fn test_button_all_variants() {
    let variants = [
        ButtonVariant::Default,
        ButtonVariant::Primary,
        ButtonVariant::Danger,
        ButtonVariant::Ghost,
        ButtonVariant::Success,
    ];

    for variant in variants {
        let btn = Button::new("Test").variant(variant);
        let mut buffer = Buffer::new(20, 3);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        btn.render(&mut ctx);
    }
}

// =============================================================================
// Builder Methods Tests
// =============================================================================

#[test]
fn test_button_icon() {
    let btn = Button::new("Save").icon('💾');
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    btn.render(&mut ctx);

    let mut found_icon = false;
    for x in area.x..area.x + area.width {
        if let Some(cell) = buffer.get(x, area.y) {
            if cell.symbol == '💾' {
                found_icon = true;
                break;
            }
        }
    }
    assert!(found_icon);
}

#[test]
fn test_button_width() {
    let btn = Button::new("OK").width(20);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    btn.render(&mut ctx);

    // Button with width 20 renders successfully
    // Verify the button text appears somewhere
    let mut found_ok = false;
    for x in 0..area.width {
        if let Some(cell) = buffer.get(x, area.y) {
            if cell.symbol == 'O' {
                found_ok = true;
                break;
            }
        }
    }
    assert!(found_ok, "Button text 'OK' should appear in buffer");
}

#[test]
fn test_button_focused() {
    let btn = Button::new("Test").focused(true);
    assert!(btn.is_focused());
}

#[test]
fn test_button_disabled() {
    let btn = Button::new("Test").disabled(true);
    assert!(btn.is_disabled());
}

#[test]
fn test_button_custom_colors() {
    let btn = Button::new("Custom").fg(Color::RED).bg(Color::BLUE);
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    btn.render(&mut ctx);

    let cell = buffer.get(area.x, area.y).unwrap();
    assert_eq!(cell.bg, Some(Color::BLUE));
}

#[test]
fn test_button_builder_chain() {
    let btn = Button::new("Submit")
        .variant(ButtonVariant::Primary)
        .icon('✓')
        .width(25)
        .focused(true)
        .disabled(false)
        .fg(Color::YELLOW)
        .bg(Color::BLACK);

    assert!(btn.is_focused());
    assert!(!btn.is_disabled());

    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    btn.render(&mut ctx);

    let mut found_icon = false;
    for x in area.x..area.x + area.width {
        if let Some(cell) = buffer.get(x, area.y) {
            if cell.symbol == '✓' {
                found_icon = true;
                break;
            }
        }
    }
    assert!(found_icon);
}

// =============================================================================
// State Query Methods Tests
// =============================================================================

#[test]
fn test_button_is_pressed() {
    let mut btn = Button::new("Test");
    assert!(!btn.is_pressed());

    let area = Rect::new(0, 0, 10, 1);
    let down = MouseEvent::new(5, 0, MouseEventKind::Down(MouseButton::Left));
    btn.handle_mouse(&down, area);
    assert!(btn.is_pressed());
}

#[test]
fn test_button_is_hovered() {
    let mut btn = Button::new("Test");
    assert!(!btn.is_hovered());

    let area = Rect::new(0, 0, 10, 1);
    let move_event = MouseEvent::new(5, 0, MouseEventKind::Move);
    btn.handle_mouse(&move_event, area);
    assert!(btn.is_hovered());
}

// =============================================================================
// Keyboard Handling Tests (handle_key method - returns bool)
// =============================================================================

#[test]
fn test_button_handle_key_enter() {
    let mut btn = Button::new("Test");
    assert!(btn.handle_key(&Key::Enter));
}

#[test]
fn test_button_handle_key_space() {
    let mut btn = Button::new("Test");
    assert!(btn.handle_key(&Key::Char(' ')));
}

#[test]
fn test_button_handle_key_other() {
    let mut btn = Button::new("Test");
    assert!(!btn.handle_key(&Key::Char('a')));
    assert!(!btn.handle_key(&Key::Char('x')));
    assert!(!btn.handle_key(&Key::Tab));
}

#[test]
fn test_button_handle_key_disabled() {
    let mut btn = Button::new("Test").disabled(true);
    assert!(!btn.handle_key(&Key::Enter));
    assert!(!btn.handle_key(&Key::Char(' ')));
}

// =============================================================================
// Interactive Trait - handle_key Tests (returns EventResult)
// =============================================================================

#[test]
fn test_button_interactive_handle_key_enter() {
    let mut btn = Button::new("Test");
    let event = KeyEvent::new(Key::Enter);

    let result = Interactive::handle_key(&mut btn, &event);
    assert_eq!(result, EventResult::ConsumedAndRender);
}

#[test]
fn test_button_interactive_handle_key_space() {
    let mut btn = Button::new("Test");
    let event = KeyEvent::new(Key::Char(' '));

    let result = Interactive::handle_key(&mut btn, &event);
    assert_eq!(result, EventResult::ConsumedAndRender);
}

#[test]
fn test_button_interactive_handle_key_ignored() {
    let mut btn = Button::new("Test");
    let event = KeyEvent::new(Key::Char('a'));

    let result = Interactive::handle_key(&mut btn, &event);
    assert_eq!(result, EventResult::Ignored);
}

#[test]
fn test_button_interactive_handle_key_disabled() {
    let mut btn = Button::new("Test").disabled(true);
    let event = KeyEvent::new(Key::Enter);

    let result = Interactive::handle_key(&mut btn, &event);
    assert_eq!(result, EventResult::Ignored);
}

// =============================================================================
// Mouse Handling Tests (handle_mouse method - returns (bool, bool))
// =============================================================================

#[test]
fn test_button_handle_mouse_click_inside() {
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
fn test_button_handle_mouse_click_outside_after_press() {
    let mut btn = Button::new("Test");
    let area = Rect::new(10, 5, 10, 1);

    let down = MouseEvent::new(15, 5, MouseEventKind::Down(MouseButton::Left));
    btn.handle_mouse(&down, area);
    assert!(btn.is_pressed());

    let up = MouseEvent::new(0, 0, MouseEventKind::Up(MouseButton::Left));
    let (needs_render, clicked) = btn.handle_mouse(&up, area);
    assert!(needs_render);
    assert!(!clicked);
    assert!(!btn.is_pressed());
}

#[test]
fn test_button_handle_mouse_hover_enter() {
    let mut btn = Button::new("Test");
    let area = Rect::new(10, 5, 10, 1);

    let enter = MouseEvent::new(15, 5, MouseEventKind::Move);
    let (needs_render, _) = btn.handle_mouse(&enter, area);
    assert!(needs_render);
    assert!(btn.is_hovered());
}

#[test]
fn test_button_handle_mouse_hover_move_inside() {
    let mut btn = Button::new("Test");
    let area = Rect::new(10, 5, 10, 1);

    let enter = MouseEvent::new(15, 5, MouseEventKind::Move);
    btn.handle_mouse(&enter, area);

    let inside = MouseEvent::new(16, 5, MouseEventKind::Move);
    let (needs_render, _) = btn.handle_mouse(&inside, area);
    assert!(!needs_render);
    assert!(btn.is_hovered());
}

#[test]
fn test_button_handle_mouse_hover_leave() {
    let mut btn = Button::new("Test");
    let area = Rect::new(10, 5, 10, 1);

    let enter = MouseEvent::new(15, 5, MouseEventKind::Move);
    btn.handle_mouse(&enter, area);

    let leave = MouseEvent::new(0, 0, MouseEventKind::Move);
    let (needs_render, _) = btn.handle_mouse(&leave, area);
    assert!(needs_render);
    assert!(!btn.is_hovered());
}

#[test]
fn test_button_handle_mouse_down_disabled() {
    let mut btn = Button::new("Test").disabled(true);
    let area = Rect::new(10, 5, 10, 1);

    let down = MouseEvent::new(15, 5, MouseEventKind::Down(MouseButton::Left));
    let (needs_render, clicked) = btn.handle_mouse(&down, area);
    assert!(!needs_render);
    assert!(!clicked);
    assert!(!btn.is_pressed());
}

#[test]
fn test_button_handle_mouse_up_disabled() {
    let mut btn = Button::new("Test").disabled(true);
    let area = Rect::new(10, 5, 10, 1);

    let up = MouseEvent::new(15, 5, MouseEventKind::Up(MouseButton::Left));
    let (needs_render, clicked) = btn.handle_mouse(&up, area);
    assert!(!needs_render);
    assert!(!clicked);
}

#[test]
fn test_button_handle_mouse_move_disabled() {
    let mut btn = Button::new("Test").disabled(true);
    let area = Rect::new(10, 5, 10, 1);

    let move_event = MouseEvent::new(15, 5, MouseEventKind::Move);
    let (needs_render, _) = btn.handle_mouse(&move_event, area);
    assert!(!needs_render);
    assert!(!btn.is_hovered());
}

#[test]
fn test_button_handle_mouse_already_pressed() {
    let mut btn = Button::new("Test");
    let area = Rect::new(10, 5, 10, 1);

    let down1 = MouseEvent::new(15, 5, MouseEventKind::Down(MouseButton::Left));
    let (needs_render, _) = btn.handle_mouse(&down1, area);
    assert!(needs_render);

    let down2 = MouseEvent::new(16, 5, MouseEventKind::Down(MouseButton::Left));
    let (needs_render, _) = btn.handle_mouse(&down2, area);
    assert!(!needs_render);
}

#[test]
fn test_button_handle_mouse_right_button() {
    let mut btn = Button::new("Test");
    let area = Rect::new(10, 5, 10, 1);

    let right_click = MouseEvent::new(15, 5, MouseEventKind::Down(MouseButton::Right));
    let (needs_render, clicked) = btn.handle_mouse(&right_click, area);
    assert!(!needs_render);
    assert!(!clicked);
}

// =============================================================================
// Interactive Trait - handle_mouse Tests (returns EventResult)
// =============================================================================

#[test]
fn test_button_interactive_handle_mouse_down() {
    let mut btn = Button::new("Test");
    let area = Rect::new(10, 5, 10, 1);
    let event = MouseEvent::new(15, 5, MouseEventKind::Down(MouseButton::Left));

    let result = Interactive::handle_mouse(&mut btn, &event, area);
    assert_eq!(result, EventResult::ConsumedAndRender);
    assert!(btn.is_pressed());
}

#[test]
fn test_button_interactive_handle_mouse_up_inside() {
    let mut btn = Button::new("Test");
    let area = Rect::new(10, 5, 10, 1);

    let down = MouseEvent::new(15, 5, MouseEventKind::Down(MouseButton::Left));
    Interactive::handle_mouse(&mut btn, &down, area);

    let up = MouseEvent::new(15, 5, MouseEventKind::Up(MouseButton::Left));
    let result = Interactive::handle_mouse(&mut btn, &up, area);
    assert_eq!(result, EventResult::ConsumedAndRender);
}

#[test]
fn test_button_interactive_handle_mouse_up_outside() {
    let mut btn = Button::new("Test");
    let area = Rect::new(10, 5, 10, 1);

    let down = MouseEvent::new(15, 5, MouseEventKind::Down(MouseButton::Left));
    Interactive::handle_mouse(&mut btn, &down, area);

    let up = MouseEvent::new(0, 0, MouseEventKind::Up(MouseButton::Left));
    let result = Interactive::handle_mouse(&mut btn, &up, area);
    assert_eq!(result, EventResult::Consumed);
}

#[test]
fn test_button_interactive_handle_mouse_hover() {
    let mut btn = Button::new("Test");
    let area = Rect::new(10, 5, 10, 1);
    let event = MouseEvent::new(15, 5, MouseEventKind::Move);

    let result = Interactive::handle_mouse(&mut btn, &event, area);
    assert_eq!(result, EventResult::ConsumedAndRender);
    assert!(btn.is_hovered());
}

#[test]
fn test_button_interactive_handle_mouse_no_hover_change() {
    let mut btn = Button::new("Test");
    let area = Rect::new(10, 5, 10, 1);

    let enter = MouseEvent::new(15, 5, MouseEventKind::Move);
    Interactive::handle_mouse(&mut btn, &enter, area);

    let inside = MouseEvent::new(16, 5, MouseEventKind::Move);
    let result = Interactive::handle_mouse(&mut btn, &inside, area);
    assert_eq!(result, EventResult::Ignored);
}

#[test]
fn test_button_interactive_handle_mouse_disabled() {
    let mut btn = Button::new("Test").disabled(true);
    let area = Rect::new(10, 5, 10, 1);
    let event = MouseEvent::new(15, 5, MouseEventKind::Down(MouseButton::Left));

    let result = Interactive::handle_mouse(&mut btn, &event, area);
    assert_eq!(result, EventResult::Ignored);
}

// =============================================================================
// Focusable Tests
// =============================================================================

#[test]
fn test_button_focusable_when_enabled() {
    let btn = Button::new("Test");
    assert!(Interactive::focusable(&btn));
}

#[test]
fn test_button_not_focusable_when_disabled() {
    let btn = Button::new("Test").disabled(true);
    assert!(!Interactive::focusable(&btn));
}

// =============================================================================
// Focus/Blur Tests
// =============================================================================

#[test]
fn test_button_on_focus() {
    let mut btn = Button::new("Test");
    assert!(!btn.is_focused());

    Interactive::on_focus(&mut btn);
    assert!(btn.is_focused());
}

#[test]
fn test_button_on_blur() {
    let mut btn = Button::new("Test").focused(true);
    let area = Rect::new(0, 0, 10, 1);

    let down = MouseEvent::new(5, 0, MouseEventKind::Down(MouseButton::Left));
    btn.handle_mouse(&down, area);
    let enter = MouseEvent::new(5, 0, MouseEventKind::Move);
    btn.handle_mouse(&enter, area);

    assert!(btn.is_pressed());
    assert!(btn.is_hovered());

    Interactive::on_blur(&mut btn);
    assert!(!btn.is_focused());
    assert!(!btn.is_pressed());
    assert!(!btn.is_hovered());
}

#[test]
fn test_button_on_blur_preserves_disabled() {
    let mut btn = Button::new("Test").focused(true).disabled(true);

    Interactive::on_blur(&mut btn);
    assert!(!btn.is_focused());
    assert!(btn.is_disabled());
}

// =============================================================================
// Render Tests
// =============================================================================

#[test]
fn test_button_render_basic() {
    let btn = Button::new("OK").width(6);
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(1, 1, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    btn.render(&mut ctx);

    let cell = buffer.get(area.x, area.y).unwrap();
    assert_eq!(cell.symbol, ' ');
}

#[test]
fn test_button_render_with_icon() {
    let btn = Button::new("Save").icon('💾');
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(1, 1, 15, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    btn.render(&mut ctx);

    let mut found_icon = false;
    for x in area.x..area.x + area.width {
        if let Some(cell) = buffer.get(x, area.y) {
            if cell.symbol == '💾' {
                found_icon = true;
                break;
            }
        }
    }
    assert!(found_icon);
}

#[test]
fn test_button_render_focused() {
    let btn = Button::new("Submit").focused(true);
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(2, 1, 15, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    btn.render(&mut ctx);

    if area.x > 0 {
        let left_bracket = buffer.get(area.x - 1, area.y);
        assert!(left_bracket.is_some());
    }
}

#[test]
fn test_button_render_disabled() {
    let btn = Button::new("Disabled").disabled(true);
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(1, 1, 15, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    btn.render(&mut ctx);

    let cell = buffer.get(area.x, area.y).unwrap();
    assert_eq!(cell.symbol, ' ');
}

#[test]
fn test_button_render_zero_area() {
    let btn = Button::new("Test");
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 0, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    btn.render(&mut ctx);
}

#[test]
fn test_button_render_all_variants() {
    let variants = [
        ButtonVariant::Default,
        ButtonVariant::Primary,
        ButtonVariant::Danger,
        ButtonVariant::Ghost,
        ButtonVariant::Success,
    ];

    for variant in variants {
        let btn = Button::new("Test").variant(variant);
        let mut buffer = Buffer::new(20, 3);
        let area = Rect::new(1, 1, 15, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        btn.render(&mut ctx);

        let cell = buffer.get(area.x, area.y).unwrap();
        assert_eq!(cell.symbol, ' ');
    }
}

#[test]
fn test_button_render_focused_no_brackets_when_disabled() {
    let btn = Button::new("Test").focused(true).disabled(true);
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(2, 1, 15, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    btn.render(&mut ctx);

    let left_bracket = buffer.get(area.x - 1, area.y);
    if let Some(cell) = left_bracket {
        assert_ne!(cell.symbol, '[');
    }
}

// =============================================================================
// CSS/Styling Tests
// =============================================================================

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
fn test_button_css_classes_from_view_trait() {
    let btn = Button::new("Action").class("btn").class("primary");

    let classes = View::classes(&btn);
    assert_eq!(classes.len(), 2);
    assert!(classes.contains(&"btn".to_string()));
    assert!(classes.contains(&"primary".to_string()));
}

#[test]
fn test_button_styled_view_set_id() {
    let mut btn = Button::new("Test");
    btn.set_id("test-id");
    assert_eq!(View::id(&btn), Some("test-id"));
}

#[test]
fn test_button_styled_view_add_class() {
    let mut btn = Button::new("Test");
    btn.add_class("active");
    assert!(btn.has_class("active"));
}

#[test]
fn test_button_styled_view_remove_class() {
    let mut btn = Button::new("Test").class("active");
    btn.remove_class("active");
    assert!(!btn.has_class("active"));
}

#[test]
fn test_button_styled_view_toggle_class() {
    let mut btn = Button::new("Test");

    btn.toggle_class("selected");
    assert!(btn.has_class("selected"));

    btn.toggle_class("selected");
    assert!(!btn.has_class("selected"));
}

#[test]
fn test_button_styled_view_has_class() {
    let btn = Button::new("Test").class("active");
    assert!(btn.has_class("active"));
    assert!(!btn.has_class("inactive"));
}

#[test]
fn test_button_classes_builder() {
    let btn = Button::new("Test").classes(vec!["class1", "class2", "class3"]);

    assert!(btn.has_class("class1"));
    assert!(btn.has_class("class2"));
    assert!(btn.has_class("class3"));
    assert_eq!(View::classes(&btn).len(), 3);
}

#[test]
fn test_button_duplicate_class_not_added() {
    let btn = Button::new("Test").class("test").class("test");

    let classes = View::classes(&btn);
    assert_eq!(classes.len(), 1);
    assert!(classes.contains(&"test".to_string()));
}

// =============================================================================
// CSS Color Context Tests
// =============================================================================

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
// Variant Color Tests (behavioral testing through rendering)
// =============================================================================

#[test]
fn test_button_default_variant_renders() {
    let btn = Button::new("Test");
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    btn.render(&mut ctx);

    let cell = buffer.get(area.x, area.y).unwrap();
    assert_eq!(cell.symbol, ' ');
    assert!(cell.bg.is_some());
}

#[test]
fn test_button_primary_variant_renders() {
    let btn = Button::primary("Test");
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    btn.render(&mut ctx);

    let cell = buffer.get(area.x, area.y).unwrap();
    assert_eq!(cell.symbol, ' ');
    assert!(cell.bg.is_some());
}

#[test]
fn test_button_danger_variant_renders() {
    let btn = Button::danger("Test");
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    btn.render(&mut ctx);

    let cell = buffer.get(area.x, area.y).unwrap();
    assert_eq!(cell.symbol, ' ');
    assert!(cell.bg.is_some());
}

#[test]
fn test_button_ghost_variant_renders() {
    let btn = Button::ghost("Test");
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    btn.render(&mut ctx);

    let cell = buffer.get(area.x, area.y).unwrap();
    assert_eq!(cell.symbol, ' ');
    assert!(cell.bg.is_some());
}

#[test]
fn test_button_success_variant_renders() {
    let btn = Button::success("Test");
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    btn.render(&mut ctx);

    let cell = buffer.get(area.x, area.y).unwrap();
    assert_eq!(cell.symbol, ' ');
    assert!(cell.bg.is_some());
}

// =============================================================================
// Width and Layout Tests
// =============================================================================

#[test]
fn test_button_render_centering() {
    let btn = Button::new("OK");
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    btn.render(&mut ctx);

    let mut found_text = false;
    for x in 0..area.width {
        if let Some(cell) = buffer.get(x, area.y) {
            if cell.symbol == 'O' {
                found_text = true;
                break;
            }
        }
    }
    assert!(found_text);
}

#[test]
fn test_button_render_with_icon_and_label() {
    let btn = Button::new("Save").icon('💾').width(15);
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    btn.render(&mut ctx);

    let mut found_icon = false;
    let mut found_label = false;

    for x in 0..area.width {
        if let Some(cell) = buffer.get(x, area.y) {
            if cell.symbol == '💾' {
                found_icon = true;
            }
            if cell.symbol == 'S' {
                found_label = true;
            }
        }
    }

    assert!(found_icon);
    assert!(found_label);
}

// =============================================================================
// Complex Interaction Tests
// =============================================================================

#[test]
fn test_button_full_click_cycle() {
    let mut btn = Button::new("Click Me");
    let area = Rect::new(5, 5, 15, 1);

    assert!(!btn.is_pressed());
    assert!(!btn.is_hovered());

    let hover = MouseEvent::new(10, 5, MouseEventKind::Move);
    let (needs_render, _) = btn.handle_mouse(&hover, area);
    assert!(needs_render);
    assert!(btn.is_hovered());

    let down = MouseEvent::new(10, 5, MouseEventKind::Down(MouseButton::Left));
    let (needs_render, clicked) = btn.handle_mouse(&down, area);
    assert!(needs_render);
    assert!(!clicked);
    assert!(btn.is_pressed());
    assert!(btn.is_hovered());

    let up = MouseEvent::new(10, 5, MouseEventKind::Up(MouseButton::Left));
    let (needs_render, clicked) = btn.handle_mouse(&up, area);
    assert!(needs_render);
    assert!(clicked);
    assert!(!btn.is_pressed());
    assert!(btn.is_hovered());

    let away = MouseEvent::new(0, 0, MouseEventKind::Move);
    let (needs_render, _) = btn.handle_mouse(&away, area);
    assert!(needs_render);
    assert!(!btn.is_hovered());
}

#[test]
fn test_button_drag_out_and_release() {
    let mut btn = Button::new("Test");
    let area = Rect::new(10, 5, 10, 1);

    let down = MouseEvent::new(15, 5, MouseEventKind::Down(MouseButton::Left));
    btn.handle_mouse(&down, area);
    assert!(btn.is_pressed());

    let away = MouseEvent::new(5, 5, MouseEventKind::Move);
    btn.handle_mouse(&away, area);
    assert!(!btn.is_hovered());

    let up = MouseEvent::new(5, 5, MouseEventKind::Up(MouseButton::Left));
    let (needs_render, clicked) = btn.handle_mouse(&up, area);
    assert!(needs_render);
    assert!(!clicked);
    assert!(!btn.is_pressed());
}

#[test]
fn test_button_multiple_key_presses() {
    let mut btn = Button::new("Test");

    for _ in 0..5 {
        assert!(btn.handle_key(&Key::Enter));
    }

    for _ in 0..5 {
        assert!(btn.handle_key(&Key::Char(' ')));
    }
}

// =============================================================================
// Edge Cases
// =============================================================================

#[test]
fn test_button_empty_label() {
    let btn = Button::new("");
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    btn.render(&mut ctx);
}

#[test]
fn test_button_very_long_label() {
    let long_label = "This is a very long button label that exceeds the area";
    let btn = Button::new(long_label);

    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    btn.render(&mut ctx);
}

#[test]
fn test_button_width_larger_than_area() {
    let btn = Button::new("OK").width(100);

    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    btn.render(&mut ctx);
}

#[test]
fn test_button_icon_only() {
    let btn = Button::new("").icon('✓');

    let mut buffer = Buffer::new(10, 3);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    btn.render(&mut ctx);

    let mut found_icon = false;
    for x in 0..area.width {
        if let Some(cell) = buffer.get(x, area.y) {
            if cell.symbol == '✓' {
                found_icon = true;
                break;
            }
        }
    }
    assert!(found_icon);
}

#[test]
fn test_button_click_at_edge() {
    let mut btn = Button::new("Test");
    let area = Rect::new(10, 5, 10, 1);

    let left_click = MouseEvent::new(10, 5, MouseEventKind::Down(MouseButton::Left));
    btn.handle_mouse(&left_click, area);
    assert!(btn.is_pressed());

    let left_release = MouseEvent::new(10, 5, MouseEventKind::Up(MouseButton::Left));
    let (_, clicked) = btn.handle_mouse(&left_release, area);
    assert!(clicked);

    let right_click = MouseEvent::new(19, 5, MouseEventKind::Down(MouseButton::Left));
    btn.handle_mouse(&right_click, area);
    assert!(btn.is_pressed());

    let right_release = MouseEvent::new(19, 5, MouseEventKind::Up(MouseButton::Left));
    let (_, clicked) = btn.handle_mouse(&right_release, area);
    assert!(clicked);
}

// =============================================================================
// Meta and Debug Tests
// =============================================================================

#[test]
fn test_button_meta() {
    let btn = Button::new("Test")
        .element_id("test-btn")
        .class("primary")
        .class("large");

    let meta = btn.meta();
    assert_eq!(meta.widget_type, "Button");
    assert_eq!(meta.id, Some("test-btn".to_string()));
    assert!(meta.classes.contains("primary"));
    assert!(meta.classes.contains("large"));
}

#[test]
fn test_button_clone() {
    let btn1 = Button::new("Test")
        .variant(ButtonVariant::Primary)
        .icon('✓')
        .width(20)
        .focused(true)
        .disabled(false);

    let btn2 = btn1.clone();

    assert_eq!(btn1.is_focused(), btn2.is_focused());
    assert_eq!(btn1.is_disabled(), btn2.is_disabled());
}

#[test]
fn test_button_debug_format() {
    let btn = Button::new("Test").variant(ButtonVariant::Primary);
    let debug_str = format!("{:?}", btn);

    assert!(debug_str.contains("Button"));
}
