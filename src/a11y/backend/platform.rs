//! Screen reader backend - platform-specific implementations

use crate::utils::accessibility::Priority;
use crate::utils::lock::{read_or_recover, write_or_recover};
#[cfg(target_os = "linux")]
use crate::utils::shell::sanitize_string;
use std::sync::{Arc, RwLock};

use super::types::ScreenReader;

/// macOS VoiceOver backend
pub struct MacOSBackend {
    available: bool,
}

impl MacOSBackend {
    pub fn new() -> Self {
        Self {
            available: super::detection::is_voiceover_running(),
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
            // Not available, do nothing
        } else {
            // Use NSAccessibility to post notification
            // In a real implementation, this would use objc or cocoa crate
            #[cfg(target_os = "macos")]
            {
                use crate::utils::shell::escape_applescript;
                use std::process::Command;

                // Use osascript to trigger VoiceOver announcement
                let script = if priority == Priority::Assertive {
                    format!(
                        "tell application \"VoiceOver\" to output \"{}\"",
                        escape_applescript(message)
                    )
                } else {
                    format!(
                        "tell application \"System Events\" to set value of attribute \"AXDescription\" of menu bar 1 to \"{}\"",
                        escape_applescript(message)
                    )
                };

                let _ = Command::new("osascript").arg("-e").arg(&script).spawn();
            }
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
            available: super::detection::is_windows_screen_reader_running(),
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
                use crate::utils::shell::escape_powershell;
                use std::process::Command;

                // Use PowerShell to trigger announcement via SAPI
                // Using single-quoted string with proper escaping
                let script = format!(
                    "Add-Type -AssemblyName System.Speech; \
                     $synth = New-Object System.Speech.Synthesis.SpeechSynthesizer; \
                     $synth.Speak('{}')",
                    escape_powershell(message)
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
            available: super::detection::is_atspi_available(),
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

                // Sanitize message to prevent command injection
                let sanitized = sanitize_string(message);

                // Use spd-say (speech-dispatcher) as fallback
                let mut cmd = Command::new("spd-say");

                if priority == Priority::Assertive {
                    cmd.arg("--priority").arg("important");
                }

                cmd.arg(&sanitized);
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
        read_or_recover(&self.announcements).clone()
    }

    /// Clear logged announcements
    pub fn clear(&self) {
        write_or_recover(&self.announcements).clear();
    }

    /// Get the last announcement
    pub fn last(&self) -> Option<LoggedAnnouncement> {
        read_or_recover(&self.announcements).last().cloned()
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

        write_or_recover(&self.announcements).push(announcement);

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
pub struct NullBackend;

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

#[cfg(test)]
mod tests {
    use super::*;

    // MacOSBackend tests
    #[test]
    fn test_macos_backend_new() {
        let backend = MacOSBackend::new();
        // available depends on VoiceOver running, just verify it creates
        let _ = backend.available;
    }

    #[test]
    fn test_macos_backend_default() {
        let backend = MacOSBackend::default();
        let _ = backend.available;
    }

    #[test]
    fn test_macos_backend_is_available() {
        let backend = MacOSBackend::new();
        let _ = backend.is_available();
        // Just verify it doesn't panic
    }

    #[test]
    fn test_macos_backend_active_screen_reader() {
        let backend = MacOSBackend::new();
        let name = backend.active_screen_reader();
        // Should return Some("VoiceOver") if available, None otherwise
        if backend.is_available() {
            assert_eq!(name, Some("VoiceOver".to_string()));
        } else {
            assert!(name.is_none());
        }
    }

    #[test]
    fn test_macos_backend_announce_not_available() {
        let backend = MacOSBackend { available: false };
        // Should not panic when not available
        backend.announce("test", Priority::Polite);
    }

    // WindowsBackend tests
    #[test]
    fn test_windows_backend_new() {
        let backend = WindowsBackend::new();
        let _ = backend.available;
    }

    #[test]
    fn test_windows_backend_default() {
        let backend = WindowsBackend::default();
        let _ = backend.available;
    }

    #[test]
    fn test_windows_backend_is_available() {
        let backend = WindowsBackend::new();
        let _ = backend.is_available();
    }

    #[test]
    fn test_windows_backend_announce_not_available() {
        let backend = WindowsBackend { available: false };
        backend.announce("test", Priority::Polite);
    }

    // LinuxBackend tests
    #[test]
    fn test_linux_backend_new() {
        let backend = LinuxBackend::new();
        let _ = backend.available;
    }

    #[test]
    fn test_linux_backend_default() {
        let backend = LinuxBackend::default();
        let _ = backend.available;
    }

    #[test]
    fn test_linux_backend_is_available() {
        let backend = LinuxBackend::new();
        let _ = backend.is_available();
    }

    #[test]
    fn test_linux_backend_active_screen_reader() {
        let backend = LinuxBackend::new();
        let name = backend.active_screen_reader();
        if backend.is_available() {
            assert_eq!(name, Some("AT-SPI".to_string()));
        } else {
            assert!(name.is_none());
        }
    }

    #[test]
    fn test_linux_backend_announce_not_available() {
        let backend = LinuxBackend { available: false };
        backend.announce("test", Priority::Polite);
    }

    // LoggingBackend tests
    #[test]
    fn test_logging_backend_new() {
        let backend = LoggingBackend::new();
        assert!(backend.announcements().is_empty());
    }

    #[test]
    fn test_logging_backend_default() {
        let backend = LoggingBackend::default();
        assert!(backend.announcements().is_empty());
    }

    #[test]
    fn test_logging_backend_announce() {
        let backend = LoggingBackend::new();
        backend.announce("Hello world", Priority::Polite);

        let announcements = backend.announcements();
        assert_eq!(announcements.len(), 1);
        assert_eq!(announcements[0].message, "Hello world");
        assert_eq!(announcements[0].priority, Priority::Polite);
    }

    #[test]
    fn test_logging_backend_multiple_announces() {
        let backend = LoggingBackend::new();
        backend.announce("Message 1", Priority::Polite);
        backend.announce("Message 2", Priority::Assertive);
        backend.announce("Message 3", Priority::Polite);

        let announcements = backend.announcements();
        assert_eq!(announcements.len(), 3);
        assert_eq!(announcements[0].message, "Message 1");
        assert_eq!(announcements[1].message, "Message 2");
        assert_eq!(announcements[2].message, "Message 3");
    }

    #[test]
    fn test_logging_backend_clear() {
        let backend = LoggingBackend::new();
        backend.announce("Test", Priority::Polite);
        backend.announce("Test 2", Priority::Polite);

        assert_eq!(backend.announcements().len(), 2);

        backend.clear();
        assert!(backend.announcements().is_empty());
    }

    #[test]
    fn test_logging_backend_last() {
        let backend = LoggingBackend::new();
        assert!(backend.last().is_none());

        backend.announce("First", Priority::Polite);
        assert_eq!(backend.last().unwrap().message, "First");

        backend.announce("Second", Priority::Assertive);
        assert_eq!(backend.last().unwrap().message, "Second");
    }

    #[test]
    fn test_logging_backend_is_available() {
        let backend = LoggingBackend::new();
        assert!(backend.is_available());
    }

    #[test]
    fn test_logging_backend_active_screen_reader() {
        let backend = LoggingBackend::new();
        assert_eq!(
            backend.active_screen_reader(),
            Some("LoggingBackend".to_string())
        );
    }

    #[test]
    fn test_logged_announcement_fields() {
        let backend = LoggingBackend::new();
        backend.announce("Test message", Priority::Assertive);

        let announcements = backend.announcements();
        let logged = &announcements[0];

        assert_eq!(logged.message, "Test message");
        assert_eq!(logged.priority, Priority::Assertive);
        // timestamp should be very recent
        let elapsed = logged.timestamp.elapsed().as_millis();
        assert!(elapsed < 100); // Should be less than 100ms
    }

    // NullBackend tests
    #[test]
    fn test_null_backend_announce() {
        let backend = NullBackend;
        // Should not panic
        backend.announce("test", Priority::Polite);
    }

    #[test]
    fn test_null_backend_is_available() {
        let backend = NullBackend;
        assert!(!backend.is_available());
    }

    #[test]
    fn test_null_backend_active_screen_reader() {
        let backend = NullBackend;
        assert!(backend.active_screen_reader().is_none());
    }

    #[test]
    fn test_macos_backend_stop() {
        let backend = MacOSBackend::new();
        // Should not panic
        backend.stop();
    }

    #[test]
    fn test_linux_backend_stop() {
        let backend = LinuxBackend::new();
        // Should not panic
        backend.stop();
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_linux_backend_sanitizes_dangerous_input() {
        use crate::utils::shell::sanitize_string;
        let backend = LinuxBackend::new();

        // Should not panic or execute commands
        // Characters like ;, |, &, backticks should be removed
        let dangerous = "Hello; rm -rf /";
        let sanitized = sanitize_string(dangerous);
        assert!(!sanitized.contains(';'));
        assert!(!sanitized.contains("rm"));

        // announce should not panic even with dangerous input
        backend.announce("Test; command", Priority::Polite);
        backend.announce("Test | pipe", Priority::Polite);
        backend.announce("Test `backtick`", Priority::Polite);
    }

    #[test]
    fn test_windows_backend_active_screen_reader() {
        let backend = WindowsBackend::new();
        let name = backend.active_screen_reader();
        if backend.is_available() {
            assert!(name.is_some());
        } else {
            assert!(name.is_none());
        }
    }
}
