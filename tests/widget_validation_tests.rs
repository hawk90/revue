//! Tests for widget validation (ValidationError and validators module)

use revue::widget::{validators, Validatable, ValidationError, ValidationResult};

// ============================================================
// ValidationError — new()
// ============================================================

#[test]
fn validation_error_new_custom() {
    let err = ValidationError::new("custom message", "CUSTOM");
    assert_eq!(err.message, "custom message");
    assert_eq!(err.code, "CUSTOM");
}

#[test]
fn validation_error_new_from_string() {
    let err = ValidationError::new(String::from("owned message"), "CODE");
    assert_eq!(err.message, "owned message");
}

// ============================================================
// ValidationError — factory methods
// ============================================================

#[test]
fn validation_error_required() {
    let err = ValidationError::required("Name");
    assert_eq!(err.message, "Name is required");
    assert_eq!(err.code, "REQUIRED");
}

#[test]
fn validation_error_min_length() {
    let err = ValidationError::min_length("Password", 8);
    assert_eq!(err.message, "Password must be at least 8 characters");
    assert_eq!(err.code, "MIN_LENGTH");
}

#[test]
fn validation_error_max_length() {
    let err = ValidationError::max_length("Username", 20);
    assert_eq!(err.message, "Username must be at most 20 characters");
    assert_eq!(err.code, "MAX_LENGTH");
}

#[test]
fn validation_error_pattern() {
    let err = ValidationError::pattern("Phone", r"\d{3}-\d{4}");
    assert_eq!(err.message, r"Phone must match pattern: \d{3}-\d{4}");
    assert_eq!(err.code, "PATTERN");
}

#[test]
fn validation_error_range() {
    let err = ValidationError::range("Age", 0, 150);
    assert_eq!(err.message, "Age must be between 0 and 150");
    assert_eq!(err.code, "RANGE");
}

#[test]
fn validation_error_email() {
    let err = ValidationError::email("notanemail");
    assert_eq!(err.message, "'notanemail' is not a valid email address");
    assert_eq!(err.code, "EMAIL");
}

// ============================================================
// ValidationError — Display and Error traits
// ============================================================

#[test]
fn validation_error_display() {
    let err = ValidationError::required("Email");
    assert_eq!(format!("{}", err), "Email is required");
}

#[test]
fn validation_error_is_std_error() {
    let err = ValidationError::new("test", "TEST");
    let _: &dyn std::error::Error = &err;
}

// ============================================================
// validators::require
// ============================================================

#[test]
fn require_empty_returns_err() {
    let result = validators::require(&"", "Name");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, "REQUIRED");
}

#[test]
fn require_non_empty_returns_ok() {
    let result = validators::require(&"John", "Name");
    assert!(result.is_ok());
}

#[test]
fn require_whitespace_only_returns_ok() {
    // whitespace is not considered empty by to_string().is_empty()
    let result = validators::require(&" ", "Name");
    assert!(result.is_ok());
}

// ============================================================
// validators::min_length
// ============================================================

#[test]
fn min_length_below_minimum_returns_err() {
    let result = validators::min_length("ab", 3, "Password");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, "MIN_LENGTH");
}

#[test]
fn min_length_at_minimum_returns_ok() {
    let result = validators::min_length("abc", 3, "Password");
    assert!(result.is_ok());
}

#[test]
fn min_length_above_minimum_returns_ok() {
    let result = validators::min_length("abcdef", 3, "Password");
    assert!(result.is_ok());
}

#[test]
fn min_length_empty_with_zero_min_returns_ok() {
    let result = validators::min_length("", 0, "Field");
    assert!(result.is_ok());
}

// ============================================================
// validators::max_length
// ============================================================

#[test]
fn max_length_above_maximum_returns_err() {
    let result = validators::max_length("abcdef", 3, "Username");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, "MAX_LENGTH");
}

#[test]
fn max_length_at_maximum_returns_ok() {
    let result = validators::max_length("abc", 3, "Username");
    assert!(result.is_ok());
}

#[test]
fn max_length_below_maximum_returns_ok() {
    let result = validators::max_length("ab", 3, "Username");
    assert!(result.is_ok());
}

// ============================================================
// validators::email
// ============================================================

#[test]
fn email_valid_returns_ok() {
    let result = validators::email("user@example.com");
    assert!(result.is_ok());
}

#[test]
fn email_missing_at_returns_err() {
    let result = validators::email("userexample.com");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, "EMAIL");
}

#[test]
fn email_missing_dot_returns_err() {
    let result = validators::email("user@examplecom");
    assert!(result.is_err());
}

#[test]
fn email_empty_returns_err() {
    let result = validators::email("");
    assert!(result.is_err());
}

#[test]
fn email_at_and_dot_present_returns_ok() {
    let result = validators::email("a@b.c");
    assert!(result.is_ok());
}

