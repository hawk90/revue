//! Notification types tests
//!
//! Tests for Notification and related types extracted from source files.

use revue::style::Color;
use revue::widget::NotificationLevel;
use revue::widget::{Notification, NotificationPosition};

// =========================================================================
// NotificationLevel enum tests
// =========================================================================

#[test]
fn test_notification_level_default() {
    assert_eq!(NotificationLevel::default(), NotificationLevel::Info);
}

#[test]
fn test_notification_level_clone() {
    let level = NotificationLevel::Warning;
    let cloned = level.clone();
    assert_eq!(level, cloned);
}

#[test]
fn test_notification_level_copy() {
    let level1 = NotificationLevel::Error;
    let level2 = level1;
    assert_eq!(level1, NotificationLevel::Error);
    assert_eq!(level2, NotificationLevel::Error);
}

#[test]
fn test_notification_level_partial_eq() {
    assert_eq!(NotificationLevel::Info, NotificationLevel::Info);
    assert_eq!(NotificationLevel::Success, NotificationLevel::Success);
    assert_eq!(NotificationLevel::Warning, NotificationLevel::Warning);
    assert_eq!(NotificationLevel::Error, NotificationLevel::Error);
    assert_eq!(NotificationLevel::Debug, NotificationLevel::Debug);

    assert_ne!(NotificationLevel::Info, NotificationLevel::Success);
    assert_ne!(NotificationLevel::Warning, NotificationLevel::Error);
}

#[test]
fn test_notification_level_all_variants_distinct() {
    let levels = [
        NotificationLevel::Info,
        NotificationLevel::Success,
        NotificationLevel::Warning,
        NotificationLevel::Error,
        NotificationLevel::Debug,
    ];

    for (i, level1) in levels.iter().enumerate() {
        for (j, level2) in levels.iter().enumerate() {
            if i == j {
                assert_eq!(level1, level2);
            } else {
                assert_ne!(level1, level2);
            }
        }
    }
}

#[test]
fn test_notification_level_icon() {
    assert_eq!(NotificationLevel::Info.icon(), 'ℹ');
    assert_eq!(NotificationLevel::Success.icon(), '✓');
    assert_eq!(NotificationLevel::Warning.icon(), '⚠');
    assert_eq!(NotificationLevel::Error.icon(), '✗');
    assert_eq!(NotificationLevel::Debug.icon(), '⚙');
}

#[test]
fn test_notification_level_color() {
    let info_color = NotificationLevel::Info.color();
    let success_color = NotificationLevel::Success.color();
    let warning_color = NotificationLevel::Warning.color();
    let error_color = NotificationLevel::Error.color();
    let debug_color = NotificationLevel::Debug.color();

    assert_eq!(info_color, Color::CYAN);
    assert_eq!(success_color, Color::GREEN);
    assert_eq!(warning_color, Color::YELLOW);
    assert_eq!(error_color, Color::RED);
    assert_eq!(debug_color, Color::MAGENTA);
}

#[test]
fn test_notification_level_bg_color() {
    assert_eq!(NotificationLevel::Info.bg_color(), Color::rgb(20, 50, 60));
    assert_eq!(
        NotificationLevel::Success.bg_color(),
        Color::rgb(20, 50, 30)
    );
    assert_eq!(
        NotificationLevel::Warning.bg_color(),
        Color::rgb(60, 50, 20)
    );
    assert_eq!(NotificationLevel::Error.bg_color(), Color::rgb(60, 20, 20));
    assert_eq!(NotificationLevel::Debug.bg_color(), Color::rgb(40, 20, 50));
}

// =========================================================================
// NotificationPosition enum tests
// =========================================================================

#[test]
fn test_notification_position_default() {
    assert_eq!(
        NotificationPosition::default(),
        NotificationPosition::TopRight
    );
}

#[test]
fn test_notification_position_clone() {
    let pos = NotificationPosition::TopLeft;
    let cloned = pos.clone();
    assert_eq!(pos, cloned);
}

#[test]
fn test_notification_position_copy() {
    let pos1 = NotificationPosition::BottomCenter;
    let pos2 = pos1;
    assert_eq!(pos1, NotificationPosition::BottomCenter);
    assert_eq!(pos2, NotificationPosition::BottomCenter);
}

