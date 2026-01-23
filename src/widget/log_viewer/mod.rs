//! Advanced Log Viewer widget
//!
//! Provides a feature-rich log viewer with regex search, filtering, bookmarks,
//! JSON log parsing, live tail mode, and timestamp navigation.

mod entry;
mod filter;
mod parser;
mod types;
mod view;

// Public exports
pub use entry::{LogEntry, SearchMatch};
pub use filter::LogFilter;
pub use parser::LogParser;
pub use types::{LogLevel, TimestampFormat};
pub use view::{log_entry, log_filter, log_parser, log_viewer, LogViewer};

#[cfg(test)]
mod tests;
