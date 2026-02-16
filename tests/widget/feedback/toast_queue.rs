//! Tests for ToastQueue widget
//!
//! Extracted from src/widget/feedback/toast_queue.rs

use revue::prelude::*;
use revue::widget::toast_queue::{StackDirection, ToastEntry, ToastPriority, ToastQueue};
use std::thread::sleep;
use std::time::Duration;

// =========================================================================
// StackDirection enum tests
// =========================================================================

#[test]
fn test_stack_direction_default() {
    assert_eq!(StackDirection::default(), StackDirection::Down);
}

#[test]
fn test_stack_direction_clone() {
    let dir1 = StackDirection::Up;
    let dir2 = dir1.clone();
    assert_eq!(dir1, dir2);
}

#[test]
fn test_stack_direction_copy() {
    let dir1 = StackDirection::Down;
    let dir2 = dir1;
    assert_eq!(dir1, StackDirection::Down);
    assert_eq!(dir2, StackDirection::Down);
}

#[test]
fn test_stack_direction_partial_eq() {
    assert_eq!(StackDirection::Down, StackDirection::Down);
    assert_eq!(StackDirection::Up, StackDirection::Up);
    assert_ne!(StackDirection::Down, StackDirection::Up);
}

#[test]
fn test_stack_direction_all_variants() {
    let directions = [StackDirection::Down, StackDirection::Up];

    for (i, dir1) in directions.iter().enumerate() {
        for (j, dir2) in directions.iter().enumerate() {
            if i == j {
                assert_eq!(dir1, dir2);
            } else {
                assert_ne!(dir1, dir2);
            }
        }
    }
}

// =========================================================================
// ToastPriority enum tests
// =========================================================================

#[test]
fn test_toast_priority_default() {
    assert_eq!(ToastPriority::default(), ToastPriority::Normal);
}

#[test]
fn test_toast_priority_clone() {
    let prio1 = ToastPriority::High;
    let prio2 = prio1.clone();
    assert_eq!(prio1, prio2);
}

#[test]
fn test_toast_priority_copy() {
    let prio1 = ToastPriority::Critical;
    let prio2 = prio1;
    assert_eq!(prio1, ToastPriority::Critical);
    assert_eq!(prio2, ToastPriority::Critical);
}

#[test]
fn test_toast_priority_partial_eq() {
    assert_eq!(ToastPriority::Low, ToastPriority::Low);
    assert_eq!(ToastPriority::Normal, ToastPriority::Normal);
    assert_eq!(ToastPriority::High, ToastPriority::High);
    assert_eq!(ToastPriority::Critical, ToastPriority::Critical);

    assert_ne!(ToastPriority::Low, ToastPriority::Normal);
    assert_ne!(ToastPriority::High, ToastPriority::Critical);
}

#[test]
fn test_toast_priority_ord() {
    assert!(ToastPriority::Low < ToastPriority::Normal);
    assert!(ToastPriority::Normal < ToastPriority::High);
    assert!(ToastPriority::High < ToastPriority::Critical);
}

