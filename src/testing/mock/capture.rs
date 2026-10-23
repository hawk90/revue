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

    /// Get all text content: a wide glyph is one character, trailing
    /// spaces and blank rows are trimmed
    pub fn text(&self) -> String {
        crate::testing::screen_text::screen_text(&self.buffer)
    }

    /// Get a specific line, trailing spaces trimmed
    pub fn line(&self, row: u16) -> String {
        crate::testing::screen_text::row_text(&self.buffer, row)
    }

    /// Check if contains text
    pub fn contains(&self, text: &str) -> bool {
        self.text().contains(text)
    }

    /// Find text position: the column and row of its first occurrence
    pub fn find(&self, text: &str) -> Option<(u16, u16)> {
        crate::testing::screen_text::find(&self.buffer, text)
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
