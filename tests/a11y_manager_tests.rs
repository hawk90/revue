//! Tests for accessibility manager module

use revue::utils::accessibility::{
    accessibility_manager, announce, announce_now, shared_accessibility, AccessibilityManager,
    AccessibleNode, AccessibleState, Announcement, Priority, Role, SharedAccessibility,
};

// =========================================================================
// AccessibilityManager::new() and default tests
// =========================================================================

#[test]
fn test_accessibility_manager_new() {
    let mgr = AccessibilityManager::new();
    assert!(mgr.is_enabled());
    assert!(!mgr.prefers_reduced_motion());
    assert!(!mgr.is_high_contrast());
    assert!(mgr.focusable_nodes().is_empty());
    assert!(mgr.root().is_none());
    assert!(mgr.focus().is_none());
    assert!(mgr.pending_announcements().is_empty());
}

#[test]
fn test_accessibility_manager_default() {
    let mgr = AccessibilityManager::default();
    assert!(mgr.is_enabled());
}

// =========================================================================
// AccessibilityManager::set_enabled() / is_enabled() tests
// =========================================================================

#[test]
fn test_accessibility_manager_set_enabled_true() {
    let mut mgr = AccessibilityManager::new();
    mgr.set_enabled(true);
    assert!(mgr.is_enabled());
}

#[test]
fn test_accessibility_manager_set_enabled_false() {
    let mut mgr = AccessibilityManager::new();
    mgr.set_enabled(false);
    assert!(!mgr.is_enabled());
}

// =========================================================================
// AccessibilityManager::set_reduce_motion() / prefers_reduced_motion() tests
// =========================================================================

#[test]
fn test_accessibility_manager_set_reduce_motion_true() {
    let mut mgr = AccessibilityManager::new();
    mgr.set_reduce_motion(true);
    assert!(mgr.prefers_reduced_motion());
}

#[test]
fn test_accessibility_manager_set_reduce_motion_false() {
    let mut mgr = AccessibilityManager::new();
    mgr.set_reduce_motion(false);
    assert!(!mgr.prefers_reduced_motion());
}

// =========================================================================
// AccessibilityManager::set_high_contrast() / is_high_contrast() tests
// =========================================================================

#[test]
fn test_accessibility_manager_set_high_contrast_true() {
    let mut mgr = AccessibilityManager::new();
    mgr.set_high_contrast(true);
    assert!(mgr.is_high_contrast());
}

#[test]
fn test_accessibility_manager_set_high_contrast_false() {
    let mut mgr = AccessibilityManager::new();
    mgr.set_high_contrast(false);
    assert!(!mgr.is_high_contrast());
}

// =========================================================================
// AccessibilityManager::set_root() / root() tests
// =========================================================================

#[test]
fn test_accessibility_manager_set_root_str() {
    let mut mgr = AccessibilityManager::new();
    mgr.add_node(AccessibleNode::with_id("root", Role::Main));
    mgr.set_root("root");
    assert_eq!(mgr.root().map(|n| n.id.as_str()), Some("root"));
}

#[test]
fn test_accessibility_manager_set_root_string() {
    let mut mgr = AccessibilityManager::new();
    mgr.add_node(AccessibleNode::with_id("root", Role::Main));
    mgr.set_root(String::from("root"));
    assert_eq!(mgr.root().map(|n| n.id.as_str()), Some("root"));
}

#[test]
fn test_accessibility_manager_root_none() {
    let mgr = AccessibilityManager::new();
    assert!(mgr.root().is_none());
}

// =========================================================================
// AccessibilityManager::add_node() / remove_node() / get_node() tests
// =========================================================================

#[test]
fn test_accessibility_manager_add_node() {
    let mut mgr = AccessibilityManager::new();
    let node = AccessibleNode::with_id("test", Role::Button);
    mgr.add_node(node);
    assert!(mgr.get_node("test").is_some());
}

#[test]
fn test_accessibility_manager_get_node() {
    let mut mgr = AccessibilityManager::new();
    let node = AccessibleNode::with_id("test", Role::Button).label("Test");
    mgr.add_node(node);
    let retrieved = mgr.get_node("test");
    assert_eq!(retrieved.map(|n| n.label.as_deref()), Some(Some("Test")));
}

