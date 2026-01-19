//! Log types and enums for advanced log viewer

use crate::style::Color;

/// Log level for filtering and display
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    /// Fatal/critical-level logging (most severe)
    Fatal,
}

impl LogLevel {
    /// Get display color for this log level
    pub fn color(&self) -> Color {
        match self {
            LogLevel::Trace => Color::rgb(100, 100, 100),
            LogLevel::Debug => Color::rgb(150, 150, 150),
            LogLevel::Info => Color::CYAN,
            LogLevel::Warning => Color::YELLOW,
            LogLevel::Error => Color::RED,
            LogLevel::Fatal => Color::rgb(255, 50, 50),
        }
    }

    /// Get icon character for this log level
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

    /// Get short label for this log level
    pub fn label(&self) -> &'static str {
        match self {
            LogLevel::Trace => "TRC",
            LogLevel::Debug => "DBG",
            LogLevel::Info => "INF",
            LogLevel::Warning => "WRN",
            LogLevel::Error => "ERR",
            LogLevel::Fatal => "FTL",
        }
    }

    /// Parse log level from string (case-insensitive)
    pub fn parse(s: &str) -> Option<LogLevel> {
        match s.to_uppercase().as_str() {
            "TRACE" | "TRC" => Some(LogLevel::Trace),
            "DEBUG" | "DBG" => Some(LogLevel::Debug),
            "INFO" | "INF" => Some(LogLevel::Info),
            "WARN" | "WARNING" | "WRN" => Some(LogLevel::Warning),
            "ERROR" | "ERR" => Some(LogLevel::Error),
            "FATAL" | "FTL" | "CRITICAL" | "CRIT" => Some(LogLevel::Fatal),
            _ => None,
        }
    }
}

/// Timestamp format for parsing and display
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TimestampFormat {
    /// ISO 8601 format (2024-01-15T10:30:00)
    #[default]
    Iso8601,
    /// Unix timestamp (seconds since epoch)
    Unix,
    /// Unix timestamp (milliseconds)
    UnixMillis,
    /// Time only (HH:MM:SS)
    TimeOnly,
    /// Date and time (YYYY-MM-DD HH:MM:SS)
    DateTime,
    /// Custom format (use with parser)
    Custom,
}
