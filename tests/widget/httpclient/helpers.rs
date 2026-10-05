//! http_client() and the per-method helpers

use revue::widget::{
    http_client, http_delete, http_get, http_patch, http_post, http_put, HttpClient, HttpMethod,
    RequestState,
};

#[test]
fn test_http_client_helper() {
    let client = http_client();
    assert_eq!(client.state(), RequestState::Idle);
    assert_eq!(client.request().url(), "");
    assert_eq!(client.request().method, HttpMethod::GET);
}

/// A per-method helper such as http_get.
type Helper = fn(String) -> HttpClient;

#[test]
fn test_method_helpers_set_method_and_url() {
    let cases: [(Helper, HttpMethod); 5] = [
        (http_get, HttpMethod::GET),
        (http_post, HttpMethod::POST),
        (http_put, HttpMethod::PUT),
        (http_delete, HttpMethod::DELETE),
        (http_patch, HttpMethod::PATCH),
    ];
    for (helper, method) in cases {
        let client = helper("https://api.example.com/users".to_string());
        assert_eq!(client.request().method, method);
        assert_eq!(client.request().url(), "https://api.example.com/users");
        assert_eq!(client.state(), RequestState::Idle);
        assert!(client.error().is_none());
    }
}

#[test]
fn test_method_helper_accepts_str_and_long_paths() {
    let url = "https://example.com/very/long/path/that/goes/on/and/on";
    assert_eq!(http_put(url).request().url(), url);
}

#[test]
fn test_method_helper_with_invalid_url_reports_error() {
    let client = http_get("");
    assert_eq!(client.request().url(), "");
    assert_eq!(client.state(), RequestState::Error);
    assert!(client.error().unwrap().contains("Invalid URL"));
}
