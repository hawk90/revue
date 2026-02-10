//! Tests for the Alert widget public API

use revue::event::Key;
use revue::widget::{alert, Alert, AlertLevel, AlertVariant};

#[test]
fn test_alert_builder_pattern() {
    // Test that we can create and configure an alert using only public methods
    let a = Alert::new("Test message")
        .title("My Title")
        .level(AlertLevel::Error)
        .variant(AlertVariant::Outlined)
        .dismissible(true)
        .icon(false);

    // Test public methods
    assert!(a.is_dismissed() == false);
    assert_eq!(a.height(), 4); // With title and outlined variant
}

#[test]
fn test_alert_level_builder_methods() {
    // Test the builder methods for AlertLevel
    let info = Alert::info("Info message");
    assert!(info.is_dismissed() == false);

    let success = Alert::success("Success message");
    assert!(success.is_dismissed() == false);

    let warning = Alert::warning("Warning message");
    assert!(warning.is_dismissed() == false);

    let error = Alert::error("Error message");
    assert!(error.is_dismissed() == false);
}

#[test]
fn test_alert_state_methods() {
    let mut a = Alert::new("Test");

    // Test initial state
    assert!(!a.is_dismissed());

    // Test dismiss
    a.dismiss();
    assert!(a.is_dismissed());

    // Test reset
    a.reset();
    assert!(!a.is_dismissed());
}

#[test]
fn test_alert_height_calculations() {
    // Test height calculations using only public methods
    let a1 = Alert::new("Message").variant(AlertVariant::Filled);
    assert_eq!(a1.height(), 3);

    let a2 = Alert::new("Message")
        .title("Title")
        .variant(AlertVariant::Filled);
    assert_eq!(a2.height(), 4);

    let a3 = Alert::new("Message").variant(AlertVariant::Minimal);
    assert_eq!(a3.height(), 1);

    let a4 = Alert::new("Message")
        .title("Title")
        .variant(AlertVariant::Minimal);
    assert_eq!(a4.height(), 2);
}

#[test]
fn test_alert_key_handling() {
    let mut a = Alert::new("Test").dismissible(true);

    // Test that non-dismiss keys don't work
    assert!(!a.handle_key(&Key::Char('a')));
    assert!(!a.is_dismissed());

    // Test dismiss keys
    assert!(a.handle_key(&Key::Char('x')));
    assert!(a.is_dismissed());

    // Reset for next test
    a.reset();
    assert!(!a.is_dismissed());

    assert!(a.handle_key(&Key::Char('X')));
    assert!(a.is_dismissed());

    // Reset for next test
    a.reset();
    assert!(!a.is_dismissed());

    assert!(a.handle_key(&Key::Escape));
    assert!(a.is_dismissed());
}

#[test]
fn test_alert_helper_functions() {
    // Test the public helper functions
    let a = alert("Helper message");
    assert!(a.is_dismissed() == false);
    assert_eq!(a.height(), 1); // Default is minimal with no title

    let i = Alert::info("Info");
    assert!(i.is_dismissed() == false);

    let s = Alert::success("Success");
    assert!(s.is_dismissed() == false);

    let w = Alert::warning("Warning");
    assert!(w.is_dismissed() == false);

    let e = Alert::error("Error");
    assert!(e.is_dismissed() == false);
}

#[test]
fn test_alert_default() {
    let a = Alert::default();
    assert!(!a.is_dismissed());
    assert_eq!(a.height(), 1); // "Alert" message with no title
}

#[test]
fn test_alert_level_icons() {
    // Test that we can access icons through public methods
    assert_eq!(AlertLevel::Info.icon(), 'ℹ');
    assert_eq!(AlertLevel::Success.icon(), '✓');
    assert_eq!(AlertLevel::Warning.icon(), '⚠');
    assert_eq!(AlertLevel::Error.icon(), '✗');
}

#[test]
fn test_alert_level_colors() {
    // Test color access through public methods
    let info_color = AlertLevel::Info.color();
    let success_color = AlertLevel::Success.color();
    let warning_color = AlertLevel::Warning.color();
    let error_color = AlertLevel::Error.color();

    // Just verify they return colors (don't compare specific values unless needed)
    assert!(info_color != success_color);
    assert!(success_color != warning_color);
    assert!(warning_color != error_color);
}

#[test]
fn test_alert_variant_default() {
    let variant = AlertVariant::default();
    assert_eq!(variant, AlertVariant::Filled);
}

#[test]
fn test_alert_custom_icon() {
    let a = Alert::new("Test").custom_icon('★');
    // The custom icon is used in rendering, but we can verify it doesn't break anything
    assert!(!a.is_dismissed());
    assert_eq!(a.height(), 1);
}
