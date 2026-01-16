//! Screen reader backend implementations
//!
//! Platform-specific backends for screen reader communication.

use crate::utils::accessibility::Priority;
use std::sync::{Arc, OnceLock, RwLock};

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
            if is_voiceover_running() {
                return BackendType::MacOS;
            }
        }

        #[cfg(target_os = "windows")]
        {
            if is_windows_screen_reader_running() {
                return BackendType::Windows;
            }
        }

        #[cfg(target_os = "linux")]
        {
            if is_atspi_available() {
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

/// Main screen reader backend
pub struct ScreenReaderBackend {
    inner: Box<dyn ScreenReader>,
    config: ScreenReaderConfig,
    last_announcement: RwLock<Option<std::time::Instant>>,
}

impl ScreenReaderBackend {
    /// Create a new backend with the given type
    pub fn new(backend_type: BackendType) -> Self {
        let resolved_type = if backend_type == BackendType::Auto {
            BackendType::detect()
        } else {
            backend_type
        };

        let inner: Box<dyn ScreenReader> = match resolved_type {
            BackendType::MacOS => Box::new(MacOSBackend::new()),
            BackendType::Windows => Box::new(WindowsBackend::new()),
            BackendType::Linux => Box::new(LinuxBackend::new()),
            BackendType::Logging => Box::new(LoggingBackend::new()),
            BackendType::None | BackendType::Auto => Box::new(NullBackend),
        };

        Self {
            inner,
            config: ScreenReaderConfig {
                backend_type: resolved_type,
                ..Default::default()
            },
            last_announcement: RwLock::new(None),
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: ScreenReaderConfig) -> Self {
        let mut backend = Self::new(config.backend_type);
        backend.config = config;
        backend
    }

    /// Get the backend type
    pub fn backend_type(&self) -> BackendType {
        self.config.backend_type
    }

    /// Announce a message
    pub fn announce(&self, message: impl AsRef<str>, priority: Priority) {
        let message = message.as_ref();

        // Debounce check
        if self.config.debounce_ms > 0 {
            let now = std::time::Instant::now();
            let mut last = self.last_announcement.write().unwrap();

            if let Some(last_time) = *last {
                let elapsed = now.duration_since(last_time).as_millis() as u64;
                if elapsed < self.config.debounce_ms && priority == Priority::Polite {
                    return;
                }
            }

            *last = Some(now);
        }

        // Log if enabled
        if self.config.log_announcements {
            let priority_str = match priority {
                Priority::Polite => "polite",
                Priority::Assertive => "assertive",
            };
            eprintln!("[a11y:{}] {}", priority_str, message);
        }

        self.inner.announce(message, priority);
    }

    /// Check if screen reader is available
    pub fn is_available(&self) -> bool {
        self.inner.is_available()
    }

    /// Get active screen reader name
    pub fn active_screen_reader(&self) -> Option<String> {
        self.inner.active_screen_reader()
    }

    /// Announce focus change
    pub fn announce_focus(&self, label: &str, role: &str) {
        if self.config.announce_roles {
            self.inner.announce_focus(label, role);
        } else {
            self.inner.announce(label, Priority::Polite);
        }
    }

    /// Stop current speech
    pub fn stop(&self) {
        self.inner.stop();
    }
}

// =============================================================================
// Platform-specific backends
// =============================================================================

/// macOS VoiceOver backend
pub struct MacOSBackend {
    available: bool,
}

impl MacOSBackend {
    pub fn new() -> Self {
        Self {
            available: is_voiceover_running(),
        }
    }
}

impl Default for MacOSBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenReader for MacOSBackend {
    #[allow(unused_variables)]
    fn announce(&self, message: &str, priority: Priority) {
        if !self.available {
            return;
        }

        // Use NSAccessibility to post notification
        // In a real implementation, this would use objc or cocoa crate
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;

            // Use osascript to trigger VoiceOver announcement
            let script = if priority == Priority::Assertive {
                format!(
                    "tell application \"VoiceOver\" to output \"{}\"",
                    message.replace('"', "\\\"")
                )
            } else {
                format!(
                    "tell application \"System Events\" to set value of attribute \"AXDescription\" of menu bar 1 to \"{}\"",
                    message.replace('"', "\\\"")
                )
            };

            let _ = Command::new("osascript").arg("-e").arg(&script).spawn();
        }
    }

    fn is_available(&self) -> bool {
        self.available
    }

    fn active_screen_reader(&self) -> Option<String> {
        if self.available {
            Some("VoiceOver".to_string())
        } else {
            None
        }
    }

    fn stop(&self) {
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            let _ = Command::new("osascript")
                .arg("-e")
                .arg("tell application \"VoiceOver\" to stop speaking")
                .spawn();
        }
    }
}

/// Windows Narrator/NVDA backend
pub struct WindowsBackend {
    available: bool,
}

impl WindowsBackend {
    pub fn new() -> Self {
        Self {
            available: is_windows_screen_reader_running(),
        }
    }
}

impl Default for WindowsBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenReader for WindowsBackend {
    #[allow(unused_variables)]
    fn announce(&self, message: &str, priority: Priority) {
        if !self.available {
            // Not available, do nothing
        } else {
            // Use UI Automation to announce
            // In a real implementation, this would use windows-rs crate
            #[cfg(target_os = "windows")]
            {
                use std::process::Command;

                // Use PowerShell to trigger announcement via SAPI
                let script = format!(
                    "Add-Type -AssemblyName System.Speech; \
                     $synth = New-Object System.Speech.Synthesis.SpeechSynthesizer; \
                     $synth.Speak('{}')",
                    message.replace('\'', "''")
                );

                let _ = Command::new("powershell")
                    .arg("-Command")
                    .arg(&script)
                    .spawn();
            }
        }
    }

    fn is_available(&self) -> bool {
        self.available
    }

    fn active_screen_reader(&self) -> Option<String> {
        if self.available {
            // Could detect specific screen reader (NVDA, JAWS, Narrator)
            Some("Windows Screen Reader".to_string())
        } else {
            None
        }
    }
}

/// Linux AT-SPI backend (Orca, etc.)
pub struct LinuxBackend {
    available: bool,
}

impl LinuxBackend {
    pub fn new() -> Self {
        Self {
            available: is_atspi_available(),
        }
    }
}

impl Default for LinuxBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenReader for LinuxBackend {
    #[allow(unused_variables)]
    fn announce(&self, message: &str, priority: Priority) {
        if !self.available {
            // Not available, do nothing
        } else {
            // Use AT-SPI D-Bus interface
            // In a real implementation, this would use atspi or zbus crate
            #[cfg(target_os = "linux")]
            {
                use std::process::Command;

                // Use spd-say (speech-dispatcher) as fallback
                let mut cmd = Command::new("spd-say");

                if priority == Priority::Assertive {
                    cmd.arg("--priority").arg("important");
                }

                cmd.arg(message);
                let _ = cmd.spawn();
            }
        }
    }

    fn is_available(&self) -> bool {
        self.available
    }

    fn active_screen_reader(&self) -> Option<String> {
        if self.available {
            // Could detect Orca specifically
            Some("AT-SPI".to_string())
        } else {
            None
        }
    }

    fn stop(&self) {
        #[cfg(target_os = "linux")]
        {
            use std::process::Command;
            let _ = Command::new("spd-say").arg("--cancel").spawn();
        }
    }
}

/// Logging backend for testing
pub struct LoggingBackend {
    announcements: Arc<RwLock<Vec<LoggedAnnouncement>>>,
}

/// A logged announcement for testing
#[derive(Clone, Debug)]
pub struct LoggedAnnouncement {
    /// The message that was announced
    pub message: String,
    /// The priority of the announcement
    pub priority: Priority,
    /// When the announcement was made
    pub timestamp: std::time::Instant,
}

impl LoggingBackend {
    /// Create a new logging backend
    pub fn new() -> Self {
        Self {
            announcements: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Get all logged announcements
    pub fn announcements(&self) -> Vec<LoggedAnnouncement> {
        self.announcements.read().unwrap().clone()
    }

    /// Clear logged announcements
    pub fn clear(&self) {
        self.announcements.write().unwrap().clear();
    }

    /// Get the last announcement
    pub fn last(&self) -> Option<LoggedAnnouncement> {
        self.announcements.read().unwrap().last().cloned()
    }
}

impl Default for LoggingBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenReader for LoggingBackend {
    fn announce(&self, message: &str, priority: Priority) {
        let announcement = LoggedAnnouncement {
            message: message.to_string(),
            priority,
            timestamp: std::time::Instant::now(),
        };

        self.announcements.write().unwrap().push(announcement);

        // Also print to stderr for visibility
        let priority_str = match priority {
            Priority::Polite => "polite",
            Priority::Assertive => "ASSERTIVE",
        };
        eprintln!("[ScreenReader:{}] {}", priority_str, message);
    }

    fn is_available(&self) -> bool {
        true
    }

    fn active_screen_reader(&self) -> Option<String> {
        Some("LoggingBackend".to_string())
    }
}

/// Null backend (no-op)
struct NullBackend;

impl ScreenReader for NullBackend {
    fn announce(&self, _message: &str, _priority: Priority) {
        // No-op
    }

    fn is_available(&self) -> bool {
        false
    }

    fn active_screen_reader(&self) -> Option<String> {
        None
    }
}

// =============================================================================
// Platform detection helpers
// =============================================================================

/// Check if VoiceOver is running on macOS
fn is_voiceover_running() -> bool {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;

        // Check if VoiceOver process is running
        if let Ok(output) = Command::new("pgrep").arg("-x").arg("VoiceOver").output() {
            return output.status.success();
        }

        // Alternative: check defaults
        if let Ok(output) = Command::new("defaults")
            .arg("read")
            .arg("com.apple.universalaccess")
            .arg("voiceOverOnOffKey")
            .output()
        {
            if let Ok(stdout) = String::from_utf8(output.stdout) {
                return stdout.trim() == "1";
            }
        }
    }

    false
}

/// Check if a Windows screen reader is running
fn is_windows_screen_reader_running() -> bool {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;

        // Check for common screen readers
        let screen_readers = ["nvda", "narrator", "jfw"]; // JAWS

        for sr in &screen_readers {
            if let Ok(output) = Command::new("tasklist")
                .arg("/FI")
                .arg(format!("IMAGENAME eq {}.exe", sr))
                .output()
            {
                if let Ok(stdout) = String::from_utf8(output.stdout) {
                    if stdout.contains(sr) {
                        return true;
                    }
                }
            }
        }
    }

    false
}

/// Check if AT-SPI is available on Linux
fn is_atspi_available() -> bool {
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;

        // Check if speech-dispatcher is available
        if let Ok(output) = Command::new("which").arg("spd-say").output() {
            if output.status.success() {
                return true;
            }
        }

        // Check for Orca
        if let Ok(output) = Command::new("pgrep").arg("-x").arg("orca").output() {
            return output.status.success();
        }

        // Check D-Bus for AT-SPI registry
        if let Ok(output) = Command::new("dbus-send")
            .arg("--session")
            .arg("--print-reply")
            .arg("--dest=org.a11y.Bus")
            .arg("/org/a11y/bus")
            .arg("org.freedesktop.DBus.Properties.Get")
            .arg("string:org.a11y.Status")
            .arg("string:IsEnabled")
            .output()
        {
            if let Ok(stdout) = String::from_utf8(output.stdout) {
                return stdout.contains("true");
            }
        }
    }

    false
}

// =============================================================================
// Global backend management
// =============================================================================

/// Global screen reader backend
static BACKEND: OnceLock<ScreenReaderBackend> = OnceLock::new();

/// Initialize the global backend
pub fn init_backend(backend_type: BackendType) -> &'static ScreenReaderBackend {
    BACKEND.get_or_init(|| ScreenReaderBackend::new(backend_type))
}

/// Get the global backend (initializes with auto-detect if needed)
pub fn get_backend() -> &'static ScreenReaderBackend {
    BACKEND.get_or_init(|| ScreenReaderBackend::new(BackendType::Auto))
}

/// Set a custom backend (must be called before any other backend functions)
pub fn set_backend(backend: ScreenReaderBackend) -> bool {
    BACKEND.set(backend).is_ok()
}

/// Announce to the global screen reader
pub fn announce_to_screen_reader(message: impl Into<String>, priority: Priority) {
    get_backend().announce(message.into(), priority);
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
