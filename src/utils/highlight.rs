//! Text highlighting utilities
//!
//! Provides utilities for highlighting matched text in search results,
//! fuzzy matches, and other highlighting scenarios.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::utils::{highlight_matches, HighlightSpan};
//!
//! // Highlight fuzzy match
//! let indices = vec![0, 3, 7];  // Matched character positions
//! let spans = highlight_matches("CommandPalette", &indices);
//!
//! // Highlight search term
//! let spans = highlight_substring("Hello World", "World");
//! ```

use crate::style::Color;

/// A span of text with optional highlighting
#[derive(Clone, Debug, PartialEq)]
pub struct HighlightSpan {
    /// The text content
    pub text: String,
    /// Whether this span is highlighted
    pub highlighted: bool,
    /// Start index in original string
    pub start: usize,
    /// End index in original string (exclusive)
    pub end: usize,
}

impl HighlightSpan {
    /// Create a new highlight span
    pub fn new(text: impl Into<String>, highlighted: bool, start: usize, end: usize) -> Self {
        Self {
            text: text.into(),
            highlighted,
            start,
            end,
        }
    }

    /// Create a normal (non-highlighted) span
    pub fn normal(text: impl Into<String>, start: usize, end: usize) -> Self {
        Self::new(text, false, start, end)
    }

    /// Create a highlighted span
    pub fn highlighted(text: impl Into<String>, start: usize, end: usize) -> Self {
        Self::new(text, true, start, end)
    }
}

/// Highlight specific character indices in a string
///
/// Creates spans where characters at the given indices are highlighted.
/// Useful for fuzzy match highlighting.
///
/// # Example
///
/// ```rust,ignore
/// use revue::utils::highlight_matches;
///
/// let spans = highlight_matches("CommandPalette", &[0, 7]);
/// // Returns: [("C", true), ("ommand", false), ("P", true), ("alette", false)]
/// ```
pub fn highlight_matches(text: &str, indices: &[usize]) -> Vec<HighlightSpan> {
    if indices.is_empty() {
        return vec![HighlightSpan::normal(text.to_string(), 0, text.len())];
    }

    let chars: Vec<char> = text.chars().collect();
    let mut spans = Vec::new();
    let mut current_start = 0;
    let mut current_text = String::new();
    let mut in_highlight = false;

    for (i, &ch) in chars.iter().enumerate() {
        let should_highlight = indices.contains(&i);

        if should_highlight != in_highlight {
            // State change
            if !current_text.is_empty() {
                let byte_start = text
                    .char_indices()
                    .nth(current_start)
                    .map(|(i, _)| i)
                    .unwrap_or(0);
                let byte_end = text
                    .char_indices()
                    .nth(current_start + current_text.chars().count())
                    .map(|(i, _)| i)
                    .unwrap_or(text.len());

                spans.push(HighlightSpan::new(
                    current_text.clone(),
                    in_highlight,
                    byte_start,
                    byte_end,
                ));
            }
            current_text.clear();
            current_start = i;
            in_highlight = should_highlight;
        }

        current_text.push(ch);
    }

    // Final span
    if !current_text.is_empty() {
        let byte_start = text
            .char_indices()
            .nth(current_start)
            .map(|(i, _)| i)
            .unwrap_or(0);

        spans.push(HighlightSpan::new(
            current_text,
            in_highlight,
            byte_start,
            text.len(),
        ));
    }

    spans
}

/// Highlight all occurrences of a substring
///
/// Case-insensitive by default.
///
/// # Example
///
/// ```rust,ignore
/// use revue::utils::highlight_substring;
///
/// let spans = highlight_substring("Hello World, Hello!", "hello");
/// // Highlights both "Hello" occurrences
/// ```
pub fn highlight_substring(text: &str, pattern: &str) -> Vec<HighlightSpan> {
    highlight_substring_case(text, pattern, false)
}

/// Highlight all occurrences of a substring with case sensitivity option
pub fn highlight_substring_case(
    text: &str,
    pattern: &str,
    case_sensitive: bool,
) -> Vec<HighlightSpan> {
    if pattern.is_empty() {
        return vec![HighlightSpan::normal(text.to_string(), 0, text.len())];
    }

    let search_text = if case_sensitive {
        text.to_string()
    } else {
        text.to_lowercase()
    };
    let search_pattern = if case_sensitive {
        pattern.to_string()
    } else {
        pattern.to_lowercase()
    };

    let mut spans = Vec::new();
    let mut last_end = 0;

    for (start, _) in search_text.match_indices(&search_pattern) {
        let end = start + pattern.len();

        // Add non-highlighted span before match
        if start > last_end {
            spans.push(HighlightSpan::normal(
                text[last_end..start].to_string(),
                last_end,
                start,
            ));
        }

        // Add highlighted span
        spans.push(HighlightSpan::highlighted(
            text[start..end].to_string(),
            start,
            end,
        ));

        last_end = end;
    }

    // Add remaining non-highlighted text
    if last_end < text.len() {
        spans.push(HighlightSpan::normal(
            text[last_end..].to_string(),
            last_end,
            text.len(),
        ));
    }

    if spans.is_empty() {
        spans.push(HighlightSpan::normal(text.to_string(), 0, text.len()));
    }

    spans
}