#[test]
fn test_notification_position_partial_eq() {
    assert_eq!(
        NotificationPosition::TopRight,
        NotificationPosition::TopRight
    );
    assert_eq!(NotificationPosition::TopLeft, NotificationPosition::TopLeft);
    assert_eq!(
        NotificationPosition::TopCenter,
        NotificationPosition::TopCenter
    );
    assert_eq!(
        NotificationPosition::BottomRight,
        NotificationPosition::BottomRight
    );
    assert_eq!(
        NotificationPosition::BottomLeft,
        NotificationPosition::BottomLeft
    );
    assert_eq!(
        NotificationPosition::BottomCenter,
        NotificationPosition::BottomCenter
    );

    assert_ne!(
        NotificationPosition::TopRight,
        NotificationPosition::TopLeft
    );
    assert_ne!(
        NotificationPosition::BottomLeft,
        NotificationPosition::BottomCenter
    );
}

#[test]
fn test_notification_position_all_variants_distinct() {
    let positions = [
        NotificationPosition::TopRight,
        NotificationPosition::TopLeft,
        NotificationPosition::TopCenter,
        NotificationPosition::BottomRight,
        NotificationPosition::BottomLeft,
        NotificationPosition::BottomCenter,
    ];

    for (i, pos1) in positions.iter().enumerate() {
        for (j, pos2) in positions.iter().enumerate() {
            if i == j {
                assert_eq!(pos1, pos2);
            } else {
                assert_ne!(pos1, pos2);
            }
        }
    }
}

// =========================================================================
// Notification struct tests
// =========================================================================

#[test]
fn test_notification_new_default_values() {
    let n = Notification::new("Test message");
    assert!(n.title.is_none());
    assert!(matches!(n.level, NotificationLevel::Info));
    assert_eq!(n.duration, 100);
    assert_eq!(n.tick, 0);
    assert!(n.dismissible);
    assert!(n.progress.is_none());
    assert!(n.action.is_none());
    assert_eq!(n.created_at, 0);
}

#[test]
fn test_notification_title_builder() {
    let n = Notification::new("Message").title("Custom Title");
    assert_eq!(n.title, Some("Custom Title".to_string()));
}

#[test]
fn test_notification_title_with_string() {
    let n = Notification::new("Message").title("Title".to_string());
    assert_eq!(n.title, Some("Title".to_string()));
}

#[test]
fn test_notification_level_builder() {
    let n = Notification::new("Message").level(NotificationLevel::Error);
    assert!(matches!(n.level, NotificationLevel::Error));
}

#[test]
fn test_notification_duration_builder() {
    let n = Notification::new("Message").duration(200);
    assert_eq!(n.duration, 200);
}

#[test]
fn test_notification_duration_zero() {
    let n = Notification::new("Message").duration(0);
    assert_eq!(n.duration, 0);
    assert!(!n.is_expired()); // Duration 0 means permanent
}

#[test]
fn test_notification_dismissible_builder() {
    let n = Notification::new("Message").dismissible(false);
    assert!(!n.dismissible);
}

#[test]
fn test_notification_progress_builder() {
    let n = Notification::new("Message").progress(0.5);
    assert_eq!(n.progress, Some(0.5));
}

#[test]
fn test_notification_progress_clamping_above() {
    let n = Notification::new("Message").progress(1.5);
    assert_eq!(n.progress, Some(1.0)); // Clamped to 1.0
}

#[test]
fn test_notification_progress_clamping_below() {
    let n = Notification::new("Message").progress(-0.5);
    assert_eq!(n.progress, Some(0.0)); // Clamped to 0.0
}

#[test]
fn test_notification_progress_boundary_values() {
    let n1 = Notification::new("Message").progress(0.0);
    assert_eq!(n1.progress, Some(0.0));

    let n2 = Notification::new("Message").progress(1.0);
    assert_eq!(n2.progress, Some(1.0));
}

#[test]
fn test_notification_action_builder() {
    let n = Notification::new("Message").action("Click here");
    assert_eq!(n.action, Some("Click here".to_string()));
}

#[test]
fn test_notification_info_shortcut() {
    let n = Notification::info("Info message");
    assert!(matches!(n.level, NotificationLevel::Info));
    assert_eq!(n.message, "Info message");
}

#[test]
fn test_notification_success_shortcut() {
    let n = Notification::success("Success message");
    assert!(matches!(n.level, NotificationLevel::Success));
    assert_eq!(n.message, "Success message");
}

