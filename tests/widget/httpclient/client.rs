//! HttpClient widget tests

use revue::widget::developer::httpclient::client::HttpClient;
use revue::widget::developer::httpclient::types::{HttpColors, HttpMethod, RequestState, ResponseView};
use std::time::Duration;

// =========================================================================
// Constructor tests
// =========================================================================

#[test]
fn test_http_client_new() {
    let client = HttpClient::new();
    assert_eq!(client.request().url, "");
    assert_eq!(client.state(), RequestState::Idle);
    assert!(client.response().is_none());
}

#[test]
fn test_http_client_default() {
    let client = HttpClient::default();
    assert_eq!(client.state(), RequestState::Idle);
    assert!(client.history_for_testing().is_empty());
}

// =========================================================================
// Builder tests
// =========================================================================

#[test]
fn test_http_client_url() {
    let client = HttpClient::new().url("https://example.com");
    assert_eq!(client.request().url, "https://example.com");
    assert_eq!(client.url_cursor_for_testing(), 19); // "https://example.com" has 19 characters
}

#[test]
fn test_http_client_url_with_string() {
    let url = String::from("https://test.com");
    let client = HttpClient::new().url(url);
    assert_eq!(client.request().url, "https://test.com");
}

#[test]
fn test_http_client_method() {
    let client = HttpClient::new().method(HttpMethod::POST);
    assert_eq!(client.request().method, HttpMethod::POST);
}

#[test]
fn test_http_client_header() {
    let client = HttpClient::new().header("Accept", "application/json");
    assert_eq!(
        client.request().headers.get("Accept"),
        Some(&"application/json".to_string())
    );
}

#[test]
fn test_http_client_body() {
    let client = HttpClient::new().body("{\"test\":true}");
    assert_eq!(client.request().body, "{\"test\":true}");
}

#[test]
fn test_http_client_colors() {
    let colors = HttpColors::default();
    let client = HttpClient::new().colors(colors.clone());
    assert_eq!(client.colors_for_testing().tab_active, colors.tab_active);
}

// =========================================================================
// Getter tests
// =========================================================================

#[test]
fn test_http_client_request() {
    let client = HttpClient::new().url("https://example.com");
    assert_eq!(client.request().url, "https://example.com");
}

#[test]
fn test_http_client_request_mut() {
    let mut client = HttpClient::new();
    client.request_mut().url = "https://test.com".to_string();
    assert_eq!(client.request().url, "https://test.com");
}

#[test]
fn test_http_client_response_none() {
    let client = HttpClient::new();
    assert!(client.response().is_none());
}

#[test]
fn test_http_client_response_some() {
    let mut client = HttpClient::new();
    let response = revue::widget::developer::httpclient::response::HttpResponse {
        status: 200,
        status_text: "OK".to_string(),
        headers: std::collections::HashMap::new(),
        body: "test".to_string(),
        time: Duration::from_millis(100),
        size: 4,
    };
    client.set_response(response);
    assert!(client.response().is_some());
    assert_eq!(client.response().unwrap().status, 200);
}

#[test]
fn test_http_client_state() {
    let client = HttpClient::new();
    assert_eq!(client.state(), RequestState::Idle);
}

#[test]
fn test_http_client_error_none() {
    let client = HttpClient::new();
    assert!(client.error().is_none());
}

#[test]
fn test_http_client_error_some() {
    let mut client = HttpClient::new();
    client.set_error("Test error");
    assert_eq!(client.error(), Some("Test error"));
}

// =========================================================================
// State changing tests
// =========================================================================

#[test]
fn test_http_client_set_view() {
    let mut client = HttpClient::new();
    client.set_view(ResponseView::Headers);
    assert!(matches!(client.view_for_testing(), ResponseView::Headers));
}

#[test]
fn test_http_client_toggle_headers() {
    let mut client = HttpClient::new();
    let initial = client.show_headers_for_testing();
    client.toggle_headers();
    assert_ne!(client.show_headers_for_testing(), initial);
}

#[test]
fn test_http_client_set_url() {
    let mut client = HttpClient::new();
    client.set_url("https://example.com");
    assert_eq!(client.request().url, "https://example.com");
    assert_eq!(client.url_cursor_for_testing(), 19); // "https://example.com" has 19 characters
}

#[test]
fn test_http_client_cycle_method() {
    let mut client = HttpClient::new();
    assert_eq!(client.request().method, HttpMethod::GET);
    client.cycle_method();
    assert_eq!(client.request().method, HttpMethod::POST);
    client.cycle_method();
    assert_eq!(client.request().method, HttpMethod::PUT);
}

