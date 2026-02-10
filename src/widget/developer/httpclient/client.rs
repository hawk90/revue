//! HTTP Client widget

use std::time::{Duration, Instant};

use super::request::HttpRequest;
use super::response::HttpResponse;
use super::types::{HttpColors, HttpMethod, RequestState, ResponseView};

use crate::utils::format_size_compact;
use crate::widget::traits::WidgetProps;

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

    // Test helper methods (doc hidden)

    /// Get colors for testing (doc hidden)
    #[doc(hidden)]
    pub fn colors_for_testing(&self) -> &HttpColors {
        &self.colors
    }

    /// Get view for testing (doc hidden)
    #[doc(hidden)]
    pub fn view_for_testing(&self) -> ResponseView {
        self.view
    }

    /// Get show_headers for testing (doc hidden)
    #[doc(hidden)]
    pub fn show_headers_for_testing(&self) -> bool {
        self.show_headers
    }

    /// Get url_cursor for testing (doc hidden)
    #[doc(hidden)]
    pub fn url_cursor_for_testing(&self) -> usize {
        self.url_cursor
    }

    /// Get body_scroll for testing (doc hidden)
    #[doc(hidden)]
    pub fn body_scroll_for_testing(&self) -> usize {
        self.body_scroll
    }

    /// Set body_scroll for testing (doc hidden)
    #[doc(hidden)]
    pub fn set_body_scroll_for_testing(&mut self, value: usize) {
        self.body_scroll = value;
    }

    /// Get history for testing (doc hidden)
    #[doc(hidden)]
    pub fn history_for_testing(&self) -> &[HttpRequest] {
        &self.history
    }

    /// Get history_index for testing (doc hidden)
    #[doc(hidden)]
    pub fn history_index_for_testing(&self) -> usize {
        self.history_index
    }

    /// Add current request to history for testing (doc hidden)
    #[doc(hidden)]
    pub fn add_to_history_for_testing(&mut self) {
        self.history.push(self.request.clone());
        self.history_index = self.history.len();
    }

    /// Add default request to history for testing (doc hidden)
    #[doc(hidden)]
    pub fn add_default_to_history_for_testing(&mut self) {
        self.history.push(HttpRequest::default());
        self.history_index = 1;
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
