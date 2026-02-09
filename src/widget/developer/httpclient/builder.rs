//! Request builder for fluent API

use std::io::Write;

use super::request::HttpRequest;
use super::types::HttpMethod;

/// A fluent request builder for constructing HTTP requests
///
/// # Example
///
/// ```rust,ignore
/// use revue::widget::RequestBuilder;
///
/// let request = RequestBuilder::get("https://api.example.com/users")
///     .header("Authorization", "Bearer token")
///     .param("page", "1")
///     .param("limit", "10")
///     .build();
/// ```
pub struct RequestBuilder {
    request: HttpRequest,
}

impl RequestBuilder {
    /// Create a new GET request builder
    pub fn get(url: impl Into<String>) -> Self {
        Self {
            request: HttpRequest::new(url).method(HttpMethod::GET),
        }
    }

    /// Create a new POST request builder
    pub fn post(url: impl Into<String>) -> Self {
        Self {
            request: HttpRequest::new(url).method(HttpMethod::POST),
        }
    }

    /// Create a new PUT request builder
    pub fn put(url: impl Into<String>) -> Self {
        Self {
            request: HttpRequest::new(url).method(HttpMethod::PUT),
        }
    }

    /// Create a new DELETE request builder
    pub fn delete(url: impl Into<String>) -> Self {
        Self {
            request: HttpRequest::new(url).method(HttpMethod::DELETE),
        }
    }

    /// Create a new PATCH request builder
    pub fn patch(url: impl Into<String>) -> Self {
        Self {
            request: HttpRequest::new(url).method(HttpMethod::PATCH),
        }
    }

    /// Add a header
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.request = self.request.header(key, value);
        self
    }

    /// Add a query parameter
    pub fn param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.request = self.request.param(key, value);
        self
    }

    /// Set the request body
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.request = self.request.body(body);
        self
    }

    /// Set JSON body with Content-Type header
    pub fn json(self, body: impl Into<String>) -> Self {
        self.header("Content-Type", "application/json").body(body)
    }

    /// Set form body with Content-Type header
    pub fn form(self, body: impl Into<String>) -> Self {
        self.header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
    }

    /// Set bearer token authorization
    pub fn bearer_auth(self, token: impl Into<String>) -> Self {
        self.header("Authorization", format!("Bearer {}", token.into()))
    }

    /// Set basic authorization
    pub fn basic_auth(self, username: impl Into<String>, password: impl Into<String>) -> Self {
        let credentials = format!("{}:{}", username.into(), password.into());
        // Simple base64 encoding
        let encoded = base64_encode(credentials.as_bytes());
        let mut encoder = Vec::new();
        let _ = write!(encoder, "Basic {}", encoded);
        self.header(
            "Authorization",
            String::from_utf8(encoder).unwrap_or_default(),
        )
    }

    /// Build the request
    pub fn build(self) -> HttpRequest {
        self.request
    }
}

