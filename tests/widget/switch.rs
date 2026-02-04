//! Switch widget tests

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{switch, toggle, Switch, SwitchStyle, View};

// ==================== Constructor Tests ====================

#[test]
fn test_switch_new() {
    let s = Switch::new();
    assert!(!s.is_on());
}

#[test]
fn test_switch_default() {
    let s = Switch::default();
    assert!(!s.is_on());
}

#[test]
fn test_switch_helper() {
    let s = switch().on(true);
    assert!(s.is_on());
}

#[test]
fn test_toggle_helper() {
    let s = toggle("Enable");
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);

    let text: String = (0..30)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(text.contains("Enable"));
}

// ==================== State Tests ====================

#[test]
fn test_switch_on() {
    let s = Switch::new().on(true);
    assert!(s.is_on());

    let s = Switch::new().on(false);
    assert!(!s.is_on());
}

#[test]
fn test_switch_checked_alias() {
    // Test checked() is an alias for on()
    let s = Switch::new().checked(true);
    assert!(s.is_on());
    assert!(s.is_checked());

    let s = Switch::new().checked(false);
    assert!(!s.is_on());
    assert!(!s.is_checked());
}

#[test]
fn test_switch_toggle() {
    let mut s = Switch::new();
    assert!(!s.is_on());

    s.toggle();
    assert!(s.is_on());

    s.toggle();
    assert!(!s.is_on());
}

#[test]
fn test_switch_set() {
    let mut s = Switch::new();

    s.set(true);
    assert!(s.is_on());

    s.set(false);
    assert!(!s.is_on());
}

#[test]
fn test_switch_disabled_toggle() {
    let mut s = Switch::new().disabled(true);

    s.toggle();
    assert!(!s.is_on()); // Should not change when disabled

    s.set(true);
    assert!(!s.is_on()); // set() also respects disabled
}

// ==================== Builder Tests ====================

#[test]
fn test_switch_focused() {
    let _s = Switch::new().focused(true);
    // Private field - just verify it compiles
}

#[test]
fn test_switch_disabled() {
    let _s = Switch::new().disabled(true);
    // Private field - just verify it compiles
}

#[test]
fn test_switch_label() {
    let _s = Switch::new().label("Dark Mode");
    // Private field - just verify it compiles
}

#[test]
fn test_switch_label_right() {
    let _s = Switch::new().label_right();
    // Private field - just verify it compiles
}

#[test]
fn test_switch_width() {
    let _s = Switch::new().width(10);
    // Private field - just verify it compiles
}

#[test]
fn test_switch_width_minimum() {
    let _s = Switch::new().width(2);
    // Width should be clamped to min 4
}

#[test]
fn test_switch_on_color() {
    let _s = Switch::new().on_color(Color::CYAN);
    // Private field - just verify it compiles
}

#[test]
fn test_switch_off_color() {
    let _s = Switch::new().off_color(Color::rgb(100, 100, 100));
    // Private field - just verify it compiles
}

#[test]
fn test_switch_track_color() {
    let _s = Switch::new().track_color(Color::BLUE);
    // Private field - just verify it compiles
}

#[test]
fn test_switch_text() {
    let _s = Switch::new().text("YES", "NO");
    // Private fields - just verify it compiles
}

#[test]
fn test_switch_builder_chain() {
    let _s = Switch::new()
        .on(true)
        .focused(true)
        .disabled(false)
        .label("Toggle")
        .label_right()
        .width(8)
        .style(SwitchStyle::IOS)
        .on_color(Color::GREEN)
        .off_color(Color::rgb(100, 100, 100))
        .track_color(Color::BLUE)
        .text("ON", "OFF");
    // Just verify it compiles
}

// ==================== Style Tests ====================

#[test]
fn test_switch_style_default() {
    let style = SwitchStyle::default();
    assert_eq!(style, SwitchStyle::Default);
}

#[test]
fn test_switch_style_all_variants() {
    let _ = SwitchStyle::Default;
    let _ = SwitchStyle::IOS;
    let _ = SwitchStyle::Material;
    let _ = SwitchStyle::Text;
    let _ = SwitchStyle::Emoji;
    let _ = SwitchStyle::Block;
}

#[test]
fn test_switch_style_partial_eq() {
    assert_eq!(SwitchStyle::Default, SwitchStyle::Default);
    assert_eq!(SwitchStyle::IOS, SwitchStyle::IOS);
    assert_ne!(SwitchStyle::Default, SwitchStyle::IOS);
}

// ==================== Key Handling Tests ====================

#[test]
fn test_switch_handle_key_enter() {
    let mut s = Switch::new().focused(true);
    assert!(!s.is_on());

    assert!(s.handle_key(&Key::Enter));
    assert!(s.is_on());
}

