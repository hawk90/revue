//! Form validation pattern
//!
//! Provides reusable form field and validation state for input forms.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::patterns::{FormState, FormField, Validator};
//!
//! let mut form = FormState::new()
//!     .field("username", FormField::text()
//!         .required()
//!         .min_length(3)
//!         .max_length(20))
//!     .field("email", FormField::email())
//!     .field("age", FormField::number()
//!         .min(0)
//!         .max(150));
//!
//! form.set_value("username", "john");
//! form.set_value("email", "john@example.com");
//!
//! if form.validate_all() {
//!     println!("Form is valid!");
//! }
//! ```

use std::collections::HashMap;

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
    ///
    /// For simple pattern matching. Use custom validator for complex patterns.
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

/// Form field type
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FieldType {
    /// Text input
    #[default]
    Text,
    /// Password input (masked)
    Password,
    /// Email input
    Email,
    /// Number input
    Number,
    /// Integer input
    Integer,
    /// Multi-line text
    TextArea,
}

/// Form field configuration
pub struct FormField {
    /// Field type
    pub field_type: FieldType,
    /// Field label
    pub label: String,
    /// Placeholder text
    pub placeholder: String,
    /// Current value
    pub value: String,
    /// Validation errors
    pub errors: Vec<ValidationError>,
    /// Validators
    validators: Vec<ValidatorFn>,
    /// Whether field has been touched
    pub touched: bool,
    /// Whether field is disabled
    pub disabled: bool,
}

impl Default for FormField {
    fn default() -> Self {
        Self::text()
    }
}

impl FormField {
    /// Create a text field
    pub fn text() -> Self {
        Self {
            field_type: FieldType::Text,
            label: String::new(),
            placeholder: String::new(),
            value: String::new(),
            errors: Vec::new(),
            validators: Vec::new(),
            touched: false,
            disabled: false,
        }
    }

    /// Create a password field
    pub fn password() -> Self {
        Self {
            field_type: FieldType::Password,
            ..Self::text()
        }
    }

    /// Create an email field
    pub fn email() -> Self {
        Self {
            field_type: FieldType::Email,
            validators: vec![Validators::email()],
            ..Self::text()
        }
    }

    /// Create a number field
    pub fn number() -> Self {
        Self {
            field_type: FieldType::Number,
            validators: vec![Validators::numeric()],
            ..Self::text()
        }
    }

    /// Create an integer field
    pub fn integer() -> Self {
        Self {
            field_type: FieldType::Integer,
            validators: vec![Validators::integer()],
            ..Self::text()
        }
    }

    /// Create a textarea field
    pub fn textarea() -> Self {
        Self {
            field_type: FieldType::TextArea,
            ..Self::text()
        }
    }

    /// Set field label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    /// Set placeholder text
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set initial value
    pub fn initial_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    /// Mark as required
    pub fn required(mut self) -> Self {
        self.validators.insert(0, Validators::required());
        self
    }

    /// Set minimum length
    pub fn min_length(mut self, min: usize) -> Self {
        self.validators.push(Validators::min_length(min));
        self
    }

    /// Set maximum length
    pub fn max_length(mut self, max: usize) -> Self {
        self.validators.push(Validators::max_length(max));
        self
    }

    /// Set minimum value (for number fields)
    pub fn min(mut self, min: f64) -> Self {
        self.validators.push(Validators::min_value(min));
        self
    }

    /// Set maximum value (for number fields)
    pub fn max(mut self, max: f64) -> Self {
        self.validators.push(Validators::max_value(max));
        self
    }

