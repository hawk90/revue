//! Accessibility tree generation and A11y testing utilities
//!
//! # Screen reader output (deprecated)
//!
//! The screen reader backend (`init`, `announce`, `announce_now`,
//! `ScreenReaderBackend`, `BackendType` and the rest) is deprecated since
//! 3.17.0 and will be removed: platform screen-reader output is unsupported
//! (#937). A terminal app has no window of its own; the terminal emulator
//! owns the accessibility tree and screen readers read the terminal's text,
//! so talking to the screen reader from the app is not dependable. On
//! macOS, polite announcements set a menu bar attribute instead of
//! speaking, and VoiceOver has no command to stop speech.
//!
//! Announcements are a queue instead: [`crate::utils::announce`] and
//! [`crate::utils::announce_now`] add to it, and the app reads it with
//! [`crate::utils::take_announcements`] (to show in a status line, say).
//!
//! # Features
//!
//! | Feature | Description |
//!|---------|-------------|
//! | **A11y Testing** | Test accessibility features ([`A11yTestRunner`]) |
//! | **Accessibility Tree** | Generate an A11y tree ([`AccessibilityTree`]) |
//! | **Keyboard Nav** | Test keyboard navigation ([`KeyboardNavigator`]) |

// Deprecated as a whole (#937); its own uses of its items are not warnings
#[allow(deprecated)]
mod backend;
pub mod testing;
mod tree;

#[allow(deprecated)]
pub use backend::{
    announce_to_screen_reader, get_backend, init_backend, set_backend, BackendType,
    LoggedAnnouncement, LoggingBackend, ScreenReader, ScreenReaderBackend, ScreenReaderConfig,
};
pub use testing::{A11yTestRunner, KeyboardNavigator};
pub use tree::{AccessibilityTree, AccessibilityTreeBuilder, TreeNode, TreeNodeId};

use crate::utils::accessibility::Priority;

/// Initialize the screen reader system with auto-detection
#[deprecated(
    since = "3.17.0",
    note = "platform screen-reader output is unsupported (#937): queue announcements with `revue::utils::announce` / `announce_now` and read them with `revue::utils::take_announcements`"
)]
#[allow(deprecated)]
pub fn init() -> &'static ScreenReaderBackend {
    init_backend(BackendType::Auto)
}

/// Initialize with a specific backend type
#[deprecated(
    since = "3.17.0",
    note = "platform screen-reader output is unsupported (#937): queue announcements with `revue::utils::announce` / `announce_now` and read them with `revue::utils::take_announcements`"
)]
#[allow(deprecated)]
pub fn init_with(backend_type: BackendType) -> &'static ScreenReaderBackend {
    init_backend(backend_type)
}

/// Announce a message to the screen reader (polite)
#[deprecated(
    since = "3.17.0",
    note = "platform screen-reader output is unsupported (#937): queue announcements with `revue::utils::announce` / `announce_now` and read them with `revue::utils::take_announcements`"
)]
#[allow(deprecated)]
pub fn announce(message: impl Into<String>) {
    announce_to_screen_reader(message, Priority::Polite);
}

/// Announce a message immediately (assertive)
#[deprecated(
    since = "3.17.0",
    note = "platform screen-reader output is unsupported (#937): queue announcements with `revue::utils::announce` / `announce_now` and read them with `revue::utils::take_announcements`"
)]
#[allow(deprecated)]
pub fn announce_now(message: impl Into<String>) {
    announce_to_screen_reader(message, Priority::Assertive);
}

/// Check if a screen reader is available
#[deprecated(
    since = "3.17.0",
    note = "platform screen-reader output is unsupported (#937): queue announcements with `revue::utils::announce` / `announce_now` and read them with `revue::utils::take_announcements`"
)]
#[allow(deprecated)]
pub fn is_available() -> bool {
    get_backend().is_available()
}

/// Get the name of the active screen reader
#[deprecated(
    since = "3.17.0",
    note = "platform screen-reader output is unsupported (#937): queue announcements with `revue::utils::announce` / `announce_now` and read them with `revue::utils::take_announcements`"
)]
#[allow(deprecated)]
pub fn active_screen_reader() -> Option<String> {
    get_backend().active_screen_reader()
}
// All tests use only public functions from revue::a11y
