//! Validation module integration tests

use revue::widget::validation::{validators, Validatable, ValidationError, ValidationResult};
use std::fmt::Display;

// ==================== ValidationError Tests ====================

#[test]
fn test_validation_error_new() {
    let err = ValidationError::new("Custom error message", "CUSTOM_CODE");
    assert_eq!(err.message, "Custom error message");
    assert_eq!(err.code, "CUSTOM_CODE");
}

#[test]
fn test_validation_error_required() {
    let err = ValidationError::required("Email");
    assert_eq!(err.message, "Email is required");
    assert_eq!(err.code, "REQUIRED");
}

#[test]
fn test_validation_error_required_various_fields() {
    assert_eq!(
        ValidationError::required("Username").message,
        "Username is required"
    );
    assert_eq!(
        ValidationError::required("Password").message,
        "Password is required"
    );
    assert_eq!(
        ValidationError::required("Address").message,
        "Address is required"
    );
}

#[test]
fn test_validation_error_min_length() {
    let err = ValidationError::min_length("Password", 8);
    assert_eq!(err.message, "Password must be at least 8 characters");
    assert_eq!(err.code, "MIN_LENGTH");
}

#[test]
fn test_validation_error_min_length_various() {
    assert_eq!(
        ValidationError::min_length("Username", 3).message,
        "Username must be at least 3 characters"
    );
    assert_eq!(
        ValidationError::min_length("Code", 10).message,
        "Code must be at least 10 characters"
    );
}

#[test]
fn test_validation_error_max_length() {
    let err = ValidationError::max_length("Username", 20);
    assert_eq!(err.message, "Username must be at most 20 characters");
    assert_eq!(err.code, "MAX_LENGTH");
}

#[test]
fn test_validation_error_max_length_various() {
    assert_eq!(
        ValidationError::max_length("Name", 50).message,
        "Name must be at most 50 characters"
    );
    assert_eq!(
        ValidationError::max_length("Bio", 500).message,
        "Bio must be at most 500 characters"
    );
}

#[test]
fn test_validation_error_pattern() {
    let err = ValidationError::pattern("Password", "[A-Z]");
    assert_eq!(err.message, "Password must match pattern: [A-Z]");
    assert_eq!(err.code, "PATTERN");
}

#[test]
fn test_validation_error_pattern_various() {
    assert_eq!(
        ValidationError::pattern("Email", "@").message,
        "Email must match pattern: @"
    );
    assert_eq!(
        ValidationError::pattern("Phone", "[0-9]").message,
        "Phone must match pattern: [0-9]"
    );
}

#[test]
fn test_validation_error_range() {
    let err = ValidationError::range(150, 1, 120);
    assert_eq!(err.message, "150 must be between 1 and 120");
    assert_eq!(err.code, "RANGE");
}

#[test]
fn test_validation_error_range_various() {
    assert_eq!(
        ValidationError::range(0, 1, 10).message,
        "0 must be between 1 and 10"
    );
    assert_eq!(
        ValidationError::range(-5, 0, 100).message,
        "-5 must be between 0 and 100"
    );
    assert_eq!(
        ValidationError::range("abc", "a", "z").message,
        "abc must be between a and z"
    );
}

#[test]
fn test_validation_error_email() {
    let err = ValidationError::email("invalid-email");
    assert_eq!(err.message, "'invalid-email' is not a valid email address");
    assert_eq!(err.code, "EMAIL");
}

#[test]
fn test_validation_error_email_various() {
    assert_eq!(
        ValidationError::email("test").message,
        "'test' is not a valid email address"
    );
    assert_eq!(
        ValidationError::email("@example.com").message,
        "'@example.com' is not a valid email address"
    );
}

#[test]
fn test_validation_error_display() {
    let err = ValidationError::required("Field");
    assert_eq!(format!("{}", err), "Field is required");
}

#[test]
fn test_validation_error_clone() {
    let err1 = ValidationError::required("Email");
    let err2 = err1.clone();
    assert_eq!(err1.message, err2.message);
    assert_eq!(err1.code, err2.code);
}

#[test]
fn test_validation_error_partial_eq() {
    let err1 = ValidationError::required("Email");
    let err2 = ValidationError::required("Email");
    assert_eq!(err1, err2);

    let err3 = ValidationError::required("Password");
    assert_ne!(err1, err3);
}

// ==================== Validatable Trait Tests ====================

struct TestValidatable {
    value: String,
    should_fail: bool,
}

impl Validatable for TestValidatable {
    type Error = ValidationError;

