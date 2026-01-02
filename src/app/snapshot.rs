//! Snapshot testing utilities for UI components

use crate::layout::Rect;
use crate::render::Buffer;
use crate::widget::{View, RenderContext};
use std::path::{Path, PathBuf};
use std::fs;

/// Snapshot test result
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnapshotResult {
    /// Snapshot matches
    Match,
    /// Snapshot differs
    Mismatch {
        /// Expected snapshot content
        expected: String,
        /// Actual rendered content
        actual: String,
        /// Line-by-line differences (line number, expected, actual)
        diff: Vec<(usize, String, String)>,
    },
    /// New snapshot created
    Created,
    /// Snapshot file not found
    NotFound,
}

impl SnapshotResult {
    /// Check if snapshot matches
    pub fn is_match(&self) -> bool {
        matches!(self, SnapshotResult::Match | SnapshotResult::Created)
    }

    /// Check if snapshot mismatches
    pub fn is_mismatch(&self) -> bool {
        matches!(self, SnapshotResult::Mismatch { .. })
    }
}

/// Snapshot configuration
#[derive(Clone, Debug)]
pub struct SnapshotConfig {
    /// Directory to store snapshots
    pub snapshot_dir: PathBuf,
    /// Whether to update snapshots
    pub update_snapshots: bool,
    /// Include ANSI colors in snapshot
    pub include_colors: bool,
    /// Include modifiers (bold, italic, etc.)
    pub include_modifiers: bool,
}

impl Default for SnapshotConfig {
    fn default() -> Self {
        Self {
            snapshot_dir: PathBuf::from("snapshots"),
            update_snapshots: std::env::var("UPDATE_SNAPSHOTS").is_ok(),
            include_colors: false,
            include_modifiers: false,
        }
    }
}

impl SnapshotConfig {
    /// Set snapshot directory
    pub fn snapshot_dir(mut self, dir: impl AsRef<Path>) -> Self {
        self.snapshot_dir = dir.as_ref().to_path_buf();
        self
    }

    /// Set whether to update snapshots
    pub fn update_snapshots(mut self, update: bool) -> Self {
        self.update_snapshots = update;
        self
    }

    /// Include colors in snapshot
    pub fn include_colors(mut self, include: bool) -> Self {
        self.include_colors = include;
        self
    }

    /// Include modifiers in snapshot
    pub fn include_modifiers(mut self, include: bool) -> Self {
        self.include_modifiers = include;
        self
    }
}

/// Snapshot tester
pub struct Snapshot {
    config: SnapshotConfig,
}

impl Snapshot {
    /// Create a new snapshot tester
    pub fn new() -> Self {
        Self {
            config: SnapshotConfig::default(),
        }
    }

    /// Create with custom config
    pub fn with_config(config: SnapshotConfig) -> Self {
        Self { config }
    }

    /// Set config
    pub fn config(mut self, config: SnapshotConfig) -> Self {
        self.config = config;
        self
    }

    /// Render a view to a buffer
    pub fn render_view<V: View>(&self, view: &V, width: u16, height: u16) -> Buffer {
        let mut buffer = Buffer::new(width, height);
        let area = Rect::new(0, 0, width, height);
        let mut ctx = RenderContext::new(&mut buffer, area);
        view.render(&mut ctx);
        buffer
    }

    /// Convert buffer to string representation
    pub fn buffer_to_string(&self, buffer: &Buffer) -> String {
        let mut lines = Vec::new();

        for y in 0..buffer.height() {
            let mut line = String::new();
            for x in 0..buffer.width() {
                if let Some(cell) = buffer.get(x, y) {
                    if self.config.include_colors {
                        if let Some(fg) = cell.fg {
                            line.push_str(&format!("\x1b[38;2;{};{};{}m", fg.r, fg.g, fg.b));
                        }
                        if let Some(bg) = cell.bg {
                            line.push_str(&format!("\x1b[48;2;{};{};{}m", bg.r, bg.g, bg.b));
                        }
                    }
                    if self.config.include_modifiers {
                        if cell.modifier.contains(crate::render::Modifier::BOLD) {
                            line.push_str("\x1b[1m");
                        }
                        if cell.modifier.contains(crate::render::Modifier::ITALIC) {
                            line.push_str("\x1b[3m");
                        }
                        if cell.modifier.contains(crate::render::Modifier::UNDERLINE) {
                            line.push_str("\x1b[4m");
                        }
                    }
                    line.push(cell.symbol);
                    if self.config.include_colors || self.config.include_modifiers {
                        line.push_str("\x1b[0m");
                    }
                } else {
                    line.push(' ');
                }
            }
            // Trim trailing spaces but keep line structure
            let trimmed = line.trim_end();
            lines.push(trimmed.to_string());
        }

        // Remove trailing empty lines
        while lines.last().map(|l| l.is_empty()).unwrap_or(false) {
            lines.pop();
        }

        lines.join("\n")
    }

    /// Get snapshot file path
    fn snapshot_path(&self, name: &str) -> PathBuf {
        self.config.snapshot_dir.join(format!("{}.snap", name))
    }

    /// Assert snapshot matches
    pub fn assert_snapshot<V: View>(&self, name: &str, view: &V, width: u16, height: u16) -> SnapshotResult {
        let buffer = self.render_view(view, width, height);
        let actual = self.buffer_to_string(&buffer);
        self.assert_snapshot_string(name, &actual)
    }

