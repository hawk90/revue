//! Screen reader backend - platform-specific implementations

use crate::utils::accessibility::Priority;
use crate::utils::lock::{read_or_recover, write_or_recover};
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
