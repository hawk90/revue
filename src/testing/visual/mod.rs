//! Visual regression testing for TUI applications
//!
//! Provides comprehensive visual testing including color, style, and layout comparison.
//! Unlike simple text snapshots, visual tests capture the full rendered appearance.
//!
//! # Features
//!
//! | Feature | Description |
//! |---------|-------------|
//! | **Full Render Capture** | Colors, styles, and text |
//! | **Diff Visualization** | See exactly what changed |
//! | **Threshold Testing** | Allow minor variations |
//! | **CI Integration** | GitHub Actions, GitLab CI |
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::testing::{VisualTest, VisualTestConfig};
//!
//! #[test]
//! fn test_button_styles() {
//!     let mut test = VisualTest::new("button_styles");
//!
//!     // Render your widget
//!     let buffer = render_button();
//!
//!     // Compare against golden file
//!     test.assert_matches(&buffer);
//! }
//! ```
//!
//! # Updating Golden Files
//!
//! ```bash
//! # Update all visual tests
//! REVUE_UPDATE_VISUALS=1 cargo test
//!
//! # Update specific test
//! REVUE_UPDATE_VISUALS=1 cargo test test_button_styles
//! ```

mod capture;
mod comparison;
mod helpers;
pub mod types;

#[cfg(test)]
mod tests;

// Re-export public API for backward compatibility
pub use types::{
    CapturedCell, CellDiff, VisualCapture, VisualDiff, VisualTestConfig, VisualTestResult,
};

use crate::render::Buffer;
use std::fs;
use std::path::PathBuf;

/// Visual regression test instance
pub struct VisualTest {
    /// Test name (used for file naming)
    pub(crate) name: String,
    /// Configuration
    pub(crate) config: VisualTestConfig,
    /// Test group/category
    pub(crate) group: Option<String>,
}

impl VisualTest {
    /// Create a new visual test
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            config: VisualTestConfig::default(),
            group: None,
        }
    }

    /// Create with custom configuration
    pub fn with_config(name: impl Into<String>, config: VisualTestConfig) -> Self {
        Self {
            name: name.into(),
            config,
            group: None,
        }
    }

    /// Set test group (creates subdirectory)
    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }

    /// Get the golden file path
    pub(crate) fn golden_path(&self) -> PathBuf {
        let mut path = self.config.golden_dir.clone();
        if let Some(ref group) = self.group {
            path = path.join(group);
        }
        path.join(format!("{}.golden", self.name))
    }

    /// Get the diff file path
    pub(crate) fn diff_path(&self) -> PathBuf {
        let mut path = self.config.golden_dir.clone();
        if let Some(ref group) = self.group {
            path = path.join(group);
        }
        path.join(format!("{}.diff", self.name))
    }

    /// Assert that buffer matches golden file
    pub fn assert_matches(&self, buffer: &Buffer) -> VisualTestResult {
        let actual = VisualCapture::from_buffer(
            buffer,
            self.config.include_styles,
            self.config.include_colors,
        );
        let golden_path = self.golden_path();

        // Ensure directory exists
        if let Some(parent) = golden_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .unwrap_or_else(|e| panic!("Failed to create golden directory: {}", e));
            }
        }

        if self.config.update_mode {
            // Update mode: save new golden file
            actual
                .save(&golden_path)
                .unwrap_or_else(|e| panic!("Failed to save golden file: {}", e));
            println!("Updated golden file: {}", self.name);
            return VisualTestResult::Updated;
        }

        if !golden_path.exists() {
            if self.config.fail_on_missing {
                panic!("Golden file not found: {:?}", golden_path);
            } else {
                // Create new golden file
                actual
                    .save(&golden_path)
                    .unwrap_or_else(|e| panic!("Failed to create golden file: {}", e));
                println!("Created golden file: {}", self.name);
                return VisualTestResult::Created;
            }
        }

        // Load and compare
        let expected = VisualCapture::load(&golden_path)
            .unwrap_or_else(|e| panic!("Failed to load golden file: {}", e));

        let diff = actual.diff(&expected, self.config.color_tolerance);

        if diff.has_differences() {
            // Generate diff file if enabled
            if self.config.generate_diff {
                let diff_content = diff.to_string();
                let diff_path = self.diff_path();
                fs::write(&diff_path, &diff_content)
                    .unwrap_or_else(|e| panic!("Failed to write diff file: {}", e));
            }

            panic!(
                "\nVisual regression detected in '{}'!\n\n\
                 {}\n\n\
                 To update golden files, run:\n\
                 REVUE_UPDATE_VISUALS=1 cargo test\n",
                self.name,
                diff.summary()
            );
        }

        VisualTestResult::Passed
    }

    /// Compare two buffers and return diff
    pub fn compare(&self, actual: &Buffer, expected: &Buffer) -> VisualDiff {
        let actual_capture = VisualCapture::from_buffer(
            actual,
            self.config.include_styles,
            self.config.include_colors,
        );
        let expected_capture = VisualCapture::from_buffer(
            expected,
            self.config.include_styles,
            self.config.include_colors,
        );
        actual_capture.diff(&expected_capture, self.config.color_tolerance)
    }
}
