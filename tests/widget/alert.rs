//! Alert widget tests

use revue::event::Key;
use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{alert, Alert, AlertLevel, AlertVariant, StyledView, View};

// ==================== Constructor Tests ====================

#[test]
fn test_alert_new() {
    let _a = Alert::new("Test message");
    // Private field - just verify it compiles
}

#[test]
fn test_alert_default() {
    let _a = Alert::default();
    // Private field - just verify it compiles
}

#[test]
fn test_alert_helper() {
    let _a = alert("Test message");
    // Private field - just verify it compiles
}

// ==================== Builder Tests ====================

#[test]
fn test_alert_title() {
    let _a = Alert::new("Message").title("Title");
    // Private field - just verify it compiles
}

#[test]
fn test_alert_level() {
    let _a = Alert::new("Message").level(AlertLevel::Warning);
    // Private field - just verify it compiles
}

#[test]
fn test_alert_variant() {
    let _a = Alert::new("Message").variant(AlertVariant::Minimal);
    // Private field - just verify it compiles
}

#[test]
fn test_alert_dismissible() {
    let _a = Alert::new("Message").dismissible(true);
    // Private field - just verify it compiles
}

#[test]
fn test_alert_not_dismissible() {
    let _a = Alert::new("Message").dismissible(false);
    // Private field - just verify it compiles
}

#[test]
fn test_alert_focused() {
    let _a = Alert::new("Test").focused(true);
    // Private field - just verify it compiles
}

#[test]
fn test_alert_not_focused() {
    let _a = Alert::new("Test").focused(false);
    // Private field - just verify it compiles
}

#[test]
fn test_alert_disabled() {
    let _a = Alert::new("Test").disabled(true);
    // Private field - just verify it compiles
}

#[test]
fn test_alert_not_disabled() {
    let _a = Alert::new("Test").disabled(false);
    // Private field - just verify it compiles
}

#[test]
fn test_alert_icon() {
    let _a = Alert::new("Message").icon(true);
    // Private field - just verify it compiles
}

#[test]
fn test_alert_no_icon() {
    let _a = Alert::new("Message").icon(false);
    // Private field - just verify it compiles
}

#[test]
fn test_alert_custom_icon() {
    let _a = Alert::new("Message").custom_icon('★');
    // Private field - just verify it compiles
}

#[test]
fn test_alert_fg() {
    let _a = Alert::new("Message").fg(Color::RED);
    // Private field - just verify it compiles
}

#[test]
fn test_alert_bg() {
    let _a = Alert::new("Message").bg(Color::BLUE);
    // Private field - just verify it compiles
}

#[test]
fn test_alert_builder_chain() {
    let _a = Alert::new("Message")
        .title("Title")
        .level(AlertLevel::Warning)
        .variant(AlertVariant::Filled)
        .dismissible(true)
        .focused(true)
        .disabled(false)
        .icon(true)
        .fg(Color::WHITE)
        .bg(Color::BLACK);
    // Just verify it compiles
}

// ==================== Dismissal Tests ====================

#[test]
fn test_alert_dismiss() {
    let mut a = Alert::new("Test").dismissible(true);
    assert!(!a.is_dismissed());

    a.dismiss();
    assert!(a.is_dismissed());
}

#[test]
fn test_alert_reset() {
    let mut a = Alert::new("Test").dismissible(true);
    a.dismiss();
    assert!(a.is_dismissed());

    a.reset();
    assert!(!a.is_dismissed());
}

#[test]
fn test_alert_dismiss_not_dismissible() {
    let mut a = Alert::new("Test").dismissible(false);
    assert!(!a.is_dismissed());

    // Can still manually dismiss
    a.dismiss();
    assert!(a.is_dismissed());
}

// ==================== Key Handling Tests ====================

#[test]
fn test_alert_handle_key_x() {
    let mut a = Alert::new("Test").dismissible(true);
    assert!(a.handle_key(&Key::Char('x')));
    assert!(a.is_dismissed());
}

