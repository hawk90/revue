//! HTTP backend trait and mock implementation

use std::time::Duration;

use super::request::HttpRequest;
use super::response::HttpResponse;

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // MockHttpBackend tests
    // =========================================================================

    #[test]
    fn test_mock_http_backend_new() {
        let backend = MockHttpBackend::new();
        assert!(backend.responses.read().is_ok());
        assert!(backend.responses.read().unwrap().is_empty());
    }

    #[test]
    fn test_mock_http_backend_default() {
        let backend = MockHttpBackend::default();
        assert!(backend.responses.read().unwrap().is_empty());
    }

    #[test]
    fn test_mock_http_backend_send_no_mock() {
        let backend = MockHttpBackend::new();
        let request = HttpRequest::new("https://example.com");
        let result = backend.send(&request);
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status, 200);
        assert!(response.body.contains("mock"));
    }

    #[test]
    fn test_mock_http_backend_send_with_wildcard() {
        let backend = MockHttpBackend::new();
        backend.mock_response(
            "*",
            HttpResponse {
                status: 200,
                status_text: "OK".to_string(),
                headers: std::collections::HashMap::new(),
                body: "wildcard response".to_string(),
                time: Duration::from_millis(10),
                size: 17,
            },
        );

        let request = HttpRequest::new("https://any-url.com");
        let result = backend.send(&request);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().body, "wildcard response");
    }

    #[test]
    fn test_mock_http_backend_send_url_pattern() {
        let backend = MockHttpBackend::new();
        backend.mock_response(
            "api",
            HttpResponse {
                status: 201,
                status_text: "Created".to_string(),
                headers: std::collections::HashMap::new(),
                body: "created".to_string(),
                time: Duration::from_millis(5),
                size: 7,
            },
        );

        let request = HttpRequest::new("https://example.com/api/users");
        let result = backend.send(&request);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 201);
    }

    #[test]
    fn test_mock_http_backend_mock_response() {
        let backend = MockHttpBackend::new();
        let response = HttpResponse {
            status: 200,
            status_text: "OK".to_string(),
            headers: std::collections::HashMap::new(),
            body: "test body".to_string(),
            time: Duration::from_millis(50),
            size: 9,
        };
        backend.mock_response("test", response.clone());

        let request = HttpRequest::new("https://example.com/test");
        let result = backend.send(&request);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().body, "test body");
    }

    #[test]
    fn test_mock_http_backend_mock_json() {
        let backend = MockHttpBackend::new();
        backend.mock_json("api/users", 200, r#"{"name":"test"}"#);

        let request = HttpRequest::new("https://example.com/api/users");
        let result = backend.send(&request);
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status, 200);
        assert_eq!(response.status_text, "OK");
        assert!(response.body.contains("test"));
        assert_eq!(
            response.headers.get("Content-Type"),
            Some(&"application/json".to_string())
        );
    }

    #[test]
    fn test_mock_http_backend_mock_error() {
        let backend = MockHttpBackend::new();
        backend.mock_error("api/error", 404, "Not found");

        let request = HttpRequest::new("https://example.com/api/error");
        let result = backend.send(&request);
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status, 404);
        assert!(response.body.contains("Not found"));
    }

    #[test]
    fn test_mock_http_backend_multiple_mocks() {
        let backend = MockHttpBackend::new();
        backend.mock_response(
            "api1",
            HttpResponse {
                status: 200,
                status_text: "OK".to_string(),
                headers: std::collections::HashMap::new(),
                body: "response1".to_string(),
                time: Duration::from_millis(10),
                size: 9,
            },
        );
        backend.mock_response(
            "api2",
            HttpResponse {
                status: 201,
                status_text: "Created".to_string(),
                headers: std::collections::HashMap::new(),
                body: "response2".to_string(),
                time: Duration::from_millis(10),
                size: 9,
            },
        );

        let request1 = HttpRequest::new("https://example.com/api1");
        let result1 = backend.send(&request1);
        assert_eq!(result1.unwrap().body, "response1");

        let request2 = HttpRequest::new("https://example.com/api2");
        let result2 = backend.send(&request2);
        assert_eq!(result2.unwrap().body, "response2");
    }

    #[test]
    fn test_mock_http_backend_latest_mock_takes_precedence() {
        let backend = MockHttpBackend::new();
        backend.mock_response(
            "test",
            HttpResponse {
                status: 200,
                status_text: "OK".to_string(),
                headers: std::collections::HashMap::new(),
                body: "first".to_string(),
                time: Duration::from_millis(10),
                size: 5,
            },
        );
        backend.mock_response(
            "test",
            HttpResponse {
                status: 201,
                status_text: "Created".to_string(),
                headers: std::collections::HashMap::new(),
                body: "second".to_string(),
                time: Duration::from_millis(10),
                size: 6,
            },
        );

        let request = HttpRequest::new("https://example.com/test");
        let result = backend.send(&request);
        assert_eq!(result.unwrap().body, "second");
    }

    // =========================================================================
    // HttpBackend trait tests
    // =========================================================================

    #[test]
    fn test_http_backend_trait_send() {
        let backend = MockHttpBackend::new();
        let request = HttpRequest::new("https://example.com");
        // Test that the trait method is callable
        let result = backend.send(&request);
        assert!(result.is_ok());
    }

    #[test]
    fn test_http_backend_send_and_sync() {
        // Test that MockHttpBackend implements Send + Sync
        use std::sync::{Arc, Mutex};
        let backend = Arc::new(Mutex::new(MockHttpBackend::new()));
        let backend_clone = Arc::clone(&backend);

        std::thread::spawn(move || {
            let mut b = backend_clone.lock().unwrap();
            b.mock_response(
                "test",
                HttpResponse {
                    status: 200,
                    status_text: "OK".to_string(),
                    headers: std::collections::HashMap::new(),
                    body: "test".to_string(),
                    time: Duration::from_millis(10),
                    size: 4,
                },
            );
        })
        .join()
        .unwrap();
    }
}

