//! HTTP Client widget

use std::time::{Duration, Instant};

use super::request::HttpRequest;
use super::response::HttpResponse;
use super::types::{HttpColors, HttpMethod, RequestState, ResponseView};

use crate::utils::format_size_compact;
use crate::widget::traits::WidgetProps;

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Constructor tests
    // =========================================================================

    #[test]
    fn test_http_client_new() {
        let client = HttpClient::new();
        assert_eq!(client.request.url, "");
        assert_eq!(client.state, RequestState::Idle);
        assert!(client.response.is_none());
    }

    #[test]
    fn test_http_client_default() {
        let client = HttpClient::default();
        assert_eq!(client.state, RequestState::Idle);
        assert!(client.history.is_empty());
    }

    // =========================================================================
    // Builder tests
    // =========================================================================

    #[test]
    fn test_http_client_url() {
        let client = HttpClient::new().url("https://example.com");
        assert_eq!(client.request.url, "https://example.com");
        assert_eq!(client.url_cursor, 19); // "https://example.com" has 19 characters
    }

    #[test]
    fn test_http_client_url_with_string() {
        let url = String::from("https://test.com");
        let client = HttpClient::new().url(url);
        assert_eq!(client.request.url, "https://test.com");
    }

    #[test]
    fn test_http_client_method() {
        let client = HttpClient::new().method(HttpMethod::POST);
        assert_eq!(client.request.method, HttpMethod::POST);
    }

    #[test]
    fn test_http_client_header() {
        let client = HttpClient::new().header("Accept", "application/json");
        assert_eq!(
            client.request.headers.get("Accept"),
            Some(&"application/json".to_string())
        );
    }

    #[test]
    fn test_http_client_body() {
        let client = HttpClient::new().body("{\"test\":true}");
        assert_eq!(client.request.body, "{\"test\":true}");
    }

    #[test]
    fn test_http_client_colors() {
        let colors = HttpColors::default();
        let client = HttpClient::new().colors(colors.clone());
        assert_eq!(client.colors.tab_active, colors.tab_active);
    }

    // =========================================================================
    // Getter tests
    // =========================================================================

    #[test]
    fn test_http_client_request() {
        let client = HttpClient::new().url("https://example.com");
        assert_eq!(client.request().url, "https://example.com");
    }

    #[test]
    fn test_http_client_request_mut() {
        let mut client = HttpClient::new();
        client.request_mut().url = "https://test.com".to_string();
        assert_eq!(client.request.url, "https://test.com");
    }

    #[test]
    fn test_http_client_response_none() {
        let client = HttpClient::new();
        assert!(client.response().is_none());
    }

    #[test]
    fn test_http_client_response_some() {
        let mut client = HttpClient::new();
        let response = HttpResponse {
            status: 200,
            status_text: "OK".to_string(),
            headers: std::collections::HashMap::new(),
            body: "test".to_string(),
            time: Duration::from_millis(100),
            size: 4,
        };
        client.set_response(response);
        assert!(client.response().is_some());
        assert_eq!(client.response().unwrap().status, 200);
    }

    #[test]
    fn test_http_client_state() {
        let client = HttpClient::new();
        assert_eq!(client.state(), RequestState::Idle);
    }

    #[test]
    fn test_http_client_error_none() {
        let client = HttpClient::new();
        assert!(client.error().is_none());
    }

    #[test]
    fn test_http_client_error_some() {
        let mut client = HttpClient::new();
        client.set_error("Test error");
        assert_eq!(client.error(), Some("Test error"));
    }

    // =========================================================================
    // State changing tests
    // =========================================================================

    #[test]
    fn test_http_client_set_view() {
        let mut client = HttpClient::new();
        client.set_view(ResponseView::Headers);
        assert!(matches!(client.view, ResponseView::Headers));
    }

    #[test]
    fn test_http_client_toggle_headers() {
        let mut client = HttpClient::new();
        let initial = client.show_headers;
        client.toggle_headers();
        assert_ne!(client.show_headers, initial);
    }

    #[test]
    fn test_http_client_set_url() {
        let mut client = HttpClient::new();
        client.set_url("https://example.com");
        assert_eq!(client.request.url, "https://example.com");
        assert_eq!(client.url_cursor, 19); // "https://example.com" has 19 characters
    }

    #[test]
    fn test_http_client_cycle_method() {
        let mut client = HttpClient::new();
        assert_eq!(client.request.method, HttpMethod::GET);
        client.cycle_method();
        assert_eq!(client.request.method, HttpMethod::POST);
        client.cycle_method();
        assert_eq!(client.request.method, HttpMethod::PUT);
    }

    #[test]
    fn test_http_client_cycle_method_full_cycle() {
        let mut client = HttpClient::new();
        let methods = [
            HttpMethod::GET,
            HttpMethod::POST,
            HttpMethod::PUT,
            HttpMethod::DELETE,
            HttpMethod::PATCH,
            HttpMethod::HEAD,
            HttpMethod::OPTIONS,
        ];
        for expected in &methods {
            assert_eq!(client.request.method, *expected);
            client.cycle_method();
        }
        assert_eq!(client.request.method, HttpMethod::GET);
    }

    #[test]
    fn test_http_client_send() {
        let mut client = HttpClient::new().url("https://example.com");
        client.send();
        assert_eq!(client.state, RequestState::Success);
        assert!(client.response.is_some());
        assert_eq!(client.response.unwrap().status, 200);
    }

    #[test]
    fn test_http_client_send_saves_to_history() {
        let mut client = HttpClient::new().url("https://example.com");
        client.send();
        assert_eq!(client.history.len(), 1);
        assert_eq!(client.history_index, 1);
    }

    #[test]
    fn test_http_client_set_response_success() {
        let mut client = HttpClient::new();
        let response = HttpResponse {
            status: 200,
            status_text: "OK".to_string(),
            headers: std::collections::HashMap::new(),
            body: "success".to_string(),
            time: Duration::from_millis(50),
            size: 7,
        };
        client.set_response(response);
        assert_eq!(client.state, RequestState::Success);
    }

    #[test]
    fn test_http_client_set_response_error() {
        let mut client = HttpClient::new();
        let response = HttpResponse {
            status: 404,
            status_text: "Not Found".to_string(),
            headers: std::collections::HashMap::new(),
            body: "error".to_string(),
            time: Duration::from_millis(10),
            size: 5,
        };
        client.set_response(response);
        assert_eq!(client.state, RequestState::Error);
    }

    #[test]
    fn test_http_client_set_error() {
        let mut client = HttpClient::new();
        client.set_error("Connection failed");
        assert_eq!(client.state, RequestState::Error);
        assert_eq!(client.error, Some("Connection failed".to_string()));
    }

    #[test]
    fn test_http_client_clear() {
        let mut client = HttpClient::new();
        client.set_error("test");
        client.body_scroll = 10;
        client.clear();
        assert_eq!(client.state, RequestState::Idle);
        assert!(client.response.is_none());
        assert!(client.error.is_none());
        assert_eq!(client.body_scroll, 0);
    }

    // =========================================================================
    // Scroll tests
    // =========================================================================

    #[test]
    fn test_http_client_scroll_down() {
        let mut client = HttpClient::new();
        client.scroll_down(10);
        assert_eq!(client.body_scroll, 10);
        client.scroll_down(5);
        assert_eq!(client.body_scroll, 15);
    }

    #[test]
    fn test_http_client_scroll_up() {
        let mut client = HttpClient::new();
        client.body_scroll = 20;
        client.scroll_up(5);
        assert_eq!(client.body_scroll, 15);
    }

    #[test]
    fn test_http_client_scroll_up_clamps_to_zero() {
        let mut client = HttpClient::new();
        client.body_scroll = 5;
        client.scroll_up(10);
        assert_eq!(client.body_scroll, 0);
    }

    // =========================================================================
    // History tests
    // =========================================================================

    #[test]
    fn test_http_client_history_back() {
        let mut client = HttpClient::new();
        client.set_url("url1");
        client.history.push(client.request.clone());
        client.history_index = 1;
        client.set_url("url2");
        client.history.push(client.request.clone());
        client.history_index = 2;

        client.history_back();
        assert_eq!(client.history_index, 1);
        assert_eq!(client.request.url, "url2"); // history[1] contains "url2"
    }

    #[test]
    fn test_http_client_history_back_at_start() {
        let mut client = HttpClient::new();
        client.history_back();
        assert_eq!(client.history_index, 0);
    }

    #[test]
    fn test_http_client_history_forward() {
        let mut client = HttpClient::new();
        client.set_url("url1");
        client.history.push(client.request.clone());
        client.history_index = 0;

        client.history_forward();
        assert_eq!(client.history_index, 1);
    }

    #[test]
    fn test_http_client_history_forward_at_end() {
        let mut client = HttpClient::new();
        client.history.push(HttpRequest::default());
        client.history_index = 1;

        client.history_forward();
        assert_eq!(client.history_index, 1); // Stays at 1 (end of history)
        assert!(client.request.url.is_empty());
    }
}

