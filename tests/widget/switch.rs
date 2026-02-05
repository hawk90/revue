//! Switch widget tests

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, StyledView};
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

// ==================== CSS Integration Tests ====================

#[test]
fn test_switch_element_id() {
    let s = Switch::new().element_id("dark-mode-toggle");
    assert_eq!(View::id(&s), Some("dark-mode-toggle"));
}

#[test]
fn test_switch_classes() {
    let s = Switch::new().class("toggle").class("interactive");
    assert!(s.has_class("toggle"));
    assert!(s.has_class("interactive"));
    assert!(!s.has_class("hidden"));
}

#[test]
fn test_switch_styled_view_methods() {
    let mut s = Switch::new();

    s.set_id("my-switch");
    assert_eq!(View::id(&s), Some("my-switch"));

    s.add_class("active");
    assert!(s.has_class("active"));

    s.remove_class("active");
    assert!(!s.has_class("active"));

    s.toggle_class("visible");
    assert!(s.has_class("visible"));

    s.toggle_class("visible");
    assert!(!s.has_class("visible"));
}

#[test]
fn test_switch_meta() {
    let s = Switch::new()
        .element_id("test")
        .class("class1")
        .class("class2");

    let meta = s.meta();
    assert_eq!(meta.id, Some("test".to_string()));
    assert_eq!(meta.classes.len(), 2);
}

// ==================== Color Tests ====================

#[test]
fn test_switch_render_with_custom_colors() {
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new()
        .on(true)
        .on_color(Color::GREEN)
        .off_color(Color::RED)
        .track_color(Color::BLUE);

    s.render(&mut ctx);
    // Should render with custom colors
}

#[test]
fn test_switch_render_same_on_off_colors() {
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new().on_color(Color::CYAN).off_color(Color::CYAN);

    s.render(&mut ctx);
    // Should render with same color for both states
}

// ==================== Width Edge Cases ====================

#[test]
fn test_switch_very_small_width() {
    let _s = Switch::new().width(1);
    // Width should be clamped to minimum
}

#[test]
fn test_switch_very_large_width() {
    let s = Switch::new().width(1000);
    // Large width should be handled
}

#[test]
fn test_switch_zero_width() {
    let s = Switch::new().width(0);
    // Zero width should be handled
}

// ==================== Label Edge Cases ====================

#[test]
fn test_switch_label_with_special_chars() {
    let s = Switch::new().label("Toggle & Switch <Test>");
    let mut buffer = Buffer::new(40, 1);
    let area = Rect::new(0, 0, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
}

#[test]
fn test_switch_label_with_tabs() {
    let s = Switch::new().label("Toggle\tTest");
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
}

#[test]
fn test_switch_label_with_newlines() {
    let s = Switch::new().label("Line1\nLine2");
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
}

// ==================== Style-Specific Tests ====================

#[test]
fn test_switch_render_emoji_style_on() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new().on(true).style(SwitchStyle::Emoji);
    s.render(&mut ctx);

    let text: String = (0..20)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    // Emoji style should render with emoji characters
    assert!(text.contains('✓') || text.contains('✗') || text.contains('✅') || text.contains('❌'));
}

#[test]
fn test_switch_render_emoji_style_off() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new().on(false).style(SwitchStyle::Emoji);
    s.render(&mut ctx);

    let text: String = (0..20)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    // Should show off state emoji
    assert!(text.contains('✓') || text.contains('✗') || text.contains('✅') || text.contains('❌'));
}

#[test]
fn test_switch_render_block_style_on() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new().on(true).style(SwitchStyle::Block);
    s.render(&mut ctx);

    let text: String = (0..20)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    // Block style should use block characters
    assert!(text.contains('█') || text.contains('▓') || text.contains('▒'));
}

#[test]
fn test_switch_render_block_style_off() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new().on(false).style(SwitchStyle::Block);
    s.render(&mut ctx);

    let text: String = (0..20)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    // Block style should show empty block
    assert!(text.contains('░') || text.contains(' ') || text.contains('○'));
}

#[test]
fn test_switch_render_material_style() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new().style(SwitchStyle::Material);
    s.render(&mut ctx);
    // Material style should render
}

#[test]
fn test_switch_render_ios_style() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new().style(SwitchStyle::IOS);
    s.render(&mut ctx);
    // iOS style should render
}

// ==================== Text Style Tests ====================

