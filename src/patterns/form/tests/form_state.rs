//! FormState tests

use crate::patterns::form::FormState;

// FormState construction tests
#[test]
fn test_form_state_new() {
    let form = FormState::new().build();
    assert!(form.field_names().is_empty());
    assert!(form.focused().is_none());
    assert!(!form.is_submitted());
}

#[test]
fn test_form_state_default() {
    let form = FormState::default();
    assert!(form.field_names().is_empty());
}

#[test]
fn test_form_state_builder_default() {
    let builder = FormState::new();
    let form = builder.build();
    assert!(form.is_valid());
}

// FormState field access tests
#[test]
fn test_form_state() {
    let form = FormState::new()
        .field("username", |f| f.required())
        .field("email", |f| f.email())
        .build();

    form.set_value("username", "john");
    form.set_value("email", "john@example.com");

    // Auto validates
    assert!(form.is_valid());
}

#[test]
fn test_form_state_get() {
    let form = FormState::new().field("name", |f| f.label("Name")).build();
    let field = form.get("name");
    assert!(field.is_some());
    assert_eq!(field.unwrap().label, "Name");
}

#[test]
fn test_form_state_get_none() {
    let form = FormState::new().build();
    assert!(form.get("nonexistent").is_none());
}

// FormState value manipulation tests
#[test]
fn test_form_state_set_value() {
    let form = FormState::new().field("name", |f| f).build();
    form.set_value("name", "modified");
    assert_eq!(form.value("name"), Some("modified".to_string()));
}

#[test]
fn test_form_state_value() {
    let form = FormState::new().field("name", |f| f).build();
    form.set_value("name", "test");
    assert_eq!(form.value("name"), Some("test".to_string()));
}

#[test]
fn test_form_state_value_none() {
    let form = FormState::new().build();
    assert!(form.value("nonexistent").is_none());
}

#[test]
fn test_form_state_set_value_marks_touched() {
    let form = FormState::new().field("name", |f| f).build();
    form.set_value("name", "test");
    assert!(form.get("name").unwrap().is_touched());
}

#[test]
fn test_form_state_set_value_nonexistent() {
    let form = FormState::new().build();
    form.set_value("nonexistent", "value"); // Should not panic
}

// FormState validation tests
#[test]
fn test_form_field_auto_validates() {
    let form = FormState::new().field("name", |f| f.required()).build();
    // Empty value fails required - auto validated
    assert!(!form.is_valid());
    form.set_value("name", "valid");
    assert!(form.is_valid());
}

#[test]
fn test_form_errors() {
    let form = FormState::new().field("username", |f| f.required()).build();

    // Empty value - auto validates with errors
    assert!(form.has_errors());
    let errors = form.errors();
    assert_eq!(errors.len(), 1);
}

#[test]
fn test_form_errors_multiple_fields() {
    let form = FormState::new()
        .field("a", |f| f.required())
        .field("b", |f| f.required())
        .build();
    // Both fields empty - auto validates
    let errors = form.errors();
    assert_eq!(errors.len(), 2);
}

// FormState focus tests
#[test]
fn test_form_focus() {
    let form = FormState::new()
        .field("a", |f| f)
        .field("b", |f| f)
        .field("c", |f| f)
        .build();

    form.focus("a");
    assert_eq!(form.focused(), Some("a".to_string()));

    form.focus_next();
    assert_eq!(form.focused(), Some("b".to_string()));

    form.focus_next();
    assert_eq!(form.focused(), Some("c".to_string()));

    form.focus_next(); // Wraps around
    assert_eq!(form.focused(), Some("a".to_string()));

    form.focus_prev();
    assert_eq!(form.focused(), Some("c".to_string()));
}

#[test]
fn test_form_focus_nonexistent() {
    let form = FormState::new().field("a", |f| f).build();
    form.focus("nonexistent");
    assert!(form.focused().is_none());
}

#[test]
fn test_form_focus_next_empty() {
    let form = FormState::new().build();
    form.focus_next(); // Should not panic
    assert!(form.focused().is_none());
}

#[test]
fn test_form_focus_prev_empty() {
    let form = FormState::new().build();
    form.focus_prev(); // Should not panic
    assert!(form.focused().is_none());
}

