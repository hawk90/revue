//! HTTP Method & Request State Tests

use revue::widget::{http_delete, http_get, http_patch, http_post, http_put};
use revue::widget::{HttpClient, HttpMethod, HttpResponse, RequestState};
use std::collections::HashMap;
use std::time::Duration;

#[test]
fn test_http_client_method_get() {
    let client = HttpClient::new().method(HttpMethod::GET);
    assert_eq!(client.request().method, HttpMethod::GET);
}

#[test]
fn test_http_client_method_post() {
    let client = HttpClient::new().method(HttpMethod::POST);
    assert_eq!(client.request().method, HttpMethod::POST);
}

#[test]
fn test_get_helper() {
    let client = http_get("https://api.example.com/users");
    assert_eq!(client.request().method, HttpMethod::GET);
    assert_eq!(client.request().url, "https://api.example.com/users");
}

#[test]
fn test_post_helper() {
    let client = http_post("https://api.example.com/users");
    assert_eq!(client.request().method, HttpMethod::POST);
}

#[test]
fn test_cycle_method() {
    let mut client = HttpClient::new().method(HttpMethod::GET);
    client.cycle_method();
    assert_eq!(client.request().method, HttpMethod::POST);
}

#[test]
fn test_request_state_initial() {
    let client = HttpClient::new();
    assert_eq!(client.state(), RequestState::Idle);
}

#[test]
fn test_request_state_after_send() {
    let mut client = http_get("https://example.com");
    client.send();
    assert_eq!(client.state(), RequestState::Success);
}

#[test]
fn test_request_state_error() {
    let mut client = HttpClient::new();
    client.set_error("Connection failed");
    assert_eq!(client.state(), RequestState::Error);
}