#[test]
fn test_accessibility_manager_get_node_not_found() {
    let mgr = AccessibilityManager::new();
    assert!(mgr.get_node("nonexistent").is_none());
}

#[test]
fn test_accessibility_manager_remove_node() {
    let mut mgr = AccessibilityManager::new();
    let node = AccessibleNode::with_id("test", Role::Button);
    mgr.add_node(node);
    let removed = mgr.remove_node("test");
    assert!(removed.is_some());
    assert!(mgr.get_node("test").is_none());
}

#[test]
fn test_accessibility_manager_remove_node_not_found() {
    let mut mgr = AccessibilityManager::new();
    assert!(mgr.remove_node("nonexistent").is_none());
}

// =========================================================================
// AccessibilityManager::get_node_mut() tests
// =========================================================================

#[test]
fn test_accessibility_manager_get_node_mut() {
    let mut mgr = AccessibilityManager::new();
    let node = AccessibleNode::with_id("test", Role::Button).label("Old");
    mgr.add_node(node);
    if let Some(node_mut) = mgr.get_node_mut("test") {
        node_mut.label = Some("New".to_string());
    }
    assert_eq!(
        mgr.get_node("test").and_then(|n| n.label.as_deref()),
        Some("New")
    );
}

#[test]
fn test_accessibility_manager_get_node_mut_not_found() {
    let mut mgr = AccessibilityManager::new();
    assert!(mgr.get_node_mut("nonexistent").is_none());
}

// =========================================================================
// AccessibilityManager::update_state() tests
// =========================================================================

#[test]
fn test_accessibility_manager_update_state() {
    let mut mgr = AccessibilityManager::new();
    let node = AccessibleNode::with_id("test", Role::Button);
    mgr.add_node(node);
    let new_state = AccessibleState::new().disabled(true);
    mgr.update_state("test", new_state);
    assert!(mgr
        .get_node("test")
        .map(|n| n.state.disabled)
        .unwrap_or(false));
}

#[test]
fn test_accessibility_manager_update_state_nonexistent() {
    let mut mgr = AccessibilityManager::new();
    let state = AccessibleState::new();
    mgr.update_state("nonexistent", state); // Should not panic
}

// =========================================================================
// AccessibilityManager::set_focus() / focus() / focused_node() tests
// =========================================================================

#[test]
fn test_accessibility_manager_set_focus() {
    let mut mgr = AccessibilityManager::new();
    let node = AccessibleNode::with_id("btn1", Role::Button).label("Button 1");
    mgr.add_node(node);
    mgr.set_focus("btn1");
    assert_eq!(mgr.focus(), Some("btn1"));
    assert!(mgr
        .get_node("btn1")
        .map(|n| n.state.focused)
        .unwrap_or(false));
}

#[test]
fn test_accessibility_manager_set_focus_updates_old_focus() {
    let mut mgr = AccessibilityManager::new();
    let node1 = AccessibleNode::with_id("btn1", Role::Button).label("Button 1");
    let node2 = AccessibleNode::with_id("btn2", Role::Button).label("Button 2");
    mgr.add_node(node1);
    mgr.add_node(node2);
    mgr.set_focus("btn1");
    mgr.set_focus("btn2");
    assert!(!mgr
        .get_node("btn1")
        .map(|n| n.state.focused)
        .unwrap_or(true));
    assert!(mgr
        .get_node("btn2")
        .map(|n| n.state.focused)
        .unwrap_or(false));
}

#[test]
fn test_accessibility_manager_focus_none() {
    let mgr = AccessibilityManager::new();
    assert_eq!(mgr.focus(), None);
}

#[test]
fn test_accessibility_manager_focused_node() {
    let mut mgr = AccessibilityManager::new();
    let node = AccessibleNode::with_id("btn1", Role::Button).label("Button 1");
    mgr.add_node(node);
    mgr.set_focus("btn1");
    assert_eq!(mgr.focused_node().map(|n| n.id.as_str()), Some("btn1"));
}

#[test]
fn test_accessibility_manager_focused_node_none() {
    let mgr = AccessibilityManager::new();
    assert!(mgr.focused_node().is_none());
}

