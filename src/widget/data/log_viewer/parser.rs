//! Log parsing functionality for advanced log viewer

#![allow(clippy::iter_skip_next)]
use super::entry::LogEntry;
use super::types::LogLevel;

/// Log parsing configuration
#[derive(Clone, Debug)]
pub struct LogParser {
    /// Enable JSON log parsing
    pub json_parsing: bool,
    /// Timestamp regex pattern
    pub timestamp_pattern: Option<String>,
    /// Level regex pattern
    pub level_pattern: Option<String>,
    /// Source regex pattern
    pub source_pattern: Option<String>,
    /// Message regex pattern
    pub message_pattern: Option<String>,
    /// JSON level field name
    pub json_level_field: String,
    /// JSON message field name
    pub json_message_field: String,
    /// JSON timestamp field name
    pub json_timestamp_field: String,
    /// JSON source field name
    pub json_source_field: String,
}

impl Default for LogParser {
    fn default() -> Self {
        Self {
            json_parsing: true,
            timestamp_pattern: None,
            level_pattern: None,
            source_pattern: None,
            message_pattern: None,
            json_level_field: "level".to_string(),
            json_message_field: "msg".to_string(),
            json_timestamp_field: "time".to_string(),
            json_source_field: "source".to_string(),
        }
    }
}

impl LogParser {
    /// Create a new parser with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable/disable JSON parsing
    pub fn json_parsing(mut self, enable: bool) -> Self {
        self.json_parsing = enable;
        self
    }

    /// Set JSON field names
    pub fn json_fields(
        mut self,
        level: impl Into<String>,
        message: impl Into<String>,
        timestamp: impl Into<String>,
    ) -> Self {
        self.json_level_field = level.into();
        self.json_message_field = message.into();
        self.json_timestamp_field = timestamp.into();
        self
    }

    /// Parse a raw log line into a LogEntry
    pub fn parse(&self, raw: &str, line_number: usize) -> LogEntry {
        let mut entry = LogEntry::new(raw, line_number);

        // Try JSON parsing first
        if self.json_parsing && raw.trim_start().starts_with('{') {
            if let Some(parsed) = self.parse_json(raw) {
                return parsed.line_number(line_number);
            }
        }

        // Try common log formats
        self.parse_standard(&mut entry);

        entry
    }

    /// Parse JSON log format
    fn parse_json(&self, raw: &str) -> Option<LogEntry> {
        // Simple JSON parsing without external dependency
        let trimmed = raw.trim();
        if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
            return None;
        }

        let mut entry = LogEntry::new(raw, 0);
        let mut fields = Vec::new();

        // Extract key-value pairs (simplified JSON parsing)
        let content = &trimmed[1..trimmed.len() - 1];
        let mut in_string = false;
        let mut escape_next = false;
        let mut current_key = String::new();
        let mut current_value = String::new();
        let mut parsing_key = true;
        let mut depth = 0;

        for ch in content.chars() {
            if escape_next {
                if parsing_key {
                    current_key.push(ch);
                } else {
                    current_value.push(ch);
                }
                escape_next = false;
                continue;
            }

            match ch {
                '\\' => {
                    escape_next = true;
                }
                '"' if depth == 0 => {
                    in_string = !in_string;
                }
                ':' if !in_string && depth == 0 && parsing_key => {
                    parsing_key = false;
                }
                ',' if !in_string && depth == 0 => {
                    // End of key-value pair
                    let key = current_key.trim().trim_matches('"').to_string();
                    let value = current_value.trim().trim_matches('"').to_string();

                    if !key.is_empty() {
                        self.apply_json_field(&mut entry, &key, &value);
                        fields.push((key, value));
                    }

                    current_key.clear();
                    current_value.clear();
                    parsing_key = true;
                }
                '{' | '[' => {
                    depth += 1;
                    if !parsing_key {
                        current_value.push(ch);
                    }
                }
                '}' | ']' => {
                    if depth > 0 {
                        depth -= 1;
                    }
                    if !parsing_key {
                        current_value.push(ch);
                    }
                }
                _ => {
                    if parsing_key {
                        current_key.push(ch);
                    } else {
                        current_value.push(ch);
                    }
                }
            }
        }

        // Handle last pair
        if !current_key.is_empty() {
            let key = current_key.trim().trim_matches('"').to_string();
            let value = current_value.trim().trim_matches('"').to_string();
            self.apply_json_field(&mut entry, &key, &value);
            fields.push((key, value));
        }

