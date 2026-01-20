//! Alert widget tests

use revue::event::Key;
use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{Alert, AlertLevel, AlertVariant, StyledView, View};

#[test]
fn test_alert_dismiss() {
    let mut a = Alert::new("Test").dismissible(true);
    assert!(!a.is_dismissed());

    a.dismiss();
    assert!(a.is_dismissed());

    a.reset();
    assert!(!a.is_dismissed());
}

#[test]
fn test_alert_handle_key() {
    let mut a = Alert::new("Test").dismissible(true);

    assert!(a.handle_key(&Key::Char('x')));
    assert!(a.is_dismissed());

    a.reset();
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
fn test_alert_render_filled() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = Alert::new("Test message").variant(AlertVariant::Filled);
    a.render(&mut ctx);

    // Check border corners
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

    // Check left accent border
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

    // Check icon
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

    // Should not render anything (buffer should be default)
    assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
}

// =============================================================================

// =============================================================================
// Builder method tests
// =============================================================================

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
fn test_alert_focused() {
    let alert = Alert::new("Test").focused(true);
    assert!(alert.is_focused());
}

#[test]
fn test_alert_disabled() {
    let alert = Alert::new("Test").disabled(true);
    assert!(alert.is_disabled());
}

#[test]
fn test_alert_color_builders() {
    use revue::style::Color;

    let alert = Alert::new("Test").fg(Color::RED).bg(Color::BLUE);

    // Color builders should be available via state builders
    let _ = alert;
}

// =============================================================================
// AlertLevel variant tests
// =============================================================================

#[test]
fn test_alert_level_info_render() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let alert = Alert::info("Information message").title("Info");
    alert.render(&mut ctx);

    // Check that info icon is rendered
    assert_eq!(buffer.get(2, 1).unwrap().symbol, 'ℹ');
    // Check that title is rendered with bold modifier
    let title_cell = buffer.get(4, 1).unwrap();
    assert_eq!(title_cell.symbol, 'I');
    assert!(title_cell.modifier.contains(Modifier::BOLD));
}

#[test]
fn test_alert_level_success_render() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let alert = Alert::success("Operation completed");
    alert.render(&mut ctx);

    // Check that success icon is rendered
    assert_eq!(buffer.get(2, 1).unwrap().symbol, '✓');
}

#[test]
fn test_alert_level_warning_render() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let alert = Alert::warning("Proceed with caution");
    alert.render(&mut ctx);

    // Check that warning icon is rendered
    assert_eq!(buffer.get(2, 1).unwrap().symbol, '⚠');
}

#[test]
fn test_alert_level_error_render() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let alert = Alert::error("Critical error occurred");
    alert.render(&mut ctx);

    // Check that error icon is rendered
    assert_eq!(buffer.get(2, 1).unwrap().symbol, '✗');
}

#[test]
fn test_alert_level_bg_colors() {
    let info_bg = AlertLevel::Info.bg_color();
    let success_bg = AlertLevel::Success.bg_color();
    let warning_bg = AlertLevel::Warning.bg_color();
    let error_bg = AlertLevel::Error.bg_color();

    // Verify different background colors
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

    // Verify different border colors
    assert_ne!(info_border, success_border);
    assert_ne!(success_border, warning_border);
    assert_ne!(warning_border, error_border);
}

// =============================================================================
// AlertVariant tests
// =============================================================================

#[test]
fn test_alert_variant_filled_render() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let alert = Alert::new("Test message")
        .title("Title")
        .variant(AlertVariant::Filled);
    alert.render(&mut ctx);

    // Check border corners for filled variant
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '╭');
    assert_eq!(buffer.get(39, 0).unwrap().symbol, '╮');
    assert_eq!(buffer.get(0, 4).unwrap().symbol, '╰');
    assert_eq!(buffer.get(39, 4).unwrap().symbol, '╯');

    // Check background is filled
    let bg_cell = buffer.get(1, 1).unwrap();
    assert!(bg_cell.bg.is_some());
}

