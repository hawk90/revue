//! Form validation types and common validators

/// Validation error
#[derive(Clone, Debug, PartialEq)]
pub struct ValidationError {
    /// Error message
    pub message: String,
}

impl ValidationError {
    /// Create a new validation error
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

/// Validator function type
pub type ValidatorFn = Box<dyn Fn(&str) -> Result<(), ValidationError> + Send + Sync>;

/// Common validators
pub struct Validators;

impl Validators {
    /// Required field validator
    pub fn required() -> ValidatorFn {
        Box::new(|value| {
            if value.trim().is_empty() {
                Err(ValidationError::new("This field is required"))
            } else {
                Ok(())
            }
        })
    }

    /// Minimum length validator
    pub fn min_length(min: usize) -> ValidatorFn {
        Box::new(move |value| {
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

    /// Maximum length validator
    pub fn max_length(max: usize) -> ValidatorFn {
        Box::new(move |value| {
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

    /// Email format validator
    pub fn email() -> ValidatorFn {
        Box::new(|value| {
            if value.is_empty() {
                return Ok(());
            }

            // Basic email validation without regex
            // 1. Must contain exactly one '@'
            // 2. Must have at least one character before '@'
            // 3. Must have at least one '.' after '@'
            // 4. Must have at least one character after the last '.'
            // 5. No whitespace allowed
            // 6. Reasonable length limits

            if value.contains(char::is_whitespace) {
                return Err(ValidationError::new(
                    "Invalid email: cannot contain whitespace",
                ));
            }

            if value.len() > 254 {
                return Err(ValidationError::new(
                    "Invalid email: too long (max 254 characters)",
                ));
            }

            let parts: Vec<&str> = value.split('@').collect();
            if parts.len() != 2 {
                return Err(ValidationError::new(
                    "Invalid email: must contain exactly one '@'",
                ));
            }

            let local_part = parts[0];
            let domain_part = parts[1];

            if local_part.is_empty() || local_part.len() > 64 {
                return Err(ValidationError::new("Invalid email: local part is invalid"));
            }

            if domain_part.is_empty() || domain_part.len() > 253 {
                return Err(ValidationError::new(
                    "Invalid email: domain part is invalid",
                ));
            }

            // Domain must contain at least one dot
            if !domain_part.contains('.') {
                return Err(ValidationError::new(
                    "Invalid email: domain must contain '.'",
                ));
            }

            // Last dot must not be at the end
            if let Some(last_dot_pos) = domain_part.rfind('.') {
                if last_dot_pos == domain_part.len() - 1 {
                    return Err(ValidationError::new(
                        "Invalid email: domain must have characters after '.'",
                    ));
                }
            }

            Ok(())
        })
    }

    /// Numeric validator
    pub fn numeric() -> ValidatorFn {
        Box::new(|value| {
            if value.is_empty() {
                return Ok(());
            }
            if value.parse::<f64>().is_ok() {
                Ok(())
            } else {
                Err(ValidationError::new("Must be a number"))
            }
        })
    }

    /// Integer validator
    pub fn integer() -> ValidatorFn {
        Box::new(|value| {
            if value.is_empty() {
                return Ok(());
            }
            if value.parse::<i64>().is_ok() {
                Ok(())
            } else {
                Err(ValidationError::new("Must be an integer"))
            }
        })
    }

    /// Minimum value validator (for numbers)
    pub fn min_value(min: f64) -> ValidatorFn {
        Box::new(move |value| {
            if value.is_empty() {
                return Ok(());
            }
            match value.parse::<f64>() {
                Ok(n) if n >= min => Ok(()),
                Ok(_) => Err(ValidationError::new(format!("Must be at least {}", min))),
                Err(_) => Err(ValidationError::new("Must be a number")),
            }
        })
    }

    /// Maximum value validator (for numbers)
    pub fn max_value(max: f64) -> ValidatorFn {
        Box::new(move |value| {
            if value.is_empty() {
                return Ok(());
            }
            match value.parse::<f64>() {
                Ok(n) if n <= max => Ok(()),
                Ok(_) => Err(ValidationError::new(format!("Must be at most {}", max))),
                Err(_) => Err(ValidationError::new("Must be a number")),
            }
        })
    }

    /// Pattern validator using contains check
    pub fn contains(substring: &str, message: &str) -> ValidatorFn {
        let substring = substring.to_string();
        let message = message.to_string();
        Box::new(move |value| {
            if value.is_empty() {
                return Ok(());
            }
            if value.contains(&substring) {
                Ok(())
            } else {
                Err(ValidationError::new(&message))
            }
        })
    }

    /// Alphanumeric validator
    pub fn alphanumeric() -> ValidatorFn {
        Box::new(|value| {
            if value.is_empty() {
                return Ok(());
            }
            if value.chars().all(|c| c.is_alphanumeric()) {
                Ok(())
            } else {
                Err(ValidationError::new(
                    "Must contain only letters and numbers",
                ))
            }
        })
    }

    /// No whitespace validator
    pub fn no_whitespace() -> ValidatorFn {
        Box::new(|value| {
            if value.chars().any(|c| c.is_whitespace()) {
                Err(ValidationError::new("Must not contain whitespace"))
            } else {
                Ok(())
            }
        })
    }

    /// Custom validator
    pub fn custom<F>(f: F) -> ValidatorFn
    where
        F: Fn(&str) -> Result<(), ValidationError> + Send + Sync + 'static,
    {
        Box::new(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_error_new() {
        let err = ValidationError::new("test error");
        assert_eq!(err.message, "test error");
    }

    #[test]
    fn test_required_validator() {
        let validator = Validators::required();
        assert!(validator("hello").is_ok());
        assert!(validator("  ").is_err());
        assert!(validator("").is_err());
    }

    #[test]
    fn test_min_length_validator() {
        let validator = Validators::min_length(3);
        assert!(validator("abc").is_ok());
        assert!(validator("abcd").is_ok());
        assert!(validator("ab").is_err());
    }

    #[test]
    fn test_max_length_validator() {
        let validator = Validators::max_length(5);
        assert!(validator("abc").is_ok());
        assert!(validator("abcde").is_ok());
        assert!(validator("abcdef").is_err());
    }

    #[test]
    fn test_email_validator() {
        let validator = Validators::email();

        // Valid emails
        assert!(validator("test@example.com").is_ok());
        assert!(validator("").is_ok()); // empty is ok
        assert!(validator("user.name@example.com").is_ok());
        assert!(validator("user+tag@example.co.uk").is_ok());

        // Invalid emails
        assert!(validator("invalid").is_err()); // no @
        assert!(validator("no-at-sign.com").is_err()); // no @
        assert!(validator("@example.com").is_err()); // no local part
        assert!(validator("user@").is_err()); // no domain
        assert!(validator("user@domain").is_err()); // no dot in domain
        assert!(validator("user@domain.").is_err()); // dot at end
        assert!(validator("user @example.com").is_err()); // whitespace
        assert!(validator("a@b.c").is_ok()); // minimal valid email
    }

    #[test]
    fn test_numeric_validator() {
        let validator = Validators::numeric();
        assert!(validator("123").is_ok());
        assert!(validator("12.34").is_ok());
        assert!(validator("-5.5").is_ok());
        assert!(validator("").is_ok());
        assert!(validator("abc").is_err());
    }

    #[test]
    fn test_integer_validator() {
        let validator = Validators::integer();
        assert!(validator("123").is_ok());
        assert!(validator("-456").is_ok());
        assert!(validator("").is_ok());
        assert!(validator("12.34").is_err());
        assert!(validator("abc").is_err());
    }

    #[test]
    fn test_min_value_validator() {
        let validator = Validators::min_value(10.0);
        assert!(validator("10").is_ok());
        assert!(validator("15").is_ok());
        assert!(validator("").is_ok());
        assert!(validator("5").is_err());
        assert!(validator("abc").is_err());
    }

    #[test]
    fn test_max_value_validator() {
        let validator = Validators::max_value(100.0);
        assert!(validator("100").is_ok());
        assert!(validator("50").is_ok());
        assert!(validator("").is_ok());
        assert!(validator("150").is_err());
        assert!(validator("abc").is_err());
    }

    #[test]
    fn test_contains_validator() {
        let validator = Validators::contains("@", "Must contain @");
        assert!(validator("test@example").is_ok());
        assert!(validator("").is_ok());
        assert!(validator("no-at-sign").is_err());
    }

    #[test]
    fn test_alphanumeric_validator() {
        let validator = Validators::alphanumeric();
        assert!(validator("abc123").is_ok());
        assert!(validator("").is_ok());
        assert!(validator("abc-123").is_err());
        assert!(validator("abc 123").is_err());
    }

    #[test]
    fn test_no_whitespace_validator() {
        let validator = Validators::no_whitespace();
        assert!(validator("abc123").is_ok());
        assert!(validator("").is_ok());
        assert!(validator("abc 123").is_err());
        assert!(validator("abc\t123").is_err());
    }

    #[test]
    fn test_custom_validator() {
        let validator = Validators::custom(|value| {
            if value.starts_with("test") {
                Ok(())
            } else {
                Err(ValidationError::new("Must start with 'test'"))
            }
        });
        assert!(validator("test123").is_ok());
        assert!(validator("hello").is_err());
    }
}