/// HTTP Client widget
///
/// # Example
///
/// ```rust,ignore
/// use revue::prelude::*;
///
/// let mut client = HttpClient::new()
///     .url("https://api.example.com/users")
///     .method(HttpMethod::GET);
///
/// // Send request (async)
/// client.send();
///
/// // Check response
/// if let Some(response) = client.response() {
///     println!("Status: {}", response.status);
/// }
/// ```
pub struct HttpClient {
    /// Current request
    pub(super) request: HttpRequest,
    /// Last response
    pub(super) response: Option<HttpResponse>,
    /// Request state
    pub(super) state: RequestState,
    /// Error message
    pub(super) error: Option<String>,
    /// Response view mode
    pub(super) view: ResponseView,
    /// Colors
    pub(super) colors: HttpColors,
    /// URL cursor position
    pub(super) url_cursor: usize,
    /// Body scroll
    pub(super) body_scroll: usize,
    /// Request history
    pub(super) history: Vec<HttpRequest>,
    /// History index
    pub(super) history_index: usize,
    /// Show headers panel
    pub(super) show_headers: bool,
    /// Widget properties
    pub props: WidgetProps,
}

impl HttpClient {
    /// Create a new HTTP client
    pub fn new() -> Self {
        Self {
            request: HttpRequest::default(),
            response: None,
            state: RequestState::Idle,
            error: None,
            view: ResponseView::Body,
            colors: HttpColors::default(),
            url_cursor: 0,
            body_scroll: 0,
            history: Vec::new(),
            history_index: 0,
            show_headers: false,
            props: WidgetProps::new(),
        }
    }

