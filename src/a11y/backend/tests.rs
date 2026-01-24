use crate::a11y::backend::platform::{
    LinuxBackend, LoggedAnnouncement, MacOSBackend, NullBackend, WindowsBackend,
};
use crate::a11y::backend::{BackendType, LoggingBackend, ScreenReaderBackend, ScreenReaderConfig};
use crate::utils::accessibility::Priority;

#[test]
fn test_backend_type_detect() {
    // In test environment, should return None or Logging
    let detected = BackendType::detect();
    assert!(matches!(detected, BackendType::None | BackendType::Logging));
}

#[test]
fn test_backend_type_name() {
    assert_eq!(BackendType::MacOS.name(), "macOS/VoiceOver");
    assert_eq!(BackendType::Windows.name(), "Windows/Narrator");
    assert_eq!(BackendType::Linux.name(), "Linux/AT-SPI");
    assert_eq!(BackendType::Logging.name(), "Logging");
}

#[test]
fn test_logging_backend() {
    let backend = LoggingBackend::new();

    backend.announce("Test message", Priority::Polite);
    backend.announce("Urgent!", Priority::Assertive);

    let announcements = backend.announcements();
    assert_eq!(announcements.len(), 2);
    assert_eq!(announcements[0].message, "Test message");
    assert_eq!(announcements[0].priority, Priority::Polite);
    assert_eq!(announcements[1].message, "Urgent!");
    assert_eq!(announcements[1].priority, Priority::Assertive);
}

#[test]
fn test_logging_backend_clear() {
    let backend = LoggingBackend::new();

    backend.announce("Test", Priority::Polite);
    assert_eq!(backend.announcements().len(), 1);

    backend.clear();
    assert!(backend.announcements().is_empty());
}

#[test]
fn test_logging_backend_last() {
    let backend = LoggingBackend::new();

    assert!(backend.last().is_none());

    backend.announce("First", Priority::Polite);
    backend.announce("Second", Priority::Polite);

    assert_eq!(backend.last().unwrap().message, "Second");
}

#[test]
fn test_null_backend() {
    let backend = NullBackend;

    backend.announce("Test", Priority::Polite);
    assert!(!backend.is_available());
    assert!(backend.active_screen_reader().is_none());
}

#[test]
fn test_screen_reader_backend_new() {
    let backend = ScreenReaderBackend::new(BackendType::Logging);
    assert_eq!(backend.backend_type(), BackendType::Logging);
    assert!(backend.is_available());
}

#[test]
fn test_screen_reader_backend_announce() {
    let backend = ScreenReaderBackend::new(BackendType::Logging);

    backend.announce("Test message", Priority::Polite);
    // Logging backend prints to stderr, so we just verify no panic
}

#[test]
fn test_screen_reader_config_default() {
    let config = ScreenReaderConfig::default();
    assert_eq!(config.backend_type, BackendType::Auto);
    assert!(!config.log_announcements);
    assert_eq!(config.debounce_ms, 100);
    assert!(config.announce_roles);
}

#[test]
fn test_screen_reader_backend_with_config() {
    let config = ScreenReaderConfig {
        backend_type: BackendType::Logging,
        log_announcements: true,
        debounce_ms: 0,
        announce_roles: false,
    };

    let backend = ScreenReaderBackend::with_config(config);
    assert_eq!(backend.backend_type(), BackendType::Logging);
}

#[test]
fn test_macos_backend_new() {
    let backend = MacOSBackend::new();
    // Should be unavailable in test environment (not macOS or no VoiceOver)
    #[cfg(not(target_os = "macos"))]
    assert!(!backend.is_available());
    #[cfg(target_os = "macos")]
    let _ = backend.is_available(); // Just check it doesn't panic
}

#[test]
fn test_windows_backend_new() {
    let backend = WindowsBackend::new();
    #[cfg(not(target_os = "windows"))]
    assert!(!backend.is_available());
    #[cfg(target_os = "windows")]
    let _ = backend.is_available(); // Just check it doesn't panic
}

#[test]
fn test_linux_backend_new() {
    let backend = LinuxBackend::new();
    // May or may not be available depending on environment
    let _ = backend.is_available();
}

#[test]
fn test_announce_focus() {
    let backend = ScreenReaderBackend::new(BackendType::Logging);
    backend.announce_focus("Submit", "button");
    // Should announce "Submit, button"
}

#[test]
fn test_debounce() {
    let config = ScreenReaderConfig {
        backend_type: BackendType::Logging,
        debounce_ms: 1000, // 1 second
        ..Default::default()
    };

    let backend = ScreenReaderBackend::with_config(config);

    // First announcement should go through
    backend.announce("First", Priority::Polite);

    // Second polite announcement within debounce window should be skipped
    backend.announce("Second", Priority::Polite);

    // But assertive should always go through
    backend.announce("Urgent", Priority::Assertive);
}

#[test]
fn test_logged_announcement() {
    let announcement = LoggedAnnouncement {
        message: "Test".to_string(),
        priority: Priority::Polite,
        timestamp: std::time::Instant::now(),
    };

    assert_eq!(announcement.message, "Test");
    assert_eq!(announcement.priority, Priority::Polite);
}
