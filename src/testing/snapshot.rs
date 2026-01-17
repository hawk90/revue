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

                // Normalize line endings for cross-platform compatibility
                let expected_normalized = expected.replace("\r\n", "\n");
                let actual_normalized = content.replace("\r\n", "\n");

                let expected_trimmed = expected_normalized.trim();
                let actual_trimmed = actual_normalized.trim();

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

    // =========================================================================
    // SnapshotManager constructor tests
    // =========================================================================

    #[test]
    fn test_snapshot_manager_default() {
        let manager = SnapshotManager::default();
        assert_eq!(manager.snapshot_dir, PathBuf::from("tests/snapshots"));
    }

    #[test]
    fn test_snapshot_manager_with_dir() {
        let manager = SnapshotManager::with_dir("custom/snapshots");
        assert_eq!(manager.snapshot_dir, PathBuf::from("custom/snapshots"));
    }

    #[test]
    fn test_snapshot_manager_with_pathbuf_dir() {
        let dir = PathBuf::from("/tmp/test_snapshots");
        let manager = SnapshotManager::with_dir(dir.clone());
        assert_eq!(manager.snapshot_dir, dir);
    }

    // =========================================================================
    // Snapshot path tests
    // =========================================================================

    #[test]
    fn test_snapshot_path_with_custom_dir() {
        let manager = SnapshotManager::with_dir("my/path");
        let path = manager.snapshot_path("widget");
        assert_eq!(path, PathBuf::from("my/path/widget.snap"));
    }

    #[test]
    fn test_snapshot_path_nested_name() {
        let manager = SnapshotManager::new();
        let path = manager.snapshot_path("widgets/button");
        assert_eq!(path, PathBuf::from("tests/snapshots/widgets/button.snap"));
    }

    // =========================================================================
    // buffer_to_string tests
    // =========================================================================

    #[test]
    fn test_buffer_to_string_empty() {
        let buffer = Buffer::new(5, 3);
        let text = buffer_to_string(&buffer);
        // Empty buffer should produce empty string (all spaces trimmed)
        assert!(text.is_empty() || text.chars().all(|c| c.is_whitespace() || c == '\n'));
    }

    #[test]
    fn test_buffer_to_string_single_line() {
        let buffer = make_buffer("Hello");
        let text = buffer_to_string(&buffer);
        assert_eq!(text, "Hello");
    }

    #[test]
    fn test_buffer_to_string_multiple_lines() {
        let buffer = make_buffer("Line1\nLine2\nLine3");
        let text = buffer_to_string(&buffer);
        assert_eq!(text, "Line1\nLine2\nLine3");
    }

    #[test]
    fn test_buffer_to_string_unicode() {
        let buffer = make_buffer("Hello 世界");
        let text = buffer_to_string(&buffer);
        assert_eq!(text, "Hello 世界");
    }

    #[test]
    fn test_buffer_to_string_with_box_chars() {
        let buffer = make_buffer("┌───┐\n│   │\n└───┘");
        let text = buffer_to_string(&buffer);
        assert!(text.contains("┌"));
        assert!(text.contains("└"));
    }

    // =========================================================================
    // Snapshot exists tests
    // =========================================================================

    #[test]
    fn test_snapshot_exists_nonexistent() {
        let manager = SnapshotManager::with_dir("/tmp/nonexistent_snapshot_dir_12345");
        assert!(!manager.snapshot_exists("definitely_does_not_exist"));
    }

    // =========================================================================
    // List snapshots tests
    // =========================================================================

    #[test]
    fn test_list_snapshots_nonexistent_dir() {
        let manager = SnapshotManager::with_dir("/tmp/nonexistent_snapshot_dir_12345");
        let snapshots = manager.list_snapshots();
        assert!(snapshots.is_empty());
    }

    // =========================================================================
    // Integration tests with temp directory
    // =========================================================================

    #[test]
    fn test_full_snapshot_workflow() {
        use std::env;

        // Create temp directory
        let temp_dir = env::temp_dir().join("revue_snapshot_test");
        let _ = fs::remove_dir_all(&temp_dir); // Clean up any previous run
        fs::create_dir_all(&temp_dir).unwrap();

        let manager = SnapshotManager::with_dir(&temp_dir);

        // Initially no snapshots
        assert!(!manager.snapshot_exists("test_snap"));
        assert!(manager.list_snapshots().is_empty());

        // Create a snapshot
        manager.assert_snapshot("test_snap", "Hello World");

        // Should now exist
        assert!(manager.snapshot_exists("test_snap"));
        let snapshots = manager.list_snapshots();
        assert!(snapshots.contains(&"test_snap".to_string()));

        // Delete the snapshot
        manager.delete_snapshot("test_snap");
        assert!(!manager.snapshot_exists("test_snap"));

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_assert_snapshot_creates_file() {
        use std::env;

        let temp_dir = env::temp_dir().join("revue_snapshot_test_2");
        let _ = fs::remove_dir_all(&temp_dir);

        let manager = SnapshotManager::with_dir(&temp_dir);
        manager.assert_snapshot("new_snapshot", "Test content");

        let path = temp_dir.join("new_snapshot.snap");
        assert!(path.exists());

        let content = fs::read_to_string(&path).unwrap();
        assert_eq!(content, "Test content");

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_assert_snapshot_matches() {
        use std::env;

        let temp_dir = env::temp_dir().join("revue_snapshot_test_3");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        // Pre-create a snapshot file
        let path = temp_dir.join("existing.snap");
        fs::write(&path, "Expected content").unwrap();

        let manager = SnapshotManager::with_dir(&temp_dir);

        // This should not panic since content matches
        manager.assert_snapshot("existing", "Expected content");

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_buffer_snapshot() {
        use std::env;

        let temp_dir = env::temp_dir().join("revue_snapshot_test_4");
        let _ = fs::remove_dir_all(&temp_dir);

        let manager = SnapshotManager::with_dir(&temp_dir);
        let buffer = make_buffer("Test\nBuffer");

        manager.assert_buffer_snapshot("buffer_test", &buffer);

        let path = temp_dir.join("buffer_test.snap");
        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("Test"));
        assert!(content.contains("Buffer"));

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
