//! Form validation pattern with reactive state
//!
//! Provides reactive form field and validation state for input forms.
//! Values, errors, and validity are automatically computed using Signal/Computed.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! let form = FormState::new()
//!     .field("username", |f| f
//!         .label("Username")
//!         .required()
//!         .min_length(3)
//!         .max_length(20))
//!     .field("email", |f| f.email().required())
//!     .field("age", |f| f.number().min(0.0).max(150.0))
//!     .build();
//!
//! form.set_value("username", "john");
//! form.set_value("email", "john@example.com");
//!
//! // Validity is automatically computed
//! if form.is_valid() {
//!     println!("Form is valid!");
//! }
//! ```

mod validators;

pub use validators::{ValidationError, ValidatorFn, Validators};

use crate::reactive::{computed, signal, Computed, Signal};
use std::collections::HashMap;
use std::sync::Arc;

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

/// Builder for creating reactive form fields
pub struct FormFieldBuilder {
    field_type: FieldType,
    label: String,
    placeholder: String,
    initial_value: String,
    validators: Vec<ValidatorFn>,
    disabled: bool,
    /// Field name to match against (for password confirmation, etc.)
    matches_field: Option<String>,
}

impl Default for FormFieldBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl FormFieldBuilder {
    /// Create a new field builder (defaults to text field)
    pub fn new() -> Self {
        Self {
            field_type: FieldType::Text,
            label: String::new(),
            placeholder: String::new(),
            initial_value: String::new(),
            validators: Vec::new(),
            disabled: false,
            matches_field: None,
        }
    }

    /// Set field type to text
    pub fn text(mut self) -> Self {
        self.field_type = FieldType::Text;
        self
    }

    /// Set field type to password
    pub fn password(mut self) -> Self {
        self.field_type = FieldType::Password;
        self
    }

    /// Set field type to email (adds email validator)
    pub fn email(mut self) -> Self {
        self.field_type = FieldType::Email;
        self.validators.push(Validators::email());
        self
    }

    /// Set field type to number (adds numeric validator)
    pub fn number(mut self) -> Self {
        self.field_type = FieldType::Number;
        self.validators.push(Validators::numeric());
        self
    }

    /// Set field type to integer (adds integer validator)
    pub fn integer(mut self) -> Self {
        self.field_type = FieldType::Integer;
        self.validators.push(Validators::integer());
        self
    }

    /// Set field type to textarea
    pub fn textarea(mut self) -> Self {
        self.field_type = FieldType::TextArea;
        self
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
        self.initial_value = value.into();
        self
    }

    /// Mark as required (inserts at front for priority)
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

    /// Match this field's value against another field
    ///
    /// Useful for password confirmation fields.
    /// The actual validation is set up when FormState is built.
    pub fn matches(mut self, field_name: impl Into<String>) -> Self {
        self.matches_field = Some(field_name.into());
        self
    }

    /// Build the reactive FormField
    pub fn build(self) -> FormField {
        self.build_with_match(None)
    }

    /// Build the reactive FormField with optional match signal
    pub(crate) fn build_with_match(self, match_signal: Option<Signal<String>>) -> FormField {
        let value = signal(self.initial_value);
        let touched = signal(false);
        let validators = Arc::new(self.validators);

        // Create computed errors that auto-update when value changes
        let value_for_errors = value.clone();
        let validators_for_errors = validators.clone();
        let match_signal_clone = match_signal.clone();
        let errors = computed(move || {
            let val = value_for_errors.get();
            let mut errs: Vec<ValidationError> = validators_for_errors
                .iter()
                .filter_map(|v| v(&val).err())
                .collect();

            // Check matches constraint
            if let Some(ref match_sig) = match_signal_clone {
                let match_val = match_sig.get();
                if val != match_val {
                    errs.push(ValidationError::new("Fields do not match"));
                }
            }

            errs
        });

        FormField {
            field_type: self.field_type,
            label: self.label,
            placeholder: self.placeholder,
            value,
            errors,
            touched,
            disabled: self.disabled,
            validators,
        }
    }

    /// Get the matches_field if set
    pub(crate) fn get_matches_field(&self) -> Option<&str> {
        self.matches_field.as_deref()
    }
}