#[test]
fn test_toast_priority_all_variants() {
    let priorities = [
        ToastPriority::Low,
        ToastPriority::Normal,
        ToastPriority::High,
        ToastPriority::Critical,
    ];

    for (i, prio1) in priorities.iter().enumerate() {
        for (j, prio2) in priorities.iter().enumerate() {
            if i == j {
                assert_eq!(prio1, prio2);
            } else {
                assert_ne!(prio1, prio2);
            }
        }
    }
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
// ToastEntry builder tests
// =========================================================================

#[test]
fn test_toast_entry_builder() {
    let entry = ToastEntry::new("Test", ToastLevel::Info)
        .with_id("test-id")
        .with_priority(ToastPriority::High)
        .with_duration(Duration::from_secs(10))
        .dismissible(false);

    assert_eq!(entry.id, Some("test-id".to_string()));
    assert_eq!(entry.priority, ToastPriority::High);
    assert_eq!(entry.duration, Some(Duration::from_secs(10)));
    assert!(!entry.dismissible);
}

#[test]
fn test_toast_entry_new_with_string() {
    let msg = String::from("Owned message");
    let entry = ToastEntry::new(msg, ToastLevel::Success);
    assert_eq!(entry.message, "Owned message");
    assert_eq!(entry.level, ToastLevel::Success);
}

#[test]
fn test_toast_entry_empty_message() {
    let entry = ToastEntry::new("", ToastLevel::Info);
    assert_eq!(entry.message, "");
}

#[test]
fn test_toast_entry_with_id_string() {
    let id = String::from("test-id");
    let entry = ToastEntry::new("Message", ToastLevel::Info).with_id(id);
    assert_eq!(entry.id, Some("test-id".to_string()));
}

#[test]
fn test_toast_entry_default_values() {
    let entry = ToastEntry::new("Test", ToastLevel::Info);
    assert!(entry.id.is_none());
    assert_eq!(entry.priority, ToastPriority::Normal);
    assert!(entry.duration.is_none());
    assert!(entry.shown_at.is_none());
    assert!(entry.dismissible);
}

#[test]
fn test_toast_entry_dismissible_true() {
    let entry = ToastEntry::new("Test", ToastLevel::Info).dismissible(true);
    assert!(entry.dismissible);
}

#[test]
fn test_toast_entry_builder_chain() {
    let entry = ToastEntry::new("Chain test", ToastLevel::Warning)
        .with_id("chain-id")
        .with_priority(ToastPriority::Critical)
        .with_duration(Duration::from_millis(500))
        .dismissible(false);

    assert_eq!(entry.message, "Chain test");
    assert_eq!(entry.id, Some("chain-id".to_string()));
    assert_eq!(entry.priority, ToastPriority::Critical);
    assert_eq!(entry.duration, Some(Duration::from_millis(500)));
    assert!(!entry.dismissible);
}

#[test]
fn test_toast_entry_clone() {
    let entry1 = ToastEntry::new("Test message", ToastLevel::Info)
        .with_id("test-id")
        .with_priority(ToastPriority::High)
        .with_duration(Duration::from_secs(10))
        .dismissible(false);

    let entry2 = entry1.clone();

    assert_eq!(entry1.id, entry2.id);
    assert_eq!(entry1.message, entry2.message);
    assert_eq!(entry1.level, entry2.level);
    assert_eq!(entry1.priority, entry2.priority);
    assert_eq!(entry1.duration, entry2.duration);
    assert_eq!(entry1.dismissible, entry2.dismissible);
}

// =========================================================================
// ToastQueue basic tests
// =========================================================================

#[test]
fn test_toast_queue_new() {
    let queue = ToastQueue::new();
    assert!(queue.is_empty());
    assert_eq!(queue.get_max_visible(), 5);
}

#[test]
fn test_toast_queue_push() {
    let mut queue = ToastQueue::new();
    queue.push("Test message", ToastLevel::Info);
    assert_eq!(queue.pending_count(), 1);
}

#[test]
fn test_toast_queue_tick() {
    let mut queue = ToastQueue::new();
    queue.push("Test", ToastLevel::Info);
    queue.tick();
    assert_eq!(queue.visible_count(), 1);
    assert_eq!(queue.pending_count(), 0);
}

#[test]
fn test_toast_queue_max_visible() {
    let mut queue = ToastQueue::new().max_visible(2);
    for i in 0..5 {
        queue.push(format!("Toast {}", i), ToastLevel::Info);
    }
    queue.tick();
    assert_eq!(queue.visible_count(), 2);
    assert_eq!(queue.pending_count(), 3);
}

#[test]
fn test_toast_queue_deduplication() {
    let mut queue = ToastQueue::new();
    queue.push_with_id("test-1", "Message 1", ToastLevel::Info);
    queue.push_with_id("test-1", "Message 1 duplicate", ToastLevel::Info);
    assert_eq!(queue.pending_count(), 1);
}

#[test]
fn test_toast_queue_deduplication_disabled() {
    let mut queue = ToastQueue::new().deduplicate(false);
    queue.push_with_id("test-1", "Message 1", ToastLevel::Info);
    queue.push_with_id("test-1", "Message 1 duplicate", ToastLevel::Info);
    assert_eq!(queue.pending_count(), 2);
}

#[test]
fn test_toast_queue_priority() {
    let mut queue = ToastQueue::new();
    queue
        .push_entry(ToastEntry::new("Low", ToastLevel::Info).with_priority(ToastPriority::Low));
    queue.push_entry(
        ToastEntry::new("High", ToastLevel::Info).with_priority(ToastPriority::High),
    );
    queue.push_entry(
        ToastEntry::new("Normal", ToastLevel::Info).with_priority(ToastPriority::Normal),
    );

    // High priority should be first
    assert_eq!(queue.get_queue()[0].message, "High");
    assert_eq!(queue.get_queue()[1].message, "Normal");
    assert_eq!(queue.get_queue()[2].message, "Low");
}

#[test]
fn test_toast_queue_dismiss() {
    let mut queue = ToastQueue::new();
    queue.push_with_id("test-1", "Message", ToastLevel::Info);
    queue.tick();
    assert_eq!(queue.visible_count(), 1);

    queue.dismiss("test-1");
    assert_eq!(queue.visible_count(), 0);
}

#[test]
fn test_toast_queue_dismiss_all() {
    let mut queue = ToastQueue::new();
    queue.push("Toast 1", ToastLevel::Info);
    queue.push("Toast 2", ToastLevel::Info);
    queue.tick();
    assert_eq!(queue.visible_count(), 2);

    queue.dismiss_all();
    assert_eq!(queue.visible_count(), 0);
}

#[test]
fn test_toast_queue_clear() {
    let mut queue = ToastQueue::new();
    queue.push_entry(ToastEntry::new("Non-dismissible", ToastLevel::Error).dismissible(false));
    queue.push("Dismissible", ToastLevel::Info);
    queue.tick();

    queue.dismiss_all();
    // Non-dismissible should remain
    assert_eq!(queue.visible_count(), 1);

    queue.clear();
    assert_eq!(queue.visible_count(), 0);
}

#[test]
fn test_toast_queue_expiry() {
    let mut queue = ToastQueue::new().default_duration(Duration::from_millis(50));
    queue.push("Short lived", ToastLevel::Info);
    queue.tick();
    assert_eq!(queue.visible_count(), 1);

    sleep(Duration::from_millis(60));
    queue.tick();
    assert_eq!(queue.visible_count(), 0);
}

#[test]
fn test_toast_queue_helpers() {
    let mut queue = ToastQueue::new();
    queue.info("Info");
    queue.success("Success");
    queue.warning("Warning");
    queue.error("Error");
    assert_eq!(queue.pending_count(), 4);
}

#[test]
fn test_toast_queue_render() {
    let mut queue = ToastQueue::new();
    queue.push("Test toast", ToastLevel::Success);
    queue.tick();

    let mut buffer = Buffer::new(50, 20);
    let area = Rect::new(0, 0, 50, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    queue.render(&mut ctx);
}

#[test]
fn test_toast_queue_render_empty() {
    let queue = ToastQueue::new();
    let mut buffer = Buffer::new(50, 20);
    let area = Rect::new(0, 0, 50, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    queue.render(&mut ctx);
}

#[test]
fn test_toast_queue_positions() {
    let positions = [
        ToastPosition::TopLeft,
        ToastPosition::TopCenter,
        ToastPosition::TopRight,
        ToastPosition::BottomLeft,
        ToastPosition::BottomCenter,
        ToastPosition::BottomRight,
    ];

    for pos in positions {
        let mut queue = ToastQueue::new().position(pos);
        queue.push("Test", ToastLevel::Info);
        queue.tick();

        let mut buffer = Buffer::new(50, 20);
        let area = Rect::new(0, 0, 50, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);
        queue.render(&mut ctx);
    }
}

#[test]
fn test_toast_queue_stack_directions() {
    for dir in [StackDirection::Down, StackDirection::Up] {
        let mut queue = ToastQueue::new().stack_direction(dir);
        queue.push("Toast 1", ToastLevel::Info);
        queue.push("Toast 2", ToastLevel::Info);
        queue.tick();

        let mut buffer = Buffer::new(50, 20);
        let area = Rect::new(0, 0, 50, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);
        queue.render(&mut ctx);
    }
}

#[test]
fn test_toast_queue_helper() {
    let queue = toast_queue();
    assert!(queue.is_empty());
}

#[test]
fn test_toast_queue_default() {
    let queue = ToastQueue::default();
    assert!(queue.is_empty());
}

#[test]
fn test_toast_queue_total_count() {
    let mut queue = ToastQueue::new().max_visible(1);
    queue.push("Toast 1", ToastLevel::Info);
    queue.push("Toast 2", ToastLevel::Info);
    queue.tick();

    assert_eq!(queue.visible_count(), 1);
    assert_eq!(queue.pending_count(), 1);
    assert_eq!(queue.total_count(), 2);
}

#[test]
fn test_toast_queue_dismiss_first() {
    let mut queue = ToastQueue::new();
    queue.push("Toast 1", ToastLevel::Info);
    queue.push("Toast 2", ToastLevel::Info);
    queue.tick();

    queue.dismiss_first();
    assert_eq!(queue.visible_count(), 1);
}

// =========================================================================
// ToastQueue builder tests
// =========================================================================

#[test]
fn test_toast_queue_position_builder() {
    let queue = ToastQueue::new().position(ToastPosition::BottomLeft);
    assert_eq!(queue.get_position(), ToastPosition::BottomLeft);
}

#[test]
fn test_toast_queue_stack_direction_builder() {
    let queue = ToastQueue::new().stack_direction(StackDirection::Up);
    assert_eq!(queue.get_stack_direction(), StackDirection::Up);
}

#[test]
fn test_toast_queue_max_visible_builder() {
    let queue = ToastQueue::new().max_visible(10);
    assert_eq!(queue.get_max_visible(), 10);
}

#[test]
fn test_toast_queue_default_duration_builder() {
    let queue = ToastQueue::new().default_duration(Duration::from_secs(5));
    assert_eq!(queue.get_default_duration(), Duration::from_secs(5));
}

#[test]
fn test_toast_queue_gap_builder() {
    let queue = ToastQueue::new().gap(2);
    assert_eq!(queue.get_gap(), 2);
}

#[test]
fn test_toast_queue_toast_width_builder() {
    let queue = ToastQueue::new().toast_width(60);
    assert_eq!(queue.get_toast_width(), 60);
}

#[test]
fn test_toast_queue_deduplicate_builder() {
    let queue = ToastQueue::new().deduplicate(false);
    assert!(!queue.get_deduplicate());
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
// ToastQueue edge case tests
// =========================================================================

#[test]
fn test_toast_queue_dismiss_first_empty() {
    let mut queue = ToastQueue::new();
    queue.dismiss_first(); // Should not panic
    assert_eq!(queue.visible_count(), 0);
}

#[test]
fn test_toast_queue_dismiss_first_non_dismissible() {
    let mut queue = ToastQueue::new();
    queue.push_entry(ToastEntry::new("Non-dismissible", ToastLevel::Info).dismissible(false));
    queue.tick();
    queue.dismiss_first();
    assert_eq!(queue.visible_count(), 1); // Should not dismiss
}

#[test]
fn test_toast_queue_dismiss_nonexistent_id() {
    let mut queue = ToastQueue::new();
    queue.push("Test", ToastLevel::Info);
    queue.tick();
    queue.dismiss("nonexistent-id");
    assert_eq!(queue.visible_count(), 1); // Should not remove anything
}

#[test]
fn test_toast_queue_tick_with_max_visible_zero() {
    let mut queue = ToastQueue::new().max_visible(0);
    queue.push("Test", ToastLevel::Info);
    queue.tick();
    assert_eq!(queue.visible_count(), 0); // No toasts should be visible
}

#[test]
fn test_toast_queue_priority_ordering_multiple() {
    let mut queue = ToastQueue::new();
    queue
        .push_entry(ToastEntry::new("Low", ToastLevel::Info).with_priority(ToastPriority::Low));
    queue.push_entry(
        ToastEntry::new("Critical", ToastLevel::Info).with_priority(ToastPriority::Critical),
    );
    queue.push_entry(
        ToastEntry::new("Normal", ToastLevel::Info).with_priority(ToastPriority::Normal),
    );
    queue.push_entry(
        ToastEntry::new("High", ToastLevel::Info).with_priority(ToastPriority::High),
    );

    // Should be ordered by priority: Critical, High, Normal, Low
    assert_eq!(queue.get_queue()[0].message, "Critical");
    assert_eq!(queue.get_queue()[1].message, "High");
    assert_eq!(queue.get_queue()[2].message, "Normal");
    assert_eq!(queue.get_queue()[3].message, "Low");
}
