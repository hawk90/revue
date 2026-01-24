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
mod tests {
#![allow(unused_imports)]

//! Tests for visual regression testing

use crate::render::Cell;
use crate::style::Color;
use crate::testing::visual::{
    comparison::colors_match,
    helpers::parse_hex_color,
    types::{CapturedCell, VisualCapture, VisualDiff, VisualTestConfig, VisualTestResult},
    VisualTest,
};

fn make_buffer(text: &str) -> crate::render::Buffer {
    let lines: Vec<&str> = text.lines().collect();
    let height = lines.len() as u16;
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0) as u16;

    let mut buffer = crate::render::Buffer::new(width.max(1), height.max(1));
    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            buffer.set(x as u16, y as u16, Cell::new(ch));
        }
    }
    buffer
}

#[test]
fn test_visual_capture_from_buffer() {
    let buffer = make_buffer("Hello
World");
    let config = VisualTestConfig::default();
    let capture = VisualCapture::from_buffer(&buffer, config.include_styles, config.include_colors);

    assert_eq!(capture.width, 5);
    assert_eq!(capture.height, 2);
    assert_eq!(capture.get(0, 0).unwrap().symbol, 'H');
    assert_eq!(capture.get(4, 0).unwrap().symbol, 'o');
    assert_eq!(capture.get(0, 1).unwrap().symbol, 'W');
}

#[test]
fn test_visual_capture_diff_identical() {
    let buffer = make_buffer("Test");
    let config = VisualTestConfig::default();
    let capture1 =
        VisualCapture::from_buffer(&buffer, config.include_styles, config.include_colors);
    let capture2 =
        VisualCapture::from_buffer(&buffer, config.include_styles, config.include_colors);

    let diff = capture1.diff(&capture2, 0);
    assert!(!diff.has_differences());
}

#[test]
fn test_visual_capture_diff_different() {
    let buffer1 = make_buffer("Hello");
    let buffer2 = make_buffer("World");
    let config = VisualTestConfig::default();
    let capture1 =
        VisualCapture::from_buffer(&buffer1, config.include_styles, config.include_colors);
    let capture2 =
        VisualCapture::from_buffer(&buffer2, config.include_styles, config.include_colors);

    let diff = capture1.diff(&capture2, 0);
    assert!(diff.has_differences());
    assert!(!diff.differences.is_empty());
}

#[test]
fn test_visual_capture_diff_size_mismatch() {
    let buffer1 = make_buffer("Hi");
    let buffer2 = make_buffer("Hello");
    let config = VisualTestConfig::default();
    let capture1 =
        VisualCapture::from_buffer(&buffer1, config.include_styles, config.include_colors);
    let capture2 =
        VisualCapture::from_buffer(&buffer2, config.include_styles, config.include_colors);

    let diff = capture1.diff(&capture2, 0);
    assert!(diff.has_differences());
    assert!(diff.size_mismatch.is_some());
}

#[test]
fn test_captured_cell_matches_exact() {
    let cell1 = CapturedCell::from_char('A');
    let cell2 = CapturedCell::from_char('A');
    assert!(cell1.matches(&cell2, 0, true, true));
}

#[test]
fn test_captured_cell_matches_different_char() {
    let cell1 = CapturedCell::from_char('A');
    let cell2 = CapturedCell::from_char('B');
    assert!(!cell1.matches(&cell2, 0, true, true));
}

#[test]
fn test_color_tolerance() {
    let c1 = Some(Color::rgb(100, 100, 100));
    let c2 = Some(Color::rgb(105, 100, 100));

    // Exact match fails
    assert!(!colors_match(&c1, &c2, 0));

    // Within tolerance passes
    assert!(colors_match(&c1, &c2, 10));
}

#[test]
fn test_visual_test_config_default() {
    let config = VisualTestConfig::default();
    assert_eq!(config.golden_dir, std::path::PathBuf::from("tests/golden"));
    assert_eq!(config.color_tolerance, 0);
    assert!(config.include_styles);
    assert!(config.include_colors);
}

#[test]
fn test_serialize_deserialize() {
    let buffer = make_buffer("AB
CD");
    let config = VisualTestConfig::default();
    let capture = VisualCapture::from_buffer(&buffer, config.include_styles, config.include_colors);

    let serialized = capture.serialize();
    let deserialized = VisualCapture::deserialize(&serialized).unwrap();

    assert_eq!(capture.width, deserialized.width);
    assert_eq!(capture.height, deserialized.height);
    assert_eq!(
        capture.get(0, 0).unwrap().symbol,
        deserialized.get(0, 0).unwrap().symbol
    );
}

#[test]
fn test_parse_hex_color() {
    assert_eq!(parse_hex_color("#ff0000"), Some((255, 0, 0)));
    assert_eq!(parse_hex_color("#00ff00"), Some((0, 255, 0)));
    assert_eq!(parse_hex_color("#0000ff"), Some((0, 0, 255)));
    assert_eq!(parse_hex_color("ffffff"), Some((255, 255, 255)));
    assert_eq!(parse_hex_color("invalid"), None);
}

// VisualTestConfig tests

#[test]
fn test_config_with_dir() {
    let config = VisualTestConfig::with_dir("custom/path");
    assert_eq!(config.golden_dir, std::path::PathBuf::from("custom/path"));
}

#[test]
fn test_config_tolerance() {
    let config = VisualTestConfig::default().tolerance(10);
    assert_eq!(config.color_tolerance, 10);
}

#[test]
fn test_config_generate_diff() {
    let config = VisualTestConfig::default().generate_diff(false);
    assert!(!config.generate_diff);
}

#[test]
fn test_config_include_styles() {
    let config = VisualTestConfig::default().include_styles(false);
    assert!(!config.include_styles);
}

#[test]
fn test_config_include_colors() {
    let config = VisualTestConfig::default().include_colors(false);
    assert!(!config.include_colors);
}

#[test]
fn test_config_clone() {
    let config = VisualTestConfig::default().tolerance(5);
    let cloned = config.clone();
    assert_eq!(cloned.color_tolerance, 5);
}

// VisualTest tests

#[test]
fn test_visual_test_new() {
    let test = VisualTest::new("my_test");
    assert_eq!(test.name, "my_test");
    assert!(test.group.is_none());
}

#[test]
fn test_visual_test_with_config() {
    let config = VisualTestConfig::default().tolerance(10);
    let test = VisualTest::with_config("test", config);
    assert_eq!(test.config.color_tolerance, 10);
}

#[test]
fn test_visual_test_group() {
    let test = VisualTest::new("test").group("buttons");
    assert_eq!(test.group, Some("buttons".to_string()));
}

#[test]
fn test_visual_test_golden_path() {
    let test = VisualTest::new("button_test");
    let path = test.golden_path();
    assert!(path.to_string_lossy().contains("button_test.golden"));
}

#[test]
fn test_visual_test_golden_path_with_group() {
    let test = VisualTest::new("button_test").group("widgets");
    let path = test.golden_path();
    assert!(path.to_string_lossy().contains("widgets"));
    assert!(path.to_string_lossy().contains("button_test.golden"));
}

#[test]
fn test_visual_test_compare() {
    let test = VisualTest::new("test");
    let buffer1 = make_buffer("Hello");
    let buffer2 = make_buffer("Hello");

    let diff = test.compare(&buffer1, &buffer2);
    assert!(!diff.has_differences());
}

// VisualTestResult tests

#[test]
fn test_visual_test_result_equality() {
    assert_eq!(VisualTestResult::Passed, VisualTestResult::Passed);
    assert_ne!(VisualTestResult::Passed, VisualTestResult::Failed);
}

#[test]
fn test_visual_test_result_copy() {
    let result = VisualTestResult::Created;
    let copied = result;
    assert_eq!(copied, VisualTestResult::Created);
}

// CapturedCell tests

#[test]
fn test_captured_cell_default() {
    let cell = CapturedCell::default();
    assert_eq!(cell.symbol, ' ');
    assert!(cell.fg.is_none());
    assert!(cell.bg.is_none());
    assert!(!cell.bold);
    assert!(!cell.italic);
    assert!(!cell.underline);
    assert!(!cell.dim);
}

#[test]
fn test_captured_cell_from_char() {
    let cell = CapturedCell::from_char('X');
    assert_eq!(cell.symbol, 'X');
    assert!(cell.fg.is_none());
}

#[test]
fn test_captured_cell_matches_ignore_colors() {
    let cell1 = CapturedCell {
        symbol: 'A',
        fg: Some(Color::rgb(255, 0, 0)),
        ..Default::default()
    };
    let cell2 = CapturedCell {
        symbol: 'A',
        fg: Some(Color::rgb(0, 255, 0)),
        ..Default::default()
    };

    // Without colors, should match
    assert!(cell1.matches(&cell2, 0, false, false));
    // With colors, should not match
    assert!(!cell1.matches(&cell2, 0, false, true));
}

#[test]
fn test_captured_cell_matches_ignore_styles() {
    let cell1 = CapturedCell {
        symbol: 'A',
        bold: true,
        ..Default::default()
    };
    let cell2 = CapturedCell {
        symbol: 'A',
        bold: false,
        ..Default::default()
    };

    // Without styles, should match
    assert!(cell1.matches(&cell2, 0, false, false));
    // With styles, should not match
    assert!(!cell1.matches(&cell2, 0, true, false));
}

#[test]
fn test_captured_cell_clone() {
    let cell = CapturedCell {
        symbol: 'X',
        bold: true,
        fg: Some(Color::rgb(100, 100, 100)),
        ..Default::default()
    };
    let cloned = cell.clone();
    assert_eq!(cloned.symbol, 'X');
    assert!(cloned.bold);
}

// Color matching tests

#[test]
fn test_colors_match_both_none() {
    assert!(colors_match(&None, &None, 0));
}

#[test]
fn test_colors_match_one_none() {
    let color = Some(Color::rgb(100, 100, 100));
    // Without tolerance, mismatched
    assert!(!colors_match(&color, &None, 0));
    assert!(!colors_match(&None, &color, 0));
    // With max tolerance, matches
    assert!(colors_match(&color, &None, 255));
}

#[test]
fn test_colors_match_exact() {
    let c1 = Some(Color::rgb(100, 150, 200));
    let c2 = Some(Color::rgb(100, 150, 200));
    assert!(colors_match(&c1, &c2, 0));
}

#[test]
fn test_colors_match_within_tolerance() {
    let c1 = Some(Color::rgb(100, 100, 100));
    let c2 = Some(Color::rgb(105, 95, 102));

    assert!(!colors_match(&c1, &c2, 0));
    assert!(!colors_match(&c1, &c2, 4));
    assert!(colors_match(&c1, &c2, 5));
    assert!(colors_match(&c1, &c2, 10));
}

// VisualCapture tests

#[test]
fn test_capture_get_out_of_bounds() {
    let buffer = make_buffer("AB");
    let config = VisualTestConfig::default();
    let capture = VisualCapture::from_buffer(&buffer, config.include_styles, config.include_colors);

    assert!(capture.get(0, 0).is_some());
    assert!(capture.get(100, 100).is_none());
}

#[test]
fn test_capture_serialize_contains_header() {
    let buffer = make_buffer("Test");
    let config = VisualTestConfig::default();
    let capture = VisualCapture::from_buffer(&buffer, config.include_styles, config.include_colors);

    let serialized = capture.serialize();
    assert!(serialized.contains("# Visual Golden File"));
    assert!(serialized.contains("# Size:"));
    assert!(serialized.contains("## Text"));
}

#[test]
fn test_capture_serialize_contains_text() {
    let buffer = make_buffer("Hello
World");
    let config = VisualTestConfig::default();
    let capture = VisualCapture::from_buffer(&buffer, config.include_styles, config.include_colors);

    let serialized = capture.serialize();
    assert!(serialized.contains("Hello"));
    assert!(serialized.contains("World"));
}

// VisualDiff tests

#[test]
fn test_diff_has_differences_size_mismatch() {
    let diff = VisualDiff {
        size_mismatch: Some(((10, 5), (20, 10))),
        differences: vec![],
        actual_width: 10,
        actual_height: 5,
        expected_width: 20,
        expected_height: 10,
    };
    assert!(diff.has_differences());
}

#[test]
fn test_diff_has_differences_cell_diff() {
    let diff = VisualDiff {
        size_mismatch: None,
        differences: vec![crate::testing::visual::types::CellDiff {
            x: 0,
            y: 0,
            actual: CapturedCell::from_char('A'),
            expected: CapturedCell::from_char('B'),
        }],
        actual_width: 10,
        actual_height: 5,
        expected_width: 10,
        expected_height: 5,
    };
    assert!(diff.has_differences());
}

#[test]
fn test_diff_no_differences() {
    let diff = VisualDiff {
        size_mismatch: None,
        differences: vec![],
        actual_width: 10,
        actual_height: 5,
        expected_width: 10,
        expected_height: 5,
    };
    assert!(!diff.has_differences());
}

#[test]
fn test_diff_summary_size_mismatch() {
    let diff = VisualDiff {
        size_mismatch: Some(((10, 5), (20, 10))),
        differences: vec![],
        actual_width: 10,
        actual_height: 5,
        expected_width: 20,
        expected_height: 10,
    };

    let summary = diff.summary();
    assert!(summary.contains("Size mismatch"));
    assert!(summary.contains("10x5"));
    assert!(summary.contains("20x10"));
}

#[test]
fn test_diff_summary_cell_differences() {
    let diff = VisualDiff {
        size_mismatch: None,
        differences: vec![
            crate::testing::visual::types::CellDiff {
                x: 0,
                y: 0,
                actual: CapturedCell::from_char('A'),
                expected: CapturedCell::from_char('B'),
            },
            crate::testing::visual::types::CellDiff {
                x: 1,
                y: 1,
                actual: CapturedCell::from_char('X'),
                expected: CapturedCell::from_char('Y'),
            },
        ],
        actual_width: 10,
        actual_height: 5,
        expected_width: 10,
        expected_height: 5,
    };

    let summary = diff.summary();
    assert!(summary.contains("2 cell difference"));
}

#[test]
fn test_diff_summary_many_differences() {
    let mut differences = Vec::new();
    for i in 0..15 {
        differences.push(crate::testing::visual::types::CellDiff {
            x: i,
            y: 0,
            actual: CapturedCell::from_char('A'),
            expected: CapturedCell::from_char('B'),
        });
    }

    let diff = VisualDiff {
        size_mismatch: None,
        differences,
        actual_width: 20,
        actual_height: 5,
        expected_width: 20,
        expected_height: 5,
    };

    let summary = diff.summary();
    assert!(summary.contains("15 cell difference"));
    assert!(summary.contains("... and 5 more"));
}

#[test]
fn test_diff_display() {
    let diff = VisualDiff {
        size_mismatch: None,
        differences: vec![],
        actual_width: 10,
        actual_height: 5,
        expected_width: 10,
        expected_height: 5,
    };

    let display = format!("{}", diff);
    assert!(display.contains("0 cell difference"));
}

// Parse hex color tests

#[test]
fn test_parse_hex_color_short() {
    assert_eq!(parse_hex_color("abc"), None);
}

#[test]
fn test_parse_hex_color_long() {
    assert_eq!(parse_hex_color("#aabbccdd"), None);
}

#[test]
fn test_parse_hex_color_gray() {
    assert_eq!(parse_hex_color("#808080"), Some((128, 128, 128)));
}

}

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
