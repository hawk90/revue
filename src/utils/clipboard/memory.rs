//! In-memory clipboard backend

use super::{prepare_content, ClipboardBackend, ClipboardResult};
use crate::utils::lock::lock_or_recover;

/// In-memory clipboard for testing or sandboxed environments
///
/// It accepts and stores content as the system clipboard does: oversized
/// content is refused, and ANSI escapes and control characters are stripped.
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
        let content = prepare_content(content)?;
        let mut guard = lock_or_recover(&self.content);
        *guard = content;
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
