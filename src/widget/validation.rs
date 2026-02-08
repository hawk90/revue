//! Validation traits and common validators for widgets
//!
//! Provides reusable validation utilities for form input and other interactive widgets.

use std::fmt;

/// Result type for validation operations
pub type ValidationResult<T = ()> = Result<T, ValidationError>;

/// Validation error
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    /// Error message
    pub message: String,
    /// Error code for programmatic handling
    pub code: &'static str,
}

impl ValidationError {
    /// Create a new validation error
    pub fn new(message: impl Into<String>, code: &'static str) -> Self {
        Self {
            message: message.into(),
            code,
        }
    }

    /// Create a required field error
    pub fn required(field: impl fmt::Display) -> Self {
        Self::new(format!("{} is required", field), "REQUIRED")
    }

    /// Create a min length error
    pub fn min_length(field: impl fmt::Display, min: usize) -> Self {
        Self::new(
            format!("{} must be at least {} characters", field, min),
            "MIN_LENGTH",
        )
    }

    /// Create a max length error
    pub fn max_length(field: impl fmt::Display, max: usize) -> Self {
        Self::new(
            format!("{} must be at most {} characters", field, max),
            "MAX_LENGTH",
        )
    }

    /// Create a pattern mismatch error
    pub fn pattern(field: impl fmt::Display, pattern: &str) -> Self {
        Self::new(
            format!("{} must match pattern: {}", field, pattern),
            "PATTERN",
        )
    }

    /// Create a range error
    pub fn range(value: impl fmt::Display, min: impl fmt::Display, max: impl fmt::Display) -> Self {
        Self::new(
            format!("{} must be between {} and {}", value, min, max),
            "RANGE",
        )
    }

    /// Create an email format error
    pub fn email(value: impl fmt::Display) -> Self {
        Self::new(format!("'{}' is not a valid email address", value), "EMAIL")
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ValidationError {}

/// Trait for types that can be validated
///
/// Widgets can implement this trait to provide consistent validation
/// across different input types.
///
/// # Example
///
/// ```rust,ignore
/// use revue::widget::validation::{Validatable, ValidationError, ValidationResult};
///
/// struct EmailInput {
///     value: String,
/// }
///
/// impl Validatable for EmailInput {
///     type Error = ValidationError;
///
///     fn validate(&self) -> ValidationResult {
///         if self.value.is_empty() {
///             return Err(ValidationError::required("Email"));
///         }
///         if !self.value.contains('@') {
///             return Err(ValidationError::email(&self.value));
///         }
///         Ok(())
///     }
///
///     fn is_valid(&self) -> bool {
///         self.validate().is_ok()
///     }
/// }
/// ```
pub trait Validatable {
    /// The error type for validation failures
    type Error: std::error::Error + Send + Sync + 'static;

    /// Validate the current value
    ///
    /// Returns `Ok(())` if valid, `Err(Error)` if invalid.
    fn validate(&self) -> ValidationResult<Self::Error>;

    /// Check if the current value is valid
    ///
    /// This is a convenience method that calls `validate()` and
    /// returns `true` if validation passes.
    fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }
}

/// Common validators for reuse across widgets
pub mod validators {
    use super::ValidationError;
    use std::fmt::Display;

    /// Validate that a value is not empty
    pub fn require<T: Display>(value: &T, field: impl Display) -> Result<(), ValidationError> {
        if value.to_string().is_empty() {
            Err(ValidationError::required(field))
        } else {
            Ok(())
        }
    }

    /// Validate minimum length
    pub fn min_length(value: &str, min: usize, field: impl Display) -> Result<(), ValidationError> {
        if value.len() < min {
            Err(ValidationError::min_length(field, min))
        } else {
            Ok(())
        }
    }

    /// Validate maximum length
    pub fn max_length(value: &str, max: usize, field: impl Display) -> Result<(), ValidationError> {
        if value.len() > max {
            Err(ValidationError::max_length(field, max))
        } else {
            Ok(())
        }
    }

    /// Validate email format (basic check for @ symbol)
    pub fn email(value: &str) -> Result<(), ValidationError> {
        if !value.contains('@') || !value.contains('.') {
            Err(ValidationError::email(value))
        } else {
            Ok(())
        }
    }

    /// Validate numeric range
    pub fn range<T: Display + PartialOrd>(
        value: T,
        min: T,
        max: T,
        field: impl Display,
    ) -> Result<(), ValidationError> {
        if value < min || value > max {
            Err(ValidationError::new(
                format!("{} must be between {} and {}", field, min, max),
                "RANGE",
            ))
        } else {
            Ok(())
        }
    }

    /// Validate with a custom predicate
    pub fn custom<T>(
        value: &T,
        predicate: impl Fn(&T) -> bool,
        error: impl Fn() -> ValidationError,
    ) -> Result<(), ValidationError> {
        if predicate(value) {
            Ok(())
        } else {
            Err(error())
        }
    }

