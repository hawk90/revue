//! HttpClient widget state tests
//!
//! The in-source tests in src/widget/developer/httpclient/tests.rs cover
//! a new client being Idle, builder method/header/body, and send() on a
//! helper-built client.

use revue::style::Color;
use revue::widget::{
    HttpClient, HttpMethod, HttpRequest, HttpResponse, RequestState, ResponseView,
};
use std::collections::HashMap;
use std::time::Duration;

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

// =========================================================================
// Construction and builders
// =========================================================================

#[test]
fn test_http_client_new() {
    let client = HttpClient::new();
    assert_eq!(client.state(), RequestState::Idle);
    assert!(client.response().is_none());
    assert!(client.error().is_none());
    assert_eq!(client.request().url(), "");
    assert_eq!(client.request().method, HttpMethod::GET);
    assert!(client.request().headers.is_empty());
    assert!(client.request().params.is_empty());
    assert_eq!(client.request().body, "");
    assert_eq!(client.view_for_testing(), ResponseView::Body);
    assert!(!client.show_headers_for_testing());
    assert_eq!(client.body_scroll_for_testing(), 0);
    assert!(client.history_for_testing().is_empty());
    assert_eq!(client.history_index_for_testing(), 0);
}

#[test]
fn test_http_client_default_matches_new() {
    let client = HttpClient::default();
    assert_eq!(client.state(), RequestState::Idle);
    assert_eq!(client.request().url(), "");
    assert!(client.history_for_testing().is_empty());
}

#[test]
fn test_http_client_url() {
    let client = HttpClient::new().url(String::from("https://example.com"));
    assert_eq!(client.request().url(), "https://example.com");
    assert_eq!(client.url_cursor_for_testing(), 19);
    assert!(client.error().is_none());
}

#[test]
fn test_http_client_url_last_one_wins() {
    let client = HttpClient::new()
        .url("https://api.example.com")
        .url("https://other.com");
    assert_eq!(client.request().url(), "https://other.com");
}

#[test]
fn test_http_client_url_keeps_special_chars_and_long_paths() {
    let special = "https://example.com/path?query=test%20space";
    assert_eq!(HttpClient::new().url(special).request().url(), special);

    let long = format!("https://example.com{}", "/segment".repeat(250));
    assert_eq!(HttpClient::new().url(long.clone()).request().url(), long);
}

#[test]
fn test_http_client_invalid_url_sets_error() {
    for url in ["", "ftp://example.com", "javascript:alert(1)"] {
        let client = HttpClient::new().url(url);
        assert_eq!(client.request().url(), "", "{url}");
        assert_eq!(client.state(), RequestState::Error, "{url}");
        assert!(client.error().unwrap().contains("Invalid URL"), "{url}");
    }
}

#[test]
fn test_http_client_headers() {
    let client = HttpClient::new()
        .header("Authorization", "Bearer token")
        .header("Content-Type", "application/json")
        .header("X-Custom", "value1")
        .header("X-Custom", "value2");
    let headers = &client.request().headers;
    assert_eq!(headers.len(), 3);
    assert_eq!(
        headers.get("Authorization").map(String::as_str),
        Some("Bearer token")
    );
    assert_eq!(headers.get("X-Custom").map(String::as_str), Some("value2"));
}

#[test]
fn test_http_client_body() {
    let unicode = r#"{"message":"안녕하세요"}"#;
    assert_eq!(HttpClient::new().body(unicode).request().body, unicode);
    assert_eq!(HttpClient::new().body("").request().body, "");
}

#[test]
fn test_http_client_colors() {
    let mut colors = HttpClient::new().colors_for_testing().clone();
    colors.tab_active = Color::RED;
    let client = HttpClient::new().colors(colors);
    assert_eq!(client.colors_for_testing().tab_active, Color::RED);
}

#[test]
fn test_http_client_request_mut() {
    let mut client = HttpClient::new();
    *client.request_mut() = HttpRequest::new("https://test.com")
        .unwrap()
        .method(HttpMethod::PUT);
    client.request_mut().body = "payload".to_string();
    assert_eq!(client.request().url(), "https://test.com");
    assert_eq!(client.request().method, HttpMethod::PUT);
    assert_eq!(client.request().body, "payload");
}

// =========================================================================
// State changes
// =========================================================================

#[test]
fn test_http_client_set_url() {
    let mut client = HttpClient::new();
    client.set_url("https://example.com");
    assert_eq!(client.request().url(), "https://example.com");
    assert_eq!(client.url_cursor_for_testing(), 19);
}

#[test]
fn test_http_client_set_invalid_url_keeps_old_url() {
    let mut client = HttpClient::new().url("https://example.com");
    client.set_url("");
    assert_eq!(client.request().url(), "https://example.com");
    assert_eq!(client.url_cursor_for_testing(), 19);
    assert_eq!(client.state(), RequestState::Error);
    assert!(client.error().unwrap().contains("Invalid URL"));
}

#[test]
fn test_http_client_set_view() {
    let mut client = HttpClient::new();
    client.set_view(ResponseView::Headers);
    assert_eq!(client.view_for_testing(), ResponseView::Headers);
    client.set_view(ResponseView::Raw);
    assert_eq!(client.view_for_testing(), ResponseView::Raw);
}

#[test]
fn test_http_client_toggle_headers() {
    let mut client = HttpClient::new();
    client.toggle_headers();
    assert!(client.show_headers_for_testing());
    client.toggle_headers();
    assert!(!client.show_headers_for_testing());
}

