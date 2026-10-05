//! Tests for ToastQueue widget
//!
//! Extracted from src/widget/feedback/toast_queue.rs

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::{
    toast_queue, RenderContext, StackDirection, ToastEntry, ToastLevel, ToastPosition,
    ToastPriority, ToastQueue, View,
};
use std::thread::sleep;
use std::time::Duration;

fn messages(entries: &[ToastEntry]) -> Vec<&str> {
    entries.iter().map(|e| e.message.as_str()).collect()
}

fn render(queue: &ToastQueue, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, height));
    queue.render(&mut ctx);
    buffer
}

fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).map(|c| c.symbol).unwrap_or(' '))
        .filter(|&c| c != '\0')
        .collect()
}

/// (x, y) of every toast's top-left corner, top to bottom.
fn corners(buffer: &Buffer) -> Vec<(u16, u16)> {
    let mut found = Vec::new();
    for y in 0..buffer.height() {
        for x in 0..buffer.width() {
            if buffer.get(x, y).map(|c| c.symbol) == Some('╭') {
                found.push((x, y));
            }
        }
    }
    found
}

// =========================================================================
// Enums
// =========================================================================

#[test]
fn test_enum_defaults() {
    assert_eq!(StackDirection::default(), StackDirection::Down);
    assert_eq!(ToastPriority::default(), ToastPriority::Normal);
    assert_ne!(StackDirection::Down, StackDirection::Up);
}

#[test]
fn test_toast_priority_ordering() {
    let mut priorities = vec![
        ToastPriority::Normal,
        ToastPriority::Low,
        ToastPriority::Critical,
        ToastPriority::High,
    ];
    priorities.sort();
    assert_eq!(
        priorities,
        vec![
            ToastPriority::Low,
            ToastPriority::Normal,
            ToastPriority::High,
            ToastPriority::Critical,
        ]
    );
}

// =========================================================================
// ToastEntry
// =========================================================================

#[test]
fn test_toast_entry_defaults() {
    let entry = ToastEntry::new(String::from("Owned message"), ToastLevel::Success);
    assert_eq!(entry.message, "Owned message");
    assert_eq!(entry.level, ToastLevel::Success);
    assert!(entry.id.is_none());
    assert_eq!(entry.priority, ToastPriority::Normal);
    assert!(entry.duration.is_none());
    assert!(entry.shown_at.is_none());
    assert!(entry.dismissible);
}

#[test]
fn test_toast_entry_builder() {
    let entry = ToastEntry::new("Chain test", ToastLevel::Warning)
        .with_id(String::from("chain-id"))
        .with_priority(ToastPriority::Critical)
        .with_duration(Duration::from_millis(500))
        .dismissible(false);
    assert_eq!(entry.message, "Chain test");
    assert_eq!(entry.id.as_deref(), Some("chain-id"));
    assert_eq!(entry.priority, ToastPriority::Critical);
    assert_eq!(entry.duration, Some(Duration::from_millis(500)));
    assert!(!entry.dismissible);
    assert!(entry.dismissible(true).dismissible);
}

// =========================================================================
// ToastQueue configuration
// =========================================================================

#[test]
fn test_toast_queue_new_defaults() {
    let queue = ToastQueue::new();
    assert!(queue.is_empty());
    assert_eq!(queue.total_count(), 0);
    assert_eq!(queue.get_position(), ToastPosition::TopRight);
    assert_eq!(queue.get_stack_direction(), StackDirection::Down);
    assert_eq!(queue.get_max_visible(), 5);
    assert_eq!(queue.get_default_duration(), Duration::from_secs(4));
    assert_eq!(queue.get_gap(), 1);
    assert_eq!(queue.get_toast_width(), 40);
    assert!(queue.get_deduplicate());
    assert!(!queue.is_paused());
}

#[test]
fn test_toast_queue_helper_and_default() {
    for queue in [toast_queue(), ToastQueue::default()] {
        assert!(queue.is_empty());
        assert_eq!(queue.get_max_visible(), 5);
    }
}

#[test]
fn test_toast_queue_builder_chain() {
    let queue = ToastQueue::new()
        .position(ToastPosition::TopLeft)
        .stack_direction(StackDirection::Up)
        .max_visible(3)
        .default_duration(Duration::from_secs(3))
        .gap(2)
        .toast_width(50)
        .deduplicate(false);
    assert_eq!(queue.get_position(), ToastPosition::TopLeft);
    assert_eq!(queue.get_stack_direction(), StackDirection::Up);
    assert_eq!(queue.get_max_visible(), 3);
    assert_eq!(queue.get_default_duration(), Duration::from_secs(3));
    assert_eq!(queue.get_gap(), 2);
    assert_eq!(queue.get_toast_width(), 50);
    assert!(!queue.get_deduplicate());
}

// =========================================================================
// Queueing
// =========================================================================

