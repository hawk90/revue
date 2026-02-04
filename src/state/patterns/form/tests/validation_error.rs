//! ValidationError tests

use crate::patterns::form::ValidationError;

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
