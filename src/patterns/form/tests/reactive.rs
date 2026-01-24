//! Reactive validation and matches validator tests

use crate::patterns::form::FormState;

// Reactive behavior tests
#[test]
fn test_form_reactive_validation() {
    let form = FormState::new()
        .field("email", |f| f.email().required())
        .build();

    // Initially invalid (empty + required)
    assert!(!form.is_valid());

    // Set invalid email
    form.set_value("email", "invalid");
    assert!(!form.is_valid());

    // Set valid email
    form.set_value("email", "test@example.com");
    assert!(form.is_valid());

    // Change to invalid
    form.set_value("email", "bad");
    assert!(!form.is_valid());
}

// Matches validator tests
#[test]
fn test_matches_validator() {
    let form = FormState::new()
        .field("password", |f| f.password().required().min_length(8))
        .field("confirm", |f| f.password().required().matches("password"))
        .build();

    // Set password
    form.set_value("password", "secret123");

    // Confirm doesn't match
    form.set_value("confirm", "different");
    assert!(!form.is_valid());
    assert!(form.get("confirm").unwrap().has_errors());

    // Confirm matches
    form.set_value("confirm", "secret123");
    assert!(form.is_valid());
    assert!(!form.get("confirm").unwrap().has_errors());
}

#[test]
fn test_matches_validator_reactive() {
    let form = FormState::new()
        .field("password", |f| f.password())
        .field("confirm", |f| f.password().matches("password"))
        .build();

    // Both empty - should match
    assert!(form.is_valid());

    // Set both to same value
    form.set_value("password", "test123");
    form.set_value("confirm", "test123");
    assert!(form.is_valid());

    // Change password - confirm should now fail
    form.set_value("password", "changed");
    assert!(!form.is_valid());
    let confirm_errors = form.get("confirm").unwrap().errors();
    assert!(confirm_errors.iter().any(|e| e.message.contains("match")));

    // Update confirm to match
    form.set_value("confirm", "changed");
    assert!(form.is_valid());
}

#[test]
fn test_matches_nonexistent_field() {
    // If matches field doesn't exist, just build without match validation
    let form = FormState::new()
        .field("confirm", |f| f.matches("nonexistent"))
        .build();

    form.set_value("confirm", "anything");
    // Should be valid since target doesn't exist
    assert!(form.is_valid());
}