#[test]
fn test_form_focus_prev_wrap() {
    let form = FormState::new().field("a", |f| f).field("b", |f| f).build();
    form.focus("a");
    form.focus_prev();
    assert_eq!(form.focused(), Some("b".to_string()));
}

#[test]
fn test_form_blur() {
    let form = FormState::new().field("a", |f| f).build();
    form.focus("a");
    assert!(form.focused().is_some());
    form.blur();
    assert!(form.focused().is_none());
}

// FormState iteration tests
#[test]
fn test_form_field_names() {
    let form = FormState::new()
        .field("first", |f| f)
        .field("second", |f| f)
        .build();
    let names = form.field_names();
    assert_eq!(names, &["first", "second"]);
}

#[test]
fn test_form_iter() {
    let form = FormState::new()
        .field("a", |f| f.label("A"))
        .field("b", |f| f.label("B"))
        .build();
    let fields: Vec<_> = form.iter().collect();
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].0, "a");
    assert_eq!(fields[0].1.label, "A");
    assert_eq!(fields[1].0, "b");
    assert_eq!(fields[1].1.label, "B");
}

#[test]
fn test_form_values() {
    let form = FormState::new().field("a", |f| f).field("b", |f| f).build();
    form.set_value("a", "value_a");
    form.set_value("b", "value_b");
    let values = form.values();
    assert_eq!(values.get("a"), Some(&"value_a".to_string()));
    assert_eq!(values.get("b"), Some(&"value_b".to_string()));
}

#[test]
fn test_form_values_empty() {
    let form = FormState::new().build();
    let values = form.values();
    assert!(values.is_empty());
}

// FormState submit/reset tests
#[test]
fn test_form_submit() {
    let form = FormState::new().field("name", |f| f.required()).build();

    // Empty submission should fail
    assert!(!form.submit());
    assert!(form.is_submitted());

    // With value should succeed
    form.set_value("name", "John");
    assert!(form.submit());
}

#[test]
fn test_form_submit_touches_all_fields() {
    let form = FormState::new().field("a", |f| f).field("b", |f| f).build();
    assert!(!form.get("a").unwrap().is_touched());
    assert!(!form.get("b").unwrap().is_touched());
    form.submit();
    assert!(form.get("a").unwrap().is_touched());
    assert!(form.get("b").unwrap().is_touched());
}

#[test]
fn test_form_reset() {
    let form = FormState::new().field("name", |f| f).build();

    form.set_value("name", "John");
    form.submit();

    form.reset();
    assert_eq!(form.value("name"), Some("".to_string()));
    assert!(!form.is_submitted());
}

#[test]
fn test_form_reset_clears_touched() {
    let form = FormState::new().field("name", |f| f).build();
    form.set_value("name", "value");
    assert!(form.get("name").unwrap().is_touched());
    form.reset();
    assert!(!form.get("name").unwrap().is_touched());
}

#[test]
fn test_form_reset_clears_errors() {
    let form = FormState::new().field("name", |f| f.required()).build();
    // Has errors initially (required + empty)
    assert!(!form.get("name").unwrap().errors().is_empty());
    // Set valid value then reset
    form.set_value("name", "valid");
    form.reset();
    // After reset, empty again = has errors
    assert!(!form.get("name").unwrap().errors().is_empty());
}

// Edge case tests
#[test]
fn test_form_state_whitespace_only_required() {
    // Whitespace-only strings should fail required validation
    let form = FormState::new().field("name", |f| f.required()).build();
    form.set_value("name", "   ");
    assert!(!form.is_valid());
}

#[test]
fn test_form_state_whitespace_only_min_length() {
    // Whitespace-only strings should fail min_length validation
    let form = FormState::new().field("name", |f| f.min_length(3)).build();
    form.set_value("name", "  "); // Only spaces
    assert!(!form.is_valid());
}

#[test]
fn test_form_state_empty_string_required() {
    // Empty string should fail required validation
    let form = FormState::new().field("name", |f| f.required()).build();
    form.set_value("name", "");
    assert!(!form.is_valid());
}

