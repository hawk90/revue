//! Tests for a11y::backend::global extracted from src/a11y/backend/global.rs
//!
//! All tests use only public functions from the a11y::backend module.

use revue::a11y::{announce_to_screen_reader, get_backend, init_backend, BackendType};
use revue::utils::accessibility::Priority;

// =========================================================================
// Backend initialization tests
// =========================================================================

#[test]
fn test_get_backend_auto_initializes() {
    // Should not panic and should return a backend
    let backend = get_backend();
    // We can't test much without exposing internals, but we can verify it doesn't crash
    let _ = backend;
}

#[test]
fn test_init_backend_with_auto() {
    // Initialize with Auto type
    let backend = init_backend(BackendType::Auto);
    let _ = backend;
}

#[test]
fn test_init_backend_with_logging() {
    // Initialize with Logging type for testing
    let backend = init_backend(BackendType::Logging);
    let _ = backend;
}

#[test]
fn test_init_backend_with_none() {
    // Initialize with None type (silent)
    let backend = init_backend(BackendType::None);
    let _ = backend;
}

#[test]
fn test_get_backend_returns_same_instance() {
    // get_backend should return the same instance each time
    let backend1 = get_backend();
    let backend2 = get_backend();
    // They should be the same reference (same address)
    assert_eq!(std::ptr::from_ref(backend1), std::ptr::from_ref(backend2));
}

#[test]
fn test_init_backend_returns_same_instance() {
    // init_backend should return the same instance each time
    let backend1 = init_backend(BackendType::None);
    let backend2 = get_backend(); // get_backend should return the same instance
    assert_eq!(std::ptr::from_ref(backend1), std::ptr::from_ref(backend2));
}

// =========================================================================
// Announcement tests
// =========================================================================

#[test]
fn test_announce_to_screen_reader_wrapper() {
    // Should not panic
    announce_to_screen_reader("Test message", Priority::Polite);
}

#[test]
fn test_announce_with_assertive_priority() {
    // Should not panic with assertive priority
    announce_to_screen_reader("Important message", Priority::Assertive);
}

#[test]
fn test_announce_with_polite_priority() {
    // Should not panic with polite priority
    announce_to_screen_reader("Polite message", Priority::Polite);
}

#[test]
fn test_multiple_announces() {
    // Should handle multiple announces
    announce_to_screen_reader("Message 1", Priority::Polite);
    announce_to_screen_reader("Message 2", Priority::Polite);
    announce_to_screen_reader("Message 3", Priority::Polite);
}

// Note: set_backend tests are difficult because OnceLock can only be set once
// and the state persists across tests, so we skip them
