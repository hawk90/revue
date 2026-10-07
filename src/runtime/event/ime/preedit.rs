//! Preedit (composition) string segments for rendering

use super::ImeState;

// =============================================================================
// Preedit String Builder
// =============================================================================

/// Builder for rendering preedit (composition) string with styling
#[derive(Debug, Clone)]
pub struct PreeditString {
    /// Segments of the preedit string
    segments: Vec<PreeditSegment>,
}

/// A segment of preedit text with styling
#[derive(Debug, Clone)]
pub struct PreeditSegment {
    /// The text content
    pub text: String,
    /// Whether this segment is highlighted (selected)
    pub highlighted: bool,
    /// Whether this segment has the cursor
    pub has_cursor: bool,
    /// Cursor position within segment (if has_cursor)
    pub cursor_pos: usize,
}

impl PreeditString {
    /// Create new preedit string
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }

    /// Create from IME state
    pub fn from_ime(ime: &ImeState) -> Self {
        let text = ime.composing_text();
        let cursor = ime.cursor();

        if text.is_empty() {
            return Self::new();
        }

        // Split text at cursor position
        let chars: Vec<char> = text.chars().collect();
        let (before, after) = chars.split_at(cursor.min(chars.len()));

        let mut preedit = Self::new();

        if !before.is_empty() {
            preedit.segments.push(PreeditSegment {
                text: before.iter().collect(),
                highlighted: false,
                has_cursor: false,
                cursor_pos: 0,
            });
        }

        // Add cursor marker segment
        preedit.segments.push(PreeditSegment {
            text: String::new(),
            highlighted: false,
            has_cursor: true,
            cursor_pos: 0,
        });

        if !after.is_empty() {
            preedit.segments.push(PreeditSegment {
                text: after.iter().collect(),
                highlighted: false,
                has_cursor: false,
                cursor_pos: 0,
            });
        }

        preedit
    }

    /// Add a segment
    pub fn add_segment(&mut self, segment: PreeditSegment) {
        self.segments.push(segment);
    }

    /// Get all segments
    pub fn segments(&self) -> &[PreeditSegment] {
        &self.segments
    }

    /// Get full text
    pub fn text(&self) -> String {
        self.segments.iter().map(|s| s.text.as_str()).collect()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty() || self.text().is_empty()
    }
}

impl Default for PreeditString {
    fn default() -> Self {
        Self::new()
    }
}
