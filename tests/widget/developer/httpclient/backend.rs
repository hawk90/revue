//! MockHttpBackend tests
//!
//! The default mock response, a custom response matched by URL, mock_json
//! status/content type, mock_error, the "*" wildcard and "most recent
//! match wins" are covered by the in-source tests in
//! src/widget/developer/httpclient/tests.rs.

use revue::widget::{HttpBackend, HttpRequest, HttpResponse, MockHttpBackend};
use std::collections::HashMap;
use std::time::Duration;

fn request(url: &str) -> HttpRequest {
    HttpRequest::new(url).expect("valid URL")
}

fn response(status: u16, body: &str) -> HttpResponse {
    HttpResponse {
        status,
        status_text: String::new(),
        headers: HashMap::new(),
        body: body.to_string(),
        time: Duration::from_millis(10),
        size: body.len(),
    }
}

#[test]
fn test_mock_http_backend_starts_empty() {
    assert!(MockHttpBackend::new()
        .responses_for_testing()
        .unwrap()
        .is_empty());
    assert!(MockHttpBackend::default()
        .responses_for_testing()
        .unwrap()
        .is_empty());
}

#[test]
fn test_mock_http_backend_records_mocks() {
    let backend = MockHttpBackend::new();
    backend.mock_response("a", response(200, "a"));
    backend.mock_json("b", 201, "{}");
    assert_eq!(backend.responses_for_testing().unwrap().len(), 2);
}

#[test]
fn test_mock_http_backend_mock_json_headers() {
    let backend = MockHttpBackend::new();
    backend.mock_json("api/users", 200, r#"{"name":"test"}"#);

    let response = backend
        .send(&request("https://example.com/api/users"))
        .unwrap();
    assert_eq!(response.status, 200);
    assert_eq!(response.status_text, "OK");
    assert_eq!(response.body, r#"{"name":"test"}"#);
    assert_eq!(
        response.headers.get("Content-Type").map(String::as_str),
        Some("application/json")
    );
}

#[test]
fn test_mock_http_backend_mock_error_body_and_status_text() {
    let backend = MockHttpBackend::new();
    backend.mock_error("missing", 404, "Not found");
    backend.mock_error("teapot", 418, "short and stout");

    let response = backend
        .send(&request("https://example.com/missing"))
        .unwrap();
    assert_eq!(response.status, 404);
    assert_eq!(response.status_text, "Not Found");
    assert_eq!(response.body, r#"{"error": "Not found"}"#);
    assert_eq!(response.size, response.body.len());

    let response = backend
        .send(&request("https://example.com/teapot"))
        .unwrap();
    assert_eq!(response.status, 418);
    assert_eq!(response.status_text, "Unknown");
}

#[test]
fn test_mock_http_backend_unmatched_url_gets_default() {
    let backend = MockHttpBackend::new();
    backend.mock_response("api/users", response(201, "users"));

    let response = backend.send(&request("https://example.com/other")).unwrap();
    assert_eq!(response.status, 200);
    assert_ne!(response.body, "users");
}

#[test]
fn test_mock_http_backend_multiple_mocks() {
    let backend = MockHttpBackend::new();
    backend.mock_response("api1", response(200, "response1"));
    backend.mock_response("api2", response(201, "response2"));

    let first = backend.send(&request("https://example.com/api1")).unwrap();
    assert_eq!((first.status, first.body.as_str()), (200, "response1"));

    let second = backend.send(&request("https://example.com/api2")).unwrap();
    assert_eq!((second.status, second.body.as_str()), (201, "response2"));
}

#[test]
fn test_mock_http_backend_mocks_from_another_thread() {
    use std::sync::Arc;

    let backend = Arc::new(MockHttpBackend::new());
    let writer = Arc::clone(&backend);
    std::thread::spawn(move || writer.mock_response("test", response(202, "threaded")))
        .join()
        .unwrap();

    let response = backend.send(&request("https://example.com/test")).unwrap();
    assert_eq!(response.status, 202);
    assert_eq!(response.body, "threaded");
}
