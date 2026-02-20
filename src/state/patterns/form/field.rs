//! Form field with builder and reactive validation

use super::types::FieldType;
use super::validators::{ValidationError, ValidatorFn, Validators};
use crate::reactive::{computed, signal, Computed, Signal};
use std::sync::Arc;

/// Builder for creating reactive form fields
pub struct FormFieldBuilder {
    field_type: FieldType,
    label: String,
    placeholder: String,
    helper_text: String,
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
            helper_text: String::new(),
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

    /// Set helper text displayed below the field
    pub fn helper_text(mut self, text: impl Into<String>) -> Self {
        self.helper_text = text.into();
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
            helper_text: self.helper_text,
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
    /// Helper text displayed below the field
    pub helper_text: String,
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

    /// Get the helper text
    pub fn helper_text(&self) -> &str {
        &self.helper_text
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
