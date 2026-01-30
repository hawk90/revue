//! Focus system edge case tests
//!
//! Tests for edge cases, boundary conditions, and potential panics in the focus system.
//! Specifically addresses the critical issue in src/event/focus.rs lines 193-214
//! where `ids[0]` and `ids[ids.len()-1]` could panic if `ids` is empty.

use revue::event::{Direction, FocusManager, FocusTrap, FocusTrapConfig};

// ==================== Empty Widget List Tests ====================

/// Test that next() doesn't panic with empty widget list
/// This tests line 194: `ids[0]` which could panic if ids is empty
#[test]
fn test_focus_next_with_empty_widgets() {
    let mut fm = FocusManager::new();
    // No widgets registered
    // This used to be a panic risk at line 194: ids[0]
    fm.next();
    // Should not panic, current should remain None
    assert!(fm.current().is_none());
}

/// Test that prev() doesn't panic with empty widget list
/// This tests line 211 and 213: `ids[ids.len()-1]` which could panic
#[test]
fn test_focus_prev_with_empty_widgets() {
    let mut fm = FocusManager::new();
    // No widgets registered
    // This used to be a panic risk at line 211: ids[ids.len()-1]
    fm.prev();
    // Should not panic, current should remain None
    assert!(fm.current().is_none());
}

/// Test navigation with empty trapped list
#[test]
fn test_focus_nav_with_empty_trap() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.register(2);

    // Create an empty trap
    fm.trap_focus(100);
    // Don't add any widgets to trap

    assert!(fm.is_trapped());

    // These should not panic even with empty trapped list
    fm.next();
    fm.prev();

    // When trap is active but empty, it falls back to all widgets
    // So navigation will work with non-empty widget list
    // Just verify it doesn't panic
    assert!(fm.is_trapped());
}

/// Test 2D navigation with empty widget list
#[test]
fn test_move_focus_with_no_widgets() {
    let mut fm = FocusManager::new();
    // No widgets registered, no focus set

    // All directions should return false, not panic
    assert!(!fm.move_focus(Direction::Up));
    assert!(!fm.move_focus(Direction::Down));
    assert!(!fm.move_focus(Direction::Left));
    assert!(!fm.move_focus(Direction::Right));
}

/// Test 2D navigation without position data
#[test]
fn test_move_focus_without_position() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.focus(1);

    // Widget has no position, can't do 2D nav
    // Should return false, not panic
    assert!(!fm.move_focus(Direction::Right));
}

// ==================== Single Widget Tests ====================

#[test]
fn test_focus_next_with_single_widget() {
    let mut fm = FocusManager::new();
    fm.register(1);

    fm.next();
    assert_eq!(fm.current(), Some(1));

    // Wrapping on single widget
    fm.next();
    assert_eq!(fm.current(), Some(1));
}

#[test]
fn test_focus_prev_with_single_widget() {
    let mut fm = FocusManager::new();
    fm.register(1);

    fm.prev();
    assert_eq!(fm.current(), Some(1));

    // Wrapping on single widget
    fm.prev();
    assert_eq!(fm.current(), Some(1));
}

#[test]
fn test_2d_nav_with_single_widget() {
    let mut fm = FocusManager::new();
    fm.register_with_position(1, 10, 10);
    fm.focus(1);

    // No other widgets to navigate to
    assert!(!fm.move_focus(Direction::Up));
    assert!(!fm.move_focus(Direction::Down));
    assert!(!fm.move_focus(Direction::Left));
    assert!(!fm.move_focus(Direction::Right));
}

// ==================== Trap Edge Cases ====================

#[test]
fn test_trap_with_single_child() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.register(2);
    fm.register(3);

    fm.focus(1);

    // Trap with single widget
    fm.trap_focus(100);
    fm.add_to_trap(2);

    assert!(fm.is_trapped());

    // Should only focus on 2
    fm.next();
    assert_eq!(fm.current(), Some(2));

    // Wraps within single widget trap
    fm.next();
    assert_eq!(fm.current(), Some(2));
}

#[test]
fn test_push_trap_with_empty_children() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.focus(1);

    // Push trap with no children
    fm.push_trap(100, &[]);

    // Current focus should be cleared (no children to focus)
    // Should not panic
    assert_eq!(fm.trap_depth(), 1);
}

#[test]
fn test_nested_empty_traps() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.focus(1);

    // Push multiple empty traps
    fm.push_trap(100, &[]);
    assert_eq!(fm.trap_depth(), 1);

    fm.push_trap(200, &[]);
    assert_eq!(fm.trap_depth(), 2);

    // Pop should work
    fm.pop_trap();
    assert_eq!(fm.trap_depth(), 1);

    fm.pop_trap();
    assert_eq!(fm.trap_depth(), 0);
}

#[test]
fn test_pop_trap_without_push() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.focus(1);

    // Pop without push should use release_trap_and_restore
    let result = fm.pop_trap();
    // Should return false (no stack to pop from)
    assert!(!result);

    // Focus should be preserved (no saved focus to restore)
    assert_eq!(fm.current(), Some(1));
}

#[test]
fn test_focus_trap_helper_with_no_children() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.focus(1);

    let mut trap = FocusTrap::new(100); // No children
    trap.activate(&mut fm);

    // Should activate but not crash
    assert!(trap.is_active());
    assert!(fm.is_trapped());

    // Deactivate should restore
    trap.deactivate(&mut fm);
    assert_eq!(fm.current(), Some(1));
}

