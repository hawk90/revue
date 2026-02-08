//! Validation tests for input widgets

use revue::widget::input;

// =========================================================================
// Basic validation tests
// =========================================================================

#[test]
fn test_input_validate_always_true() {
    let i = input().validate(|_| true);
    assert!(i.validator.is_some());
}

#[test]
fn test_input_validate_always_false() {
    let i = input().validate(|_| false);
    assert!(i.validator.is_some());
}

#[test]
fn test_input_validate_with_condition() {
    let i = input().validate(|s| s.len() >= 3);
    assert!(i.validator.is_some());
}

#[test]
fn test_input_validate_non_empty() {
    let i = input().validate(|s| !s.is_empty());
    assert!(i.validator.is_some());
}

#[test]
fn test_input_validate_max_length() {
    let i = input().validate(|s| s.len() <= 10);
    assert!(i.validator.is_some());
}

#[test]
fn test_input_validate_with_regex_like() {
    let i = input().validate(|s| s.chars().all(|c| c.is_ascii_digit()));
    assert!(i.validator.is_some());
}

#[test]
fn test_input_validate_email_like() {
    let i = input().validate(|s| s.contains('@') && s.contains('.'));
    assert!(i.validator.is_some());
}

#[test]
fn test_input_validate_with_value() {
    let i = input().value("test").validate(|s| s == "test");
    assert!(i.validator.is_some());
}

#[test]
fn test_input_validate_chaining() {
    let i = input()
        .placeholder("Enter email")
        .validate(|s| s.contains('@'))
        .value("test@example.com");
    assert!(i.validator.is_some());
    assert_eq!(i.text(), "test@example.com");
}

#[test]
fn test_input_multiple_validate_calls() {
    let i = input()
        .validate(|_| true)
        .validate(|_| false);
    // Last validator should win
    assert!(i.validator.is_some());
}

#[test]
fn test_input_validate_with_complex_logic() {
    let i = input().validate(|s| {
        s.len() >= 3 && s.len() <= 20 && s.chars().all(|c| c.is_alphanumeric())
    });
    assert!(i.validator.is_some());
}

#[test]
fn test_input_validate_closure_captures_variable() {
    let min_length = 5;
    let i = input().validate(move |s| s.len() >= min_length);
    assert!(i.validator.is_some());
}

#[test]
fn test_input_validate_with_string_pattern() {
    let pattern = "hello";
    let i = input().validate(move |s| s.starts_with(pattern));
    assert!(i.validator.is_some());
}

