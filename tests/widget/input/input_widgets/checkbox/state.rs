//! Checkbox state tests

use revue::widget::{checkbox, ToggleWidget};

#[test]
fn test_checkbox_toggle() {
    let mut c = checkbox("A");
    c.toggle();
    assert!(c.is_checked());
    assert!(c.is_on());
    c.toggle();
    assert!(!c.is_checked());
}

#[test]
fn test_checkbox_toggle_disabled() {
    let mut c = checkbox("A").disabled(true);
    c.toggle();
    assert!(!c.is_checked());
}

#[test]
fn test_checkbox_set_checked() {
    let mut c = checkbox("A");
    c.set_checked(true);
    assert!(c.is_checked());
    c.set_checked(false);
    assert!(!c.is_checked());
}

#[test]
fn test_checkbox_set_checked_ignores_disabled() {
    // Programmatic changes are allowed even when the user cannot toggle it
    let mut c = checkbox("A").disabled(true);
    c.set_checked(true);
    assert!(c.is_checked());
    c.set_on(false);
    assert!(!c.is_on());
}

#[test]
fn test_checkbox_toggle_focus_state() {
    let mut c = checkbox("A");
    assert!(!c.is_toggle_focused());
    c.set_toggle_focused(true);
    assert!(c.is_toggle_focused());
    assert!(c.is_focused());
}

#[test]
fn test_checkbox_focusable_unless_disabled() {
    assert!(checkbox("A").toggle_focusable());
    assert!(!checkbox("A").disabled(true).toggle_focusable());
}
