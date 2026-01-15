#![allow(clippy::needless_range_loop, clippy::explicit_counter_loop)]
//! HTTP Client widget for REST API testing
//!
//! A Postman-like widget for making HTTP requests and viewing responses.
//!
//! # Features
//!
//! - Multiple HTTP methods (GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS)
//! - Request headers and body
//! - Query parameters with URL builder
//! - Response body with JSON/XML formatting
//! - Loading and error states
//! - Request history navigation
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{HttpClient, HttpMethod};
//!
//! let mut client = HttpClient::new()
//!     .url("https://api.example.com/users")
//!     .method(HttpMethod::GET)
//!     .header("Authorization", "Bearer token");
//!
//! // Send request
//! client.send();
//!
//! // Check response
//! if let Some(response) = client.response() {
//!     println!("Status: {}", response.status);
//!     println!("Body: {}", response.body);
//! }
//! ```

use super::traits::{RenderContext, View, WidgetProps};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::{impl_props_builders, impl_styled_view};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// HTTP method
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HttpMethod {
    /// HTTP GET request
    #[default]
    GET,
    /// HTTP POST request
    POST,
    /// HTTP PUT request
    PUT,
    /// HTTP DELETE request
    DELETE,
    /// HTTP PATCH request
    PATCH,
    /// HTTP HEAD request
    HEAD,
    /// HTTP OPTIONS request
    OPTIONS,
}

impl HttpMethod {
    /// Get method name
    pub fn name(&self) -> &'static str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::HEAD => "HEAD",
            HttpMethod::OPTIONS => "OPTIONS",
        }
    }

    /// Get method color
    pub fn color(&self) -> Color {
        match self {
            HttpMethod::GET => Color::rgb(97, 175, 239),      // Blue
            HttpMethod::POST => Color::rgb(152, 195, 121),    // Green
            HttpMethod::PUT => Color::rgb(229, 192, 123),     // Yellow
            HttpMethod::DELETE => Color::rgb(224, 108, 117),  // Red
            HttpMethod::PATCH => Color::rgb(198, 120, 221),   // Purple
            HttpMethod::HEAD => Color::rgb(86, 182, 194),     // Cyan
            HttpMethod::OPTIONS => Color::rgb(171, 178, 191), // Gray
        }
    }
}

/// Request state
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RequestState {
    /// No request in progress
    #[default]
    Idle,
    /// Request is being sent
    Sending,
    /// Request completed successfully
    Success,
    /// Request failed with error
    Error,
}

/// HTTP response
#[derive(Clone, Debug, Default)]
pub struct HttpResponse {
    /// Status code
    pub status: u16,
    /// Status text
    pub status_text: String,
    /// Response headers
    pub headers: HashMap<String, String>,
    /// Response body
    pub body: String,
    /// Response time
    pub time: Duration,
    /// Response size in bytes
    pub size: usize,
}

/// Content type of response
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ContentType {
    /// JSON content
    Json,
    /// XML content
    Xml,
    /// HTML content
    Html,
    /// Plain text
    #[default]
    Text,
    /// Binary data
    Binary,
}

impl ContentType {
    /// Detect content type from Content-Type header
    pub fn from_header(header: Option<&str>) -> Self {
        match header {
            Some(h) if h.contains("application/json") => Self::Json,
            Some(h) if h.contains("text/json") => Self::Json,
            Some(h) if h.contains("application/xml") => Self::Xml,
            Some(h) if h.contains("text/xml") => Self::Xml,
            Some(h) if h.contains("text/html") => Self::Html,
            Some(h) if h.contains("text/plain") => Self::Text,
            Some(h) if h.contains("application/octet-stream") => Self::Binary,
            _ => Self::Text,
        }
    }
}

