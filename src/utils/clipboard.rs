//! Clipboard utilities for copy/paste operations
//!
//! Provides cross-platform clipboard access for TUI applications.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::utils::clipboard::{copy, paste, Clipboard};
//!
//! // Simple copy/paste
//! copy("Hello, World!").unwrap();
//! let text = paste().unwrap();
//!
//! // Using clipboard instance
//! let clipboard = Clipboard::new();
//! clipboard.set("Some text").unwrap();
//! let content = clipboard.get().unwrap();
//! ```

use super::lock::lock_or_recover;
use std::io::{self, Write};
use std::process::{Command, Stdio};

/// Clipboard error type
#[derive(Debug)]
pub enum ClipboardError {
    /// No clipboard tool available
    NoClipboardTool,
    /// I/O error
    Io(io::Error),
    /// Command failed
    CommandFailed(String),
    /// Invalid UTF-8 in clipboard content
    InvalidUtf8,
}

impl std::fmt::Display for ClipboardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClipboardError::NoClipboardTool => write!(f, "No clipboard tool available"),
            ClipboardError::Io(e) => write!(f, "I/O error: {}", e),
            ClipboardError::CommandFailed(msg) => write!(f, "Command failed: {}", msg),
            ClipboardError::InvalidUtf8 => write!(f, "Invalid UTF-8 in clipboard content"),
        }
    }
}

impl std::error::Error for ClipboardError {}

impl From<io::Error> for ClipboardError {
    fn from(err: io::Error) -> Self {
        ClipboardError::Io(err)
    }
}

/// Result type for clipboard operations
pub type ClipboardResult<T> = Result<T, ClipboardError>;

/// Clipboard backend trait
pub trait ClipboardBackend {
    /// Copy text to clipboard
    fn set(&self, content: &str) -> ClipboardResult<()>;

    /// Get text from clipboard
    fn get(&self) -> ClipboardResult<String>;

    /// Check if clipboard contains text
    fn has_text(&self) -> ClipboardResult<bool>;

    /// Clear clipboard
    fn clear(&self) -> ClipboardResult<()>;
}

/// Platform-specific clipboard implementation
#[derive(Clone, Debug, Default)]
pub struct SystemClipboard;

impl SystemClipboard {
    /// Create new system clipboard instance
    pub fn new() -> Self {
        Self
    }

