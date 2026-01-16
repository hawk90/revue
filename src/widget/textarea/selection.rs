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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_new() {
        let sel = Selection::new((1, 5), (3, 10));
        assert_eq!(sel.start, (1, 5));
        assert_eq!(sel.end, (3, 10));
    }

    #[test]
    fn test_selection_normalized_already_normalized() {
        let sel = Selection::new((1, 5), (3, 10));
        let norm = sel.normalized();
        assert_eq!(norm.start, (1, 5));
        assert_eq!(norm.end, (3, 10));
    }

    #[test]
    fn test_selection_normalized_reversed_lines() {
        let sel = Selection::new((3, 10), (1, 5));
        let norm = sel.normalized();
        assert_eq!(norm.start, (1, 5));
        assert_eq!(norm.end, (3, 10));
    }

    #[test]
    fn test_selection_normalized_same_line_reversed() {
        let sel = Selection::new((2, 10), (2, 5));
        let norm = sel.normalized();
        assert_eq!(norm.start, (2, 5));
        assert_eq!(norm.end, (2, 10));
    }

    #[test]
    fn test_selection_normalized_same_line_already_normalized() {
        let sel = Selection::new((2, 5), (2, 10));
        let norm = sel.normalized();
        assert_eq!(norm.start, (2, 5));
        assert_eq!(norm.end, (2, 10));
    }

    #[test]
    fn test_selection_contains_single_line() {
        let sel = Selection::new((2, 5), (2, 10));
        // Within selection
        assert!(sel.contains(2, 5));
        assert!(sel.contains(2, 7));
        assert!(sel.contains(2, 9));
        // Outside selection (end is exclusive)
        assert!(!sel.contains(2, 10));
        assert!(!sel.contains(2, 4));
        assert!(!sel.contains(1, 7));
        assert!(!sel.contains(3, 7));
    }

    #[test]
    fn test_selection_contains_multi_line() {
        let sel = Selection::new((1, 5), (3, 10));
        // First line - from column 5 onwards
        assert!(sel.contains(1, 5));
        assert!(sel.contains(1, 10));
        assert!(sel.contains(1, 100));
        assert!(!sel.contains(1, 4));
        // Middle line - all columns
        assert!(sel.contains(2, 0));
        assert!(sel.contains(2, 50));
        // Last line - up to column 10 (exclusive)
        assert!(sel.contains(3, 0));
        assert!(sel.contains(3, 9));
        assert!(!sel.contains(3, 10));
        assert!(!sel.contains(3, 15));
        // Outside lines
        assert!(!sel.contains(0, 5));
        assert!(!sel.contains(4, 5));
    }

    #[test]
    fn test_selection_contains_reversed_selection() {
        // Selection specified backwards should still work
        let sel = Selection::new((3, 10), (1, 5));
        assert!(sel.contains(2, 0));
        assert!(sel.contains(1, 5));
        assert!(!sel.contains(1, 4));
    }

    #[test]
    fn test_selection_contains_line_before_start() {
        let sel = Selection::new((5, 0), (10, 5));
        assert!(!sel.contains(4, 0));
        assert!(!sel.contains(4, 100));
    }

    #[test]
    fn test_selection_contains_line_after_end() {
        let sel = Selection::new((5, 0), (10, 5));
        assert!(!sel.contains(11, 0));
        assert!(!sel.contains(100, 0));
    }
}
