//! Captured render output for assertions

use crate::render::Buffer;

/// Captured render output for assertions
#[derive(Debug, Clone)]
pub struct RenderCapture {
    buffer: Buffer,
    width: u16,
    height: u16,
}

impl RenderCapture {
    /// Create a new render capture
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            buffer: Buffer::new(width, height),
            width,
            height,
        }
    }

    /// Create from existing buffer with dimensions
    pub fn from_buffer(buffer: Buffer, width: u16, height: u16) -> Self {
        Self {
            buffer,
            width,
            height,
        }
    }

    /// Get buffer reference
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// Get mutable buffer reference
    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }

    /// Get size
    pub fn size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    /// Get all text content
    pub fn text(&self) -> String {
        let mut lines = Vec::new();
        for y in 0..self.height {
            let mut line = String::new();
            for x in 0..self.width {
                if let Some(cell) = self.buffer.get(x, y) {
                    line.push(cell.symbol);
                } else {
                    line.push(' ');
                }
            }
            lines.push(line.trim_end().to_string());
        }
        // Remove trailing empty lines
        while lines.last().is_some_and(|l| l.is_empty()) {
            lines.pop();
        }
        lines.join("\n")
    }

    /// Get a specific line
    pub fn line(&self, row: u16) -> String {
        if row >= self.height {
            return String::new();
        }
        let mut line = String::new();
        for x in 0..self.width {
            if let Some(cell) = self.buffer.get(x, row) {
                line.push(cell.symbol);
            } else {
                line.push(' ');
            }
        }
        line.trim_end().to_string()
    }

    /// Check if contains text
    pub fn contains(&self, text: &str) -> bool {
        self.text().contains(text)
    }

    /// Find text position
    pub fn find(&self, text: &str) -> Option<(u16, u16)> {
        for y in 0..self.height {
            let line = self.line(y);
            if let Some(x) = line.find(text) {
                return Some((x as u16, y));
            }
        }
        None
    }

    /// Get character at position
    pub fn char_at(&self, x: u16, y: u16) -> Option<char> {
        self.buffer.get(x, y).map(|c| c.symbol)
    }

    /// Count occurrences of a character
    pub fn count_char(&self, ch: char) -> usize {
        self.text().chars().filter(|&c| c == ch).count()
    }

    /// Count occurrences of a string
    pub fn count_str(&self, s: &str) -> usize {
        self.text().matches(s).count()
    }

    /// Clear the capture
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Compare with another capture
    pub fn diff(&self, other: &RenderCapture) -> Vec<(u16, u16, char, char)> {
        let mut diffs = Vec::new();
        let max_y = self.height.max(other.height);
        let max_x = self.width.max(other.width);

        for y in 0..max_y {
            for x in 0..max_x {
                let c1 = self.char_at(x, y).unwrap_or(' ');
                let c2 = other.char_at(x, y).unwrap_or(' ');
                if c1 != c2 {
                    diffs.push((x, y, c1, c2));
                }
            }
        }
        diffs
    }
}
