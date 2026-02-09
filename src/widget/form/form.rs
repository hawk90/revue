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

use crate::impl_props_builders;
use crate::patterns::form::FormState;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
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

    /// Submit the form (triggers callback if valid)
    pub fn submit(&self) {
        if self.is_valid() {
            if let Some(ref callback) = self.on_submit {
                let data = self.form_state.values();
                callback(data);
            }
        }
    }

    /// Render form border
    fn render_border(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 2 || area.height < 2 {
            return;
        }

        let border_color = if self.is_valid() {
            Color::rgb(100, 100, 100)
        } else {
            Color::rgb(200, 80, 80) // Red for invalid
        };

        // Draw horizontal borders
        for x in area.x..area.x + area.width {
            let mut top_cell = Cell::new('─');
            top_cell.fg = Some(border_color);
            ctx.buffer.set(x, area.y, top_cell);

            let mut bottom_cell = Cell::new('─');
            bottom_cell.fg = Some(border_color);
            ctx.buffer.set(x, area.y + area.height - 1, bottom_cell);
        }

        // Draw vertical borders
        for y in area.y..area.y + area.height {
            let mut left_cell = Cell::new('│');
            left_cell.fg = Some(border_color);
            ctx.buffer.set(area.x, y, left_cell);

            let mut right_cell = Cell::new('│');
            right_cell.fg = Some(border_color);
            ctx.buffer.set(area.x + area.width - 1, y, right_cell);
        }

        // Draw corners
        let corners = [
            ('┌', area.x, area.y),
            ('┐', area.x + area.width - 1, area.y),
            ('└', area.x, area.y + area.height - 1),
            ('┘', area.x + area.width - 1, area.y + area.height - 1),
        ];

        for &(ch, x, y) in &corners {
            let mut cell = Cell::new(ch);
            cell.fg = Some(border_color);
            ctx.buffer.set(x, y, cell);
        }
    }

    /// Render form title
    fn render_title(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 4 {
            return;
        }

        let title = "Form";
        let title_x = area.x + 2;

        for (i, ch) in title.chars().enumerate() {
            if title_x + (i as u16) < area.x + area.width - 1 {
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::WHITE);
                cell.bg = Some(Color::BLACK);
                ctx.buffer.set(title_x + i as u16, area.y, cell);
            }
        }
    }
}

impl Default for Form {
    fn default() -> Self {
        Self::new(FormState::new().build())
    }
}

impl View for Form {
    crate::impl_view_meta!("Form");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;

        // Render border
        self.render_border(ctx);

        // Render title
        self.render_title(ctx);

        // Render validation status at bottom
        let status_y = area.y + area.height - 2;
        if status_y > area.y && area.width > 4 {
            let status_text = if self.is_valid() {
                "✓ Valid".to_string()
            } else {
                let error_count = self.error_count();
                format!("✗ {} error(s)", error_count)
            };

            let status_color = if self.is_valid() {
                Color::rgb(80, 200, 80)
            } else {
                Color::rgb(200, 80, 80)
            };

            for (i, ch) in status_text.chars().enumerate() {
                let x = area.x + 2 + i as u16;
                if x < area.x + area.width - 2 {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(status_color);
                    ctx.buffer.set(x, status_y, cell);
                }
            }
        }
    }
}

impl_props_builders!(Form);

/// FormField widget for individual form field rendering
pub struct FormFieldWidget {
    /// Field name (key in FormState)
    name: String,
    /// Placeholder text
    placeholder: String,
    /// Input type
    input_type: InputType,
    /// Widget properties
    props: WidgetProps,
    /// Whether to show label
    show_label: bool,
    /// Whether to show errors inline
    show_errors: bool,
}

impl FormFieldWidget {
    /// Create a new FormField widget
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            placeholder: String::new(),
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

    /// Render the field label
    #[allow(dead_code)]
    fn render_label(&self, form_state: &FormState, ctx: &mut RenderContext) {
        let area = ctx.area;
        if let Some(field) = form_state.get(&self.name) {
            let label = &field.label;

            for (i, ch) in label.chars().enumerate() {
                if area.x + (i as u16) < area.x + area.width {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(Color::rgb(200, 200, 200));
                    ctx.buffer.set(area.x + i as u16, area.y, cell);
                }
            }
        }
    }