/// Reactive form field with automatic validation
///
/// Values and errors are managed using Signal/Computed for automatic reactivity.
/// When the value changes, errors are automatically recomputed.
#[derive(Clone)]
pub struct FormField {
    /// Field type
    pub field_type: FieldType,
    /// Field label
    pub label: String,
    /// Placeholder text
    pub placeholder: String,
    /// Reactive current value
    value: Signal<String>,
    /// Computed validation errors (auto-recalculates when value changes)
    errors: Computed<Vec<ValidationError>>,
    /// Reactive touched state
    touched: Signal<bool>,
    /// Whether field is disabled
    pub disabled: bool,
    /// Validators (kept for potential dynamic validation)
    #[allow(dead_code)]
    validators: Arc<Vec<ValidatorFn>>,
}

impl Default for FormField {
    fn default() -> Self {
        FormFieldBuilder::new().build()
    }
}

impl FormField {
    /// Create a text field builder
    pub fn text() -> FormFieldBuilder {
        FormFieldBuilder::new().text()
    }

    /// Create a password field builder
    pub fn password() -> FormFieldBuilder {
        FormFieldBuilder::new().password()
    }

    /// Create an email field builder (includes email validator)
    pub fn email() -> FormFieldBuilder {
        FormFieldBuilder::new().email()
    }

    /// Create a number field builder (includes numeric validator)
    pub fn number() -> FormFieldBuilder {
        FormFieldBuilder::new().number()
    }

    /// Create an integer field builder (includes integer validator)
    pub fn integer() -> FormFieldBuilder {
        FormFieldBuilder::new().integer()
    }

    /// Create a textarea field builder
    pub fn textarea() -> FormFieldBuilder {
        FormFieldBuilder::new().textarea()
    }

    /// Get the current value (clones the value)
    pub fn value(&self) -> String {
        self.value.get()
    }

    /// Get the value signal for reactive access
    pub fn value_signal(&self) -> &Signal<String> {
        &self.value
    }

    /// Set the field value (automatically triggers validation)
    pub fn set_value(&self, value: impl Into<String>) {
        self.value.set(value.into());
        self.touched.set(true);
    }

    /// Update the field value with a function
    pub fn update_value(&self, f: impl FnOnce(&mut String)) {
        self.value.update(f);
        self.touched.set(true);
    }

    /// Get validation errors (computed automatically)
    pub fn errors(&self) -> Vec<ValidationError> {
        self.errors.get()
    }

    /// Check if field is valid (no errors)
    pub fn is_valid(&self) -> bool {
        self.errors.get().is_empty()
    }

    /// Check if field has errors
    pub fn has_errors(&self) -> bool {
        !self.errors.get().is_empty()
    }

    /// Get first error message
    pub fn first_error(&self) -> Option<String> {
        self.errors.get().first().map(|e| e.message.clone())
    }

    /// Check if field has been touched
    pub fn is_touched(&self) -> bool {
        self.touched.get()
    }

    /// Mark field as touched
    pub fn touch(&self) {
        self.touched.set(true);
    }

    /// Reset field to initial state
    pub fn reset(&self) {
        self.value.set(String::new());
        self.touched.set(false);
    }

    /// Get the touched signal for reactive access
    pub fn touched_signal(&self) -> &Signal<bool> {
        &self.touched
    }
}

/// Builder for creating reactive form state
pub struct FormStateBuilder {
    fields: Vec<(String, FormFieldBuilder)>,
}

impl Default for FormStateBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl FormStateBuilder {
    /// Create a new form state builder
    pub fn new() -> Self {
        Self { fields: Vec::new() }
    }

    /// Add a field to the form using a builder function
    pub fn field(
        mut self,
        name: impl Into<String>,
        builder_fn: impl FnOnce(FormFieldBuilder) -> FormFieldBuilder,
    ) -> Self {
        let builder = builder_fn(FormFieldBuilder::new());
        self.fields.push((name.into(), builder));
        self
    }

