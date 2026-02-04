//! FieldType tests

use crate::patterns::form::FieldType;

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
