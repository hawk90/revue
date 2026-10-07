//! Most-recent-first history of copied entries

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
