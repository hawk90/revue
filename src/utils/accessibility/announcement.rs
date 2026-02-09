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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    // =========================================================================
    // Priority tests
    // =========================================================================

    #[test]
    fn test_priority_partial_eq() {
        assert_eq!(Priority::Polite, Priority::Polite);
        assert_eq!(Priority::Assertive, Priority::Assertive);
        assert_ne!(Priority::Polite, Priority::Assertive);
    }

    #[test]
    fn test_priority_clone() {
        let priority = Priority::Polite;
        let cloned = priority.clone();
        assert_eq!(priority, cloned);
    }

    #[test]
    fn test_priority_copy() {
        let priority = Priority::Assertive;
        let copied = priority;
        assert_eq!(priority, Priority::Assertive);
        assert_eq!(copied, Priority::Assertive);
    }

    // =========================================================================
    // Announcement::polite() tests
    // =========================================================================

    #[test]
    fn test_announcement_polite_str() {
        let ann = Announcement::polite("Test message");
        assert_eq!(ann.message, "Test message");
        assert_eq!(ann.priority, Priority::Polite);
    }

    #[test]
    fn test_announcement_polite_string() {
        let ann = Announcement::polite(String::from("Test message"));
        assert_eq!(ann.message, "Test message");
        assert_eq!(ann.priority, Priority::Polite);
    }

    #[test]
    fn test_announcement_polite_empty() {
        let ann = Announcement::polite("");
        assert_eq!(ann.message, "");
        assert_eq!(ann.priority, Priority::Polite);
    }

    #[test]
    fn test_announcement_polite_timestamp_set() {
        let before = std::time::Instant::now();
        let ann = Announcement::polite("Test");
        let after = std::time::Instant::now();
        assert!(ann.timestamp >= before);
        assert!(ann.timestamp <= after);
    }

    // =========================================================================
    // Announcement::assertive() tests
    // =========================================================================

    #[test]
    fn test_announcement_assertive_str() {
        let ann = Announcement::assertive("Test message");
        assert_eq!(ann.message, "Test message");
        assert_eq!(ann.priority, Priority::Assertive);
    }

    #[test]
    fn test_announcement_assertive_string() {
        let ann = Announcement::assertive(String::from("Test message"));
        assert_eq!(ann.message, "Test message");
        assert_eq!(ann.priority, Priority::Assertive);
    }

    #[test]
    fn test_announcement_assertive_empty() {
        let ann = Announcement::assertive("");
        assert_eq!(ann.message, "");
        assert_eq!(ann.priority, Priority::Assertive);
    }

    #[test]
    fn test_announcement_assertive_timestamp_set() {
        let before = std::time::Instant::now();
        let ann = Announcement::assertive("Test");
        let after = std::time::Instant::now();
        assert!(ann.timestamp >= before);
        assert!(ann.timestamp <= after);
    }

    // =========================================================================
    // Announcement clone tests
    // =========================================================================

    #[test]
    fn test_announcement_clone_polite() {
        let ann = Announcement::polite("Test");
        let cloned = ann.clone();
        assert_eq!(ann.message, cloned.message);
        assert_eq!(ann.priority, cloned.priority);
    }

    #[test]
    fn test_announcement_clone_assertive() {
        let ann = Announcement::assertive("Test");
        let cloned = ann.clone();
        assert_eq!(ann.message, cloned.message);
        assert_eq!(ann.priority, cloned.priority);
    }

    // =========================================================================
    // Announcement public field tests
    // =========================================================================

    #[test]
    fn test_announcement_public_message() {
        let mut ann = Announcement::polite("Test");
        assert_eq!(ann.message, "Test");
        ann.message = "Updated".to_string();
        assert_eq!(ann.message, "Updated");
    }

    #[test]
    fn test_announcement_public_priority() {
        let mut ann = Announcement::polite("Test");
        assert_eq!(ann.priority, Priority::Polite);
        ann.priority = Priority::Assertive;
        assert_eq!(ann.priority, Priority::Assertive);
    }

    #[test]
    fn test_announcement_public_timestamp() {
        let ann = Announcement::polite("Test");
        std::thread::sleep(Duration::from_millis(10));
        // timestamp should be set at creation time
        assert!(ann.timestamp.elapsed() >= Duration::from_millis(10));
    }

    // =========================================================================
    // Announcement ordering tests
    // =========================================================================

    #[test]
    fn test_announcement_timestamp_ordering() {
        let ann1 = Announcement::polite("First");
        std::thread::sleep(Duration::from_millis(10));
        let ann2 = Announcement::polite("Second");
        assert!(ann2.timestamp > ann1.timestamp);
    }

    #[test]
    fn test_announcement_different_priorities() {
        let polite = Announcement::polite("Polite");
        let assertive = Announcement::assertive("Assertive");
        assert_eq!(polite.priority, Priority::Polite);
        assert_eq!(assertive.priority, Priority::Assertive);
    }
}
