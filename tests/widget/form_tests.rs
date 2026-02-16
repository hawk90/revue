//! Tests for form and FormField widgets
//!
//! Extracted from src/widget/form/form.rs

use revue::patterns::form::FormState;
use revue::widget::form::{ErrorDisplayStyle, Form, FormFieldWidget, InputType};
use revue::widget::form;
use revue::widget::form_field;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

// =========================================================================
// InputType enum tests
// =========================================================================

#[test]
fn test_input_type_default() {
    let input_type = InputType::default();
    assert_eq!(input_type, InputType::Text);
}

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
// ErrorDisplayStyle enum tests
// =========================================================================

#[test]
fn test_error_display_style_default() {
    let style = ErrorDisplayStyle::default();
    assert_eq!(style, ErrorDisplayStyle::Inline);
}

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
// Form creation and builder tests
// =========================================================================

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

    assert_eq!(form\.get_submit_text(), Some(&"Send".to_string()));
    assert!(form\.get_show_errors());
    assert_eq!(form\.get_error_style(), ErrorDisplayStyle::Summary);
}

#[test]
fn test_form_new_default_values() {
    let form_state = FormState::new().build();
    let form = Form::new(form_state);
    assert!(form\.get_submit_text().is_none());
    assert!(form\.get_show_errors());
    assert_eq!(form\.get_error_style(), ErrorDisplayStyle::Inline);
}

#[test]
fn test_form_show_errors_false() {
    let form_state = FormState::new().build();
    let form = Form::new(form_state).show_errors(false);
    assert!(!form\.get_show_errors());
}

#[test]
fn test_form_error_style_both() {
    let form_state = FormState::new().build();
    let form = Form::new(form_state).error_style(ErrorDisplayStyle::Both);
    assert_eq!(form\.get_error_style(), ErrorDisplayStyle::Both);
}

#[test]
fn test_form_builder_chain() {
    let form_state = FormState::new().build();
    let form = Form::new(form_state)
        .submit_text("Submit Form")
        .show_errors(true)
        .error_style(ErrorDisplayStyle::Both);
    assert_eq!(form\.get_submit_text(), Some(&"Submit Form".to_string()));
    assert!(form\.get_show_errors());
    assert_eq!(form\.get_error_style(), ErrorDisplayStyle::Both);
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
    let callback_called = Arc::new(AtomicBool::new(false));
    let callback_called_clone = callback_called.clone();

    let form = Form::new(form_state).on_submit(Arc::new(move |_data| {
        callback_called_clone.store(true, Ordering::SeqCst);
    }));

    form.submit();
    assert!(callback_called.load(Ordering::SeqCst));
}

