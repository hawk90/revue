//! HttpResponse tests

use revue::widget::developer::httpclient::response::HttpResponse;
use revue::widget::developer::httpclient::types::ContentType;
use revue::style::Color;
use std::time::Duration;

// =========================================================================
// HttpResponse struct tests
// =========================================================================

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
fn test_http_response_clone() {
    let mut response1 = HttpResponse::default();
    response1.status = 200;
    response1.body = "test".to_string();
    let response2 = response1.clone();
    assert_eq!(response1.status, response2.status);
    assert_eq!(response1.body, response2.body);
}

#[test]
fn test_http_response_debug() {
    let response = HttpResponse::default();
    let debug_str = format!("{:?}", response);
    assert!(debug_str.contains("HttpResponse"));
}

#[test]
fn test_is_success_true_2xx() {
    let mut response = HttpResponse::default();
    response.status = 200;
    assert!(response.is_success());

    response.status = 250;
    assert!(response.is_success());

    response.status = 299;
    assert!(response.is_success());
}

#[test]
fn test_is_success_false_3xx() {
    let mut response = HttpResponse::default();
    response.status = 301;
    assert!(!response.is_success());
}

#[test]
fn test_is_success_false_4xx() {
    let mut response = HttpResponse::default();
    response.status = 404;
    assert!(!response.is_success());
}

#[test]
fn test_is_success_false_5xx() {
    let mut response = HttpResponse::default();
    response.status = 500;
    assert!(!response.is_success());
}

#[test]
fn test_status_color_2xx() {
    let mut response = HttpResponse::default();
    response.status = 200;
    assert_eq!(response.status_color(), Color::rgb(152, 195, 121));
}

#[test]
fn test_status_color_3xx() {
    let mut response = HttpResponse::default();
    response.status = 301;
    assert_eq!(response.status_color(), Color::rgb(229, 192, 123));
}

#[test]
fn test_status_color_4xx() {
    let mut response = HttpResponse::default();
    response.status = 404;
    assert_eq!(response.status_color(), Color::rgb(224, 108, 117));
}

#[test]
fn test_status_color_5xx() {
    let mut response = HttpResponse::default();
    response.status = 500;
    assert_eq!(response.status_color(), Color::rgb(198, 120, 221));
}

#[test]
fn test_status_color_other() {
    let mut response = HttpResponse::default();
    response.status = 100;
    assert_eq!(response.status_color(), Color::rgb(171, 178, 191));
}

#[test]
fn test_content_type_json() {
    let mut response = HttpResponse::default();
    response
        .headers
        .insert("Content-Type".to_string(), "application/json".to_string());
    assert_eq!(response.content_type(), ContentType::Json);
}

#[test]
fn test_content_type_xml() {
    let mut response = HttpResponse::default();
    response
        .headers
        .insert("Content-Type".to_string(), "application/xml".to_string());
    assert_eq!(response.content_type(), ContentType::Xml);
}

#[test]
fn test_content_type_html() {
    let mut response = HttpResponse::default();
    response
        .headers
        .insert("Content-Type".to_string(), "text/html".to_string());
    assert_eq!(response.content_type(), ContentType::Html);
}

#[test]
fn test_content_type_text() {
    let mut response = HttpResponse::default();
    response
        .headers
        .insert("Content-Type".to_string(), "text/plain".to_string());
    assert_eq!(response.content_type(), ContentType::Text);
}

#[test]
fn test_content_type_missing() {
    let response = HttpResponse::default();
    assert_eq!(response.content_type(), ContentType::Text);
}

#[test]
fn test_pretty_json_valid() {
    let mut response = HttpResponse::default();
    response.body = r#"{"name":"test","value":123}"#.to_string();
    let formatted = response.pretty_json();
    assert!(formatted.is_some());
    let formatted_str = formatted.unwrap();
    assert!(formatted_str.contains("{"));
    assert!(formatted_str.contains("\n"));
}

#[test]
fn test_pretty_json_empty() {
    let mut response = HttpResponse::default();
    response.body = "".to_string();
    let formatted = response.pretty_json();
    // Empty string returns None (line 116-120: if result.is_empty() { None })
    assert!(formatted.is_none());
}

#[test]
fn test_format_json_nested() {
    let response = HttpResponse::default();
    let json = r#"{"outer":{"inner":"value"}}"#;
    let formatted = response.format_json(json);
    assert!(formatted.is_some());
    let formatted_str = formatted.unwrap();
    assert!(formatted_str.contains("outer"));
    assert!(formatted_str.contains("inner"));
}

#[test]
fn test_format_json_array() {
    let response = HttpResponse::default();
    let json = r#"[1,2,3]"#;
    let formatted = response.format_json(json);
    assert!(formatted.is_some());
    let formatted_str = formatted.unwrap();
    assert!(formatted_str.contains("["));
}

#[test]
fn test_format_json_with_strings() {
    let response = HttpResponse::default();
    let json = r#"{"key":"value with spaces"}"#;
    let formatted = response.format_json(json);
    assert!(formatted.is_some());
    let formatted_str = formatted.unwrap();
    assert!(formatted_str.contains("value with spaces"));
}

#[test]
fn test_formatted_body_json() {
    let mut response = HttpResponse::default();
    response
        .headers
        .insert("Content-Type".to_string(), "application/json".to_string());
    response.body = r#"{"test":true}"#.to_string();
    let formatted = response.formatted_body();
    assert!(formatted.contains("{"));
    assert!(formatted.contains("\n"));
}

#[test]
fn test_formatted_body_non_json() {
    let mut response = HttpResponse::default();
    response
        .headers
        .insert("Content-Type".to_string(), "text/plain".to_string());
    response.body = "plain text".to_string();
    let formatted = response.formatted_body();
    assert_eq!(formatted, "plain text");
}

#[test]
fn test_formatted_body_invalid_json_fallback() {
    let mut response = HttpResponse::default();
    response
        .headers
        .insert("Content-Type".to_string(), "application/json".to_string());
    response.body = "not json".to_string();
    let formatted = response.formatted_body();
    // format_json strips whitespace outside strings, so "not json" becomes "notjson"
    assert_eq!(formatted, "notjson");
}
