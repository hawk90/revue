//! Capture names: the names tree-sitter grammars tag source ranges with, and
//! how each one is drawn with a [`SyntaxTheme`]

use crate::style::Color;
use crate::widget::syntax::{HighlightSpan, SyntaxTheme};

/// Standard highlight capture names used by tree-sitter grammars
pub(super) const HIGHLIGHT_NAMES: &[&str] = &[
    "attribute",
    "comment",
    "constant",
    "constant.builtin",
    "constructor",
    "embedded",
    "escape",
    "function",
    "function.builtin",
    "function.method",
    "keyword",
    "label",
    "number",
    "operator",
    "property",
    "punctuation",
    "punctuation.bracket",
    "punctuation.delimiter",
    "punctuation.special",
    "string",
    "string.special",
    "tag",
    "type",
    "type.builtin",
    "variable",
    "variable.builtin",
    "variable.parameter",
];

/// Map highlight names to theme colors
fn highlight_name_to_color(name: &str, theme: &SyntaxTheme) -> Color {
    match name {
        "keyword" => theme.keyword,
        "type" | "type.builtin" | "constructor" => theme.type_name,
        "function" | "function.builtin" | "function.method" => theme.function,
        "string" | "string.special" | "escape" => theme.string,
        "number" => theme.number,
        "comment" => theme.comment,
        "operator" => theme.operator,
        "punctuation" | "punctuation.bracket" | "punctuation.delimiter" | "punctuation.special" => {
            theme.punctuation
        }
        "constant" | "constant.builtin" => theme.constant,
        "attribute" | "tag" | "label" => theme.attribute,
        "variable" | "variable.builtin" | "variable.parameter" | "property" => theme.variable,
        "embedded" => theme.function,
        _ => theme.punctuation, // default
    }
}

/// Map highlight names to bold style
fn highlight_name_is_bold(name: &str) -> bool {
    matches!(name, "keyword" | "constant" | "constant.builtin")
}

/// Map highlight names to italic style
fn highlight_name_is_italic(name: &str) -> bool {
    matches!(name, "comment")
}

/// How a capture is drawn: its theme color, and whether it is bold or italic
#[derive(Clone, Copy)]
pub(super) struct CaptureStyle {
    color: Color,
    bold: bool,
    italic: bool,
}

impl CaptureStyle {
    /// The style of capture `highlight_idx` (an index into `HIGHLIGHT_NAMES`)
    pub(super) fn of(highlight_idx: usize, theme: &SyntaxTheme) -> Self {
        let name = HIGHLIGHT_NAMES
            .get(highlight_idx)
            .copied()
            .unwrap_or("punctuation");
        let color = highlight_name_to_color(name, theme);
        let bold = highlight_name_is_bold(name);
        let italic = highlight_name_is_italic(name);
        Self {
            color,
            bold,
            italic,
        }
    }

    /// A span over `start..end` in this style
    pub(super) fn span(self, start: usize, end: usize) -> HighlightSpan {
        let mut span = HighlightSpan::new(start, end, self.color);
        if self.bold {
            span = span.bold();
        }
        if self.italic {
            span = span.italic();
        }
        span
    }
}
