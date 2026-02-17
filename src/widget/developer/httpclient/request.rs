//! HTTP request types

use std::collections::HashMap;

use super::types::HttpMethod;

/// Maximum URL length to prevent DoS
const MAX_URL_LENGTH: usize = 8192;

/// Allowed URL schemes for security
const ALLOWED_SCHEMES: &[&str] = &["http://", "https://"];

/// HTTP request builder
#[derive(Clone, Debug, Default)]
pub struct HttpRequest {
    /// HTTP method
    pub method: HttpMethod,
    /// Request URL (private: use url() getter for access)
    pub(crate) url: String,
    /// Request headers
    pub headers: HashMap<String, String>,
    /// Request body
    pub body: String,
    /// Query parameters
    pub params: HashMap<String, String>,
}

impl HttpRequest {
    /// Create a new request with URL validation
    ///
    /// # Security
    ///
    /// Only allows http:// and https:// schemes to prevent SSRF attacks.
    /// Enforces maximum URL length to prevent DoS.
    ///
    /// # Errors
    ///
    /// Returns `None` if:
    /// - URL doesn't start with http:// or https://
    /// - URL exceeds MAX_URL_LENGTH
    pub fn new(url: impl Into<String>) -> Option<Self> {
        let url = url.into();

        // Validate URL length
        if url.len() > MAX_URL_LENGTH {
            return None;
        }

        // Validate URL scheme (prevent SSRF)
        let has_valid_scheme = ALLOWED_SCHEMES.iter().any(|scheme| {
            url.as_bytes()
                .get(0..scheme.len())
                .map(|prefix| prefix == scheme.as_bytes())
                .unwrap_or(false)
        });

        if !has_valid_scheme {
            return None;
        }

        Some(Self {
            url,
            ..Default::default()
        })
    }

    /// Get the URL (immutable access)
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Set method
    pub fn method(mut self, method: HttpMethod) -> Self {
        self.method = method;
        self
    }

    /// Add header
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// Set body
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = body.into();
        self
    }

    /// Add query parameter
    pub fn param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.params.insert(key.into(), value.into());
        self
    }

    /// Build URL with query parameters
    ///
    /// URL-encodes all query parameters to prevent injection attacks.
    /// Enforces maximum URL length.
    pub fn full_url(&self) -> String {
        if self.params.is_empty() {
            self.url.clone()
        } else {
            let params: Vec<String> = self
                .params
                .iter()
                .map(|(k, v)| format!("{}={}", percent_encode(k), percent_encode(v)))
                .collect();
            format!("{}?{}", self.url, params.join("&"))
        }
    }
}

/// Percent-encode a string for use in URL query parameters
///
/// Encodes all characters except unreserved characters (A-Z, a-z, 0-9, -, _, ., ~).
/// This prevents injection attacks through special characters.
fn percent_encode(s: &str) -> String {
    let mut encoded = String::with_capacity(s.len());
    for byte in s.as_bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(*byte as char);
            }
            b' ' => encoded.push('+'),
            _ => {
                encoded.push('%');
                encoded.push(hex_char(*byte >> 4));
                encoded.push(hex_char(*byte & 0x0F));
            }
        }
    }
    encoded
}

/// Convert a nibble (0-15) to its uppercase hex character
/// Uses uppercase per RFC 3986 (recommended for URL encoding)
fn hex_char(nibble: u8) -> char {
    match nibble {
        0..=9 => (b'0' + nibble) as char,
        10..=15 => (b'A' + nibble - 10) as char,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percent_encode() {
        assert_eq!(percent_encode("hello"), "hello");
        assert_eq!(percent_encode("hello world"), "hello+world");
        assert_eq!(percent_encode("a&b=c"), "a%26b%3Dc");
        assert_eq!(percent_encode("<script>"), "%3Cscript%3E");
        assert_eq!(percent_encode("a/b"), "a%2Fb");
    }

    #[test]
    fn test_new_valid_urls() {
        assert!(HttpRequest::new("http://example.com").is_some());
        assert!(HttpRequest::new("https://example.com").is_some());
        assert!(HttpRequest::new("https://example.com/path?query=value").is_some());
    }

    #[test]
    fn test_new_rejects_invalid_schemes() {
        // SSRF prevention: reject non-http schemes
        assert!(HttpRequest::new("file:///etc/passwd").is_none());
        assert!(HttpRequest::new("ftp://example.com").is_none());
        assert!(HttpRequest::new("javascript:alert(1)").is_none());
        assert!(HttpRequest::new("data:text/html,<script>alert(1)</script>").is_none());
        assert!(HttpRequest::new("//example.com").is_none()); // scheme-relative
    }

    #[test]
    fn test_new_rejects_long_urls() {
        let long_url = format!("https://example.com/?{}", "a".repeat(MAX_URL_LENGTH));
        assert!(HttpRequest::new(long_url).is_none());
    }

    #[test]
    fn test_full_url_encoding() {
        let req = HttpRequest::new("https://example.com").unwrap();
        let req = req.param("q", "hello world").param("filter", "a&b=c");

        let full = req.full_url();
        assert!(full.contains("q=hello+world"));
        assert!(full.contains("filter=a%26b%3Dc"));
    }

    #[test]
    fn test_full_url_no_params() {
        let req = HttpRequest::new("https://example.com/path").unwrap();
        assert_eq!(req.full_url(), "https://example.com/path");
    }
}
