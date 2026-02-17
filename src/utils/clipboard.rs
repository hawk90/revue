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
use crate::constants::MAX_CLIPBOARD_SIZE;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::sync::OnceLock;

/// Sanitize clipboard content by removing dangerous characters
fn sanitize_clipboard_content(content: &str) -> String {
    content
        .chars()
        .filter(|&c| {
            // Keep printable characters and common whitespace
            // Filter out null bytes and other control characters except:
            // - \t (tab), \n (newline), \r (carriage return)
            c >= ' ' || c == '\t' || c == '\n' || c == '\r'
        })
        .collect()
}

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
    /// Invalid input (e.g., too large)
    InvalidInput(String),
}

impl std::fmt::Display for ClipboardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClipboardError::NoClipboardTool => write!(f, "No clipboard tool available"),
            ClipboardError::Io(e) => write!(f, "I/O error: {}", e),
            ClipboardError::CommandFailed(msg) => write!(f, "Command failed: {}", msg),
            ClipboardError::InvalidUtf8 => write!(f, "Invalid UTF-8 in clipboard content"),
            ClipboardError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
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

/// Cached clipboard command detection to avoid repeated subprocess spawning
static COPY_COMMAND_CACHE: OnceLock<Option<(&'static str, &'static [&'static str])>> =
    OnceLock::new();
static PASTE_COMMAND_CACHE: OnceLock<Option<(&'static str, &'static [&'static str])>> =
    OnceLock::new();

/// Platform-specific clipboard implementation
#[derive(Clone, Debug, Default)]
pub struct SystemClipboard;

impl SystemClipboard {
    /// Create new system clipboard instance
    pub fn new() -> Self {
        Self
    }

    /// Get the appropriate copy command for the platform (cached)
    fn copy_command() -> Option<(&'static str, &'static [&'static str])> {
        *COPY_COMMAND_CACHE.get_or_init(|| {
            #[cfg(target_os = "macos")]
            {
                Some(("pbcopy", &[]))
            }

            #[cfg(target_os = "linux")]
            {
                // Try xclip first, then xsel, then wl-copy
                // Use direct command execution instead of shell to avoid injection risk
                let check_cmd = |cmd: &str| -> bool {
                    // Most clipboard tools support --version or --help for availability check
                    // Try to execute the command directly without shell
                    Command::new(cmd)
                        .arg("--version")
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .status()
                        .map(|s| s.success())
                        .unwrap_or(false)
                };

                if check_cmd("xclip") {
                    Some(("xclip", &["-selection", "clipboard"]))
                } else if check_cmd("xsel") {
                    Some(("xsel", &["--clipboard", "--input"]))
                } else if check_cmd("wl-copy") {
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
        })
    }

    /// Get the appropriate paste command for the platform (cached)
    fn paste_command() -> Option<(&'static str, &'static [&'static str])> {
        *PASTE_COMMAND_CACHE.get_or_init(|| {
            #[cfg(target_os = "macos")]
            {
                Some(("pbpaste", &[]))
            }

            #[cfg(target_os = "linux")]
            {
                // Try xclip first, then xsel, then wl-paste
                // Use direct command execution instead of shell to avoid injection risk
                let check_cmd = |cmd: &str| -> bool {
                    // Most clipboard tools support --version or --help for availability check
                    // Try to execute the command directly without shell
                    Command::new(cmd)
                        .arg("--version")
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .status()
                        .map(|s| s.success())
                        .unwrap_or(false)
                };

                if check_cmd("xclip") {
                    Some(("xclip", &["-selection", "clipboard", "-o"]))
                } else if check_cmd("xsel") {
                    Some(("xsel", &["--clipboard", "--output"]))
                } else if check_cmd("wl-paste") {
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
        })
    }
}

impl ClipboardBackend for SystemClipboard {
    fn set(&self, content: &str) -> ClipboardResult<()> {
        // Validate size to prevent DoS
        if content.len() > MAX_CLIPBOARD_SIZE {
            return Err(ClipboardError::InvalidInput(format!(
                "Clipboard content too large ({} bytes, max {})",
                content.len(),
                MAX_CLIPBOARD_SIZE
            )));
        }

        // Sanitize content to remove dangerous control characters
        let sanitized = sanitize_clipboard_content(content);

        let (cmd, args) = Self::copy_command().ok_or(ClipboardError::NoClipboardTool)?;

        let mut child = Command::new(cmd)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(sanitized.as_bytes())?;
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
            // Validate size to prevent DoS through large clipboard content
            if output.stdout.len() > MAX_CLIPBOARD_SIZE {
                return Err(ClipboardError::InvalidInput(format!(
                    "Clipboard content too large: {} bytes (max: {})",
                    output.stdout.len(),
                    MAX_CLIPBOARD_SIZE
                )));
            }
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
    ///
    /// # Errors
    ///
    /// Returns `Err(ClipboardError)` if the clipboard backend cannot set the content.
    pub fn set(&self, content: &str) -> ClipboardResult<()> {
        self.backend.set(content)
    }

    /// Get text from clipboard
    ///
    /// # Errors
    ///
    /// Returns `Err(ClipboardError)` if the clipboard backend cannot access the content.
    pub fn get(&self) -> ClipboardResult<String> {
        self.backend.get()
    }

    /// Check if clipboard contains text
    ///
    /// # Errors
    ///
    /// Returns `Err(ClipboardError)` if the clipboard backend cannot be accessed.
    pub fn has_text(&self) -> ClipboardResult<bool> {
        self.backend.has_text()
    }

    /// Clear clipboard
    ///
    /// # Errors
    ///
    /// Returns `Err(ClipboardError)` if the clipboard backend cannot clear the content.
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
///
/// # Errors
///
/// Returns `Err(ClipboardError)` if the system clipboard cannot be accessed or set.
pub fn copy(content: &str) -> ClipboardResult<()> {
    SystemClipboard::new().set(content)
}

/// Get text from system clipboard
///
/// # Errors
///
/// Returns `Err(ClipboardError)` if the system clipboard cannot be accessed.
pub fn paste() -> ClipboardResult<String> {
    SystemClipboard::new().get()
}

/// Check if system clipboard has text
///
/// # Errors
///
/// Returns `Err(ClipboardError)` if the system clipboard cannot be accessed.
pub fn has_text() -> ClipboardResult<bool> {
    SystemClipboard::new().has_text()
}

/// Clear system clipboard
///
/// # Errors
///
/// Returns `Err(ClipboardError)` if the system clipboard cannot be cleared.
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