#[test]
fn test_multiple_activations_of_same_trap() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.register(2);
    fm.focus(1);

    let mut trap = FocusTrap::new(100).with_children(&[2]);

    trap.activate(&mut fm);
    assert!(trap.is_active());
    assert_eq!(fm.current(), Some(2));

    // Activate again - should be idempotent
    trap.activate(&mut fm);
    assert!(trap.is_active());

    trap.deactivate(&mut fm);
    assert!(!trap.is_active());
    assert_eq!(fm.current(), Some(1));

    // Deactivate again - should be idempotent
    trap.deactivate(&mut fm);
    assert!(!trap.is_active());
}

// ==================== Unregister Edge Cases ====================

#[test]
fn test_unregister_last_widget_while_focused() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.focus(1);

    // Unregister the only focused widget
    fm.unregister(1);

    // Should clear focus
    assert!(fm.current().is_none());
}

#[test]
fn test_unregister_from_trapped_list() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.register(2);
    fm.register(3);

    fm.trap_focus(100);
    fm.add_to_trap(2);
    fm.add_to_trap(3);
    fm.focus(2);

    // Unregister a trapped widget
    fm.unregister(2);

    // Focus should be cleared or move to next
    // Trap should still be active
    assert!(fm.is_trapped());

    // Navigation should work with remaining trapped widget
    fm.next();
    // Should handle gracefully
}

#[test]
fn test_unregister_all_widgets_during_navigation() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.register(2);
    fm.focus(1);

    // Unregister all widgets
    fm.unregister(1);
    fm.unregister(2);

    // Navigation should not panic
    fm.next();
    fm.prev();

    assert!(fm.current().is_none());
}

// ==================== Large Widget ID Tests ====================

#[test]
fn test_focus_with_large_ids() {
    let mut fm = FocusManager::new();
    fm.register(u64::MAX - 1);
    fm.register(u64::MAX);

    fm.next();
    assert_eq!(fm.current(), Some(u64::MAX - 1));

    fm.next();
    assert_eq!(fm.current(), Some(u64::MAX));

    // Wrap around
    fm.next();
    assert_eq!(fm.current(), Some(u64::MAX - 1));
}

#[test]
fn test_focus_trap_with_large_container_id() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.focus(1);

    fm.trap_focus(u64::MAX);
    assert_eq!(fm.trap_container(), Some(u64::MAX));
}

// ==================== Invalid Focus Target Tests ====================

#[test]
fn test_focus_nonexistent_widget() {
    let mut fm = FocusManager::new();
    fm.register(1);

    // Try to focus on non-existent widget
    fm.focus(999);

    // Should not crash, focus should remain unchanged or None
    assert!(fm.current().is_none());
}

#[test]
fn test_focus_after_clearing_all_widgets() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.register(2);
    fm.focus(1);

    // Unregister all widgets
    fm.unregister(1);
    fm.unregister(2);

    // Try to focus on non-existent widget
    fm.focus(1);
    assert!(fm.current().is_none());
}

// ==================== Boundary Position Tests ====================

#[test]
fn test_2d_nav_with_boundary_positions() {
    let mut fm = FocusManager::new();

    // Register widgets at u16 boundaries
    fm.register_with_position(1, 0, 0);
    fm.register_with_position(2, u16::MAX, 0);
    fm.register_with_position(3, 0, u16::MAX);
    fm.register_with_position(4, u16::MAX, u16::MAX);

    fm.focus(1);

    // Navigate right to u16::MAX
    assert!(fm.move_focus(Direction::Right));
    assert_eq!(fm.current(), Some(2));

    // Navigate down to u16::MAX
    assert!(fm.move_focus(Direction::Down));
    assert_eq!(fm.current(), Some(4));
}

#[test]
fn test_2d_nav_with_same_positions() {
    let mut fm = FocusManager::new();

    // Multiple widgets at same position
    fm.register_with_position(1, 10, 10);
    fm.register_with_position(2, 10, 10);
    fm.register_with_position(3, 10, 10);

    fm.focus(1);

    // Navigation should handle duplicate positions
    // May find one of the other widgets or return false
    let result = fm.move_focus(Direction::Right);
    // Just verify it doesn't panic
    let _ = result;
}

// ==================== Rapid Navigation Tests ====================

#[test]
fn test_rapid_next_prev() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.register(2);
    fm.register(3);

    // Rapid navigation should not cause issues
    for _ in 0..100 {
        fm.next();
    }
    assert!(fm.current().is_some());

    for _ in 0..100 {
        fm.prev();
    }
    assert!(fm.current().is_some());
}

#[test]
fn test_alternating_next_prev() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.register(2);
    fm.register(3);

    fm.focus(2);

    // Alternate between next and prev
    for _ in 0..50 {
        fm.next();
        fm.prev();
    }
    // Should stay on widget 2
    assert_eq!(fm.current(), Some(2));
}

// ==================== Mixed Registration and Navigation ====================

#[test]
fn test_register_during_navigation() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.focus(1);

    fm.next();
    assert_eq!(fm.current(), Some(1));

    // Register more widgets
    fm.register(2);
    fm.register(3);

    fm.next();
    assert_eq!(fm.current(), Some(2));
}

#[test]
fn test_unregister_during_navigation() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.register(2);
    fm.register(3);
    fm.focus(1);

    fm.next();
    assert_eq!(fm.current(), Some(2));

    // Unregister current widget
    fm.unregister(2);

    // Next should work
    fm.next();
    // Should move to another widget or stay
    let current = fm.current();
    assert!(current.is_some() && current != Some(2));
}

// ==================== Default Implementation ====================

#[test]
fn test_focus_manager_default() {
    let mut fm = FocusManager::default();
    assert!(fm.current().is_none());
    // Should not panic on any operations
    fm.next();
    fm.prev();
    assert!(fm.current().is_none());
}

#[test]
fn test_focus_trap_config_default() {
    let config = FocusTrapConfig::default();
    assert!(config.restore_on_release);
    assert!(config.loop_focus);
    assert!(config.initial_focus.is_none());
}
