//! HttpBackend and MockHttpBackend tests

use revue::widget::developer::httpclient::backend::{HttpBackend, MockHttpBackend};
use revue::widget::developer::httpclient::request::HttpRequest;
use revue::widget::developer::httpclient::response::HttpResponse;
use std::time::Duration;

// =========================================================================
// MockHttpBackend tests
// =========================================================================

#[test]
fn test_mock_http_backend_new() {
    let backend = MockHttpBackend::new();
    assert!(backend.responses_for_testing().is_ok());
    assert!(backend.responses_for_testing().unwrap().is_empty());
}

#[test]
fn test_mock_http_backend_default() {
    let backend = MockHttpBackend::default();
    assert!(backend.responses_for_testing().unwrap().is_empty());
}

#[test]
fn test_mock_http_backend_send_no_mock() {
    let backend = MockHttpBackend::new();
    let request = HttpRequest::new("https://example.com");
    let result = backend.send(&request);
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status, 200);
    assert!(response.body.contains("mock"));
}

#[test]
fn test_mock_http_backend_send_with_wildcard() {
    let backend = MockHttpBackend::new();
    backend.mock_response(
        "*",
        HttpResponse {
            status: 200,
            status_text: "OK".to_string(),
            headers: std::collections::HashMap::new(),
            body: "wildcard response".to_string(),
            time: Duration::from_millis(10),
            size: 17,
        },
    );

    let request = HttpRequest::new("https://any-url.com");
    let result = backend.send(&request);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().body, "wildcard response");
}

#[test]
fn test_mock_http_backend_send_url_pattern() {
    let backend = MockHttpBackend::new();
    backend.mock_response(
        "api",
        HttpResponse {
            status: 201,
            status_text: "Created".to_string(),
            headers: std::collections::HashMap::new(),
            body: "created".to_string(),
            time: Duration::from_millis(5),
            size: 7,
        },
    );

    let request = HttpRequest::new("https://example.com/api/users");
    let result = backend.send(&request);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().status, 201);
}

#[test]
fn test_mock_http_backend_mock_response() {
    let backend = MockHttpBackend::new();
    let response = HttpResponse {
        status: 200,
        status_text: "OK".to_string(),
        headers: std::collections::HashMap::new(),
        body: "test body".to_string(),
        time: Duration::from_millis(50),
        size: 9,
    };
    backend.mock_response("test", response.clone());

    let request = HttpRequest::new("https://example.com/test");
    let result = backend.send(&request);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().body, "test body");
}

#[test]
fn test_mock_http_backend_mock_json() {
    let backend = MockHttpBackend::new();
    backend.mock_json("api/users", 200, r#"{"name":"test"}"#);

    let request = HttpRequest::new("https://example.com/api/users");
    let result = backend.send(&request);
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status, 200);
    assert_eq!(response.status_text, "OK");
    assert!(response.body.contains("test"));
    assert_eq!(
        response.headers.get("Content-Type"),
        Some(&"application/json".to_string())
    );
}

#[test]
fn test_mock_http_backend_mock_error() {
    let backend = MockHttpBackend::new();
    backend.mock_error("api/error", 404, "Not found");

    let request = HttpRequest::new("https://example.com/api/error");
    let result = backend.send(&request);
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status, 404);
    assert!(response.body.contains("Not found"));
}

#[test]
fn test_mock_http_backend_multiple_mocks() {
    let backend = MockHttpBackend::new();
    backend.mock_response(
        "api1",
        HttpResponse {
            status: 200,
            status_text: "OK".to_string(),
            headers: std::collections::HashMap::new(),
            body: "response1".to_string(),
            time: Duration::from_millis(10),
            size: 9,
        },
    );
    backend.mock_response(
        "api2",
        HttpResponse {
            status: 201,
            status_text: "Created".to_string(),
            headers: std::collections::HashMap::new(),
            body: "response2".to_string(),
            time: Duration::from_millis(10),
            size: 9,
        },
    );

    let request1 = HttpRequest::new("https://example.com/api1");
    let result1 = backend.send(&request1);
    assert_eq!(result1.unwrap().body, "response1");

    let request2 = HttpRequest::new("https://example.com/api2");
    let result2 = backend.send(&request2);
    assert_eq!(result2.unwrap().body, "response2");
}

#[test]
fn test_mock_http_backend_latest_mock_takes_precedence() {
    let backend = MockHttpBackend::new();
    backend.mock_response(
        "test",
        HttpResponse {
            status: 200,
            status_text: "OK".to_string(),
            headers: std::collections::HashMap::new(),
            body: "first".to_string(),
            time: Duration::from_millis(10),
            size: 5,
        },
    );
    backend.mock_response(
        "test",
        HttpResponse {
            status: 201,
            status_text: "Created".to_string(),
            headers: std::collections::HashMap::new(),
            body: "second".to_string(),
            time: Duration::from_millis(10),
            size: 6,
        },
    );

    let request = HttpRequest::new("https://example.com/test");
    let result = backend.send(&request);
    assert_eq!(result.unwrap().body, "second");
}

// =========================================================================
// HttpBackend trait tests
// =========================================================================

#[test]
fn test_http_backend_trait_send() {
    let backend = MockHttpBackend::new();
    let request = HttpRequest::new("https://example.com");
    // Test that the trait method is callable
    let result = backend.send(&request);
    assert!(result.is_ok());
}

#[test]
fn test_http_backend_send_and_sync() {
    // Test that MockHttpBackend implements Send + Sync
    use std::sync::{Arc, Mutex};
    let backend = Arc::new(Mutex::new(MockHttpBackend::new()));
    let backend_clone = Arc::clone(&backend);

    std::thread::spawn(move || {
        let b = backend_clone.lock().unwrap();
        b.mock_response(
            "test",
            HttpResponse {
                status: 200,
                status_text: "OK".to_string(),
                headers: std::collections::HashMap::new(),
                body: "test".to_string(),
                time: Duration::from_millis(10),
                size: 4,
            },
        );
    })
    .join()
    .unwrap();
}