#[test]
fn test_toast_queue_push_and_tick() {
    let mut queue = ToastQueue::new();
    queue.push("Test message", ToastLevel::Info);
    assert_eq!(queue.pending_count(), 1);
    assert_eq!(queue.visible_count(), 0);
    assert!(!queue.is_empty());

    queue.tick();
    assert_eq!(queue.visible_count(), 1);
    assert_eq!(queue.pending_count(), 0);
    assert!(queue.get_visible()[0].shown_at.is_some());
}

#[test]
fn test_toast_queue_level_helpers() {
    let mut queue = ToastQueue::new();
    queue.info("Info");
    queue.success("Success");
    queue.warning("Warning");
    queue.error("Error");
    let levels: Vec<ToastLevel> = queue.get_queue().iter().map(|e| e.level).collect();
    assert_eq!(
        levels,
        [
            ToastLevel::Info,
            ToastLevel::Success,
            ToastLevel::Warning,
            ToastLevel::Error
        ]
    );
    assert_eq!(
        messages(queue.get_queue()),
        ["Info", "Success", "Warning", "Error"]
    );
}

#[test]
fn test_toast_queue_max_visible() {
    let mut queue = ToastQueue::new().max_visible(2);
    for i in 0..5 {
        queue.push(format!("Toast {}", i), ToastLevel::Info);
    }
    queue.tick();
    assert_eq!(messages(queue.get_visible()), ["Toast 0", "Toast 1"]);
    assert_eq!(queue.pending_count(), 3);
    assert_eq!(queue.total_count(), 5);

    // Dismissing one makes room for the next on the following tick.
    queue.dismiss_first();
    queue.tick();
    assert_eq!(messages(queue.get_visible()), ["Toast 1", "Toast 2"]);
}

#[test]
fn test_toast_queue_max_visible_zero() {
    let mut queue = ToastQueue::new().max_visible(0);
    queue.push("Test", ToastLevel::Info);
    queue.tick();
    assert_eq!(queue.visible_count(), 0);
    assert_eq!(queue.pending_count(), 1);
}

#[test]
fn test_toast_queue_deduplication() {
    let mut queue = ToastQueue::new();
    queue.push_with_id("test-1", "Message 1", ToastLevel::Info);
    queue.push_with_id("test-1", "Message 1 duplicate", ToastLevel::Info);
    assert_eq!(messages(queue.get_queue()), ["Message 1"]);

    // A visible toast also blocks its id.
    queue.tick();
    queue.push_with_id("test-1", "Again", ToastLevel::Info);
    assert_eq!(queue.total_count(), 1);

    // Toasts without an id are never deduplicated.
    queue.push("Same", ToastLevel::Info);
    queue.push("Same", ToastLevel::Info);
    assert_eq!(queue.pending_count(), 2);
}

#[test]
fn test_toast_queue_deduplication_disabled() {
    let mut queue = ToastQueue::new().deduplicate(false);
    queue.push_with_id("test-1", "Message 1", ToastLevel::Info);
    queue.push_with_id("test-1", "Message 1 duplicate", ToastLevel::Info);
    assert_eq!(queue.pending_count(), 2);
}

#[test]
fn test_toast_queue_priority_ordering() {
    let mut queue = ToastQueue::new();
    for (message, priority) in [
        ("Low", ToastPriority::Low),
        ("Critical", ToastPriority::Critical),
        ("Normal 1", ToastPriority::Normal),
        ("High", ToastPriority::High),
        ("Normal 2", ToastPriority::Normal),
    ] {
        queue.push_entry(ToastEntry::new(message, ToastLevel::Info).with_priority(priority));
    }
    // Highest first; equal priorities keep their order.
    assert_eq!(
        messages(queue.get_queue()),
        ["Critical", "High", "Normal 1", "Normal 2", "Low"]
    );
}

// =========================================================================
// Dismissing
// =========================================================================

#[test]
fn test_toast_queue_dismiss_by_id() {
    let mut queue = ToastQueue::new().max_visible(1);
    queue.push_with_id("shown", "Shown", ToastLevel::Info);
    queue.push_with_id("pending", "Pending", ToastLevel::Info);
    queue.tick();

    queue.dismiss("nonexistent-id");
    assert_eq!(queue.total_count(), 2);

    queue.dismiss("pending");
    assert_eq!(queue.pending_count(), 0);
    queue.dismiss("shown");
    assert!(queue.is_empty());
}

#[test]
fn test_toast_queue_dismiss_first() {
    let mut queue = ToastQueue::new();
    queue.dismiss_first();
    assert_eq!(queue.visible_count(), 0);

    queue.push("Toast 1", ToastLevel::Info);
    queue.push("Toast 2", ToastLevel::Info);
    queue.tick();
    queue.dismiss_first();
    assert_eq!(messages(queue.get_visible()), ["Toast 2"]);
}

#[test]
fn test_toast_queue_dismiss_first_non_dismissible() {
    let mut queue = ToastQueue::new();
    queue.push_entry(ToastEntry::new("Sticky", ToastLevel::Info).dismissible(false));
    queue.push("Other", ToastLevel::Info);
    queue.tick();
    queue.dismiss_first();
    assert_eq!(messages(queue.get_visible()), ["Sticky", "Other"]);
}

