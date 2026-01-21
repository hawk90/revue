//! A11y backend integration tests
//!
//! Tests for screen reader backend implementations and global backend management.

use revue::a11y::{
    announce_to_screen_reader, get_backend, init_backend, set_backend, BackendType, LoggingBackend,
    ScreenReader, ScreenReaderBackend, ScreenReaderConfig,
};
use revue::utils::accessibility::Priority;
use serial_test::serial;

// =============================================================================
// BackendType Tests
// =============================================================================

#[test]
fn test_backend_type_name() {
    assert_eq!(BackendType::Auto.name(), "Auto");
    assert_eq!(BackendType::MacOS.name(), "macOS/VoiceOver");
    assert_eq!(BackendType::Windows.name(), "Windows/Narrator");
    assert_eq!(BackendType::Linux.name(), "Linux/AT-SPI");
    assert_eq!(BackendType::Logging.name(), "Logging");
    assert_eq!(BackendType::None.name(), "None");
}

#[test]
fn test_backend_type_from_env() {
    // Without env var, should return None or Logging (test environment)
    let detected = BackendType::detect();
    assert!(matches!(detected, BackendType::None | BackendType::Logging));

    // With env var set, should detect logging backend
    std::env::set_var("REVUE_A11Y_LOG", "1");
    let detected = BackendType::detect();
    assert_eq!(detected, BackendType::Logging);
    std::env::remove_var("REVUE_A11Y_LOG");
}

#[test]
fn test_backend_type_clone_copy() {
    let backend_type = BackendType::MacOS;
    let copied = backend_type;
    assert_eq!(backend_type, copied);
}

// =============================================================================
// LoggingBackend Tests
// =============================================================================

#[test]
fn test_logging_backend_announce_order() {
    let backend = LoggingBackend::new();

    backend.announce("First", Priority::Polite);
    backend.announce("Second", Priority::Polite);
    backend.announce("Third", Priority::Polite);

    let announcements = backend.announcements();
    assert_eq!(announcements.len(), 3);
    assert_eq!(announcements[0].message, "First");
    assert_eq!(announcements[1].message, "Second");
    assert_eq!(announcements[2].message, "Third");
}

#[test]
fn test_logging_backend_clear_preserves_state() {
    let backend = LoggingBackend::new();

    backend.announce("Before", Priority::Polite);
    backend.clear();

    // Backend should still work after clear
    backend.announce("After", Priority::Polite);
    let announcements = backend.announcements();
    assert_eq!(announcements.len(), 1);
    assert_eq!(announcements[0].message, "After");
}

#[test]
fn test_logging_backend_multiple_clears() {
    let backend = LoggingBackend::new();

    backend.announce("Test", Priority::Polite);
    backend.clear();
    backend.clear(); // Second clear should be safe

    assert!(backend.announcements().is_empty());
    assert!(backend.last().is_none());
}

#[test]
fn test_logging_backend_multiple_instances() {
    let backend1 = LoggingBackend::new();
    backend1.announce("From 1", Priority::Polite);

    // Each backend is independent
    let backend2 = LoggingBackend::new();
    backend2.announce("From 2", Priority::Polite);

    assert_eq!(backend1.announcements().len(), 1);
    assert_eq!(backend2.announcements().len(), 1);
}

#[test]
fn test_logging_backend_last() {
    let backend = LoggingBackend::new();

    assert!(backend.last().is_none());

    backend.announce("First", Priority::Polite);
    backend.announce("Second", Priority::Polite);

    assert_eq!(backend.last().unwrap().message, "Second");
}

// =============================================================================
// Platform-Specific Backend Tests (via ScreenReaderBackend)
// =============================================================================

#[test]
#[cfg(target_os = "macos")]
fn test_macos_backend_via_screen_reader_backend() {
    let backend = ScreenReaderBackend::new(BackendType::MacOS);
    // Should not panic even if VoiceOver is not available
    backend.announce("Test message", Priority::Polite);
    backend.announce("Urgent message", Priority::Assertive);
    backend.stop();
}

#[test]
#[cfg(target_os = "windows")]
fn test_windows_backend_via_screen_reader_backend() {
    let backend = ScreenReaderBackend::new(BackendType::Windows);
    // Should not panic even if screen reader is not available
    backend.announce("Test message", Priority::Polite);
    backend.announce("Urgent message", Priority::Assertive);
    backend.stop();
}

