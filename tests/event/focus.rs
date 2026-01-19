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

#[test]
fn test_focus_trap_config() {
    let config = FocusTrapConfig::default();
    assert!(config.restore_on_release);
    assert!(config.loop_focus);
    assert!(config.initial_focus.is_none());
}
