//! Rich error messages for CSS parsing
//!
//! Provides detailed error messages with source snippets, suggestions,
//! and error codes for easy debugging.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::style::parse_css;
//!
//! let css = ".button { colr: red; }"; // typo in "color"
//!
//! match parse_css(css) {
//!     Ok(_) => println!("Parsed successfully"),
//!     Err(e) => {
//!         // Rich error output with suggestions
//!         eprintln!("{}", e.pretty_print(css));
//!     }
//! }
//! ```

use std::fmt;

// =============================================================================
// Error Code
// =============================================================================

/// CSS error codes for documentation lookup
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    /// E001: Invalid syntax
    InvalidSyntax,
    /// E002: Unknown property
    UnknownProperty,
    /// E003: Invalid value for property
    InvalidValue,
    /// E004: Missing closing brace
    MissingBrace,
    /// E005: Missing semicolon
    MissingSemicolon,
    /// E006: Invalid selector
    InvalidSelector,
    /// E007: Undefined variable
    UndefinedVariable,
    /// E008: Invalid color format
    InvalidColor,
    /// E009: Invalid number/unit
    InvalidNumber,
    /// E010: Empty rule
    EmptyRule,
}

impl ErrorCode {
    /// Get the error code string (e.g., "E001")
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidSyntax => "E001",
            Self::UnknownProperty => "E002",
            Self::InvalidValue => "E003",
            Self::MissingBrace => "E004",
            Self::MissingSemicolon => "E005",
            Self::InvalidSelector => "E006",
            Self::UndefinedVariable => "E007",
            Self::InvalidColor => "E008",
            Self::InvalidNumber => "E009",
            Self::EmptyRule => "E010",
        }
    }

    /// Get a short description
    pub fn description(&self) -> &'static str {
        match self {
            Self::InvalidSyntax => "invalid syntax",
            Self::UnknownProperty => "unknown CSS property",
            Self::InvalidValue => "invalid property value",
            Self::MissingBrace => "missing closing brace '}'",
            Self::MissingSemicolon => "missing semicolon ';'",
            Self::InvalidSelector => "invalid CSS selector",
            Self::UndefinedVariable => "undefined CSS variable",
            Self::InvalidColor => "invalid color format",
            Self::InvalidNumber => "invalid number or unit",
            Self::EmptyRule => "empty CSS rule",
        }
    }

    /// Get help text with more details
    pub fn help(&self) -> &'static str {
        match self {
            Self::InvalidSyntax =>
                "Check for mismatched brackets, quotes, or unexpected characters",
            Self::UnknownProperty =>
                "Check spelling or see the supported properties list",
            Self::InvalidValue =>
                "The value format doesn't match what this property expects",
            Self::MissingBrace =>
                "Every '{' must have a matching '}'",
            Self::MissingSemicolon =>
                "Each CSS declaration should end with ';'",
            Self::InvalidSelector =>
                "Selectors should be like '.class', '#id', or 'element'",
            Self::UndefinedVariable =>
                "Define variables in :root { --name: value; }",
            Self::InvalidColor =>
                "Use formats like #rgb, #rrggbb, rgb(r,g,b), or named colors",
            Self::InvalidNumber =>
                "Numbers should be like '10', '10px', '50%', or '0.5'",
            Self::EmptyRule =>
                "Add at least one property declaration inside the rule",
        }
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

// =============================================================================
// Error Severity
// =============================================================================

/// Error severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// Error - parsing failed
    Error,
    /// Warning - parsed with issues
    Warning,
    /// Hint - suggestion for improvement
    Hint,
}

impl Severity {
    /// Get ANSI color for terminal output
    pub fn color(&self) -> &'static str {
        match self {
            Self::Error => "\x1b[31m",   // Red
            Self::Warning => "\x1b[33m", // Yellow
            Self::Hint => "\x1b[36m",    // Cyan
        }
    }

    /// Get label text
    pub fn label(&self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Hint => "hint",
        }
    }
}

// =============================================================================
// Source Location
// =============================================================================

