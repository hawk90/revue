//! Form validation utilities
//!
//! Provides composable validators for form inputs.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::utils::validation::{Validator, required, min_length, email, all_of};
//!
//! // Simple validation
//! let result = required()("hello");
//! assert!(result.is_ok());
//!
//! // Chained validators
//! let email_validator = all_of(&[
//!     required(),
//!     min_length(5),
//!     email(),
//! ]);
//!
//! assert!(email_validator("user@example.com").is_ok());
//! assert!(email_validator("").is_err());
//! ```

use std::fmt;

/// Validation error with message
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationError {
    /// Error message
    pub message: String,
    /// Optional field name
    pub field: Option<String>,
}

impl ValidationError {
    /// Create new validation error
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            field: None,
        }
    }

    /// Create validation error with field name
    pub fn with_field(message: impl Into<String>, field: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            field: Some(field.into()),
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.field {
            Some(field) => write!(f, "{}: {}", field, self.message),
            None => write!(f, "{}", self.message),
        }
    }
}

impl std::error::Error for ValidationError {}

/// Result type for validation
pub type ValidationResult = Result<(), ValidationError>;

/// Validator function type
pub type Validator = Box<dyn Fn(&str) -> ValidationResult + Send + Sync>;

/// Create a required field validator
pub fn required() -> Validator {
    Box::new(|value: &str| {
        if value.trim().is_empty() {
            Err(ValidationError::new("This field is required"))
        } else {
            Ok(())
        }
    })
}

/// Create a minimum length validator
pub fn min_length(min: usize) -> Validator {
    Box::new(move |value: &str| {
        if value.len() < min {
            Err(ValidationError::new(format!(
                "Must be at least {} characters",
                min
            )))
        } else {
            Ok(())
        }
    })
}

/// Create a maximum length validator
pub fn max_length(max: usize) -> Validator {
    Box::new(move |value: &str| {
        if value.len() > max {
            Err(ValidationError::new(format!(
                "Must be at most {} characters",
                max
            )))
        } else {
            Ok(())
        }
    })
}

/// Create a length range validator
pub fn length_range(min: usize, max: usize) -> Validator {
    Box::new(move |value: &str| {
        let len = value.len();
        if len < min || len > max {
            Err(ValidationError::new(format!(
                "Must be between {} and {} characters",
                min, max
            )))
        } else {
            Ok(())
        }
    })
}

/// Create an email validator
pub fn email() -> Validator {
    Box::new(|value: &str| {
        if value.is_empty() {
            return Ok(()); // Use required() for mandatory
        }

        // Simple email validation
        let parts: Vec<&str> = value.split('@').collect();
        if parts.len() != 2 {
            return Err(ValidationError::new("Invalid email format"));
        }

        let (local, domain) = (parts[0], parts[1]);

        if local.is_empty() || domain.is_empty() {
            return Err(ValidationError::new("Invalid email format"));
        }

        if !domain.contains('.') {
            return Err(ValidationError::new("Invalid email domain"));
        }

        Ok(())
    })
}

/// Create a URL validator
pub fn url() -> Validator {
    Box::new(|value: &str| {
        if value.is_empty() {
            return Ok(());
        }

        if !value.starts_with("http://") && !value.starts_with("https://") {
            return Err(ValidationError::new(
                "URL must start with http:// or https://",
            ));
        }

        let rest = value
            .trim_start_matches("https://")
            .trim_start_matches("http://");
        if rest.is_empty() || !rest.contains('.') {
            return Err(ValidationError::new("Invalid URL format"));
        }

        Ok(())
    })
}

/// Create a pattern validator using regex-like matching
pub fn pattern(pattern: &'static str, message: &'static str) -> Validator {
    Box::new(move |value: &str| {
        if value.is_empty() {
            return Ok(());
        }

        // Simple pattern matching (no full regex to avoid dependency)
        let matches = match pattern {
            r"^\d+$" => value.chars().all(|c| c.is_ascii_digit()),
            r"^[a-zA-Z]+$" => value.chars().all(|c| c.is_ascii_alphabetic()),
            r"^[a-zA-Z0-9]+$" => value.chars().all(|c| c.is_ascii_alphanumeric()),
            r"^[a-zA-Z0-9_]+$" => value.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'),
            r"^[a-zA-Z0-9_-]+$" => value
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-'),
            r"^[a-z]+$" => value.chars().all(|c| c.is_ascii_lowercase()),
            r"^[A-Z]+$" => value.chars().all(|c| c.is_ascii_uppercase()),
            _ => true, // Unknown pattern, pass through
        };

        if matches {
            Ok(())
        } else {
            Err(ValidationError::new(message))
        }
    })
}

/// Create a numeric validator
pub fn numeric() -> Validator {
    pattern(r"^\d+$", "Must be a number")
}

/// Create an alphabetic validator
pub fn alphabetic() -> Validator {
    pattern(r"^[a-zA-Z]+$", "Must contain only letters")
}

