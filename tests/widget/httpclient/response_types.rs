//! Response Types Tests

use revue::widget::{ContentType, HttpClient, HttpResponse, ResponseView};
use std::collections::HashMap;
use std::time::Duration;

#[test]
fn test_response_after_send() {
    let mut client = revue::widget::http_get("https://example.com");
    client.send();
    assert!(client.response().is_some());
}

#[test]
fn test_response_status_code() {
    let mut client = HttpClient::new();
    client.set_response(HttpResponse {
        status: 201,
        status_text: "Created".to_string(),
        headers: HashMap::new(),
        body: "".to_string(),
        time: Duration::from_millis(10),
        size: 0,
    });
    assert_eq!(client.response().unwrap().status, 201);
}

#[test]
fn test_response_is_success_2xx() {
    let mut response = HttpResponse::default();
    for status in 200..=299 {
        response.status = status;
        assert!(response.is_success());
    }
}

#[test]
fn test_content_type_json() {
    assert_eq!(ContentType::from_header(Some("application/json")), ContentType::Json);
}

#[test]
fn test_content_type_xml() {
    assert_eq!(ContentType::from_header(Some("application/xml")), ContentType::Xml);
}

#[test]
fn test_content_type_none() {
    assert_eq!(ContentType::from_header(None), ContentType::Text);
}

#[test]
fn test_response_view_default() {
    let view = ResponseView::default();
    assert_eq!(view, ResponseView::Body);
}

#[test]
fn test_set_view() {
    let mut client = HttpClient::new();
    client.set_view(ResponseView::Headers);
}