/// Source code location
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SourceLocation {
    /// Line number (1-indexed)
    pub line: usize,
    /// Column number (1-indexed)
    pub column: usize,
    /// Byte offset in source
    pub offset: usize,
    /// Length of the problematic span
    pub length: usize,
}

impl SourceLocation {
    /// Create a new source location
    pub fn new(line: usize, column: usize, offset: usize, length: usize) -> Self {
        Self { line, column, offset, length }
    }

    /// Create from byte offset in source
    pub fn from_offset(source: &str, offset: usize) -> Self {
        let mut line = 1;
        let mut column = 1;

        for (i, ch) in source.char_indices() {
            if i >= offset {
                break;
            }
            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }

        Self {
            line,
            column,
            offset,
            length: 1,
        }
    }

    /// Create from byte offset with length
    pub fn from_offset_len(source: &str, offset: usize, length: usize) -> Self {
        let mut loc = Self::from_offset(source, offset);
        loc.length = length;
        loc
    }
}

// =============================================================================
// Suggestion
// =============================================================================

/// A suggestion for fixing an error
#[derive(Debug, Clone)]
pub struct Suggestion {
    /// What to suggest
    pub message: String,
    /// Optional replacement text
    pub replacement: Option<String>,
}

impl Suggestion {
    /// Create a simple suggestion
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            replacement: None,
        }
    }

    /// Create a suggestion with replacement
    pub fn with_fix(message: impl Into<String>, replacement: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            replacement: Some(replacement.into()),
        }
    }
}

// =============================================================================
// Rich Parse Error
// =============================================================================

/// Rich CSS parsing error with context
#[derive(Debug, Clone)]
pub struct RichParseError {
    /// Error code
    pub code: ErrorCode,
    /// Severity level
    pub severity: Severity,
    /// Error message
    pub message: String,
    /// Source location
    pub location: SourceLocation,
    /// Suggestions for fixing
    pub suggestions: Vec<Suggestion>,
    /// Additional notes
    pub notes: Vec<String>,
}

impl RichParseError {
    /// Create a new error
    pub fn new(code: ErrorCode, message: impl Into<String>, location: SourceLocation) -> Self {
        Self {
            code,
            severity: Severity::Error,
            message: message.into(),
            location,
            suggestions: Vec::new(),
            notes: Vec::new(),
        }
    }

    /// Set severity
    pub fn severity(mut self, severity: Severity) -> Self {
        self.severity = severity;
        self
    }

    /// Add a suggestion
    pub fn suggest(mut self, suggestion: Suggestion) -> Self {
        self.suggestions.push(suggestion);
        self
    }

    /// Add a note
    pub fn note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    /// Pretty print the error with source context
    pub fn pretty_print(&self, source: &str) -> String {
        let mut output = String::new();
        let reset = "\x1b[0m";
        let bold = "\x1b[1m";
        let dim = "\x1b[2m";
        let blue = "\x1b[34m";
        let cyan = "\x1b[36m";

        // Error header
        output.push_str(&format!(
            "{}{}{}: {}[{}]{} {}\n",
            bold, self.severity.color(), self.severity.label(),
            reset, self.code, reset, self.message
        ));

        // Location
        output.push_str(&format!(
            "  {}-->{} line {}, column {}\n",
            blue, reset, self.location.line, self.location.column
        ));

        // Source snippet
        let lines: Vec<&str> = source.lines().collect();
        let line_idx = self.location.line.saturating_sub(1);

        if line_idx < lines.len() {
            let line_num_width = (self.location.line + 1).to_string().len().max(3);

            // Context line before
            if line_idx > 0 {
                output.push_str(&format!(
                    "  {}{:>width$} |{} {}\n",
                    dim, line_idx, reset, lines[line_idx - 1],
                    width = line_num_width
                ));
            }

            // Error line
            output.push_str(&format!(
                "  {}{:>width$} |{} {}\n",
                blue, self.location.line, reset, lines[line_idx],
                width = line_num_width
            ));

            // Pointer line
            let pointer_offset = self.location.column.saturating_sub(1);
            let pointer_len = self.location.length.max(1);
            output.push_str(&format!(
                "  {:>width$} {} {}{}{}{}",
                "", "|",
                " ".repeat(pointer_offset),
                self.severity.color(),
                "^".repeat(pointer_len),
                reset,
                width = line_num_width
            ));

            // Inline hint
            if !self.suggestions.is_empty() {
                output.push_str(&format!(" {}", self.suggestions[0].message));
            }
            output.push('\n');

            // Context line after
            if line_idx + 1 < lines.len() {
                output.push_str(&format!(
                    "  {}{:>width$} |{} {}\n",
                    dim, self.location.line + 1, reset, lines[line_idx + 1],
                    width = line_num_width
                ));
            }
        }

        // Suggestions
        for suggestion in &self.suggestions {
            if let Some(replacement) = &suggestion.replacement {
                output.push_str(&format!(
                    "\n  {}help:{} try `{}`",
                    cyan, reset, replacement
                ));
            }
        }

        // Notes
        for note in &self.notes {
            output.push_str(&format!("\n  {}note:{} {}", cyan, reset, note));
        }

        // Help
        output.push_str(&format!(
            "\n  {}help:{} {}\n",
            cyan, reset, self.code.help()
        ));

        output
    }

