//! URL Builder Tests

use revue::widget::HttpClient;

#[test]
fn test_http_client_url_builder() {
    let client = HttpClient::new().url("https://api.example.com");
    assert_eq!(client.request().url, "https://api.example.com");
}

#[test]
fn test_http_client_url_chaining() {
    let client = HttpClient::new().url("https://api.example.com").url("https://other.com");
    assert_eq!(client.request().url, "https://other.com");
}

#[test]
fn test_http_client_set_url() {
    let mut client = HttpClient::new();
    client.set_url("https://api.example.com/users");
    assert_eq!(client.request().url, "https://api.example.com/users");
}

#[test]
fn test_http_client_empty_url() {
    let client = HttpClient::new().url("");
    assert_eq!(client.request().url, "");
}

#[test]
fn test_http_client_set_empty_url() {
    let mut client = HttpClient::new().url("https://example.com");
    client.set_url("");
    assert_eq!(client.request().url, "");
}
