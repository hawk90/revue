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

use crate::render::{Buffer, Modifier};
use crate::style::Color;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

// =============================================================================
// Visual Test Configuration
// =============================================================================

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

// =============================================================================
// Visual Test
// =============================================================================

/// Visual regression test instance
pub struct VisualTest {
    /// Test name (used for file naming)
    name: String,
    /// Configuration
    config: VisualTestConfig,
    /// Test group/category
    group: Option<String>,
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
    fn golden_path(&self) -> PathBuf {
        let mut path = self.config.golden_dir.clone();
        if let Some(ref group) = self.group {
            path = path.join(group);
        }
        path.join(format!("{}.golden", self.name))
    }

    /// Get the diff file path
    fn diff_path(&self) -> PathBuf {
        let mut path = self.config.golden_dir.clone();
        if let Some(ref group) = self.group {
            path = path.join(group);
        }
        path.join(format!("{}.diff", self.name))
    }

    /// Assert that buffer matches golden file
    pub fn assert_matches(&self, buffer: &Buffer) -> VisualTestResult {
        let actual = VisualCapture::from_buffer(buffer, &self.config);
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
        let actual_capture = VisualCapture::from_buffer(actual, &self.config);
        let expected_capture = VisualCapture::from_buffer(expected, &self.config);
        actual_capture.diff(&expected_capture, self.config.color_tolerance)
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

// =============================================================================
// Visual Capture
// =============================================================================

/// Captured visual state of a buffer
#[derive(Debug, Clone)]
pub struct VisualCapture {
    /// Width of capture
    pub width: u16,
    /// Height of capture
    pub height: u16,
    /// Cell data
    cells: Vec<CapturedCell>,
    /// Include styles
    include_styles: bool,
    /// Include colors
    include_colors: bool,
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
            if !colors_match(&self.fg, &other.fg, tolerance) {
                return false;
            }
            if !colors_match(&self.bg, &other.bg, tolerance) {
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

/// Check if two colors match within tolerance
fn colors_match(a: &Option<Color>, b: &Option<Color>, tolerance: u8) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(_), None) | (None, Some(_)) => tolerance == 255,
        (Some(c1), Some(c2)) => {
            if tolerance == 0 {
                c1 == c2
            } else {
                // Compare RGB components
                let (r1, g1, b1) = color_to_rgb(c1);
                let (r2, g2, b2) = color_to_rgb(c2);
                let dr = (r1 as i16 - r2 as i16).unsigned_abs() as u8;
                let dg = (g1 as i16 - g2 as i16).unsigned_abs() as u8;
                let db = (b1 as i16 - b2 as i16).unsigned_abs() as u8;
                dr <= tolerance && dg <= tolerance && db <= tolerance
            }
        }
    }
}

/// Convert Color to RGB tuple
fn color_to_rgb(color: &Color) -> (u8, u8, u8) {
    (color.r, color.g, color.b)
}

impl VisualCapture {
    /// Create from buffer
    pub fn from_buffer(buffer: &Buffer, config: &VisualTestConfig) -> Self {
        let width = buffer.width();
        let height = buffer.height();
        let mut cells = Vec::with_capacity((width * height) as usize);

        for y in 0..height {
            for x in 0..width {
                let cell = if let Some(buf_cell) = buffer.get(x, y) {
                    CapturedCell {
                        symbol: buf_cell.symbol,
                        fg: if config.include_colors {
                            buf_cell.fg
                        } else {
                            None
                        },
                        bg: if config.include_colors {
                            buf_cell.bg
                        } else {
                            None
                        },
                        bold: config.include_styles && buf_cell.modifier.contains(Modifier::BOLD),
                        italic: config.include_styles
                            && buf_cell.modifier.contains(Modifier::ITALIC),
                        underline: config.include_styles
                            && buf_cell.modifier.contains(Modifier::UNDERLINE),
                        dim: config.include_styles && buf_cell.modifier.contains(Modifier::DIM),
                    }
                } else {
                    CapturedCell::default()
                };
                cells.push(cell);
            }
        }

        Self {
            width,
            height,
            cells,
            include_styles: config.include_styles,
            include_colors: config.include_colors,
        }
    }

