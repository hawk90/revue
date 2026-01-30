//! Visual capture implementation for buffers

use crate::render::{Buffer, Modifier};
use crate::style::Color;
use crate::testing::visual::types::{CapturedCell, VisualCapture};
use crate::testing::visual::{comparison::color_to_rgb, helpers::parse_hex_color};
use std::collections::HashMap;
use std::path::Path;

impl VisualCapture {
    /// Create from buffer
    pub fn from_buffer(buffer: &Buffer, include_styles: bool, include_colors: bool) -> Self {
        let width = buffer.width();
        let height = buffer.height();
        let mut cells = Vec::with_capacity((width * height) as usize);

        for y in 0..height {
            for x in 0..width {
                let cell = if let Some(buf_cell) = buffer.get(x, y) {
                    CapturedCell {
                        symbol: buf_cell.symbol,
                        fg: if include_colors { buf_cell.fg } else { None },
                        bg: if include_colors { buf_cell.bg } else { None },
                        bold: include_styles && buf_cell.modifier.contains(Modifier::BOLD),
                        italic: include_styles && buf_cell.modifier.contains(Modifier::ITALIC),
                        underline: include_styles
                            && buf_cell.modifier.contains(Modifier::UNDERLINE),
                        dim: include_styles && buf_cell.modifier.contains(Modifier::DIM),
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
            include_styles,
            include_colors,
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
    pub fn diff(&self, other: &Self, tolerance: u8) -> crate::testing::visual::types::VisualDiff {
        let mut differences = Vec::new();

        // Check size mismatch
        if self.width != other.width || self.height != other.height {
            return crate::testing::visual::types::VisualDiff {
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
                    differences.push(crate::testing::visual::types::CellDiff {
                        x,
                        y,
                        actual: actual.clone(),
                        expected: expected.clone(),
                    });
                }
            }
        }

        crate::testing::visual::types::VisualDiff {
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
        std::fs::write(path, content)
    }

    /// Load from file
    pub fn load(path: &Path) -> std::io::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Self::deserialize(&content)
    }

    /// Serialize to string format
    pub fn serialize(&self) -> String {
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
                output.push('\n');
            }
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
                output.push('\n');
            }
        }

        output
    }

    /// Deserialize from string format
    pub fn deserialize(content: &str) -> std::io::Result<Self> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Cell;

    fn create_test_buffer(width: u16, height: u16) -> Buffer {
        let mut buffer = Buffer::new(width, height);
        // Add some test content
        for y in 0..height {
            for x in 0..width {
                let digit = ((x + y) % 10) as u8 + b'0';
                let mut cell = Cell::new(digit as char);
                if x == 0 && y == 0 {
                    cell.fg = Some(Color::RED);
                    cell.modifier |= Modifier::BOLD;
                }
                if x == 1 && y == 0 {
                    cell.bg = Some(Color::BLUE);
                    cell.modifier |= Modifier::ITALIC;
                }
                buffer.set(x, y, cell);
            }
        }
        buffer
    }

    #[test]
    fn test_from_buffer_basic() {
        let buffer = create_test_buffer(5, 3);
        let capture = VisualCapture::from_buffer(&buffer, false, false);

        assert_eq!(capture.width, 5);
        assert_eq!(capture.height, 3);
        assert_eq!(capture.cells.len(), 15);
        assert!(!capture.include_styles);
        assert!(!capture.include_colors);
    }

    #[test]
    fn test_from_buffer_with_styles() {
        let buffer = create_test_buffer(5, 3);
        let capture = VisualCapture::from_buffer(&buffer, true, false);

        assert!(capture.include_styles);
        // First cell should have bold
        assert!(capture.cells[0].bold);
        // Second cell should have italic
        assert!(capture.cells[1].italic);
    }

    #[test]
    fn test_from_buffer_with_colors() {
        let buffer = create_test_buffer(5, 3);
        let capture = VisualCapture::from_buffer(&buffer, false, true);

        assert!(capture.include_colors);
        // First cell should have red fg
        assert_eq!(capture.cells[0].fg, Some(Color::RED));
        // Second cell should have blue bg
        assert_eq!(capture.cells[1].bg, Some(Color::BLUE));
    }

    #[test]
    fn test_from_buffer_empty_cell() {
        let buffer = Buffer::new(3, 3);
        // Don't set anything - cells should be empty

        let capture = VisualCapture::from_buffer(&buffer, false, false);
        assert_eq!(capture.cells.len(), 9);
    }

    #[test]
    fn test_get_valid_position() {
        let buffer = create_test_buffer(5, 3);
        let capture = VisualCapture::from_buffer(&buffer, false, false);

        assert!(capture.get(0, 0).is_some());
        assert!(capture.get(4, 2).is_some());
        assert!(capture.get(2, 1).is_some());
    }

    #[test]
    fn test_get_out_of_bounds() {
        let buffer = create_test_buffer(5, 3);
        let capture = VisualCapture::from_buffer(&buffer, false, false);

        assert!(capture.get(5, 0).is_none()); // x out of bounds
        assert!(capture.get(0, 3).is_none()); // y out of bounds
        assert!(capture.get(10, 10).is_none()); // both out of bounds
    }

    #[test]
    fn test_get_index_calculation() {
        let buffer = create_test_buffer(5, 3);
        let capture = VisualCapture::from_buffer(&buffer, false, false);

        // Index should be y * width + x
        // At (2, 1): index = 1 * 5 + 2 = 7
        let cell = capture.get(2, 1).unwrap();
        assert_eq!(capture.cells[7].symbol, cell.symbol);
    }

    #[test]
    fn test_diff_identical() {
        let buffer = create_test_buffer(5, 3);
        let capture1 = VisualCapture::from_buffer(&buffer, false, false);
        let capture2 = VisualCapture::from_buffer(&buffer, false, false);

        let diff = capture1.diff(&capture2, 0);
        assert!(diff.size_mismatch.is_none());
        assert!(diff.differences.is_empty());
    }

    #[test]
    fn test_diff_size_mismatch() {
        let buffer1 = create_test_buffer(5, 3);
        let buffer2 = create_test_buffer(4, 2);
        let capture1 = VisualCapture::from_buffer(&buffer1, false, false);
        let capture2 = VisualCapture::from_buffer(&buffer2, false, false);

        let diff = capture1.diff(&capture2, 0);
        assert!(diff.size_mismatch.is_some());
        assert_eq!(diff.actual_width, 5);
        assert_eq!(diff.actual_height, 3);
        assert_eq!(diff.expected_width, 4);
        assert_eq!(diff.expected_height, 2);
    }

    #[test]
    fn test_diff_cell_difference() {
        let mut buffer1 = Buffer::new(3, 3);
        let cell1 = Cell::new('A');
        buffer1.set(0, 0, cell1);

        let mut buffer2 = Buffer::new(3, 3);
        let cell2 = Cell::new('B');
        buffer2.set(0, 0, cell2);

        let capture1 = VisualCapture::from_buffer(&buffer1, false, false);
        let capture2 = VisualCapture::from_buffer(&buffer2, false, false);

        let diff = capture1.diff(&capture2, 0);
        assert!(diff.size_mismatch.is_none());
        assert_eq!(diff.differences.len(), 1);
        assert_eq!(diff.differences[0].x, 0);
        assert_eq!(diff.differences[0].y, 0);
    }

    #[test]
    fn test_serialize_text_section() {
        let buffer = create_test_buffer(3, 2);
        let capture = VisualCapture::from_buffer(&buffer, false, false);

        let serialized = capture.serialize();
        assert!(serialized.contains("# Visual Golden File"));
        assert!(serialized.contains("# Size: 3x2"));
        assert!(serialized.contains("## Text"));
    }

    #[test]
    fn test_serialize_with_styles() {
        let buffer = create_test_buffer(3, 2);
        let capture = VisualCapture::from_buffer(&buffer, true, false);

        let serialized = capture.serialize();
        assert!(serialized.contains("# Styles: true"));
        assert!(serialized.contains("## Styles"));
    }

    #[test]
    fn test_serialize_with_colors() {
        let buffer = create_test_buffer(3, 2);
        let capture = VisualCapture::from_buffer(&buffer, false, true);

        let serialized = capture.serialize();
        assert!(serialized.contains("# Colors: true"));
        assert!(serialized.contains("## Colors"));
    }

    #[test]
    fn test_serialize_color_format() {
        let buffer = create_test_buffer(3, 2);
        let capture = VisualCapture::from_buffer(&buffer, false, true);

        let serialized = capture.serialize();
        // Red should be #ff0000, Blue should be #0000ff
        assert!(serialized.contains("#ff0000") || serialized.contains("#FF0000"));
    }

    #[test]
    fn test_serialize_style_format() {
        let buffer = create_test_buffer(3, 2);
        let capture = VisualCapture::from_buffer(&buffer, true, false);

        let serialized = capture.serialize();
        // Should contain B for bold and I for italic
        assert!(serialized.contains('B') || serialized.contains('I'));
    }

    #[test]
    fn test_deserialize_basic() {
        let content = "# Visual Golden File
# Size: 2x2
# Styles: false
# Colors: false

## Text
AB
CD";

        let capture = VisualCapture::deserialize(content).unwrap();
        assert_eq!(capture.width, 2);
        assert_eq!(capture.height, 2);
        assert_eq!(capture.cells[0].symbol, 'A');
        assert_eq!(capture.cells[1].symbol, 'B');
        assert_eq!(capture.cells[2].symbol, 'C');
        assert_eq!(capture.cells[3].symbol, 'D');
    }

    #[test]
    fn test_deserialize_with_styles() {
        let content = "# Visual Golden File
# Size: 2x1
# Styles: true
# Colors: false

## Text
AB

## Styles
0:0:B
1:0:I";

        let capture = VisualCapture::deserialize(content).unwrap();
        assert!(capture.include_styles);
        assert!(capture.cells[0].bold);
        assert!(capture.cells[1].italic);
    }

    #[test]
    fn test_deserialize_with_colors() {
        let content = "# Visual Golden File
# Size: 2x1
# Styles: false
# Colors: true

## Text
AB

## Colors
0:0,fg:#ff0000
1:0,bg:#0000ff";

        let capture = VisualCapture::deserialize(content).unwrap();
        assert!(capture.include_colors);
        assert_eq!(capture.cells[0].fg, Some(Color::rgb(255, 0, 0)));
        assert_eq!(capture.cells[1].bg, Some(Color::rgb(0, 0, 255)));
    }

    #[test]
    fn test_deserialize_infer_size() {
        let content = "# Visual Golden File
# Size: 0x0
# Styles: false
# Colors: false

## Text
ABC
DE";

        let capture = VisualCapture::deserialize(content).unwrap();
        assert_eq!(capture.width, 3); // Longest line
        assert_eq!(capture.height, 2); // Number of text lines
    }

    #[test]
    fn test_deserialize_partial_colors() {
        let content = "# Visual Golden File
# Size: 2x2
# Styles: false
# Colors: true

## Text
AB
CD

## Colors
0:0,fg:#ff0000
1:1,bg:#00ff00";

        let capture = VisualCapture::deserialize(content).unwrap();
        // (0,0) should have red fg
        assert_eq!(capture.cells[0].fg, Some(Color::rgb(255, 0, 0)));
        // (1,1) should have green bg
        assert_eq!(capture.cells[3].bg, Some(Color::rgb(0, 255, 0)));
        // Other cells should have no colors
        assert!(capture.cells[1].fg.is_none());
        assert!(capture.cells[2].bg.is_none());
    }

    #[test]
    fn test_deserialize_multiple_styles_per_cell() {
        let content = "# Visual Golden File
# Size: 1x1
# Styles: true
# Colors: false

## Text
A

## Styles
0:0:BIUD";

        let capture = VisualCapture::deserialize(content).unwrap();
        assert!(capture.cells[0].bold);
        assert!(capture.cells[0].italic);
        assert!(capture.cells[0].underline);
        assert!(capture.cells[0].dim);
    }

    #[test]
    fn test_deserialize_with_comments() {
        let content = "# Visual Golden File
# Size: 0x0
# Styles: false
# Colors: false

## Text
A
# This is a comment in the text section (should be skipped)
B";

        let capture = VisualCapture::deserialize(content).unwrap();
        // Size 0x0 means infer from text lines
        // We have 2 text lines ("A" and "B") - comment is skipped
        assert_eq!(capture.height, 2);
        assert_eq!(capture.width, 1);
        assert_eq!(capture.cells[0].symbol, 'A');
        assert_eq!(capture.cells[1].symbol, 'B');
    }

    #[test]
    fn test_deserialize_empty_content() {
        let content = "";
        let result = VisualCapture::deserialize(content);
        // Should create a 0x0 capture
        assert!(result.is_ok());
        let capture = result.unwrap();
        assert_eq!(capture.width, 0);
        assert_eq!(capture.height, 0);
    }

    #[test]
    fn test_serialize_roundtrip() {
        let buffer = create_test_buffer(3, 2);
        let original = VisualCapture::from_buffer(&buffer, true, true);

        let serialized = original.serialize();
        let deserialized = VisualCapture::deserialize(&serialized).unwrap();

        assert_eq!(original.width, deserialized.width);
        assert_eq!(original.height, deserialized.height);
        assert_eq!(original.cells.len(), deserialized.cells.len());
    }

    #[test]
    fn test_diff_with_tolerance() {
        let mut buffer1 = Buffer::new(2, 2);
        let mut cell = Cell::new('A');
        cell.fg = Some(Color::rgb(255, 0, 0));
        buffer1.set(0, 0, cell);

        let mut buffer2 = Buffer::new(2, 2);
        let mut cell2 = Cell::new('A');
        cell2.fg = Some(Color::rgb(254, 0, 0)); // Very similar color
        buffer2.set(0, 0, cell2);

        let capture1 = VisualCapture::from_buffer(&buffer1, false, true);
        let capture2 = VisualCapture::from_buffer(&buffer2, false, true);

        // With tolerance 0, should have difference
        let diff = capture1.diff(&capture2, 0);
        assert!(!diff.differences.is_empty());

        // With tolerance 10, should match
        let diff = capture1.diff(&capture2, 10);
        assert!(diff.differences.is_empty());
    }

    #[test]
    fn test_serialize_includes_all_flags() {
        let content = "# Visual Golden File
# Size: 1x1
# Styles: true
# Colors: true

## Text
A

## Colors
0:0,fg:#ff0000 0:0,bg:#00ff00

## Styles
0:0:BIUD";

        let capture = VisualCapture::deserialize(content).unwrap();
        let serialized = capture.serialize();

        assert!(serialized.contains("# Styles: true"));
        assert!(serialized.contains("# Colors: true"));
        assert!(serialized.contains("## Text"));
        assert!(serialized.contains("## Colors"));
        assert!(serialized.contains("## Styles"));
    }
}
