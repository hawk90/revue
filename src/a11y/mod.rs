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

// Tests extracted to tests/a11y_public_api_tests.rs
// All tests use only public functions from revue::a11y