#[test]
fn test_toast_queue_dismiss_all_and_clear() {
    let mut queue = ToastQueue::new().max_visible(2);
    queue.push_entry(ToastEntry::new("Sticky", ToastLevel::Error).dismissible(false));
    queue.push("Dismissible", ToastLevel::Info);
    queue.push("Pending", ToastLevel::Info);
    queue.tick();

    queue.dismiss_all();
    assert_eq!(messages(queue.get_visible()), ["Sticky"]);
    assert_eq!(queue.pending_count(), 0);

    queue.clear();
    assert!(queue.is_empty());
}

// =========================================================================
// Expiry and pausing
// =========================================================================

#[test]
fn test_toast_queue_expiry() {
    let mut queue = ToastQueue::new().default_duration(Duration::from_millis(50));
    queue.push("Short lived", ToastLevel::Info);
    queue.push_entry(
        ToastEntry::new("Long lived", ToastLevel::Info).with_duration(Duration::from_secs(60)),
    );
    queue.tick();
    assert_eq!(queue.visible_count(), 2);

    sleep(Duration::from_millis(60));
    queue.tick();
    assert_eq!(messages(queue.get_visible()), ["Long lived"]);
}

#[test]
fn test_toast_queue_pending_toasts_do_not_expire() {
    let mut queue = ToastQueue::new()
        .max_visible(0)
        .default_duration(Duration::from_millis(1));
    queue.push("Waiting", ToastLevel::Info);
    sleep(Duration::from_millis(5));
    queue.tick();
    assert_eq!(queue.pending_count(), 1);
}

#[test]
fn test_toast_queue_pause_needs_pause_on_hover() {
    let mut queue = ToastQueue::new();
    queue.pause();
    assert!(!queue.is_paused());

    let mut queue = ToastQueue::new().pause_on_hover(true);
    queue.pause();
    assert!(queue.is_paused());
    queue.resume();
    assert!(!queue.is_paused());
}

#[test]
fn test_toast_queue_paused_toasts_do_not_expire() {
    let mut queue = ToastQueue::new()
        .pause_on_hover(true)
        .default_duration(Duration::from_millis(20));
    queue.push("Hovered", ToastLevel::Info);
    queue.tick();
    queue.pause();
    sleep(Duration::from_millis(30));
    queue.tick();
    assert_eq!(queue.visible_count(), 1);

    queue.resume();
    queue.tick();
    assert_eq!(queue.visible_count(), 0);
}

// =========================================================================
// Rendering
// =========================================================================

#[test]
fn test_toast_queue_render_empty_draws_nothing() {
    let buffer = render(&ToastQueue::new(), 50, 20);
    assert!((0..20).all(|y| row(&buffer, y).trim().is_empty()));
}

#[test]
fn test_toast_queue_render_toast() {
    let mut queue = ToastQueue::new().position(ToastPosition::TopLeft);
    queue.push("Test toast", ToastLevel::Success);
    queue.tick();
    let buffer = render(&queue, 50, 20);
    assert_eq!(corners(&buffer), [(1, 1)]);
    let content = row(&buffer, 2);
    assert!(content.contains("│ ✓ Test toast"), "{content:?}");
    // Dismissible toasts show a close mark.
    assert!(content.contains('×'), "{content:?}");
}

#[test]
fn test_toast_queue_render_positions() {
    // 40 wide, 3 tall, 1 cell margin.
    let cases = [
        (ToastPosition::TopLeft, (1, 1)),
        (ToastPosition::TopCenter, (5, 1)),
        (ToastPosition::TopRight, (9, 1)),
        (ToastPosition::BottomLeft, (1, 15)),
        (ToastPosition::BottomCenter, (5, 15)),
        (ToastPosition::BottomRight, (9, 15)),
    ];
    for (position, corner) in cases {
        let mut queue = ToastQueue::new().position(position);
        queue.push("Test", ToastLevel::Info);
        queue.tick();
        assert_eq!(corners(&render(&queue, 50, 20)), [corner], "{position:?}");
    }
}

#[test]
fn test_toast_queue_render_stacks_down_with_gap() {
    let mut queue = ToastQueue::new().position(ToastPosition::TopLeft).gap(2);
    queue.push("Toast 1", ToastLevel::Info);
    queue.push("Toast 2", ToastLevel::Info);
    queue.tick();
    let buffer = render(&queue, 50, 20);
    assert_eq!(corners(&buffer), [(1, 1), (1, 6)]);
    assert!(row(&buffer, 2).contains("Toast 1"));
    assert!(row(&buffer, 7).contains("Toast 2"));
}

#[test]
fn test_toast_queue_render_non_dismissible_has_no_close_mark() {
    let mut queue = ToastQueue::new();
    queue.push_entry(ToastEntry::new("Sticky", ToastLevel::Info).dismissible(false));
    queue.tick();
    let buffer = render(&queue, 50, 20);
    assert!(!row(&buffer, 2).contains('×'));
}
