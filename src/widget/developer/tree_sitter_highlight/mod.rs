//! Tree-sitter based syntax highlighting
//!
//! Provides accurate syntax highlighting using tree-sitter parsers.
//! This module is only available when the `syntax-highlighting` feature is enabled.

mod captures;
mod languages;
mod lines;

use tree_sitter_highlight::{HighlightEvent, Highlighter};

use crate::widget::syntax::{HighlightSpan, Language, SyntaxTheme};
use captures::CaptureStyle;
use languages::config;

/// Tree-sitter based syntax highlighter
pub struct TreeSitterHighlighter {
    highlighter: Highlighter,
    language: Language,
    theme: SyntaxTheme,
}

/// How deep brackets may nest and still go to tree-sitter. Its time grows with
/// the square of the depth on unbalanced brackets: 10 000 deep takes about a
/// second, on every call. Nothing written by hand nests this deep.
const MAX_BRACKET_DEPTH: usize = 1000;

/// Whether `source`'s brackets nest deeper than [`MAX_BRACKET_DEPTH`]. Counted
/// in one pass over any kind of bracket, ignoring what is in strings: it only
/// has to find input that is pathological.
fn nests_too_deep(source: &str) -> bool {
    let mut depth = 0usize;
    for byte in source.bytes() {
        match byte {
            b'(' | b'[' | b'{' => {
                depth += 1;
                if depth > MAX_BRACKET_DEPTH {
                    return true;
                }
            }
            b')' | b']' | b'}' => depth = depth.saturating_sub(1),
            _ => {}
        }
    }
    false
}

impl TreeSitterHighlighter {
    /// Create a new tree-sitter highlighter for the given language
    pub fn new(language: Language) -> Self {
        Self {
            highlighter: Highlighter::new(),
            language,
            theme: SyntaxTheme::default(),
        }
    }

    /// Create a new highlighter with a custom theme
    pub fn with_theme(language: Language, theme: SyntaxTheme) -> Self {
        Self {
            highlighter: Highlighter::new(),
            language,
            theme,
        }
    }

    /// Set the language
    pub fn language(mut self, language: Language) -> Self {
        self.language = language;
        self
    }

    /// Set the theme
    pub fn theme(mut self, theme: SyntaxTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Check if tree-sitter highlighting is available for a language
    pub fn is_supported(language: Language) -> bool {
        config(language).is_some()
    }

    /// The captured byte ranges of `source`, each with its style, in the
    /// order tree-sitter reports them. `None` when there is nothing to
    /// highlight with: no language, an unsupported one, a failed parse, or
    /// brackets nested too deep to parse in reasonable time.
    fn captures(&mut self, source: &str) -> Option<Vec<(usize, usize, CaptureStyle)>> {
        if nests_too_deep(source) {
            return None;
        }
        let config = config(self.language)?;

        let highlights =
            match self
                .highlighter
                .highlight(config, source.as_bytes(), None, None, |_| None)
            {
                Ok(h) => h,
                Err(_) => return None,
            };

        let mut captures = Vec::new();
        // Highlights nest (an escape inside a string): the innermost one
        // colors the text, and when it ends the one around it goes on
        let mut open: Vec<usize> = Vec::new();

        for event in highlights {
            let Ok(event) = event else {
                continue;
            };

            match event {
                HighlightEvent::Source { start, end } => {
                    if let Some(&highlight_idx) = open.last() {
                        captures.push((start, end, CaptureStyle::of(highlight_idx, &self.theme)));
                    }
                }
                HighlightEvent::HighlightStart(highlight) => open.push(highlight.0),
                HighlightEvent::HighlightEnd => {
                    open.pop();
                }
            }
        }

        Some(captures)
    }

    /// Highlight a line of code
    ///
    /// Returns highlight spans for the given line using tree-sitter parsing.
    pub fn highlight_line(&mut self, line: &str) -> Vec<HighlightSpan> {
        let Some(captures) = self.captures(line) else {
            return Vec::new();
        };

        // Spans are in characters; tree-sitter counts bytes
        let cols = lines::CharColumns::new(line);
        let mut spans: Vec<HighlightSpan> = captures
            .into_iter()
            .map(|(start, end, style)| style.span(cols.at(start), cols.at(end)))
            .collect();

        // Sort spans by start position
        spans.sort_by_key(|s| s.start);
        spans
    }

    /// Highlight multiple lines of code
    ///
    /// For better parsing accuracy when the code spans multiple lines.
    pub fn highlight_code(&mut self, code: &str) -> Vec<Vec<HighlightSpan>> {
        let Some(captures) = self.captures(code) else {
            return code.lines().map(|_| Vec::new()).collect();
        };

        lines::split_by_lines(code, &captures)
    }
}