impl HttpResponse {
    /// Check if status is success (2xx)
    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status)
    }

    /// Get status color
    pub fn status_color(&self) -> Color {
        match self.status {
            200..=299 => Color::rgb(152, 195, 121), // Green
            300..=399 => Color::rgb(229, 192, 123), // Yellow
            400..=499 => Color::rgb(224, 108, 117), // Red
            500..=599 => Color::rgb(198, 120, 221), // Purple
            _ => Color::rgb(171, 178, 191),         // Gray
        }
    }

    /// Get content type
    pub fn content_type(&self) -> ContentType {
        ContentType::from_header(self.headers.get("Content-Type").map(|s| s.as_str()))
    }

    /// Try to format body as pretty JSON
    pub fn pretty_json(&self) -> Option<String> {
        self.format_json(&self.body)
    }

    /// Format JSON string with indentation
    fn format_json(&self, json: &str) -> Option<String> {
        // Simple JSON formatter without external dependencies
        let mut result = String::new();
        let mut indent = 0usize;
        let mut in_string = false;
        let mut escape_next = false;

        for ch in json.chars() {
            if escape_next {
                result.push(ch);
                escape_next = false;
                continue;
            }

            if ch == '\\' && in_string {
                result.push(ch);
                escape_next = true;
                continue;
            }

            if ch == '"' {
                in_string = !in_string;
                result.push(ch);
                continue;
            }

            if in_string {
                result.push(ch);
                continue;
            }

            match ch {
                '{' | '[' => {
                    result.push(ch);
                    indent += 1;
                    result.push('\n');
                    result.push_str(&"  ".repeat(indent));
                }
                '}' | ']' => {
                    indent = indent.saturating_sub(1);
                    result.push('\n');
                    result.push_str(&"  ".repeat(indent));
                    result.push(ch);
                }
                ',' => {
                    result.push(ch);
                    result.push('\n');
                    result.push_str(&"  ".repeat(indent));
                }
                ':' => {
                    result.push_str(": ");
                }
                ' ' | '\n' | '\r' | '\t' => {
                    // Skip whitespace outside strings
                }
                _ => {
                    result.push(ch);
                }
            }
        }

        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }

    /// Get formatted body based on content type
    pub fn formatted_body(&self) -> String {
        match self.content_type() {
            ContentType::Json => self.pretty_json().unwrap_or_else(|| self.body.clone()),
            _ => self.body.clone(),
        }
    }
}

/// HTTP request builder
#[derive(Clone, Debug, Default)]
pub struct HttpRequest {
    /// HTTP method
    pub method: HttpMethod,
    /// Request URL
    pub url: String,
    /// Request headers
    pub headers: HashMap<String, String>,
    /// Request body
    pub body: String,
    /// Query parameters
    pub params: HashMap<String, String>,
}

impl HttpRequest {
    /// Create a new request
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            ..Default::default()
        }
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
    pub fn full_url(&self) -> String {
        if self.params.is_empty() {
            self.url.clone()
        } else {
            let params: Vec<String> = self
                .params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            format!("{}?{}", self.url, params.join("&"))
        }
    }
}

/// View mode for response
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ResponseView {
    /// Show response body (parsed/formatted)
    #[default]
    Body,
    /// Show response headers
    Headers,
    /// Show raw response
    Raw,
}

/// HTTP Client widget colors
#[derive(Clone, Debug)]
pub struct HttpColors {
    /// URL bar background
    pub url_bg: Color,
    /// Method badge background
    pub method_bg: Color,
    /// Header key color
    pub header_key: Color,
    /// Header value color
    pub header_value: Color,
    /// Tab background
    pub tab_bg: Color,
    /// Active tab background
    pub tab_active: Color,
}