#[test]
fn test_accessibility_manager_set_focus_not_focusable() {
    let mut mgr = AccessibilityManager::new();
    let node =
        AccessibleNode::with_id("btn1", Role::Button).state(AccessibleState::new().disabled(true));
    mgr.add_node(node);
    mgr.set_focus("btn1");
    assert_eq!(mgr.focus(), None); // Should not focus disabled node
}

// =========================================================================
// AccessibilityManager::focus_next() / focus_prev() tests
// =========================================================================

#[test]
fn test_accessibility_manager_focus_next() {
    let mut mgr = AccessibilityManager::new();
    mgr.add_node(AccessibleNode::with_id("btn1", Role::Button));
    mgr.add_node(AccessibleNode::with_id("btn2", Role::Button));
    mgr.add_node(AccessibleNode::with_id("lbl1", Role::Generic));
    mgr.set_focus("btn1");
    let next = mgr.focus_next();
    assert_eq!(next, Some("btn2"));
}

#[test]
fn test_accessibility_manager_focus_next_wraps() {
    let mut mgr = AccessibilityManager::new();
    mgr.add_node(AccessibleNode::with_id("btn1", Role::Button));
    mgr.add_node(AccessibleNode::with_id("btn2", Role::Button));
    mgr.set_focus("btn2");
    let next = mgr.focus_next();
    assert_eq!(next, Some("btn1")); // Wraps around
}

#[test]
fn test_accessibility_manager_focus_next_no_focusable() {
    let mut mgr = AccessibilityManager::new();
    mgr.add_node(AccessibleNode::with_id("lbl1", Role::Generic));
    assert_eq!(mgr.focus_next(), None);
}

#[test]
fn test_accessibility_manager_focus_prev() {
    let mut mgr = AccessibilityManager::new();
    mgr.add_node(AccessibleNode::with_id("btn1", Role::Button));
    mgr.add_node(AccessibleNode::with_id("btn2", Role::Button));
    mgr.add_node(AccessibleNode::with_id("lbl1", Role::Generic));
    mgr.set_focus("btn2");
    let prev = mgr.focus_prev();
    assert_eq!(prev, Some("btn1"));
}

#[test]
fn test_accessibility_manager_focus_prev_wraps() {
    let mut mgr = AccessibilityManager::new();
    mgr.add_node(AccessibleNode::with_id("btn1", Role::Button));
    mgr.add_node(AccessibleNode::with_id("btn2", Role::Button));
    mgr.set_focus("btn1");
    let prev = mgr.focus_prev();
    assert_eq!(prev, Some("btn2")); // Wraps around
}

#[test]
fn test_accessibility_manager_focus_prev_no_focusable() {
    let mut mgr = AccessibilityManager::new();
    mgr.add_node(AccessibleNode::with_id("lbl1", Role::Generic));
    assert_eq!(mgr.focus_prev(), None);
}

// =========================================================================
// AccessibilityManager::announce() tests
// =========================================================================

#[test]
fn test_accessibility_manager_announce() {
    let mut mgr = AccessibilityManager::new();
    let ann = Announcement::polite("Test");
    mgr.announce(ann);
    assert_eq!(mgr.pending_announcements().len(), 1);
}

#[test]
fn test_accessibility_manager_announce_when_disabled() {
    let mut mgr = AccessibilityManager::new();
    mgr.set_enabled(false);
    mgr.announce(Announcement::polite("Test"));
    assert_eq!(mgr.pending_announcements().len(), 0);
}

// =========================================================================
// AccessibilityManager::announce_polite() / announce_assertive() tests
// =========================================================================

#[test]
fn test_accessibility_manager_announce_polite() {
    let mut mgr = AccessibilityManager::new();
    mgr.announce_polite("Polite message");
    let pending = mgr.pending_announcements();
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].priority, Priority::Polite);
}

#[test]
fn test_accessibility_manager_announce_assertive() {
    let mut mgr = AccessibilityManager::new();
    mgr.announce_assertive("Assertive message");
    let pending = mgr.pending_announcements();
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].priority, Priority::Assertive);
}