    /// Build the reactive FormState
    pub fn build(self) -> FormState {
        let mut fields = HashMap::new();
        let mut field_order = Vec::new();
        let mut pending_matches: Vec<(String, FormFieldBuilder, String)> = Vec::new();

        // First pass: build fields without matches, collect pending matches
        for (name, builder) in self.fields {
            field_order.push(name.clone());
            let match_field_opt = builder.get_matches_field().map(|s| s.to_string());
            if let Some(match_field) = match_field_opt {
                pending_matches.push((name, builder, match_field));
            } else {
                fields.insert(name, builder.build());
            }
        }

        // Second pass: build fields with matches (target field must exist)
        for (name, builder, match_field) in pending_matches {
            let match_signal = fields.get(&match_field).map(|f| f.value_signal().clone());
            fields.insert(name, builder.build_with_match(match_signal));
        }

        let focused = signal(None);
        let submitted = signal(false);

        FormState {
            fields,
            field_order,
            focused,
            submitted,
        }
    }
}

/// Reactive form state managing multiple fields
///
/// Form validity is computed based on all field validations.
#[derive(Clone)]
pub struct FormState {
    /// Form fields by name
    fields: HashMap<String, FormField>,
    /// Field order for iteration
    field_order: Vec<String>,
    /// Currently focused field (reactive)
    focused: Signal<Option<String>>,
    /// Whether form has been submitted (reactive)
    submitted: Signal<bool>,
}

impl Default for FormState {
    fn default() -> Self {
        FormStateBuilder::new().build()
    }
}

impl FormState {
    /// Create a new form state builder
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> FormStateBuilder {
        FormStateBuilder::new()
    }

    /// Get a field by name
    pub fn get(&self, name: &str) -> Option<&FormField> {
        self.fields.get(name)
    }

    /// Get field value
    pub fn value(&self, name: &str) -> Option<String> {
        self.fields.get(name).map(|f| f.value())
    }

    /// Set field value (automatically triggers validation)
    pub fn set_value(&self, name: &str, value: impl Into<String>) {
        if let Some(field) = self.fields.get(name) {
            field.set_value(value);
        }
    }

    /// Check if form is valid (checks all fields)
    pub fn is_valid(&self) -> bool {
        self.fields.values().all(|f| f.is_valid())
    }

    /// Check if form has any errors
    pub fn has_errors(&self) -> bool {
        self.fields.values().any(|f| f.has_errors())
    }

    /// Get all field names with errors
    pub fn errors(&self) -> Vec<(String, String)> {
        self.fields
            .iter()
            .filter_map(|(name, field)| field.first_error().map(|err| (name.clone(), err)))
            .collect()
    }

    /// Get currently focused field name
    pub fn focused(&self) -> Option<String> {
        self.focused.get()
    }

    /// Set focused field
    pub fn focus(&self, name: impl Into<String>) {
        let name = name.into();
        if self.fields.contains_key(&name) {
            self.focused.set(Some(name));
        }
    }

    /// Focus next field
    pub fn focus_next(&self) {
        if self.field_order.is_empty() {
            return;
        }

        let current_idx = self
            .focused
            .get()
            .as_ref()
            .and_then(|name| self.field_order.iter().position(|n| n == name))
            .unwrap_or(0);

        let next_idx = (current_idx + 1) % self.field_order.len();
        self.focused.set(Some(self.field_order[next_idx].clone()));
    }

    /// Focus previous field
    pub fn focus_prev(&self) {
        if self.field_order.is_empty() {
            return;
        }

        let current_idx = self
            .focused
            .get()
            .as_ref()
            .and_then(|name| self.field_order.iter().position(|n| n == name))
            .unwrap_or(0);

        let prev_idx = if current_idx == 0 {
            self.field_order.len() - 1
        } else {
            current_idx - 1
        };

        self.focused.set(Some(self.field_order[prev_idx].clone()));
    }

