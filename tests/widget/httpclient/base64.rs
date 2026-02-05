//! Base64 Encoding Tests

use super::common::test_base64_encode;

#[test]
fn test_base64_encode_simple() {
    assert_eq!(test_base64_encode(b"Hello"), "SGVsbG8=");
    assert_eq!(test_base64_encode(b"Hi"), "SGk=");
    assert_eq!(test_base64_encode(b"A"), "QQ==");
}

#[test]
fn test_base64_encode_empty() {
    assert_eq!(test_base64_encode(b""), "");
}

#[test]
fn test_base64_encode_even_length() {
    assert_eq!(test_base64_encode(b"HelloWorld"), "SGVsbG9Xb3JsZA==");
}

#[test]
fn test_base64_encode_odd_length() {
    assert_eq!(test_base64_encode(b"Hello!"), "SGVsbG8h");
}

#[test]
fn test_base64_encode_credentials() {
    let encoded = test_base64_encode(b"user:pass");
    assert_eq!(encoded, "dXNlcjpwYXNz");
}

#[test]
fn test_base64_encode_special_chars() {
    let encoded = test_base64_encode(b"test@email.com");
    assert_eq!(encoded, "dGVzdEBlbWFpbC5jb20=");
}

#[test]
fn test_base64_encode_long_string() {
    let input = "a".repeat(100);
    let encoded = test_base64_encode(input.as_bytes());
    assert!(encoded.len() > input.len());
}
