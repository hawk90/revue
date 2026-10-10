//! Tests for the Form and FormFieldWidget widgets

use revue::patterns::form::FormState;
use revue::widget::{
    form_field, form_widget as form, ErrorDisplayStyle, Form, FormFieldWidget, InputType, View,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| s.to_string()).collect()
}

fn email_form() -> FormState {
    FormState::new()
        .field("email", |f| f.label("Email").required().email())
        .build()
}

/// Every data map a submit callback was called with.
type Calls = Arc<Mutex<Vec<HashMap<String, String>>>>;

/// A form whose submit callback records the data it was called with.
fn recording_form(state: FormState) -> (Form, Calls) {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let sink = calls.clone();
    let form = Form::new(state).on_submit(Arc::new(move |data| {
        sink.lock().unwrap().push(data);
    }));
    (form, calls)
}

// =========================================================================
// InputType / ErrorDisplayStyle
// =========================================================================

#[test]
fn test_input_type_default_and_variants() {
    assert_eq!(InputType::default(), InputType::Text);
    let variants = [
        InputType::Text,
        InputType::Password,
        InputType::Email,
        InputType::Number,
    ];
    for (i, a) in variants.iter().enumerate() {
        for (j, b) in variants.iter().enumerate() {
            assert_eq!(i == j, a == b, "{a:?} vs {b:?}");
        }
    }
}

#[test]
fn test_error_display_style_default_and_variants() {
    assert_eq!(ErrorDisplayStyle::default(), ErrorDisplayStyle::Inline);
    assert_ne!(ErrorDisplayStyle::Inline, ErrorDisplayStyle::Summary);
    assert_ne!(ErrorDisplayStyle::Inline, ErrorDisplayStyle::Both);
    assert_ne!(ErrorDisplayStyle::Summary, ErrorDisplayStyle::Both);
}

// =========================================================================
// Form
// =========================================================================

#[test]
fn test_form_new_defaults() {
    let form = Form::new(FormState::new().build());
    assert!(form.is_valid());
    assert_eq!(form.error_count(), 0);
    assert!(form.get_show_errors());
    assert_eq!(form.get_error_style(), ErrorDisplayStyle::Inline);
    assert!(form.form_state().values().is_empty());
    assert_eq!(form.id(), None);
    assert!(View::classes(&form).is_empty());
}

#[test]
fn test_form_default_matches_new() {
    let form = Form::default();
    assert!(form.is_valid());
    assert_eq!(form.error_count(), 0);
    assert!(form.get_show_errors());
    assert_eq!(form.get_error_style(), ErrorDisplayStyle::Inline);
}

#[test]
fn test_form_helper_fn() {
    let form = form(email_form());
    assert_eq!(form.form_state().field_names(), ["email".to_string()]);
}

#[test]
fn test_form_builder() {
    let form = Form::new(FormState::new().build())
        .show_errors(false)
        .error_style(ErrorDisplayStyle::Summary);

    assert!(!form.get_show_errors());
    assert_eq!(form.get_error_style(), ErrorDisplayStyle::Summary);

    let form = form.error_style(ErrorDisplayStyle::Both);
    assert_eq!(form.get_error_style(), ErrorDisplayStyle::Both);
}

#[test]
fn test_form_shares_its_form_state() {
    let state = email_form();
    let form = Form::new(state.clone());
    state.set_value("email", "user@example.com");
    assert_eq!(
        form.form_state().value("email").as_deref(),
        Some("user@example.com")
    );
}

#[test]
fn test_form_submit_without_callback_does_nothing() {
    let state = email_form();
    state.set_value("email", "user@example.com");
    let form = Form::new(state.clone());
    form.submit();
    assert_eq!(state.value("email").as_deref(), Some("user@example.com"));
}

#[test]
fn test_form_submit_passes_values_to_callback() {
    let state = email_form();
    state.set_value("email", "user@example.com");
    let (form, calls) = recording_form(state);

    assert!(form.is_valid());
    form.submit();

    let calls = calls.lock().unwrap();
    assert_eq!(calls.len(), 1);
    assert_eq!(
        calls[0].get("email").map(String::as_str),
        Some("user@example.com")
    );
}

#[test]
fn test_form_submit_empty_form_calls_callback() {
    let (form, calls) = recording_form(FormState::new().build());
    form.submit();
    let calls = calls.lock().unwrap();
    assert_eq!(calls.len(), 1);
    assert!(calls[0].is_empty());
}

#[test]
fn test_form_submit_not_called_when_invalid() {
    let (form, calls) = recording_form(email_form());

    // The required email is missing.
    assert!(!form.is_valid());
    assert!(form.error_count() > 0);

    form.submit();
    assert!(calls.lock().unwrap().is_empty());
}

// =========================================================================
// FormFieldWidget
// =========================================================================