#[test]
fn test_alert_variant_outlined_render() {
    let mut buffer = Buffer::new(40, 3);
    let area = Rect::new(0, 0, 40, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let alert = Alert::new("Test message")
        .title("Title")
        .variant(AlertVariant::Outlined);
    alert.render(&mut ctx);

    // Check left accent border
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┃');
    assert_eq!(buffer.get(0, 1).unwrap().symbol, '┃');
    assert_eq!(buffer.get(0, 2).unwrap().symbol, '┃');
}

#[test]
fn test_alert_variant_minimal_render() {
    let mut buffer = Buffer::new(40, 2);
    let area = Rect::new(0, 0, 40, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let alert = Alert::new("Test message")
        .title("Title")
        .variant(AlertVariant::Minimal);
    alert.render(&mut ctx);

    // Check icon is rendered
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'ℹ');
    // Title should be on first line
    assert_eq!(buffer.get(2, 0).unwrap().symbol, 'T');
    // Message should be on second line
    assert_eq!(buffer.get(2, 1).unwrap().symbol, 'T');
}

#[test]
fn test_alert_variant_minimal_no_title() {
    let mut buffer = Buffer::new(40, 1);
    let area = Rect::new(0, 0, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let alert = Alert::new("Test message").variant(AlertVariant::Minimal);
    alert.render(&mut ctx);

    // Check icon and message on same line
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'ℹ');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, 'T');
}

// =============================================================================
// Dismiss functionality tests
// =============================================================================

#[test]
fn test_alert_dismiss_not_dismissible() {
    let mut a = Alert::new("Test").dismissible(false);
    assert!(!a.is_dismissed());

    // Can still manually dismiss even if not dismissible via keyboard
    a.dismiss();
    assert!(a.is_dismissed());
}

#[test]
fn test_alert_handle_key_dismiss_x_uppercase() {
    let mut a = Alert::new("Test").dismissible(true);

    assert!(a.handle_key(&Key::Char('X')));
    assert!(a.is_dismissed());
}

#[test]
fn test_alert_handle_key_already_dismissed() {
    let mut a = Alert::new("Test").dismissible(true);
    a.dismiss();

    // Should not handle keys when already dismissed
    assert!(!a.handle_key(&Key::Char('x')));
    assert!(a.is_dismissed());
}

#[test]
fn test_alert_handle_key_other_keys() {
    let mut a = Alert::new("Test").dismissible(true);

    // Other keys should not dismiss
    assert!(!a.handle_key(&Key::Char('a')));
    assert!(!a.handle_key(&Key::Enter));
    assert!(!a.is_dismissed());
}

#[test]
fn test_alert_dismissible_render_dismiss_button() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let alert = Alert::new("Test")
        .dismissible(true)
        .variant(AlertVariant::Filled);
    alert.render(&mut ctx);

    // Check dismiss button (×) is present
    assert_eq!(buffer.get(37, 1).unwrap().symbol, '×');
}

#[test]
fn test_alert_not_dismissible_no_dismiss_button() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let alert = Alert::new("Test")
        .dismissible(false)
        .variant(AlertVariant::Filled);
    alert.render(&mut ctx);

    // Dismiss button should not be present
    assert_ne!(buffer.get(37, 1).unwrap().symbol, '×');
}

// =============================================================================
// Render operation tests
// =============================================================================

#[test]
fn test_alert_render_with_title() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let alert = Alert::new("Test message")
        .title("Alert Title")
        .variant(AlertVariant::Filled);
    alert.render(&mut ctx);

    // Check title is rendered
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

    // Icon should not be rendered at position 2
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

    // Custom icon should be rendered
    assert_eq!(buffer.get(2, 1).unwrap().symbol, '★');
}

#[test]
fn test_alert_render_too_small_area() {
    let mut buffer = Buffer::new(40, 5);
    let too_small = Rect::new(0, 0, 3, 1); // width < 5
    let mut ctx = RenderContext::new(&mut buffer, too_small);

    let alert = Alert::new("Test message");
    alert.render(&mut ctx);

    // Should not crash or render when area is too small
}

#[test]
fn test_alert_render_message_truncation() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let long_message = "This is a very long message that should be truncated";
    let alert = Alert::new(long_message).variant(AlertVariant::Filled);
    alert.render(&mut ctx);

    // Should render without panicking, message is truncated
    assert_eq!(buffer.get(2, 1).unwrap().symbol, 'ℹ');
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

    // Check left border
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┃');
    // Check dismiss button
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

    // Check icon
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'ℹ');
    // Check dismiss button
    assert_eq!(buffer.get(39, 0).unwrap().symbol, '×');
}

// =============================================================================
// Height calculation tests
// =============================================================================

#[test]
fn test_alert_height_outlined() {
    let outlined = Alert::new("msg").variant(AlertVariant::Outlined);
    assert_eq!(outlined.height(), 3);

    let outlined_title = Alert::new("msg")
        .title("Title")
        .variant(AlertVariant::Outlined);
    assert_eq!(outlined_title.height(), 4);
}

// =============================================================================
// AlertLevel::Default test
// =============================================================================

#[test]
fn test_alert_level_default() {
    let default_level = AlertLevel::default();
    assert_eq!(default_level, AlertLevel::Info);
}
