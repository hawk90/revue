//! Clipboard utilities for copy/paste operations
//!
//! Provides cross-platform clipboard access for TUI applications.
//!
//! # Example
//!
//! Not run as a test: it needs the system clipboard (CI machines have
//! none) and would overwrite yours.
//!
//! ```no_run
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

mod history;
mod memory;
mod system;

use std::io;

pub use history::ClipboardHistory;
pub use memory::MemoryClipboard;
pub use system::SystemClipboard;

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

/// Check and clean content before a built-in backend stores it
///
/// Content over [`MAX_CLIPBOARD_SIZE`](crate::constants::MAX_CLIPBOARD_SIZE)
/// bytes is refused, to prevent DoS. ANSI escape sequences, and control
/// characters other than tab, newline and carriage return, are removed, to
/// prevent terminal injection when the content is pasted.
fn prepare_content(content: &str) -> ClipboardResult<String> {
    use crate::constants::MAX_CLIPBOARD_SIZE;

    if content.len() > MAX_CLIPBOARD_SIZE {
        return Err(ClipboardError::InvalidInput(format!(
            "Clipboard content too large ({} bytes, max {})",
            content.len(),
            MAX_CLIPBOARD_SIZE
        )));
    }

    let stripped = crate::utils::ansi::strip_ansi(content);
    Ok(stripped
        .chars()
        .filter(|&c| (c >= ' ' && c != '\x1b') || c == '\t' || c == '\n' || c == '\r')
        .collect())
}

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
