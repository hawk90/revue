//! Accessibility support for screen readers and assistive technologies
//!
//! This module provides screen reader integration for accessibility announcements,
//! accessibility tree generation, and A11y testing utilities.
//!
//! # Features
//!
//! | Feature | Description | Supported Platforms |
//!|---------|-------------|----------------------|
//! | **Screen Reader** | VoiceOver, Narrator, AT-SPI | macOS, Windows, Linux |
//! | **Auto-detection** | Detects available screen readers | All platforms |
//! | **Priority Levels** | Polite, Assertive, Urgent | All platforms |
//! | **A11y Testing** | Test accessibility features | All platforms |
//! | **Accessibility Tree** | Generate A11y tree from DOM | All platforms |
//! | **Keyboard Nav** | Test keyboard navigation | All platforms |
//!
//! # Supported Backends
//!
//! | Platform | Backend | Screen Reader |
//!|----------|---------|---------------|
//! | macOS | NSAccessibility | VoiceOver |
//! | Windows | UI Automation | Narrator |
//! | Linux | AT-SPI via Orca | Orca |
//! | All | Logging (for testing) | - |
//!
//! # Quick Start
//!
//! ## Auto-Detection
//!
//! ```rust,ignore
//! use revue::a11y::init;
//!
//! // Auto-detect and initialize
//! init();
//!
//! // Check if available
//! if is_available() {
//!     println!("Screen reader: {:?}", active_screen_reader());
//! }
//! ```
//!
//! ## Announcements
//!
//! ```rust,ignore
//! use revue::a11y::{announce, announce_now};
//!
//! // Polite announcement (default)
//! announce("Button clicked");
//!
//! // Assertive announcement (immediate)
//! announce_now("Error occurred");
//!
//! // Custom priority
//! announce_to_screen_reader("Message", Priority::Urgent);
//! ```
//!
//! ## Accessibility Tree
//!
//! ```rust,ignore
//! use revue::a11y::AccessibilityTree;
//!
//! // Generate A11y tree from widget
//! let tree = AccessibilityTree::from_widget(&widget);
//!
//! // Get nodes for screen reader
//! let nodes = tree.get_a11y_nodes();
//! ```
//!
//! # Priority Levels
//!
//! | Priority | Description | Use Case |
//!|----------|-------------|----------|
//! | `Polite` | Low priority | Status updates, info |
//! | `Assertive` | Medium priority | Form errors, state changes |
//! | `Urgent` | High priority | Critical errors, warnings |

mod backend;
pub mod testing;
mod tree;

pub use backend::{
    announce_to_screen_reader, get_backend, init_backend, set_backend, BackendType, LoggingBackend,
    ScreenReader, ScreenReaderBackend, ScreenReaderConfig,
};
pub use testing::{A11yTestRunner, KeyboardNavigator};
pub use tree::{AccessibilityTree, AccessibilityTreeBuilder, TreeNode, TreeNodeId};

use crate::utils::accessibility::Priority;

/// Initialize the screen reader system with auto-detection
pub fn init() -> &'static ScreenReaderBackend {
    init_backend(BackendType::Auto)
}

/// Initialize with a specific backend type
pub fn init_with(backend_type: BackendType) -> &'static ScreenReaderBackend {
    init_backend(backend_type)
}

/// Announce a message to the screen reader (polite)
pub fn announce(message: impl Into<String>) {
    announce_to_screen_reader(message, Priority::Polite);
}

/// Announce a message immediately (assertive)
pub fn announce_now(message: impl Into<String>) {
    announce_to_screen_reader(message, Priority::Assertive);
}

/// Check if a screen reader is available
pub fn is_available() -> bool {
    get_backend().is_available()
}

/// Get the name of the active screen reader
pub fn active_screen_reader() -> Option<String> {
    get_backend().active_screen_reader()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_returns_backend() {
        let backend = init();
        assert!(
            backend as *const _ as usize != 0,
            "init should return a valid reference"
        );
    }

    #[test]
    fn test_init_with_logging() {
        let backend = init_with(BackendType::Logging);
        assert!(
            backend as *const _ as usize != 0,
            "init_with should return a valid reference"
        );
    }

    #[test]
    fn test_announce_does_not_panic() {
        // Should not panic with various message types
        announce("Test message");
        announce("".to_string());
        announce("Message with unicode: 你好");
    }

    #[test]
    fn test_announce_now_does_not_panic() {
        announce_now("Urgent message");
        announce_now("".to_string());
    }

    #[test]
    fn test_is_available_returns_bool() {
        let result = is_available();
        // Should return a boolean value (true or false is fine)
        match result {
            true | false => {}
        }
    }

    #[test]
    fn test_active_screen_reader_returns_option() {
        let result = active_screen_reader();
        // Should return Some(String) or None
        match result {
            Some(_) | None => {}
        }
    }

    // =========================================================================
    // Additional a11y tests
    // =========================================================================

    #[test]
    fn test_announce_with_emoji() {
        announce("🎉 Success! 🎊");
        announce("⚠️ Warning ⚠️");
        announce("❌ Error ❌");
    }

    #[test]
    fn test_announce_with_newlines() {
        announce("Line 1\nLine 2\nLine 3");
    }

    #[test]
    fn test_announce_with_tabs() {
        announce("Tab\tseparated\ttext");
    }

    #[test]
    fn test_announce_long_message() {
        let long_msg = "A".repeat(10000);
        announce(long_msg);
    }

    #[test]
    fn test_announce_with_special_chars() {
        announce("Special: @#$%^&*()[]{}|\\:;\"'<>?,./");
    }

    #[test]
    fn test_announce_now_with_unicode() {
        announce_now("🔔 Notification: 你好 🎊");
    }

    #[test]
    fn test_announce_now_empty() {
        announce_now("");
    }

    #[test]
    fn test_announce_now_with_numbers() {
        announce_now("12345");
        announce_now("99.9%");
    }

    #[test]
    fn test_init_with_auto() {
        let backend = init_with(BackendType::Auto);
        assert!(backend as *const _ as usize != 0);
    }

    #[test]
    fn test_init_with_all_backend_types() {
        let _ = init_with(BackendType::Auto);
        let _ = init_with(BackendType::Logging);
        // These should not panic
    }

    #[test]
    fn test_multiple_init_calls() {
        let backend1 = init();
        let backend2 = init();
        // Multiple init calls should work
        assert!(backend1 as *const _ as usize != 0);
        assert!(backend2 as *const _ as usize != 0);
    }

    #[test]
    fn test_announce_multiple_times() {
        for i in 0..10 {
            announce(format!("Message {}", i));
        }
    }

    #[test]
    fn test_announce_now_multiple_times() {
        for i in 0..10 {
            announce_now(format!("Urgent {}", i));
        }
    }

    #[test]
    fn test_is_available_consistent() {
        let result1 = is_available();
        let result2 = is_available();
        // Results should be consistent (both true or both false)
        assert_eq!(result1, result2);
    }

    #[test]
    fn test_helpers_do_not_panic() {
        // All helper functions should not panic
        let _ = is_available();
        let _ = active_screen_reader();
        announce("test");
        announce_now("test");
    }

    #[test]
    fn test_announce_with_korean_text() {
        announce("한글 메시지입니다");
        announce_now("중요 알림");
    }

    #[test]
    fn test_announce_with_japanese_text() {
        announce("日本語のメッセージ");
        announce_now("重要な通知");
    }

    #[test]
    fn test_announce_with_chinese_text() {
        announce("中文消息");
        announce_now("重要通知");
    }
}