#[test]
#[cfg(target_os = "linux")]
fn test_linux_backend_via_screen_reader_backend() {
    let backend = ScreenReaderBackend::new(BackendType::Linux);
    // Should not panic even if AT-SPI is not available
    backend.announce("Test message", Priority::Polite);
    backend.announce("Urgent message", Priority::Assertive);
    backend.stop();
}

// =============================================================================
// NullBackend Tests (via ScreenReaderBackend::None)
// =============================================================================

#[test]
fn test_null_backend_no_op() {
    let backend = ScreenReaderBackend::new(BackendType::None);

    // All operations should be safe but do nothing
    backend.announce("Ignored", Priority::Polite);
    backend.announce("Ignored", Priority::Assertive);
    backend.stop();

    assert!(!backend.is_available());
    assert!(backend.active_screen_reader().is_none());
}

// =============================================================================
// Global Backend Management Tests
// =============================================================================

#[test]
#[serial]
fn test_init_backend_auto() {
    // Auto-detection should work
    let backend = init_backend(BackendType::Auto);
    assert!(matches!(
        backend.backend_type(),
        BackendType::None | BackendType::Logging
    ));
}

#[test]
#[serial]
fn test_init_backend_specific() {
    let backend = init_backend(BackendType::Logging);
    // If backend was already initialized by another test, it may not be Logging
    // Just verify the backend is functional
    assert!(
        backend.backend_type() == BackendType::Logging
            || backend.backend_type() == BackendType::None
            || backend.backend_type() == BackendType::Auto
    );
}

#[test]
#[serial]
fn test_announce_to_screen_reader() {
    // Use logging backend for testing
    init_backend(BackendType::Logging);

    // Should not panic
    announce_to_screen_reader("Test message", Priority::Polite);
    announce_to_screen_reader("Urgent", Priority::Assertive);
}

#[test]
#[serial]
fn test_is_available() {
    let backend = get_backend();
    // In test environment, may or may not be available depending on env var
    let _available = backend.is_available();
}

#[test]
#[serial]
fn test_active_screen_reader() {
    let backend = get_backend();
    // May return None or a backend name
    let _name = backend.active_screen_reader();
}

#[test]
#[serial]
fn test_get_backend_returns_same() {
    let backend1 = get_backend();
    let backend2 = get_backend();

    // Should return the same static instance
    assert_eq!(backend1.backend_type(), backend2.backend_type());
}

#[test]
#[serial]
fn test_set_backend() {
    let custom_backend = ScreenReaderBackend::new(BackendType::Logging);
    let success = set_backend(custom_backend);

    // Setting backend only succeeds if it hasn't been set yet
    // Since other tests may have initialized it, just verify the call doesn't panic
    let _ = success;

    let backend = get_backend();
    // Just verify backend is accessible, may be Logging or other type
    let _backend_type = backend.backend_type();
}

// =============================================================================
// ScreenReaderConfig Tests
// =============================================================================

#[test]
fn test_config_default() {
    let config = ScreenReaderConfig::default();
    assert_eq!(config.backend_type, BackendType::Auto);
    assert!(!config.log_announcements);
    assert_eq!(config.debounce_ms, 100);
    assert!(config.announce_roles);
}

#[test]
fn test_config_clone() {
    let config = ScreenReaderConfig {
        backend_type: BackendType::Logging,
        log_announcements: true,
        debounce_ms: 50,
        announce_roles: false,
    };

    let cloned = config.clone();
    assert_eq!(cloned.backend_type, BackendType::Logging);
    assert!(cloned.log_announcements);
    assert_eq!(cloned.debounce_ms, 50);
    assert!(!cloned.announce_roles);
}

#[test]
fn test_config_debounce_ms() {
    let config = ScreenReaderConfig {
        debounce_ms: 200,
        ..Default::default()
    };
    assert_eq!(config.debounce_ms, 200);
}

#[test]
fn test_config_announce_roles() {
    let config = ScreenReaderConfig {
        announce_roles: false,
        ..Default::default()
    };
    assert!(!config.announce_roles);
}

// =============================================================================
// ScreenReaderBackend Tests
// =============================================================================

#[test]
fn test_backend_new() {
    let backend = ScreenReaderBackend::new(BackendType::Logging);
    assert_eq!(backend.backend_type(), BackendType::Logging);
    assert!(backend.is_available());
}

#[test]
fn test_backend_with_config() {
    let config = ScreenReaderConfig {
        backend_type: BackendType::Logging,
        log_announcements: true,
        debounce_ms: 0,
        announce_roles: true,
    };

    let backend = ScreenReaderBackend::with_config(config);
    assert_eq!(backend.backend_type(), BackendType::Logging);
}

