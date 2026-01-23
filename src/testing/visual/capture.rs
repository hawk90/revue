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
