//! NotificationCenter tests
//!
//! Tests for NotificationCenter extracted from source files.

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::notification_center;
use revue::widget::{Notification, NotificationLevel, NotificationPosition};

// =========================================================================
// Notification basic tests
// =========================================================================

#[test]
fn test_notification_new() {
    let n = Notification::new("Test message");
    assert_eq!(n.message, "Test message");
    assert!(matches!(n.level, NotificationLevel::Info));
}

#[test]
fn test_notification_levels() {
    let info = Notification::info("Info");
    let success = Notification::success("Success");
    let warning = Notification::warning("Warning");
    let error = Notification::error("Error");

    assert!(matches!(info.level, NotificationLevel::Info));
    assert!(matches!(success.level, NotificationLevel::Success));
    assert!(matches!(warning.level, NotificationLevel::Warning));
    assert!(matches!(error.level, NotificationLevel::Error));
}

#[test]
fn test_notification_builder() {
    let n = Notification::new("Test")
        .title("Title")
        .level(NotificationLevel::Warning)
        .duration(50)
        .dismissible(false)
        .progress(0.5)
        .action("Retry");

    assert_eq!(n.title, Some("Title".to_string()));
    assert!(matches!(n.level, NotificationLevel::Warning));
    assert_eq!(n.duration, 50);
    assert!(!n.dismissible);
    assert_eq!(n.progress, Some(0.5));
    assert_eq!(n.action, Some("Retry".to_string()));
}

#[test]
fn test_notification_expired() {
    let mut n = Notification::new("Test").duration(10);
    assert!(!n.is_expired());

    n.tick = 10;
    assert!(n.is_expired());
}

#[test]
fn test_notification_remaining() {
    let mut n = Notification::new("Test").duration(100);
    assert_eq!(n.remaining(), 1.0);

    n.tick = 50;
    assert_eq!(n.remaining(), 0.5);
}

// =========================================================================
// NotificationCenter basic tests
// =========================================================================

#[test]
fn test_center_new() {
    let c = notification_center();
    assert!(c.is_empty());
    assert_eq!(c.count(), 0);
}

#[test]
fn test_center_push() {
    let mut c = notification_center();
    c.push(Notification::info("Test"));
    assert_eq!(c.count(), 1);
}

#[test]
fn test_center_shortcuts() {
    let mut c = notification_center();
    c.info("Info");
    c.success("Success");
    c.warning("Warning");
    c.error("Error");
    assert_eq!(c.count(), 4);
}

#[test]
fn test_center_dismiss() {
    let mut c = notification_center();
    c.info("Test");
    let id = c.notifications[0].id;
    c.dismiss(id);
    assert!(c.is_empty());
}

#[test]
fn test_center_clear() {
    let mut c = notification_center();
    c.info("1");
    c.info("2");
    c.info("3");
    c.clear();
    assert!(c.is_empty());
}

#[test]
fn test_center_tick() {
    let mut c = notification_center();
    c.push(Notification::info("Test").duration(2));

    c.tick();
    assert_eq!(c.count(), 1);

    c.tick();
    assert!(c.is_empty());
}

#[test]
fn test_center_selection() {
    let mut c = notification_center();
    c.info("1");
    c.info("2");
    c.info("3");

    c.select_next();
    assert_eq!(c.selected, Some(0));

    c.select_next();
    assert_eq!(c.selected, Some(1));

    c.select_prev();
    assert_eq!(c.selected, Some(0));
}

#[test]
fn test_center_render() {
    let mut buffer = Buffer::new(50, 20);
    let area = Rect::new(0, 0, 50, 20);
    let mut ctx = revue::widget::traits::RenderContext::new(&mut buffer, area);

    let mut c = notification_center();
    c.info("Test notification");
    c.render(&mut ctx);
    // Smoke test
}

#[test]
fn test_level_icon() {
    assert_eq!(NotificationLevel::Info.icon(), 'ℹ');
    assert_eq!(NotificationLevel::Success.icon(), '✓');
    assert_eq!(NotificationLevel::Warning.icon(), '⚠');
    assert_eq!(NotificationLevel::Error.icon(), '✗');
}

