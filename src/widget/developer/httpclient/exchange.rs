//! HTTP Client widget: the request/response exchange and its state

use std::time::Instant;

use super::response::HttpResponse;
use super::types::RequestState;
use super::{HttpClient, HttpRequest};

impl HttpClient {
    /// Update URL with validation
    ///
    /// Invalid URLs will set an error state instead of panicking.
    pub fn set_url(&mut self, url: impl Into<String>) {
        let url_str = url.into();
        if let Some(valid_req) = HttpRequest::new(&url_str) {
            self.request.url = valid_req.url;
            self.url_cursor = self.request.url.len();
        } else {
            self.reject_url();
        }
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
        self.scroll.scroll_to_top();

        self.state = RequestState::Success;
    }

    /// Set mock response (for testing)
    pub fn set_response(&mut self, response: HttpResponse) {
        let is_success = response.is_success();
        self.response = Some(response);
        // A new response is read from its first line
        self.scroll.scroll_to_top();
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
        self.scroll.set_content(0);
    }

    /// Record that a URL was refused: the message names the rules
    /// `HttpRequest::new` checks.
    pub(super) fn reject_url(&mut self) {
        self.error = Some("Invalid URL: only http:// and https:// schemes are allowed, and URL must not exceed 8192 characters".to_string());
        self.state = RequestState::Error;
    }
}