    /// Get cell at position
    pub fn get(&self, x: u16, y: u16) -> Option<&CapturedCell> {
        if x < self.width && y < self.height {
            let idx = (y * self.width + x) as usize;
            self.cells.get(idx)
        } else {
            None
        }
    }

    /// Compare with another capture
    pub fn diff(&self, other: &Self, tolerance: u8) -> VisualDiff {
        let mut differences = Vec::new();

        // Check size mismatch
        if self.width != other.width || self.height != other.height {
            return VisualDiff {
                size_mismatch: Some(((self.width, self.height), (other.width, other.height))),
                differences,
                actual_width: self.width,
                actual_height: self.height,
                expected_width: other.width,
                expected_height: other.height,
            };
        }

        // Compare cells
        for y in 0..self.height {
            for x in 0..self.width {
                let actual = self.get(x, y).unwrap();
                let expected = other.get(x, y).unwrap();

                if !actual.matches(
                    expected,
                    tolerance,
                    self.include_styles,
                    self.include_colors,
                ) {
                    differences.push(CellDiff {
                        x,
                        y,
                        actual: actual.clone(),
                        expected: expected.clone(),
                    });
                }
            }
        }

        VisualDiff {
            size_mismatch: None,
            differences,
            actual_width: self.width,
            actual_height: self.height,
            expected_width: other.width,
            expected_height: other.height,
        }
    }

    /// Save to file
    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        let content = self.serialize();
        fs::write(path, content)
    }

    /// Load from file
    pub fn load(path: &Path) -> std::io::Result<Self> {
        let content = fs::read_to_string(path)?;
        Self::deserialize(&content)
    }