        if !fields.is_empty() {
            entry.json_fields = Some(fields);
            Some(entry)
        } else {
            None
        }
    }

    /// Apply JSON field to entry based on configured field names
    fn apply_json_field(&self, entry: &mut LogEntry, key: &str, value: &str) {
        let key_lower = key.to_lowercase();

        if key_lower == self.json_level_field.to_lowercase()
            || key_lower == "level"
            || key_lower == "severity"
        {
            if let Some(level) = LogLevel::parse(value) {
                entry.level = level;
            }
        } else if key_lower == self.json_message_field.to_lowercase()
            || key_lower == "msg"
            || key_lower == "message"
        {
            entry.message = value.to_string();
        } else if key_lower == self.json_timestamp_field.to_lowercase()
            || key_lower == "time"
            || key_lower == "timestamp"
            || key_lower == "ts"
        {
            entry.timestamp = Some(value.to_string());
            // Try to parse as number for sorting
            if let Ok(ts) = value.parse::<i64>() {
                entry.timestamp_value = Some(ts);
            }
        } else if key_lower == self.json_source_field.to_lowercase()
            || key_lower == "source"
            || key_lower == "logger"
            || key_lower == "caller"
        {
            entry.source = Some(value.to_string());
        }
    }

    /// Parse standard log formats
    fn parse_standard(&self, entry: &mut LogEntry) {
        let raw = entry.raw.clone();
        let mut remaining = raw.as_str();

        // Try to extract timestamp at beginning
        // Common patterns: [2024-01-15 10:30:00], 2024-01-15T10:30:00, 10:30:00
        if let Some(ts_end) = self.find_timestamp_end(remaining) {
            let timestamp = &remaining[..ts_end];
            entry.timestamp = Some(timestamp.trim_matches(|c| c == '[' || c == ']').to_string());
            remaining = remaining[ts_end..].trim_start();
        }

        // Try to extract log level
        // Common patterns: [INFO], INFO, info:
        if let Some((level, end)) = self.find_level(remaining) {
            entry.level = level;
            remaining = remaining[end..].trim_start();
        }

        // Try to extract source
        // Common patterns: [module], module:, (module)
        if let Some((source, end)) = self.find_source(remaining) {
            entry.source = Some(source);
            remaining = remaining[end..].trim_start();
        }

        // Remaining is the message
        if !remaining.is_empty() {
            entry.message = remaining.to_string();
        }
    }

    /// Find end of timestamp in string
    fn find_timestamp_end(&self, s: &str) -> Option<usize> {
        // Check for bracketed timestamp [...]
        if s.starts_with('[') {
            if let Some(end) = s.find(']') {
                // Verify it looks like a timestamp
                let content = &s[1..end];
                if content.contains(':') || content.contains('-') || content.contains('/') {
                    return Some(end + 1);
                }
            }
        }

        // Check for ISO timestamp
        if s.len() >= 19 {
            let prefix = &s[..19];
            if prefix.chars().filter(|c| *c == '-').count() >= 2
                && prefix.chars().filter(|c| *c == ':').count() >= 2
            {
                // Check for milliseconds
                // Use char_indices for O(n) instead of O(n²) with .chars().nth()
                if s.len() > 19 {
                    if let Some((idx, ch)) = s.char_indices().nth(19) {
                        if ch == '.' {
                            let mut end = idx + 1;
                            // Check for digits after the dot
                            for (byte_idx, c) in s[end..].char_indices() {
                                if !c.is_ascii_digit() {
                                    return Some(end + byte_idx);
                                }
                                end += byte_idx;
                            }
                            return Some(end);
                        }
                    }
                }
                return Some(19);
            }
        }

        // Check for time only HH:MM:SS
        if s.len() >= 8 {
            let prefix = &s[..8];
            let chars: Vec<char> = prefix.chars().collect();
            if chars.len() == 8
                && chars[2] == ':'
                && chars[5] == ':'
                && chars[0].is_ascii_digit()
                && chars[1].is_ascii_digit()
                && chars[3].is_ascii_digit()
                && chars[4].is_ascii_digit()
                && chars[6].is_ascii_digit()
                && chars[7].is_ascii_digit()
            {
                return Some(8);
            }
        }

        None
    }

    /// Find log level in string
    fn find_level(&self, s: &str) -> Option<(LogLevel, usize)> {
        // Check for bracketed level [LEVEL]
        if s.starts_with('[') {
            if let Some(end) = s.find(']') {
                let content = &s[1..end];
                if let Some(level) = LogLevel::parse(content) {
                    return Some((level, end + 1));
                }
            }
        }

        // Check for level followed by colon or space
        let levels = [
            ("TRACE", LogLevel::Trace),
            ("DEBUG", LogLevel::Debug),
            ("INFO", LogLevel::Info),
            ("WARN", LogLevel::Warning),
            ("WARNING", LogLevel::Warning),
            ("ERROR", LogLevel::Error),
            ("FATAL", LogLevel::Fatal),
            ("CRITICAL", LogLevel::Fatal),
        ];

        let s_upper = s.to_uppercase();
        for (name, level) in levels {
            if s_upper.starts_with(name) {
                let end = name.len();
                if s.len() > end {
                    // Use skip().next() for O(n) instead of O(n²) with .chars().nth()
                    let next = s.chars().skip(end).next();
                    if next == Some(':') || next == Some(' ') || next == Some(']') {
                        let skip = if next == Some(':') { 1 } else { 0 };
                        return Some((level, end + skip));
                    }
                }
            }
        }

        None
    }

    /// Find source/logger name in string
    fn find_source(&self, s: &str) -> Option<(String, usize)> {
        // Check for bracketed source [source]
        if s.starts_with('[') {
            if let Some(end) = s.find(']') {
                let content = &s[1..end];
                // Make sure it's not a level we already parsed
                if LogLevel::parse(content).is_none()
                    && !content.contains(' ')
                    && content.len() < 50
                {
                    return Some((content.to_string(), end + 1));
                }
            }
        }

        // Check for source followed by colon
        if let Some(colon_pos) = s.find(':') {
            if colon_pos < 50 {
                let potential = &s[..colon_pos];
                // Should be a simple identifier
                if !potential.contains(' ')
                    && !potential.is_empty()
                    && LogLevel::parse(potential).is_none()
                {
                    return Some((potential.to_string(), colon_pos + 1));
                }
            }
        }

        None
    }
}

/// Trait for setting line number on log entries
pub trait WithLineNumber {
    fn line_number(self, n: usize) -> Self;
}

impl WithLineNumber for LogEntry {
    fn line_number(mut self, n: usize) -> Self {
        self.line_number = n;
        self
    }
}