    /// Clear focus
    pub fn blur(&self) {
        self.focused.set(None);
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

    /// Submit the form (touches all fields and returns validity)
    pub fn submit(&self) -> bool {
        self.submitted.set(true);

        // Touch all fields
        for field in self.fields.values() {
            field.touch();
        }

        self.is_valid()
    }

    /// Check if form has been submitted
    pub fn is_submitted(&self) -> bool {
        self.submitted.get()
    }

    /// Reset form to initial state
    pub fn reset(&self) {
        for field in self.fields.values() {
            field.reset();
        }
        self.submitted.set(false);
    }

    /// Get form values as a map
    pub fn values(&self) -> HashMap<String, String> {
        self.fields
            .iter()
            .map(|(name, field)| (name.clone(), field.value()))
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
        let field = FormField::text().build();
        assert_eq!(field.field_type, FieldType::Text);
        assert!(field.label.is_empty());
        assert!(field.placeholder.is_empty());
        assert!(field.value().is_empty());
        assert!(!field.is_touched());
        assert!(!field.disabled);
    }

    #[test]
    fn test_form_field_password() {
        let field = FormField::password().build();
        assert_eq!(field.field_type, FieldType::Password);
    }

    #[test]
    fn test_form_field_email() {
        let field = FormField::email().build();
        assert_eq!(field.field_type, FieldType::Email);
        // Should have email validator - auto validates
        field.set_value("invalid");
        assert!(!field.is_valid());
    }

    #[test]
    fn test_form_field_number() {
        let field = FormField::number().build();
        assert_eq!(field.field_type, FieldType::Number);
        // Should have numeric validator - auto validates
        field.set_value("abc");
        assert!(!field.is_valid());
        field.set_value("123.45");
        assert!(field.is_valid());
    }

    #[test]
    fn test_form_field_integer() {
        let field = FormField::integer().build();
        assert_eq!(field.field_type, FieldType::Integer);
        // Should have integer validator - auto validates
        field.set_value("123.45");
        assert!(!field.is_valid());
        field.set_value("123");
        assert!(field.is_valid());
    }

    #[test]
    fn test_form_field_textarea() {
        let field = FormField::textarea().build();
        assert_eq!(field.field_type, FieldType::TextArea);
    }

    #[test]
    fn test_form_field_default() {
        let field = FormField::default();
        assert_eq!(field.field_type, FieldType::Text);
    }

    #[test]
    fn test_form_field_label() {
        let field = FormField::text().label("Username").build();
        assert_eq!(field.label, "Username");
    }

    #[test]
    fn test_form_field_placeholder() {
        let field = FormField::text().placeholder("Enter username").build();
        assert_eq!(field.placeholder, "Enter username");
    }

    #[test]
    fn test_form_field_initial_value() {
        let field = FormField::text().initial_value("default").build();
        assert_eq!(field.value(), "default");
    }

    #[test]
    fn test_form_field_disabled() {
        let field = FormField::text().disabled(true).build();
        assert!(field.disabled);
    }

    #[test]
    fn test_form_field_min_max() {
        let field = FormField::number().min(5.0).max(10.0).build();
        field.set_value("7");
        assert!(field.is_valid());
        field.set_value("3");
        assert!(!field.is_valid());
        field.set_value("15");
        assert!(!field.is_valid());
    }

    #[test]
    fn test_form_field_custom_validator() {
        let field = FormField::text()
            .validator(Validators::custom(|v| {
                if v.len() == 4 {
                    Ok(())
                } else {
                    Err(ValidationError::new("Must be 4 chars"))
                }
            }))
            .build();
        field.set_value("abcd");
        assert!(field.is_valid());
        field.set_value("abc");
        assert!(!field.is_valid());
    }

    #[test]
    fn test_form_field() {
        let field = FormField::text()
            .label("Username")
            .required()
            .min_length(3)
            .build();

        field.set_value("ab");
        assert!(!field.is_valid());
        assert!(field.has_errors());

        field.set_value("abc");
        assert!(field.is_valid());
        assert!(!field.has_errors());
    }

    #[test]
    fn test_form_field_is_valid() {
        let field = FormField::text().build();
        assert!(field.is_valid()); // no errors by default
    }

    #[test]
    fn test_form_field_first_error() {
        let field = FormField::text().required().min_length(5).build();
        // Empty value triggers required error (auto-computed)
        assert!(field.first_error().is_some());
        assert!(field.first_error().unwrap().contains("required"));
    }

    #[test]
    fn test_form_field_first_error_none() {
        let field = FormField::text().build();
        assert!(field.first_error().is_none());
    }

    #[test]
    fn test_form_field_multiple_errors() {
        let field = FormField::text().required().min_length(10).build();
        // Empty value - errors are auto-computed
        // Required fails, min_length also fails on empty
        assert!(!field.errors().is_empty());
    }

    #[test]
    fn test_form_field_errors_auto_update() {
        let field = FormField::text().required().build();
        // Empty - has error
        assert!(!field.errors().is_empty());
        // Set valid value - errors auto-clear
        field.set_value("valid");
        assert!(field.errors().is_empty());
    }

    // FormState tests
    #[test]
    fn test_form_state_new() {
        let form = FormState::new().build();
        assert!(form.field_names().is_empty());
        assert!(form.focused().is_none());
        assert!(!form.is_submitted());
    }

    #[test]
    fn test_form_state_default() {
        let form = FormState::default();
        assert!(form.field_names().is_empty());
    }

    #[test]
    fn test_form_state() {
        let form = FormState::new()
            .field("username", |f| f.required())
            .field("email", |f| f.email())
            .build();

        form.set_value("username", "john");
        form.set_value("email", "john@example.com");

        // Auto validates
        assert!(form.is_valid());
    }

    #[test]
    fn test_form_state_get() {
        let form = FormState::new().field("name", |f| f.label("Name")).build();
        let field = form.get("name");
        assert!(field.is_some());
        assert_eq!(field.unwrap().label, "Name");
    }

    #[test]
    fn test_form_state_get_none() {
        let form = FormState::new().build();
        assert!(form.get("nonexistent").is_none());
    }

    #[test]
    fn test_form_state_set_value() {
        let form = FormState::new().field("name", |f| f).build();
        form.set_value("name", "modified");
        assert_eq!(form.value("name"), Some("modified".to_string()));
    }

    #[test]
    fn test_form_state_value() {
        let form = FormState::new().field("name", |f| f).build();
        form.set_value("name", "test");
        assert_eq!(form.value("name"), Some("test".to_string()));
    }

    #[test]
    fn test_form_state_value_none() {
        let form = FormState::new().build();
        assert!(form.value("nonexistent").is_none());
    }

    #[test]
    fn test_form_state_set_value_marks_touched() {
        let form = FormState::new().field("name", |f| f).build();
        form.set_value("name", "test");
        assert!(form.get("name").unwrap().is_touched());
    }

    #[test]
    fn test_form_state_set_value_nonexistent() {
        let form = FormState::new().build();
        form.set_value("nonexistent", "value"); // Should not panic
    }

    #[test]
    fn test_form_field_auto_validates() {
        let form = FormState::new().field("name", |f| f.required()).build();
        // Empty value fails required - auto validated
        assert!(!form.is_valid());
        form.set_value("name", "valid");
        assert!(form.is_valid());
    }

    #[test]
    fn test_form_errors() {
        let form = FormState::new().field("username", |f| f.required()).build();

        // Empty value - auto validates with errors
        assert!(form.has_errors());
        let errors = form.errors();
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn test_form_errors_multiple_fields() {
        let form = FormState::new()
            .field("a", |f| f.required())
            .field("b", |f| f.required())
            .build();
        // Both fields empty - auto validates
        let errors = form.errors();
        assert_eq!(errors.len(), 2);
    }

    #[test]
    fn test_form_focus() {
        let form = FormState::new()
            .field("a", |f| f)
            .field("b", |f| f)
            .field("c", |f| f)
            .build();

        form.focus("a");
        assert_eq!(form.focused(), Some("a".to_string()));

        form.focus_next();
        assert_eq!(form.focused(), Some("b".to_string()));

        form.focus_next();
        assert_eq!(form.focused(), Some("c".to_string()));

        form.focus_next(); // Wraps around
        assert_eq!(form.focused(), Some("a".to_string()));

        form.focus_prev();
        assert_eq!(form.focused(), Some("c".to_string()));
    }

    #[test]
    fn test_form_focus_nonexistent() {
        let form = FormState::new().field("a", |f| f).build();
        form.focus("nonexistent");
        assert!(form.focused().is_none());
    }

    #[test]
    fn test_form_focus_next_empty() {
        let form = FormState::new().build();
        form.focus_next(); // Should not panic
        assert!(form.focused().is_none());
    }

    #[test]
    fn test_form_focus_prev_empty() {
        let form = FormState::new().build();
        form.focus_prev(); // Should not panic
        assert!(form.focused().is_none());
    }

    #[test]
    fn test_form_focus_prev_wrap() {
        let form = FormState::new().field("a", |f| f).field("b", |f| f).build();
        form.focus("a");
        form.focus_prev();
        assert_eq!(form.focused(), Some("b".to_string()));
    }

    #[test]
    fn test_form_blur() {
        let form = FormState::new().field("a", |f| f).build();
        form.focus("a");
        assert!(form.focused().is_some());
        form.blur();
        assert!(form.focused().is_none());
    }

    #[test]
    fn test_form_field_names() {
        let form = FormState::new()
            .field("first", |f| f)
            .field("second", |f| f)
            .build();
        let names = form.field_names();
        assert_eq!(names, &["first", "second"]);
    }

    #[test]
    fn test_form_iter() {
        let form = FormState::new()
            .field("a", |f| f.label("A"))
            .field("b", |f| f.label("B"))
            .build();
        let fields: Vec<_> = form.iter().collect();
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].0, "a");
        assert_eq!(fields[0].1.label, "A");
        assert_eq!(fields[1].0, "b");
        assert_eq!(fields[1].1.label, "B");
    }

