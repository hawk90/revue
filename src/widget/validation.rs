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
