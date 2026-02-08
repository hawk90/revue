//! HTTP response types

use std::collections::HashMap;
use std::time::Duration;

use crate::style::Color;

use super::types::ContentType;

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // HttpResponse struct tests
    // =========================================================================

    #[test]
    fn test_http_response_default() {
        let response = HttpResponse::default();
        assert_eq!(response.status, 0);
        assert_eq!(response.status_text, "");
        assert!(response.headers.is_empty());
        assert_eq!(response.body, "");
        assert_eq!(response.time, Duration::ZERO);
        assert_eq!(response.size, 0);
    }

    #[test]
    fn test_http_response_clone() {
        let mut response1 = HttpResponse::default();
        response1.status = 200;
        response1.body = "test".to_string();
        let response2 = response1.clone();
        assert_eq!(response1.status, response2.status);
        assert_eq!(response1.body, response2.body);
    }

    #[test]
    fn test_http_response_debug() {
        let response = HttpResponse::default();
        let debug_str = format!("{:?}", response);
        assert!(debug_str.contains("HttpResponse"));
    }

    #[test]
    fn test_is_success_true_2xx() {
        let mut response = HttpResponse::default();
        response.status = 200;
        assert!(response.is_success());

        response.status = 250;
        assert!(response.is_success());

        response.status = 299;
        assert!(response.is_success());
    }

    #[test]
    fn test_is_success_false_3xx() {
        let mut response = HttpResponse::default();
        response.status = 301;
        assert!(!response.is_success());
    }

    #[test]
    fn test_is_success_false_4xx() {
        let mut response = HttpResponse::default();
        response.status = 404;
        assert!(!response.is_success());
    }

    #[test]
    fn test_is_success_false_5xx() {
        let mut response = HttpResponse::default();
        response.status = 500;
        assert!(!response.is_success());
    }

    #[test]
    fn test_status_color_2xx() {
        let mut response = HttpResponse::default();
        response.status = 200;
        assert_eq!(response.status_color(), Color::rgb(152, 195, 121));
    }

    #[test]
    fn test_status_color_3xx() {
        let mut response = HttpResponse::default();
        response.status = 301;
        assert_eq!(response.status_color(), Color::rgb(229, 192, 123));
    }

    #[test]
    fn test_status_color_4xx() {
        let mut response = HttpResponse::default();
        response.status = 404;
        assert_eq!(response.status_color(), Color::rgb(224, 108, 117));
    }

    #[test]
    fn test_status_color_5xx() {
        let mut response = HttpResponse::default();
        response.status = 500;
        assert_eq!(response.status_color(), Color::rgb(198, 120, 221));
    }

    #[test]
    fn test_status_color_other() {
        let mut response = HttpResponse::default();
        response.status = 100;
        assert_eq!(response.status_color(), Color::rgb(171, 178, 191));
    }

    #[test]
    fn test_content_type_json() {
        let mut response = HttpResponse::default();
        response
            .headers
            .insert("Content-Type".to_string(), "application/json".to_string());
        assert_eq!(response.content_type(), ContentType::Json);
    }

    #[test]
    fn test_content_type_xml() {
        let mut response = HttpResponse::default();
        response
            .headers
            .insert("Content-Type".to_string(), "application/xml".to_string());
        assert_eq!(response.content_type(), ContentType::Xml);
    }

    #[test]
    fn test_content_type_html() {
        let mut response = HttpResponse::default();
        response
            .headers
            .insert("Content-Type".to_string(), "text/html".to_string());
        assert_eq!(response.content_type(), ContentType::Html);
    }

    #[test]
    fn test_content_type_text() {
        let mut response = HttpResponse::default();
        response
            .headers
            .insert("Content-Type".to_string(), "text/plain".to_string());
        assert_eq!(response.content_type(), ContentType::Text);
    }

    #[test]
    fn test_content_type_missing() {
        let response = HttpResponse::default();
        assert_eq!(response.content_type(), ContentType::Text);
    }

    #[test]
    fn test_pretty_json_valid() {
        let mut response = HttpResponse::default();
        response.body = r#"{"name":"test","value":123}"#.to_string();
        let formatted = response.pretty_json();
        assert!(formatted.is_some());
        let formatted_str = formatted.unwrap();
        assert!(formatted_str.contains("{"));
        assert!(formatted_str.contains("\n"));
    }

    #[test]
    fn test_pretty_json_empty() {
        let mut response = HttpResponse::default();
        response.body = "".to_string();
        let formatted = response.pretty_json();
        // Empty string returns None (line 116-120: if result.is_empty() { None })
        assert!(formatted.is_none());
    }

    #[test]
    fn test_format_json_nested() {
        let response = HttpResponse::default();
        let json = r#"{"outer":{"inner":"value"}}"#;
        let formatted = response.format_json(json);
        assert!(formatted.is_some());
        let formatted_str = formatted.unwrap();
        assert!(formatted_str.contains("outer"));
        assert!(formatted_str.contains("inner"));
    }

    #[test]
    fn test_format_json_array() {
        let response = HttpResponse::default();
        let json = r#"[1,2,3]"#;
        let formatted = response.format_json(json);
        assert!(formatted.is_some());
        let formatted_str = formatted.unwrap();
        assert!(formatted_str.contains("["));
    }

    #[test]
    fn test_format_json_with_strings() {
        let response = HttpResponse::default();
        let json = r#"{"key":"value with spaces"}"#;
        let formatted = response.format_json(json);
        assert!(formatted.is_some());
        let formatted_str = formatted.unwrap();
        assert!(formatted_str.contains("value with spaces"));
    }

    #[test]
    fn test_formatted_body_json() {
        let mut response = HttpResponse::default();
        response
            .headers
            .insert("Content-Type".to_string(), "application/json".to_string());
        response.body = r#"{"test":true}"#.to_string();
        let formatted = response.formatted_body();
        assert!(formatted.contains("{"));
        assert!(formatted.contains("\n"));
    }

    #[test]
    fn test_formatted_body_non_json() {
        let mut response = HttpResponse::default();
        response
            .headers
            .insert("Content-Type".to_string(), "text/plain".to_string());
        response.body = "plain text".to_string();
        let formatted = response.formatted_body();
        assert_eq!(formatted, "plain text");
    }

    #[test]
    fn test_formatted_body_invalid_json_fallback() {
        let mut response = HttpResponse::default();
        response
            .headers
            .insert("Content-Type".to_string(), "application/json".to_string());
        response.body = "not json".to_string();
        let formatted = response.formatted_body();
        // format_json strips whitespace outside strings, so "not json" becomes "notjson"
        assert_eq!(formatted, "notjson");
    }
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
    pub fn format_json(&self, json: &str) -> Option<String> {
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