    /// Validate using a string pattern (simple contains check)
    pub fn pattern(value: &str, pattern: &str, field: impl Display) -> Result<(), ValidationError> {
        if value.contains(pattern) {
            Ok(())
        } else {
            Err(ValidationError::pattern(field, pattern))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_error_required() {
        let err = ValidationError::required("Email");
        assert_eq!(err.message, "Email is required");
        assert_eq!(err.code, "REQUIRED");
    }

    #[test]
    fn test_validation_error_email() {
        let err = ValidationError::email("invalid");
        assert!(err.message.contains("not a valid email"));
        assert_eq!(err.code, "EMAIL");
    }

    #[test]
    fn test_validator_require() {
        assert!(validators::require(&"", "Field").is_err());
        assert!(validators::require(&"value", "Field").is_ok());
    }

    #[test]
    fn test_validator_min_length() {
        assert!(validators::min_length("ab", 3, "Field").is_err());
        assert!(validators::min_length("abc", 3, "Field").is_ok());
    }

    #[test]
    fn test_validator_max_length() {
        assert!(validators::max_length("abc", 2, "Field").is_err());
        assert!(validators::max_length("ab", 2, "Field").is_ok());
    }

    #[test]
    fn test_validator_email() {
        assert!(validators::email("invalid").is_err());
        assert!(validators::email("test@example.com").is_ok());
    }

    #[test]
    fn test_validator_range() {
        assert!(validators::range(5, 0, 10, "Value").is_ok());
        assert!(validators::range(15, 0, 10, "Value").is_err());
        assert!(validators::range(-5, 0, 10, "Value").is_err());
    }

    // =========================================================================
    // ValidationError::new tests
    // =========================================================================

    #[test]
    fn test_validation_error_new() {
        let err = ValidationError::new("Custom error", "CUSTOM");
        assert_eq!(err.message, "Custom error");
        assert_eq!(err.code, "CUSTOM");
    }

    // =========================================================================
    // ValidationError::min_length tests
    // =========================================================================

    #[test]
    fn test_validation_error_min_length() {
        let err = ValidationError::min_length("Password", 8);
        assert!(err.message.contains("Password"));
        assert!(err.message.contains("8"));
        assert!(err.message.contains("at least"));
        assert_eq!(err.code, "MIN_LENGTH");
    }

    // =========================================================================
    // ValidationError::max_length tests
    // =========================================================================

    #[test]
    fn test_validation_error_max_length() {
        let err = ValidationError::max_length("Username", 20);
        assert!(err.message.contains("Username"));
        assert!(err.message.contains("20"));
        assert!(err.message.contains("at most"));
        assert_eq!(err.code, "MAX_LENGTH");
    }

    // =========================================================================
    // ValidationError::pattern tests
    // =========================================================================

    #[test]
    fn test_validation_error_pattern() {
        let err = ValidationError::pattern("Field", "[0-9]+");
        assert!(err.message.contains("Field"));
        assert!(err.message.contains("[0-9]+"));
        assert!(err.message.contains("pattern"));
        assert_eq!(err.code, "PATTERN");
    }

    // =========================================================================
    // ValidationError::range tests
    // =========================================================================

    #[test]
    fn test_validation_error_range() {
        let err = ValidationError::range(150, 0, 100);
        assert!(err.message.contains("150"));
        assert!(err.message.contains("0"));
        assert!(err.message.contains("100"));
        assert!(err.message.contains("between"));
        assert_eq!(err.code, "RANGE");
    }

    // =========================================================================
    // validators::custom tests
    // =========================================================================

    #[test]
    fn test_validator_custom_passes() {
        let value = 42;
        let result = validators::custom(
            &value,
            |v| *v > 18,
            || ValidationError::new("Must be adult", "ADULT"),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_validator_custom_fails() {
        let value = 15;
        let result = validators::custom(
            &value,
            |v| *v > 18,
            || ValidationError::new("Must be adult", "ADULT"),
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code, "ADULT");
    }

    // =========================================================================
    // validators::pattern tests
    // =========================================================================

    #[test]
    fn test_validator_pattern_matches() {
        assert!(validators::pattern("hello world", "world", "Field").is_ok());
    }

    #[test]
    fn test_validator_pattern_no_match() {
        let result = validators::pattern("hello", "world", "Field");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code, "PATTERN");
    }

    // =========================================================================
    // ValidationError Display trait tests
    // =========================================================================

    #[test]
    fn test_validation_error_display() {
        let err = ValidationError::new("Test error", "TEST");
        let display_string = format!("{}", err);
        assert_eq!(display_string, "Test error");
    }

    // =========================================================================
    // ValidationError Debug trait tests
    // =========================================================================

    #[test]
    fn test_validation_error_debug() {
        let err = ValidationError::new("Test", "TEST");
        let debug_string = format!("{:?}", err);
        assert!(debug_string.contains("Test"));
        assert!(debug_string.contains("TEST"));
    }

    // =========================================================================
    // ValidationError PartialEq tests
    // =========================================================================

    #[test]
    fn test_validation_error_eq_same() {
        let err1 = ValidationError::new("Same", "CODE");
        let err2 = ValidationError::new("Same", "CODE");
        assert_eq!(err1, err2);
    }

    #[test]
    fn test_validation_error_eq_different_message() {
        let err1 = ValidationError::new("First", "CODE");
        let err2 = ValidationError::new("Second", "CODE");
        assert_ne!(err1, err2);
    }

    #[test]
    fn test_validation_error_eq_different_code() {
        let err1 = ValidationError::new("Same", "CODE1");
        let err2 = ValidationError::new("Same", "CODE2");
        assert_ne!(err1, err2);
    }

    // =========================================================================
    // ValidationError Clone tests
    // =========================================================================

    #[test]
    fn test_validation_error_clone() {
        let err1 = ValidationError::new("Test", "CODE");
        let err2 = err1.clone();
        assert_eq!(err1, err2);
    }
}