/// Trait for HTTP backend implementations
///
/// Implement this trait to provide actual HTTP functionality.
/// This allows the widget to work with different HTTP libraries.
///
/// # Example
///
/// Implementing a custom HTTP backend:
///
/// ```rust,ignore
/// use revue::widget::{HttpBackend, HttpRequest, HttpResponse};
///
/// struct MyHttpBackend;
///
/// impl HttpBackend for MyHttpBackend {
///     fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
///         // Example using ureq:
///         // let response = ureq::request(&request.method.to_string(), &request.url)
///         //     .send()
///         //     .map_err(|e| e.to_string())?;
///         //
///         // Ok(HttpResponse {
///         //     status: response.status(),
///         //     headers: response.headers_names()
///         //         .map(|name| (name, response.header(&name).unwrap().to_string()))
///         //         .collect(),
///         //     body: response.into_string().map_err(|e| e.to_string())?,
///         // })
///
///         // For mock implementations in tests, see MockHttpBackend
///         Err("Not implemented".to_string())
///     }
/// }
/// ```
pub trait HttpBackend: Send + Sync {
    /// Send an HTTP request and return the response
    fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String>;
}

/// Mock HTTP backend for testing
#[derive(Default)]
pub struct MockHttpBackend {
    responses: std::sync::RwLock<Vec<(String, HttpResponse)>>,
}

impl MockHttpBackend {
    /// Create a new mock backend
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a mock response for a URL pattern
    pub fn mock_response(&self, url_pattern: impl Into<String>, response: HttpResponse) {
        if let Ok(mut responses) = self.responses.write() {
            responses.push((url_pattern.into(), response));
        }
    }

    /// Add a mock JSON response
    pub fn mock_json(&self, url_pattern: impl Into<String>, status: u16, json: impl Into<String>) {
        let body = json.into();
        let response = HttpResponse {
            status,
            status_text: Self::status_text(status).to_string(),
            headers: [("Content-Type".to_string(), "application/json".to_string())]
                .into_iter()
                .collect(),
            body: body.clone(),
            time: Duration::from_millis(50),
            size: body.len(),
        };
        self.mock_response(url_pattern, response);
    }

    /// Add a mock error response
    pub fn mock_error(
        &self,
        url_pattern: impl Into<String>,
        status: u16,
        message: impl Into<String>,
    ) {
        let body = format!(r#"{{"error": "{}"}}"#, message.into());
        let response = HttpResponse {
            status,
            status_text: Self::status_text(status).to_string(),
            headers: [("Content-Type".to_string(), "application/json".to_string())]
                .into_iter()
                .collect(),
            body: body.clone(),
            time: Duration::from_millis(10),
            size: body.len(),
        };
        self.mock_response(url_pattern, response);
    }

    fn status_text(status: u16) -> &'static str {
        match status {
            200 => "OK",
            201 => "Created",
            204 => "No Content",
            301 => "Moved Permanently",
            302 => "Found",
            304 => "Not Modified",
            400 => "Bad Request",
            401 => "Unauthorized",
            403 => "Forbidden",
            404 => "Not Found",
            405 => "Method Not Allowed",
            500 => "Internal Server Error",
            502 => "Bad Gateway",
            503 => "Service Unavailable",
            _ => "Unknown",
        }
    }
}

impl HttpBackend for MockHttpBackend {
    fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
        if let Ok(responses) = self.responses.read() {
            for (pattern, response) in responses.iter().rev() {
                if request.url.contains(pattern) || pattern == "*" {
                    return Ok(response.clone());
                }
            }
        }

        // Default mock response
        let body = r#"{"status": "mock", "message": "No mock configured"}"#;
        Ok(HttpResponse {
            status: 200,
            status_text: "OK".to_string(),
            headers: [("Content-Type".to_string(), "application/json".to_string())]
                .into_iter()
                .collect(),
            body: body.to_string(),
            time: Duration::from_millis(1),
            size: body.len(),
        })
    }
}
