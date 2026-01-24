//! Validator tests

use crate::patterns::form::{ValidationError, Validators};

// Required validator tests
#[test]
fn test_required_validator() {
    let validator = Validators::required();
    assert!(validator("hello").is_ok());
    assert!(validator("").is_err());
    assert!(validator("   ").is_err());
}

#[test]
fn test_required_validator_error_message() {
    let validator = Validators::required();
    let err = validator("").unwrap_err();
    assert!(err.message.contains("required"));
}

// Email validator tests
#[test]
fn test_email_validator() {
    let validator = Validators::email();
    assert!(validator("test@example.com").is_ok());
    assert!(validator("invalid").is_err());
    assert!(validator("").is_ok()); // Empty is ok (use required for that)
}

#[test]
fn test_email_validator_error_message() {
    let validator = Validators::email();
    let err = validator("invalid").unwrap_err();
    assert!(err.message.contains("email"));
}

#[test]
fn test_email_validator_edge_cases() {
    let validator = Validators::email();
    assert!(validator("a@b.c").is_ok()); // minimal valid
    assert!(validator("user.name+tag@example.co.uk").is_ok());
    assert!(validator("missing@dot").is_err()); // no dot
    assert!(validator("missing.at.com").is_err()); // no @
}

// Min length validator tests
#[test]
fn test_min_length() {
    let validator = Validators::min_length(3);
    assert!(validator("abc").is_ok());
    assert!(validator("ab").is_err());
}

#[test]
fn test_min_length_zero() {
    let validator = Validators::min_length(0);
    assert!(validator("").is_ok());
    assert!(validator("a").is_ok());
}

#[test]
fn test_min_length_error_message() {
    let validator = Validators::min_length(5);
    let err = validator("abc").unwrap_err();
    assert!(err.message.contains("5"));
    assert!(err.message.contains("at least"));
}

// Max length validator tests
#[test]
fn test_max_length() {
    let validator = Validators::max_length(5);
    assert!(validator("abc").is_ok());
    assert!(validator("abcde").is_ok());
    assert!(validator("abcdef").is_err());
}

#[test]
fn test_max_length_zero() {
    let validator = Validators::max_length(0);
    assert!(validator("").is_ok());
    assert!(validator("a").is_err());
}

#[test]
fn test_max_length_error_message() {
    let validator = Validators::max_length(3);
    let err = validator("abcde").unwrap_err();
    assert!(err.message.contains("3"));
    assert!(err.message.contains("at most"));
}

// Numeric validator tests
#[test]
fn test_numeric_validator() {
    let validator = Validators::numeric();
    assert!(validator("123").is_ok());
    assert!(validator("12.5").is_ok());
    assert!(validator("-5").is_ok());
    assert!(validator("abc").is_err());
}

#[test]
fn test_numeric_empty() {
    let validator = Validators::numeric();
    assert!(validator("").is_ok());
}

#[test]
fn test_numeric_error_message() {
    let validator = Validators::numeric();
    let err = validator("not a number").unwrap_err();
    assert!(err.message.contains("number"));
}

// Integer validator tests
#[test]
fn test_integer_validator() {
    let validator = Validators::integer();
    assert!(validator("123").is_ok());
    assert!(validator("-456").is_ok());
    assert!(validator("12.5").is_err()); // floats not allowed
    assert!(validator("abc").is_err());
}

#[test]
fn test_integer_empty() {
    let validator = Validators::integer();
    assert!(validator("").is_ok());
}

#[test]
fn test_integer_error_message() {
    let validator = Validators::integer();
    let err = validator("3.14").unwrap_err();
    assert!(err.message.contains("integer"));
}

// Min value validator tests
#[test]
fn test_min_value() {
    let validator = Validators::min_value(10.0);
    assert!(validator("15").is_ok());
    assert!(validator("10").is_ok());
    assert!(validator("5").is_err());
}

#[test]
fn test_min_value_empty() {
    let validator = Validators::min_value(10.0);
    assert!(validator("").is_ok());
}

#[test]
fn test_min_value_not_a_number() {
    let validator = Validators::min_value(10.0);
    let err = validator("abc").unwrap_err();
    assert!(err.message.contains("number"));
}

#[test]
fn test_min_value_error_message() {
    let validator = Validators::min_value(100.0);
    let err = validator("50").unwrap_err();
    assert!(err.message.contains("100"));
    assert!(err.message.contains("at least"));
}

// Max value validator tests
#[test]
fn test_max_value() {
    let validator = Validators::max_value(10.0);
    assert!(validator("5").is_ok());
    assert!(validator("10").is_ok());
    assert!(validator("15").is_err());
}

#[test]
fn test_max_value_empty() {
    let validator = Validators::max_value(10.0);
    assert!(validator("").is_ok());
}

#[test]
fn test_max_value_not_a_number() {
    let validator = Validators::max_value(10.0);
    let err = validator("abc").unwrap_err();
    assert!(err.message.contains("number"));
}

#[test]
fn test_max_value_error_message() {
    let validator = Validators::max_value(50.0);
    let err = validator("100").unwrap_err();
    assert!(err.message.contains("50"));
    assert!(err.message.contains("at most"));
}

// Contains validator tests
#[test]
fn test_contains_validator() {
    let validator = Validators::contains("@", "Must contain @");
    assert!(validator("test@example").is_ok());
    assert!(validator("test").is_err());
}

#[test]
fn test_contains_empty() {
    let validator = Validators::contains("@", "error");
    assert!(validator("").is_ok());
}

#[test]
fn test_contains_error_message() {
    let validator = Validators::contains("xyz", "Custom error message");
    let err = validator("abc").unwrap_err();
    assert_eq!(err.message, "Custom error message");
}

// Alphanumeric validator tests
#[test]
fn test_alphanumeric_validator() {
    let validator = Validators::alphanumeric();
    assert!(validator("abc123").is_ok());
    assert!(validator("ABC").is_ok());
    assert!(validator("123").is_ok());
    assert!(validator("abc-123").is_err());
    assert!(validator("abc 123").is_err());
}

#[test]
fn test_alphanumeric_empty() {
    let validator = Validators::alphanumeric();
    assert!(validator("").is_ok());
}

#[test]
fn test_alphanumeric_error_message() {
    let validator = Validators::alphanumeric();
    let err = validator("hello!").unwrap_err();
    assert!(err.message.contains("letters and numbers"));
}

// No whitespace validator tests
#[test]
fn test_no_whitespace_validator() {
    let validator = Validators::no_whitespace();
    assert!(validator("nospaces").is_ok());
    assert!(validator("has space").is_err());
    assert!(validator("has\ttab").is_err());
    assert!(validator("has\nnewline").is_err());
}

#[test]
fn test_no_whitespace_empty() {
    let validator = Validators::no_whitespace();
    assert!(validator("").is_ok());
}

#[test]
fn test_no_whitespace_error_message() {
    let validator = Validators::no_whitespace();
    let err = validator("with space").unwrap_err();
    assert!(err.message.contains("whitespace"));
}

// Custom validator tests
#[test]
fn test_custom_validator() {
    let validator = Validators::custom(|value| {
        if value.starts_with("ok") {
            Ok(())
        } else {
            Err(ValidationError::new("Must start with 'ok'"))
        }
    });
    assert!(validator("ok_value").is_ok());
    assert!(validator("bad_value").is_err());
}

#[test]
fn test_custom_validator_error() {
    let validator = Validators::custom(|_| Err(ValidationError::new("always fails")));
    let err = validator("anything").unwrap_err();
    assert_eq!(err.message, "always fails");
}