#[test]
fn test_http_client_cycle_method_full_cycle() {
    let mut client = HttpClient::new();
    let methods = [
        HttpMethod::GET,
        HttpMethod::POST,
        HttpMethod::PUT,
        HttpMethod::DELETE,
        HttpMethod::PATCH,
        HttpMethod::HEAD,
        HttpMethod::OPTIONS,
    ];
    for expected in &methods {
        assert_eq!(client.request().method, *expected);
        client.cycle_method();
    }
    assert_eq!(client.request().method, HttpMethod::GET);
}

#[test]
fn test_http_client_send() {
    let mut client = HttpClient::new().url("https://example.com");
    client.send();
    assert_eq!(client.state(), RequestState::Success);
    assert!(client.response().is_some());
    assert_eq!(client.response().unwrap().status, 200);
}

#[test]
fn test_http_client_send_saves_to_history() {
    let mut client = HttpClient::new().url("https://example.com");
    client.send();
    assert_eq!(client.history_for_testing().len(), 1);
    assert_eq!(client.history_index_for_testing(), 1);
}

#[test]
fn test_http_client_set_response_success() {
    let mut client = HttpClient::new();
    let response = revue::widget::developer::httpclient::response::HttpResponse {
        status: 200,
        status_text: "OK".to_string(),
        headers: std::collections::HashMap::new(),
        body: "success".to_string(),
        time: Duration::from_millis(50),
        size: 7,
    };
    client.set_response(response);
    assert_eq!(client.state(), RequestState::Success);
}

#[test]
fn test_http_client_set_response_error() {
    let mut client = HttpClient::new();
    let response = revue::widget::developer::httpclient::response::HttpResponse {
        status: 404,
        status_text: "Not Found".to_string(),
        headers: std::collections::HashMap::new(),
        body: "error".to_string(),
        time: Duration::from_millis(10),
        size: 5,
    };
    client.set_response(response);
    assert_eq!(client.state(), RequestState::Error);
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
    let mut client = HttpClient::new();
    client.set_error("test");
    client.set_body_scroll_for_testing(10);
    client.clear();
    assert_eq!(client.state(), RequestState::Idle);
    assert!(client.response().is_none());
    assert!(client.error().is_none());
    assert_eq!(client.body_scroll_for_testing(), 0);
}

// =========================================================================
// Scroll tests
// =========================================================================

#[test]
fn test_http_client_scroll_down() {
    let mut client = HttpClient::new();
    client.scroll_down(10);
    assert_eq!(client.body_scroll_for_testing(), 10);
    client.scroll_down(5);
    assert_eq!(client.body_scroll_for_testing(), 15);
}

#[test]
fn test_http_client_scroll_up() {
    let mut client = HttpClient::new();
    client.set_body_scroll_for_testing(20);
    client.scroll_up(5);
    assert_eq!(client.body_scroll_for_testing(), 15);
}

#[test]
fn test_http_client_scroll_up_clamps_to_zero() {
    let mut client = HttpClient::new();
    client.set_body_scroll_for_testing(5);
    client.scroll_up(10);
    assert_eq!(client.body_scroll_for_testing(), 0);
}

// =========================================================================
// History tests
// =========================================================================

#[test]
fn test_http_client_history_back() {
    let mut client = HttpClient::new();
    client.set_url("url1");
    client.add_to_history_for_testing();
    client.set_url("url2");
    client.add_to_history_for_testing();

    client.history_back();
    assert_eq!(client.history_index_for_testing(), 1);
    assert_eq!(client.request().url, "url2"); // history[1] contains "url2"
}

#[test]
fn test_http_client_history_back_at_start() {
    let mut client = HttpClient::new();
    client.history_back();
    assert_eq!(client.history_index_for_testing(), 0);
}

#[test]
fn test_http_client_history_forward() {
    let mut client = HttpClient::new();
    client.set_url("url1");
    client.add_to_history_for_testing();

    client.history_forward();
    assert_eq!(client.history_index_for_testing(), 1);
}

#[test]
fn test_http_client_history_forward_at_end() {
    let mut client = HttpClient::new();
    client.add_default_to_history_for_testing();

    client.history_forward();
    assert_eq!(client.history_index_for_testing(), 1); // Stays at 1 (end of history)
    assert!(client.request().url.is_empty());
}