#[test]
fn test_center_positions() {
    let positions = [
        NotificationPosition::TopRight,
        NotificationPosition::TopLeft,
        NotificationPosition::TopCenter,
        NotificationPosition::BottomRight,
        NotificationPosition::BottomLeft,
        NotificationPosition::BottomCenter,
    ];

    for pos in positions {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = revue::widget::traits::RenderContext::new(&mut buffer, area);

        let mut c = notification_center().position(pos);
        c.info("Test");
        c.render(&mut ctx);
    }
}

#[test]
fn test_helper() {
    let c = notification_center().max_visible(3);
    assert_eq!(c.max_visible, 3);
}

// =========================================================================
// NotificationCenter builder method tests
// =========================================================================

#[test]
fn test_notification_center_position_builder() {
    let center = notification_center().position(NotificationPosition::TopLeft);
    assert_eq!(center.position, NotificationPosition::TopLeft);
}

#[test]
fn test_notification_center_max_visible_builder() {
    let center = notification_center().max_visible(10);
    assert_eq!(center.max_visible, 10);
}

#[test]
fn test_notification_center_max_visible_minimum() {
    let center = notification_center().max_visible(0);
    assert_eq!(center.max_visible, 1); // Minimum is 1
}

#[test]
fn test_notification_center_width_builder() {
    let center = notification_center().width(60);
    assert_eq!(center.width, 60);
}

#[test]
fn test_notification_center_width_minimum() {
    let center = notification_center().width(10);
    assert_eq!(center.width, 20); // Minimum is 20
}

#[test]
fn test_notification_center_show_icons_builder() {
    let center = notification_center().show_icons(false);
    assert!(!center.show_icons);
}

#[test]
fn test_notification_center_show_timer_builder() {
    let center = notification_center().show_timer(false);
    assert!(!center.show_timer);
}

#[test]
fn test_notification_center_spacing_builder() {
    let center = notification_center().spacing(3);
    assert_eq!(center.spacing, 3);
}

#[test]
fn test_notification_center_focused_builder() {
    let center = notification_center().focused(true);
    assert!(center.focused);
}

// =========================================================================
// NotificationCenter builder chain tests
// =========================================================================

#[test]
fn test_notification_center_builder_chain() {
    let center = notification_center()
        .position(NotificationPosition::BottomLeft)
        .max_visible(7)
        .width(50)
        .show_icons(true)
        .show_timer(false)
        .spacing(2)
        .focused(true);

    assert_eq!(center.position, NotificationPosition::BottomLeft);
    assert_eq!(center.max_visible, 7);
    assert_eq!(center.width, 50);
    assert!(center.show_icons);
    assert!(!center.show_timer);
    assert_eq!(center.spacing, 2);
    assert!(center.focused);
}

// =========================================================================
// NotificationCenter edge case tests
// =========================================================================

#[test]
fn test_center_dismiss_selected_empty() {
    let mut c = notification_center();
    c.dismiss_selected(); // Should not panic
    assert_eq!(c.count(), 0);
}

#[test]
fn test_center_dismiss_selected_no_selection() {
    let mut c = notification_center();
    c.info("Test");
    c.dismiss_selected(); // Should not panic (no selection)
    assert_eq!(c.count(), 1);
}

#[test]
fn test_center_dismiss_selected_out_of_bounds() {
    let mut c = notification_center();
    c.info("Test");
    c.selected = Some(5); // Out of bounds
    c.dismiss_selected(); // Should not panic
    assert_eq!(c.count(), 1);
}

#[test]
fn test_center_tick_empty() {
    let mut c = notification_center();
    c.tick(); // Should not panic
    assert!(c.is_empty());
}

#[test]
fn test_center_tick_updates_tick_counter() {
    let mut c = notification_center();
    assert_eq!(c.tick_counter, 0);
    c.tick();
    assert_eq!(c.tick_counter, 1);
    c.tick();
    assert_eq!(c.tick_counter, 2);
}

#[test]
fn test_center_tick_updates_notification_ticks() {
    let mut c = notification_center();
    c.push(Notification::new("Test").duration(10));
    c.tick();
    assert_eq!(c.notifications[0].tick, 1);
    c.tick();
    assert_eq!(c.notifications[0].tick, 2);
}

