//! Constructor Tests - 생성자 테스트

use revue::widget::{HttpClient, HttpMethod, RequestState};

#[test]
fn test_http_client_new() {
    let client = HttpClient::new();
    assert_eq!(client.state(), RequestState::Idle);
    assert!(client.response().is_none());
    assert!(client.error().is_none());
    assert_eq!(client.request().url, "");
    assert_eq!(client.request().method, HttpMethod::default());
}

#[test]
fn test_http_client_default() {
    let client = HttpClient::default();
    assert_eq!(client.state(), RequestState::Idle);
    assert!(client.request().headers.is_empty());
    assert!(client.request().params.is_empty());
    assert_eq!(client.request().body, "");
}

#[test]
fn test_http_client_helper() {
    let client = revue::widget::http_client();
    assert_eq!(client.state(), RequestState::Idle);
}