    fn validate(&self) -> ValidationResult<Self::Error> {
        if self.should_fail {
            Err(ValidationError::required("TestField"))
        } else if self.value.is_empty() {
            Err(ValidationError::required("Value"))
        } else {
            // Return a dummy ValidationError for success
            Ok(ValidationError::new("", "OK"))
        }
    }
}

#[test]
fn test_validatable_trait_success() {
    let v = TestValidatable {
        value: "valid".to_string(),
        should_fail: false,
    };
    assert!(v.is_valid());
    assert!(v.validate().is_ok());
}

#[test]
fn test_validatable_trait_failure_empty() {
    let v = TestValidatable {
        value: "".to_string(),
        should_fail: false,
    };
    assert!(!v.is_valid());
    assert!(v.validate().is_err());
}

#[test]
fn test_validatable_trait_failure_custom() {
    let v = TestValidatable {
        value: "value".to_string(),
        should_fail: true,
    };
    assert!(!v.is_valid());
    assert!(v.validate().is_err());
}

// ==================== Validators::require Tests ====================

#[test]
fn test_validator_require_empty_string() {
    let result = validators::require(&"", "Field");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, "REQUIRED");
}

#[test]
fn test_validator_require_whitespace_only() {
    let result = validators::require(&"   ", "Field");
    // Non-whitespace characters are detected
    // The implementation uses to_string().is_empty(), so whitespace is NOT empty
    assert!(result.is_ok());
}

#[test]
fn test_validator_require_valid_string() {
    assert!(validators::require(&"value", "Field").is_ok());
    assert!(validators::require(&"test value", "Field").is_ok());
}

#[test]
fn test_validator_require_numeric_types() {
    assert!(validators::require(&42, "Count").is_ok());
    assert!(validators::require(&0, "Zero").is_ok());
    assert!(validators::require(&3.14, "Pi").is_ok());
}

#[test]
fn test_validator_require_various_field_names() {
    let result1 = validators::require(&"", "Email");
    assert_eq!(result1.unwrap_err().message, "Email is required");

    let result2 = validators::require(&"", "Password");
    assert_eq!(result2.unwrap_err().message, "Password is required");
}

// ==================== Validators::min_length Test ====================

#[test]
fn test_validator_min_length_too_short() {
    let result = validators::min_length("ab", 3, "Field");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, "MIN_LENGTH");
}

#[test]
fn test_validator_min_length_exact() {
    assert!(validators::min_length("abc", 3, "Field").is_ok());
}

#[test]
fn test_validator_min_length_longer() {
    assert!(validators::min_length("abcd", 3, "Field").is_ok());
}

#[test]
fn test_validator_min_length_empty_string() {
    let result = validators::min_length("", 5, "Field");
    assert!(result.is_err());
}

#[test]
fn test_validator_min_length_various_thresholds() {
    assert!(validators::min_length("a", 1, "Field").is_ok());
    assert!(validators::min_length("ab", 2, "Field").is_ok());
    assert!(validators::min_length("abc", 3, "Field").is_ok());
    assert!(validators::min_length("abcd", 4, "Field").is_ok());
}

#[test]
fn test_validator_min_length_unicode() {
    // Length is in bytes, not characters
    assert!(validators::min_length("hello", 5, "Field").is_ok());
    assert!(validators::min_length("こんにちは", 15, "Field").is_ok());
}

// ==================== Validators::max_length Tests ====================

#[test]
fn test_validator_max_length_too_long() {
    let result = validators::max_length("abcd", 3, "Field");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, "MAX_LENGTH");
}

#[test]
fn test_validator_max_length_exact() {
    assert!(validators::max_length("abc", 3, "Field").is_ok());
}

#[test]
fn test_validator_max_length_shorter() {
    assert!(validators::max_length("ab", 3, "Field").is_ok());
}

#[test]
fn test_validator_max_length_empty_string() {
    assert!(validators::max_length("", 5, "Field").is_ok());
}

#[test]
fn test_validator_max_length_various_thresholds() {
    assert!(validators::max_length("a", 1, "Field").is_ok());
    assert!(validators::max_length("ab", 2, "Field").is_ok());
    assert!(validators::max_length("abc", 3, "Field").is_ok());
    assert!(validators::max_length("abcd", 10, "Field").is_ok());
}

// ==================== Validators::email Tests ====================

#[test]
fn test_validator_email_valid() {
    assert!(validators::email("test@example.com").is_ok());
    assert!(validators::email("user.name@domain.co.uk").is_ok());
    assert!(validators::email("admin+test@mail server.com").is_ok());
}

#[test]
fn test_validator_email_no_at() {
    let result = validators::email("invalidemail.com");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, "EMAIL");
}

#[test]
fn test_validator_email_no_dot() {
    let result = validators::email("test@invalidcom");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, "EMAIL");
}

