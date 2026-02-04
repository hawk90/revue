//! HttpRequest, RequestBuilder, MockHttpBackend Tests

use revue::widget::{http_get, http_post, HttpClient, HttpMethod, HttpRequest};
use revue::widget::{HttpResponse, MockHttpBackend, RequestBuilder};
use std::collections::HashMap;
use std::time::Duration;

#[test]
fn test_http_request_new() {
    let req = HttpRequest::new("https://api.example.com");
    assert_eq!(req.url, "https://api.example.com");
}

#[test]
fn test_http_request_builder_chain() {
    let req = HttpRequest::new("https://api.example.com")
        .method(HttpMethod::POST)
        .header("Authorization", "Bearer token");
    assert_eq!(req.method, HttpMethod::POST);
}

#[test]
fn test_http_request_params() {
    let req = HttpRequest::new("https://api.example.com")
        .param("search", "rust")
        .param("sort", "desc");
    assert_eq!(req.params.get("search"), Some(&"rust".to_string()));
}

#[test]
fn test_full_url_with_params() {
    let req = HttpRequest::new("https://api.example.com/users")
        .param("page", "1")
        .param("limit", "10");
    let url = req.full_url();
    assert!(url.contains("page=1"));
}

#[test]
fn test_request_builder_get() {
    let request = RequestBuilder::get("https://api.example.com").build();
    assert_eq!(request.method, HttpMethod::GET);
}

#[test]
fn test_request_builder_post() {
    let request = RequestBuilder::post("https://api.example.com").build();
    assert_eq!(request.method, HttpMethod::POST);
}

#[test]
fn test_request_builder_json() {
    let request = RequestBuilder::post("https://api.example.com")
        .json(r#"{"key":"value"}"#)
        .build();
    assert_eq!(request.headers.get("Content-Type"), Some(&"application/json".to_string()));
}

#[test]
fn test_mock_backend_default() {
    let backend = MockHttpBackend::new();
    let request = HttpRequest::new("https://any.url.com");
    let response = backend.send(&request).unwrap();
    assert_eq!(response.status, 200);
}

#[test]
fn test_mock_backend_custom_response() {
    let backend = MockHttpBackend::new();
    backend.mock_response("example.com", HttpResponse {
        status: 201,
        status_text: "Created".to_string(),
        headers: HashMap::new(),
        body: "custom body".to_string(),
        time: Duration::from_millis(10),
        size: 11,
    });
    let request = HttpRequest::new("https://example.com/test");
    let response = backend.send(&request).unwrap();
    assert_eq!(response.status, 201);
}