    #[test]
    fn test_form_submit() {
        let form = FormState::new().field("name", |f| f.required()).build();

        // Empty submission should fail
        assert!(!form.submit());
        assert!(form.is_submitted());

        // With value should succeed
        form.set_value("name", "John");
        assert!(form.submit());
    }

    #[test]
    fn test_form_submit_touches_all_fields() {
        let form = FormState::new().field("a", |f| f).field("b", |f| f).build();
        assert!(!form.get("a").unwrap().is_touched());
        assert!(!form.get("b").unwrap().is_touched());
        form.submit();
        assert!(form.get("a").unwrap().is_touched());
        assert!(form.get("b").unwrap().is_touched());
    }

    #[test]
    fn test_form_reset() {
        let form = FormState::new().field("name", |f| f).build();

        form.set_value("name", "John");
        form.submit();

        form.reset();
        assert_eq!(form.value("name"), Some("".to_string()));
        assert!(!form.is_submitted());
    }

    #[test]
    fn test_form_reset_clears_touched() {
        let form = FormState::new().field("name", |f| f).build();
        form.set_value("name", "value");
        assert!(form.get("name").unwrap().is_touched());
        form.reset();
        assert!(!form.get("name").unwrap().is_touched());
    }

    #[test]
    fn test_form_reset_clears_errors() {
        let form = FormState::new().field("name", |f| f.required()).build();
        // Has errors initially (required + empty)
        assert!(!form.get("name").unwrap().errors().is_empty());
        // Set valid value then reset
        form.set_value("name", "valid");
        form.reset();
        // After reset, empty again = has errors
        assert!(!form.get("name").unwrap().errors().is_empty());
    }

