//! Form and FormField widgets for automated form rendering
//!
//! These widgets provide automatic two-way binding with FormState,
//! reducing form creation boilerplate by 50-70%.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::prelude::*;
//! use revue::patterns::form::FormState;
//! use revue::widget::{Form, FormField};
//!
//! let form_state = FormState::new()
//!     .field("email", |f| f.label("Email").required().email())
//!     .field("password", |f| f.label("Password").required().min_length(8))
//!     .build();
//!
//! Form::new(form_state.clone())
//!     .on_submit(|data| {
//!         println!("Form submitted: {:?}", data);
//!     })
//!     .child(FormField::new("email").placeholder("Enter email"))
//!     .child(FormField::new("password").input_type(InputType::Password))
//!     .child(Button::new("Submit").submit());
//! ```

mod render;

use crate::impl_props_builders;
use crate::patterns::form::FormState;
use crate::widget::traits::WidgetProps;
use std::collections::HashMap;
use std::sync::Arc;

/// Type alias for form submit callback
type SubmitCallback = Arc<dyn Fn(HashMap<String, String>)>;

/// Input type for FormField
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InputType {
    /// Standard text input
    #[default]
    Text,
    /// Password input (masked)
    Password,
    /// Email input
    Email,
    /// Number input
    Number,
}

/// Form widget for automated form rendering with FormState binding
///
/// Cloning a `Form` gives a second view over the same form: the
/// [`FormState`]'s fields are reactive signals, so the clone and the original
/// read and write the same values (and share the submit callback). Build a
/// new `FormState` for an independent form.
#[derive(Clone)]
pub struct Form {
    /// Form state for two-way binding
    form_state: FormState,
    /// Submit callback
    on_submit: Option<SubmitCallback>,
    /// Form widget properties
    props: WidgetProps,
    /// Custom submit button text (None = default)
    submit_text: Option<String>,
    /// Whether to show validation errors inline
    show_errors: bool,
    /// Error display style
    error_style: ErrorDisplayStyle,
}

/// How to display validation errors
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ErrorDisplayStyle {
    /// Show errors below each field
    #[default]
    Inline,
    /// Show errors at the bottom of the form
    Summary,
    /// Show both inline and summary
    Both,
}

impl Form {
    /// Create a new Form with FormState binding
    pub fn new(form_state: FormState) -> Self {
        Self {
            form_state,
            on_submit: None,
            props: WidgetProps::default(),
            submit_text: None,
            show_errors: true,
            error_style: ErrorDisplayStyle::default(),
        }
    }

    /// Set submit callback
    pub fn on_submit(mut self, callback: SubmitCallback) -> Self {
        self.on_submit = Some(callback);
        self
    }

    /// Set custom submit button text
    #[deprecated(
        since = "3.7.0",
        note = "Form draws no submit button, so this text is never shown; put a `Button` with your label next to the form and call `Form::submit` from it"
    )]
    pub fn submit_text(mut self, text: impl Into<String>) -> Self {
        self.submit_text = Some(text.into());
        self
    }

    /// Set whether to show validation errors inline
    pub fn show_errors(mut self, show: bool) -> Self {
        self.show_errors = show;
        self
    }

    /// Set error display style
    pub fn error_style(mut self, style: ErrorDisplayStyle) -> Self {
        self.error_style = style;
        self
    }

    /// Get the form state
    pub fn form_state(&self) -> &FormState {
        &self.form_state
    }

    /// Check if form is valid
    pub fn is_valid(&self) -> bool {
        self.form_state.is_valid()
    }

    /// Get the number of errors in the form
    pub fn error_count(&self) -> usize {
        self.form_state.errors().len()
    }

    /// Get submit button text
    #[deprecated(
        since = "3.7.0",
        note = "Form draws no submit button, so this text is never shown; put a `Button` with your label next to the form and call `Form::submit` from it"
    )]
    pub fn get_submit_text(&self) -> Option<&String> {
        self.submit_text.as_ref()
    }

    /// Get show_errors flag
    pub fn get_show_errors(&self) -> bool {
        self.show_errors
    }

    /// Get error display style
    pub fn get_error_style(&self) -> ErrorDisplayStyle {
        self.error_style
    }

    /// Submit the form (triggers callback if valid)
    pub fn submit(&self) {
        if self.is_valid() {
            if let Some(ref callback) = self.on_submit {
                let data = self.form_state.values();
                callback(data);
            }
        }
    }
}

impl Default for Form {
    fn default() -> Self {
        Self::new(FormState::new().build())
    }
}

impl_props_builders!(Form);

/// FormField widget for individual form field rendering
#[derive(Clone)]
pub struct FormFieldWidget {
    /// Field name (key in FormState)
    name: String,
    /// Placeholder text
    placeholder: String,
    /// Helper text displayed below the field
    helper_text: String,
    /// Input type
    input_type: InputType,
    /// Widget properties
    props: WidgetProps,
    /// Whether to show label
    show_label: bool,
    /// Whether to show errors inline
    show_errors: bool,
}

#[allow(dead_code)]
impl FormFieldWidget {
    /// Create a new FormField widget
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            placeholder: String::new(),
            helper_text: String::new(),
            input_type: InputType::Text,
            props: WidgetProps::default(),
            show_label: true,
            show_errors: true,
        }
    }

    /// Set placeholder text
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = text.into();
        self
    }

    /// Set helper text displayed below the field
    pub fn helper_text(mut self, text: impl Into<String>) -> Self {
        self.helper_text = text.into();
        self
    }

    /// Set input type
    pub fn input_type(mut self, input_type: InputType) -> Self {
        self.input_type = input_type;
        self
    }

    /// Set whether to show label
    pub fn show_label(mut self, show: bool) -> Self {
        self.show_label = show;
        self
    }

    /// Set whether to show errors inline
    pub fn show_errors(mut self, show: bool) -> Self {
        self.show_errors = show;
        self
    }

    /// Get the field name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get placeholder
    pub fn get_placeholder(&self) -> Option<&String> {
        if self.placeholder.is_empty() {
            None
        } else {
            Some(&self.placeholder)
        }
    }

    /// Get input type
    pub fn get_input_type(&self) -> InputType {
        self.input_type
    }

    /// Get show_label flag
    pub fn get_show_label(&self) -> bool {
        self.show_label
    }

    /// Get show_errors flag
    pub fn get_show_errors(&self) -> bool {
        self.show_errors
    }
}

impl Default for FormFieldWidget {
    fn default() -> Self {
        Self::new("")
    }
}

impl_props_builders!(FormFieldWidget);

/// Convenience function to create a Form
pub fn form(form_state: FormState) -> Form {
    Form::new(form_state)
}

/// Convenience function to create a FormField
pub fn form_field(name: impl Into<String>) -> FormFieldWidget {
    FormFieldWidget::new(name)
}

// Re-export FormField from patterns module for convenience
pub use crate::patterns::form::FormField;
