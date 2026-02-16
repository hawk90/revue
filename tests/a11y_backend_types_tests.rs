//! Tests for a11y::backend::types extracted from src/a11y/backend/types.rs
//!
//! All tests use only public methods and fields from BackendType and ScreenReaderConfig.

use revue::a11y::{BackendType, ScreenReaderConfig};

// =========================================================================
// BackendType::name() tests
// =========================================================================

#[test]
fn test_backend_type_name() {
    assert_eq!(BackendType::Auto.name(), "Auto");
    assert_eq!(BackendType::MacOS.name(), "macOS/VoiceOver");
    assert_eq!(BackendType::Windows.name(), "Windows/Narrator");
    assert_eq!(BackendType::Linux.name(), "Linux/AT-SPI");
    assert_eq!(BackendType::Logging.name(), "Logging");
    assert_eq!(BackendType::None.name(), "None");
}

// =========================================================================
// BackendType::detect() tests
// =========================================================================

#[test]
fn test_backend_type_detect_returns_valid_type() {
    let detected = BackendType::detect();
    // Should return one of the valid variants
    match detected {
        BackendType::Auto
        | BackendType::MacOS
        | BackendType::Windows
        | BackendType::Linux
        | BackendType::Logging
        | BackendType::None => {}
    }
}

#[test]
fn test_backend_type_detect_with_logging_env() {
    // Set the logging environment variable
    std::env::set_var("REVUE_A11Y_LOG", "1");
    let detected = BackendType::detect();
    std::env::remove_var("REVUE_A11Y_LOG");
    assert_eq!(detected, BackendType::Logging);
}

// =========================================================================
// BackendType trait implementations tests
// =========================================================================

#[test]
fn test_backend_type_default() {
    let default = BackendType::default();
    assert_eq!(default, BackendType::Auto);
}

#[test]
fn test_backend_type_clone() {
    let backend = BackendType::MacOS;
    let cloned = backend;
    assert_eq!(backend, cloned);
}

#[test]
fn test_backend_type_equality() {
    assert_eq!(BackendType::Auto, BackendType::Auto);
    assert_eq!(BackendType::MacOS, BackendType::MacOS);
    assert_ne!(BackendType::MacOS, BackendType::Windows);
}

#[test]
fn test_backend_type_debug() {
    let debug_str = format!("{:?}", BackendType::Auto);
    assert!(debug_str.contains("Auto"));
}

// =========================================================================
// ScreenReaderConfig tests
// =========================================================================

#[test]
fn test_screen_reader_config_default() {
    let config = ScreenReaderConfig::default();
    assert_eq!(config.backend_type, BackendType::Auto);
    assert_eq!(config.log_announcements, false);
    assert_eq!(config.debounce_ms, 100);
    assert_eq!(config.announce_roles, true);
}

#[test]
fn test_screen_reader_config_public_fields() {
    let config = ScreenReaderConfig {
        backend_type: BackendType::Logging,
        log_announcements: true,
        debounce_ms: 50,
        announce_roles: false,
    };
    assert_eq!(config.backend_type, BackendType::Logging);
    assert_eq!(config.log_announcements, true);
    assert_eq!(config.debounce_ms, 50);
    assert_eq!(config.announce_roles, false);
}

#[test]
fn test_screen_reader_config_clone() {
    let config = ScreenReaderConfig::default();
    let cloned = config.clone();
    assert_eq!(config.backend_type, cloned.backend_type);
    assert_eq!(config.log_announcements, cloned.log_announcements);
    assert_eq!(config.debounce_ms, cloned.debounce_ms);
    assert_eq!(config.announce_roles, cloned.announce_roles);
}
