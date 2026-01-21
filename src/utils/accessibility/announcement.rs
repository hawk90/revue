//! Announcement for screen readers

/// Announcement priority
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Priority {
    /// Polite - wait for idle
    Polite,
    /// Assertive - interrupt
    Assertive,
}

/// Announcement for screen readers
#[derive(Clone, Debug)]
pub struct Announcement {
    /// Message text
    pub message: String,
    /// Priority level
    pub priority: Priority,
    /// Timestamp
    pub timestamp: std::time::Instant,
}

impl Announcement {
    /// Create new polite announcement
    pub fn polite(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            priority: Priority::Polite,
            timestamp: std::time::Instant::now(),
        }
    }

    /// Create new assertive announcement
    pub fn assertive(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            priority: Priority::Assertive,
            timestamp: std::time::Instant::now(),
        }
    }
}