    #[test]
    fn test_form_values() {
        let form = FormState::new().field("a", |f| f).field("b", |f| f).build();
        form.set_value("a", "value_a");
        form.set_value("b", "value_b");
        let values = form.values();
        assert_eq!(values.get("a"), Some(&"value_a".to_string()));
        assert_eq!(values.get("b"), Some(&"value_b".to_string()));
    }

    #[test]
    fn test_form_values_empty() {
        let form = FormState::new().build();
        let values = form.values();
        assert!(values.is_empty());
    }

    // Integration tests
    #[test]
    fn test_complete_form_workflow() {
        let form = FormState::new()
            .field("username", |f| {
                f.label("Username").required().min_length(3).max_length(20)
            })
            .field("email", |f| f.email().label("Email").required())
            .field("age", |f| f.integer().label("Age").min(0.0).max(150.0))
            .build();

        // Initial state - has errors due to required fields
        assert!(!form.is_submitted());
        assert!(form.has_errors());

        // Set some values
        form.set_value("username", "john");
        form.set_value("email", "john@example.com");
        form.set_value("age", "25");

        // Now valid (auto-computed)
        assert!(form.is_valid());

        // Submit
        assert!(form.submit());

        // Get values
        let values = form.values();
        assert_eq!(values.get("username"), Some(&"john".to_string()));
        assert_eq!(values.get("email"), Some(&"john@example.com".to_string()));
        assert_eq!(values.get("age"), Some(&"25".to_string()));
    }