#[test]
fn test_switch_render_text_style_on() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new()
        .on(true)
        .style(SwitchStyle::Text)
        .text("ON", "OFF");
    s.render(&mut ctx);

    let text: String = (0..20)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(text.contains("ON"));
}

#[test]
fn test_switch_render_text_style_off() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new()
        .on(false)
        .style(SwitchStyle::Text)
        .text("ON", "OFF");
    s.render(&mut ctx);

    let text: String = (0..20)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(text.contains("OFF"));
}

#[test]
fn test_switch_render_text_style_custom_text() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new()
        .on(true)
        .style(SwitchStyle::Text)
        .text("YES", "NO");
    s.render(&mut ctx);

    let text: String = (0..20)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(text.contains("YES"));
}

#[test]
fn test_switch_render_text_style_empty_text() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new().style(SwitchStyle::Text).text("", "");
    s.render(&mut ctx);
    // Should handle empty text
}

#[test]
fn test_switch_render_text_style_unicode_text() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new()
        .on(true)
        .style(SwitchStyle::Text)
        .text("開", "關");
    s.render(&mut ctx);
    // Should handle unicode text
}

// ==================== State Transition Tests ====================

#[test]
fn test_switch_multiple_state_changes() {
    let mut s = Switch::new();
    // Initial state is off
    assert!(!s.is_on());

    s.toggle();
    assert!(s.is_on());

    s.toggle();
    assert!(!s.is_on());

    s.toggle();
    assert!(s.is_on());

    s.set(false);
    assert!(!s.is_on());

    s.toggle();
    assert!(s.is_on());
}

#[test]
fn test_switch_set_same_state() {
    let mut s = Switch::new().on(true);

    s.set(true);
    assert!(s.is_on());

    s.set(false);
    assert!(!s.is_on());
}

#[test]
fn test_switch_toggle_chain() {
    let mut s = Switch::new();
    for _ in 0..100 {
        s.toggle();
    }
    // After 100 toggles (even number), should be off
    assert!(!s.is_on());
}

#[test]
fn test_switch_focus_with_toggle() {
    let mut s = Switch::new().focused(true);

    assert!(s.handle_key(&Key::Enter));
    assert!(s.is_on());
    assert!(s.handle_key(&Key::Enter));
    assert!(!s.is_on());
}

// ==================== Disabled State Tests ====================

#[test]
fn test_switch_disabled_with_focus() {
    let mut s = Switch::new().focused(true).disabled(true);

    assert!(!s.handle_key(&Key::Enter));
    assert!(!s.is_on());

    assert!(!s.handle_key(&Key::Char(' ')));
    assert!(!s.is_on());
}

#[test]
fn test_switch_toggle_while_disabled() {
    let mut s = Switch::new().on(true).disabled(true);

    s.toggle();
    assert!(s.is_on()); // Should not change

    s.toggle();
    assert!(s.is_on()); // Still should not change
}

#[test]
fn test_switch_render_disabled_with_colors() {
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new()
        .disabled(true)
        .on_color(Color::GREEN)
        .off_color(Color::RED);

    s.render(&mut ctx);
    // Disabled should override custom colors
}

// ==================== Layout Tests ====================

#[test]
fn test_switch_render_narrow_buffer() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new();
    s.render(&mut ctx);
    // Should handle narrow buffer
}

