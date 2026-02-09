//! HTTP client types

use crate::style::Color;

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // HttpMethod enum tests
    // =========================================================================

    #[test]
    fn test_http_method_default() {
        assert_eq!(HttpMethod::default(), HttpMethod::GET);
    }

    #[test]
    fn test_http_method_clone() {
        let method = HttpMethod::POST;
        assert_eq!(method, method.clone());
    }

    #[test]
    fn test_http_method_copy() {
        let method1 = HttpMethod::GET;
        let method2 = method1;
        assert_eq!(method1, HttpMethod::GET);
        assert_eq!(method2, HttpMethod::GET);
    }

    #[test]
    fn test_http_method_equality() {
        assert_eq!(HttpMethod::GET, HttpMethod::GET);
        assert_eq!(HttpMethod::POST, HttpMethod::POST);
        assert_ne!(HttpMethod::GET, HttpMethod::POST);
    }

    #[test]
    fn test_http_method_debug() {
        let debug_str = format!("{:?}", HttpMethod::GET);
        assert!(debug_str.contains("GET"));
    }

    #[test]
    fn test_http_method_name_get() {
        assert_eq!(HttpMethod::GET.name(), "GET");
    }

    #[test]
    fn test_http_method_name_post() {
        assert_eq!(HttpMethod::POST.name(), "POST");
    }

    #[test]
    fn test_http_method_name_put() {
        assert_eq!(HttpMethod::PUT.name(), "PUT");
    }

    #[test]
    fn test_http_method_name_delete() {
        assert_eq!(HttpMethod::DELETE.name(), "DELETE");
    }

    #[test]
    fn test_http_method_name_patch() {
        assert_eq!(HttpMethod::PATCH.name(), "PATCH");
    }

    #[test]
    fn test_http_method_name_head() {
        assert_eq!(HttpMethod::HEAD.name(), "HEAD");
    }

    #[test]
    fn test_http_method_name_options() {
        assert_eq!(HttpMethod::OPTIONS.name(), "OPTIONS");
    }

    #[test]
    fn test_http_method_color_get() {
        let color = HttpMethod::GET.color();
        assert_eq!(color, Color::rgb(97, 175, 239));
    }

    #[test]
    fn test_http_method_color_post() {
        let color = HttpMethod::POST.color();
        assert_eq!(color, Color::rgb(152, 195, 121));
    }

    #[test]
    fn test_http_method_color_put() {
        let color = HttpMethod::PUT.color();
        assert_eq!(color, Color::rgb(229, 192, 123));
    }

    #[test]
    fn test_http_method_color_delete() {
        let color = HttpMethod::DELETE.color();
        assert_eq!(color, Color::rgb(224, 108, 117));
    }

    #[test]
    fn test_http_method_color_patch() {
        let color = HttpMethod::PATCH.color();
        assert_eq!(color, Color::rgb(198, 120, 221));
    }

    #[test]
    fn test_http_method_color_head() {
        let color = HttpMethod::HEAD.color();
        assert_eq!(color, Color::rgb(86, 182, 194));
    }

    #[test]
    fn test_http_method_color_options() {
        let color = HttpMethod::OPTIONS.color();
        assert_eq!(color, Color::rgb(171, 178, 191));
    }

    // =========================================================================
    // RequestState enum tests
    // =========================================================================

    #[test]
    fn test_request_state_default() {
        assert_eq!(RequestState::default(), RequestState::Idle);
    }

    #[test]
    fn test_request_state_clone() {
        let state = RequestState::Sending;
        assert_eq!(state, state.clone());
    }

    #[test]
    fn test_request_state_copy() {
        let state1 = RequestState::Success;
        let state2 = state1;
        assert_eq!(state1, RequestState::Success);
        assert_eq!(state2, RequestState::Success);
    }

    #[test]
    fn test_request_state_equality() {
        assert_eq!(RequestState::Idle, RequestState::Idle);
        assert_eq!(RequestState::Success, RequestState::Success);
        assert_ne!(RequestState::Idle, RequestState::Error);
    }

    #[test]
    fn test_request_state_debug() {
        let debug_str = format!("{:?}", RequestState::Error);
        assert!(debug_str.contains("Error"));
    }

    // =========================================================================
    // ContentType enum tests
    // =========================================================================

    #[test]
    fn test_content_type_default() {
        assert_eq!(ContentType::default(), ContentType::Text);
    }

    #[test]
    fn test_content_type_clone() {
        let ct = ContentType::Json;
        assert_eq!(ct, ct.clone());
    }

    #[test]
    fn test_content_type_copy() {
        let ct1 = ContentType::Xml;
        let ct2 = ct1;
        assert_eq!(ct1, ContentType::Xml);
        assert_eq!(ct2, ContentType::Xml);
    }

    #[test]
    fn test_content_type_equality() {
        assert_eq!(ContentType::Json, ContentType::Json);
        assert_eq!(ContentType::Html, ContentType::Html);
        assert_ne!(ContentType::Json, ContentType::Xml);
    }

    #[test]
    fn test_content_type_debug() {
        let debug_str = format!("{:?}", ContentType::Binary);
        assert!(debug_str.contains("Binary"));
    }

    #[test]
    fn test_content_type_from_header_json() {
        let ct = ContentType::from_header(Some("application/json"));
        assert_eq!(ct, ContentType::Json);
    }

    #[test]
    fn test_content_type_from_header_json_text() {
        let ct = ContentType::from_header(Some("text/json"));
        assert_eq!(ct, ContentType::Json);
    }

    #[test]
    fn test_content_type_from_header_xml() {
        let ct = ContentType::from_header(Some("application/xml"));
        assert_eq!(ct, ContentType::Xml);
    }

    #[test]
    fn test_content_type_from_header_xml_text() {
        let ct = ContentType::from_header(Some("text/xml"));
        assert_eq!(ct, ContentType::Xml);
    }

    #[test]
    fn test_content_type_from_header_html() {
        let ct = ContentType::from_header(Some("text/html"));
        assert_eq!(ct, ContentType::Html);
    }

    #[test]
    fn test_content_type_from_header_text() {
        let ct = ContentType::from_header(Some("text/plain"));
        assert_eq!(ct, ContentType::Text);
    }

    #[test]
    fn test_content_type_from_header_binary() {
        let ct = ContentType::from_header(Some("application/octet-stream"));
        assert_eq!(ct, ContentType::Binary);
    }

    #[test]
    fn test_content_type_from_header_none() {
        let ct = ContentType::from_header(None);
        assert_eq!(ct, ContentType::Text);
    }

    #[test]
    fn test_content_type_from_header_unknown() {
        let ct = ContentType::from_header(Some("application/unknown"));
        assert_eq!(ct, ContentType::Text);
    }

    // =========================================================================
    // ResponseView enum tests
    // =========================================================================

    #[test]
    fn test_response_view_default() {
        assert_eq!(ResponseView::default(), ResponseView::Body);
    }

    #[test]
    fn test_response_view_clone() {
        let view = ResponseView::Headers;
        assert_eq!(view, view.clone());
    }

    #[test]
    fn test_response_view_copy() {
        let view1 = ResponseView::Raw;
        let view2 = view1;
        assert_eq!(view1, ResponseView::Raw);
        assert_eq!(view2, ResponseView::Raw);
    }

    #[test]
    fn test_response_view_equality() {
        assert_eq!(ResponseView::Body, ResponseView::Body);
        assert_eq!(ResponseView::Headers, ResponseView::Headers);
        assert_ne!(ResponseView::Body, ResponseView::Raw);
    }

    #[test]
    fn test_response_view_debug() {
        let debug_str = format!("{:?}", ResponseView::Headers);
        assert!(debug_str.contains("Headers"));
    }

    // =========================================================================
    // HttpColors tests
    // =========================================================================

    #[test]
    fn test_http_colors_default() {
        let colors = HttpColors::default();
        assert_eq!(colors.url_bg, Color::rgb(30, 30, 40));
        assert_eq!(colors.method_bg, Color::rgb(40, 40, 60));
        assert_eq!(colors.header_key, Color::rgb(97, 175, 239));
        assert_eq!(colors.header_value, Color::rgb(171, 178, 191));
        assert_eq!(colors.tab_bg, Color::rgb(40, 40, 50));
        assert_eq!(colors.tab_active, Color::rgb(60, 60, 80));
    }

    #[test]
    fn test_http_colors_clone() {
        let colors1 = HttpColors::default();
        let colors2 = colors1.clone();
        assert_eq!(colors1.url_bg, colors2.url_bg);
        assert_eq!(colors1.tab_active, colors2.tab_active);
    }

    #[test]
    fn test_http_colors_debug() {
        let colors = HttpColors::default();
        let debug_str = format!("{:?}", colors);
        assert!(debug_str.contains("HttpColors"));
    }
}

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