    /// Format without ANSI colors (for logs)
    pub fn plain_text(&self, source: &str) -> String {
        let mut output = String::new();

        // Error header
        output.push_str(&format!(
            "{}: [{}] {}\n",
            self.severity.label(), self.code, self.message
        ));

        // Location
        output.push_str(&format!(
            "  --> line {}, column {}\n",
            self.location.line, self.location.column
        ));

        // Source snippet
        let lines: Vec<&str> = source.lines().collect();
        let line_idx = self.location.line.saturating_sub(1);

        if line_idx < lines.len() {
            output.push_str(&format!(
                "  {} | {}\n",
                self.location.line, lines[line_idx]
            ));

            let pointer_offset = self.location.column.saturating_sub(1);
            let pointer_len = self.location.length.max(1);
            output.push_str(&format!(
                "    | {}{}\n",
                " ".repeat(pointer_offset),
                "^".repeat(pointer_len)
            ));
        }

        // Suggestions
        for suggestion in &self.suggestions {
            output.push_str(&format!("  help: {}\n", suggestion.message));
            if let Some(replacement) = &suggestion.replacement {
                output.push_str(&format!("    try: {}\n", replacement));
            }
        }

        // Notes
        for note in &self.notes {
            output.push_str(&format!("  note: {}\n", note));
        }

        output
    }
}

impl std::error::Error for RichParseError {}

impl fmt::Display for RichParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {} (line {}, column {})",
            self.code, self.message, self.location.line, self.location.column
        )
    }
}

// =============================================================================
// Error Collection
// =============================================================================

/// Collection of parse errors for error recovery
#[derive(Debug, Default)]
pub struct ParseErrors {
    /// Collected errors
    errors: Vec<RichParseError>,
    /// Maximum errors before giving up
    max_errors: usize,
}

impl ParseErrors {
    /// Create a new error collection
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            max_errors: 10,
        }
    }

    /// Set maximum errors
    pub fn max_errors(mut self, max: usize) -> Self {
        self.max_errors = max;
        self
    }

    /// Add an error
    pub fn push(&mut self, error: RichParseError) {
        self.errors.push(error);
    }

    /// Check if we've hit max errors
    pub fn is_full(&self) -> bool {
        self.errors.len() >= self.max_errors
    }

    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        self.errors.iter().any(|e| e.severity == Severity::Error)
    }

    /// Get all errors
    pub fn errors(&self) -> &[RichParseError] {
        &self.errors
    }

    /// Get error count
    pub fn len(&self) -> usize {
        self.errors.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Pretty print all errors
    pub fn pretty_print(&self, source: &str) -> String {
        let mut output = String::new();

        for error in &self.errors {
            output.push_str(&error.pretty_print(source));
            output.push('\n');
        }

        // Summary
        let error_count = self.errors.iter()
            .filter(|e| e.severity == Severity::Error)
            .count();
        let warning_count = self.errors.iter()
            .filter(|e| e.severity == Severity::Warning)
            .count();

        if error_count > 0 || warning_count > 0 {
            output.push_str(&format!(
                "\x1b[1m{} error(s), {} warning(s)\x1b[0m\n",
                error_count, warning_count
            ));
        }

        output
    }
}

