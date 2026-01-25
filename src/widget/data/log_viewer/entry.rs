//! Log entry types for advanced log viewer

use super::types::LogLevel;

/// A parsed log entry
#[derive(Clone, Debug)]
pub struct LogEntry {
    /// Original raw line
    pub raw: String,
    /// Parsed message content
    pub message: String,
    /// Detected log level
    pub level: LogLevel,
    /// Parsed timestamp string
    pub timestamp: Option<String>,
    /// Timestamp as sortable value (for jump-to-time)
    pub timestamp_value: Option<i64>,
    /// Source/logger name
    pub source: Option<String>,
    /// Line number in original source
    pub line_number: usize,
    /// Is this line bookmarked
    pub bookmarked: bool,
    /// Parsed JSON fields (if JSON log)
    pub json_fields: Option<Vec<(String, String)>>,
    /// Is expanded to show full details
    pub expanded: bool,
}

impl LogEntry {
    /// Create a new log entry from raw text
    pub fn new(raw: impl Into<String>, line_number: usize) -> Self {
        let raw = raw.into();
        Self {
            message: raw.clone(),
            raw,
            level: LogLevel::Info,
            timestamp: None,
            timestamp_value: None,
            source: None,
            line_number,
            bookmarked: false,
            json_fields: None,
            expanded: false,
        }
    }

    /// Set log level
    pub fn level(mut self, level: LogLevel) -> Self {
        self.level = level;
        self
    }

    /// Set message
    pub fn message(mut self, msg: impl Into<String>) -> Self {
        self.message = msg.into();
        self
    }

    /// Set timestamp
    pub fn timestamp(mut self, ts: impl Into<String>) -> Self {
        self.timestamp = Some(ts.into());
        self
    }

    /// Set timestamp value for sorting
    pub fn timestamp_value(mut self, value: i64) -> Self {
        self.timestamp_value = Some(value);
        self
    }

    /// Set source
    pub fn source(mut self, src: impl Into<String>) -> Self {
        self.source = Some(src.into());
        self
    }

    /// Set JSON fields
    pub fn json_fields(mut self, fields: Vec<(String, String)>) -> Self {
        self.json_fields = Some(fields);
        self
    }

    /// Toggle bookmark status
    pub fn toggle_bookmark(&mut self) {
        self.bookmarked = !self.bookmarked;
    }

    /// Toggle expanded state
    pub fn toggle_expanded(&mut self) {
        self.expanded = !self.expanded;
    }
}

/// Search match information
#[derive(Clone, Debug)]
pub struct SearchMatch {
    /// Entry index
    pub entry_index: usize,
    /// Start position in message
    pub start: usize,
    /// End position in message
    pub end: usize,
}
