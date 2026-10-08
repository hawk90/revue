//! Log levels, log entries and display formats

use crate::style::Color;
use crate::widget::theme::{DISABLED_FG, LIGHT_GRAY};

/// Log level
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    /// Trace-level logging (most verbose)
    Trace,
    /// Debug-level logging
    Debug,
    /// Info-level logging (default)
    #[default]
    Info,
    /// Warning-level logging
    Warning,
    /// Error-level logging
    Error,
    /// Fatal/critical-level logging
    Fatal,
}

impl LogLevel {
    /// Get color for log level
    pub fn color(&self) -> Color {
        match self {
            LogLevel::Trace => DISABLED_FG,
            LogLevel::Debug => LIGHT_GRAY,
            LogLevel::Info => Color::CYAN,
            LogLevel::Warning => Color::YELLOW,
            LogLevel::Error => Color::RED,
            LogLevel::Fatal => Color::rgb(255, 50, 50),
        }
    }

    /// Get icon for log level
    pub fn icon(&self) -> char {
        match self {
            LogLevel::Trace => '·',
            LogLevel::Debug => '○',
            LogLevel::Info => '●',
            LogLevel::Warning => '⚠',
            LogLevel::Error => '✗',
            LogLevel::Fatal => '☠',
        }
    }

    /// Get label for log level
    pub fn label(&self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARN",
            LogLevel::Error => "ERROR",
            LogLevel::Fatal => "FATAL",
        }
    }
}

/// A log entry
#[derive(Clone, Debug)]
pub struct LogEntry {
    /// Log message
    pub message: String,
    /// Log level
    pub level: LogLevel,
    /// Timestamp
    pub timestamp: Option<String>,
    /// Source/module
    pub source: Option<String>,
    /// Is expanded: draw the detail lines under the message
    pub expanded: bool,
    /// Additional lines
    pub details: Vec<String>,
}

impl LogEntry {
    /// Create a new log entry
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            level: LogLevel::Info,
            timestamp: None,
            source: None,
            expanded: false,
            details: Vec::new(),
        }
    }

    /// Set log level
    pub fn level(mut self, level: LogLevel) -> Self {
        self.level = level;
        self
    }

    /// Set as trace
    pub fn trace(mut self) -> Self {
        self.level = LogLevel::Trace;
        self
    }

    /// Set as debug
    pub fn debug(mut self) -> Self {
        self.level = LogLevel::Debug;
        self
    }

    /// Set as info
    pub fn info(mut self) -> Self {
        self.level = LogLevel::Info;
        self
    }

    /// Set as warning
    pub fn warning(mut self) -> Self {
        self.level = LogLevel::Warning;
        self
    }

    /// Set as error
    pub fn error(mut self) -> Self {
        self.level = LogLevel::Error;
        self
    }

    /// Set as fatal
    pub fn fatal(mut self) -> Self {
        self.level = LogLevel::Fatal;
        self
    }

    /// Set timestamp
    pub fn timestamp(mut self, ts: impl Into<String>) -> Self {
        self.timestamp = Some(ts.into());
        self
    }

    /// Set source
    pub fn source(mut self, src: impl Into<String>) -> Self {
        self.source = Some(src.into());
        self
    }

    /// Add detail line
    pub fn detail(mut self, line: impl Into<String>) -> Self {
        self.details.push(line.into());
        self
    }

    /// Add details
    pub fn details(mut self, lines: Vec<String>) -> Self {
        self.details.extend(lines);
        self
    }

    /// Toggle expanded
    pub fn toggle(&mut self) {
        self.expanded = !self.expanded;
    }
}

/// Log display format
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LogFormat {
    /// Simple message only
    Simple,
    /// With level indicator
    #[default]
    Standard,
    /// With timestamp and source
    Detailed,
    /// Custom format
    #[deprecated(
        since = "3.5.0",
        note = "there is no way to supply a custom format; it draws like `Standard`. Use `Standard` and the column builders (`timestamps`, `sources`, `icons`, `labels`) instead"
    )]
    Custom,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_log_level_private_methods() {
        // Test private implementation details that can't be tested via public API
        use super::*;

        // Access private field through public API
        let entry = LogEntry::new("Test");
        // Note: This is testing that the private field exists and is properly initialized
        assert_eq!(entry.message, "Test");
        assert_eq!(entry.level, LogLevel::Info);

        // Test the builder pattern implementation
        let entry = LogEntry::new("Test").level(LogLevel::Error);
        assert_eq!(entry.level, LogLevel::Error);
    }
}
