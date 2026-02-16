//! HTTP Client helper functions

use super::client::HttpClient;
use super::types::HttpMethod;

/// Create a new HTTP client
pub fn http_client() -> HttpClient {
    HttpClient::new()
}

/// Create a GET request
pub fn get(url: impl Into<String>) -> HttpClient {
    HttpClient::new().url(url).method(HttpMethod::GET)
}

/// Create a POST request
pub fn post(url: impl Into<String>) -> HttpClient {
    HttpClient::new().url(url).method(HttpMethod::POST)
}

/// Create a PUT request
pub fn put(url: impl Into<String>) -> HttpClient {
    HttpClient::new().url(url).method(HttpMethod::PUT)
}

/// Create a DELETE request
pub fn delete(url: impl Into<String>) -> HttpClient {
    HttpClient::new().url(url).method(HttpMethod::DELETE)
}

/// Create a PATCH request
pub fn patch(url: impl Into<String>) -> HttpClient {
    HttpClient::new().url(url).method(HttpMethod::PATCH)
}