#[test]
fn test_form_submit_with_data() {
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
// Form Default trait tests
// =========================================================================

#[test]
fn test_form_default() {
    let form = Form::default();
    assert!(form.is_valid());
    assert!(form\.get_submit_text().is_none());
}

#[test]
fn test_form_default_complete_state() {
    let form = Form::default();
    assert!(form.is_valid());
    assert!(form\.get_submit_text().is_none());
    assert!(form\.get_show_errors());
    assert_eq!(form\.get_error_style(), ErrorDisplayStyle::Inline);
    assert_eq!(form.error_count(), 0);
}

// =========================================================================
// FormFieldWidget creation and builder tests
// =========================================================================

#[test]
fn test_form_field_new() {
    let field = FormFieldWidget::new("username");
    assert_eq!(field\.name(), "username");
    assert_eq!(field\.get_input_type(), InputType::Text);
}

#[test]
fn test_form_field_builder() {
    let field = FormFieldWidget::new("email")
        .placeholder("Enter email")
        .input_type(InputType::Email)
        .show_label(false)
        .show_errors(false);

    assert_eq!(field\.get_placeholder(), Some(&"Enter email".to_string()));
    assert_eq!(field\.get_input_type(), InputType::Email);
    assert!(!field\.get_show_label());
    assert!(!field\.get_show_errors());
}

#[test]
fn test_form_field_new_default_values() {
    let field = FormFieldWidget::new("test");
    assert_eq!(field\.name(), "test");
    assert_eq!(field\.get_placeholder(), None);
    assert_eq!(field\.get_input_type(), InputType::Text);
    assert!(field\.get_show_label());
    assert!(field\.get_show_errors());
}

#[test]
fn test_form_field_password_input_type() {
    let field = FormFieldWidget::new("pass").input_type(InputType::Password);
    assert_eq!(field\.get_input_type(), InputType::Password);
}

#[test]
fn test_form_field_number_input_type() {
    let field = FormFieldWidget::new("age").input_type(InputType::Number);
    assert_eq!(field\.get_input_type(), InputType::Number);
}

#[test]
fn test_form_field_show_label_true() {
    let field = FormFieldWidget::new("test").show_label(true);
    assert!(field\.get_show_label());
}

#[test]
fn test_form_field_show_errors_true() {
    let field = FormFieldWidget::new("test").show_errors(true);
    assert!(field\.get_show_errors());
}

#[test]
fn test_form_field_builder_chain() {
    let field = FormFieldWidget::new("email")
        .placeholder("user@example.com")
        .input_type(InputType::Email)
        .show_label(false)
        .show_errors(true);
    assert_eq!(field\.get_placeholder(), Some(&"user@example.com".to_string()));
    assert_eq!(field\.get_input_type(), InputType::Email);
    assert!(!field\.get_show_label());
    assert!(field\.get_show_errors());
}

// =========================================================================
// FormFieldWidget method tests
// =========================================================================

#[test]
fn test_form_field_name() {
    let field = FormFieldWidget::new("username");
    assert_eq!(field\.name(), "username");
}

// =========================================================================
// FormFieldWidget Default trait tests
// =========================================================================

#[test]
fn test_form_field_default() {
    let field = FormFieldWidget::default();
    assert_eq!(field\.name(), "");
    assert_eq!(field\.get_input_type(), InputType::Text);
}

#[test]
fn test_form_field_default_complete_state() {
    let field = FormFieldWidget::default();
    assert_eq!(field\.name(), "");
    assert_eq!(field\.get_placeholder(), None);
    assert_eq!(field\.get_input_type(), InputType::Text);
    assert!(field\.get_show_label());
    assert!(field\.get_show_errors());
}

// =========================================================================
// Helper function tests
// =========================================================================

#[test]
fn test_convenience_functions() {
    let form_state = FormState::new().build();
    let form = form(form_state);
    assert!(form\.get_submit_text().is_none());

    let field = form_field("password");
    assert_eq!(field\.name(), "password");
}

#[test]
fn test_form_helper_fn() {
    let form_state = FormState::new().build();
    let form = form(form_state);
    assert!(form.is_valid());
}

#[test]
fn test_form_field_helper_fn() {
    let field = form_field("test_field");
    assert_eq!(field\.name(), "test_field");
}

#[test]
fn test_form_helper_with_builder() {
    let form_state = FormState::new().build();
    let form = form(form_state)
        .element_id("test-form")
        .class("container")
        .submit_text("Submit");

    assert_eq!(form.element_id(), Some(&"test-form".to_string()));
    assert_eq!(form.classes(), &["container".to_string()]);
    assert_eq!(form\.get_submit_text(), Some(&"Submit".to_string()));
}

#[test]
fn test_form_field_helper_with_builder() {
    let field = form_field("username")
        .element_id("user-input")
        .class("required")
        .placeholder("Enter username");

    assert_eq!(field\.name(), "username");
    assert_eq!(field.element_id(), Some(&"user-input".to_string()));
    assert_eq!(field.classes(), &["required".to_string()]);
    assert_eq!(field\.get_placeholder(), Some(&"Enter username".to_string()));
}

// =========================================================================
// Form element_id and class tests
// =========================================================================

#[test]
fn test_form_element_id() {
    let form_state = FormState::new().build();
    let form = Form::new(form_state).element_id("my-form");
    assert_eq!(form.element_id(), Some(&"my-form".to_string()));
}

#[test]
fn test_form_element_id_multiple() {
    let form_state = FormState::new().build();
    let form = Form::new(form_state)
        .element_id("first-id")
        .element_id("second-id");
    // Last one wins
    assert_eq!(form.element_id(), Some(&"second-id".to_string()));
}

#[test]
fn test_form_class() {
    let form_state = FormState::new().build();
    let form = Form::new(form_state).class("form-container");
    assert_eq!(form.classes(), &["form-container".to_string()]);
}

#[test]
fn test_form_class_multiple() {
    let form_state = FormState::new().build();
    let form = Form::new(form_state)
        .class("container")
        .class("dark-mode")
        .class("large");
    assert_eq!(
        form.classes(),
        &[
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
    assert_eq!(form.classes(), &["container".to_string()]);
}

#[test]
fn test_form_classes_vec() {
    let form_state = FormState::new().build();
    let form = Form::new(form_state).classes(vec!["class1", "class2", "class3"]);
    assert_eq!(
        form.classes(),
        &[
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
        form.classes(),
        &["class1".to_string(), "class2".to_string()]
    );
}

#[test]
fn test_form_classes_with_duplicates() {
    let form_state = FormState::new().build();
    let form = Form::new(form_state).classes(vec!["class1", "class2", "class1"]);
    // Duplicates should not be added
    assert_eq!(
        form.classes(),
        &["class1".to_string(), "class2".to_string()]
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
        form.classes(),
        &[
            "single".to_string(),
            "vec1".to_string(),
            "vec2".to_string(),
            "another".to_string()
        ]
    );
}

// =========================================================================
// FormFieldWidget element_id and class tests
// =========================================================================

#[test]
fn test_form_field_element_id() {
    let field = FormFieldWidget::new("email").element_id("email-field");
    assert_eq!(field.element_id(), Some(&"email-field".to_string()));
}

#[test]
fn test_form_field_element_id_override() {
    let field = FormFieldWidget::new("email")
        .element_id("first")
        .element_id("second");
    assert_eq!(field.element_id(), Some(&"second".to_string()));
}

#[test]
fn test_form_field_class() {
    let field = FormFieldWidget::new("username").class("input-field");
    assert_eq!(field.classes(), &["input-field".to_string()]);
}

#[test]
fn test_form_field_class_multiple() {
    let field = FormFieldWidget::new("password")
        .class("required")
        .class("validated");
    assert_eq!(
        field.classes(),
        &["required".to_string(), "validated".to_string()]
    );
}

#[test]
fn test_form_field_class_no_duplicate() {
    let field = FormFieldWidget::new("email").class("input").class("input");
    assert_eq!(field.classes(), &["input".to_string()]);
}

#[test]
fn test_form_field_classes_vec() {
    let field = FormFieldWidget::new("name").classes(vec!["class1", "class2"]);
    assert_eq!(
        field.classes(),
        &["class1".to_string(), "class2".to_string()]
    );
}

#[test]
fn test_form_field_classes_slice() {
    let field = FormFieldWidget::new("age").classes(["class1", "class2", "class3"]);
    assert_eq!(
        field.classes(),
        &[
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
        field.classes(),
        &["a".to_string(), "b".to_string(), "c".to_string()]
    );
}

#[test]
fn test_form_field_mixed_classes() {
    let field = FormFieldWidget::new("mixed")
        .class("first")
        .classes(vec!["second", "third"])
        .class("fourth");
    assert_eq!(
        field.classes(),
        &[
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

    assert_eq!(form.element_id(), Some(&"login-form".to_string()));
    assert_eq!(
        form.classes(),
        &[
            "form-container".to_string(),
            "dark-theme".to_string(),
            "large".to_string(),
            "animated".to_string()
        ]
    );
    assert_eq!(form\.get_submit_text(), Some(&"Login".to_string()));
    assert!(form\.get_show_errors());
    assert_eq!(form\.get_error_style(), ErrorDisplayStyle::Both);
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

    assert_eq!(field.element_id(), Some(&"email-input".to_string()));
    assert_eq!(
        field.classes(),
        &[
            "required".to_string(),
            "validated".to_string(),
            "email-field".to_string()
        ]
    );
    assert_eq!(field\.get_placeholder(), Some(&"user@example.com".to_string()));
    assert_eq!(field\.get_input_type(), InputType::Email);
    assert!(field\.get_show_label());
    assert!(field\.get_show_errors());
}

// =========================================================================
// Form edge case tests
// =========================================================================

#[test]
fn test_form_empty_string_element_id() {
    let form_state = FormState::new().build();
    let form = Form::new(form_state).element_id("");
    assert_eq!(form.element_id(), Some(&"".to_string()));
}

#[test]
fn test_form_empty_string_class() {
    let form_state = FormState::new().build();
    let form = Form::new(form_state).class("");
    // Empty string class is still added
    assert_eq!(form.classes(), &["".to_string()]);
}

#[test]
fn test_form_classes_empty_vec() {
    let form_state = FormState::new().build();
    let form = Form::new(form_state).classes(Vec::<&str>::new());
    assert!(form.classes().is_empty());
}

#[test]
fn test_form_classes_empty_array() {
    let form_state = FormState::new().build();
    let form = Form::new(form_state).classes([] as [&str; 0]);
    assert!(form.classes().is_empty());
}

// =========================================================================
// FormFieldWidget edge case tests
// =========================================================================

#[test]
fn test_form_field_empty_name() {
    let field = FormFieldWidget::new("");
    assert_eq!(field\.name(), "");
}

#[test]
fn test_form_field_empty_string_element_id() {
    let field = FormFieldWidget::new("test").element_id("");
    assert_eq!(field.element_id(), Some(&"".to_string()));
}

#[test]
fn test_form_field_empty_string_class() {
    let field = FormFieldWidget::new("test").class("");
    assert_eq!(field.classes(), &["".to_string()]);
}

#[test]
fn test_form_field_classes_empty_iterator() {
    let field = FormFieldWidget::new("test").classes(Vec::<&str>::new());
    assert!(field.classes().is_empty());
}

#[test]
fn test_form_field_name_with_special_chars() {
    let field = FormFieldWidget::new("user-email-field");
    assert_eq!(field\.name(), "user-email-field");
}

#[test]
fn test_form_field_name_with_unicode() {
    let field = FormFieldWidget::new("用户邮箱");
    assert_eq!(field\.name(), "用户邮箱");
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

    assert_eq!(form.classes().len(), 9);
    assert_eq!(form\.get_submit_text(), Some(&"Submit".to_string()));
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

    assert_eq!(field.classes().len(), 5);
    assert_eq!(field\.get_placeholder(), Some(&"placeholder".to_string()));
    assert_eq!(field\.get_input_type(), InputType::Email);
    assert!(!field\.get_show_label());
    assert!(!field\.get_show_errors());
}