/// Create an alphanumeric validator
pub fn alphanumeric() -> Validator {
    pattern(r"^[a-zA-Z0-9]+$", "Must contain only letters and numbers")
}

/// Create a lowercase validator
pub fn lowercase() -> Validator {
    pattern(r"^[a-z]+$", "Must be lowercase")
}

/// Create an uppercase validator
pub fn uppercase() -> Validator {
    pattern(r"^[A-Z]+$", "Must be uppercase")
}

/// Create a minimum value validator for numeric strings
pub fn min_value(min: i64) -> Validator {
    Box::new(move |value: &str| {
        if value.is_empty() {
            return Ok(());
        }

        match value.parse::<i64>() {
            Ok(n) if n >= min => Ok(()),
            Ok(_) => Err(ValidationError::new(format!("Must be at least {}", min))),
            Err(_) => Err(ValidationError::new("Must be a number")),
        }
    })
}

/// Create a maximum value validator for numeric strings
pub fn max_value(max: i64) -> Validator {
    Box::new(move |value: &str| {
        if value.is_empty() {
            return Ok(());
        }

        match value.parse::<i64>() {
            Ok(n) if n <= max => Ok(()),
            Ok(_) => Err(ValidationError::new(format!("Must be at most {}", max))),
            Err(_) => Err(ValidationError::new("Must be a number")),
        }
    })
}

/// Create a value range validator for numeric strings
pub fn value_range(min: i64, max: i64) -> Validator {
    Box::new(move |value: &str| {
        if value.is_empty() {
            return Ok(());
        }

        match value.parse::<i64>() {
            Ok(n) if n >= min && n <= max => Ok(()),
            Ok(_) => Err(ValidationError::new(format!(
                "Must be between {} and {}",
                min, max
            ))),
            Err(_) => Err(ValidationError::new("Must be a number")),
        }
    })
}

/// Create a custom validator with a predicate function
pub fn custom<F>(predicate: F, message: &'static str) -> Validator
where
    F: Fn(&str) -> bool + Send + Sync + 'static,
{
    Box::new(move |value: &str| {
        if predicate(value) {
            Ok(())
        } else {
            Err(ValidationError::new(message))
        }
    })
}

/// Create a validator that passes if any of the validators pass
pub fn any_of(validators: Vec<Validator>) -> Validator {
    Box::new(move |value: &str| {
        for validator in &validators {
            if validator(value).is_ok() {
                return Ok(());
            }
        }
        Err(ValidationError::new("None of the conditions were met"))
    })
}

/// Create a validator that passes if all validators pass
pub fn all_of(validators: Vec<Validator>) -> Validator {
    Box::new(move |value: &str| {
        for validator in &validators {
            validator(value)?;
        }
        Ok(())
    })
}

/// Create a validator that matches one of the allowed values
pub fn one_of(values: &'static [&'static str]) -> Validator {
    Box::new(move |value: &str| {
        if value.is_empty() || values.contains(&value) {
            Ok(())
        } else {
            Err(ValidationError::new(format!(
                "Must be one of: {}",
                values.join(", ")
            )))
        }
    })
}

/// Create a validator that ensures value is not in the list
pub fn not_one_of(values: &'static [&'static str]) -> Validator {
    Box::new(move |value: &str| {
        if values.contains(&value) {
            Err(ValidationError::new("This value is not allowed"))
        } else {
            Ok(())
        }
    })
}

/// Create a validator that ensures value matches another field
pub fn matches(other_value: String, field_name: &'static str) -> Validator {
    Box::new(move |value: &str| {
        if value == other_value {
            Ok(())
        } else {
            Err(ValidationError::new(format!("Must match {}", field_name)))
        }
    })
}

/// Form validation helper
pub struct FormValidator {
    fields: Vec<(String, Vec<Validator>)>,
}

impl FormValidator {
    /// Create new form validator
    pub fn new() -> Self {
        Self { fields: Vec::new() }
    }

    /// Add field with validators
    pub fn field(mut self, name: impl Into<String>, validators: Vec<Validator>) -> Self {
        self.fields.push((name.into(), validators));
        self
    }

    /// Validate all fields
    ///
    /// # Errors
    ///
    /// Returns `Err(Vec<ValidationError>)` with a list of validation errors
    /// if any field fails validation. Each error includes the field name
    /// and a description of what went wrong.
    pub fn validate(&self, values: &[(&str, &str)]) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        for (field_name, validators) in &self.fields {
            let value = values
                .iter()
                .find(|(name, _)| name == field_name)
                .map(|(_, v)| *v)
                .unwrap_or("");

            for validator in validators {
                if let Err(mut err) = validator(value) {
                    err.field = Some(field_name.clone());
                    errors.push(err);
                    break; // One error per field
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl Default for FormValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let result =
            validator.validate(&[("email", "user@example.com"), ("password", "secret123")]);
        assert!(result.is_ok());

        let result = validator.validate(&[("email", "invalid"), ("password", "short")]);
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 2);
    }
}
