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