// =========================================================================
// AccessibilityManager::clear_announcements() tests
// =========================================================================

#[test]
fn test_accessibility_manager_clear_announcements() {
    let mut mgr = AccessibilityManager::new();
    mgr.announce_polite("Test 1");
    mgr.announce_polite("Test 2");
    mgr.clear_announcements();
    assert_eq!(mgr.pending_announcements().len(), 0);
}

// =========================================================================
// AccessibilityManager::focusable_nodes() tests
// =========================================================================

#[test]
fn test_accessibility_manager_focusable_nodes() {
    let mut mgr = AccessibilityManager::new();
    mgr.add_node(AccessibleNode::with_id("btn1", Role::Button));
    mgr.add_node(AccessibleNode::with_id("btn2", Role::Button));
    mgr.add_node(AccessibleNode::with_id("lbl1", Role::Generic));
    let focusable = mgr.focusable_nodes();
    assert_eq!(focusable.len(), 2);
}

#[test]
fn test_accessibility_manager_focusable_nodes_empty() {
    let mgr = AccessibilityManager::new();
    assert!(mgr.focusable_nodes().is_empty());
}

// =========================================================================
// AccessibilityManager::landmarks() tests
// =========================================================================

#[test]
fn test_accessibility_manager_landmarks() {
    let mut mgr = AccessibilityManager::new();
    mgr.add_node(AccessibleNode::with_id("nav1", Role::Navigation));
    mgr.add_node(AccessibleNode::with_id("main1", Role::Main));
    mgr.add_node(AccessibleNode::with_id("btn1", Role::Button));
    let landmarks = mgr.landmarks();
    assert_eq!(landmarks.len(), 2);
}

#[test]
fn test_accessibility_manager_landmarks_empty() {
    let mgr = AccessibilityManager::new();
    assert!(mgr.landmarks().is_empty());
}

// =========================================================================
// AccessibilityManager::clear() tests
// =========================================================================

#[test]
fn test_accessibility_manager_clear() {
    let mut mgr = AccessibilityManager::new();
    mgr.add_node(AccessibleNode::with_id("btn1", Role::Button));
    mgr.set_root("btn1");
    mgr.set_focus("btn1");
    mgr.announce_polite("Test");
    mgr.clear();
    assert!(mgr.focusable_nodes().is_empty());
    assert!(mgr.root().is_none());
    assert!(mgr.focus().is_none());
}

// =========================================================================
// Helper function tests
// =========================================================================

#[test]
fn test_accessibility_manager_helper() {
    let mgr = accessibility_manager();
    assert!(mgr.is_enabled());
}

// =========================================================================
// SharedAccessibility tests
// =========================================================================

#[test]
fn test_shared_accessibility_new() {
    let shared = SharedAccessibility::new();
    assert!(shared.focus().is_none());
}

#[test]
fn test_shared_accessibility_default() {
    let shared = SharedAccessibility::default();
    // Should create without panicking
    let _ = shared;
}

#[test]
fn test_shared_accessibility_announce() {
    let shared = SharedAccessibility::new();
    shared.announce("Test message");
    // Should not panic
}

#[test]
fn test_shared_accessibility_announce_now() {
    let shared = SharedAccessibility::new();
    shared.announce_now("Test message");
    // Should not panic
}

#[test]
fn test_shared_accessibility_set_focus() {
    let shared = SharedAccessibility::new();
    shared.set_focus("test-id");
    // Should not panic
}

#[test]
fn test_shared_accessibility_focus_none() {
    let shared = SharedAccessibility::new();
    assert_eq!(shared.focus(), None);
}

// =========================================================================
// Convenience function tests
// =========================================================================

#[test]
fn test_announce_function() {
    let ann = announce("Test");
    assert_eq!(ann.message, "Test");
    assert_eq!(ann.priority, Priority::Polite);
}

#[test]
fn test_announce_now_function() {
    let ann = announce_now("Test");
    assert_eq!(ann.message, "Test");
    assert_eq!(ann.priority, Priority::Assertive);
}

#[test]
fn test_shared_accessibility_helper() {
    let shared = shared_accessibility();
    // Should create without panicking
    let _ = shared;
}
