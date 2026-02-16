//! HTTP client types

use crate::style::Color;

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
