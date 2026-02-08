//! HTTP Client helper functions

use super::client::HttpClient;
use super::types::HttpMethod;

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Helper function tests
    // =========================================================================

    #[test]
    fn test_http_client_function() {
        let client = http_client();
        let _ = client;
    }

    #[test]
    fn test_get_function() {
        let client = get("https://example.com");
        let _ = client;
    }

    #[test]
    fn test_get_function_with_string() {
        let url = String::from("https://test.com");
        let client = get(url);
        let _ = client;
    }

    #[test]
    fn test_post_function() {
        let client = post("https://example.com");
        let _ = client;
    }

    #[test]
    fn test_put_function() {
        let client = put("https://example.com");
        let _ = client;
    }

    #[test]
    fn test_delete_function() {
        let client = delete("https://example.com");
        let _ = client;
    }

    #[test]
    fn test_patch_function() {
        let client = patch("https://example.com");
        let _ = client;
    }

    // =========================================================================
    // Edge cases
    // =========================================================================

    #[test]
    fn test_get_with_empty_url() {
        let client = get("");
        let _ = client;
    }

    #[test]
    fn test_post_with_string_url() {
        let url = "https://api.example.com".to_string();
        let client = post(url);
        let _ = client;
    }

    #[test]
    fn test_put_with_long_url() {
        let url = "https://example.com/very/long/path/that/goes/on/and/on";
        let client = put(url);
        let _ = client;
    }
}

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