    #[test]
    fn test_form_with_invalid_data() {
        let form = FormState::new()
            .field("username", |f| f.required().min_length(5))
            .field("email", |f| f.email())
            .build();

        form.set_value("username", "ab"); // too short
        form.set_value("email", "invalid"); // no @ or .

        assert!(!form.submit());
        assert!(form.has_errors());
        let errors = form.errors();
        assert_eq!(errors.len(), 2);
    }

    // Reactive behavior tests
    #[test]
    fn test_form_reactive_validation() {
        let form = FormState::new()
            .field("email", |f| f.email().required())
            .build();

        // Initially invalid (empty + required)
        assert!(!form.is_valid());

        // Set invalid email
        form.set_value("email", "invalid");
        assert!(!form.is_valid());

        // Set valid email
        form.set_value("email", "test@example.com");
        assert!(form.is_valid());

        // Change to invalid
        form.set_value("email", "bad");
        assert!(!form.is_valid());
    }

    // Matches validator tests
    #[test]
    fn test_matches_validator() {
        let form = FormState::new()
            .field("password", |f| f.password().required().min_length(8))
            .field("confirm", |f| f.password().required().matches("password"))
            .build();

        // Set password
        form.set_value("password", "secret123");

        // Confirm doesn't match
        form.set_value("confirm", "different");
        assert!(!form.is_valid());
        assert!(form.get("confirm").unwrap().has_errors());

        // Confirm matches
        form.set_value("confirm", "secret123");
        assert!(form.is_valid());
        assert!(!form.get("confirm").unwrap().has_errors());
    }

    #[test]
    fn test_matches_validator_reactive() {
        let form = FormState::new()
            .field("password", |f| f.password())
            .field("confirm", |f| f.password().matches("password"))
            .build();

        // Both empty - should match
        assert!(form.is_valid());

        // Set both to same value
        form.set_value("password", "test123");
        form.set_value("confirm", "test123");
        assert!(form.is_valid());

        // Change password - confirm should now fail
        form.set_value("password", "changed");
        assert!(!form.is_valid());
        let confirm_errors = form.get("confirm").unwrap().errors();
        assert!(confirm_errors.iter().any(|e| e.message.contains("match")));

        // Update confirm to match
        form.set_value("confirm", "changed");
        assert!(form.is_valid());
    }

    #[test]
    fn test_matches_nonexistent_field() {
        // If matches field doesn't exist, just build without match validation
        let form = FormState::new()
            .field("confirm", |f| f.matches("nonexistent"))
            .build();

        form.set_value("confirm", "anything");
        // Should be valid since target doesn't exist
        assert!(form.is_valid());
    }

    #[test]
    fn test_form_field_update_value() {
        let field = FormFieldBuilder::new().build();
        field.update_value(|v| v.push_str("hello"));
        assert_eq!(field.value(), "hello");
        assert!(field.is_touched());
    }

    #[test]
    fn test_form_field_touch() {
        let field = FormFieldBuilder::new().build();
        assert!(!field.is_touched());
        field.touch();
        assert!(field.is_touched());
    }

    #[test]
    fn test_form_field_touched_signal() {
        let field = FormFieldBuilder::new().build();
        let signal = field.touched_signal();
        assert!(!signal.get());
        field.touch();
        assert!(signal.get());
    }

    #[test]
    fn test_form_field_builder_default() {
        let builder = FormFieldBuilder::default();
        let field = builder.build();
        assert_eq!(field.field_type, FieldType::Text);
    }

    #[test]
    fn test_form_state_builder_default() {
        let builder = FormStateBuilder::default();
        let form = builder.build();
        assert!(form.is_valid());
    }

    #[test]
    fn test_form_field_validator_method() {
        let field = FormFieldBuilder::new()
            .validator(Box::new(|v: &str| {
                if v.contains("bad") {
                    Err(ValidationError::new("Contains bad"))
                } else {
                    Ok(())
                }
            }))
            .build();

        field.set_value("good");
        assert!(field.is_valid());

        field.set_value("bad word");
        assert!(!field.is_valid());
    }
}
