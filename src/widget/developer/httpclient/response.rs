//! HTTP response types

use std::collections::HashMap;
use std::time::Duration;

use crate::style::Color;

use super::types::ContentType;

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