#[test]
fn test_switch_handle_key_space() {
    let mut s = Switch::new().focused(true).on(true);

    assert!(s.handle_key(&Key::Char(' ')));
    assert!(!s.is_on());
}

#[test]
fn test_switch_handle_key_unfocused() {
    let mut s = Switch::new().focused(false);

    assert!(!s.handle_key(&Key::Enter));
    assert!(!s.handle_key(&Key::Char(' ')));
    assert!(!s.is_on());
}

#[test]
fn test_switch_handle_key_disabled() {
    let mut s = Switch::new().focused(true).disabled(true);

    assert!(!s.handle_key(&Key::Enter));
    assert!(!s.handle_key(&Key::Char(' ')));
    assert!(!s.is_on());
}

#[test]
fn test_switch_handle_key_invalid() {
    let mut s = Switch::new().focused(true);

    assert!(!s.handle_key(&Key::Tab));
    assert!(!s.handle_key(&Key::Up));
    assert!(!s.handle_key(&Key::Char('x')));
    assert!(!s.handle_key(&Key::Char('z')));
    assert!(!s.is_on());
}

// ==================== Rendering Tests ====================

#[test]
fn test_switch_render_default() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new().on(true);
    s.render(&mut ctx);
}

#[test]
fn test_switch_render_off() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new().on(false);
    s.render(&mut ctx);
}

#[test]
fn test_switch_render_all_styles() {
    let styles = [
        SwitchStyle::Default,
        SwitchStyle::IOS,
        SwitchStyle::Material,
        SwitchStyle::Text,
        SwitchStyle::Emoji,
        SwitchStyle::Block,
    ];

    for style in styles {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let s = Switch::new().style(style);
        s.render(&mut ctx);
    }
}

#[test]
fn test_switch_render_with_label_left() {
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new().label("Dark Mode");
    s.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'D');
}

#[test]
fn test_switch_render_with_label_right() {
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new().label_right().label("Dark Mode");
    s.render(&mut ctx);

    // Label should be on the right
    let text: String = (0..30)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(text.contains("Dark Mode"));
}

#[test]
fn test_switch_render_focused() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new().focused(true);
    s.render(&mut ctx);
    // Should render with focus indicator
}

#[test]
fn test_switch_render_disabled() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new().disabled(true).label("Disabled");
    s.render(&mut ctx);
    // Should render with disabled styling
}

#[test]
fn test_switch_render_zero_area() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 0, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new();
    s.render(&mut ctx);
    // Should handle zero area gracefully
}

#[test]
fn test_switch_render_custom_width() {
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new().width(12);
    s.render(&mut ctx);
    // Should render with custom width
}

#[test]
fn test_switch_render_text_style_custom() {
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new().style(SwitchStyle::Text).text("YES", "NO");

    s.render(&mut ctx);
    // Should render with custom text
}

#[test]
fn test_switch_render_text_style_default_text() {
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new().style(SwitchStyle::Text);

    s.render(&mut ctx);
    // Should render with default ON/OFF text
}

// ==================== Edge Cases ====================

#[test]
fn test_switch_empty_label() {
    let s = Switch::new().label("");
    // Should handle empty label
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
}

#[test]
fn test_switch_unicode_label() {
    let s = Switch::new().label("ダークモード");
    // Should handle unicode label
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
}

#[test]
fn test_switch_very_long_label() {
    let long_label = "This is a very long label that exceeds normal width";
    let s = Switch::new().label(long_label);
    // Should handle long label
    let mut buffer = Buffer::new(40, 1);
    let area = Rect::new(0, 0, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
}

#[test]
fn test_switch_multiple_toggles() {
    let mut s = Switch::new();

    for _ in 0..10 {
        s.toggle();
    }

    // After 10 toggles (even number), should be off
    assert!(!s.is_on());

    s.toggle();
    assert!(s.is_on());
}

#[test]
fn test_switch_set_while_disabled() {
    let mut s = Switch::new().disabled(true);

    s.set(true);
    assert!(!s.is_on());

    s.set(false);
    assert!(!s.is_on());
}

#[test]
fn test_switch_toggle_on_off() {
    let mut s = Switch::new().on(true);

    s.toggle();
    assert!(!s.is_on());

    s.toggle();
    assert!(s.is_on());
}

#[test]
fn test_switch_state_query_methods() {
    let s_on = Switch::new().on(true);
    assert!(s_on.is_on());
    assert!(s_on.is_checked());

    let s_off = Switch::new().on(false);
    assert!(!s_off.is_on());
    assert!(!s_off.is_checked());
}
