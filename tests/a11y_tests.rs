//! Accessibility integration tests
//!
//! Tests for screen reader backend integration and accessibility tree.

mod a11y;

// =============================================================================
// Public API Convenience Functions Tests
// =============================================================================
//
// Tests for the convenience functions in src/a11y/mod.rs
//
// These are the public API functions that users typically import:
// - `init()` - Initialize with auto-detection
// - `init_with()` - Initialize with specific backend
// - `announce()` - Announce polite message
// - `announce_now()` - Announce assertive message
// - `is_available()` - Check if screen reader available
// - `active_screen_reader()` - Get active screen reader name

use revue::a11y::{active_screen_reader, BackendType};
use revue::a11y::{announce, announce_now, init, init_with, is_available};
use serial_test::serial;

#[test]
#[serial]
fn test_init_returns_backend() {
    // `init()` calls init_backend with Auto and returns the backend reference
    // Note: Auto may resolve to different backends depending on environment
    let backend = init();
    // Should return a valid backend reference (Auto, Logging, or None)
    let backend_type = backend.backend_type();
    // In test environment with REVUE_A11Y_LOG not set, Auto typically resolves to None or Logging
    assert!(matches!(
        backend_type,
        BackendType::Auto | BackendType::None | BackendType::Logging
    ));
}

#[test]
#[serial]
fn test_init_with_returns_backend() {
    // `init_with()` should return a backend reference
    // Note: OnceLock means if backend already initialized, this returns existing backend
    let backend = init_with(BackendType::Logging);
    let _backend_type = backend.backend_type();
    // Test passes if it doesn't panic
}

#[test]
#[serial]
fn test_announce_convenience_function() {
    // Convenience function should work without importing Priority
    announce("Hello, world!");
    announce("This is a polite announcement");
    // Should not panic
}

#[test]
#[serial]
fn test_announce_now_convenience_function() {
    // Convenience function for assertive announcements
    announce_now("Urgent message!");
    announce_now("Important alert");
    // Should not panic
}

#[test]
#[serial]
fn test_announce_with_string_types() {
    // Accept String
    announce(String::from("Owned string"));
    // Accept &str
    announce("Borrowed string");
    // Accept Cow-like types via Into<String>
    announce(format!("Formatted {}", "message"));
}

#[test]
#[serial]
fn test_announce_now_with_string_types() {
    announce_now(String::from("Urgent owned"));
    announce_now("Urgent borrowed");
    announce_now(format!("Urgent {}", "formatted"));
}

#[test]
#[serial]
fn test_is_available_returns_bool() {
    // Should return a boolean without panicking
    let available = is_available();
    let _ = available; // Type check
}

#[test]
#[serial]
fn test_active_screen_reader_returns_optional() {
    // Should return Option<String> without panicking
    let name = active_screen_reader();
    // Could be Some or None depending on backend type
    let _ = name; // Type check
}

#[test]
#[serial]
fn test_announce_and_is_available() {
    announce("Test message");
    // is_available() should work after announce
    let _available = is_available();
}

#[test]
#[serial]
fn test_announce_and_active_screen_reader() {
    announce("Test message");
    // active_screen_reader() should work after announce
    let _name = active_screen_reader();
}

#[test]
#[serial]
fn test_announce_now_and_is_available() {
    announce_now("Urgent message");
    let _available = is_available();
}

#[test]
#[serial]
fn test_multiple_announces() {
    for i in 0..5 {
        announce(format!("Message {}", i));
    }
}

#[test]
#[serial]
fn test_multiple_announce_nows() {
    for i in 0..5 {
        announce_now(format!("Urgent {}", i));
    }
}

#[test]
#[serial]
fn test_mixed_announcements() {
    announce("Polite 1");
    announce_now("Assertive 1");
    announce("Polite 2");
    announce_now("Assertive 2");
}

#[test]
#[serial]
fn test_convenience_functions_full_workflow() {
    // Full workflow using only convenience functions
    // This tests that all functions work together without panicking

    // Check initial state
    let _available = is_available();
    let _name = active_screen_reader();

    // Make announcements
    announce("Starting workflow");
    announce_now("Important step");

    // Check state after announcements
    let _available_after = is_available();
    let _name_after = active_screen_reader();

    // Complete workflow
    announce("Workflow complete");

    // If we got here without panicking, the test passes
}

#[test]
#[serial]
fn test_init_then_announce() {
    // Initialize backend first
    let backend = init();
    let _ = backend;

    // Then use convenience functions
    announce("After init");
    announce_now("After init now");
}

#[test]
#[serial]
fn test_init_with_then_announce() {
    // Initialize with specific backend
    // Note: May not actually change backend if already initialized
    let backend = init_with(BackendType::Logging);
    let _ = backend;

    // Convenience functions should still work
    announce("After init_with");
    announce_now("After init_with now");
}

#[test]
#[serial]
fn test_convenience_functions_empty_string() {
    announce("");
    announce_now("");
    // Should handle empty strings gracefully
}

#[test]
#[serial]
fn test_convenience_functions_unicode() {
    announce("Hello 세계");
    announce_now("Important 🚨");
    // Should handle unicode gracefully
}

#[test]
#[serial]
fn test_convenience_functions_newlines() {
    announce("Line 1\nLine 2");
    announce_now("Urgent\nMessage");
    // Should handle newlines gracefully
}
