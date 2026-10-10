//! Focus management tests

use revue::event::{Direction, FocusManager, FocusTrap, FocusTrapConfig};
use revue::layout::Rect;

#[test]
fn test_focus_manager_new() {
    let fm = FocusManager::new();
    assert!(fm.current().is_none());
}

#[test]
fn test_focus_register() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.register(2);
    fm.register(3);

    // No focus yet
    assert!(fm.current().is_none());
}

#[test]
fn test_focus_next() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.register(2);
    fm.register(3);

    fm.next();
    assert_eq!(fm.current(), Some(1));

    fm.next();
    assert_eq!(fm.current(), Some(2));

    fm.next();
    assert_eq!(fm.current(), Some(3));

    // Wrap around
    fm.next();
    assert_eq!(fm.current(), Some(1));
}

#[test]
fn test_focus_prev() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.register(2);
    fm.register(3);

    fm.prev();
    assert_eq!(fm.current(), Some(3));

    fm.prev();
    assert_eq!(fm.current(), Some(2));

    fm.prev();
    assert_eq!(fm.current(), Some(1));

    // Wrap around
    fm.prev();
    assert_eq!(fm.current(), Some(3));
}

#[test]
fn test_focus_specific_widget() {
    let mut fm = FocusManager::new();
    fm.register(10);
    fm.register(20);
    fm.register(30);

    fm.focus(20);
    assert_eq!(fm.current(), Some(20));

    fm.focus(30);
    assert_eq!(fm.current(), Some(30));
}

#[test]
fn test_is_focused() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.register(2);

    fm.focus(1);
    assert!(fm.is_focused(1));
    assert!(!fm.is_focused(2));
}

#[test]
fn test_blur() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.next();
    assert!(fm.current().is_some());

    fm.blur();
    assert!(fm.current().is_none());
}

#[test]
fn test_unregister() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.register(2);
    fm.register(3);

    fm.focus(2);
    fm.unregister(1);

    // Focus should adjust
    assert_eq!(fm.current(), Some(2));
}

// 2D Navigation Tests

#[test]
fn test_2d_navigation_right() {
    let mut fm = FocusManager::new();
    // Layout:  [1] [2] [3]
    //           x=0  10  20
    fm.register_with_position(1, 0, 0);
    fm.register_with_position(2, 10, 0);
    fm.register_with_position(3, 20, 0);

    fm.focus(1);
    assert!(fm.move_focus(Direction::Right));
    assert_eq!(fm.current(), Some(2));

    assert!(fm.move_focus(Direction::Right));
    assert_eq!(fm.current(), Some(3));

    // No more to the right
    assert!(!fm.move_focus(Direction::Right));
}

#[test]
fn test_2d_navigation_down() {
    let mut fm = FocusManager::new();
    // Layout:  [1]
    //          [2]
    //          [3]
    fm.register_with_position(1, 0, 0);
    fm.register_with_position(2, 0, 10);
    fm.register_with_position(3, 0, 20);

    fm.focus(1);
    assert!(fm.move_focus(Direction::Down));
    assert_eq!(fm.current(), Some(2));

    assert!(fm.move_focus(Direction::Down));
    assert_eq!(fm.current(), Some(3));
}

#[test]
fn test_2d_navigation_grid() {
    let mut fm = FocusManager::new();
    // Layout:  [1] [2]
    //          [3] [4]
    fm.register_with_position(1, 0, 0);
    fm.register_with_position(2, 10, 0);
    fm.register_with_position(3, 0, 10);
    fm.register_with_position(4, 10, 10);

    fm.focus(1);

    // Right to 2
    assert!(fm.move_focus(Direction::Right));
    assert_eq!(fm.current(), Some(2));

    // Down to 4
    assert!(fm.move_focus(Direction::Down));
    assert_eq!(fm.current(), Some(4));

    // Left to 3
    assert!(fm.move_focus(Direction::Left));
    assert_eq!(fm.current(), Some(3));

    // Up to 1
    assert!(fm.move_focus(Direction::Up));
    assert_eq!(fm.current(), Some(1));
}

#[test]
fn test_register_with_bounds() {
    let mut fm = FocusManager::new();
    let bounds = Rect::new(10, 5, 20, 10);
    fm.register_with_bounds(1, bounds);

    fm.focus(1);
    assert_eq!(fm.current(), Some(1));
}

// Focus Trapping Tests

#[test]
fn test_focus_trap() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.register(2);
    fm.register(3); // Modal button 1
    fm.register(4); // Modal button 2

    // Focus on widget 1
    fm.focus(1);
    assert_eq!(fm.current(), Some(1));

    // Trap focus to modal (widgets 3 and 4)
    fm.trap_focus(100); // Modal container ID
    fm.add_to_trap(3);
    fm.add_to_trap(4);

    assert!(fm.is_trapped());

    // Tab should now only cycle between 3 and 4
    fm.focus(3);
    fm.next();
    assert_eq!(fm.current(), Some(4));

    fm.next();
    assert_eq!(fm.current(), Some(3)); // Wraps within trap

    // Release trap
    fm.release_trap();
    assert!(!fm.is_trapped());

    // Now Tab cycles all widgets again
    fm.focus(1);
    fm.next();
    assert_eq!(fm.current(), Some(2));
}

#[test]
fn test_trap_container() {
    let mut fm = FocusManager::new();
    fm.trap_focus(42);
    assert_eq!(fm.trap_container(), Some(42));

    fm.release_trap();
    assert_eq!(fm.trap_container(), None);
}

// Focus Restoration Tests

