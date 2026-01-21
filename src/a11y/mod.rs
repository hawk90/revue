//! Screen reader backend integration
//!
//! Provides actual screen reader integration for accessibility announcements.
//! Supports multiple backends:
//!
//! - **macOS**: NSAccessibility / VoiceOver
//! - **Windows**: UI Automation / MSAA
//! - **Linux**: AT-SPI
//! - **Testing**: Logging backend for tests
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::a11y::{ScreenReaderBackend, announce_to_screen_reader};
//!
//! // Initialize with auto-detected backend
//! let backend = ScreenReaderBackend::detect();
//!
//! // Announce to screen reader
//! backend.announce("Button clicked", Priority::Polite);
//!
//! // Or use the global function
//! announce_to_screen_reader("Form submitted");
//! ```

mod backend;
mod testing;
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