    /// Set URL
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.request.url = url.into();
        self.url_cursor = self.request.url.len();
        self
    }

    /// Set method
    pub fn method(mut self, method: HttpMethod) -> Self {
        self.request.method = method;
        self
    }

    /// Add header
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.request.headers.insert(key.into(), value.into());
        self
    }

    /// Set body
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.request.body = body.into();
        self
    }

    /// Set colors
    pub fn colors(mut self, colors: HttpColors) -> Self {
        self.colors = colors;
        self
    }

    /// Get current request
    pub fn request(&self) -> &HttpRequest {
        &self.request
    }

    /// Get mutable request
    pub fn request_mut(&mut self) -> &mut HttpRequest {
        &mut self.request
    }

    /// Get response
    pub fn response(&self) -> Option<&HttpResponse> {
        self.response.as_ref()
    }

    /// Get state
    pub fn state(&self) -> RequestState {
        self.state
    }

    /// Get error
    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    /// Set response view
    pub fn set_view(&mut self, view: ResponseView) {
        self.view = view;
    }

    /// Toggle headers panel
    pub fn toggle_headers(&mut self) {
        self.show_headers = !self.show_headers;
    }

    /// Update URL
    pub fn set_url(&mut self, url: impl Into<String>) {
        self.request.url = url.into();
        self.url_cursor = self.request.url.len();
    }

    /// Cycle method
    pub fn cycle_method(&mut self) {
        self.request.method = match self.request.method {
            HttpMethod::GET => HttpMethod::POST,
            HttpMethod::POST => HttpMethod::PUT,
            HttpMethod::PUT => HttpMethod::DELETE,
            HttpMethod::DELETE => HttpMethod::PATCH,
            HttpMethod::PATCH => HttpMethod::HEAD,
            HttpMethod::HEAD => HttpMethod::OPTIONS,
            HttpMethod::OPTIONS => HttpMethod::GET,
        };
    }

    /// Send request (mock implementation - real impl requires async)
    pub fn send(&mut self) {
        self.state = RequestState::Sending;
        self.error = None;

        // Save to history
        self.history.push(self.request.clone());
        self.history_index = self.history.len();

        // Mock response for now (real implementation would use reqwest)
        let start = Instant::now();

        // Simulate response
        let mock_body = r#"{
  "status": "success",
  "message": "Request received",
  "timestamp": "2024-01-01T00:00:00Z"
}"#;

        self.response = Some(HttpResponse {
            status: 200,
            status_text: "OK".to_string(),
            headers: [
                ("Content-Type".to_string(), "application/json".to_string()),
                ("Content-Length".to_string(), mock_body.len().to_string()),
            ]
            .into_iter()
            .collect(),
            body: mock_body.to_string(),
            time: start.elapsed(),
            size: mock_body.len(),
        });

        self.state = RequestState::Success;
    }

    /// Set mock response (for testing)
    pub fn set_response(&mut self, response: HttpResponse) {
        let is_success = response.is_success();
        self.response = Some(response);
        self.state = if is_success {
            RequestState::Success
        } else {
            RequestState::Error
        };
    }

    /// Set error
    pub fn set_error(&mut self, error: impl Into<String>) {
        self.error = Some(error.into());
        self.state = RequestState::Error;
    }

    /// Clear response
    pub fn clear(&mut self) {
        self.response = None;
        self.error = None;
        self.state = RequestState::Idle;
        self.body_scroll = 0;
    }

    /// Scroll body down
    pub fn scroll_down(&mut self, amount: usize) {
        self.body_scroll = self.body_scroll.saturating_add(amount);
    }

    /// Scroll body up
    pub fn scroll_up(&mut self, amount: usize) {
        self.body_scroll = self.body_scroll.saturating_sub(amount);
    }

    /// Navigate history back
    pub fn history_back(&mut self) {
        if self.history_index > 0 {
            self.history_index -= 1;
            if let Some(req) = self.history.get(self.history_index) {
                self.request = req.clone();
            }
        }
    }

    /// Navigate history forward
    pub fn history_forward(&mut self) {
        if self.history_index < self.history.len() {
            self.history_index += 1;
            if let Some(req) = self.history.get(self.history_index) {
                self.request = req.clone();
            }
        }
    }

    /// Format duration
    pub(super) fn format_duration(d: Duration) -> String {
        let ms = d.as_millis();
        if ms < 1000 {
            format!("{}ms", ms)
        } else {
            format!("{:.2}s", d.as_secs_f64())
        }
    }

    /// Format size
    pub(super) fn format_size(bytes: usize) -> String {
        format_size_compact(bytes as u64)
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}