#[test]
fn test_alert_handle_key_x_uppercase() {
    let mut a = Alert::new("Test").dismissible(true);
    assert!(a.handle_key(&Key::Char('X')));
    assert!(a.is_dismissed());
}

#[test]
fn test_alert_handle_key_escape() {
    let mut a = Alert::new("Test").dismissible(true);
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
fn test_alert_handle_key_already_dismissed() {
    let mut a = Alert::new("Test").dismissible(true);
    a.dismiss();

    assert!(!a.handle_key(&Key::Char('x')));
    assert!(a.is_dismissed());
}

#[test]
fn test_alert_handle_key_other_keys() {
    let mut a = Alert::new("Test").dismissible(true);

    assert!(!a.handle_key(&Key::Char('a')));
    assert!(!a.handle_key(&Key::Enter));
    assert!(!a.handle_key(&Key::Tab));
    assert!(!a.is_dismissed());
}

// ==================== AlertLevel Tests ====================

#[test]
fn test_alert_level_info() {
    let level = AlertLevel::Info;
    assert_eq!(level.color(), Color::CYAN);
    assert_eq!(level.icon(), 'ℹ');
}

#[test]
fn test_alert_level_success() {
    let level = AlertLevel::Success;
    assert_eq!(level.color(), Color::GREEN);
    assert_eq!(level.icon(), '✓');
}

#[test]
fn test_alert_level_warning() {
    let level = AlertLevel::Warning;
    assert_eq!(level.color(), Color::YELLOW);
    assert_eq!(level.icon(), '⚠');
}

#[test]
fn test_alert_level_error() {
    let level = AlertLevel::Error;
    assert_eq!(level.color(), Color::RED);
    assert_eq!(level.icon(), '✗');
}

#[test]
fn test_alert_level_default() {
    let level = AlertLevel::default();
    assert_eq!(level, AlertLevel::Info);
}

#[test]
fn test_alert_level_partial_eq() {
    assert_eq!(AlertLevel::Info, AlertLevel::Info);
    assert_eq!(AlertLevel::Success, AlertLevel::Success);
    assert_ne!(AlertLevel::Info, AlertLevel::Error);
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
fn test_alert_level_bg_colors() {
    let info_bg = AlertLevel::Info.bg_color();
    let success_bg = AlertLevel::Success.bg_color();
    let warning_bg = AlertLevel::Warning.bg_color();
    let error_bg = AlertLevel::Error.bg_color();

    assert_ne!(info_bg, success_bg);
    assert_ne!(success_bg, warning_bg);
    assert_ne!(warning_bg, error_bg);
}

#[test]
fn test_alert_level_border_colors() {
    let info_border = AlertLevel::Info.border_color();
    let success_border = AlertLevel::Success.border_color();
    let warning_border = AlertLevel::Warning.border_color();
    let error_border = AlertLevel::Error.border_color();

    assert_ne!(info_border, success_border);
    assert_ne!(success_border, warning_border);
    assert_ne!(warning_border, error_border);
}

// ==================== AlertVariant Tests ====================

#[test]
fn test_alert_variant_default() {
    let variant = AlertVariant::default();
    assert_eq!(variant, AlertVariant::Filled);
}

#[test]
fn test_alert_variant_partial_eq() {
    assert_eq!(AlertVariant::Filled, AlertVariant::Filled);
    assert_eq!(AlertVariant::Outlined, AlertVariant::Outlined);
    assert_eq!(AlertVariant::Minimal, AlertVariant::Minimal);
    assert_ne!(AlertVariant::Filled, AlertVariant::Outlined);
}

#[test]
fn test_alert_variant_all_variants() {
    let _ = AlertVariant::Filled;
    let _ = AlertVariant::Outlined;
    let _ = AlertVariant::Minimal;
}

// ==================== Helper Functions Tests ====================

#[test]
fn test_alert_helper_function() {
    let _a = alert("Test message");
    // Just verify it compiles
}

#[test]
fn test_alert_info_helper() {
    let _a = Alert::info("Info message");
    // Just verify it compiles
}

#[test]
fn test_alert_success_helper() {
    let _a = Alert::success("Success message");
    // Just verify it compiles
}

#[test]
fn test_alert_warning_helper() {
    let _a = Alert::warning("Warning message");
    // Just verify it compiles
}

#[test]
fn test_alert_error_helper() {
    let _a = Alert::error("Error message");
    // Just verify it compiles
}

// ==================== Height Tests ====================

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
fn test_alert_height_outlined() {
    let outlined = Alert::new("msg").variant(AlertVariant::Outlined);
    assert_eq!(outlined.height(), 3);

    let outlined_title = Alert::new("msg")
        .title("Title")
        .variant(AlertVariant::Outlined);
    assert_eq!(outlined_title.height(), 4);
}

#[test]
fn test_alert_height_dismissed() {
    let mut dismissed = Alert::new("msg").dismissible(true);
    dismissed.dismiss();
    assert_eq!(dismissed.height(), 0);
}

// ==================== Rendering Tests ====================

#[test]
fn test_alert_render_filled() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = Alert::new("Test message").variant(AlertVariant::Filled);
    a.render(&mut ctx);

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

    // Should not render anything
    assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
}

#[test]
fn test_alert_render_with_title() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let alert = Alert::new("Test message")
        .title("Alert Title")
        .variant(AlertVariant::Filled);
    alert.render(&mut ctx);

    let title_cell = buffer.get(4, 1).unwrap();
    assert_eq!(title_cell.symbol, 'A');
    assert!(title_cell.modifier.contains(Modifier::BOLD));
}

