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
            if value.contains('@') && value.contains('.') {
                Ok(())
            } else {
                Err(ValidationError::new("Invalid email format"))
            }
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