    /// Assert snapshot string matches
    pub fn assert_snapshot_string(&self, name: &str, actual: &str) -> SnapshotResult {
        let path = self.snapshot_path(name);

        // Ensure directory exists
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        // Read existing snapshot
        let expected = fs::read_to_string(&path).ok();

        match expected {
            Some(expected) if expected == *actual => {
                SnapshotResult::Match
            }
            Some(expected) if self.config.update_snapshots => {
                fs::write(&path, actual).ok();
                SnapshotResult::Created
            }
            Some(expected) => {
                let diff = self.compute_diff(&expected, actual);
                SnapshotResult::Mismatch {
                    expected,
                    actual: actual.to_string(),
                    diff,
                }
            }
            None if self.config.update_snapshots => {
                fs::write(&path, actual).ok();
                SnapshotResult::Created
            }
            None => {
                fs::write(&path, actual).ok();
                SnapshotResult::Created
            }
        }
    }

    /// Compute line-by-line diff
    fn compute_diff(&self, expected: &str, actual: &str) -> Vec<(usize, String, String)> {
        let expected_lines: Vec<&str> = expected.lines().collect();
        let actual_lines: Vec<&str> = actual.lines().collect();

        let mut diff = Vec::new();
        let max_lines = expected_lines.len().max(actual_lines.len());

        for i in 0..max_lines {
            let exp = expected_lines.get(i).copied().unwrap_or("");
            let act = actual_lines.get(i).copied().unwrap_or("");

            if exp != act {
                diff.push((i + 1, exp.to_string(), act.to_string()));
            }
        }

        diff
    }

    /// Format diff for display
    pub fn format_diff(result: &SnapshotResult) -> String {
        match result {
            SnapshotResult::Match => "Snapshot matches!".to_string(),
            SnapshotResult::Created => "Snapshot created!".to_string(),
            SnapshotResult::NotFound => "Snapshot not found!".to_string(),
            SnapshotResult::Mismatch { diff, .. } => {
                let mut output = String::from("Snapshot mismatch:\n");
                for (line, expected, actual) in diff {
                    output.push_str(&format!(
                        "Line {}:\n  - {}\n  + {}\n",
                        line, expected, actual
                    ));
                }
                output
            }
        }
    }
}

impl Default for Snapshot {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to create a snapshot tester
pub fn snapshot() -> Snapshot {
    Snapshot::new()
}

/// Assert that a view matches its snapshot
///
/// # Panics
/// Panics if the snapshot doesn't match and UPDATE_SNAPSHOTS is not set
#[macro_export]
macro_rules! assert_snapshot {
    ($name:expr, $view:expr) => {
        assert_snapshot!($name, $view, 80, 24)
    };
    ($name:expr, $view:expr, $width:expr, $height:expr) => {{
        let snap = $crate::app::snapshot::snapshot();
        let result = snap.assert_snapshot($name, $view, $width, $height);
        if result.is_mismatch() {
            panic!(
                "Snapshot '{}' mismatch!\n{}",
                $name,
                $crate::app::snapshot::Snapshot::format_diff(&result)
            );
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widget::Text;

    #[test]
    fn test_snapshot_config_default() {
        let config = SnapshotConfig::default();
        assert!(!config.include_colors);
        assert!(!config.include_modifiers);
    }

    #[test]
    fn test_snapshot_config_builder() {
        let config = SnapshotConfig::default()
            .snapshot_dir("test_snapshots")
            .include_colors(true)
            .include_modifiers(true);

        assert!(config.include_colors);
        assert!(config.include_modifiers);
    }

    #[test]
    fn test_snapshot_new() {
        let snap = Snapshot::new();
        assert!(!snap.config.include_colors);
    }

    #[test]
    fn test_render_view() {
        let snap = Snapshot::new();
        let text = Text::new("Hello");
        let buffer = snap.render_view(&text, 10, 3);

        assert_eq!(buffer.width(), 10);
        assert_eq!(buffer.height(), 3);
    }

    #[test]
    fn test_buffer_to_string() {
        let snap = Snapshot::new();
        let text = Text::new("Test");
        let buffer = snap.render_view(&text, 10, 1);
        let output = snap.buffer_to_string(&buffer);

        assert!(output.contains("Test"));
    }

    #[test]
    fn test_snapshot_result_is_match() {
        assert!(SnapshotResult::Match.is_match());
        assert!(SnapshotResult::Created.is_match());
        assert!(!SnapshotResult::NotFound.is_match());
    }

    #[test]
    fn test_snapshot_result_is_mismatch() {
        let mismatch = SnapshotResult::Mismatch {
            expected: "a".to_string(),
            actual: "b".to_string(),
            diff: vec![(1, "a".to_string(), "b".to_string())],
        };
        assert!(mismatch.is_mismatch());
        assert!(!SnapshotResult::Match.is_mismatch());
    }

    #[test]
    fn test_compute_diff() {
        let snap = Snapshot::new();
        let diff = snap.compute_diff("line1\nline2", "line1\nchanged");

        assert_eq!(diff.len(), 1);
        assert_eq!(diff[0].0, 2);
        assert_eq!(diff[0].1, "line2");
        assert_eq!(diff[0].2, "changed");
    }

    #[test]
    fn test_format_diff_match() {
        let output = Snapshot::format_diff(&SnapshotResult::Match);
        assert!(output.contains("matches"));
    }

    #[test]
    fn test_format_diff_mismatch() {
        let result = SnapshotResult::Mismatch {
            expected: "a".to_string(),
            actual: "b".to_string(),
            diff: vec![(1, "a".to_string(), "b".to_string())],
        };
        let output = Snapshot::format_diff(&result);
        assert!(output.contains("mismatch"));
        assert!(output.contains("Line 1"));
    }

    #[test]
    fn test_snapshot_helper() {
        let snap = snapshot();
        // Verify snapshot helper returns valid instance
        assert!(snap.config.snapshot_dir.as_os_str().len() > 0);
    }
}