/// Simple base64 encoder (no external dependencies)
#[doc(hidden)]
pub fn base64_encode(data: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut result = String::new();
    let chunks = data.chunks(3);

    for chunk in chunks {
        let b0 = chunk[0] as usize;
        let b1 = chunk.get(1).copied().unwrap_or(0) as usize;
        let b2 = chunk.get(2).copied().unwrap_or(0) as usize;

        result.push(ALPHABET[b0 >> 2] as char);
        result.push(ALPHABET[((b0 & 0x03) << 4) | (b1 >> 4)] as char);

        if chunk.len() > 1 {
            result.push(ALPHABET[((b1 & 0x0F) << 2) | (b2 >> 6)] as char);
        } else {
            result.push('=');
        }

        if chunk.len() > 2 {
            result.push(ALPHABET[b2 & 0x3F] as char);
        } else {
            result.push('=');
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // RequestBuilder::get tests
    // =========================================================================

    #[test]
    fn test_request_builder_get() {
        let builder = RequestBuilder::get("https://example.com");
        let request = builder.build();
        assert_eq!(request.url, "https://example.com");
        assert_eq!(request.method, HttpMethod::GET);
    }

    #[test]
    fn test_request_builder_get_with_string() {
        let builder = RequestBuilder::get(String::from("https://api.test.com"));
        let request = builder.build();
        assert_eq!(request.url, "https://api.test.com");
    }

    // =========================================================================
    // RequestBuilder::post tests
    // =========================================================================

    #[test]
    fn test_request_builder_post() {
        let builder = RequestBuilder::post("https://example.com");
        let request = builder.build();
        assert_eq!(request.url, "https://example.com");
        assert_eq!(request.method, HttpMethod::POST);
    }

    // =========================================================================
    // RequestBuilder::put tests
    // =========================================================================

    #[test]
    fn test_request_builder_put() {
        let builder = RequestBuilder::put("https://example.com");
        let request = builder.build();
        assert_eq!(request.url, "https://example.com");
        assert_eq!(request.method, HttpMethod::PUT);
    }

    // =========================================================================
    // RequestBuilder::delete tests
    // =========================================================================

    #[test]
    fn test_request_builder_delete() {
        let builder = RequestBuilder::delete("https://example.com");
        let request = builder.build();
        assert_eq!(request.url, "https://example.com");
        assert_eq!(request.method, HttpMethod::DELETE);
    }

    // =========================================================================
    // RequestBuilder::patch tests
    // =========================================================================

    #[test]
    fn test_request_builder_patch() {
        let builder = RequestBuilder::patch("https://example.com");
        let request = builder.build();
        assert_eq!(request.url, "https://example.com");
        assert_eq!(request.method, HttpMethod::PATCH);
    }

    // =========================================================================
    // RequestBuilder::header tests
    // =========================================================================

    #[test]
    fn test_request_builder_header_single() {
        let builder =
            RequestBuilder::get("https://example.com").header("Authorization", "Bearer token");
        let request = builder.build();
        assert!(request.headers.contains_key("Authorization"));
    }

    #[test]
    fn test_request_builder_header_multiple() {
        let builder = RequestBuilder::get("https://example.com")
            .header("Accept", "application/json")
            .header("User-Agent", "TestClient");
        let request = builder.build();
        assert!(request.headers.contains_key("Accept"));
        assert!(request.headers.contains_key("User-Agent"));
    }

    // =========================================================================
    // RequestBuilder::param tests
    // =========================================================================

    #[test]
    fn test_request_builder_param_single() {
        let builder = RequestBuilder::get("https://example.com").param("page", "1");
        let request = builder.build();
        assert!(request.params.contains_key("page"));
        assert_eq!(request.params.get("page").map(|v| v.as_str()), Some("1"));
    }

    #[test]
    fn test_request_builder_param_multiple() {
        let builder = RequestBuilder::get("https://example.com")
            .param("page", "1")
            .param("limit", "10");
        let request = builder.build();
        assert!(request.params.contains_key("page"));
        assert!(request.params.contains_key("limit"));
        assert_eq!(request.params.get("page").map(|v| v.as_str()), Some("1"));
        assert_eq!(request.params.get("limit").map(|v| v.as_str()), Some("10"));
    }

    // =========================================================================
    // RequestBuilder::body tests
    // =========================================================================

    #[test]
    fn test_request_builder_body() {
        let builder = RequestBuilder::post("https://example.com").body("{\"key\":\"value\"}");
        let request = builder.build();
        assert_eq!(request.body, "{\"key\":\"value\"}");
    }

    #[test]
    fn test_request_builder_body_empty() {
        let builder = RequestBuilder::post("https://example.com").body("");
        let request = builder.build();
        assert_eq!(request.body, "");
    }

    // =========================================================================
    // RequestBuilder::json tests
    // =========================================================================

    #[test]
    fn test_request_builder_json() {
        let builder = RequestBuilder::post("https://example.com").json("{\"test\":true}");
        let request = builder.build();
        assert_eq!(request.body, "{\"test\":true}");
        assert!(request.headers.contains_key("Content-Type"));
        assert_eq!(
            request.headers.get("Content-Type").map(|v| v.as_str()),
            Some("application/json")
        );
    }

    // =========================================================================
    // RequestBuilder::form tests
    // =========================================================================

    #[test]
    fn test_request_builder_form() {
        let builder = RequestBuilder::post("https://example.com").form("key=value&foo=bar");
        let request = builder.build();
        assert_eq!(request.body, "key=value&foo=bar");
        assert!(request.headers.contains_key("Content-Type"));
        assert_eq!(
            request.headers.get("Content-Type").map(|v| v.as_str()),
            Some("application/x-www-form-urlencoded")
        );
    }

    // =========================================================================
    // RequestBuilder::bearer_auth tests
    // =========================================================================

    #[test]
    fn test_request_builder_bearer_auth() {
        let builder = RequestBuilder::get("https://example.com").bearer_auth("my-token");
        let request = builder.build();
        assert!(request.headers.contains_key("Authorization"));
        let auth_header = request.headers.get("Authorization").map(|v| v.as_str());
        assert_eq!(auth_header, Some("Bearer my-token"));
    }

    // =========================================================================
    // RequestBuilder::basic_auth tests
    // =========================================================================

    #[test]
    fn test_request_builder_basic_auth() {
        let builder = RequestBuilder::get("https://example.com").basic_auth("user", "pass");
        let request = builder.build();
        assert!(request.headers.contains_key("Authorization"));
        let auth_header = request.headers.get("Authorization").map(|v| v.as_str());
        assert!(auth_header.unwrap().starts_with("Basic "));
    }

    #[test]
    fn test_request_builder_basic_auth_empty_password() {
        let builder = RequestBuilder::get("https://example.com").basic_auth("user", "");
        let request = builder.build();
        assert!(request.headers.contains_key("Authorization"));
    }

    // =========================================================================
    // RequestBuilder::build tests
    // =========================================================================

    #[test]
    fn test_request_builder_build_simple() {
        let request = RequestBuilder::get("https://example.com").build();
        assert_eq!(request.url, "https://example.com");
        assert_eq!(request.method, HttpMethod::GET);
    }

    #[test]
    fn test_request_builder_build_complex() {
        let request = RequestBuilder::post("https://api.example.com/data")
            .header("Authorization", "Bearer token")
            .header("Accept", "application/json")
            .param("version", "v1")
            .json("{\"key\":\"value\"}")
            .build();

        assert_eq!(request.method, HttpMethod::POST);
        assert!(request.headers.contains_key("Authorization"));
        assert!(request.headers.contains_key("Content-Type"));
        assert!(request.params.contains_key("version"));
        assert_eq!(
            request.params.get("version").map(|v| v.as_str()),
            Some("v1")
        );
    }

    // =========================================================================
    // RequestBuilder chaining tests
    // =========================================================================

    #[test]
    fn test_request_builder_chain_all_methods() {
        let request = RequestBuilder::post("https://example.com")
            .header("X-Custom", "value")
            .param("debug", "true")
            .bearer_auth("token")
            .body("request body")
            .build();

        assert_eq!(request.method, HttpMethod::POST);
        assert!(request.headers.contains_key("X-Custom"));
        assert!(request.headers.contains_key("Authorization"));
        assert!(request.params.contains_key("debug"));
        assert_eq!(
            request.params.get("debug").map(|v| v.as_str()),
            Some("true")
        );
        assert_eq!(request.body, "request body");
    }

    // =========================================================================
    // base64_encode tests
    // =========================================================================

    #[test]
    fn test_base64_encode_empty() {
        assert_eq!(base64_encode(b""), "");
    }

    #[test]
    fn test_base64_encode_single_byte() {
        assert_eq!(base64_encode(b"M"), "TQ==");
    }

    #[test]
    fn test_base64_encode_two_bytes() {
        assert_eq!(base64_encode(b"Ma"), "TWE=");
    }

    #[test]
    fn test_base64_encode_three_bytes() {
        assert_eq!(base64_encode(b"Man"), "TWFu");
    }

    #[test]
    fn test_base64_encode_four_bytes() {
        assert_eq!(base64_encode(b"Mana"), "TWFuYQ==");
    }

    #[test]
    fn test_base64_encode_six_bytes() {
        assert_eq!(base64_encode(b"Manam"), "TWFuYW0=");
    }

    #[test]
    fn test_base64_encode_hello() {
        assert_eq!(base64_encode(b"Hello"), "SGVsbG8=");
    }

    #[test]
    fn test_base64_encode_credentials() {
        // user:pass in base64
        assert_eq!(base64_encode(b"user:pass"), "dXNlcjpwYXNz");
    }

    #[test]
    fn test_base64_encode_all_bytes() {
        // Test all 256 byte values would be too long,
        // just verify it works on a larger input
        let input = b"The quick brown fox jumps over the lazy dog.";
        let result = base64_encode(input);
        // Just verify it's a valid base64 string (only valid chars)
        assert!(result
            .chars()
            .all(|c| c.is_alphanumeric() || c == '+' || c == '/' || c == '='));
    }

    #[test]
    fn test_base64_encode_binary_data() {
        // Test with binary data (null bytes)
        assert_eq!(base64_encode(&[0, 0, 0]), "AAAA");
        assert_eq!(base64_encode(&[255, 255, 255]), "////");
    }
}