    /// Get the appropriate copy command for the platform
    fn copy_command() -> Option<(&'static str, &'static [&'static str])> {
        #[cfg(target_os = "macos")]
        {
            Some(("pbcopy", &[]))
        }

        #[cfg(target_os = "linux")]
        {
            // Try xclip first, then xsel
            if Command::new("xclip").arg("--version").output().is_ok() {
                Some(("xclip", &["-selection", "clipboard"]))
            } else if Command::new("xsel").arg("--version").output().is_ok() {
                Some(("xsel", &["--clipboard", "--input"]))
            } else if Command::new("wl-copy").arg("--version").output().is_ok() {
                Some(("wl-copy", &[]))
            } else {
                None
            }
        }

        #[cfg(target_os = "windows")]
        {
            Some(("clip", &[]))
        }

        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
        {
            None
        }
    }

    /// Get the appropriate paste command for the platform
    fn paste_command() -> Option<(&'static str, &'static [&'static str])> {
        #[cfg(target_os = "macos")]
        {
            Some(("pbpaste", &[]))
        }

        #[cfg(target_os = "linux")]
        {
            // Try xclip first, then xsel
            if Command::new("xclip").arg("--version").output().is_ok() {
                Some(("xclip", &["-selection", "clipboard", "-o"]))
            } else if Command::new("xsel").arg("--version").output().is_ok() {
                Some(("xsel", &["--clipboard", "--output"]))
            } else if Command::new("wl-paste").arg("--version").output().is_ok() {
                Some(("wl-paste", &[]))
            } else {
                None
            }
        }

        #[cfg(target_os = "windows")]
        {
            // Windows paste is more complex, use PowerShell
            Some(("powershell", &["-Command", "Get-Clipboard"]))
        }

        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
        {
            None
        }
    }
}

impl ClipboardBackend for SystemClipboard {
    fn set(&self, content: &str) -> ClipboardResult<()> {
        let (cmd, args) = Self::copy_command().ok_or(ClipboardError::NoClipboardTool)?;

        let mut child = Command::new(cmd)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(content.as_bytes())?;
        }

        let status = child.wait()?;
        if status.success() {
            Ok(())
        } else {
            Err(ClipboardError::CommandFailed(format!(
                "{} exited with status: {:?}",
                cmd,
                status.code()
            )))
        }
    }

    fn get(&self) -> ClipboardResult<String> {
        let (cmd, args) = Self::paste_command().ok_or(ClipboardError::NoClipboardTool)?;

        let output = Command::new(cmd)
            .args(args)
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .output()?;

        if output.status.success() {
            String::from_utf8(output.stdout).map_err(|_| ClipboardError::InvalidUtf8)
        } else {
            Err(ClipboardError::CommandFailed(format!(
                "{} exited with status: {:?}",
                cmd,
                output.status.code()
            )))
        }
    }

    fn has_text(&self) -> ClipboardResult<bool> {
        match self.get() {
            Ok(content) => Ok(!content.is_empty()),
            Err(ClipboardError::NoClipboardTool) => Err(ClipboardError::NoClipboardTool),
            _ => Ok(false),
        }
    }

    fn clear(&self) -> ClipboardResult<()> {
        self.set("")
    }
}

/// In-memory clipboard for testing or sandboxed environments
#[derive(Clone, Debug, Default)]
pub struct MemoryClipboard {
    content: std::sync::Arc<std::sync::Mutex<String>>,
}

impl MemoryClipboard {
    /// Create new in-memory clipboard
    pub fn new() -> Self {
        Self {
            content: std::sync::Arc::new(std::sync::Mutex::new(String::new())),
        }
    }
}

impl ClipboardBackend for MemoryClipboard {
    fn set(&self, content: &str) -> ClipboardResult<()> {
        let mut guard = lock_or_recover(&self.content);
        *guard = content.to_string();
        Ok(())
    }

    fn get(&self) -> ClipboardResult<String> {
        let guard = lock_or_recover(&self.content);
        Ok(guard.clone())
    }

    fn has_text(&self) -> ClipboardResult<bool> {
        let guard = lock_or_recover(&self.content);
        Ok(!guard.is_empty())
    }

    fn clear(&self) -> ClipboardResult<()> {
        let mut guard = lock_or_recover(&self.content);
        guard.clear();
        Ok(())
    }
}

/// Main clipboard type with pluggable backend
pub struct Clipboard {
    backend: Box<dyn ClipboardBackend + Send + Sync>,
}

impl Clipboard {
    /// Create clipboard with system backend
    pub fn new() -> Self {
        Self {
            backend: Box::new(SystemClipboard::new()),
        }
    }

    /// Create clipboard with custom backend
    pub fn with_backend<B: ClipboardBackend + Send + Sync + 'static>(backend: B) -> Self {
        Self {
            backend: Box::new(backend),
        }
    }

    /// Create in-memory clipboard (for testing)
    pub fn memory() -> Self {
        Self {
            backend: Box::new(MemoryClipboard::new()),
        }
    }

    /// Copy text to clipboard
    pub fn set(&self, content: &str) -> ClipboardResult<()> {
        self.backend.set(content)
    }

    /// Get text from clipboard
    pub fn get(&self) -> ClipboardResult<String> {
        self.backend.get()
    }

    /// Check if clipboard contains text
    pub fn has_text(&self) -> ClipboardResult<bool> {
        self.backend.has_text()
    }

    /// Clear clipboard
    pub fn clear(&self) -> ClipboardResult<()> {
        self.backend.clear()
    }
}

