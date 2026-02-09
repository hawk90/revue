//! Screen reader backend - type definitions

use crate::utils::accessibility::Priority;

/// Screen reader backend type
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BackendType {
    /// Auto-detect based on platform
    #[default]
    Auto,
    /// macOS VoiceOver
    MacOS,
    /// Windows Narrator/NVDA
    Windows,
    /// Linux AT-SPI (Orca, etc.)
    Linux,
    /// Testing/logging backend
    Logging,
    /// No screen reader (silent)
    None,
}

impl BackendType {
    /// Detect the appropriate backend for the current platform
    pub fn detect() -> Self {
        #[cfg(target_os = "macos")]
        {
            if super::detection::is_voiceover_running() {
                return BackendType::MacOS;
            }
        }

        #[cfg(target_os = "windows")]
        {
            if super::detection::is_windows_screen_reader_running() {
                return BackendType::Windows;
            }
        }

        #[cfg(target_os = "linux")]
        {
            if super::detection::is_atspi_available() {
                return BackendType::Linux;
            }
        }

        // Check for testing mode
        if std::env::var("REVUE_A11Y_LOG").is_ok() {
            return BackendType::Logging;
        }

        BackendType::None
    }

    /// Get backend name
    pub fn name(&self) -> &'static str {
        match self {
            BackendType::Auto => "Auto",
            BackendType::MacOS => "macOS/VoiceOver",
            BackendType::Windows => "Windows/Narrator",
            BackendType::Linux => "Linux/AT-SPI",
            BackendType::Logging => "Logging",
            BackendType::None => "None",
        }
    }
}

/// Screen reader trait for platform-specific implementations
pub trait ScreenReader: Send + Sync {
    /// Announce a message to the screen reader
    fn announce(&self, message: &str, priority: Priority);

    /// Check if screen reader is available
    fn is_available(&self) -> bool;

    /// Get the name of the active screen reader
    fn active_screen_reader(&self) -> Option<String>;

    /// Announce focus change
    fn announce_focus(&self, label: &str, role: &str) {
        self.announce(&format!("{}, {}", label, role), Priority::Polite);
    }

    /// Announce state change
    fn announce_state(&self, label: &str, state: &str) {
        self.announce(&format!("{}: {}", label, state), Priority::Polite);
    }

    /// Announce error
    fn announce_error(&self, message: &str) {
        self.announce(&format!("Error: {}", message), Priority::Assertive);
    }

    /// Stop any ongoing speech
    fn stop(&self) {}
}

/// Screen reader backend configuration
#[derive(Clone, Debug)]
pub struct ScreenReaderConfig {
    /// Backend type to use
    pub backend_type: BackendType,
    /// Whether to log all announcements
    pub log_announcements: bool,
    /// Minimum time between announcements (to prevent spam)
    pub debounce_ms: u64,
    /// Whether to include role in focus announcements
    pub announce_roles: bool,
}

impl Default for ScreenReaderConfig {
    fn default() -> Self {
        Self {
            backend_type: BackendType::Auto,
            log_announcements: false,
            debounce_ms: 100,
            announce_roles: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    // =========================================================================
    // Additional backend types tests
    // =========================================================================

    #[test]
    fn test_backend_type_debug() {
        let debug_str = format!("{:?}", BackendType::Auto);
        assert!(debug_str.contains("Auto"));
    }

    #[test]
    fn test_backend_type_all_variants_distinct() {
        let variants = [
            BackendType::Auto,
            BackendType::MacOS,
            BackendType::Windows,
            BackendType::Linux,
            BackendType::Logging,
            BackendType::None,
        ];
        for (i, v1) in variants.iter().enumerate() {
            for (j, v2) in variants.iter().enumerate() {
                if i != j {
                    assert_ne!(v1, v2);
                }
            }
        }
    }

    #[test]
    fn test_backend_type_copy_trait() {
        let backend = BackendType::MacOS;
        let copied = backend; // Copy trait
        assert_eq!(backend, copied);
        assert_eq!(backend, BackendType::MacOS);
    }

    #[test]
    fn test_backend_type_name_all() {
        assert_eq!(BackendType::Auto.name(), "Auto");
        assert_eq!(BackendType::MacOS.name(), "macOS/VoiceOver");
        assert_eq!(BackendType::Windows.name(), "Windows/Narrator");
        assert_eq!(BackendType::Linux.name(), "Linux/AT-SPI");
        assert_eq!(BackendType::Logging.name(), "Logging");
        assert_eq!(BackendType::None.name(), "None");
    }

    #[test]
    fn test_backend_type_partial_eq() {
        let b1 = BackendType::Logging;
        let b2 = BackendType::Logging;
        let b3 = BackendType::None;
        assert_eq!(b1, b2);
        assert_ne!(b1, b3);
    }

    #[test]
    fn test_screen_reader_config_with_zero_debounce() {
        let config = ScreenReaderConfig {
            backend_type: BackendType::Auto,
            log_announcements: false,
            debounce_ms: 0,
            announce_roles: true,
        };
        assert_eq!(config.debounce_ms, 0);
    }

    #[test]
    fn test_screen_reader_config_with_large_debounce() {
        let config = ScreenReaderConfig {
            backend_type: BackendType::Auto,
            log_announcements: false,
            debounce_ms: 10000,
            announce_roles: true,
        };
        assert_eq!(config.debounce_ms, 10000);
    }

    #[test]
    fn test_screen_reader_config_all_backend_types() {
        for backend_type in &[
            BackendType::Auto,
            BackendType::MacOS,
            BackendType::Windows,
            BackendType::Linux,
            BackendType::Logging,
            BackendType::None,
        ] {
            let config = ScreenReaderConfig {
                backend_type: *backend_type,
                ..Default::default()
            };
            assert_eq!(config.backend_type, *backend_type);
        }
    }

    #[test]
    fn test_screen_reader_config_debug() {
        let config = ScreenReaderConfig::default();
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("ScreenReaderConfig"));
    }

    #[test]
    fn test_backend_type_detect_returns_backend_type() {
        let detected = BackendType::detect();
        // Just verify it returns without panic
        let _ = detected;
    }
}
