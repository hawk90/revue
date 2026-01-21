//! Tests for accessibility module

use super::*;
use serial_test::serial;

#[test]
#[serial]
fn test_role_name() {
    assert_eq!(Role::Button.name(), "button");
    assert_eq!(Role::TextInput.name(), "textbox");
    assert_eq!(Role::Navigation.name(), "navigation");
}

#[test]
#[serial]
fn test_role_interactive() {
    assert!(Role::Button.is_interactive());
    assert!(Role::TextInput.is_interactive());
    assert!(!Role::Main.is_interactive());
    assert!(!Role::Generic.is_interactive());
}

#[test]
#[serial]
fn test_role_landmark() {
    assert!(Role::Navigation.is_landmark());
    assert!(Role::Main.is_landmark());
    assert!(!Role::Button.is_landmark());
}

#[test]
#[serial]
fn test_accessible_node() {
    let node = AccessibleNode::new(Role::Button)
        .label("Submit")
        .description("Submit the form")
        .shortcut("Enter");

    assert_eq!(node.role, Role::Button);
    assert_eq!(node.label, Some("Submit".to_string()));
    assert!(node.is_focusable());
}

#[test]
#[serial]
fn test_accessible_state() {
    let state = AccessibleState::new()
        .disabled(false)
        .checked(true)
        .expanded(false);

    assert!(!state.disabled);
    assert_eq!(state.checked, Some(true));
    assert_eq!(state.expanded, Some(false));
}

#[test]
#[serial]
fn test_node_describe() {
    let node = AccessibleNode::new(Role::Checkbox)
        .label("Accept terms")
        .state(AccessibleState::new().checked(true));

    let desc = node.describe();
    assert!(desc.contains("Accept terms"));
    assert!(desc.contains("checkbox"));
    assert!(desc.contains("checked"));
}

#[test]
#[serial]
fn test_accessibility_manager() {
    let mut manager = AccessibilityManager::new();

    let button = AccessibleNode::new(Role::Button).label("Click me");
    let button_id = button.id.clone();

    manager.add_node(button);
    assert!(manager.get_node(&button_id).is_some());
}

#[test]
#[serial]
fn test_focus_management() {
    let mut manager = AccessibilityManager::new();

    let btn1 = AccessibleNode::with_id("btn1", Role::Button).label("First");
    let btn2 = AccessibleNode::with_id("btn2", Role::Button).label("Second");

    manager.add_node(btn1);
    manager.add_node(btn2);

    manager.set_focus("btn1");
    assert_eq!(manager.focus(), Some("btn1"));

    manager.focus_next();
    assert_eq!(manager.focus(), Some("btn2"));
}

#[test]
#[serial]
fn test_announcements() {
    let mut manager = AccessibilityManager::new();

    manager.announce_polite("Message 1");
    manager.announce_assertive("Message 2");

    let announcements = manager.pending_announcements();
    assert_eq!(announcements.len(), 2);
    assert_eq!(announcements[0].priority, Priority::Polite);
    assert_eq!(announcements[1].priority, Priority::Assertive);
}

#[test]
#[serial]
fn test_disabled_manager() {
    let mut manager = AccessibilityManager::new();
    manager.set_enabled(false);

    manager.announce_polite("Test");
    assert!(manager.pending_announcements().is_empty());
}

#[test]
#[serial]
fn test_preferences() {
    let mut manager = AccessibilityManager::new();

    manager.set_reduce_motion(true);
    assert!(manager.prefers_reduced_motion());

    manager.set_high_contrast(true);
    assert!(manager.is_high_contrast());
}

#[test]
#[serial]
fn test_shared_accessibility() {
    let shared = SharedAccessibility::new();

    shared.announce("Test message");
    shared.set_focus("test-id");

    assert_eq!(shared.focus(), None); // No node registered with that ID
}

#[test]
#[serial]
fn test_announce_helper() {
    let a = announce("Test");
    assert_eq!(a.priority, Priority::Polite);

    let a = announce_now("Urgent");
    assert_eq!(a.priority, Priority::Assertive);
}

#[test]
#[serial]
fn test_value_range() {
    let state = AccessibleState::new().value_range(50.0, 0.0, 100.0);

    assert_eq!(state.value_now, Some(50.0));
    assert_eq!(state.value_min, Some(0.0));
    assert_eq!(state.value_max, Some(100.0));
}

#[test]
#[serial]
fn test_position_in_set() {
    let state = AccessibleState::new().position(3, 10);

    assert_eq!(state.pos_in_set, Some(3));
    assert_eq!(state.set_size, Some(10));
}

#[test]
#[serial]
fn test_node_not_focusable_when_disabled() {
    let node = AccessibleNode::new(Role::Button).state(AccessibleState::new().disabled(true));

    assert!(!node.is_focusable());
}

#[test]
#[serial]
fn test_node_not_focusable_when_hidden() {
    let mut state = AccessibleState::new();
    state.hidden = true;

    let node = AccessibleNode::new(Role::Button).state(state);
    assert!(!node.is_focusable());
}

#[test]
#[serial]
fn test_aria_builder_from_node() {
    let node = AccessibleNode::new(Role::Button)
        .label("Submit")
        .description("Submit the form");

    let aria = aria::AriaBuilder::new().from_node(&node).build();

    assert_eq!(aria.get("role"), Some(&"button".to_string()));
    assert_eq!(aria.get("aria-label"), Some(&"Submit".to_string()));
    assert_eq!(aria.get("tabindex"), Some(&"0".to_string()));
}

#[test]
#[serial]
fn test_aria_builder_live_region() {
    let aria = aria::AriaBuilder::new()
        .live_region(aria::LiveRegion::Polite)
        .atomic(true)
        .build();

    assert_eq!(aria.get("aria-live"), Some(&"polite".to_string()));
    assert_eq!(aria.get("aria-atomic"), Some(&"true".to_string()));
}

#[test]
#[serial]
fn test_aria_attribute_name() {
    assert_eq!(
        aria::AriaAttribute::Role("button".to_string()).name(),
        "role"
    );
    assert_eq!(
        aria::AriaAttribute::Label("test".to_string()).name(),
        "aria-label"
    );
    assert_eq!(
        aria::AriaAttribute::Checked(Some(true)).name(),
        "aria-checked"
    );
}

#[test]
#[serial]
fn test_aria_attribute_value() {
    assert_eq!(aria::AriaAttribute::Checked(Some(true)).value(), "true");
    assert_eq!(aria::AriaAttribute::Checked(None).value(), "mixed");
    assert_eq!(aria::AriaAttribute::Expanded(None).value(), "undefined");
}

#[test]
#[serial]
fn test_live_region_as_str() {
    assert_eq!(aria::LiveRegion::Off.as_str(), "off");
    assert_eq!(aria::LiveRegion::Polite.as_str(), "polite");
    assert_eq!(aria::LiveRegion::Assertive.as_str(), "assertive");
}
