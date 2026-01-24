//! FormField and FormFieldBuilder tests

use crate::patterns::form::{FieldType, FormField, Validators};

// FormField construction tests
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

// FormField builder tests
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

// FormField validation tests
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

// FormField method tests
#[test]
fn test_form_field_update_value() {
    let field = FormField::text().build();
    field.update_value(|v| v.push_str("hello"));
    assert_eq!(field.value(), "hello");
    assert!(field.is_touched());
}

#[test]
fn test_form_field_touch() {
    let field = FormField::text().build();
    assert!(!field.is_touched());
    field.touch();
    assert!(field.is_touched());
}

#[test]
fn test_form_field_touched_signal() {
    let field = FormField::text().build();
    let signal = field.touched_signal();
    assert!(!signal.get());
    field.touch();
    assert!(signal.get());
}

// FormFieldBuilder tests
#[test]
fn test_form_field_builder_default() {
    let builder = FormField::text();
    let field = builder.build();
    assert_eq!(field.field_type, FieldType::Text);
}

#[test]
fn test_form_field_validator_method() {
    let field = FormField::text()
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
