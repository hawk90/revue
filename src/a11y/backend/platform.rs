//! Screen reader backend - platform-specific implementations

use crate::utils::accessibility::Priority;
use crate::utils::lock::{read_or_recover, write_or_recover};
#[cfg(target_os = "linux")]
use crate::utils::shell::sanitize_string;
use std::process::{Command, ExitStatus, Stdio};
use std::sync::{Arc, RwLock};
use std::thread::JoinHandle;

use super::types::ScreenReader;

/// Run a screen-reader command in the background (#937).
///
/// The command gets none of the app's stdio, so nothing it prints (an
/// AppleScript error, a missing `spd-say`) lands on the terminal UI, and a
/// thread waits for it, so it leaves no zombie behind. Under `cfg(test)`
/// the command is recorded instead of run: tests never start a real
/// speech command.
#[cfg_attr(
    not(any(target_os = "macos", target_os = "windows", target_os = "linux")),
    allow(dead_code)
)]
fn run_detached(cmd: Command) {
    #[cfg(test)]
    tests::record(&cmd);
    #[cfg(not(test))]
    let _ = spawn_reaped(cmd);
}

/// Start `cmd` with null stdio and wait for it on its own thread; the
/// handle yields the exit status.
#[cfg_attr(
    all(
        not(test),
        not(any(target_os = "macos", target_os = "windows", target_os = "linux"))
    ),
    allow(dead_code)
)]
fn spawn_reaped(mut cmd: Command) -> std::io::Result<JoinHandle<std::io::Result<ExitStatus>>> {
    let mut child = cmd
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    std::thread::Builder::new()
        .name("revue-a11y-reaper".to_string())
        .spawn(move || child.wait())
}

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

                let mut cmd = Command::new("osascript");
                cmd.arg("-e").arg(&script);
                run_detached(cmd);
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
        // Like `announce`, nothing to stop without a screen reader (#937)
        #[cfg(target_os = "macos")]
        if self.available {
            let mut cmd = Command::new("osascript");
            cmd.arg("-e")
                .arg("tell application \"VoiceOver\" to stop speaking");
            run_detached(cmd);
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

                // Use PowerShell to trigger announcement via SAPI
                // Using single-quoted string with proper escaping
                let script = format!(
                    "Add-Type -AssemblyName System.Speech; \
                     $synth = New-Object System.Speech.Synthesis.SpeechSynthesizer; \
                     $synth.Speak('{}')",
                    escape_powershell(message)
                );

                let mut cmd = Command::new("powershell");
                cmd.arg("-Command").arg(&script);
                run_detached(cmd);
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
                // Sanitize message to prevent command injection
                let sanitized = sanitize_string(message);

                // Use spd-say (speech-dispatcher) as fallback
                let mut cmd = Command::new("spd-say");

                if priority == Priority::Assertive {
                    cmd.arg("--priority").arg("important");
                }

                cmd.arg(&sanitized);
                run_detached(cmd);
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
        // Like `announce`, nothing to stop without a screen reader (#937)
        #[cfg(target_os = "linux")]
        if self.available {
            let mut cmd = Command::new("spd-say");
            cmd.arg("--cancel");
            run_detached(cmd);
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

// KEEP HERE - Tests access private fields (MacOSBackend.available, WindowsBackend.available, etc.)
// and the private command seam (`run_detached` records under cfg(test))

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    thread_local! {
        /// The commands `run_detached` was given on this thread, as program
        /// then arguments
        static RAN: RefCell<Vec<Vec<String>>> = const { RefCell::new(Vec::new()) };
    }

    /// Record `cmd` in place of running it
    pub(super) fn record(cmd: &Command) {
        let mut line = vec![cmd.get_program().to_string_lossy().into_owned()];
        line.extend(cmd.get_args().map(|a| a.to_string_lossy().into_owned()));
        RAN.with(|ran| ran.borrow_mut().push(line));
    }

    /// The commands recorded on this thread since the last call
    fn take_ran() -> Vec<Vec<String>> {
        RAN.with(|ran| std::mem::take(&mut *ran.borrow_mut()))
    }

    // Commands (#937)

    #[test]
    fn test_unavailable_backends_run_no_command() {
        let _ = take_ran();
        let mac = MacOSBackend { available: false };
        mac.announce("a", Priority::Polite);
        mac.announce("a", Priority::Assertive);
        mac.stop();
        let windows = WindowsBackend { available: false };
        windows.announce("a", Priority::Assertive);
        windows.stop();
        let linux = LinuxBackend { available: false };
        linux.announce("a", Priority::Assertive);
        linux.stop();
        assert_eq!(take_ran(), Vec::<Vec<String>>::new());
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_macos_backend_commands_when_available() {
        let _ = take_ran();
        let backend = MacOSBackend { available: true };
        backend.announce("hi", Priority::Assertive);
        backend.stop();
        assert_eq!(
            take_ran(),
            [
                vec![
                    "osascript".to_string(),
                    "-e".to_string(),
                    "tell application \"VoiceOver\" to output \"hi\"".to_string(),
                ],
                vec![
                    "osascript".to_string(),
                    "-e".to_string(),
                    "tell application \"VoiceOver\" to stop speaking".to_string(),
                ],
            ]
        );
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_linux_backend_commands_when_available() {
        let _ = take_ran();
        let backend = LinuxBackend { available: true };
        backend.announce("hi", Priority::Assertive);
        backend.announce("hi", Priority::Polite);
        backend.stop();
        assert_eq!(
            take_ran(),
            [
                vec!["spd-say", "--priority", "important", "hi"],
                vec!["spd-say", "hi"],
                vec!["spd-say", "--cancel"],
            ]
            .map(|line| line.into_iter().map(String::from).collect::<Vec<_>>())
        );
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_windows_backend_command_when_available() {
        let _ = take_ran();
        let backend = WindowsBackend { available: true };
        backend.announce("hi", Priority::Assertive);
        let ran = take_ran();
        assert_eq!(ran.len(), 1);
        assert_eq!(ran[0][..2], ["powershell", "-Command"]);
        assert!(ran[0][2].ends_with("$synth.Speak('hi')"), "{ran:?}");
    }

    /// The child gets null stdin, stdout and stderr, and the returned
    /// thread reaps it: joining yields its exit status
    #[test]
    #[cfg(unix)]
    fn test_spawn_reaped_nulls_stdio_and_waits() {
        let mut cmd = Command::new("sh");
        cmd.arg("-c").arg(
            "[ /dev/stdin -ef /dev/null ] && [ /dev/stdout -ef /dev/null ] \
             && [ /dev/stderr -ef /dev/null ] && exit 7",
        );
        let status = spawn_reaped(cmd).unwrap().join().unwrap().unwrap();
        assert_eq!(status.code(), Some(7));
    }

    #[test]
    #[cfg(windows)]
    fn test_spawn_reaped_waits() {
        let mut cmd = Command::new("cmd");
        cmd.args(["/C", "exit 7"]);
        let status = spawn_reaped(cmd).unwrap().join().unwrap().unwrap();
        assert_eq!(status.code(), Some(7));
    }

    #[test]
    fn test_spawn_reaped_reports_a_missing_program() {
        let cmd = Command::new("revue-no-such-program-937");
        assert!(spawn_reaped(cmd).is_err());
    }

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

        // Bracket the call rather than asserting a wall-clock budget: on a
        // loaded CI runner "under 100ms" is a coin flip, and the thing worth
        // checking is that the timestamp is taken during `announce` rather
        // than defaulted.
        let before = std::time::Instant::now();
        backend.announce("Test message", Priority::Assertive);
        let after = std::time::Instant::now();

        let announcements = backend.announcements();
        let logged = &announcements[0];

        assert_eq!(logged.message, "Test message");
        assert_eq!(logged.priority, Priority::Assertive);
        assert!(
            logged.timestamp >= before && logged.timestamp <= after,
            "timestamp was not taken during the announce call"
        );
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
    #[cfg(target_os = "linux")]
    fn test_linux_backend_sanitizes_dangerous_input() {
        use crate::utils::shell::sanitize_string;
        let backend = LinuxBackend::new();

        // Should not panic or execute commands
        // Dangerous shell metacharacters are removed
        let dangerous = "Hello; rm -rf /";
        let sanitized = sanitize_string(dangerous);
        assert!(!sanitized.contains(';')); // Semicolon removed (prevents command chaining)
        assert!(!sanitized.contains("`")); // Backticks removed (prevents command substitution)

        // Safe alphanumeric characters and spaces are preserved
        assert!(sanitized.contains("Hello"));
        assert!(sanitized.contains("rm"));
        assert!(sanitized.contains("-rf"));

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
