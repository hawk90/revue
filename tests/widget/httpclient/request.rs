//! HttpRequest tests

use revue::widget::developer::httpclient::request::HttpRequest;
use revue::widget::developer::httpclient::types::HttpMethod;

// =========================================================================
// HttpRequest struct tests
// =========================================================================

#[test]
fn test_http_request_default() {
    let request = HttpRequest::default();
    assert_eq!(request.method, HttpMethod::default());
    assert_eq!(request.url, "");
    assert!(request.headers.is_empty());
    assert_eq!(request.body, "");
    assert!(request.params.is_empty());
}

#[test]
fn test_http_request_new() {
    let request = HttpRequest::new("https://example.com");
    assert_eq!(request.url, "https://example.com");
    assert_eq!(request.method, HttpMethod::default());
}

#[test]
fn test_http_request_new_with_string() {
    let url = String::from("https://test.com");
    let request = HttpRequest::new(url);
    assert_eq!(request.url, "https://test.com");
}

#[test]
fn test_http_request_method_get() {
    let request = HttpRequest::new("https://example.com").method(HttpMethod::GET);
    assert_eq!(request.method, HttpMethod::GET);
}

#[test]
fn test_http_request_method_post() {
    let request = HttpRequest::new("https://example.com").method(HttpMethod::POST);
    assert_eq!(request.method, HttpMethod::POST);
}

#[test]
fn test_http_request_method_put() {
    let request = HttpRequest::new("https://example.com").method(HttpMethod::PUT);
    assert_eq!(request.method, HttpMethod::PUT);
}

#[test]
fn test_http_request_method_delete() {
    let request = HttpRequest::new("https://example.com").method(HttpMethod::DELETE);
    assert_eq!(request.method, HttpMethod::DELETE);
}

#[test]
fn test_http_request_header() {
    let request =
        HttpRequest::new("https://example.com").header("Content-Type", "application/json");
    assert_eq!(
        request.headers.get("Content-Type"),
        Some(&"application/json".to_string())
    );
}

#[test]
fn test_http_request_header_multiple() {
    let request = HttpRequest::new("https://example.com")
        .header("Accept", "application/json")
        .header("Authorization", "Bearer token");
    assert_eq!(request.headers.len(), 2);
}

#[test]
fn test_http_request_body() {
    let request = HttpRequest::new("https://example.com").body("{\"test\":true}");
    assert_eq!(request.body, "{\"test\":true}");
}

#[test]
fn test_http_request_param() {
    let request = HttpRequest::new("https://example.com").param("key", "value");
    assert_eq!(request.params.get("key"), Some(&"value".to_string()));
}

#[test]
fn test_http_request_param_multiple() {
    let request = HttpRequest::new("https://example.com")
        .param("foo", "bar")
        .param("baz", "qux");
    assert_eq!(request.params.len(), 2);
}

#[test]
fn test_http_request_full_url_no_params() {
    let request = HttpRequest::new("https://example.com/api");
    assert_eq!(request.full_url(), "https://example.com/api");
}

#[test]
fn test_http_request_full_url_with_params() {
    let request = HttpRequest::new("https://example.com/api").param("key", "value");
    let full_url = request.full_url();
    assert!(full_url.contains("key=value"));
    assert!(full_url.contains("?"));
}

#[test]
fn test_http_request_full_url_with_multiple_params() {
    let request = HttpRequest::new("https://example.com/api")
        .param("foo", "bar")
        .param("baz", "qux");
    let full_url = request.full_url();
    assert!(full_url.contains("foo=bar"));
    assert!(full_url.contains("baz=qux"));
    assert!(full_url.contains("&"));
}

#[test]
fn test_http_request_builder_chain() {
    let request = HttpRequest::new("https://example.com/api")
        .method(HttpMethod::POST)
        .header("Content-Type", "application/json")
        .body("{\"test\":true}")
        .param("debug", "true");

    assert_eq!(request.url, "https://example.com/api");
    assert_eq!(request.method, HttpMethod::POST);
    assert_eq!(request.body, "{\"test\":true}");
    assert_eq!(request.params.get("debug"), Some(&"true".to_string()));
}

#[test]
fn test_http_request_clone() {
    let request1 = HttpRequest::new("https://example.com")
        .method(HttpMethod::POST)
        .body("test");
    let request2 = request1.clone();
    assert_eq!(request1.url, request2.url);
    assert_eq!(request1.method, request2.method);
    assert_eq!(request1.body, request2.body);
}

#[test]
fn test_http_request_debug() {
    let request = HttpRequest::new("https://example.com");
    let debug_str = format!("{:?}", request);
    assert!(debug_str.contains("HttpRequest"));
}

#[test]
fn test_http_request_header_overwrite() {
    let request = HttpRequest::new("https://example.com")
        .header("X-Custom", "value1")
        .header("X-Custom", "value2");
    assert_eq!(request.headers.get("X-Custom"), Some(&"value2".to_string()));
}

#[test]
fn test_http_request_param_overwrite() {
    let request = HttpRequest::new("https://example.com")
        .param("key", "value1")
        .param("key", "value2");
    assert_eq!(request.params.get("key"), Some(&"value2".to_string()));
}
