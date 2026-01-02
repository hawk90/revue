//! Snapshot testing utilities
//!
//! Allows capturing and comparing widget output against golden files.

use crate::render::Buffer;
use std::fs;
use std::path::PathBuf;

/// Snapshot manager for storing and comparing test snapshots
pub struct SnapshotManager {
    /// Base directory for snapshots
    snapshot_dir: PathBuf,
    /// Whether to update snapshots instead of comparing
    update_mode: bool,
}

impl SnapshotManager {
    /// Create a new snapshot manager
    ///
    /// Snapshots are stored in `tests/snapshots/` by default.
    /// Set `REVUE_UPDATE_SNAPSHOTS=1` environment variable to update snapshots.
    pub fn new() -> Self {
        let snapshot_dir = PathBuf::from("tests/snapshots");
        let update_mode = std::env::var("REVUE_UPDATE_SNAPSHOTS")
            .map(|v| v == "1" || v.to_lowercase() == "true")
            .unwrap_or(false);

        Self {
            snapshot_dir,
            update_mode,
        }
    }

    /// Create with custom snapshot directory
    pub fn with_dir(dir: impl Into<PathBuf>) -> Self {
        let update_mode = std::env::var("REVUE_UPDATE_SNAPSHOTS")
            .map(|v| v == "1" || v.to_lowercase() == "true")
            .unwrap_or(false);

        Self {
            snapshot_dir: dir.into(),
            update_mode,
        }
    }

    /// Get snapshot file path
    fn snapshot_path(&self, name: &str) -> PathBuf {
        self.snapshot_dir.join(format!("{}.snap", name))
    }

    /// Assert snapshot matches or create if not exists
    pub fn assert_snapshot(&self, name: &str, content: &str) {
        let path = self.snapshot_path(name);

        // Create directory if needed
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .unwrap_or_else(|e| panic!("Failed to create snapshot directory: {}", e));
            }
        }

        if self.update_mode {
            // Update mode: write snapshot
            fs::write(&path, content)
                .unwrap_or_else(|e| panic!("Failed to write snapshot '{}': {}", name, e));
            println!("Updated snapshot: {}", name);
        } else {
            // Compare mode: check against existing
            if !path.exists() {
                // Snapshot doesn't exist, create it
                fs::write(&path, content)
                    .unwrap_or_else(|e| panic!("Failed to create snapshot '{}': {}", name, e));
                println!("Created new snapshot: {}", name);
            } else {
                // Load and compare
                let expected = fs::read_to_string(&path)
                    .unwrap_or_else(|e| panic!("Failed to read snapshot '{}': {}", name, e));

                let expected_trimmed = expected.trim();
                let actual_trimmed = content.trim();

                if expected_trimmed != actual_trimmed {
                    panic!(
                        "\nSnapshot '{}' mismatch!\n\
                         \n\
                         Expected:\n\
                         {}\n\
                         \n\
                         Actual:\n\
                         {}\n\
                         \n\
                         To update snapshots, run:\n\
                         REVUE_UPDATE_SNAPSHOTS=1 cargo test\n",
                        name, expected_trimmed, actual_trimmed
                    );
                }
            }
        }
    }

    /// Assert buffer snapshot
    pub fn assert_buffer_snapshot(&self, name: &str, buffer: &Buffer) {
        let content = buffer_to_string(buffer);
        self.assert_snapshot(name, &content);
    }

    /// Delete a snapshot file
    pub fn delete_snapshot(&self, name: &str) {
        let path = self.snapshot_path(name);
        if path.exists() {
            fs::remove_file(&path)
                .unwrap_or_else(|e| panic!("Failed to delete snapshot '{}': {}", name, e));
        }
    }

    /// Check if snapshot exists
    pub fn snapshot_exists(&self, name: &str) -> bool {
        self.snapshot_path(name).exists()
    }

    /// List all snapshots
    pub fn list_snapshots(&self) -> Vec<String> {
        if !self.snapshot_dir.exists() {
            return Vec::new();
        }

        let mut snapshots = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.snapshot_dir) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".snap") {
                        let snapshot_name = name.trim_end_matches(".snap").to_string();
                        snapshots.push(snapshot_name);
                    }
                }
            }
        }
        snapshots.sort();
        snapshots
    }
}

impl Default for SnapshotManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert buffer to string representation
fn buffer_to_string(buffer: &Buffer) -> String {
    let mut lines = Vec::new();
    for y in 0..buffer.height() {
        let mut line = String::new();
        for x in 0..buffer.width() {
            if let Some(cell) = buffer.get(x, y) {
                line.push(cell.symbol);
            } else {
                line.push(' ');
            }
        }
        // Preserve line but trim trailing spaces
        lines.push(line.trim_end().to_string());
    }

    // Remove trailing empty lines
    while lines.last().map(|l| l.is_empty()).unwrap_or(false) {
        lines.pop();
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Cell;

    fn make_buffer(text: &str) -> Buffer {
        let lines: Vec<&str> = text.lines().collect();
        let height = lines.len() as u16;
        let width = lines.iter().map(|l| l.len()).max().unwrap_or(0) as u16;

        let mut buffer = Buffer::new(width.max(1), height.max(1));
        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                buffer.set(x as u16, y as u16, Cell::new(ch));
            }
        }
        buffer
    }

    #[test]
    fn test_buffer_to_string() {
        let buffer = make_buffer("Hello\nWorld");
        let text = buffer_to_string(&buffer);
        assert_eq!(text, "Hello\nWorld");
    }

    #[test]
    fn test_buffer_to_string_trailing_spaces() {
        let buffer = make_buffer("Hello   \nWorld   ");
        let text = buffer_to_string(&buffer);
        assert_eq!(text, "Hello\nWorld");
    }

    #[test]
    fn test_snapshot_manager_new() {
        let manager = SnapshotManager::new();
        assert_eq!(manager.snapshot_dir, PathBuf::from("tests/snapshots"));
    }

    #[test]
    fn test_snapshot_path() {
        let manager = SnapshotManager::new();
        let path = manager.snapshot_path("test_widget");
        assert_eq!(path, PathBuf::from("tests/snapshots/test_widget.snap"));
    }
}
