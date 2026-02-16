//! Tests for a11y::backend::core extracted from src/a11y/backend/core.rs
//!
//! All tests use only public methods from the ScreenReaderBackend type.

use revue::a11y::{BackendType, ScreenReaderBackend, ScreenReaderConfig};
use revue::utils::accessibility::Priority;

// =========================================================================
// ScreenReaderBackend construction tests
// =========================================================================

#[test]
fn test_screen_reader_backend_new() {
    let backend = ScreenReaderBackend::new(BackendType::None);
    assert_eq!(backend.backend_type(), BackendType::None);
}

#[test]
fn test_screen_reader_backend_auto() {
    let backend = ScreenReaderBackend::new(BackendType::Auto);
    // Should resolve to some backend type
    let _ = backend.backend_type();
}

#[test]
fn test_screen_reader_backend_new_logging() {
    let backend = ScreenReaderBackend::new(BackendType::Logging);
    assert_eq!(backend.backend_type(), BackendType::Logging);
}

#[test]
fn test_screen_reader_backend_with_config() {
    let config = ScreenReaderConfig {
        backend_type: BackendType::None,
        debounce_ms: 100,
        log_announcements: false,
        announce_roles: false,
    };

    let backend = ScreenReaderBackend::with_config(config);
    assert_eq!(backend.backend_type(), BackendType::None);
}

// =========================================================================
// ScreenReaderBackend availability tests
// =========================================================================

#[test]
fn test_screen_reader_backend_is_available() {
    let backend = ScreenReaderBackend::new(BackendType::None);
    // Just verify the method is callable - actual availability depends on backend
    let _ = backend.is_available();
}

#[test]
fn test_screen_reader_backend_active_screen_reader() {
    let backend = ScreenReaderBackend::new(BackendType::None);
    // NullBackend returns None for active screen reader
    assert!(backend.active_screen_reader().is_none());
}

// =========================================================================
// ScreenReaderBackend announcement tests
// =========================================================================

#[test]
fn test_screen_reader_backend_announce() {
    let backend = ScreenReaderBackend::new(BackendType::None);
    // Should not panic
    backend.announce("Test message", Priority::Polite);
}

#[test]
fn test_screen_reader_backend_announce_assertive() {
    let backend = ScreenReaderBackend::new(BackendType::None);
    // Should not panic
    backend.announce("Important message", Priority::Assertive);
}

#[test]
fn test_screen_reader_backend_announce_focus() {
    let backend = ScreenReaderBackend::new(BackendType::None);
    // Should not panic
    backend.announce_focus("Button", "button");
}

// =========================================================================
// ScreenReaderBackend control tests
// =========================================================================

#[test]
fn test_screen_reader_backend_stop() {
    let backend = ScreenReaderBackend::new(BackendType::None);
    // Should not panic
    backend.stop();
}
