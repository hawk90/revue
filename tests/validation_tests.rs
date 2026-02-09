//! Integration tests for validation utilities
//! Extracted from src/utils/validation.rs

use revue::utils::validation::*;

#[test]
fn test_required() {
    let v = required();
    assert!(v("hello").is_ok());
    assert!(v("  hello  ").is_ok());
    assert!(v("").is_err());
    assert!(v("   ").is_err());
}

#[test]
fn test_min_length() {
    let v = min_length(5);
    assert!(v("hello").is_ok());
    assert!(v("hi").is_err());
    assert!(v("").is_err());
}

#[test]
fn test_max_length() {
    let v = max_length(5);
    assert!(v("hi").is_ok());
    assert!(v("hello").is_ok());
    assert!(v("hello!").is_err());
}

#[test]
fn test_email() {
    let v = email();
    assert!(v("user@example.com").is_ok());
    assert!(v("test@test.org").is_ok());
    assert!(v("invalid").is_err());
    assert!(v("@example.com").is_err());
    assert!(v("user@").is_err());
    assert!(v("user@domain").is_err());
    assert!(v("").is_ok()); // Empty is ok, use required for mandatory
}

#[test]
fn test_url() {
    let v = url();
    assert!(v("https://example.com").is_ok());
    assert!(v("http://test.org/path").is_ok());
    assert!(v("ftp://invalid").is_err());
    assert!(v("example.com").is_err());
    assert!(v("").is_ok());
}

#[test]
fn test_numeric() {
    let v = numeric();
    assert!(v("123").is_ok());
    assert!(v("0").is_ok());
    assert!(v("abc").is_err());
    assert!(v("12a").is_err());
}

#[test]
fn test_min_value() {
    let v = min_value(10);
    assert!(v("10").is_ok());
    assert!(v("100").is_ok());
    assert!(v("5").is_err());
    assert!(v("abc").is_err());
}

#[test]
fn test_max_value() {
    let v = max_value(10);
    assert!(v("5").is_ok());
    assert!(v("10").is_ok());
    assert!(v("15").is_err());
}

#[test]
fn test_value_range() {
    let v = value_range(1, 100);
    assert!(v("50").is_ok());
    assert!(v("1").is_ok());
    assert!(v("100").is_ok());
    assert!(v("0").is_err());
    assert!(v("101").is_err());
}

#[test]
fn test_one_of() {
    let v = one_of(&["red", "green", "blue"]);
    assert!(v("red").is_ok());
    assert!(v("green").is_ok());
    assert!(v("yellow").is_err());
    assert!(v("").is_ok());
}

#[test]
fn test_all_of() {
    let v = all_of(vec![required(), min_length(3), max_length(10)]);
    assert!(v("hello").is_ok());
    assert!(v("hi").is_err()); // Too short
    assert!(v("").is_err()); // Required
    assert!(v("very long text here").is_err()); // Too long
}

#[test]
fn test_any_of() {
    let v = any_of(vec![email(), url()]);
    assert!(v("user@example.com").is_ok());
    assert!(v("https://example.com").is_ok());
    assert!(v("invalid").is_err());
}

#[test]
fn test_custom() {
    let v = custom(|s| s.starts_with("test"), "Must start with 'test'");
    assert!(v("testing").is_ok());
    assert!(v("hello").is_err());
}

#[test]
fn test_form_validator() {
    let validator = FormValidator::new()
        .field("email", vec![required(), email()])
        .field("password", vec![required(), min_length(8)]);

    let result = validator.validate(&[("email", "user@example.com"), ("password", "secret123")]);
    assert!(result.is_ok());

    let result = validator.validate(&[("email", "invalid"), ("password", "short")]);
    assert!(result.is_err());

    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 2);
}
