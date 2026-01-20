//! Log filtering functionality for advanced log viewer

use super::entry::LogEntry;
use super::types::LogLevel;

/// Filter configuration
#[derive(Clone, Debug, Default)]
pub struct LogFilter {
    /// Minimum log level to display
    pub min_level: Option<LogLevel>,
    /// Show only specific levels
    pub levels: Option<Vec<LogLevel>>,
    /// Text contains filter
    pub contains: Option<String>,
    /// Source filter
    pub source: Option<String>,
    /// Show only bookmarked entries
    pub bookmarked_only: bool,
    /// Time range start
    pub time_start: Option<i64>,
    /// Time range end
    pub time_end: Option<i64>,
}

impl LogFilter {
    /// Create a new empty filter
    pub fn new() -> Self {
        Self::default()
    }

    /// Set minimum level
    pub fn min_level(mut self, level: LogLevel) -> Self {
        self.min_level = Some(level);
        self
    }

    /// Set specific levels to show
    pub fn levels(mut self, levels: Vec<LogLevel>) -> Self {
        self.levels = Some(levels);
        self
    }

    /// Set contains filter
    pub fn contains(mut self, text: impl Into<String>) -> Self {
        self.contains = Some(text.into());
        self
    }

    /// Set source filter
    pub fn source(mut self, src: impl Into<String>) -> Self {
        self.source = Some(src.into());
        self
    }

    /// Show only bookmarked
    pub fn bookmarked_only(mut self) -> Self {
        self.bookmarked_only = true;
        self
    }

    /// Set time range
    pub fn time_range(mut self, start: i64, end: i64) -> Self {
        self.time_start = Some(start);
        self.time_end = Some(end);
        self
    }

    /// Check if entry matches filter
    pub fn matches(&self, entry: &LogEntry) -> bool {
        // Check minimum level
        if let Some(min) = self.min_level {
            if entry.level < min {
                return false;
            }
        }

        // Check specific levels
        if let Some(ref levels) = self.levels {
            if !levels.contains(&entry.level) {
                return false;
            }
        }

        // Check contains
        if let Some(ref text) = self.contains {
            let lower_text = text.to_lowercase();
            if !entry.message.to_lowercase().contains(&lower_text)
                && !entry.raw.to_lowercase().contains(&lower_text)
            {
                return false;
            }
        }

        // Check source
        if let Some(ref src) = self.source {
            if let Some(ref entry_src) = entry.source {
                if !entry_src.to_lowercase().contains(&src.to_lowercase()) {
                    return false;
                }
            } else {
                return false;
            }
        }

        // Check bookmarked
        if self.bookmarked_only && !entry.bookmarked {
            return false;
        }

        // Check time range
        if let Some(ts) = entry.timestamp_value {
            if let Some(start) = self.time_start {
                if ts < start {
                    return false;
                }
            }
            if let Some(end) = self.time_end {
                if ts > end {
                    return false;
                }
            }
        }

        true
    }
}
