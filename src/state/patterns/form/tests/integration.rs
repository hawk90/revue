//! Integration tests for complete form workflows

use crate::patterns::form::FormState;

#[test]
fn test_complete_form_workflow() {
    let form = FormState::new()
        .field("username", |f| {
            f.label("Username").required().min_length(3).max_length(20)
        })
        .field("email", |f| f.email().label("Email").required())
        .field("age", |f| f.integer().label("Age").min(0.0).max(150.0))
        .build();

    // Initial state - has errors due to required fields
    assert!(!form.is_submitted());
    assert!(form.has_errors());

    // Set some values
    form.set_value("username", "john");
    form.set_value("email", "john@example.com");
    form.set_value("age", "25");

    // Now valid (auto-computed)
    assert!(form.is_valid());

    // Submit
    assert!(form.submit());

    // Get values
    let values = form.values();
    assert_eq!(values.get("username"), Some(&"john".to_string()));
    assert_eq!(values.get("email"), Some(&"john@example.com".to_string()));
    assert_eq!(values.get("age"), Some(&"25".to_string()));
}

#[test]
fn test_form_with_invalid_data() {
    let form = FormState::new()
        .field("username", |f| f.required().min_length(5))
        .field("email", |f| f.email())
        .build();

    form.set_value("username", "ab"); // too short
    form.set_value("email", "invalid"); // no @ or .

    assert!(!form.submit());
    assert!(form.has_errors());
    let errors = form.errors();
    assert_eq!(errors.len(), 2);
}