impl Default for HttpColors {
    fn default() -> Self {
        Self {
            url_bg: Color::rgb(30, 30, 40),
            method_bg: Color::rgb(40, 40, 60),
            header_key: Color::rgb(97, 175, 239),
            header_value: Color::rgb(171, 178, 191),
            tab_bg: Color::rgb(40, 40, 50),
            tab_active: Color::rgb(60, 60, 80),
        }
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
    request: HttpRequest,
    /// Last response
    response: Option<HttpResponse>,
    /// Request state
    state: RequestState,
    /// Error message
    error: Option<String>,
    /// Response view mode
    view: ResponseView,
    /// Colors
    colors: HttpColors,
    /// URL cursor position
    url_cursor: usize,
    /// Body scroll
    body_scroll: usize,
    /// Request history
    history: Vec<HttpRequest>,
    /// History index
    history_index: usize,
    /// Show headers panel
    show_headers: bool,
    /// Widget properties
    props: WidgetProps,
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
    fn format_duration(d: Duration) -> String {
        let ms = d.as_millis();
        if ms < 1000 {
            format!("{}ms", ms)
        } else {
            format!("{:.2}s", d.as_secs_f64())
        }
    }

    /// Format size
    fn format_size(bytes: usize) -> String {
        if bytes < 1024 {
            format!("{}B", bytes)
        } else if bytes < 1024 * 1024 {
            format!("{:.1}KB", bytes as f64 / 1024.0)
        } else {
            format!("{:.1}MB", bytes as f64 / (1024.0 * 1024.0))
        }
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

impl View for HttpClient {
    crate::impl_view_meta!("HttpClient");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 40 || area.height < 10 {
            return;
        }

        // URL bar (row 0-1)
        // Method badge
        let method = self.request.method;
        let method_name = method.name();
        for (i, ch) in method_name.chars().enumerate() {
            let mut cell = Cell::new(ch);
            cell.fg = Some(method.color());
            cell.modifier = Modifier::BOLD;
            ctx.buffer.set(area.x + i as u16, area.y, cell);
        }

        // URL
        let url_start = method_name.len() as u16 + 1;
        for (i, ch) in self.request.url.chars().enumerate() {
            if url_start + i as u16 >= area.width - 1 {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(Color::WHITE);
            ctx.buffer.set(area.x + url_start + i as u16, area.y, cell);
        }

        // Send button hint
        let hint = "[Enter: Send]";
        let hint_start = area.width.saturating_sub(hint.len() as u16);
        for (i, ch) in hint.chars().enumerate() {
            let mut cell = Cell::new(ch);
            cell.fg = Some(Color::rgb(100, 100, 100));
            ctx.buffer.set(area.x + hint_start + i as u16, area.y, cell);
        }

        // Separator
        for x in 0..area.width {
            let mut cell = Cell::new('─');
            cell.fg = Some(Color::rgb(60, 60, 60));
            ctx.buffer.set(area.x + x, area.y + 1, cell);
        }

        // Response area (row 2+)
        let response_y = 2u16;

        if self.state == RequestState::Sending {
            let loading = "⠋ Sending request...";
            for (i, ch) in loading.chars().enumerate() {
                if i as u16 >= area.width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::YELLOW);
                ctx.buffer.set(area.x + i as u16, area.y + response_y, cell);
            }
        } else if let Some(error) = &self.error {
            let err_msg = format!("✗ Error: {}", error);
            for (i, ch) in err_msg.chars().enumerate() {
                if i as u16 >= area.width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::RED);
                ctx.buffer.set(area.x + i as u16, area.y + response_y, cell);
            }
        } else if let Some(response) = &self.response {
            // Status line
            let status_line = format!(
                "{} {} • {} • {}",
                response.status,
                response.status_text,
                Self::format_duration(response.time),
                Self::format_size(response.size)
            );

            let mut x = 0u16;
            for ch in status_line.chars() {
                if x >= area.width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(response.status_color());
                ctx.buffer.set(area.x + x, area.y + response_y, cell);
                x += 1;
            }

            // Tabs
            let tabs = ["Body", "Headers", "Raw"];
            let tab_y = response_y + 1;
            let mut tab_x = 0u16;
            for (i, tab) in tabs.iter().enumerate() {
                let is_active = matches!(
                    (i, self.view),
                    (0, ResponseView::Body) | (1, ResponseView::Headers) | (2, ResponseView::Raw)
                );

                for ch in tab.chars() {
                    if tab_x >= area.width {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(if is_active {
                        Color::WHITE
                    } else {
                        Color::rgb(100, 100, 100)
                    });
                    cell.bg = Some(if is_active {
                        self.colors.tab_active
                    } else {
                        self.colors.tab_bg
                    });
                    ctx.buffer.set(area.x + tab_x, area.y + tab_y, cell);
                    tab_x += 1;
                }

                // Space between tabs
                let mut cell = Cell::new(' ');
                cell.bg = Some(self.colors.tab_bg);
                ctx.buffer.set(area.x + tab_x, area.y + tab_y, cell);
                tab_x += 1;
            }

            // Fill rest of tab bar
            for x in tab_x..area.width {
                let mut cell = Cell::new(' ');
                cell.bg = Some(self.colors.tab_bg);
                ctx.buffer.set(area.x + x, area.y + tab_y, cell);
            }

            // Content
            let content_y = tab_y + 1;
            let content_height = area.height.saturating_sub(content_y);

            match self.view {
                ResponseView::Body | ResponseView::Raw => {
                    for (i, line) in response
                        .body
                        .lines()
                        .skip(self.body_scroll)
                        .take(content_height as usize)
                        .enumerate()
                    {
                        for (j, ch) in line.chars().enumerate() {
                            if j as u16 >= area.width {
                                break;
                            }
                            let mut cell = Cell::new(ch);
                            cell.fg = Some(Color::rgb(200, 200, 200));
                            ctx.buffer
                                .set(area.x + j as u16, area.y + content_y + i as u16, cell);
                        }
                    }
                }
                ResponseView::Headers => {
                    for (i, (key, value)) in response
                        .headers
                        .iter()
                        .skip(self.body_scroll)
                        .take(content_height as usize)
                        .enumerate()
                    {
                        let y = area.y + content_y + i as u16;

                        // Key
                        for (j, ch) in key.chars().enumerate() {
                            if j as u16 >= area.width / 2 {
                                break;
                            }
                            let mut cell = Cell::new(ch);
                            cell.fg = Some(self.colors.header_key);
                            ctx.buffer.set(area.x + j as u16, y, cell);
                        }

                        // Colon
                        let colon_x = key.len() as u16;
                        if colon_x + 2 < area.width {
                            let mut cell = Cell::new(':');
                            cell.fg = Some(Color::rgb(100, 100, 100));
                            ctx.buffer.set(area.x + colon_x, y, cell);

                            // Value
                            for (j, ch) in value.chars().enumerate() {
                                if colon_x + 2 + j as u16 >= area.width {
                                    break;
                                }
                                let mut cell = Cell::new(ch);
                                cell.fg = Some(self.colors.header_value);
                                ctx.buffer.set(area.x + colon_x + 2 + j as u16, y, cell);
                            }
                        }
                    }
                }
            }
        } else {
            // No response yet
            let msg = "Enter a URL and press Enter to send request";
            for (i, ch) in msg.chars().enumerate() {
                if i as u16 >= area.width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::rgb(100, 100, 100));
                ctx.buffer.set(area.x + i as u16, area.y + response_y, cell);
            }
        }
    }
}

impl_styled_view!(HttpClient);
impl_props_builders!(HttpClient);

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

// =============================================================================
// Request Builder for Fluent API
// =============================================================================

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
        use std::io::Write;
        let mut encoder = Vec::new();
        let credentials = format!("{}:{}", username.into(), password.into());
        // Simple base64 encoding
        let encoded = base64_encode(credentials.as_bytes());
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
fn base64_encode(data: &[u8]) -> String {
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

// =============================================================================
// HTTP Backend Trait
// =============================================================================

/// Trait for HTTP backend implementations
///
/// Implement this trait to provide actual HTTP functionality.
/// This allows the widget to work with different HTTP libraries.
///
/// # Example
///
/// ```rust,ignore
/// use revue::widget::{HttpBackend, HttpRequest, HttpResponse};
///
/// struct MyHttpBackend;
///
/// impl HttpBackend for MyHttpBackend {
///     fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
///         // Implement using reqwest, ureq, or other HTTP library
///         todo!()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_http_client_creation() {
        let client = HttpClient::new();
        assert_eq!(client.state(), RequestState::Idle);
    }

    #[test]
    fn test_request_builder() {
        let client = HttpClient::new()
            .url("https://api.example.com")
            .method(HttpMethod::POST)
            .header("Content-Type", "application/json")
            .body(r#"{"key": "value"}"#);

        assert_eq!(client.request().method, HttpMethod::POST);
        assert!(!client.request().headers.is_empty());
    }

    #[test]
    fn test_http_request_url() {
        let req = HttpRequest::new("https://api.example.com")
            .param("page", "1")
            .param("limit", "10");

        let url = req.full_url();
        assert!(url.contains("page=1"));
        assert!(url.contains("limit=10"));
    }

    #[test]
    fn test_send_request() {
        let mut client = get("https://httpbin.org/get");
        client.send();

        assert_eq!(client.state(), RequestState::Success);
        assert!(client.response().is_some());
    }

    #[test]
    fn test_render() {
        let client = get("https://example.com");

        let mut buffer = Buffer::new(80, 20);
        let area = Rect::new(0, 0, 80, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        client.render(&mut ctx);
    }

    #[test]
    fn test_method_colors() {
        assert_ne!(HttpMethod::GET.color(), HttpMethod::POST.color());
        assert_ne!(HttpMethod::DELETE.color(), HttpMethod::PUT.color());
    }
}
