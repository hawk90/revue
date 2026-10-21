//! HTTP Client widget: construction, builders and getters

use super::response::HttpResponse;
use super::types::{HttpColors, HttpMethod, RequestState, ResponseView};
use super::{HttpClient, HttpRequest};
use crate::utils::ScrollState;

use crate::widget::traits::WidgetProps;

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
            scroll: ScrollState::new(),
            history: Vec::new(),
            history_index: 0,
            show_headers: false,
            props: WidgetProps::new(),
        }
    }

    /// Set URL with validation
    ///
    /// Invalid URLs (wrong scheme or too long) will set an error state
    /// instead of panicking. Use `error()` to check for validation errors.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        let url_str = url.into();
        if let Some(valid_req) = HttpRequest::new(&url_str) {
            self.request.url = valid_req.url;
        } else {
            self.reject_url();
        }
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
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}
