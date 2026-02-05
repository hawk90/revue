//! Edge Cases Tests

use revue::widget::{http_get, HttpClient};

#[test]
fn test_send_with_empty_url_and_body() {
    let mut client = HttpClient::new();
    client.send();
}

#[test]
fn test_url_with_special_chars() {
    let special_url = "https://example.com/path?query=test%20space";
    let client = HttpClient::new().url(special_url);
    assert_eq!(client.request().url, special_url);
}

#[test]
fn test_body_with_unicode() {
    let unicode_body = r#"{"message":"안녕하세요"}"#;
    let client = HttpClient::new().body(unicode_body);
    assert_eq!(client.request().body, unicode_body);
}

#[test]
fn test_very_long_url() {
    let long_url = "https://example.com/".repeat(100);
    let client = HttpClient::new().url(long_url.clone());
    assert_eq!(client.request().url.len(), long_url.len());
}

#[test]
fn test_empty_headers_map() {
    let client = HttpClient::new();
    assert!(client.request().headers.is_empty());
}

#[test]
fn test_empty_params_map() {
    let client = HttpClient::new();
    assert!(client.request().params.is_empty());
}