#[test]
fn test_form_field_new_defaults() {
    let field = FormFieldWidget::new("username");
    assert_eq!(field.name(), "username");
    assert_eq!(field.get_placeholder(), None);
    assert_eq!(field.get_input_type(), InputType::Text);
    assert!(field.get_show_label());
    assert!(field.get_show_errors());
}

#[test]
fn test_form_field_default() {
    let field = FormFieldWidget::default();
    assert_eq!(field.name(), "");
    assert_eq!(field.get_placeholder(), None);
    assert_eq!(field.get_input_type(), InputType::Text);
    assert!(field.get_show_label());
    assert!(field.get_show_errors());
}

#[test]
fn test_form_field_helper_fn() {
    let field = form_field("password");
    assert_eq!(field.name(), "password");
    assert_eq!(field.get_input_type(), InputType::Text);
}

#[test]
fn test_form_field_builder() {
    let field = FormFieldWidget::new("email")
        .placeholder("Enter email")
        .input_type(InputType::Email)
        .show_label(false)
        .show_errors(false);

    assert_eq!(field.get_placeholder(), Some(&"Enter email".to_string()));
    assert_eq!(field.get_input_type(), InputType::Email);
    assert!(!field.get_show_label());
    assert!(!field.get_show_errors());

    let field = field.show_label(true).show_errors(true);
    assert!(field.get_show_label());
    assert!(field.get_show_errors());
}

#[test]
fn test_form_field_input_types() {
    for input_type in [InputType::Password, InputType::Number, InputType::Email] {
        let field = FormFieldWidget::new("f").input_type(input_type);
        assert_eq!(field.get_input_type(), input_type);
    }
}

#[test]
fn test_form_field_name_kept_verbatim() {
    for name in ["", "user-email-field", "用户邮箱"] {
        assert_eq!(FormFieldWidget::new(name).name(), name);
    }
}

// =========================================================================
// element_id / class builders, read back through View
// =========================================================================

#[test]
fn test_form_element_id() {
    let form = Form::new(FormState::new().build()).element_id("my-form");
    assert_eq!(form.id(), Some("my-form"));

    // The last one wins.
    let form = form.element_id("second-id");
    assert_eq!(form.id(), Some("second-id"));

    let form = Form::new(FormState::new().build()).element_id("");
    assert_eq!(form.id(), Some(""));
}

#[test]
fn test_form_classes_keep_order_and_skip_duplicates() {
    let form = Form::new(FormState::new().build())
        .class("single")
        .classes(vec!["vec1", "vec2", "single"])
        .class("another")
        .class("vec1");
    assert_eq!(
        View::classes(&form),
        strings(&["single", "vec1", "vec2", "another"])
    );

    let form = Form::new(FormState::new().build()).classes(["class1", "class2"]);
    assert_eq!(View::classes(&form), strings(&["class1", "class2"]));

    let form = Form::new(FormState::new().build()).class("");
    assert_eq!(View::classes(&form), strings(&[""]));

    let form = Form::new(FormState::new().build()).classes(Vec::<&str>::new());
    assert!(View::classes(&form).is_empty());
}

#[test]
fn test_form_full_builder_chain_with_props() {
    let form = form(FormState::new().build())
        .element_id("login-form")
        .class("form-container")
        .classes(vec!["large", "animated"])
        .error_style(ErrorDisplayStyle::Both);

    assert_eq!(form.id(), Some("login-form"));
    assert_eq!(
        View::classes(&form),
        strings(&["form-container", "large", "animated"])
    );
    assert_eq!(form.get_error_style(), ErrorDisplayStyle::Both);
}

#[test]
fn test_form_field_element_id() {
    let field = FormFieldWidget::new("email").element_id("first");
    assert_eq!(field.id(), Some("first"));
    let field = field.element_id("second");
    assert_eq!(field.id(), Some("second"));
}

#[test]
fn test_form_field_classes_keep_order_and_skip_duplicates() {
    let field = FormFieldWidget::new("mixed")
        .class("first")
        .classes(vec!["second", "first", "third", "second"])
        .class("fourth")
        .class("third");
    assert_eq!(
        View::classes(&field),
        strings(&["first", "second", "third", "fourth"])
    );

    let field = FormFieldWidget::new("test").classes(Vec::<&str>::new());
    assert!(View::classes(&field).is_empty());
}

#[test]
fn test_form_field_full_builder_chain_with_props() {
    let field = form_field("email")
        .element_id("email-input")
        .class("required")
        .classes(vec!["validated", "email-field"])
        .placeholder("user@example.com")
        .input_type(InputType::Email)
        .show_label(false);

    assert_eq!(field.name(), "email");
    assert_eq!(field.id(), Some("email-input"));
    assert_eq!(
        View::classes(&field),
        strings(&["required", "validated", "email-field"])
    );
    assert_eq!(
        field.get_placeholder(),
        Some(&"user@example.com".to_string())
    );
    assert_eq!(field.get_input_type(), InputType::Email);
    assert!(!field.get_show_label());
    assert!(field.get_show_errors());
}
