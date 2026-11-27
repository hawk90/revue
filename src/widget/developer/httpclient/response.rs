//! HTTP response types

use std::collections::HashMap;
use std::time::Duration;

use crate::style::Color;

use super::types::ContentType;
use crate::widget::data::json_viewer::parser::is_json;

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
        // Header names are case-insensitive (RFC 9110): a real client
        // reports them lowercase.
        let value = self
            .headers
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("content-type"))
            .map(|(_, value)| value.as_str());
        ContentType::from_header(value)
    }

    /// The headers ordered by name, ignoring ASCII case (names that differ
    /// only in case by the name as written): the same order on every run,
    /// which the `HashMap` does not keep (#1083)
    pub(super) fn sorted_headers(&self) -> Vec<(&str, &str)> {
        let mut headers: Vec<(&str, &str)> = self
            .headers
            .iter()
            .map(|(name, value)| (name.as_str(), value.as_str()))
            .collect();
        headers.sort_by(|(a, _), (b, _)| {
            a.to_ascii_lowercase()
                .cmp(&b.to_ascii_lowercase())
                .then_with(|| a.cmp(b))
        });
        headers
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

        let mut chars = json.chars().peekable();
        while let Some(ch) = chars.next() {
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
                    // An empty container stays on one line: `{}`, `[]`
                    let close = if ch == '{' { '}' } else { ']' };
                    while chars
                        .next_if(|c| matches!(c, ' ' | '\n' | '\r' | '\t'))
                        .is_some()
                    {}
                    if chars.next_if_eq(&close).is_some() {
                        result.push(close);
                        continue;
                    }
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
            // format_json re-indents tokens without validating them, so only
            // hand it a body that parses; anything else is shown as sent.
            ContentType::Json if is_json(&self.body) => {
                self.pretty_json().unwrap_or_else(|| self.body.clone())
            }
            _ => self.body.clone(),
        }
    }
}
