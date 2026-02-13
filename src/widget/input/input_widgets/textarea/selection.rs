//! Selection handling for TextArea

/// A text selection range
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Selection {
    /// Start position (line, column)
    pub start: (usize, usize),
    /// End position (line, column)
    pub end: (usize, usize),
}

impl Selection {
    /// Create a new selection
    pub fn new(start: (usize, usize), end: (usize, usize)) -> Self {
        Self { start, end }
    }

    /// Get the normalized selection (start before end)
    pub fn normalized(&self) -> Self {
        if self.start.0 > self.end.0 || (self.start.0 == self.end.0 && self.start.1 > self.end.1) {
            Self {
                start: self.end,
                end: self.start,
            }
        } else {
            *self
        }
    }

    /// Check if a position is within the selection
    pub fn contains(&self, line: usize, col: usize) -> bool {
        let norm = self.normalized();
        if line < norm.start.0 || line > norm.end.0 {
            return false;
        }
        if line == norm.start.0 && line == norm.end.0 {
            col >= norm.start.1 && col < norm.end.1
        } else if line == norm.start.0 {
            col >= norm.start.1
        } else if line == norm.end.0 {
            col < norm.end.1
        } else {
            true
        }
    }
}

// # KEEP HERE - Private tests that cannot be extracted
