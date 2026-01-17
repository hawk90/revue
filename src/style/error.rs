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
            Self::InvalidSyntax => {
                "Check for mismatched brackets, quotes, or unexpected characters"
            }
            Self::UnknownProperty => "Check spelling or see the supported properties list",
            Self::InvalidValue => "The value format doesn't match what this property expects",
            Self::MissingBrace => "Every '{' must have a matching '}'",
            Self::MissingSemicolon => "Each CSS declaration should end with ';'",
            Self::InvalidSelector => "Selectors should be like '.class', '#id', or 'element'",
            Self::UndefinedVariable => "Define variables in :root { --name: value; }",
            Self::InvalidColor => "Use formats like #rgb, #rrggbb, rgb(r,g,b), or named colors",
            Self::InvalidNumber => "Numbers should be like '10', '10px', '50%', or '0.5'",
            Self::EmptyRule => "Add at least one property declaration inside the rule",
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
        Self {
            line,
            column,
            offset,
            length,
        }
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
            bold,
            self.severity.color(),
            self.severity.label(),
            reset,
            self.code,
            reset,
            self.message
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
                    dim,
                    line_idx,
                    reset,
                    lines[line_idx - 1],
                    width = line_num_width
                ));
            }

            // Error line
            output.push_str(&format!(
                "  {}{:>width$} |{} {}\n",
                blue,
                self.location.line,
                reset,
                lines[line_idx],
                width = line_num_width
            ));

            // Pointer line
            let pointer_offset = self.location.column.saturating_sub(1);
            let pointer_len = self.location.length.max(1);
            output.push_str(&format!(
                "  {:>width$} {} {}{}{}{}",
                "",
                "|",
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
                    dim,
                    self.location.line + 1,
                    reset,
                    lines[line_idx + 1],
                    width = line_num_width
                ));
            }
        }

        // Suggestions
        for suggestion in &self.suggestions {
            if let Some(replacement) = &suggestion.replacement {
                output.push_str(&format!("\n  {}help:{} try `{}`", cyan, reset, replacement));
            }
        }

        // Notes
        for note in &self.notes {
            output.push_str(&format!("\n  {}note:{} {}", cyan, reset, note));
        }

        // Help
        output.push_str(&format!(
            "\n  {}help:{} {}\n",
            cyan,
            reset,
            self.code.help()
        ));

        output
    }

    /// Format without ANSI colors (for logs)
    pub fn plain_text(&self, source: &str) -> String {
        let mut output = String::new();

        // Error header
        output.push_str(&format!(
            "{}: [{}] {}\n",
            self.severity.label(),
            self.code,
            self.message
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
            output.push_str(&format!("  {} | {}\n", self.location.line, lines[line_idx]));

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
        let error_count = self
            .errors
            .iter()
            .filter(|e| e.severity == Severity::Error)
            .count();
        let warning_count = self
            .errors
            .iter()
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
    "color",
    "background",
    "background-color",
    "border",
    "border-color",
    "border-width",
    "border-style",
    "border-radius",
    "padding",
    "padding-top",
    "padding-right",
    "padding-bottom",
    "padding-left",
    "margin",
    "margin-top",
    "margin-right",
    "margin-bottom",
    "margin-left",
    "width",
    "height",
    "min-width",
    "min-height",
    "max-width",
    "max-height",
    "display",
    "flex-direction",
    "justify-content",
    "align-items",
    "align-self",
    "flex-grow",
    "flex-shrink",
    "flex-basis",
    "flex-wrap",
    "gap",
    "position",
    "top",
    "right",
    "bottom",
    "left",
    "font-weight",
    "font-style",
    "text-align",
    "text-decoration",
    "opacity",
    "visibility",
    "overflow",
    "cursor",
    "transition",
    "animation",
    "grid-template-columns",
    "grid-template-rows",
    "grid-column",
    "grid-row",
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

    if a_len == 0 {
        return b_len;
    }
    if b_len == 0 {
        return a_len;
    }

    let mut prev: Vec<usize> = (0..=b_len).collect();
    let mut curr = vec![0; b_len + 1];

    for i in 1..=a_len {
        curr[0] = i;
        for j in 1..=b_len {
            let cost = if a_chars[i - 1] == b_chars[j - 1] {
                0
            } else {
                1
            };
            curr[j] = (prev[j] + 1).min(curr[j - 1] + 1).min(prev[j - 1] + cost);
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
        let error = RichParseError::new(ErrorCode::UnknownProperty, "unknown property 'colr'", loc)
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

    // =========================================================================
    // ErrorCode tests
    // =========================================================================

    #[test]
    fn test_error_code_all_codes() {
        assert_eq!(ErrorCode::InvalidSyntax.code(), "E001");
        assert_eq!(ErrorCode::UnknownProperty.code(), "E002");
        assert_eq!(ErrorCode::InvalidValue.code(), "E003");
        assert_eq!(ErrorCode::MissingBrace.code(), "E004");
        assert_eq!(ErrorCode::MissingSemicolon.code(), "E005");
        assert_eq!(ErrorCode::InvalidSelector.code(), "E006");
        assert_eq!(ErrorCode::UndefinedVariable.code(), "E007");
        assert_eq!(ErrorCode::InvalidColor.code(), "E008");
        assert_eq!(ErrorCode::InvalidNumber.code(), "E009");
        assert_eq!(ErrorCode::EmptyRule.code(), "E010");
    }

    #[test]
    fn test_error_code_descriptions() {
        assert!(!ErrorCode::InvalidSyntax.description().is_empty());
        assert!(!ErrorCode::UnknownProperty.description().is_empty());
        assert!(!ErrorCode::InvalidValue.description().is_empty());
    }

    #[test]
    fn test_error_code_help() {
        assert!(!ErrorCode::InvalidSyntax.help().is_empty());
        assert!(!ErrorCode::UnknownProperty.help().is_empty());
        assert!(!ErrorCode::InvalidColor.help().is_empty());
    }

    #[test]
    fn test_error_code_display_format() {
        let code = ErrorCode::InvalidSyntax;
        assert_eq!(format!("{}", code), "E001");
    }

    #[test]
    fn test_error_code_equality() {
        assert_eq!(ErrorCode::InvalidSyntax, ErrorCode::InvalidSyntax);
        assert_ne!(ErrorCode::InvalidSyntax, ErrorCode::UnknownProperty);
    }

    #[test]
    fn test_error_code_copy() {
        let code = ErrorCode::InvalidValue;
        let copied = code;
        assert_eq!(code, copied);
    }

    // =========================================================================
    // Severity tests
    // =========================================================================

    #[test]
    fn test_severity_labels() {
        assert_eq!(Severity::Error.label(), "error");
        assert_eq!(Severity::Warning.label(), "warning");
        assert_eq!(Severity::Hint.label(), "hint");
    }

    #[test]
    fn test_severity_colors() {
        // Just verify they return ANSI codes
        assert!(Severity::Error.color().contains("\x1b["));
        assert!(Severity::Warning.color().contains("\x1b["));
        assert!(Severity::Hint.color().contains("\x1b["));
    }

    #[test]
    fn test_severity_equality() {
        assert_eq!(Severity::Error, Severity::Error);
        assert_ne!(Severity::Error, Severity::Warning);
    }

    // =========================================================================
    // SourceLocation tests
    // =========================================================================

    #[test]
    fn test_source_location_default() {
        let loc = SourceLocation::default();
        assert_eq!(loc.line, 0);
        assert_eq!(loc.column, 0);
        assert_eq!(loc.offset, 0);
        assert_eq!(loc.length, 0);
    }

    #[test]
    fn test_source_location_new() {
        let loc = SourceLocation::new(5, 10, 50, 3);
        assert_eq!(loc.line, 5);
        assert_eq!(loc.column, 10);
        assert_eq!(loc.offset, 50);
        assert_eq!(loc.length, 3);
    }

    #[test]
    fn test_source_location_from_offset_first_line() {
        let source = "hello world";
        let loc = SourceLocation::from_offset(source, 6);
        assert_eq!(loc.line, 1);
        assert_eq!(loc.column, 7);
    }

    #[test]
    fn test_source_location_from_offset_multiline() {
        let source = "line1\nline2\nline3";
        let loc = SourceLocation::from_offset(source, 12); // 'l' in "line3"
        assert_eq!(loc.line, 3);
        assert_eq!(loc.column, 1);
    }

    #[test]
    fn test_source_location_from_offset_len() {
        let source = "hello world";
        let loc = SourceLocation::from_offset_len(source, 0, 5);
        assert_eq!(loc.length, 5);
    }

    #[test]
    fn test_source_location_from_offset_empty() {
        let source = "";
        let loc = SourceLocation::from_offset(source, 0);
        assert_eq!(loc.line, 1);
        assert_eq!(loc.column, 1);
    }

    // =========================================================================
    // Suggestion tests
    // =========================================================================

    #[test]
    fn test_suggestion_new() {
        let suggestion = Suggestion::new("try something else");
        assert_eq!(suggestion.message, "try something else");
        assert!(suggestion.replacement.is_none());
    }

    #[test]
    fn test_suggestion_with_fix() {
        let suggestion = Suggestion::with_fix("did you mean", "color");
        assert_eq!(suggestion.message, "did you mean");
        assert_eq!(suggestion.replacement, Some("color".to_string()));
    }

    #[test]
    fn test_suggestion_clone() {
        let suggestion = Suggestion::with_fix("hint", "fix");
        let cloned = suggestion.clone();
        assert_eq!(cloned.message, "hint");
        assert_eq!(cloned.replacement, Some("fix".to_string()));
    }

    // =========================================================================
    // RichParseError tests
    // =========================================================================

    #[test]
    fn test_rich_parse_error_new() {
        let loc = SourceLocation::new(1, 5, 4, 3);
        let error = RichParseError::new(ErrorCode::InvalidValue, "invalid value", loc);

        assert_eq!(error.code, ErrorCode::InvalidValue);
        assert_eq!(error.severity, Severity::Error);
        assert_eq!(error.message, "invalid value");
        assert!(error.suggestions.is_empty());
        assert!(error.notes.is_empty());
    }

    #[test]
    fn test_rich_parse_error_severity() {
        let error = RichParseError::new(
            ErrorCode::EmptyRule,
            "empty rule",
            SourceLocation::default(),
        )
        .severity(Severity::Warning);

        assert_eq!(error.severity, Severity::Warning);
    }

    #[test]
    fn test_rich_parse_error_suggest() {
        let error = RichParseError::new(
            ErrorCode::UnknownProperty,
            "unknown property",
            SourceLocation::default(),
        )
        .suggest(Suggestion::new("check spelling"));

        assert_eq!(error.suggestions.len(), 1);
    }

    #[test]
    fn test_rich_parse_error_note() {
        let error = RichParseError::new(
            ErrorCode::InvalidSyntax,
            "syntax error",
            SourceLocation::default(),
        )
        .note("see documentation");

        assert_eq!(error.notes.len(), 1);
        assert_eq!(error.notes[0], "see documentation");
    }

    #[test]
    fn test_rich_parse_error_chained() {
        let error = RichParseError::new(
            ErrorCode::UnknownProperty,
            "unknown 'colr'",
            SourceLocation::new(2, 3, 10, 4),
        )
        .severity(Severity::Error)
        .suggest(Suggestion::with_fix("did you mean", "color"))
        .note("color is a valid CSS property");

        assert_eq!(error.suggestions.len(), 1);
        assert_eq!(error.notes.len(), 1);
    }

    #[test]
    fn test_rich_parse_error_display() {
        let error = RichParseError::new(
            ErrorCode::InvalidColor,
            "invalid color format",
            SourceLocation::new(3, 10, 25, 5),
        );

        let display = format!("{}", error);
        assert!(display.contains("E008"));
        assert!(display.contains("invalid color format"));
        assert!(display.contains("line 3"));
    }

    #[test]
    fn test_rich_parse_error_plain_text() {
        let source = ".button { color: invalid; }";
        let error = RichParseError::new(
            ErrorCode::InvalidValue,
            "invalid color value",
            SourceLocation::new(1, 18, 17, 7),
        );

        let plain = error.plain_text(source);
        assert!(plain.contains("error"));
        assert!(plain.contains("E003"));
        assert!(plain.contains("invalid"));
    }

    #[test]
    fn test_rich_parse_error_pretty_print_contains_code() {
        let source = ".x { y: z; }";
        let error = RichParseError::new(
            ErrorCode::UnknownProperty,
            "unknown property 'y'",
            SourceLocation::new(1, 6, 5, 1),
        );

        let pretty = error.pretty_print(source);
        assert!(pretty.contains("E002"));
    }

    // =========================================================================
    // ParseErrors collection tests
    // =========================================================================

    #[test]
    fn test_parse_errors_new() {
        let errors = ParseErrors::new();
        assert!(errors.is_empty());
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn test_parse_errors_default() {
        let errors = ParseErrors::default();
        assert!(errors.is_empty());
    }

    #[test]
    fn test_parse_errors_max_errors() {
        let errors = ParseErrors::new().max_errors(5);
        // Just verify it doesn't panic
        assert!(errors.is_empty());
    }

    #[test]
    fn test_parse_errors_is_full() {
        let mut errors = ParseErrors::new().max_errors(2);

        errors.push(RichParseError::new(
            ErrorCode::InvalidSyntax,
            "error 1",
            SourceLocation::default(),
        ));
        assert!(!errors.is_full());

        errors.push(RichParseError::new(
            ErrorCode::InvalidSyntax,
            "error 2",
            SourceLocation::default(),
        ));
        assert!(errors.is_full());
    }

    #[test]
    fn test_parse_errors_has_errors_with_warning() {
        let mut errors = ParseErrors::new();

        errors.push(
            RichParseError::new(ErrorCode::EmptyRule, "warning", SourceLocation::default())
                .severity(Severity::Warning),
        );

        assert!(!errors.has_errors()); // Only warnings, no errors
    }

    #[test]
    fn test_parse_errors_get_errors() {
        let mut errors = ParseErrors::new();
        errors.push(RichParseError::new(
            ErrorCode::InvalidSyntax,
            "test",
            SourceLocation::default(),
        ));

        let slice = errors.errors();
        assert_eq!(slice.len(), 1);
    }

    #[test]
    fn test_parse_errors_pretty_print() {
        let mut errors = ParseErrors::new();
        errors.push(RichParseError::new(
            ErrorCode::InvalidSyntax,
            "syntax error",
            SourceLocation::new(1, 1, 0, 1),
        ));

        let source = "invalid { }";
        let output = errors.pretty_print(source);
        assert!(output.contains("1 error(s)"));
    }

    // =========================================================================
    // Property suggestion tests
    // =========================================================================

    #[test]
    fn test_suggest_property_color() {
        let suggestions = suggest_property("colr");
        assert!(suggestions.contains(&"color"));
    }

    #[test]
    fn test_suggest_property_background() {
        let suggestions = suggest_property("backgrond");
        assert!(suggestions.contains(&"background"));
    }

    #[test]
    fn test_suggest_property_padding() {
        let suggestions = suggest_property("pading");
        assert!(suggestions.contains(&"padding"));
    }

    #[test]
    fn test_suggest_property_no_match() {
        let suggestions = suggest_property("xyzabc");
        // Should return empty or very few suggestions for nonsense
        assert!(suggestions.len() <= 3);
    }

    #[test]
    fn test_suggest_property_exact_match() {
        let suggestions = suggest_property("color");
        // Exact match has distance 0
        assert!(suggestions.contains(&"color"));
    }

    #[test]
    fn test_suggest_property_max_results() {
        let suggestions = suggest_property("border");
        // Should return at most 3 suggestions
        assert!(suggestions.len() <= 3);
    }

    // =========================================================================
    // Levenshtein distance tests
    // =========================================================================

    #[test]
    fn test_levenshtein_identical() {
        assert_eq!(levenshtein_distance("test", "test"), 0);
    }

    #[test]
    fn test_levenshtein_one_char_diff() {
        assert_eq!(levenshtein_distance("test", "tset"), 2); // swap = 2
        assert_eq!(levenshtein_distance("test", "tests"), 1); // insert
        assert_eq!(levenshtein_distance("test", "tes"), 1); // delete
        assert_eq!(levenshtein_distance("test", "fest"), 1); // substitute
    }

    #[test]
    fn test_levenshtein_empty_strings() {
        assert_eq!(levenshtein_distance("", ""), 0);
        assert_eq!(levenshtein_distance("abc", ""), 3);
        assert_eq!(levenshtein_distance("", "xyz"), 3);
    }

    #[test]
    fn test_levenshtein_completely_different() {
        assert_eq!(levenshtein_distance("abc", "xyz"), 3);
    }

    // =========================================================================
    // Known properties tests
    // =========================================================================

    #[test]
    fn test_known_properties_contains_common() {
        assert!(KNOWN_PROPERTIES.contains(&"color"));
        assert!(KNOWN_PROPERTIES.contains(&"background"));
        assert!(KNOWN_PROPERTIES.contains(&"padding"));
        assert!(KNOWN_PROPERTIES.contains(&"margin"));
        assert!(KNOWN_PROPERTIES.contains(&"border"));
        assert!(KNOWN_PROPERTIES.contains(&"width"));
        assert!(KNOWN_PROPERTIES.contains(&"height"));
    }

    #[test]
    fn test_known_properties_contains_flex() {
        assert!(KNOWN_PROPERTIES.contains(&"display"));
        assert!(KNOWN_PROPERTIES.contains(&"flex-direction"));
        assert!(KNOWN_PROPERTIES.contains(&"justify-content"));
        assert!(KNOWN_PROPERTIES.contains(&"align-items"));
    }

    #[test]
    fn test_known_properties_contains_grid() {
        assert!(KNOWN_PROPERTIES.contains(&"grid-template-columns"));
        assert!(KNOWN_PROPERTIES.contains(&"grid-template-rows"));
        assert!(KNOWN_PROPERTIES.contains(&"grid-column"));
        assert!(KNOWN_PROPERTIES.contains(&"grid-row"));
    }
}