impl Default for Clipboard {
    fn default() -> Self {
        Self::new()
    }
}

// Convenience functions using system clipboard

/// Copy text to system clipboard
pub fn copy(content: &str) -> ClipboardResult<()> {
    SystemClipboard::new().set(content)
}

/// Get text from system clipboard
pub fn paste() -> ClipboardResult<String> {
    SystemClipboard::new().get()
}

/// Check if system clipboard has text
pub fn has_text() -> ClipboardResult<bool> {
    SystemClipboard::new().has_text()
}

/// Clear system clipboard
pub fn clear() -> ClipboardResult<()> {
    SystemClipboard::new().clear()
}

/// Clipboard history manager
#[derive(Clone, Debug)]
pub struct ClipboardHistory {
    /// History entries
    entries: Vec<String>,
    /// Maximum history size
    max_size: usize,
}

impl ClipboardHistory {
    /// Create new clipboard history
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: Vec::with_capacity(max_size),
            max_size,
        }
    }

    /// Add entry to history
    pub fn push(&mut self, content: String) {
        // Don't add duplicates at the top
        if self.entries.first() == Some(&content) {
            return;
        }

        // Remove if exists elsewhere (to move to top)
        self.entries.retain(|e| e != &content);

        // Add to front
        self.entries.insert(0, content);

        // Trim to max size
        self.entries.truncate(self.max_size);
    }

    /// Get entry at index (0 = most recent)
    pub fn get(&self, index: usize) -> Option<&str> {
        self.entries.get(index).map(|s| s.as_str())
    }

    /// Get most recent entry
    pub fn latest(&self) -> Option<&str> {
        self.get(0)
    }

    /// Get all entries
    pub fn entries(&self) -> &[String] {
        &self.entries
    }

    /// Get number of entries
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if history is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Clear history
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

impl Default for ClipboardHistory {
    fn default() -> Self {
        Self::new(100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_clipboard() {
        let clipboard = MemoryClipboard::new();

        assert!(!clipboard.has_text().unwrap());

        clipboard.set("Hello").unwrap();
        assert!(clipboard.has_text().unwrap());
        assert_eq!(clipboard.get().unwrap(), "Hello");

        clipboard.clear().unwrap();
        assert!(!clipboard.has_text().unwrap());
    }

    #[test]
    fn test_clipboard_with_memory_backend() {
        let clipboard = Clipboard::memory();

        clipboard.set("Test content").unwrap();
        assert_eq!(clipboard.get().unwrap(), "Test content");
    }

    #[test]
    fn test_clipboard_history() {
        let mut history = ClipboardHistory::new(5);

        history.push("first".to_string());
        history.push("second".to_string());
        history.push("third".to_string());

        assert_eq!(history.latest(), Some("third"));
        assert_eq!(history.get(1), Some("second"));
        assert_eq!(history.get(2), Some("first"));
        assert_eq!(history.len(), 3);
    }

    #[test]
    fn test_clipboard_history_no_duplicates() {
        let mut history = ClipboardHistory::new(5);

        history.push("a".to_string());
        history.push("b".to_string());
        history.push("a".to_string()); // Should move to top

        assert_eq!(history.len(), 2);
        assert_eq!(history.latest(), Some("a"));
        assert_eq!(history.get(1), Some("b"));
    }

    #[test]
    fn test_clipboard_history_max_size() {
        let mut history = ClipboardHistory::new(3);

        history.push("1".to_string());
        history.push("2".to_string());
        history.push("3".to_string());
        history.push("4".to_string()); // Should evict "1"

        assert_eq!(history.len(), 3);
        assert_eq!(history.latest(), Some("4"));
        assert_eq!(history.get(2), Some("2"));
    }

    #[test]
    fn test_clipboard_history_no_duplicate_at_top() {
        let mut history = ClipboardHistory::new(5);

        history.push("same".to_string());
        history.push("same".to_string()); // Should be ignored

        assert_eq!(history.len(), 1);
    }
}