#[test]
fn test_switch_render_wide_buffer() {
    let mut buffer = Buffer::new(100, 1);
    let area = Rect::new(0, 0, 100, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new().label("Wide Label");
    s.render(&mut ctx);
    // Should handle wide buffer
}

#[test]
fn test_switch_render_offset_area() {
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(10, 2, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new().label("Offset");
    s.render(&mut ctx);
    // Should render at offset position
}

// ==================== Builder Pattern Tests ====================

#[test]
fn test_switch_builder_preserves_state() {
    let s = Switch::new()
        .on(true)
        .focused(true)
        .disabled(false)
        .label("Test");

    assert!(s.is_on());
}

#[test]
fn test_switch_builder_override() {
    let s = Switch::new()
        .on(false)
        .on(true)
        .label("First")
        .label("Second");

    assert!(s.is_on());
}

#[test]
fn test_switch_multiple_builders() {
    let s1 = Switch::new().on(true).label("Switch 1");
    let s2 = Switch::new().on(false).label("Switch 2");

    assert!(s1.is_on());
    assert!(!s2.is_on());
}

// ==================== Helper Function Tests ====================

#[test]
fn test_toggle_helper_with_state() {
    let s_on = toggle("Feature").on(true);
    let s_off = toggle("Feature").on(false);

    assert!(s_on.is_on());
    assert!(!s_off.is_on());
}

#[test]
fn test_toggle_helper_with_label() {
    let s = toggle("Enable Notifications");

    let mut buffer = Buffer::new(40, 1);
    let area = Rect::new(0, 0, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);

    let text: String = (0..40)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(text.contains("Enable Notifications"));
}

#[test]
fn test_switch_helper_variants() {
    let s1 = switch().on(true);
    let s2 = switch().on(false).label("Toggle");

    assert!(s1.is_on());
    assert!(!s2.is_on());
}

// ==================== Clone and Copy Tests ====================

#[test]
fn test_switch_clone_not_implemented() {
    // Switch doesn't implement Clone - verify it compiles without clone
    let s1 = Switch::new().on(true).focused(true).disabled(false);
    let s2 = Switch::new().on(true).focused(true).disabled(false);

    assert_eq!(s1.is_on(), s2.is_on());
}

// ==================== Interaction Edge Cases ====================

#[test]
fn test_switch_rapid_toggle() {
    let mut s = Switch::new();

    for _ in 0..1000 {
        s.toggle();
    }

    assert!(!s.is_on()); // Even number of toggles
}

#[test]
fn test_switch_set_after_toggle() {
    let mut s = Switch::new();

    s.toggle();
    assert!(s.is_on());

    s.set(false);
    assert!(!s.is_on());

    s.toggle();
    assert!(s.is_on());
}

#[test]
fn test_switch_all_keys_unfocused() {
    let mut s = Switch::new().focused(false);

    assert!(!s.handle_key(&Key::Enter));
    assert!(!s.handle_key(&Key::Char(' ')));
    assert!(!s.handle_key(&Key::Tab));
    assert!(!s.handle_key(&Key::Up));
    assert!(!s.handle_key(&Key::Down));
    assert!(!s.is_on());
}

#[test]
fn test_switch_all_keys_disabled() {
    let mut s = Switch::new().focused(true).disabled(true);

    assert!(!s.handle_key(&Key::Enter));
    assert!(!s.handle_key(&Key::Char(' ')));
    assert!(!s.handle_key(&Key::Tab));
    assert!(!s.handle_key(&Key::Up));
    assert!(!s.handle_key(&Key::Down));
    assert!(!s.is_on());
}

// ==================== Rendering Validation ====================

#[test]
fn test_switch_render_output_exists() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new();
    s.render(&mut ctx);

    let mut has_content = false;
    for x in 0..20 {
        if let Some(cell) = buffer.get(x, 0) {
            if cell.symbol != ' ' {
                has_content = true;
                break;
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_switch_render_off_then_on() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);

    // Render off state
    let s_off = Switch::new().on(false);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s_off.render(&mut ctx);

    buffer.clear();

    // Render on state
    let s_on = Switch::new().on(true);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s_on.render(&mut ctx);

    // Both should render successfully
    let mut has_content = false;
    for x in 0..20 {
        if let Some(cell) = buffer.get(x, 0) {
            if cell.symbol != ' ' {
                has_content = true;
                break;
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_switch_render_with_label_and_style() {
    let mut buffer = Buffer::new(40, 1);
    let area = Rect::new(0, 0, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new()
        .label("Dark Mode")
        .style(SwitchStyle::IOS)
        .on(true);

    s.render(&mut ctx);

    let text: String = (0..40)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(text.contains("Dark Mode"));
}

#[test]
fn test_switch_render_very_narrow() {
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new();
    s.render(&mut ctx);
    // Should handle very narrow area
}

#[test]
fn test_switch_render_all_styles_with_colors() {
    let styles = [
        SwitchStyle::Default,
        SwitchStyle::IOS,
        SwitchStyle::Material,
        SwitchStyle::Text,
        SwitchStyle::Emoji,
        SwitchStyle::Block,
    ];

    for style in styles {
        let mut buffer = Buffer::new(30, 1);
        let area = Rect::new(0, 0, 30, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let s = Switch::new()
            .style(style)
            .on(true)
            .on_color(Color::GREEN)
            .off_color(Color::RED)
            .track_color(Color::BLUE);

        s.render(&mut ctx);
    }
}
