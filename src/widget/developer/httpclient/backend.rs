//! HTTP backend trait and mock implementation

use std::time::Duration;

use super::request::HttpRequest;
use super::response::HttpResponse;

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

    /// Get responses for testing (doc hidden)
    #[doc(hidden)]
    #[allow(clippy::type_complexity)]
    pub fn responses_for_testing(
        &self,
    ) -> Result<
        std::sync::RwLockReadGuard<'_, Vec<(String, HttpResponse)>>,
        std::sync::PoisonError<std::sync::RwLockReadGuard<'_, Vec<(String, HttpResponse)>>>,
    > {
        self.responses.read()
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
