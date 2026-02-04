//! Headers & Body Tests

use revue::widget::HttpClient;

#[test]
fn test_http_client_single_header() {
    let client = HttpClient::new().header("Authorization", "Bearer token123");
    assert_eq!(client.request().headers.get("Authorization"), Some(&"Bearer token123".to_string()));
}

#[test]
fn test_http_client_multiple_headers() {
    let client = HttpClient::new()
        .header("Authorization", "Bearer token")
        .header("Content-Type", "application/json");
    assert_eq!(client.request().headers.len(), 2);
}

#[test]
fn test_http_client_header_override() {
    let client = HttpClient::new()
        .header("X-Custom", "value1")
        .header("X-Custom", "value2");
    assert_eq!(client.request().headers.get("X-Custom"), Some(&"value2".to_string()));
}

#[test]
fn test_http_client_body() {
    let client = HttpClient::new().body(r#"{"name":"test"}"#);
    assert_eq!(client.request().body, r#"{"name":"test"}"#);
}

#[test]
fn test_http_client_json_body() {
    let json = r#"{"user":"john","age":30}"#;
    let client = HttpClient::new()
        .header("Content-Type", "application/json")
        .body(json);
    assert_eq!(client.request().body, json);
}

#[test]
fn test_http_client_empty_body() {
    let client = HttpClient::new().body("");
    assert_eq!(client.request().body, "");
}