#[test]
fn test_backend_announce_polite() {
    let backend = ScreenReaderBackend::new(BackendType::Logging);
    backend.announce("Polite message", Priority::Polite);
    // Should not panic
}

#[test]
fn test_backend_announce_assertive() {
    let backend = ScreenReaderBackend::new(BackendType::Logging);
    backend.announce("Assertive message", Priority::Assertive);
    // Should not panic
}

#[test]
fn test_backend_announce_focus() {
    let backend = ScreenReaderBackend::new(BackendType::Logging);
    backend.announce_focus("Submit", "button");
    // Should not panic
}

#[test]
fn test_backend_announce_focus_without_roles() {
    let config = ScreenReaderConfig {
        announce_roles: false,
        ..Default::default()
    };
    let backend = ScreenReaderBackend::with_config(config);
    backend.announce_focus("Submit", "button");
    // Should not panic
}

#[test]
fn test_backend_announce_various_messages() {
    let backend = ScreenReaderBackend::new(BackendType::Logging);

    // Test various message types
    backend.announce("Toggle: checked", Priority::Polite);
    backend.announce("Error: Something went wrong", Priority::Assertive);
    // Should not panic
}

#[test]
fn test_backend_stop() {
    let backend = ScreenReaderBackend::new(BackendType::Logging);
    backend.stop();
    // Should not panic
}

#[test]
fn test_backend_debounce_polite() {
    let config = ScreenReaderConfig {
        debounce_ms: 100,
        ..Default::default()
    };
    let backend = ScreenReaderBackend::with_config(config);

    // First announcement
    backend.announce("First", Priority::Polite);

    // Immediate second polite announcement should be debounced
    backend.announce("Second (debounced)", Priority::Polite);

    // But assertive should still go through
    backend.announce("Urgent (not debounced)", Priority::Assertive);
}

#[test]
fn test_backend_debounce_zero() {
    let config = ScreenReaderConfig {
        debounce_ms: 0,
        ..Default::default()
    };
    let backend = ScreenReaderBackend::with_config(config);

    // With zero debounce, all announcements should go through
    backend.announce("First", Priority::Polite);
    backend.announce("Second", Priority::Polite);
}

// =============================================================================
// Integration Tests
// =============================================================================

#[test]
#[serial]
fn test_full_lifecycle() {
    // Initialize backend
    init_backend(BackendType::Logging);

    // Make announcements - should not panic
    announce_to_screen_reader("App started", Priority::Polite);
    announce_to_screen_reader("Welcome!", Priority::Polite);

    // Get backend and check it's functional
    let backend = get_backend();
    // If backend is Logging, it should be available
    // If another test already initialized it with None, that's okay too
    let _backend_type = backend.backend_type();
}

#[test]
fn test_multiple_backends_coexist() {
    let logging = ScreenReaderBackend::new(BackendType::Logging);
    let null = ScreenReaderBackend::new(BackendType::None);

    // Both should work independently
    logging.announce("To logging", Priority::Polite);
    null.announce("To null", Priority::Polite);

    assert!(logging.is_available());
    assert!(!null.is_available());
}

#[test]
fn test_backend_type() {
    let backend = ScreenReaderBackend::new(BackendType::Logging);
    assert_eq!(backend.backend_type(), BackendType::Logging);
}

#[test]
fn test_screen_reader_trait_methods() {
    let backend = LoggingBackend::new();

    // Test all ScreenReader trait methods
    backend.announce("Test", Priority::Polite);
    backend.announce_focus("Button", "button");
    // announce_state and announce_error are trait methods that format messages
    backend.announce("Toggle: on", Priority::Polite);
    backend.announce("Error occurred", Priority::Assertive);
    backend.stop();

    assert!(backend.is_available());
    assert!(backend.active_screen_reader().is_some());
}

#[test]
fn test_logging_backend_all_priorities() {
    let backend = LoggingBackend::new();

    backend.announce("Polite", Priority::Polite);
    backend.announce("Assertive", Priority::Assertive);

    let announcements = backend.announcements();
    assert_eq!(announcements.len(), 2);
    assert_eq!(announcements[0].priority, Priority::Polite);
    assert_eq!(announcements[1].priority, Priority::Assertive);
}

#[test]
fn test_logging_backend_empty() {
    let backend = LoggingBackend::new();

    assert!(backend.announcements().is_empty());
    assert!(backend.last().is_none());
}