#[test]
fn test_alert_render_without_icon() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let alert = Alert::new("Test message")
        .icon(false)
        .variant(AlertVariant::Filled);
    alert.render(&mut ctx);

    assert_ne!(buffer.get(2, 1).unwrap().symbol, 'ℹ');
}

#[test]
fn test_alert_render_with_custom_icon() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let alert = Alert::new("Test message")
        .custom_icon('★')
        .variant(AlertVariant::Filled);
    alert.render(&mut ctx);

    assert_eq!(buffer.get(2, 1).unwrap().symbol, '★');
}

#[test]
fn test_alert_render_too_small_area() {
    let mut buffer = Buffer::new(40, 5);
    let too_small = Rect::new(0, 0, 3, 1);
    let mut ctx = RenderContext::new(&mut buffer, too_small);

    let alert = Alert::new("Test message");
    alert.render(&mut ctx);
    // Should not crash
}

#[test]
fn test_alert_render_message_truncation() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let long_message = "This is a very long message that should be truncated";
    let alert = Alert::new(long_message).variant(AlertVariant::Filled);
    alert.render(&mut ctx);

    assert_eq!(buffer.get(2, 1).unwrap().symbol, 'ℹ');
}

#[test]
fn test_alert_render_dismiss_button_filled() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let alert = Alert::new("Test")
        .dismissible(true)
        .variant(AlertVariant::Filled);
    alert.render(&mut ctx);

    assert_eq!(buffer.get(37, 1).unwrap().symbol, '×');
}

#[test]
fn test_alert_render_no_dismiss_button() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let alert = Alert::new("Test")
        .dismissible(false)
        .variant(AlertVariant::Filled);
    alert.render(&mut ctx);

    assert_ne!(buffer.get(37, 1).unwrap().symbol, '×');
}