#[test]
fn test_validator_email_empty() {
    let result = validators::email("");
    assert!(result.is_err());
}

#[test]
fn test_validator_email_no_domain() {
    let result = validators::email("test@");
    assert!(result.is_err());
}

#[test]
fn test_validator_email_no_local() {
    // The email validator only checks for @ and .
    // So @example.com passes (has @ and .)
    // This is a basic validator, not a comprehensive one
    let result = validators::email("@example.com");
    assert!(result.is_ok());
}

#[test]
fn test_validator_email_multiple_at() {
    // With multiple @ but still has .
    assert!(validators::email("test@user@example.com").is_ok());
}

// ==================== Validators::range Tests ====================

#[test]
fn test_validator_range_within_bounds() {
    assert!(validators::range(5, 0, 10, "Value").is_ok());
    assert!(validators::range(0, 0, 10, "Value").is_ok());
    assert!(validators::range(10, 0, 10, "Value").is_ok());
}

#[test]
fn test_validator_range_below_min() {
    let result = validators::range(-1, 0, 10, "Value");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, "RANGE");
}

#[test]
fn test_validator_range_above_max() {
    let result = validators::range(11, 0, 10, "Value");
    assert!(result.is_err());
}

#[test]
fn test_validator_range_negative_values() {
    assert!(validators::range(-5, -10, -1, "Value").is_ok());
    assert!(validators::range(-15, -10, -1, "Value").is_err());
}

#[test]
fn test_validator_range_floating_point() {
    assert!(validators::range(0.5, 0.0, 1.0, "Value").is_ok());
    assert!(validators::range(1.5, 0.0, 1.0, "Value").is_err());
}

#[test]
fn test_validator_range_various_field_names() {
    let result1 = validators::range(150, 1, 120, "Age");
    assert_eq!(
        result1.unwrap_err().message,
        "Age must be between 1 and 120"
    );

    let result2 = validators::range(-5, 0, 100, "Score");
    assert!(result2.is_err());
}

// ==================== Validators::custom Tests ====================

#[test]
fn test_validator_custom_predicate_passes() {
    let result = validators::custom(
        &String::from("hello"),
        |s: &String| s.starts_with("h"),
        || ValidationError::new("Must start with 'h'", "PREFIX"),
    );
    assert!(result.is_ok());
}

#[test]
fn test_validator_custom_predicate_fails() {
    let result = validators::custom(
        &String::from("hello"),
        |s: &String| s.starts_with("x"),
        || ValidationError::new("Must start with 'x'", "PREFIX"),
    );
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, "PREFIX");
}

#[test]
fn test_validator_custom_with_numbers() {
    let result = validators::custom(
        &42,
        |n: &i32| *n > 0,
        || ValidationError::new("Must be positive", "POSITIVE"),
    );
    assert!(result.is_ok());

    let result2 = validators::custom(
        &-5,
        |n: &i32| *n > 0,
        || ValidationError::new("Must be positive", "POSITIVE"),
    );
    assert!(result2.is_err());
}

#[test]
fn test_validator_custom_complex_predicate() {
    let result = validators::custom(
        &String::from("password123"),
        |s: &String| s.len() >= 8 && s.chars().any(|c| c.is_ascii_digit()),
        || ValidationError::new("Password too weak", "WEAK_PASSWORD"),
    );
    assert!(result.is_ok());

    let result2 = validators::custom(
        &String::from("weak"),
        |s: &String| s.len() >= 8 && s.chars().any(|c| c.is_ascii_digit()),
        || ValidationError::new("Password too weak", "WEAK_PASSWORD"),
    );
    assert!(result2.is_err());
}

// ==================== Validators::pattern Tests ====================

#[test]
fn test_validator_pattern_contains() {
    assert!(validators::pattern("test@example.com", "@", "Email").is_ok());
}

#[test]
fn test_validator_pattern_not_contains() {
    let result = validators::pattern("invalidemail", "@", "Email");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, "PATTERN");
}

#[test]
fn test_validator_pattern_various_patterns() {
    assert!(validators::pattern("password123", "123", "Password").is_ok());
    assert!(validators::pattern("https://example.com", "://", "URL").is_ok());
    assert!(validators::pattern("(555) 123-4567", "(", "Phone").is_ok());
}

#[test]
fn test_validator_pattern_empty_string_value() {
    let result = validators::pattern("", "@", "Email");
    assert!(result.is_err());
}

#[test]
fn test_validator_pattern_empty_pattern() {
    // Empty string contains empty string
    assert!(validators::pattern("test", "", "Field").is_ok());
}

// ==================== Edge Cases and Error Handling ====================