    /// Serialize to string format
    fn serialize(&self) -> String {
        let mut output = String::new();

        // Header
        output.push_str(&format!(
            "# Visual Golden File\n# Size: {}x{}\n# Styles: {}\n# Colors: {}\n\n",
            self.width, self.height, self.include_styles, self.include_colors
        ));

        // Text layer
        output.push_str("## Text\n");
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(cell) = self.get(x, y) {
                    output.push(cell.symbol);
                } else {
                    output.push(' ');
                }
            }
            output.push('\n');
        }

        // Color layer (if included)
        if self.include_colors {
            output.push_str("\n## Colors\n");
            for y in 0..self.height {
                for x in 0..self.width {
                    if let Some(cell) = self.get(x, y) {
                        if let Some(fg) = &cell.fg {
                            let (r, g, b) = color_to_rgb(fg);
                            output.push_str(&format!(
                                "{}:{},{}:#{:02x}{:02x}{:02x} ",
                                x, y, "fg", r, g, b
                            ));
                        }
                        if let Some(bg) = &cell.bg {
                            let (r, g, b) = color_to_rgb(bg);
                            output.push_str(&format!(
                                "{}:{},{}:#{:02x}{:02x}{:02x} ",
                                x, y, "bg", r, g, b
                            ));
                        }
                    }
                }
            }
            output.push('\n');
        }

        // Style layer (if included)
        if self.include_styles {
            output.push_str("\n## Styles\n");
            for y in 0..self.height {
                for x in 0..self.width {
                    if let Some(cell) = self.get(x, y) {
                        let mut styles = Vec::new();
                        if cell.bold {
                            styles.push("B");
                        }
                        if cell.italic {
                            styles.push("I");
                        }
                        if cell.underline {
                            styles.push("U");
                        }
                        if cell.dim {
                            styles.push("D");
                        }
                        if !styles.is_empty() {
                            output.push_str(&format!("{}:{}:{} ", x, y, styles.join("")));
                        }
                    }
                }
            }
            output.push('\n');
        }

        output
    }

    /// Deserialize from string format
    fn deserialize(content: &str) -> std::io::Result<Self> {
        let mut width = 0u16;
        let mut height = 0u16;
        let mut include_styles = true;
        let mut include_colors = true;
        let mut cells = Vec::new();
        let mut in_text = false;
        let mut in_colors = false;
        let mut in_styles = false;
        let mut text_lines: Vec<String> = Vec::new();
        let mut color_data: HashMap<(u16, u16, String), (u8, u8, u8)> = HashMap::new();
        let mut style_data: HashMap<(u16, u16), (bool, bool, bool, bool)> = HashMap::new();

        for line in content.lines() {
            let line = line.trim_end();

            // Parse header
            if line.starts_with("# Size:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let size_parts: Vec<&str> = parts[2].split('x').collect();
                    if size_parts.len() == 2 {
                        width = size_parts[0].parse().unwrap_or(0);
                        height = size_parts[1].parse().unwrap_or(0);
                    }
                }
                continue;
            }
            if line.starts_with("# Styles:") {
                include_styles = line.contains("true");
                continue;
            }
            if line.starts_with("# Colors:") {
                include_colors = line.contains("true");
                continue;
            }

            // Section markers
            if line == "## Text" {
                in_text = true;
                in_colors = false;
                in_styles = false;
                continue;
            }
            if line == "## Colors" {
                in_text = false;
                in_colors = true;
                in_styles = false;
                continue;
            }
            if line == "## Styles" {
                in_text = false;
                in_colors = false;
                in_styles = true;
                continue;
            }

            if line.starts_with('#') || line.is_empty() {
                continue;
            }

            // Parse sections
            if in_text {
                text_lines.push(line.to_string());
            } else if in_colors {
                for part in line.split_whitespace() {
                    // Format: x:y,type:#rrggbb
                    if let Some((coord, color)) = part.split_once(',') {
                        if let Some((pos, hex)) = coord.split_once(':') {
                            if let Some((kind, hex_val)) = color.split_once(':') {
                                let x: u16 = pos.parse().unwrap_or(0);
                                let y: u16 = hex.parse().unwrap_or(0);
                                if let Some(rgb) = parse_hex_color(hex_val) {
                                    color_data.insert((x, y, kind.to_string()), rgb);
                                }
                            }
                        }
                    }
                }
            } else if in_styles {
                for part in line.split_whitespace() {
                    // Format: x:y:BIUD
                    let parts: Vec<&str> = part.split(':').collect();
                    if parts.len() >= 3 {
                        let x: u16 = parts[0].parse().unwrap_or(0);
                        let y: u16 = parts[1].parse().unwrap_or(0);
                        let flags = parts[2];
                        style_data.insert(
                            (x, y),
                            (
                                flags.contains('B'),
                                flags.contains('I'),
                                flags.contains('U'),
                                flags.contains('D'),
                            ),
                        );
                    }
                }
            }
        }

        // Build cells from text lines
        if height == 0 {
            height = text_lines.len() as u16;
        }
        if width == 0 && !text_lines.is_empty() {
            width = text_lines
                .iter()
                .map(|l| l.chars().count())
                .max()
                .unwrap_or(0) as u16;
        }

        for y in 0..height {
            let line = text_lines.get(y as usize).map(|s| s.as_str()).unwrap_or("");
            let chars: Vec<char> = line.chars().collect();

            for x in 0..width {
                let symbol = chars.get(x as usize).copied().unwrap_or(' ');
                let fg = color_data
                    .get(&(x, y, "fg".to_string()))
                    .map(|(r, g, b)| Color::rgb(*r, *g, *b));
                let bg = color_data
                    .get(&(x, y, "bg".to_string()))
                    .map(|(r, g, b)| Color::rgb(*r, *g, *b));
                let (bold, italic, underline, dim) = style_data
                    .get(&(x, y))
                    .copied()
                    .unwrap_or((false, false, false, false));

                cells.push(CapturedCell {
                    symbol,
                    fg,
                    bg,
                    bold,
                    italic,
                    underline,
                    dim,
                });
            }
        }

        Ok(Self {
            width,
            height,
            cells,
            include_styles,
            include_colors,
        })
    }
}

/// Parse hex color string like "#rrggbb"
fn parse_hex_color(s: &str) -> Option<(u8, u8, u8)> {
    let s = s.trim_start_matches('#');
    if s.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&s[0..2], 16).ok()?;
    let g = u8::from_str_radix(&s[2..4], 16).ok()?;
    let b = u8::from_str_radix(&s[4..6], 16).ok()?;
    Some((r, g, b))
}