#[test]
fn test_focus_restoration() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.register(2);
    fm.register(3);
    fm.register(4);

    // Focus on widget 2
    fm.focus(2);
    assert_eq!(fm.current(), Some(2));

    // Trap focus
    fm.trap_focus(100);
    fm.add_to_trap(3);
    fm.add_to_trap(4);
    fm.focus(3);

    // Saved focus should be 2
    assert_eq!(fm.saved_focus(), Some(2));

    // Release and restore
    fm.release_trap_and_restore();
    assert_eq!(fm.current(), Some(2));
}

#[test]
fn test_trap_with_initial_focus() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.register(2);
    fm.register(3);

    fm.focus(1);
    fm.trap_focus_with_initial(100, 3);
    assert_eq!(fm.current(), Some(3));
}

// Nested Focus Trap Tests

#[test]
fn test_push_pop_trap() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.register(2);
    fm.register(3);
    fm.register(4);
    fm.register(5);

    // Start on widget 1
    fm.focus(1);
    assert_eq!(fm.current(), Some(1));

    // Push first trap (modal 1)
    fm.push_trap(100, &[2, 3]);
    assert_eq!(fm.current(), Some(2));
    assert_eq!(fm.trap_depth(), 1);

    // Push second trap (modal 2)
    fm.push_trap(200, &[4, 5]);
    assert_eq!(fm.current(), Some(4));
    assert_eq!(fm.trap_depth(), 2);

    // Pop second trap - should restore to modal 1
    fm.pop_trap();
    assert_eq!(fm.current(), Some(2));
    assert_eq!(fm.trap_depth(), 1);

    // Pop first trap - should restore to original
    fm.pop_trap();
    assert_eq!(fm.current(), Some(1));
    assert_eq!(fm.trap_depth(), 0);
}

#[test]
fn test_trap_depth() {
    let mut fm = FocusManager::new();
    fm.register(1);

    assert_eq!(fm.trap_depth(), 0);

    fm.push_trap(100, &[1]);
    assert_eq!(fm.trap_depth(), 1);

    fm.push_trap(200, &[1]);
    assert_eq!(fm.trap_depth(), 2);

    fm.pop_trap();
    assert_eq!(fm.trap_depth(), 1);

    fm.pop_trap();
    assert_eq!(fm.trap_depth(), 0);
}

// FocusTrap Helper Tests

#[test]
fn test_focus_trap_helper() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.register(2);
    fm.register(3);

    fm.focus(1);

    let mut trap = FocusTrap::new(100).with_children(&[2, 3]).initial_focus(3);

    assert!(!trap.is_active());

    trap.activate(&mut fm);
    assert!(trap.is_active());
    assert_eq!(fm.current(), Some(3));

    trap.deactivate(&mut fm);
    assert!(!trap.is_active());
    assert_eq!(fm.current(), Some(1)); // Restored
}

#[test]
fn test_focus_trap_add_child() {
    let trap = FocusTrap::new(100).add_child(1).add_child(2).add_child(2); // Duplicate should be ignored

    assert_eq!(trap.container_id(), 100);
}

fn manager_with(ids: &[u64]) -> FocusManager {
    let mut fm = FocusManager::new();
    for &id in ids {
        fm.register(id);
    }
    fm
}

#[test]
fn test_focus_trap_without_loop_stops_at_the_ends() {
    let mut fm = manager_with(&[1, 2, 3, 4]);
    let mut trap = FocusTrap::new(100)
        .with_children(&[2, 3, 4])
        .loop_focus(false);
    trap.activate(&mut fm);
    assert_eq!(fm.current(), Some(2));

    fm.next();
    fm.next();
    assert_eq!(fm.current(), Some(4));
    fm.next();
    assert_eq!(fm.current(), Some(4), "Tab stays on the last element");

    fm.prev();
    fm.prev();
    assert_eq!(fm.current(), Some(2));
    fm.prev();
    assert_eq!(
        fm.current(),
        Some(2),
        "Shift+Tab stays on the first element"
    );

    // Released, Tab wraps again over every widget
    trap.deactivate(&mut fm);
    fm.focus(4);
    fm.next();
    assert_eq!(fm.current(), Some(1));
}

#[test]
fn test_focus_trap_with_loop_wraps() {
    let mut fm = manager_with(&[1, 2, 3]);
    let mut trap = FocusTrap::new(100).with_children(&[2, 3]);
    trap.activate(&mut fm);

    fm.next();
    fm.next();
    assert_eq!(fm.current(), Some(2));
    fm.prev();
    assert_eq!(fm.current(), Some(3));
}

#[test]
fn test_nested_focus_traps_keep_their_own_loop_setting() {
    let mut fm = manager_with(&[1, 2, 3, 4, 5]);
    let mut outer = FocusTrap::new(100).with_children(&[1, 2]).loop_focus(false);
    outer.activate(&mut fm);

    let mut inner = FocusTrap::new(200).with_children(&[4, 5]);
    inner.activate(&mut fm);
    fm.next();
    fm.next();
    assert_eq!(fm.current(), Some(4), "inner trap loops");

    inner.deactivate(&mut fm);
    fm.focus(2);
    fm.next();
    assert_eq!(fm.current(), Some(2), "outer trap still does not loop");
}

#[test]
fn test_focus_trap_config() {
    let config = FocusTrapConfig::default();
    assert!(config.restore_on_release);
    assert!(config.loop_focus);
    assert!(config.initial_focus.is_none());
}

#[test]
fn test_focus_trap_without_restore_keeps_focus_on_release() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.register(2);
    fm.register(3);
    fm.focus(1);

    let mut trap = FocusTrap::new(100)
        .with_children(&[2, 3])
        .initial_focus(3)
        .restore_focus_on_release(false);
    trap.activate(&mut fm);
    assert_eq!(fm.current(), Some(3));

    trap.deactivate(&mut fm);
    assert!(!fm.is_trapped());
    assert_eq!(fm.current(), Some(3)); // Not restored to 1
}

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