#[test]
fn test_http_client_cycle_method_full_cycle() {
    let mut client = HttpClient::new();
    for expected in [
        HttpMethod::GET,
        HttpMethod::POST,
        HttpMethod::PUT,
        HttpMethod::DELETE,
        HttpMethod::PATCH,
        HttpMethod::HEAD,
        HttpMethod::OPTIONS,
    ] {
        assert_eq!(client.request().method, expected);
        client.cycle_method();
    }
    assert_eq!(client.request().method, HttpMethod::GET);
}

#[test]
fn test_http_client_send_mock_response() {
    let mut client = HttpClient::new().url("https://example.com");
    client.send();
    assert_eq!(client.state(), RequestState::Success);
    let response = client.response().unwrap();
    assert_eq!(response.status, 200);
    assert_eq!(response.status_text, "OK");
    assert!(response.body.contains("\"status\": \"success\""));
    assert_eq!(response.size, response.body.len());
    assert_eq!(
        response.headers.get("Content-Length"),
        Some(&response.body.len().to_string())
    );
}

#[test]
fn test_http_client_send_clears_error() {
    let mut client = HttpClient::new().url("https://example.com");
    client.set_error("previous failure");
    client.send();
    assert!(client.error().is_none());
    assert_eq!(client.state(), RequestState::Success);
}

#[test]
fn test_http_client_set_response() {
    let mut client = HttpClient::new();
    client.set_response(response(201, "created"));
    assert_eq!(client.state(), RequestState::Success);
    assert_eq!(client.response().unwrap().status, 201);
    assert_eq!(client.response().unwrap().body, "created");

    client.set_response(response(404, "missing"));
    assert_eq!(client.state(), RequestState::Error);
    assert_eq!(client.response().unwrap().status, 404);
}

#[test]
fn test_http_client_set_error() {
    let mut client = HttpClient::new();
    client.set_error("Connection failed");
    assert_eq!(client.state(), RequestState::Error);
    assert_eq!(client.error(), Some("Connection failed"));
}

#[test]
fn test_http_client_clear() {
    let mut client = HttpClient::new().url("https://example.com");
    client.send();
    client.set_error("test");
    client.scroll_down(10);
    client.clear();
    assert_eq!(client.state(), RequestState::Idle);
    assert!(client.response().is_none());
    assert!(client.error().is_none());
    assert_eq!(client.body_scroll_for_testing(), 0);
    // The request itself is kept.
    assert_eq!(client.request().url(), "https://example.com");
}

// =========================================================================
// Scroll
// =========================================================================

#[test]
fn test_http_client_scroll() {
    let mut client = HttpClient::new();
    client.scroll_down(10);
    client.scroll_down(5);
    assert_eq!(client.body_scroll_for_testing(), 15);
    client.scroll_up(5);
    assert_eq!(client.body_scroll_for_testing(), 10);
    client.scroll_up(20);
    assert_eq!(client.body_scroll_for_testing(), 0);
}

// =========================================================================
// History
// =========================================================================

/// A client that has sent `urls` in order.
fn sent(urls: &[&str]) -> HttpClient {
    let mut client = HttpClient::new();
    for url in urls {
        client.set_url(*url);
        client.send();
    }
    client
}

#[test]
fn test_http_client_send_saves_to_history() {
    let client = sent(&["https://example.com/1", "https://example.com/2"]);
    let history: Vec<&str> = client
        .history_for_testing()
        .iter()
        .map(HttpRequest::url)
        .collect();
    assert_eq!(history, ["https://example.com/1", "https://example.com/2"]);
    assert_eq!(client.history_index_for_testing(), 2);
}

#[test]
fn test_http_client_history_back_and_forward() {
    let mut client = sent(&["https://example.com/1", "https://example.com/2"]);

    // Like shell history, the first step back recalls the latest request.
    client.history_back();
    assert_eq!(client.history_index_for_testing(), 1);
    assert_eq!(client.request().url(), "https://example.com/2");

    client.history_back();
    assert_eq!(client.history_index_for_testing(), 0);
    assert_eq!(client.request().url(), "https://example.com/1");

    // Already at the oldest entry.
    client.history_back();
    assert_eq!(client.history_index_for_testing(), 0);
    assert_eq!(client.request().url(), "https://example.com/1");

    client.history_forward();
    assert_eq!(client.history_index_for_testing(), 1);
    assert_eq!(client.request().url(), "https://example.com/2");

    // Past the newest entry the request is left as it is.
    client.history_forward();
    assert_eq!(client.history_index_for_testing(), 2);
    assert_eq!(client.request().url(), "https://example.com/2");
    client.history_forward();
    assert_eq!(client.history_index_for_testing(), 2);
}

#[test]
fn test_http_client_history_restores_whole_request() {
    let mut client = HttpClient::new()
        .url("https://example.com/post")
        .method(HttpMethod::POST)
        .body("payload");
    client.send();
    client.set_url("https://example.com/other");
    client.cycle_method();
    client.request_mut().body.clear();

    client.history_back();
    assert_eq!(client.request().url(), "https://example.com/post");
    assert_eq!(client.request().method, HttpMethod::POST);
    assert_eq!(client.request().body, "payload");
}

#[test]
fn test_http_client_history_on_empty_history() {
    let mut client = HttpClient::new();
    client.history_back();
    client.history_forward();
    assert_eq!(client.history_index_for_testing(), 0);
    assert_eq!(client.request().url(), "");
}