#[test]
fn test_form_state_trimming_whitespace() {
    // String with leading/trailing whitespace should be trimmed
    let form = FormState::new().field("name", |f| f.min_length(3)).build();
    form.set_value("name", "  abc  "); // Should trim to "abc" (length 3)
    assert!(form.is_valid());
}

#[test]
fn test_form_state_max_length_boundary() {
    // Test exact boundary condition for max_length
    let form = FormState::new().field("name", |f| f.max_length(5)).build();

    // Exactly at boundary should pass
    form.set_value("name", "12345");
    assert!(form.is_valid());

    // One over boundary should fail
    form.set_value("name", "123456");
    assert!(!form.is_valid());
}

#[test]
fn test_form_state_min_length_boundary() {
    // Test exact boundary condition for min_length
    let form = FormState::new().field("name", |f| f.min_length(3)).build();

    // One under boundary should fail
    form.set_value("name", "12");
    assert!(!form.is_valid());

    // Exactly at boundary should pass
    form.set_value("name", "123");
    assert!(form.is_valid());
}

#[test]
fn test_form_state_value_range_boundaries() {
    // Test numeric value range boundaries
    let form = FormState::new()
        .field("count", |f| f.number().min(1.0).max(10.0))
        .build();

    // Below minimum
    form.set_value("count", "0");
    assert!(!form.is_valid());

    // At minimum
    form.set_value("count", "1");
    assert!(form.is_valid());

    // At maximum
    form.set_value("count", "10");
    assert!(form.is_valid());

    // Above maximum
    form.set_value("count", "11");
    assert!(!form.is_valid());
}

#[test]
fn test_form_state_unicode_emoji() {
    // Form should handle emoji correctly
    let form = FormState::new().field("emoji", |f| f.min_length(1)).build();
    form.set_value("emoji", "😀"); // Single emoji
    assert!(form.is_valid());
}

#[test]
fn test_form_state_unicode_combining_characters() {
    // Form should handle combining characters
    let form = FormState::new()
        .field("accented", |f| f.min_length(1))
        .build();
    form.set_value("accented", "é"); // 'e' + combining acute
    assert!(form.is_valid());
}

#[test]
fn test_form_state_special_characters_email() {
    // Email validation should handle edge cases
    let form = FormState::new().field("email", |f| f.email()).build();

    // Valid email with special characters
    form.set_value("email", "user+test@example.com");
    assert!(form.is_valid());

    // Invalid: no @
    form.set_value("email", "userexample.com");
    assert!(!form.is_valid());

    // Invalid: no domain
    form.set_value("email", "user@");
    assert!(!form.is_valid());
}

#[test]
fn test_form_state_numeric_with_special_chars() {
    // Numeric validation should reject special characters
    let form = FormState::new().field("number", |f| f.number()).build();

    // Valid numbers
    form.set_value("number", "123");
    assert!(form.is_valid());

    form.set_value("number", "-123"); // Negative
    assert!(form.is_valid());

    form.set_value("number", "12.5"); // Decimal
    assert!(form.is_valid());

    // Invalid: letters
    form.set_value("number", "12a3");
    assert!(!form.is_valid());

    // Invalid: special characters
    form.set_value("number", "12$3");
    assert!(!form.is_valid());
}

#[test]
fn test_form_state_multiple_fields_validation() {
    // Test form with multiple dependent fields
    let form = FormState::new()
        .field("password", |f| f.min_length(8).label("Password"))
        .field("confirm", |f| f.matches("password").label("Confirm"))
        .build();

    // Set matching values
    form.set_value("password", "secure123");
    form.set_value("confirm", "secure123");
    assert!(form.is_valid());

    // Set non-matching values
    form.set_value("password", "secure123");
    form.set_value("confirm", "different");
    assert!(!form.is_valid());
}

#[test]
fn test_form_state_email_with_subdomain() {
    // Email should handle subdomains correctly
    let form = FormState::new().field("email", |f| f.email()).build();

    form.set_value("email", "user@mail.example.com");
    assert!(form.is_valid());
}

#[test]
fn test_form_state_email_with_hyphenated_local() {
    // Email should handle hyphenated local parts
    let form = FormState::new().field("email", |f| f.email()).build();

    form.set_value("email", "user-name@example.com");
    assert!(form.is_valid());
}
