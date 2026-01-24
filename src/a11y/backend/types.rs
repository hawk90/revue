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
