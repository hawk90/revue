//! Notification, NotificationLevel and NotificationPosition
//!
//! A level's icon and colors are private; they are read off a rendered
//! NotificationCenter.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::{
    notification_center, Notification, NotificationLevel, NotificationPosition, RenderContext, View,
};

/// Render one notification with the given level at the top left and
/// return the cells of its border corner, icon and message.
fn level_cells(level: NotificationLevel) -> (Color, char, Option<Color>, Option<Color>) {
    let mut center = notification_center().position(NotificationPosition::TopLeft);
    center.push(Notification::new("msg").level(level));
    let mut buffer = Buffer::new(50, 10);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 50, 10));
    center.render(&mut ctx);
    let corner = buffer.get(0, 0).unwrap();
    let icon = buffer.get(1, 1).unwrap();
    (corner.fg.unwrap(), icon.symbol, icon.fg, icon.bg)
}

// =========================================================================
// NotificationLevel
// =========================================================================

#[test]
fn test_notification_level_default_and_variants() {
    assert_eq!(NotificationLevel::default(), NotificationLevel::Info);
    let levels = [
        NotificationLevel::Info,
        NotificationLevel::Success,
        NotificationLevel::Warning,
        NotificationLevel::Error,
        NotificationLevel::Debug,
    ];
    for (i, a) in levels.iter().enumerate() {
        for (j, b) in levels.iter().enumerate() {
            assert_eq!(i == j, a == b);
        }
    }
}

#[test]
fn test_notification_level_icon_and_colors() {
    let expected = [
        (
            NotificationLevel::Info,
            'ℹ',
            Color::CYAN,
            Color::rgb(20, 50, 60),
        ),
        (
            NotificationLevel::Success,
            '✓',
            Color::GREEN,
            Color::rgb(20, 50, 30),
        ),
        (
            NotificationLevel::Warning,
            '⚠',
            Color::YELLOW,
            Color::rgb(60, 50, 20),
        ),
        (
            NotificationLevel::Error,
            '✗',
            Color::RED,
            Color::rgb(60, 20, 20),
        ),
        (
            NotificationLevel::Debug,
            '⚙',
            Color::MAGENTA,
            Color::rgb(40, 20, 50),
        ),
    ];
    for (level, icon, color, bg) in expected {
        let (border_fg, icon_symbol, icon_fg, icon_bg) = level_cells(level);
        assert_eq!(icon_symbol, icon, "{level:?}");
        assert_eq!(border_fg, color, "{level:?}");
        assert_eq!(icon_fg, Some(color), "{level:?}");
        assert_eq!(icon_bg, Some(bg), "{level:?}");
    }
}

// =========================================================================
// NotificationPosition
// =========================================================================

#[test]
fn test_notification_position_default_and_variants() {
    assert_eq!(
        NotificationPosition::default(),
        NotificationPosition::TopRight
    );
    let positions = [
        NotificationPosition::TopRight,
        NotificationPosition::TopLeft,
        NotificationPosition::TopCenter,
        NotificationPosition::BottomRight,
        NotificationPosition::BottomLeft,
        NotificationPosition::BottomCenter,
    ];
    for (i, a) in positions.iter().enumerate() {
        for (j, b) in positions.iter().enumerate() {
            assert_eq!(i == j, a == b);
        }
    }
}

// =========================================================================
// Notification
// =========================================================================

#[test]
fn test_notification_new_default_values() {
    let n = Notification::new("Test message");
    assert_eq!(n.message, "Test message");
    assert!(n.title.is_none());
    assert_eq!(n.level, NotificationLevel::Info);
    assert_eq!(n.duration, 100);
    assert_eq!(n.tick, 0);
    assert!(n.dismissible);
    assert!(n.progress.is_none());
    assert!(n.action.is_none());
    assert_eq!(n.created_at, 0);
}

#[test]
fn test_notification_message_kept_verbatim() {
    for msg in [
        String::new(),
        "A".repeat(1000),
        "🎉 Test message 你好 🎊".to_string(),
        "Line 1\nLine 2\nLine 3".to_string(),
    ] {
        assert_eq!(Notification::new(msg.clone()).message, msg);
    }
}

#[test]
fn test_notification_builder_chain() {
    let n = Notification::new("Chain test")
        .title("Chain Title".to_string())
        .level(NotificationLevel::Success)
        .duration(200)
        .dismissible(false)
        .progress(0.8)
        .action("Click me");
    assert_eq!(n.message, "Chain test");
    assert_eq!(n.title.as_deref(), Some("Chain Title"));
    assert_eq!(n.level, NotificationLevel::Success);
    assert_eq!(n.duration, 200);
    assert!(!n.dismissible);
    assert_eq!(n.progress, Some(0.8));
    assert_eq!(n.action.as_deref(), Some("Click me"));
}

#[test]
fn test_notification_progress_is_clamped() {
    assert_eq!(Notification::new("m").progress(1.5).progress, Some(1.0));
    assert_eq!(Notification::new("m").progress(-0.5).progress, Some(0.0));
    assert_eq!(Notification::new("m").progress(0.0).progress, Some(0.0));
    assert_eq!(Notification::new("m").progress(1.0).progress, Some(1.0));
}

#[test]
fn test_notification_level_shortcuts() {
    let cases = [
        (Notification::info("m"), NotificationLevel::Info),
        (Notification::success("m"), NotificationLevel::Success),
        (Notification::warning("m"), NotificationLevel::Warning),
        (Notification::error("m"), NotificationLevel::Error),
        (Notification::debug("m"), NotificationLevel::Debug),
    ];
    for (n, level) in cases {
        assert_eq!(n.level, level);
        assert_eq!(n.message, "m");
    }
}

#[test]
fn test_notification_is_expired_with_duration() {
    let mut n = Notification::new("Message").duration(10);
    assert!(!n.is_expired());
    n.tick = 9;
    assert!(!n.is_expired());
    n.tick = 10;
    assert!(n.is_expired());
    n.tick = 15;
    assert!(n.is_expired());
}

#[test]
fn test_notification_zero_duration_never_expires() {
    let mut n = Notification::new("Message").duration(0);
    for tick in [0, 100, 1000] {
        n.tick = tick;
        assert!(!n.is_expired());
        assert_eq!(n.remaining(), 1.0);
    }
}

#[test]
fn test_notification_remaining_with_duration() {
    let mut n = Notification::new("Message").duration(100);
    for (tick, remaining) in [(0, 1.0), (25, 0.75), (50, 0.5), (75, 0.25), (100, 0.0)] {
        n.tick = tick;
        assert_eq!(n.remaining(), remaining, "tick {tick}");
    }
}

#[test]
fn test_notification_ids_are_unique() {
    let ids: Vec<u64> = (0..10).map(|_| Notification::new("m").id).collect();
    for (i, a) in ids.iter().enumerate() {
        for b in &ids[i + 1..] {
            assert_ne!(a, b);
        }
    }
}
