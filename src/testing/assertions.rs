//! Assertion utilities for testing

use crate::render::Buffer;

/// Result of an assertion
#[derive(Debug, Clone)]
pub enum AssertionResult {
    /// Assertion passed
    Pass,
    /// Assertion failed with message
    Fail(String),
}

impl AssertionResult {
    /// Check if passed
    pub fn is_pass(&self) -> bool {
        matches!(self, AssertionResult::Pass)
    }

    /// Check if failed
    pub fn is_fail(&self) -> bool {
        matches!(self, AssertionResult::Fail(_))
    }

    /// Unwrap or panic with message
    pub fn unwrap(self) {
        if let AssertionResult::Fail(msg) = self {
            panic!("Assertion failed: {}", msg);
        }
    }
}

/// An assertion that can be run against a buffer
pub trait Assertion {
    /// Run the assertion
    fn check(&self, buffer: &Buffer) -> AssertionResult;

    /// Get assertion description
    fn description(&self) -> String;
}

/// Assert that screen contains text
#[cfg(test)]
pub struct ContainsText {
    text: String,
}

#[cfg(test)]
impl ContainsText {
    /// Create new assertion
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

#[cfg(test)]
impl Assertion for ContainsText {
    fn check(&self, buffer: &Buffer) -> AssertionResult {
        let screen = buffer_to_string(buffer);
        if screen.contains(&self.text) {
            AssertionResult::Pass
        } else {
            AssertionResult::Fail(format!(
                "Expected screen to contain '{}', but it didn't.\nScreen:\n{}",
                self.text, screen
            ))
        }
    }

    fn description(&self) -> String {
        format!("Screen contains '{}'", self.text)
    }
}

/// Assert that screen does not contain text
#[cfg(test)]
pub struct NotContainsText {
    text: String,
}

#[cfg(test)]
impl NotContainsText {
    /// Create new assertion
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

#[cfg(test)]
impl Assertion for NotContainsText {
    fn check(&self, buffer: &Buffer) -> AssertionResult {
        let screen = buffer_to_string(buffer);
        if !screen.contains(&self.text) {
            AssertionResult::Pass
        } else {
            AssertionResult::Fail(format!(
                "Expected screen NOT to contain '{}', but it did.\nScreen:\n{}",
                self.text, screen
            ))
        }
    }

    fn description(&self) -> String {
        format!("Screen does not contain '{}'", self.text)
    }
}

/// Assert that a specific line contains text
#[cfg(test)]
pub struct LineContains {
    line: u16,
    text: String,
}

#[cfg(test)]
impl LineContains {
    /// Create new assertion
    pub fn new(line: u16, text: impl Into<String>) -> Self {
        Self {
            line,
            text: text.into(),
        }
    }
}

#[cfg(test)]
impl Assertion for LineContains {
    fn check(&self, buffer: &Buffer) -> AssertionResult {
        let line_text = get_line(buffer, self.line);
        if line_text.contains(&self.text) {
            AssertionResult::Pass
        } else {
            AssertionResult::Fail(format!(
                "Expected line {} to contain '{}', but got: '{}'",
                self.line, self.text, line_text
            ))
        }
    }

    fn description(&self) -> String {
        format!("Line {} contains '{}'", self.line, self.text)
    }
}

/// Assert cell has specific character
#[cfg(test)]
pub struct CellEquals {
    x: u16,
    y: u16,
    expected: char,
}

#[cfg(test)]
impl CellEquals {
    /// Create new assertion
    pub fn new(x: u16, y: u16, expected: char) -> Self {
        Self { x, y, expected }
    }
}

#[cfg(test)]
impl Assertion for CellEquals {
    fn check(&self, buffer: &Buffer) -> AssertionResult {
        if let Some(cell) = buffer.get(self.x, self.y) {
            if cell.symbol == self.expected {
                AssertionResult::Pass
            } else {
                AssertionResult::Fail(format!(
                    "Expected cell ({}, {}) to be '{}', but got '{}'",
                    self.x, self.y, self.expected, cell.symbol
                ))
            }
        } else {
            AssertionResult::Fail(format!(
                "Cell ({}, {}) is out of bounds",
                self.x, self.y
            ))
        }
    }

    fn description(&self) -> String {
        format!("Cell ({}, {}) equals '{}'", self.x, self.y, self.expected)
    }
}

/// Assert screen matches exact text
#[cfg(test)]
pub struct ScreenEquals {
    expected: String,
}

#[cfg(test)]
impl ScreenEquals {
    /// Create new assertion
    pub fn new(expected: impl Into<String>) -> Self {
        Self {
            expected: expected.into(),
        }
    }
}

#[cfg(test)]
impl Assertion for ScreenEquals {
    fn check(&self, buffer: &Buffer) -> AssertionResult {
        let actual = buffer_to_string(buffer);
        let expected_trimmed = self.expected.trim();
        let actual_trimmed = actual.trim();

        if actual_trimmed == expected_trimmed {
            AssertionResult::Pass
        } else {
            AssertionResult::Fail(format!(
                "Screen does not match expected.\nExpected:\n{}\n\nActual:\n{}",
                expected_trimmed, actual_trimmed
            ))
        }
    }

    fn description(&self) -> String {
        "Screen matches expected text".to_string()
    }
}

// Helper functions

#[cfg(test)]
fn buffer_to_string(buffer: &Buffer) -> String {
    let mut lines = Vec::new();
    for y in 0..buffer.height() {
        let mut line = String::new();
        for x in 0..buffer.width() {
            if let Some(cell) = buffer.get(x, y) {
                line.push(cell.symbol);
            } else {
                line.push(' ');
            }
        }
        lines.push(line.trim_end().to_string());
    }

    // Remove trailing empty lines
    while lines.last().map(|l| l.is_empty()).unwrap_or(false) {
        lines.pop();
    }

    lines.join("\n")
}

#[cfg(test)]
fn get_line(buffer: &Buffer, row: u16) -> String {
    if row >= buffer.height() {
        return String::new();
    }

    let mut line = String::new();
    for x in 0..buffer.width() {
        if let Some(cell) = buffer.get(x, row) {
            line.push(cell.symbol);
        } else {
            line.push(' ');
        }
    }
    line.trim_end().to_string()
}

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
    fn test_contains_text_pass() {
        let buffer = make_buffer("Hello, World!");
        let assertion = ContainsText::new("World");
        assert!(assertion.check(&buffer).is_pass());
    }

    #[test]
    fn test_contains_text_fail() {
        let buffer = make_buffer("Hello, World!");
        let assertion = ContainsText::new("Goodbye");
        assert!(assertion.check(&buffer).is_fail());
    }

    #[test]
    fn test_not_contains_text() {
        let buffer = make_buffer("Hello, World!");
        let assertion = NotContainsText::new("Goodbye");
        assert!(assertion.check(&buffer).is_pass());
    }

    #[test]
    fn test_line_contains() {
        let buffer = make_buffer("Line 1\nLine 2\nLine 3");
        let assertion = LineContains::new(1, "Line 2");
        assert!(assertion.check(&buffer).is_pass());
    }

    #[test]
    fn test_cell_equals() {
        let buffer = make_buffer("ABC");
        let assertion = CellEquals::new(1, 0, 'B');
        assert!(assertion.check(&buffer).is_pass());
    }
}
