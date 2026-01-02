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
                Err(ValidationError::new("Must contain only letters and numbers"))
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
            .filter_map(|(name, field)| {
                field.first_error().map(|err| (name.as_str(), err))
            })
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

        let current_idx = self.focused
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

        let current_idx = self.focused
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
        self.field_order.iter().filter_map(|name| {
            self.fields.get(name).map(|field| (name.as_str(), field))
        })
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

    #[test]
    fn test_required_validator() {
        let validator = Validators::required();
        assert!(validator("hello").is_ok());
        assert!(validator("").is_err());
        assert!(validator("   ").is_err());
    }

    #[test]
    fn test_email_validator() {
        let validator = Validators::email();
        assert!(validator("test@example.com").is_ok());
        assert!(validator("invalid").is_err());
        assert!(validator("").is_ok()); // Empty is ok (use required for that)
    }

    #[test]
    fn test_min_length() {
        let validator = Validators::min_length(3);
        assert!(validator("abc").is_ok());
        assert!(validator("ab").is_err());
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
    fn test_form_field() {
        let mut field = FormField::text()
            .label("Username")
            .required()
            .min_length(3);

        field.value = "ab".to_string();
        assert!(!field.validate());
        assert!(field.has_errors());

        field.value = "abc".to_string();
        assert!(field.validate());
        assert!(!field.has_errors());
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
    fn test_form_errors() {
        let mut form = FormState::new()
            .field("username", FormField::text().required());

        form.set_value("username", "");
        form.validate_all();

        assert!(form.has_errors());
        let errors = form.errors();
        assert_eq!(errors.len(), 1);
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
    fn test_form_submit() {
        let mut form = FormState::new()
            .field("name", FormField::text().required());

        // Empty submission should fail
        assert!(!form.submit());
        assert!(form.is_submitted());

        // With value should succeed
        form.set_value("name", "John");
        assert!(form.submit());
    }

    #[test]
    fn test_form_reset() {
        let mut form = FormState::new()
            .field("name", FormField::text());

        form.set_value("name", "John");
        form.submit();

        form.reset();
        assert_eq!(form.value("name"), Some(""));
        assert!(!form.is_submitted());
    }
}