// ============================================================
// validators::range
// ============================================================

#[test]
fn range_below_min_returns_err() {
    let result = validators::range(5, 10, 100, "Age");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, "RANGE");
}

#[test]
fn range_above_max_returns_err() {
    let result = validators::range(200, 10, 100, "Age");
    assert!(result.is_err());
}

#[test]
fn range_within_returns_ok() {
    let result = validators::range(50, 10, 100, "Age");
    assert!(result.is_ok());
}

#[test]
fn range_at_min_boundary_returns_ok() {
    let result = validators::range(10, 10, 100, "Age");
    assert!(result.is_ok());
}

#[test]
fn range_at_max_boundary_returns_ok() {
    let result = validators::range(100, 10, 100, "Age");
    assert!(result.is_ok());
}

#[test]
fn range_with_floats() {
    let result = validators::range(3.14, 0.0, 10.0, "Value");
    assert!(result.is_ok());
}

// ============================================================
// validators::pattern
// ============================================================

#[test]
fn pattern_match_returns_ok() {
    let result = validators::pattern("hello world", "hello", "Text");
    assert!(result.is_ok());
}

#[test]
fn pattern_no_match_returns_err() {
    let result = validators::pattern("goodbye", "hello", "Text");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, "PATTERN");
}

#[test]
fn pattern_empty_pattern_always_matches() {
    let result = validators::pattern("anything", "", "Text");
    assert!(result.is_ok());
}

// ============================================================
// validators::custom
// ============================================================

#[test]
fn custom_predicate_true_returns_ok() {
    let result = validators::custom(
        &42,
        |v| *v > 0,
        || ValidationError::new("must be positive", "POSITIVE"),
    );
    assert!(result.is_ok());
}

#[test]
fn custom_predicate_false_returns_err() {
    let result = validators::custom(
        &-1,
        |v| *v > 0,
        || ValidationError::new("must be positive", "POSITIVE"),
    );
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, "POSITIVE");
}

#[test]
fn custom_with_string_validation() {
    let result = validators::custom(
        &"test@example.com".to_string(),
        |v| v.contains('@') && v.len() > 5,
        || ValidationError::new("invalid email", "EMAIL"),
    );
    assert!(result.is_ok());
}

// ============================================================
// Validatable trait
// ============================================================

/// Marker type to satisfy `Validatable::Error` bound (requires std::error::Error).
/// The trait signature uses `ValidationResult<Self::Error>` = `Result<Self::Error, ValidationError>`,
/// so the associated type is the *success* payload.
#[derive(Debug)]
struct ValidOk;
impl std::fmt::Display for ValidOk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "valid")
    }
}
impl std::error::Error for ValidOk {}

struct EmailInput {
    value: String,
}

impl Validatable for EmailInput {
    type Error = ValidOk;

    fn validate(&self) -> ValidationResult<Self::Error> {
        if self.value.is_empty() {
            return Err(ValidationError::required("Email"));
        }
        if !self.value.contains('@') {
            return Err(ValidationError::email(&self.value));
        }
        Ok(ValidOk)
    }
}

#[test]
fn validatable_validate_valid_input() {
    let input = EmailInput {
        value: "user@example.com".to_string(),
    };
    assert!(input.validate().is_ok());
}

#[test]
fn validatable_validate_empty_returns_required_error() {
    let input = EmailInput {
        value: String::new(),
    };
    let err = input.validate().unwrap_err();
    assert_eq!(err.code, "REQUIRED");
}

#[test]
fn validatable_validate_missing_at_returns_email_error() {
    let input = EmailInput {
        value: "nope".to_string(),
    };
    let err = input.validate().unwrap_err();
    assert_eq!(err.code, "EMAIL");
}

#[test]
fn validatable_is_valid_delegates_to_validate() {
    let valid = EmailInput {
        value: "a@b.c".to_string(),
    };
    let invalid = EmailInput {
        value: String::new(),
    };
    assert!(valid.is_valid());
    assert!(!invalid.is_valid());
}

// ============================================================
// ValidationError — clone
// ============================================================

#[test]
fn validation_error_clone() {
    let err = ValidationError::new("test message", "TEST");
    let cloned = err.clone();
    assert_eq!(err, cloned);
}

// ============================================================
// ValidationError — equality
// ============================================================

#[test]
fn validation_error_eq() {
    let a = ValidationError::new("msg", "CODE");
    let b = ValidationError::new("msg", "CODE");
    assert_eq!(a, b);
}

#[test]
fn validation_error_ne_different_message() {
    let a = ValidationError::new("msg1", "CODE");
    let b = ValidationError::new("msg2", "CODE");
    assert_ne!(a, b);
}

#[test]
fn validation_error_ne_different_code() {
    let a = ValidationError::new("msg", "CODE1");
    let b = ValidationError::new("msg", "CODE2");
    assert_ne!(a, b);
}