#[test]
fn test_validation_result_type_alias_ok() {
    let result: ValidationResult<()> = Ok(());
    assert!(result.is_ok());
}

#[test]
fn test_validation_result_type_alias_err() {
    let result: ValidationResult<()> = Err(ValidationError::required("Field"));
    assert!(result.is_err());
}

#[test]
fn test_validation_error_with_string_field_names() {
    let field = String::from("DynamicField");
    let err = ValidationError::required(&field);
    assert_eq!(err.message, "DynamicField is required");
}

#[test]
fn test_validation_error_with_display_types() {
    struct Wrapper(String);
    impl Display for Wrapper {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    let err = ValidationError::required(Wrapper("CustomField".to_string()));
    assert_eq!(err.message, "CustomField is required");
}

#[test]
fn test_validatable_with_struct() {
    struct EmailInput {
        value: String,
    }

    impl Validatable for EmailInput {
        type Error = ValidationError;

        fn validate(&self) -> ValidationResult<Self::Error> {
            if self.value.is_empty() {
                return Err(ValidationError::required("Email"));
            }
            validators::email(&self.value)?;
            Ok(ValidationError::new("", "OK"))
        }
    }

    let input = EmailInput {
        value: "test@example.com".to_string(),
    };
    assert!(input.is_valid());

    let input2 = EmailInput {
        value: "invalid".to_string(),
    };
    assert!(!input2.is_valid());

    let input3 = EmailInput {
        value: "".to_string(),
    };
    assert!(!input3.is_valid());
}

#[test]
fn test_validator_require_with_zero() {
    // 0 is displayed as "0" which is not empty
    assert!(validators::require(&0, "Zero").is_ok());
}

#[test]
fn test_validator_range_inverted_bounds() {
    // When min > max, no value can satisfy
    let result = validators::range(5, 10, 1, "Value");
    assert!(result.is_err());
}

#[test]
fn test_validator_range_equal_bounds() {
    // When min == max, only that exact value is valid
    assert!(validators::range(5, 5, 5, "Value").is_ok());
    assert!(validators::range(4, 5, 5, "Value").is_err());
    assert!(validators::range(6, 5, 5, "Value").is_err());
}

#[test]
fn test_validation_error_debug_format() {
    let err = ValidationError::required("Test");
    let debug_str = format!("{:?}", err);
    assert!(debug_str.contains("Test is required"));
    assert!(debug_str.contains("REQUIRED"));
}

// ==================== Integration Scenario Tests ====================

#[test]
fn test_form_validation_scenario() {
    struct FormData {
        username: String,
        email: String,
        age: u8,
    }

    impl Validatable for FormData {
        type Error = ValidationError;

        fn validate(&self) -> ValidationResult<Self::Error> {
            validators::require(&self.username, "Username")?;
            validators::min_length(&self.username, 3, "Username")?;
            validators::max_length(&self.username, 20, "Username")?;

            validators::require(&self.email, "Email")?;
            validators::email(&self.email)?;

            validators::range(self.age, 1, 120, "Age")?;

            Ok(ValidationError::new("", "OK"))
        }
    }

    // Valid form
    let valid_form = FormData {
        username: "john_doe".to_string(),
        email: "john@example.com".to_string(),
        age: 30,
    };
    assert!(valid_form.is_valid());

    // Invalid username (too short)
    let invalid1 = FormData {
        username: "jo".to_string(),
        email: "john@example.com".to_string(),
        age: 30,
    };
    assert!(!invalid1.is_valid());

    // Invalid email
    let invalid2 = FormData {
        username: "john_doe".to_string(),
        email: "invalid-email".to_string(),
        age: 30,
    };
    assert!(!invalid2.is_valid());

    // Invalid age
    let invalid3 = FormData {
        username: "john_doe".to_string(),
        email: "john@example.com".to_string(),
        age: 150,
    };
    assert!(!invalid3.is_valid());
}

#[test]
fn test_validator_chain_scenario() {
    // Test multiple validators in sequence
    let password = "password123";

    // Chain should pass all validators
    assert!(validators::require(&password, "Password").is_ok());
    assert!(validators::min_length(&password, 8, "Password").is_ok());
}

#[test]
fn test_validation_error_display_for_user_messages() {
    let errors = vec![
        ValidationError::required("Email"),
        ValidationError::min_length("Password", 8),
        ValidationError::email("invalid"),
        ValidationError::range(150, 1, 120),
    ];

    let messages: Vec<String> = errors.iter().map(|e| e.to_string()).collect();

    assert!(messages[0].contains("Email is required"));
    assert!(messages[1].contains("at least 8 characters"));
    assert!(messages[2].contains("not a valid email"));
    assert!(messages[3].contains("between 1 and 120"));
}