// =============================================================================
// Visual Diff
// =============================================================================

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

// =============================================================================
// Tests
// =============================================================================

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
    fn test_visual_capture_from_buffer() {
        let buffer = make_buffer("Hello\nWorld");
        let config = VisualTestConfig::default();
        let capture = VisualCapture::from_buffer(&buffer, &config);

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
        let capture1 = VisualCapture::from_buffer(&buffer, &config);
        let capture2 = VisualCapture::from_buffer(&buffer, &config);

        let diff = capture1.diff(&capture2, 0);
        assert!(!diff.has_differences());
    }

    #[test]
    fn test_visual_capture_diff_different() {
        let buffer1 = make_buffer("Hello");
        let buffer2 = make_buffer("World");
        let config = VisualTestConfig::default();
        let capture1 = VisualCapture::from_buffer(&buffer1, &config);
        let capture2 = VisualCapture::from_buffer(&buffer2, &config);

        let diff = capture1.diff(&capture2, 0);
        assert!(diff.has_differences());
        assert!(!diff.differences.is_empty());
    }

    #[test]
    fn test_visual_capture_diff_size_mismatch() {
        let buffer1 = make_buffer("Hi");
        let buffer2 = make_buffer("Hello");
        let config = VisualTestConfig::default();
        let capture1 = VisualCapture::from_buffer(&buffer1, &config);
        let capture2 = VisualCapture::from_buffer(&buffer2, &config);

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
        assert_eq!(config.golden_dir, PathBuf::from("tests/golden"));
        assert_eq!(config.color_tolerance, 0);
        assert!(config.include_styles);
        assert!(config.include_colors);
    }

    #[test]
    fn test_serialize_deserialize() {
        let buffer = make_buffer("AB\nCD");
        let config = VisualTestConfig::default();
        let capture = VisualCapture::from_buffer(&buffer, &config);

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

    // =========================================================================
    // VisualTestConfig tests
    // =========================================================================

    #[test]
    fn test_config_with_dir() {
        let config = VisualTestConfig::with_dir("custom/path");
        assert_eq!(config.golden_dir, PathBuf::from("custom/path"));
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

    // =========================================================================
    // VisualTest tests
    // =========================================================================

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

    // =========================================================================
    // VisualTestResult tests
    // =========================================================================

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

    // =========================================================================
    // CapturedCell tests
    // =========================================================================

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

    // =========================================================================
    // Color matching tests
    // =========================================================================

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

    // =========================================================================
    // VisualCapture tests
    // =========================================================================

    #[test]
    fn test_capture_get_out_of_bounds() {
        let buffer = make_buffer("AB");
        let config = VisualTestConfig::default();
        let capture = VisualCapture::from_buffer(&buffer, &config);

        assert!(capture.get(0, 0).is_some());
        assert!(capture.get(100, 100).is_none());
    }

    #[test]
    fn test_capture_serialize_contains_header() {
        let buffer = make_buffer("Test");
        let config = VisualTestConfig::default();
        let capture = VisualCapture::from_buffer(&buffer, &config);

        let serialized = capture.serialize();
        assert!(serialized.contains("# Visual Golden File"));
        assert!(serialized.contains("# Size:"));
        assert!(serialized.contains("## Text"));
    }

    #[test]
    fn test_capture_serialize_contains_text() {
        let buffer = make_buffer("Hello\nWorld");
        let config = VisualTestConfig::default();
        let capture = VisualCapture::from_buffer(&buffer, &config);

        let serialized = capture.serialize();
        assert!(serialized.contains("Hello"));
        assert!(serialized.contains("World"));
    }

    // =========================================================================
    // VisualDiff tests
    // =========================================================================

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
            differences: vec![CellDiff {
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
                CellDiff {
                    x: 0,
                    y: 0,
                    actual: CapturedCell::from_char('A'),
                    expected: CapturedCell::from_char('B'),
                },
                CellDiff {
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
            differences.push(CellDiff {
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

    // =========================================================================
    // Parse hex color tests
    // =========================================================================

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
