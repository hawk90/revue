//! HttpResponse tests
//!
//! is_success for 200/201/299/404/500, distinct status colors, content
//! type from a JSON header, pretty_json, formatted_body for JSON and text,
//! and format_json on objects, nested objects, arrays, escaped quotes and
//! empty input are covered by the in-source tests in
//! src/widget/developer/httpclient/tests.rs.

use revue::style::Color;
use revue::widget::{ContentType, HttpResponse};
use std::time::Duration;

fn with_status(status: u16) -> HttpResponse {
    HttpResponse {
        status,
        ..Default::default()
    }
}

fn with_content_type(content_type: &str, body: &str) -> HttpResponse {
    let mut response = HttpResponse {
        body: body.to_string(),
        ..Default::default()
    };
    response
        .headers
        .insert("Content-Type".to_string(), content_type.to_string());
    response
}

#[test]
fn test_http_response_default() {
    let response = HttpResponse::default();
    assert_eq!(response.status, 0);
    assert_eq!(response.status_text, "");
    assert!(response.headers.is_empty());
    assert_eq!(response.body, "");
    assert_eq!(response.time, Duration::ZERO);
    assert_eq!(response.size, 0);
}

#[test]
fn test_is_success_only_for_2xx() {
    for status in 200..=299 {
        assert!(with_status(status).is_success(), "{status}");
    }
    for status in [0, 100, 199, 300, 301, 404, 500] {
        assert!(!with_status(status).is_success(), "{status}");
    }
}

#[test]
fn test_status_color_by_class() {
    assert_eq!(with_status(200).status_color(), Color::rgb(152, 195, 121));
    assert_eq!(with_status(301).status_color(), Color::rgb(229, 192, 123));
    assert_eq!(with_status(404).status_color(), Color::rgb(224, 108, 117));
    assert_eq!(with_status(500).status_color(), Color::rgb(198, 120, 221));
    assert_eq!(with_status(100).status_color(), Color::rgb(171, 178, 191));
}

#[test]
fn test_content_type_from_response_header() {
    assert_eq!(
        with_content_type("application/xml", "").content_type(),
        ContentType::Xml
    );
    assert_eq!(
        with_content_type("text/html", "").content_type(),
        ContentType::Html
    );
    assert_eq!(
        with_content_type("text/plain", "").content_type(),
        ContentType::Text
    );
    assert_eq!(HttpResponse::default().content_type(), ContentType::Text);
}

#[test]
fn test_pretty_json_exact_layout() {
    let response = HttpResponse {
        body: r#"{"name":"test","value":123}"#.to_string(),
        ..Default::default()
    };
    assert_eq!(
        response.pretty_json().as_deref(),
        Some("{\n  \"name\": \"test\",\n  \"value\": 123\n}")
    );
}

#[test]
fn test_pretty_json_empty_body() {
    assert_eq!(HttpResponse::default().pretty_json(), None);
}

#[test]
fn test_format_json_keeps_whitespace_inside_strings() {
    let formatted = HttpResponse::default()
        .format_json(r#"{ "key" : "value with spaces" }"#)
        .unwrap();
    assert_eq!(formatted, "{\n  \"key\": \"value with spaces\"\n}");
}

#[test]
fn test_format_json_nested_indentation() {
    let formatted = HttpResponse::default()
        .format_json(r#"{"outer":{"inner":[1,2]}}"#)
        .unwrap();
    assert_eq!(
        formatted,
        "{\n  \"outer\": {\n    \"inner\": [\n      1,\n      2\n    ]\n  }\n}"
    );
}

#[test]
fn test_formatted_body_non_json_untouched() {
    let response = with_content_type("text/plain", "  plain   text  ");
    assert_eq!(response.formatted_body(), "  plain   text  ");
}

#[test]
fn test_formatted_body_json_header_on_non_json_body() {
    // A body that is not JSON is shown as sent, not re-indented.
    for body in [
        "not json",
        "Internal Server Error",
        "{\"truncated\": [1, 2",
        "{} trailing",
    ] {
        let response = with_content_type("application/json", body);
        assert_eq!(response.formatted_body(), body);
    }
}

#[test]
fn test_formatted_body_json_with_surrounding_whitespace() {
    let response = with_content_type("application/json", "  {\"a\": [1, 2]}\n");
    assert_eq!(
        response.formatted_body(),
        "{\n  \"a\": [\n    1,\n    2\n  ]\n}"
    );
}