#[test]
fn test_center_select_next_empty() {
    let mut c = notification_center();
    c.select_next(); // Should not panic
    assert_eq!(c.selected, None);
}

#[test]
fn test_center_select_prev_empty() {
    let mut c = notification_center();
    c.select_prev(); // Should not panic
    assert_eq!(c.selected, None);
}

#[test]
fn test_center_select_next_wraps() {
    let mut c = notification_center();
    c.info("1");
    c.info("2");
    c.selected = Some(1);
    c.select_next();
    assert_eq!(c.selected, Some(0)); // Wraps to start
}

#[test]
fn test_center_select_prev_wraps() {
    let mut c = notification_center();
    c.info("1");
    c.info("2");
    c.selected = Some(0);
    c.select_prev();
    assert_eq!(c.selected, Some(1)); // Wraps to end
}

#[test]
fn test_center_handle_key_not_focused() {
    let mut c = notification_center();
    c.info("Test");
    let result = c.handle_key(&Key::Down);
    assert!(!result); // Should not handle when not focused
}

#[test]
fn test_center_handle_key_empty() {
    let mut c = notification_center().focused(true);
    let result = c.handle_key(&Key::Down);
    assert!(!result); // Should not handle when empty
}

#[test]
fn test_center_handle_key_up() {
    let mut c = notification_center().focused(true);
    c.info("1");
    c.info("2");
    let result = c.handle_key(&Key::Up);
    assert!(result);
    assert_eq!(c.selected, Some(1));
}

#[test]
fn test_center_handle_key_down() {
    let mut c = notification_center().focused(true);
    c.info("1");
    c.info("2");
    let result = c.handle_key(&Key::Down);
    assert!(result);
    assert_eq!(c.selected, Some(0));
}

#[test]
fn test_center_handle_key_vim_keys() {
    let mut c = notification_center().focused(true);
    c.info("1");
    c.info("2");
    c.handle_key(&Key::Char('k'));
    assert_eq!(c.selected, Some(1));
    c.handle_key(&Key::Char('j'));
    assert_eq!(c.selected, Some(0));
}

#[test]
fn test_center_handle_key_dismiss() {
    let mut c = notification_center().focused(true);
    c.info("Test");
    c.select_next();
    let result = c.handle_key(&Key::Char('d'));
    assert!(result);
    assert!(c.is_empty());
}

#[test]
fn test_center_handle_key_delete() {
    let mut c = notification_center().focused(true);
    c.info("Test");
    c.select_next();
    let result = c.handle_key(&Key::Delete);
    assert!(result);
    assert!(c.is_empty());
}

#[test]
fn test_center_handle_key_clear() {
    let mut c = notification_center().focused(true);
    c.info("1");
    c.info("2");
    c.info("3");
    let result = c.handle_key(&Key::Char('c'));
    assert!(result);
    assert!(c.is_empty());
    assert_eq!(c.selected, None);
}

#[test]
fn test_center_handle_key_unknown() {
    let mut c = notification_center().focused(true);
    c.info("Test");
    let result = c.handle_key(&Key::Char('x'));
    assert!(!result);
}

// =========================================================================
// NotificationCenter Default trait tests
// =========================================================================

#[test]
fn test_notification_center_default() {
    let center = notification_center();
    assert!(center.is_empty());
    assert_eq!(center.count(), 0);
    assert_eq!(center.max_visible, 5);
    assert_eq!(center.position, NotificationPosition::TopRight);
    assert_eq!(center.width, 40);
    assert!(center.show_icons);
    assert!(center.show_timer);
    assert_eq!(center.spacing, 1);
    assert_eq!(center.tick_counter, 0);
    assert_eq!(center.selected, None);
    assert!(!center.focused);
}

#[test]
fn test_notification_center_default_vs_new() {
    let default_center = notification_center();
    let new_center = notification_center();

    assert_eq!(default_center.max_visible, new_center.max_visible);
    assert_eq!(default_center.position, new_center.position);
    assert_eq!(default_center.width, new_center.width);
}