    /// Add a custom validator
    pub fn validator(mut self, validator: ValidatorFn) -> Self {
        self.validators.push(validator);
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Validate the field
    pub fn validate(&mut self) -> bool {
        self.errors.clear();

        for validator in &self.validators {
            if let Err(e) = validator(&self.value) {
                self.errors.push(e);
            }
        }

        self.errors.is_empty()
    }

    /// Check if field is valid
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    /// Check if field has errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Get first error message
    pub fn first_error(&self) -> Option<&str> {
        self.errors.first().map(|e| e.message.as_str())
    }
}

/// Form state managing multiple fields
pub struct FormState {
    /// Form fields by name
    fields: HashMap<String, FormField>,
    /// Field order for iteration
    field_order: Vec<String>,
    /// Currently focused field
    focused: Option<String>,
    /// Whether form has been submitted
    submitted: bool,
}

impl Default for FormState {
    fn default() -> Self {
        Self::new()
    }
}

impl FormState {
    /// Create a new form state
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
            field_order: Vec::new(),
            focused: None,
            submitted: false,
        }
    }

    /// Add a field to the form
    pub fn field(mut self, name: impl Into<String>, field: FormField) -> Self {
        let name = name.into();
        self.field_order.push(name.clone());
        self.fields.insert(name, field);
        self
    }

    /// Get a field by name
    pub fn get(&self, name: &str) -> Option<&FormField> {
        self.fields.get(name)
    }

    /// Get a mutable field by name
    pub fn get_mut(&mut self, name: &str) -> Option<&mut FormField> {
        self.fields.get_mut(name)
    }

    /// Get field value
    pub fn value(&self, name: &str) -> Option<&str> {
        self.fields.get(name).map(|f| f.value.as_str())
    }

    /// Set field value
    pub fn set_value(&mut self, name: &str, value: impl Into<String>) {
        if let Some(field) = self.fields.get_mut(name) {
            field.value = value.into();
            field.touched = true;
        }
    }

    /// Validate a single field
    pub fn validate_field(&mut self, name: &str) -> bool {
        if let Some(field) = self.fields.get_mut(name) {
            field.validate()
        } else {
            false
        }
    }

    /// Validate all fields
    pub fn validate_all(&mut self) -> bool {
        let mut valid = true;
        for field in self.fields.values_mut() {
            if !field.validate() {
                valid = false;
            }
        }
        valid
    }

    /// Check if form is valid
    pub fn is_valid(&self) -> bool {
        self.fields.values().all(|f| f.is_valid())
    }

    /// Check if form has any errors
    pub fn has_errors(&self) -> bool {
        self.fields.values().any(|f| f.has_errors())
    }

    /// Get all field names with errors
    pub fn errors(&self) -> Vec<(&str, &str)> {
        self.fields
            .iter()
            .filter_map(|(name, field)| field.first_error().map(|err| (name.as_str(), err)))
            .collect()
    }

    /// Get currently focused field name
    pub fn focused(&self) -> Option<&str> {
        self.focused.as_deref()
    }

    /// Set focused field
    pub fn focus(&mut self, name: impl Into<String>) {
        let name = name.into();
        if self.fields.contains_key(&name) {
            self.focused = Some(name);
        }
    }

    /// Focus next field
    pub fn focus_next(&mut self) {
        if self.field_order.is_empty() {
            return;
        }

        let current_idx = self
            .focused
            .as_ref()
            .and_then(|name| self.field_order.iter().position(|n| n == name))
            .unwrap_or(0);

        let next_idx = (current_idx + 1) % self.field_order.len();
        self.focused = Some(self.field_order[next_idx].clone());
    }

    /// Focus previous field
    pub fn focus_prev(&mut self) {
        if self.field_order.is_empty() {
            return;
        }

        let current_idx = self
            .focused
            .as_ref()
            .and_then(|name| self.field_order.iter().position(|n| n == name))
            .unwrap_or(0);

        let prev_idx = if current_idx == 0 {
            self.field_order.len() - 1
        } else {
            current_idx - 1
        };

        self.focused = Some(self.field_order[prev_idx].clone());
    }

    /// Clear focus
    pub fn blur(&mut self) {
        self.focused = None;
    }

    /// Get field names in order
    pub fn field_names(&self) -> &[String] {
        &self.field_order
    }

    /// Iterate over fields in order
    pub fn iter(&self) -> impl Iterator<Item = (&str, &FormField)> {
        self.field_order
            .iter()
            .filter_map(|name| self.fields.get(name).map(|field| (name.as_str(), field)))
    }

    /// Mark form as submitted
    pub fn submit(&mut self) -> bool {
        self.submitted = true;

        // Touch all fields
        for field in self.fields.values_mut() {
            field.touched = true;
        }

        self.validate_all()
    }

    /// Check if form has been submitted
    pub fn is_submitted(&self) -> bool {
        self.submitted
    }

    /// Reset form to initial state
    pub fn reset(&mut self) {
        for field in self.fields.values_mut() {
            field.value.clear();
            field.errors.clear();
            field.touched = false;
        }
        self.submitted = false;
    }

    /// Get form values as a map
    pub fn values(&self) -> HashMap<&str, &str> {
        self.fields
            .iter()
            .map(|(name, field)| (name.as_str(), field.value.as_str()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ValidationError tests
    #[test]
    fn test_validation_error_new() {
        let err = ValidationError::new("test error");
        assert_eq!(err.message, "test error");
    }

    #[test]
    fn test_validation_error_from_string() {
        let err = ValidationError::new(String::from("string error"));
        assert_eq!(err.message, "string error");
    }

    #[test]
    fn test_validation_error_debug() {
        let err = ValidationError::new("debug test");
        let debug = format!("{:?}", err);
        assert!(debug.contains("ValidationError"));
        assert!(debug.contains("debug test"));
    }

    #[test]
    fn test_validation_error_clone() {
        let err = ValidationError::new("clone test");
        let cloned = err.clone();
        assert_eq!(err.message, cloned.message);
    }

    #[test]
    fn test_validation_error_eq() {
        let err1 = ValidationError::new("same");
        let err2 = ValidationError::new("same");
        let err3 = ValidationError::new("different");
        assert_eq!(err1, err2);
        assert_ne!(err1, err3);
    }

    // Validators tests
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

    // FieldType tests
    #[test]
    fn test_field_type_default() {
        let field_type = FieldType::default();
        assert_eq!(field_type, FieldType::Text);
    }

    #[test]
    fn test_field_type_debug() {
        assert!(format!("{:?}", FieldType::Text).contains("Text"));
        assert!(format!("{:?}", FieldType::Password).contains("Password"));
        assert!(format!("{:?}", FieldType::Email).contains("Email"));
        assert!(format!("{:?}", FieldType::Number).contains("Number"));
        assert!(format!("{:?}", FieldType::Integer).contains("Integer"));
        assert!(format!("{:?}", FieldType::TextArea).contains("TextArea"));
    }

    #[test]
    fn test_field_type_clone() {
        let ft = FieldType::Password;
        let cloned = ft;
        assert_eq!(ft, cloned);
    }

    #[test]
    fn test_field_type_eq() {
        assert_eq!(FieldType::Text, FieldType::Text);
        assert_ne!(FieldType::Text, FieldType::Password);
    }

    // FormField tests
    #[test]
    fn test_form_field_text() {
        let field = FormField::text();
        assert_eq!(field.field_type, FieldType::Text);
        assert!(field.label.is_empty());
        assert!(field.placeholder.is_empty());
        assert!(field.value.is_empty());
        assert!(!field.touched);
        assert!(!field.disabled);
    }

    #[test]
    fn test_form_field_password() {
        let field = FormField::password();
        assert_eq!(field.field_type, FieldType::Password);
    }

    #[test]
    fn test_form_field_email() {
        let mut field = FormField::email();
        assert_eq!(field.field_type, FieldType::Email);
        // Should have email validator
        field.value = "invalid".to_string();
        assert!(!field.validate());
    }

    #[test]
    fn test_form_field_number() {
        let mut field = FormField::number();
        assert_eq!(field.field_type, FieldType::Number);
        // Should have numeric validator
        field.value = "abc".to_string();
        assert!(!field.validate());
        field.value = "123.45".to_string();
        assert!(field.validate());
    }

    #[test]
    fn test_form_field_integer() {
        let mut field = FormField::integer();
        assert_eq!(field.field_type, FieldType::Integer);
        // Should have integer validator
        field.value = "123.45".to_string();
        assert!(!field.validate());
        field.value = "123".to_string();
        assert!(field.validate());
    }

    #[test]
    fn test_form_field_textarea() {
        let field = FormField::textarea();
        assert_eq!(field.field_type, FieldType::TextArea);
    }

    #[test]
    fn test_form_field_default() {
        let field = FormField::default();
        assert_eq!(field.field_type, FieldType::Text);
    }

    #[test]
    fn test_form_field_label() {
        let field = FormField::text().label("Username");
        assert_eq!(field.label, "Username");
    }

    #[test]
    fn test_form_field_placeholder() {
        let field = FormField::text().placeholder("Enter username");
        assert_eq!(field.placeholder, "Enter username");
    }

    #[test]
    fn test_form_field_initial_value() {
        let field = FormField::text().initial_value("default");
        assert_eq!(field.value, "default");
    }

    #[test]
    fn test_form_field_disabled() {
        let field = FormField::text().disabled(true);
        assert!(field.disabled);
    }

    #[test]
    fn test_form_field_min_max() {
        let mut field = FormField::number().min(5.0).max(10.0);
        field.value = "7".to_string();
        assert!(field.validate());
        field.value = "3".to_string();
        assert!(!field.validate());
        field.value = "15".to_string();
        assert!(!field.validate());
    }

    #[test]
    fn test_form_field_custom_validator() {
        let mut field = FormField::text().validator(Validators::custom(|v| {
            if v.len() == 4 {
                Ok(())
            } else {
                Err(ValidationError::new("Must be 4 chars"))
            }
        }));
        field.value = "abcd".to_string();
        assert!(field.validate());
        field.value = "abc".to_string();
        assert!(!field.validate());
    }

    #[test]
    fn test_form_field() {
        let mut field = FormField::text().label("Username").required().min_length(3);

        field.value = "ab".to_string();
        assert!(!field.validate());
        assert!(field.has_errors());

        field.value = "abc".to_string();
        assert!(field.validate());
        assert!(!field.has_errors());
    }

    #[test]
    fn test_form_field_is_valid() {
        let field = FormField::text();
        assert!(field.is_valid()); // no errors by default
    }

    #[test]
    fn test_form_field_first_error() {
        let mut field = FormField::text().required().min_length(5);
        field.value = "".to_string();
        field.validate();
        assert!(field.first_error().is_some());
        assert!(field.first_error().unwrap().contains("required"));
    }

    #[test]
    fn test_form_field_first_error_none() {
        let field = FormField::text();
        assert!(field.first_error().is_none());
    }

    #[test]
    fn test_form_field_multiple_errors() {
        let mut field = FormField::text().required().min_length(10);
        field.value = "".to_string();
        field.validate();
        // Required fails, min_length also fails on empty
        assert!(field.errors.len() >= 1);
    }

    #[test]
    fn test_form_field_validate_clears_errors() {
        let mut field = FormField::text().required();
        field.value = "".to_string();
        field.validate();
        assert!(!field.errors.is_empty());
        field.value = "valid".to_string();
        field.validate();
        assert!(field.errors.is_empty());
    }

    // FormState tests
    #[test]
    fn test_form_state_new() {
        let form = FormState::new();
        assert!(form.fields.is_empty());
        assert!(form.field_order.is_empty());
        assert!(form.focused.is_none());
        assert!(!form.submitted);
    }

    #[test]
    fn test_form_state_default() {
        let form = FormState::default();
        assert!(form.fields.is_empty());
    }

    #[test]
    fn test_form_state() {
        let mut form = FormState::new()
            .field("username", FormField::text().required())
            .field("email", FormField::email());

        form.set_value("username", "john");
        form.set_value("email", "john@example.com");

        assert!(form.validate_all());
        assert!(form.is_valid());
    }

    #[test]
    fn test_form_state_get() {
        let form = FormState::new().field("name", FormField::text().label("Name"));
        let field = form.get("name");
        assert!(field.is_some());
        assert_eq!(field.unwrap().label, "Name");
    }

    #[test]
    fn test_form_state_get_none() {
        let form = FormState::new();
        assert!(form.get("nonexistent").is_none());
    }

    #[test]
    fn test_form_state_get_mut() {
        let mut form = FormState::new().field("name", FormField::text());
        if let Some(field) = form.get_mut("name") {
            field.value = "modified".to_string();
        }
        assert_eq!(form.value("name"), Some("modified"));
    }

    #[test]
    fn test_form_state_get_mut_none() {
        let mut form = FormState::new();
        assert!(form.get_mut("nonexistent").is_none());
    }

    #[test]
    fn test_form_state_value() {
        let mut form = FormState::new().field("name", FormField::text());
        form.set_value("name", "test");
        assert_eq!(form.value("name"), Some("test"));
    }

    #[test]
    fn test_form_state_value_none() {
        let form = FormState::new();
        assert!(form.value("nonexistent").is_none());
    }

    #[test]
    fn test_form_state_set_value_marks_touched() {
        let mut form = FormState::new().field("name", FormField::text());
        form.set_value("name", "test");
        assert!(form.get("name").unwrap().touched);
    }

    #[test]
    fn test_form_state_set_value_nonexistent() {
        let mut form = FormState::new();
        form.set_value("nonexistent", "value"); // Should not panic
    }

    #[test]
    fn test_form_state_validate_field() {
        let mut form = FormState::new().field("name", FormField::text().required());
        assert!(!form.validate_field("name")); // empty value fails required
    }

    #[test]
    fn test_form_state_validate_field_nonexistent() {
        let mut form = FormState::new();
        assert!(!form.validate_field("nonexistent"));
    }

    #[test]
    fn test_form_errors() {
        let mut form = FormState::new().field("username", FormField::text().required());

        form.set_value("username", "");
        form.validate_all();

        assert!(form.has_errors());
        let errors = form.errors();
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn test_form_errors_multiple_fields() {
        let mut form = FormState::new()
            .field("a", FormField::text().required())
            .field("b", FormField::text().required());
        form.validate_all();
        let errors = form.errors();
        assert_eq!(errors.len(), 2);
    }

    #[test]
    fn test_form_focus() {
        let mut form = FormState::new()
            .field("a", FormField::text())
            .field("b", FormField::text())
            .field("c", FormField::text());

        form.focus("a");
        assert_eq!(form.focused(), Some("a"));

        form.focus_next();
        assert_eq!(form.focused(), Some("b"));

        form.focus_next();
        assert_eq!(form.focused(), Some("c"));

        form.focus_next(); // Wraps around
        assert_eq!(form.focused(), Some("a"));

        form.focus_prev();
        assert_eq!(form.focused(), Some("c"));
    }

    #[test]
    fn test_form_focus_nonexistent() {
        let mut form = FormState::new().field("a", FormField::text());
        form.focus("nonexistent");
        assert!(form.focused().is_none());
    }

    #[test]
    fn test_form_focus_next_empty() {
        let mut form = FormState::new();
        form.focus_next(); // Should not panic
        assert!(form.focused().is_none());
    }

    #[test]
    fn test_form_focus_prev_empty() {
        let mut form = FormState::new();
        form.focus_prev(); // Should not panic
        assert!(form.focused().is_none());
    }

    #[test]
    fn test_form_focus_prev_wrap() {
        let mut form = FormState::new()
            .field("a", FormField::text())
            .field("b", FormField::text());
        form.focus("a");
        form.focus_prev();
        assert_eq!(form.focused(), Some("b"));
    }

    #[test]
    fn test_form_blur() {
        let mut form = FormState::new().field("a", FormField::text());
        form.focus("a");
        assert!(form.focused().is_some());
        form.blur();
        assert!(form.focused().is_none());
    }

    #[test]
    fn test_form_field_names() {
        let form = FormState::new()
            .field("first", FormField::text())
            .field("second", FormField::text());
        let names = form.field_names();
        assert_eq!(names, &["first", "second"]);
    }

    #[test]
    fn test_form_iter() {
        let form = FormState::new()
            .field("a", FormField::text().label("A"))
            .field("b", FormField::text().label("B"));
        let fields: Vec<_> = form.iter().collect();
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].0, "a");
        assert_eq!(fields[0].1.label, "A");
        assert_eq!(fields[1].0, "b");
        assert_eq!(fields[1].1.label, "B");
    }

    #[test]
    fn test_form_submit() {
        let mut form = FormState::new().field("name", FormField::text().required());

        // Empty submission should fail
        assert!(!form.submit());
        assert!(form.is_submitted());

        // With value should succeed
        form.set_value("name", "John");
        assert!(form.submit());
    }

    #[test]
    fn test_form_submit_touches_all_fields() {
        let mut form = FormState::new()
            .field("a", FormField::text())
            .field("b", FormField::text());
        assert!(!form.get("a").unwrap().touched);
        assert!(!form.get("b").unwrap().touched);
        form.submit();
        assert!(form.get("a").unwrap().touched);
        assert!(form.get("b").unwrap().touched);
    }

    #[test]
    fn test_form_reset() {
        let mut form = FormState::new().field("name", FormField::text());

        form.set_value("name", "John");
        form.submit();

        form.reset();
        assert_eq!(form.value("name"), Some(""));
        assert!(!form.is_submitted());
    }

    #[test]
    fn test_form_reset_clears_touched() {
        let mut form = FormState::new().field("name", FormField::text());
        form.set_value("name", "value");
        assert!(form.get("name").unwrap().touched);
        form.reset();
        assert!(!form.get("name").unwrap().touched);
    }

    #[test]
    fn test_form_reset_clears_errors() {
        let mut form = FormState::new().field("name", FormField::text().required());
        form.validate_all();
        assert!(!form.get("name").unwrap().errors.is_empty());
        form.reset();
        assert!(form.get("name").unwrap().errors.is_empty());
    }

    #[test]
    fn test_form_values() {
        let mut form = FormState::new()
            .field("a", FormField::text())
            .field("b", FormField::text());
        form.set_value("a", "value_a");
        form.set_value("b", "value_b");
        let values = form.values();
        assert_eq!(values.get("a"), Some(&"value_a"));
        assert_eq!(values.get("b"), Some(&"value_b"));
    }

    #[test]
    fn test_form_values_empty() {
        let form = FormState::new();
        let values = form.values();
        assert!(values.is_empty());
    }

    // Integration tests
    #[test]
    fn test_complete_form_workflow() {
        let mut form = FormState::new()
            .field(
                "username",
                FormField::text()
                    .label("Username")
                    .required()
                    .min_length(3)
                    .max_length(20),
            )
            .field("email", FormField::email().label("Email").required())
            .field("age", FormField::integer().label("Age").min(0.0).max(150.0));

        // Initial state
        assert!(!form.is_submitted());
        assert!(!form.has_errors());

        // Set some values
        form.set_value("username", "john");
        form.set_value("email", "john@example.com");
        form.set_value("age", "25");

        // Submit
        assert!(form.submit());
        assert!(form.is_valid());

        // Get values
        let values = form.values();
        assert_eq!(values.get("username"), Some(&"john"));
        assert_eq!(values.get("email"), Some(&"john@example.com"));
        assert_eq!(values.get("age"), Some(&"25"));
    }

    #[test]
    fn test_form_with_invalid_data() {
        let mut form = FormState::new()
            .field("username", FormField::text().required().min_length(5))
            .field("email", FormField::email());

        form.set_value("username", "ab"); // too short
        form.set_value("email", "invalid"); // no @ or .

        assert!(!form.submit());
        assert!(form.has_errors());
        let errors = form.errors();
        assert_eq!(errors.len(), 2);
    }
}