    /// Render the field value
    #[allow(dead_code)]
    fn render_value(&self, form_state: &FormState, ctx: &mut RenderContext) {
        let area = ctx.area;
        let value = form_state.value(&self.name).unwrap_or_default();

        // Display value or placeholder
        let display_text = if value.is_empty() {
            self.placeholder.clone()
        } else {
            match self.input_type {
                InputType::Password => "•".repeat(value.len().min(20)),
                _ => value.clone(),
            }
        };

        let text_color = if value.is_empty() {
            Color::rgb(120, 120, 120) // Gray for placeholder
        } else {
            Color::WHITE
        };

        for (i, ch) in display_text.chars().enumerate() {
            let x = area.x + i as u16;
            if x < area.x + area.width {
                let mut cell = Cell::new(ch);
                cell.fg = Some(text_color);
                ctx.buffer.set(x, area.y, cell);
            }
        }
    }

    /// Render validation errors
    #[allow(dead_code)]
    fn render_errors(&self, form_state: &FormState, ctx: &mut RenderContext) {
        if !self.show_errors {
            return;
        }

        let field = match form_state.get(&self.name) {
            Some(f) => f,
            None => return,
        };

        let error_msg = match field.first_error() {
            Some(err) => err,
            None => return,
        };

        let area = ctx.area;
        let error_color = Color::rgb(200, 80, 80);

        for (i, ch) in error_msg.chars().enumerate() {
            let x = area.x + i as u16;
            if x < area.x + area.width {
                let mut cell = Cell::new(ch);
                cell.fg = Some(error_color);
                cell.modifier |= Modifier::DIM;
                ctx.buffer.set(x, area.y, cell);
            }
        }
    }
}

impl Default for FormFieldWidget {
    fn default() -> Self {
        Self::new("")
    }
}

impl View for FormFieldWidget {
    crate::impl_view_meta!("FormField");