// =============================================================================
// Property Suggestions (Did you mean?)
// =============================================================================

/// Common CSS properties for suggestions
pub const KNOWN_PROPERTIES: &[&str] = &[
    "color", "background", "background-color", "border", "border-color",
    "border-width", "border-style", "border-radius",
    "padding", "padding-top", "padding-right", "padding-bottom", "padding-left",
    "margin", "margin-top", "margin-right", "margin-bottom", "margin-left",
    "width", "height", "min-width", "min-height", "max-width", "max-height",
    "display", "flex-direction", "justify-content", "align-items", "align-self",
    "flex-grow", "flex-shrink", "flex-basis", "flex-wrap", "gap",
    "position", "top", "right", "bottom", "left",
    "font-weight", "font-style", "text-align", "text-decoration",
    "opacity", "visibility", "overflow", "cursor",
    "transition", "animation",
    "grid-template-columns", "grid-template-rows", "grid-column", "grid-row",
];

/// Find similar property names (Levenshtein distance)
pub fn suggest_property(unknown: &str) -> Vec<&'static str> {
    let mut suggestions: Vec<(&str, usize)> = KNOWN_PROPERTIES
        .iter()
        .filter_map(|prop| {
            let dist = levenshtein_distance(unknown, prop);
            // Only suggest if distance is reasonable
            if dist <= 3 && dist < unknown.len() {
                Some((*prop, dist))
            } else {
                None
            }
        })
        .collect();

    suggestions.sort_by_key(|(_, d)| *d);
    suggestions.into_iter().take(3).map(|(p, _)| p).collect()
}

/// Calculate Levenshtein distance between two strings
fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let a_len = a_chars.len();
    let b_len = b_chars.len();

    if a_len == 0 { return b_len; }
    if b_len == 0 { return a_len; }

    let mut prev: Vec<usize> = (0..=b_len).collect();
    let mut curr = vec![0; b_len + 1];

    for i in 1..=a_len {
        curr[0] = i;
        for j in 1..=b_len {
            let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };
            curr[j] = (prev[j] + 1)
                .min(curr[j - 1] + 1)
                .min(prev[j - 1] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }

    prev[b_len]
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_code_display() {
        assert_eq!(ErrorCode::InvalidSyntax.code(), "E001");
        assert_eq!(ErrorCode::UnknownProperty.code(), "E002");
    }

    #[test]
    fn test_source_location_from_offset() {
        let source = "line1\nline2\nline3";
        let loc = SourceLocation::from_offset(source, 6);
        assert_eq!(loc.line, 2);
        assert_eq!(loc.column, 1);
    }

    #[test]
    fn test_suggest_property() {
        let suggestions = suggest_property("colr");
        assert!(suggestions.contains(&"color"));

        let suggestions = suggest_property("backgrond");
        assert!(suggestions.contains(&"background"));
    }

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("color", "color"), 0);
        assert_eq!(levenshtein_distance("color", "colr"), 1);
        assert_eq!(levenshtein_distance("color", "colour"), 1);
        assert_eq!(levenshtein_distance("", "abc"), 3);
    }

    #[test]
    fn test_rich_error_pretty_print() {
        let source = ".button {\n  colr: red;\n}";
        let loc = SourceLocation::new(2, 3, 12, 4);
        let error = RichParseError::new(
            ErrorCode::UnknownProperty,
            "unknown property 'colr'",
            loc,
        )
        .suggest(Suggestion::with_fix("did you mean 'color'?", "color"));

        let output = error.pretty_print(source);
        assert!(output.contains("E002"));
        assert!(output.contains("colr"));
        assert!(output.contains("color"));
    }

    #[test]
    fn test_parse_errors_collection() {
        let mut errors = ParseErrors::new();
        assert!(errors.is_empty());

        errors.push(RichParseError::new(
            ErrorCode::InvalidSyntax,
            "test error",
            SourceLocation::default(),
        ));

        assert!(!errors.is_empty());
        assert_eq!(errors.len(), 1);
        assert!(errors.has_errors());
    }
}
