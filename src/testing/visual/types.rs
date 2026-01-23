//! Core types for visual regression testing

use crate::style::Color;
use std::path::PathBuf;

/// Configuration for visual regression tests
#[derive(Debug, Clone)]
pub struct VisualTestConfig {
    /// Base directory for golden files
    pub golden_dir: PathBuf,
    /// Whether to update golden files instead of comparing
    pub update_mode: bool,
    /// Tolerance for color differences (0-255)
    pub color_tolerance: u8,
    /// Whether to generate diff images
    pub generate_diff: bool,
    /// Whether to fail on missing golden files
    pub fail_on_missing: bool,
    /// Include style information (bold, italic, etc.)
    pub include_styles: bool,
    /// Include color information
    pub include_colors: bool,
}

impl Default for VisualTestConfig {
    fn default() -> Self {
        let update_mode = std::env::var("REVUE_UPDATE_VISUALS")
            .map(|v| v == "1" || v.to_lowercase() == "true")
            .unwrap_or(false);

        Self {
            golden_dir: PathBuf::from("tests/golden"),
            update_mode,
            color_tolerance: 0,
            generate_diff: true,
            fail_on_missing: false,
            include_styles: true,
            include_colors: true,
        }
    }
}

impl VisualTestConfig {
    /// Create config with custom golden directory
    pub fn with_dir(dir: impl Into<PathBuf>) -> Self {
        Self {
            golden_dir: dir.into(),
            ..Default::default()
        }
    }

    /// Set color tolerance (0 = exact match, 255 = any color matches)
    pub fn tolerance(mut self, tolerance: u8) -> Self {
        self.color_tolerance = tolerance;
        self
    }

    /// Enable or disable diff generation
    pub fn generate_diff(mut self, enable: bool) -> Self {
        self.generate_diff = enable;
        self
    }

    /// Enable or disable style comparison
    pub fn include_styles(mut self, enable: bool) -> Self {
        self.include_styles = enable;
        self
    }

    /// Enable or disable color comparison
    pub fn include_colors(mut self, enable: bool) -> Self {
        self.include_colors = enable;
        self
    }
}

/// Result of a visual test
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualTestResult {
    /// Test passed (matches golden file)
    Passed,
    /// Golden file was created
    Created,
    /// Golden file was updated
    Updated,
    /// Test failed (differences found)
    Failed,
}

/// Captured cell data
#[derive(Debug, Clone, PartialEq)]
pub struct CapturedCell {
    /// Character
    pub symbol: char,
    /// Foreground color
    pub fg: Option<Color>,
    /// Background color
    pub bg: Option<Color>,
    /// Is bold
    pub bold: bool,
    /// Is italic
    pub italic: bool,
    /// Is underline
    pub underline: bool,
    /// Is dim
    pub dim: bool,
}

impl Default for CapturedCell {
    fn default() -> Self {
        Self {
            symbol: ' ',
            fg: None,
            bg: None,
            bold: false,
            italic: false,
            underline: false,
            dim: false,
        }
    }
}

impl CapturedCell {
    /// Create from character only
    pub fn from_char(ch: char) -> Self {
        Self {
            symbol: ch,
            ..Default::default()
        }
    }

    /// Compare with tolerance for colors
    pub fn matches(
        &self,
        other: &Self,
        tolerance: u8,
        include_styles: bool,
        include_colors: bool,
    ) -> bool {
        // Symbol must match
        if self.symbol != other.symbol {
            return false;
        }

        // Compare colors if enabled
        if include_colors {
            if !crate::testing::visual::comparison::colors_match(&self.fg, &other.fg, tolerance) {
                return false;
            }
            if !crate::testing::visual::comparison::colors_match(&self.bg, &other.bg, tolerance) {
                return false;
            }
        }

        // Compare styles if enabled
        if include_styles
            && (self.bold != other.bold
                || self.italic != other.italic
                || self.underline != other.underline
                || self.dim != other.dim)
        {
            return false;
        }

        true
    }
}

/// Captured visual state of a buffer
#[derive(Debug, Clone)]
pub struct VisualCapture {
    /// Width of capture
    pub width: u16,
    /// Height of capture
    pub height: u16,
    /// Cell data
    pub cells: Vec<CapturedCell>,
    /// Include styles
    pub include_styles: bool,
    /// Include colors
    pub include_colors: bool,
}

/// Difference between two captures
#[derive(Debug)]
pub struct VisualDiff {
    /// Size mismatch (actual, expected)
    pub size_mismatch: Option<((u16, u16), (u16, u16))>,
    /// Cell differences
    pub differences: Vec<CellDiff>,
    /// Actual width
    pub actual_width: u16,
    /// Actual height
    pub actual_height: u16,
    /// Expected width
    pub expected_width: u16,
    /// Expected height
    pub expected_height: u16,
}

/// Difference in a single cell
#[derive(Debug)]
pub struct CellDiff {
    /// X position
    pub x: u16,
    /// Y position
    pub y: u16,
    /// Actual cell
    pub actual: CapturedCell,
    /// Expected cell
    pub expected: CapturedCell,
}

impl VisualDiff {
    /// Check if there are any differences
    pub fn has_differences(&self) -> bool {
        self.size_mismatch.is_some() || !self.differences.is_empty()
    }

    /// Get summary of differences
    pub fn summary(&self) -> String {
        let mut output = String::new();

        if let Some(((aw, ah), (ew, eh))) = self.size_mismatch {
            output.push_str(&format!(
                "Size mismatch: actual {}x{}, expected {}x{}\n",
                aw, ah, ew, eh
            ));
            return output;
        }

        let total = self.differences.len();
        output.push_str(&format!("Found {} cell difference(s):\n\n", total));

        // Show first 10 differences
        for (i, diff) in self.differences.iter().take(10).enumerate() {
            output.push_str(&format!(
                "  {}. ({}, {}): '{}' -> '{}'\n",
                i + 1,
                diff.x,
                diff.y,
                diff.expected.symbol,
                diff.actual.symbol
            ));

            // Show color diff if applicable
            if diff.actual.fg != diff.expected.fg {
                output.push_str(&format!(
                    "     fg: {:?} -> {:?}\n",
                    diff.expected.fg, diff.actual.fg
                ));
            }
            if diff.actual.bg != diff.expected.bg {
                output.push_str(&format!(
                    "     bg: {:?} -> {:?}\n",
                    diff.expected.bg, diff.actual.bg
                ));
            }
        }

        if total > 10 {
            output.push_str(&format!("\n  ... and {} more\n", total - 10));
        }

        output
    }
}

impl std::fmt::Display for VisualDiff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}