// =========================================================================
// NotificationCenter render tests
// =========================================================================

#[test]
fn test_center_render_with_title() {
    let mut buffer = Buffer::new(50, 20);
    let area = Rect::new(0, 0, 50, 20);
    let mut ctx = revue::widget::traits::RenderContext::new(&mut buffer, area);

    let mut c = notification_center();
    c.push(Notification::new("Test message").title("Title"));
    c.render(&mut ctx);
    // Smoke test - should render without panic
}

#[test]
fn test_center_render_with_progress() {
    let mut buffer = Buffer::new(50, 20);
    let area = Rect::new(0, 0, 50, 20);
    let mut ctx = revue::widget::traits::RenderContext::new(&mut buffer, area);

    let mut c = notification_center();
    c.push(Notification::new("Test message").progress(0.5));
    c.render(&mut ctx);
}

#[test]
fn test_center_render_with_action() {
    let mut buffer = Buffer::new(50, 20);
    let area = Rect::new(0, 0, 50, 20);
    let mut ctx = revue::widget::traits::RenderContext::new(&mut buffer, area);

    let mut c = notification_center();
    c.push(Notification::new("Test message").action("Retry"));
    c.render(&mut ctx);
}

#[test]
fn test_center_render_without_icons() {
    let mut buffer = Buffer::new(50, 20);
    let area = Rect::new(0, 0, 50, 20);
    let mut ctx = revue::widget::traits::RenderContext::new(&mut buffer, area);

    let mut c = notification_center().show_icons(false);
    c.info("Test");
    c.render(&mut ctx);
}

#[test]
fn test_center_render_without_timer() {
    let mut buffer = Buffer::new(50, 20);
    let area = Rect::new(0, 0, 50, 20);
    let mut ctx = revue::widget::traits::RenderContext::new(&mut buffer, area);

    let mut c = notification_center().show_timer(false);
    c.info("Test");
    c.render(&mut ctx);
}

#[test]
fn test_center_render_max_visible() {
    let mut buffer = Buffer::new(50, 20);
    let area = Rect::new(0, 0, 50, 20);
    let mut ctx = revue::widget::traits::RenderContext::new(&mut buffer, area);

    let mut c = notification_center().max_visible(2);
    c.info("1");
    c.info("2");
    c.info("3");
    c.render(&mut ctx);
    // Only 2 should be visible
}

#[test]
fn test_center_render_selected() {
    let mut buffer = Buffer::new(50, 20);
    let area = Rect::new(0, 0, 50, 20);
    let mut ctx = revue::widget::traits::RenderContext::new(&mut buffer, area);

    let mut c = notification_center();
    c.info("1");
    c.info("2");
    c.selected = Some(0);
    c.render(&mut ctx);
    // First notification should render as selected
}

// =========================================================================
// NotificationCenter dismissal tests
// =========================================================================

#[test]
fn test_center_dismiss_adjusts_selection() {
    let mut c = notification_center();
    c.info("1");
    c.info("2");
    c.info("3");
    c.selected = Some(2);

    // Dismiss the selected one
    let id = c.notifications[2].id;
    c.dismiss(id);

    // Selection should be adjusted
    assert_eq!(c.selected, Some(1));
}

#[test]
fn test_center_dismiss_clears_selection_if_empty() {
    let mut c = notification_center();
    c.info("Test");
    c.selected = Some(0);

    let id = c.notifications[0].id;
    c.dismiss(id);

    assert!(c.is_empty());
    assert_eq!(c.selected, None);
}

#[test]
fn test_center_tick_adjusts_selection() {
    let mut c = notification_center();
    c.push(Notification::new("Test").duration(1));
    c.selected = Some(0);

    c.tick(); // Notification expires

    assert!(c.is_empty());
    assert_eq!(c.selected, None);
}

#[test]
fn test_center_push_sets_created_at() {
    let mut c = notification_center();
    assert_eq!(c.tick_counter, 0);

    c.info("Test 1");
    assert_eq!(c.notifications[0].created_at, 0);

    c.tick();
    assert_eq!(c.tick_counter, 1);

    c.info("Test 2");
    assert_eq!(c.notifications[1].created_at, 1);
}