#[test]
fn test_notification_warning_shortcut() {
    let n = Notification::warning("Warning message");
    assert!(matches!(n.level, NotificationLevel::Warning));
    assert_eq!(n.message, "Warning message");
}

#[test]
fn test_notification_error_shortcut() {
    let n = Notification::error("Error message");
    assert!(matches!(n.level, NotificationLevel::Error));
    assert_eq!(n.message, "Error message");
}

#[test]
fn test_notification_debug_shortcut() {
    let n = Notification::debug("Debug message");
    assert!(matches!(n.level, NotificationLevel::Debug));
    assert_eq!(n.message, "Debug message");
}

#[test]
fn test_notification_is_expired_with_duration() {
    let mut n = Notification::new("Message").duration(10);
    assert!(!n.is_expired());

    n.tick = 5;
    assert!(!n.is_expired());

    n.tick = 10;
    assert!(n.is_expired());

    n.tick = 15;
    assert!(n.is_expired());
}

#[test]
fn test_notification_is_expired_permanent() {
    let mut n = Notification::new("Message").duration(0);

    n.tick = 0;
    assert!(!n.is_expired());

    n.tick = 100;
    assert!(!n.is_expired());

    n.tick = 1000;
    assert!(!n.is_expired());
}

#[test]
fn test_notification_remaining_with_duration() {
    let mut n = Notification::new("Message").duration(100);

    n.tick = 0;
    assert_eq!(n.remaining(), 1.0);

    n.tick = 25;
    assert_eq!(n.remaining(), 0.75);

    n.tick = 50;
    assert_eq!(n.remaining(), 0.5);

    n.tick = 75;
    assert_eq!(n.remaining(), 0.25);

    n.tick = 100;
    assert_eq!(n.remaining(), 0.0);
}

#[test]
fn test_notification_remaining_permanent() {
    let mut n = Notification::new("Message").duration(0);

    n.tick = 0;
    assert_eq!(n.remaining(), 1.0);

    n.tick = 100;
    assert_eq!(n.remaining(), 1.0);

    n.tick = 1000;
    assert_eq!(n.remaining(), 1.0);
}

#[test]
fn test_notification_id_unique() {
    let n1 = Notification::new("Message 1");
    let n2 = Notification::new("Message 2");

    // IDs should be different (AtomicU64 increments)
    assert_ne!(n1.id, n2.id);
}

#[test]
fn test_notification_clone() {
    let n1 = Notification::new("Test")
        .title("Title")
        .level(NotificationLevel::Warning)
        .duration(50)
        .dismissible(false)
        .progress(0.75)
        .action("Action");

    let n2 = n1.clone();

    assert_eq!(n1.id, n2.id);
    assert_eq!(n1.title, n2.title);
    assert_eq!(n1.message, n2.message);
    assert_eq!(n1.level, n2.level);
    assert_eq!(n1.duration, n2.duration);
    assert_eq!(n1.dismissible, n2.dismissible);
    assert_eq!(n1.progress, n2.progress);
    assert_eq!(n1.action, n2.action);
}

#[test]
fn test_notification_builder_chain() {
    let n = Notification::new("Chain test")
        .title("Chain Title")
        .level(NotificationLevel::Success)
        .duration(200)
        .dismissible(false)
        .progress(0.8)
        .action("Click me");

    assert_eq!(n.message, "Chain test");
    assert_eq!(n.title, Some("Chain Title".to_string()));
    assert!(matches!(n.level, NotificationLevel::Success));
    assert_eq!(n.duration, 200);
    assert!(!n.dismissible);
    assert_eq!(n.progress, Some(0.8));
    assert_eq!(n.action, Some("Click me".to_string()));
}

#[test]
fn test_notification_empty_message() {
    let n = Notification::new("");
    assert_eq!(n.message, "");
}

#[test]
fn test_notification_long_message() {
    let long_msg = "A".repeat(1000);
    let n = Notification::new(long_msg.clone());
    assert_eq!(n.message, long_msg);
}

#[test]
fn test_notification_unicode_message() {
    let n = Notification::new("🎉 Test message 你好 🎊");
    assert_eq!(n.message, "🎉 Test message 你好 🎊");
}

#[test]
fn test_notification_multiline_message() {
    let n = Notification::new("Line 1\nLine 2\nLine 3");
    assert_eq!(n.message, "Line 1\nLine 2\nLine 3");
}