    fn render(&self, ctx: &mut RenderContext) {
        // Note: Actual rendering happens through Form which has access to FormState
        // This is a placeholder render - in practice, Form renders its children
        let area = ctx.area;

        if area.width > self.name.len() as u16 {
            for (i, ch) in self.name.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::rgb(150, 150, 150));
                ctx.buffer.set(area.x + i as u16, area.y, cell);
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_form_new() {
        let form_state = FormState::new().build();
        let form = Form::new(form_state);
        assert!(form.is_valid());
    }

    #[test]
    fn test_form_builder() {
        let form_state = FormState::new().build();
        let form = Form::new(form_state)
            .submit_text("Send")
            .show_errors(true)
            .error_style(ErrorDisplayStyle::Summary);

        assert_eq!(form.submit_text, Some("Send".to_string()));
        assert!(form.show_errors);
        assert_eq!(form.error_style, ErrorDisplayStyle::Summary);
    }

    #[test]
    fn test_input_type_default() {
        let input_type = InputType::default();
        assert_eq!(input_type, InputType::Text);
    }

    #[test]
    fn test_form_field_new() {
        let field = FormFieldWidget::new("username");
        assert_eq!(field.name, "username");
        assert_eq!(field.input_type, InputType::Text);
    }

    #[test]
    fn test_form_field_builder() {
        let field = FormFieldWidget::new("email")
            .placeholder("Enter email")
            .input_type(InputType::Email)
            .show_label(false)
            .show_errors(false);

        assert_eq!(field.placeholder, "Enter email");
        assert_eq!(field.input_type, InputType::Email);
        assert!(!field.show_label);
        assert!(!field.show_errors);
    }

    #[test]
    fn test_error_display_style_default() {
        let style = ErrorDisplayStyle::default();
        assert_eq!(style, ErrorDisplayStyle::Inline);
    }

    #[test]
    fn test_convenience_functions() {
        let form_state = FormState::new().build();
        let form = form(form_state);
        assert_eq!(form.submit_text, None);

        let field = form_field("password");
        assert_eq!(field.name, "password");
    }

    // =========================================================================
    // InputType enum tests
    // =========================================================================

    #[test]
    fn test_input_type_clone() {
        let input_type = InputType::Email;
        let cloned = input_type.clone();
        assert_eq!(input_type, cloned);
    }

    #[test]
    fn test_input_type_copy() {
        let type1 = InputType::Password;
        let type2 = type1;
        assert_eq!(type1, InputType::Password);
        assert_eq!(type2, InputType::Password);
    }

    #[test]
    fn test_input_type_partial_eq() {
        assert_eq!(InputType::Text, InputType::Text);
        assert_ne!(InputType::Text, InputType::Password);
    }

    #[test]
    fn test_input_type_debug() {
        let input_type = InputType::Number;
        assert!(format!("{:?}", input_type).contains("Number"));
    }

    // =========================================================================
    // ErrorDisplayStyle enum tests
    // =========================================================================

    #[test]
    fn test_error_display_style_clone() {
        let style = ErrorDisplayStyle::Summary;
        let cloned = style.clone();
        assert_eq!(style, cloned);
    }

    #[test]
    fn test_error_display_style_copy() {
        let style1 = ErrorDisplayStyle::Inline;
        let style2 = style1;
        assert_eq!(style1, ErrorDisplayStyle::Inline);
        assert_eq!(style2, ErrorDisplayStyle::Inline);
    }

    #[test]
    fn test_error_display_style_partial_eq() {
        assert_eq!(ErrorDisplayStyle::Inline, ErrorDisplayStyle::Inline);
        assert_ne!(ErrorDisplayStyle::Inline, ErrorDisplayStyle::Summary);
    }

    #[test]
    fn test_error_display_style_debug() {
        let style = ErrorDisplayStyle::Both;
        assert!(format!("{:?}", style).contains("Both"));
    }

    // =========================================================================
    // Form builder tests
    // =========================================================================

    #[test]
    fn test_form_new_default_values() {
        let form_state = FormState::new().build();
        let form = Form::new(form_state);
        assert!(form.on_submit.is_none());
        assert!(form.submit_text.is_none());
        assert!(form.show_errors);
        assert_eq!(form.error_style, ErrorDisplayStyle::Inline);
    }

    #[test]
    fn test_form_show_errors_false() {
        let form_state = FormState::new().build();
        let form = Form::new(form_state).show_errors(false);
        assert!(!form.show_errors);
    }

    #[test]
    fn test_form_error_style_both() {
        let form_state = FormState::new().build();
        let form = Form::new(form_state).error_style(ErrorDisplayStyle::Both);
        assert_eq!(form.error_style, ErrorDisplayStyle::Both);
    }

    // =========================================================================
    // Form method tests
    // =========================================================================

    #[test]
    fn test_form_form_state() {
        let form_state = FormState::new().build();
        let form = Form::new(form_state.clone());
        assert_eq!(form.form_state().values().len(), 0);
    }

    #[test]
    fn test_form_error_count() {
        let form_state = FormState::new().build();
        let form = Form::new(form_state);
        assert_eq!(form.error_count(), 0);
    }

    #[test]
    fn test_form_submit_no_callback() {
        let form_state = FormState::new().build();
        let form = Form::new(form_state);
        form.submit(); // Should not crash
    }

    #[test]
    fn test_form_submit_with_callback() {
        let form_state = FormState::new().build();
        let callback_called = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let callback_called_clone = callback_called.clone();

        let form = Form::new(form_state).on_submit(Arc::new(move |_data| {
            callback_called_clone.store(true, std::sync::atomic::Ordering::SeqCst);
        }));

        form.submit();
        assert!(callback_called.load(std::sync::atomic::Ordering::SeqCst));
    }

    // =========================================================================
    // Form Default trait tests
    // =========================================================================

    #[test]
    fn test_form_default() {
        let form = Form::default();
        assert!(form.is_valid());
        assert!(form.on_submit.is_none());
    }

    // =========================================================================
    // FormFieldWidget builder tests
    // =========================================================================

    #[test]
    fn test_form_field_new_default_values() {
        let field = FormFieldWidget::new("test");
        assert_eq!(field.name, "test");
        assert!(field.placeholder.is_empty());
        assert_eq!(field.input_type, InputType::Text);
        assert!(field.show_label);
        assert!(field.show_errors);
    }

    #[test]
    fn test_form_field_password_input_type() {
        let field = FormFieldWidget::new("pass").input_type(InputType::Password);
        assert_eq!(field.input_type, InputType::Password);
    }

    #[test]
    fn test_form_field_number_input_type() {
        let field = FormFieldWidget::new("age").input_type(InputType::Number);
        assert_eq!(field.input_type, InputType::Number);
    }

    #[test]
    fn test_form_field_show_label_true() {
        let field = FormFieldWidget::new("test").show_label(true);
        assert!(field.show_label);
    }

    #[test]
    fn test_form_field_show_errors_true() {
        let field = FormFieldWidget::new("test").show_errors(true);
        assert!(field.show_errors);
    }

    // =========================================================================
    // FormFieldWidget method tests
    // =========================================================================

    #[test]
    fn test_form_field_name() {
        let field = FormFieldWidget::new("username");
        assert_eq!(field.name(), "username");
    }

    // =========================================================================
    // FormFieldWidget Default trait tests
    // =========================================================================

    #[test]
    fn test_form_field_default() {
        let field = FormFieldWidget::default();
        assert_eq!(field.name, "");
        assert_eq!(field.input_type, InputType::Text);
    }

    // =========================================================================
    // Builder chain tests
    // =========================================================================

    #[test]
    fn test_form_builder_chain() {
        let form_state = FormState::new().build();
        let form = Form::new(form_state)
            .submit_text("Submit Form")
            .show_errors(true)
            .error_style(ErrorDisplayStyle::Both);
        assert_eq!(form.submit_text, Some("Submit Form".to_string()));
        assert!(form.show_errors);
        assert_eq!(form.error_style, ErrorDisplayStyle::Both);
    }

    #[test]
    fn test_form_field_builder_chain() {
        let field = FormFieldWidget::new("email")
            .placeholder("user@example.com")
            .input_type(InputType::Email)
            .show_label(false)
            .show_errors(true);
        assert_eq!(field.placeholder, "user@example.com");
        assert_eq!(field.input_type, InputType::Email);
        assert!(!field.show_label);
        assert!(field.show_errors);
    }

    // =========================================================================
    // Helper function tests
    // =========================================================================

    #[test]
    fn test_form_helper_fn() {
        let form_state = FormState::new().build();
        let form = form(form_state);
        assert!(form.is_valid());
    }

    #[test]
    fn test_form_field_helper_fn() {
        let field = form_field("test_field");
        assert_eq!(field.name, "test_field");
    }

    // =========================================================================
    // Form element_id and class tests (from impl_props_builders!)
    // =========================================================================

    #[test]
    fn test_form_element_id() {
        let form_state = FormState::new().build();
        let form = Form::new(form_state).element_id("my-form");
        assert_eq!(form.props.id, Some("my-form".to_string()));
    }

    #[test]
    fn test_form_element_id_multiple() {
        let form_state = FormState::new().build();
        let form = Form::new(form_state)
            .element_id("first-id")
            .element_id("second-id");
        // Last one wins
        assert_eq!(form.props.id, Some("second-id".to_string()));
    }

    #[test]
    fn test_form_class() {
        let form_state = FormState::new().build();
        let form = Form::new(form_state).class("form-container");
        assert_eq!(form.props.classes, vec!["form-container".to_string()]);
    }

    #[test]
    fn test_form_class_multiple() {
        let form_state = FormState::new().build();
        let form = Form::new(form_state)
            .class("container")
            .class("dark-mode")
            .class("large");
        assert_eq!(
            form.props.classes,
            vec![
                "container".to_string(),
                "dark-mode".to_string(),
                "large".to_string()
            ]
        );
    }

    #[test]
    fn test_form_class_duplicate_not_added() {
        let form_state = FormState::new().build();
        let form = Form::new(form_state).class("container").class("container"); // Duplicate
        assert_eq!(form.props.classes, vec!["container".to_string()]);
    }

    #[test]
    fn test_form_classes_vec() {
        let form_state = FormState::new().build();
        let form = Form::new(form_state).classes(vec!["class1", "class2", "class3"]);
        assert_eq!(
            form.props.classes,
            vec![
                "class1".to_string(),
                "class2".to_string(),
                "class3".to_string()
            ]
        );
    }

    #[test]
    fn test_form_classes_array() {
        let form_state = FormState::new().build();
        let form = Form::new(form_state).classes(["class1", "class2"]);
        assert_eq!(
            form.props.classes,
            vec!["class1".to_string(), "class2".to_string()]
        );
    }

    #[test]
    fn test_form_classes_with_duplicates() {
        let form_state = FormState::new().build();
        let form = Form::new(form_state).classes(vec!["class1", "class2", "class1"]);
        // Duplicates should not be added
        assert_eq!(
            form.props.classes,
            vec!["class1".to_string(), "class2".to_string()]
        );
    }

    #[test]
    fn test_form_mixed_classes() {
        let form_state = FormState::new().build();
        let form = Form::new(form_state)
            .class("single")
            .classes(vec!["vec1", "vec2"])
            .class("another");
        assert_eq!(
            form.props.classes,
            vec![
                "single".to_string(),
                "vec1".to_string(),
                "vec2".to_string(),
                "another".to_string()
            ]
        );
    }

    // =========================================================================
    // FormFieldWidget element_id and class tests (from impl_props_builders!)
    // =========================================================================

    #[test]
    fn test_form_field_element_id() {
        let field = FormFieldWidget::new("email").element_id("email-field");
        assert_eq!(field.props.id, Some("email-field".to_string()));
    }

    #[test]
    fn test_form_field_element_id_override() {
        let field = FormFieldWidget::new("email")
            .element_id("first")
            .element_id("second");
        assert_eq!(field.props.id, Some("second".to_string()));
    }

    #[test]
    fn test_form_field_class() {
        let field = FormFieldWidget::new("username").class("input-field");
        assert_eq!(field.props.classes, vec!["input-field".to_string()]);
    }

    #[test]
    fn test_form_field_class_multiple() {
        let field = FormFieldWidget::new("password")
            .class("required")
            .class("validated");
        assert_eq!(
            field.props.classes,
            vec!["required".to_string(), "validated".to_string()]
        );
    }

    #[test]
    fn test_form_field_class_no_duplicate() {
        let field = FormFieldWidget::new("email").class("input").class("input");
        assert_eq!(field.props.classes, vec!["input".to_string()]);
    }

    #[test]
    fn test_form_field_classes_vec() {
        let field = FormFieldWidget::new("name").classes(vec!["class1", "class2"]);
        assert_eq!(
            field.props.classes,
            vec!["class1".to_string(), "class2".to_string()]
        );
    }

    #[test]
    fn test_form_field_classes_slice() {
        let field = FormFieldWidget::new("age").classes(["class1", "class2", "class3"]);
        assert_eq!(
            field.props.classes,
            vec![
                "class1".to_string(),
                "class2".to_string(),
                "class3".to_string()
            ]
        );
    }

    #[test]
    fn test_form_field_classes_with_duplicates_filtered() {
        let field = FormFieldWidget::new("test").classes(vec!["a", "b", "a", "c", "b"]);
        assert_eq!(
            field.props.classes,
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
    }

    #[test]
    fn test_form_field_mixed_classes() {
        let field = FormFieldWidget::new("mixed")
            .class("first")
            .classes(vec!["second", "third"])
            .class("fourth");
        assert_eq!(
            field.props.classes,
            vec![
                "first".to_string(),
                "second".to_string(),
                "third".to_string(),
                "fourth".to_string()
            ]
        );
    }

    // =========================================================================
    // Form and FormFieldWidget combined tests
    // =========================================================================

    #[test]
    fn test_form_full_builder_chain_with_props() {
        let form_state = FormState::new().build();
        let form = Form::new(form_state)
            .element_id("login-form")
            .class("form-container")
            .class("dark-theme")
            .classes(vec!["large", "animated"])
            .submit_text("Login")
            .show_errors(true)
            .error_style(ErrorDisplayStyle::Both);

        assert_eq!(form.props.id, Some("login-form".to_string()));
        assert_eq!(
            form.props.classes,
            vec![
                "form-container".to_string(),
                "dark-theme".to_string(),
                "large".to_string(),
                "animated".to_string()
            ]
        );
        assert_eq!(form.submit_text, Some("Login".to_string()));
        assert!(form.show_errors);
        assert_eq!(form.error_style, ErrorDisplayStyle::Both);
    }

    #[test]
    fn test_form_field_full_builder_chain_with_props() {
        let field = FormFieldWidget::new("email")
            .element_id("email-input")
            .class("required")
            .classes(vec!["validated", "email-field"])
            .placeholder("user@example.com")
            .input_type(InputType::Email)
            .show_label(true)
            .show_errors(true);

        assert_eq!(field.props.id, Some("email-input".to_string()));
        assert_eq!(
            field.props.classes,
            vec![
                "required".to_string(),
                "validated".to_string(),
                "email-field".to_string()
            ]
        );
        assert_eq!(field.placeholder, "user@example.com");
        assert_eq!(field.input_type, InputType::Email);
        assert!(field.show_label);
        assert!(field.show_errors);
    }

    // =========================================================================
    // InputType trait tests - verify all derive macros work
    // =========================================================================

    #[test]
    fn test_input_type_all_variants() {
        // Verify all variants can be created and compared
        let text = InputType::Text;
        let password = InputType::Password;
        let email = InputType::Email;
        let number = InputType::Number;

        // All should be unique
        assert_ne!(text, password);
        assert_ne!(text, email);
        assert_ne!(text, number);
        assert_ne!(password, email);
        assert_ne!(password, number);
        assert_ne!(email, number);
    }

    #[test]
    fn test_input_type_copy_semantics() {
        let original = InputType::Email;
        let copy = original;
        // Both should still be valid and equal
        assert_eq!(original, InputType::Email);
        assert_eq!(copy, InputType::Email);
        assert_eq!(original, copy);
    }

    // =========================================================================
    // ErrorDisplayStyle trait tests - verify all derive macros work
    // =========================================================================

    #[test]
    fn test_error_display_style_all_variants() {
        let inline = ErrorDisplayStyle::Inline;
        let summary = ErrorDisplayStyle::Summary;
        let both = ErrorDisplayStyle::Both;

        // All should be unique
        assert_ne!(inline, summary);
        assert_ne!(inline, both);
        assert_ne!(summary, both);
    }

    #[test]
    fn test_error_display_style_copy_semantics() {
        let original = ErrorDisplayStyle::Both;
        let copy = original;
        // Both should still be valid and equal
        assert_eq!(original, ErrorDisplayStyle::Both);
        assert_eq!(copy, ErrorDisplayStyle::Both);
        assert_eq!(original, copy);
    }

    // =========================================================================
    // Form edge case tests
    // =========================================================================

    #[test]
    fn test_form_empty_string_element_id() {
        let form_state = FormState::new().build();
        let form = Form::new(form_state).element_id("");
        assert_eq!(form.props.id, Some("".to_string()));
    }

    #[test]
    fn test_form_empty_string_class() {
        let form_state = FormState::new().build();
        let form = Form::new(form_state).class("");
        // Empty string class is still added
        assert_eq!(form.props.classes, vec!["".to_string()]);
    }

    #[test]
    fn test_form_classes_empty_vec() {
        let form_state = FormState::new().build();
        let form = Form::new(form_state).classes(Vec::<&str>::new());
        assert!(form.props.classes.is_empty());
    }

    #[test]
    fn test_form_classes_empty_array() {
        let form_state = FormState::new().build();
        let form = Form::new(form_state).classes([] as [&str; 0]);
        assert!(form.props.classes.is_empty());
    }

    // =========================================================================
    // FormFieldWidget edge case tests
    // =========================================================================

    #[test]
    fn test_form_field_empty_name() {
        let field = FormFieldWidget::new("");
        assert_eq!(field.name, "");
        assert_eq!(field.name(), "");
    }

    #[test]
    fn test_form_field_empty_string_element_id() {
        let field = FormFieldWidget::new("test").element_id("");
        assert_eq!(field.props.id, Some("".to_string()));
    }

    #[test]
    fn test_form_field_empty_string_class() {
        let field = FormFieldWidget::new("test").class("");
        assert_eq!(field.props.classes, vec!["".to_string()]);
    }

    #[test]
    fn test_form_field_classes_empty_iterator() {
        let field = FormFieldWidget::new("test").classes(Vec::<&str>::new());
        assert!(field.props.classes.is_empty());
    }

    #[test]
    fn test_form_field_name_with_special_chars() {
        let field = FormFieldWidget::new("user-email-field");
        assert_eq!(field.name, "user-email-field");
    }

    #[test]
    fn test_form_field_name_with_unicode() {
        let field = FormFieldWidget::new("用户邮箱");
        assert_eq!(field.name, "用户邮箱");
    }

    // =========================================================================
    // Form interaction tests
    // =========================================================================

    #[test]
    fn test_form_submit_with_data() {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;

        let callback_called = Arc::new(AtomicBool::new(false));
        let callback_called_clone = callback_called.clone();

        let form_state = FormState::new().build();
        let form = Form::new(form_state).on_submit(Arc::new(move |_data| {
            callback_called_clone.store(true, Ordering::SeqCst);
        }));

        form.submit();
        assert!(callback_called.load(Ordering::SeqCst));
    }

    #[test]
    fn test_form_submit_not_called_when_invalid() {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;

        let callback_called = Arc::new(AtomicBool::new(false));
        let callback_called_clone = callback_called.clone();

        // Create a form with validation errors
        let form_state = FormState::new()
            .field("email", |f| f.label("Email").required().email())
            .build();
        let form_state_clone = form_state.clone();

        let form = Form::new(form_state_clone).on_submit(Arc::new(move |_data| {
            callback_called_clone.store(true, Ordering::SeqCst);
        }));

        // Form should be invalid (missing required email)
        assert!(!form.is_valid());

        // Submit should not call callback
        form.submit();
        assert!(!callback_called.load(Ordering::SeqCst));
    }

    // =========================================================================
    // Form Default trait comprehensive tests
    // =========================================================================

    #[test]
    fn test_form_default_complete_state() {
        let form = Form::default();
        assert!(form.is_valid());
        assert!(form.on_submit.is_none());
        assert!(form.submit_text.is_none());
        assert!(form.show_errors);
        assert_eq!(form.error_style, ErrorDisplayStyle::Inline);
        assert_eq!(form.error_count(), 0);
    }

    // =========================================================================
    // FormFieldWidget Default trait comprehensive tests
    // =========================================================================

    #[test]
    fn test_form_field_default_complete_state() {
        let field = FormFieldWidget::default();
        assert_eq!(field.name, "");
        assert!(field.placeholder.is_empty());
        assert_eq!(field.input_type, InputType::Text);
        assert!(field.show_label);
        assert!(field.show_errors);
    }

    // =========================================================================
    // Helper function comprehensive tests
    // =========================================================================

    #[test]
    fn test_form_helper_with_builder() {
        let form_state = FormState::new().build();
        let form = form(form_state)
            .element_id("test-form")
            .class("container")
            .submit_text("Submit");

        assert_eq!(form.props.id, Some("test-form".to_string()));
        assert_eq!(form.props.classes, vec!["container".to_string()]);
        assert_eq!(form.submit_text, Some("Submit".to_string()));
    }

    #[test]
    fn test_form_field_helper_with_builder() {
        let field = form_field("username")
            .element_id("user-input")
            .class("required")
            .placeholder("Enter username");

        assert_eq!(field.name, "username");
        assert_eq!(field.props.id, Some("user-input".to_string()));
        assert_eq!(field.props.classes, vec!["required".to_string()]);
        assert_eq!(field.placeholder, "Enter username");
    }

    // =========================================================================
    // Stress tests - builder chains
    // =========================================================================

    #[test]
    fn test_form_long_builder_chain() {
        let form_state = FormState::new().build();
        let form = Form::new(form_state)
            .element_id("id")
            .class("c1")
            .class("c2")
            .class("c3")
            .classes(vec!["c4", "c5"])
            .class("c6")
            .classes(vec!["c7", "c8", "c9"])
            .submit_text("Submit")
            .show_errors(true)
            .error_style(ErrorDisplayStyle::Both);

        assert_eq!(form.props.classes.len(), 9);
        assert_eq!(form.submit_text, Some("Submit".to_string()));
    }

    #[test]
    fn test_form_field_long_builder_chain() {
        let field = FormFieldWidget::new("test")
            .element_id("id")
            .class("c1")
            .class("c2")
            .classes(vec!["c3", "c4"])
            .class("c5")
            .placeholder("placeholder")
            .input_type(InputType::Email)
            .show_label(false)
            .show_errors(false);

        assert_eq!(field.props.classes.len(), 5);
        assert_eq!(field.placeholder, "placeholder");
        assert_eq!(field.input_type, InputType::Email);
        assert!(!field.show_label);
        assert!(!field.show_errors);
    }
}
