//! Widget validation tests (ValidationError and the validators)

use revue::widget::validation::validators;
use revue::widget::validation::Validatable;
use revue::widget::validation::ValidationError;
use revue::widget::validation::ValidationResult;
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
    assert!(validators::require(&1.25, "a float").is_ok());
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
    assert!(validators::min_length(password, 8, "Password").is_ok());
}

#[test]
fn test_validation_error_display_for_user_messages() {
    let errors = [
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
    let result = validators::range(1.25, 0.0, 10.0, "Value");
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
