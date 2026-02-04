//! Form state with multi-field management

use super::field::{FormField, FormFieldBuilder};
use crate::reactive::{signal, Signal};
use std::collections::HashMap;

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
