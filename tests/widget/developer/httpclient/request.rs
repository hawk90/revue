//! HttpRequest tests
//!
//! URL validation (accepted schemes, rejected schemes, length limit),
//! percent-encoding in full_url() and full_url() without params are
//! covered by the in-source tests in src/widget/developer/httpclient/
//! request.rs.

use revue::widget::{HttpMethod, HttpRequest};

fn request(url: &str) -> HttpRequest {
    HttpRequest::new(url).expect("valid URL")
}

#[test]
fn test_http_request_default() {
    let request = HttpRequest::default();
    assert_eq!(request.method, HttpMethod::GET);
    assert_eq!(request.url(), "");
    assert!(request.headers.is_empty());
    assert_eq!(request.body, "");
    assert!(request.params.is_empty());
}

#[test]
fn test_http_request_new() {
    let request = request("https://example.com");
    assert_eq!(request.url(), "https://example.com");
    assert_eq!(request.method, HttpMethod::GET);
    assert!(request.headers.is_empty());
    assert!(request.params.is_empty());

    let owned = HttpRequest::new(String::from("https://test.com")).unwrap();
    assert_eq!(owned.url(), "https://test.com");
}

#[test]
fn test_http_request_new_rejects_empty_url() {
    assert!(HttpRequest::new("").is_none());
}

#[test]
fn test_http_request_method() {
    for method in [
        HttpMethod::POST,
        HttpMethod::PUT,
        HttpMethod::DELETE,
        HttpMethod::PATCH,
    ] {
        assert_eq!(request("https://example.com").method(method).method, method);
    }
}

#[test]
fn test_http_request_headers() {
    let request = request("https://example.com")
        .header("Accept", "application/json")
        .header("Authorization", "Bearer token");
    assert_eq!(request.headers.len(), 2);
    assert_eq!(
        request.headers.get("Accept").map(String::as_str),
        Some("application/json")
    );
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("Bearer token")
    );
}

#[test]
fn test_http_request_header_overwrite() {
    let request = request("https://example.com")
        .header("X-Custom", "value1")
        .header("X-Custom", "value2");
    assert_eq!(request.headers.len(), 1);
    assert_eq!(
        request.headers.get("X-Custom").map(String::as_str),
        Some("value2")
    );
}

#[test]
fn test_http_request_body() {
    let request = request("https://example.com").body("{\"test\":true}");
    assert_eq!(request.body, "{\"test\":true}");
}

#[test]
fn test_http_request_params() {
    let request = request("https://example.com")
        .param("foo", "bar")
        .param("baz", "qux");
    assert_eq!(request.params.len(), 2);
    assert_eq!(request.params.get("foo").map(String::as_str), Some("bar"));
    assert_eq!(request.params.get("baz").map(String::as_str), Some("qux"));
}

#[test]
fn test_http_request_param_overwrite() {
    let request = request("https://example.com")
        .param("key", "value1")
        .param("key", "value2");
    assert_eq!(request.params.len(), 1);
    assert_eq!(request.full_url(), "https://example.com?key=value2");
}

#[test]
fn test_http_request_full_url_with_one_param() {
    let request = request("https://example.com/api").param("key", "value");
    assert_eq!(request.full_url(), "https://example.com/api?key=value");
}

#[test]
fn test_http_request_full_url_with_multiple_params() {
    let full_url = request("https://example.com/api")
        .param("foo", "bar")
        .param("baz", "qux")
        .full_url();
    // Parameter order follows the map, so accept either.
    assert!(
        full_url == "https://example.com/api?foo=bar&baz=qux"
            || full_url == "https://example.com/api?baz=qux&foo=bar",
        "{full_url}"
    );
}

#[test]
fn test_http_request_builder_chain() {
    let request = request("https://example.com/api")
        .method(HttpMethod::POST)
        .header("Content-Type", "application/json")
        .body("{\"test\":true}")
        .param("debug", "true");
    assert_eq!(request.url(), "https://example.com/api");
    assert_eq!(request.method, HttpMethod::POST);
    assert_eq!(
        request.headers.get("Content-Type").map(String::as_str),
        Some("application/json")
    );
    assert_eq!(request.body, "{\"test\":true}");
    assert_eq!(request.full_url(), "https://example.com/api?debug=true");
}