#[test]
fn test_alert_render_outlined_dismiss_button() {
    let mut buffer = Buffer::new(40, 3);
    let area = Rect::new(0, 0, 40, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let alert = Alert::new("Test")
        .dismissible(true)
        .variant(AlertVariant::Outlined);
    alert.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┃');
    assert_eq!(buffer.get(38, 0).unwrap().symbol, '×');
}

#[test]
fn test_alert_render_minimal_dismiss_button() {
    let mut buffer = Buffer::new(40, 1);
    let area = Rect::new(0, 0, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let alert = Alert::new("Test")
        .dismissible(true)
        .variant(AlertVariant::Minimal);
    alert.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'ℹ');
    assert_eq!(buffer.get(39, 0).unwrap().symbol, '×');
}

// ==================== CSS Integration Tests ====================

#[test]
fn test_alert_builder_id() {
    let alert = Alert::new("Test message").element_id("test-alert");
    assert_eq!(View::id(&alert), Some("test-alert"));
}

#[test]
fn test_alert_builder_classes() {
    let alert = Alert::new("Test").class("important").class("notification");
    assert!(alert.has_class("important"));
    assert!(alert.has_class("notification"));
    assert!(!alert.has_class("secondary"));
}

#[test]
fn test_alert_builder_styled_methods() {
    let mut alert = Alert::new("Test");

    alert.set_id("my-alert");
    assert_eq!(View::id(&alert), Some("my-alert"));

    alert.add_class("primary");
    assert!(alert.has_class("primary"));

    alert.remove_class("primary");
    assert!(!alert.has_class("primary"));

    alert.toggle_class("active");
    assert!(alert.has_class("active"));

    alert.toggle_class("active");
    assert!(!alert.has_class("active"));
}

#[test]
fn test_alert_color_builders() {
    use revue::style::Color;

    let alert = Alert::new("Test").fg(Color::RED).bg(Color::BLUE);
    let _ = alert;
}

// ==================== Edge Cases ====================

#[test]
fn test_alert_empty_message() {
    let _a = Alert::new("");
    // Just verify it compiles
}

#[test]
fn test_alert_unicode_message() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let alert = Alert::new("Hello 世界 🌍");
    alert.render(&mut ctx);
    // Should handle unicode without panic
}

#[test]
fn test_alert_special_characters_message() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let alert = Alert::new("!@#$%^&*()");
    alert.render(&mut ctx);
    // Should handle special characters
}

#[test]
fn test_alert_very_long_message() {
    let long_msg = "This is an extremely long alert message that contains a lot of text and would normally wrap or truncate in most UI scenarios but the widget should handle it gracefully without panicking or crashing.";
    let _a = Alert::new(long_msg);
    // Just verify it compiles
}

#[test]
fn test_alert_zero_area() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 0, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = Alert::new("Test");
    a.render(&mut ctx);
    // Should handle zero area gracefully
}

#[test]
fn test_alert_newline_in_message() {
    let msg = "Line 1\nLine 2\nLine 3";
    let _a = Alert::new(msg);
    // Just verify it compiles
}

#[test]
fn test_alert_render_all_levels() {
    for level in [
        AlertLevel::Info,
        AlertLevel::Success,
        AlertLevel::Warning,
        AlertLevel::Error,
    ] {
        let mut buffer = Buffer::new(40, 5);
        let area = Rect::new(0, 0, 40, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let alert = Alert::new("Test").level(level);
        alert.render(&mut ctx);
        // Should render without panic for all levels
    }
}

#[test]
fn test_alert_render_all_variants() {
    for variant in [
        AlertVariant::Filled,
        AlertVariant::Outlined,
        AlertVariant::Minimal,
    ] {
        let mut buffer = Buffer::new(40, 5);
        let area = Rect::new(0, 0, 40, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let alert = Alert::new("Test").variant(variant);
        alert.render(&mut ctx);
        // Should render without panic for all variants
    }
}

#[test]
fn test_alert_focused_rendering() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let alert = Alert::new("Test").focused(true);
    alert.render(&mut ctx);
    // Should render with focus indication
}

#[test]
fn test_alert_disabled_rendering() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let alert = Alert::new("Test").disabled(true);
    alert.render(&mut ctx);
    // Should render with disabled styling
}