/// Highlight a range in a string
pub fn highlight_range(text: &str, start: usize, end: usize) -> Vec<HighlightSpan> {
    let end = end.min(text.len());
    let start = start.min(end);

    let mut spans = Vec::new();

    if start > 0 {
        spans.push(HighlightSpan::normal(text[..start].to_string(), 0, start));
    }

    if start < end {
        spans.push(HighlightSpan::highlighted(
            text[start..end].to_string(),
            start,
            end,
        ));
    }

    if end < text.len() {
        spans.push(HighlightSpan::normal(
            text[end..].to_string(),
            end,
            text.len(),
        ));
    }

    if spans.is_empty() {
        spans.push(HighlightSpan::normal(text.to_string(), 0, text.len()));
    }

    spans
}

/// Highlight multiple ranges in a string
///
/// Ranges are merged if they overlap.
pub fn highlight_ranges(text: &str, ranges: &[(usize, usize)]) -> Vec<HighlightSpan> {
    if ranges.is_empty() {
        return vec![HighlightSpan::normal(text.to_string(), 0, text.len())];
    }

    // Sort and merge overlapping ranges
    let mut sorted: Vec<(usize, usize)> = ranges.to_vec();
    sorted.sort_by_key(|r| r.0);

    let mut merged = Vec::new();
    let mut current = sorted[0];

    for &(start, end) in &sorted[1..] {
        if start <= current.1 {
            // Overlapping, merge
            current.1 = current.1.max(end);
        } else {
            merged.push(current);
            current = (start, end);
        }
    }
    merged.push(current);

    // Create spans
    let mut spans = Vec::new();
    let mut last_end = 0;

    for (start, end) in merged {
        let start = start.min(text.len());
        let end = end.min(text.len());

        if start > last_end {
            spans.push(HighlightSpan::normal(
                text[last_end..start].to_string(),
                last_end,
                start,
            ));
        }

        if start < end {
            spans.push(HighlightSpan::highlighted(
                text[start..end].to_string(),
                start,
                end,
            ));
        }

        last_end = end;
    }

    if last_end < text.len() {
        spans.push(HighlightSpan::normal(
            text[last_end..].to_string(),
            last_end,
            text.len(),
        ));
    }

    spans
}

/// Builder for applying different highlight styles
#[derive(Clone, Debug)]
pub struct Highlighter {
    /// Highlight foreground color
    pub highlight_fg: Option<Color>,
    /// Highlight background color
    pub highlight_bg: Option<Color>,
    /// Normal foreground color
    pub normal_fg: Option<Color>,
    /// Normal background color
    pub normal_bg: Option<Color>,
}

impl Default for Highlighter {
    fn default() -> Self {
        Self {
            highlight_fg: Some(Color::BLACK),
            highlight_bg: Some(Color::YELLOW),
            normal_fg: None,
            normal_bg: None,
        }
    }
}

impl Highlighter {
    /// Create a new highlighter with default colors
    pub fn new() -> Self {
        Self::default()
    }

    /// Set highlight foreground color
    pub fn highlight_fg(mut self, color: Color) -> Self {
        self.highlight_fg = Some(color);
        self
    }

    /// Set highlight background color
    pub fn highlight_bg(mut self, color: Color) -> Self {
        self.highlight_bg = Some(color);
        self
    }

    /// Set normal foreground color
    pub fn normal_fg(mut self, color: Color) -> Self {
        self.normal_fg = Some(color);
        self
    }

    /// Set normal background color
    pub fn normal_bg(mut self, color: Color) -> Self {
        self.normal_bg = Some(color);
        self
    }

    /// Create a highlighter with custom highlight color
    pub fn with_color(fg: Color, bg: Color) -> Self {
        Self {
            highlight_fg: Some(fg),
            highlight_bg: Some(bg),
            normal_fg: None,
            normal_bg: None,
        }
    }

    /// Get foreground color for a span
    pub fn fg_for(&self, span: &HighlightSpan) -> Option<Color> {
        if span.highlighted {
            self.highlight_fg
        } else {
            self.normal_fg
        }
    }

    /// Get background color for a span
    pub fn bg_for(&self, span: &HighlightSpan) -> Option<Color> {
        if span.highlighted {
            self.highlight_bg
        } else {
            self.normal_bg
        }
    }
}

/// Highlight style presets
impl Highlighter {
    /// Yellow background highlight (default)
    pub fn yellow() -> Self {
        Self::with_color(Color::BLACK, Color::YELLOW)
    }

    /// Cyan/blue highlight
    pub fn cyan() -> Self {
        Self::with_color(Color::BLACK, Color::CYAN)
    }

    /// Green highlight
    pub fn green() -> Self {
        Self::with_color(Color::BLACK, Color::GREEN)
    }

    /// Red highlight
    pub fn red() -> Self {
        Self::with_color(Color::WHITE, Color::RED)
    }

    /// Magenta highlight
    pub fn magenta() -> Self {
        Self::with_color(Color::WHITE, Color::MAGENTA)
    }

    /// Underline style (foreground only)
    pub fn underline() -> Self {
        Self {
            highlight_fg: Some(Color::CYAN),
            highlight_bg: None,
            normal_fg: None,
            normal_bg: None,
        }
    }
}
