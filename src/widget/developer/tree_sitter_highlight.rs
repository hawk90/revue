//! Tree-sitter based syntax highlighting
//!
//! Provides accurate syntax highlighting using tree-sitter parsers.
//! This module is only available when the `syntax-highlighting` feature is enabled.

use std::collections::HashMap;
use std::sync::OnceLock;

use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent, Highlighter};

use crate::style::Color;
use crate::widget::syntax::{HighlightSpan, Language, SyntaxTheme};

/// Standard highlight capture names used by tree-sitter grammars
const HIGHLIGHT_NAMES: &[&str] = &[
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

/// Language configurations cache
struct LanguageConfigs {
    configs: HashMap<Language, HighlightConfiguration>,
}

impl LanguageConfigs {
    fn new() -> Self {
        let mut configs = HashMap::new();

        // Rust
        if let Ok(mut config) = HighlightConfiguration::new(
            tree_sitter_rust::LANGUAGE.into(),
            "rust",
            tree_sitter_rust::HIGHLIGHTS_QUERY,
            tree_sitter_rust::INJECTIONS_QUERY,
            "",
        ) {
            config.configure(HIGHLIGHT_NAMES);
            configs.insert(Language::Rust, config);
        }

        // Python
        if let Ok(mut config) = HighlightConfiguration::new(
            tree_sitter_python::LANGUAGE.into(),
            "python",
            tree_sitter_python::HIGHLIGHTS_QUERY,
            "",
            "",
        ) {
            config.configure(HIGHLIGHT_NAMES);
            configs.insert(Language::Python, config);
        }

        // JavaScript
        if let Ok(mut config) = HighlightConfiguration::new(
            tree_sitter_javascript::LANGUAGE.into(),
            "javascript",
            tree_sitter_javascript::HIGHLIGHT_QUERY,
            tree_sitter_javascript::INJECTIONS_QUERY,
            tree_sitter_javascript::LOCALS_QUERY,
        ) {
            config.configure(HIGHLIGHT_NAMES);
            configs.insert(Language::JavaScript, config);
        }

        // JSON
        if let Ok(mut config) = HighlightConfiguration::new(
            tree_sitter_json::LANGUAGE.into(),
            "json",
            tree_sitter_json::HIGHLIGHTS_QUERY,
            "",
            "",
        ) {
            config.configure(HIGHLIGHT_NAMES);
            configs.insert(Language::Json, config);
        }

        // Go
        if let Ok(mut config) = HighlightConfiguration::new(
            tree_sitter_go::LANGUAGE.into(),
            "go",
            tree_sitter_go::HIGHLIGHTS_QUERY,
            "",
            "",
        ) {
            config.configure(HIGHLIGHT_NAMES);
            configs.insert(Language::Go, config);
        }

        // Bash/Shell
        if let Ok(mut config) = HighlightConfiguration::new(
            tree_sitter_bash::LANGUAGE.into(),
            "bash",
            tree_sitter_bash::HIGHLIGHT_QUERY,
            "",
            "",
        ) {
            config.configure(HIGHLIGHT_NAMES);
            configs.insert(Language::Shell, config);
        }

        // HTML
        if let Ok(mut config) = HighlightConfiguration::new(
            tree_sitter_html::LANGUAGE.into(),
            "html",
            tree_sitter_html::HIGHLIGHTS_QUERY,
            tree_sitter_html::INJECTIONS_QUERY,
            "",
        ) {
            config.configure(HIGHLIGHT_NAMES);
            configs.insert(Language::Html, config);
        }

        // CSS
        if let Ok(mut config) = HighlightConfiguration::new(
            tree_sitter_css::LANGUAGE.into(),
            "css",
            tree_sitter_css::HIGHLIGHTS_QUERY,
            "",
            "",
        ) {
            config.configure(HIGHLIGHT_NAMES);
            configs.insert(Language::Css, config);
        }

        // TOML
        if let Ok(mut config) = HighlightConfiguration::new(
            tree_sitter_toml_ng::LANGUAGE.into(),
            "toml",
            tree_sitter_toml_ng::HIGHLIGHTS_QUERY,
            "",
            "",
        ) {
            config.configure(HIGHLIGHT_NAMES);
            configs.insert(Language::Toml, config);
        }

        // YAML
        if let Ok(mut config) = HighlightConfiguration::new(
            tree_sitter_yaml::LANGUAGE.into(),
            "yaml",
            tree_sitter_yaml::HIGHLIGHTS_QUERY,
            "",
            "",
        ) {
            config.configure(HIGHLIGHT_NAMES);
            configs.insert(Language::Yaml, config);
        }

        // SQL (using tree-sitter-sequel)
        if let Ok(mut config) = HighlightConfiguration::new(
            tree_sitter_sequel::LANGUAGE.into(),
            "sql",
            tree_sitter_sequel::HIGHLIGHTS_QUERY,
            "",
            "",
        ) {
            config.configure(HIGHLIGHT_NAMES);
            configs.insert(Language::Sql, config);
        }

        // Markdown
        if let Ok(mut config) = HighlightConfiguration::new(
            tree_sitter_md::LANGUAGE.into(),
            "markdown",
            tree_sitter_md::HIGHLIGHT_QUERY_BLOCK,
            tree_sitter_md::INJECTION_QUERY_BLOCK,
            "",
        ) {
            config.configure(HIGHLIGHT_NAMES);
            configs.insert(Language::Markdown, config);
        }

        Self { configs }
    }

    fn get(&self, language: Language) -> Option<&HighlightConfiguration> {
        self.configs.get(&language)
    }
}

/// Get the global language configurations
fn get_configs() -> &'static LanguageConfigs {
    static CONFIGS: OnceLock<LanguageConfigs> = OnceLock::new();
    CONFIGS.get_or_init(LanguageConfigs::new)
}

/// Tree-sitter based syntax highlighter
pub struct TreeSitterHighlighter {
    highlighter: Highlighter,
    language: Language,
    theme: SyntaxTheme,
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
        language != Language::None && get_configs().get(language).is_some()
    }

    /// Highlight a line of code
    ///
    /// Returns highlight spans for the given line using tree-sitter parsing.
    pub fn highlight_line(&mut self, line: &str) -> Vec<HighlightSpan> {
        if self.language == Language::None {
            return Vec::new();
        }

        let Some(config) = get_configs().get(self.language) else {
            return Vec::new();
        };

        let source = line.as_bytes();
        let highlights = match self.highlighter.highlight(config, source, None, |_| None) {
            Ok(h) => h,
            Err(_) => return Vec::new(),
        };

        let mut spans = Vec::new();
        let mut current_highlight: Option<usize> = None;

        for event in highlights {
            let Ok(event) = event else {
                continue;
            };

            match event {
                HighlightEvent::Source { start, end } => {
                    if let Some(highlight_idx) = current_highlight {
                        let name = HIGHLIGHT_NAMES
                            .get(highlight_idx)
                            .copied()
                            .unwrap_or("punctuation");
                        let color = highlight_name_to_color(name, &self.theme);
                        let bold = highlight_name_is_bold(name);
                        let italic = highlight_name_is_italic(name);

                        let mut span = HighlightSpan::new(start, end, color);
                        if bold {
                            span = span.bold();
                        }
                        if italic {
                            span = span.italic();
                        }
                        spans.push(span);
                    }
                }
                HighlightEvent::HighlightStart(highlight) => {
                    current_highlight = Some(highlight.0);
                }
                HighlightEvent::HighlightEnd => {
                    current_highlight = None;
                }
            }
        }

        // Sort spans by start position
        spans.sort_by_key(|s| s.start);
        spans
    }

    /// Highlight multiple lines of code
    ///
    /// For better parsing accuracy when the code spans multiple lines.
    pub fn highlight_code(&mut self, code: &str) -> Vec<Vec<HighlightSpan>> {
        if self.language == Language::None {
            return code.lines().map(|_| Vec::new()).collect();
        }

        let Some(config) = get_configs().get(self.language) else {
            return code.lines().map(|_| Vec::new()).collect();
        };

        let source = code.as_bytes();
        let highlights = match self.highlighter.highlight(config, source, None, |_| None) {
            Ok(h) => h,
            Err(_) => return code.lines().map(|_| Vec::new()).collect(),
        };

        // Build line offset map
        let mut line_offsets: Vec<usize> = vec![0];
        for (i, c) in code.char_indices() {
            if c == '\n' {
                line_offsets.push(i + 1);
            }
        }

        let num_lines = code.lines().count();
        let mut line_spans: Vec<Vec<HighlightSpan>> = (0..num_lines).map(|_| Vec::new()).collect();

        let mut current_highlight: Option<usize> = None;

        for event in highlights {
            let Ok(event) = event else {
                continue;
            };

            match event {
                HighlightEvent::Source { start, end } => {
                    if let Some(highlight_idx) = current_highlight {
                        let name = HIGHLIGHT_NAMES
                            .get(highlight_idx)
                            .copied()
                            .unwrap_or("punctuation");
                        let color = highlight_name_to_color(name, &self.theme);
                        let bold = highlight_name_is_bold(name);
                        let italic = highlight_name_is_italic(name);

                        // Find which lines this span covers
                        let start_line = line_offsets
                            .iter()
                            .position(|&offset| offset > start)
                            .map(|p| p.saturating_sub(1))
                            .unwrap_or(line_offsets.len().saturating_sub(1));
                        let end_line = line_offsets
                            .iter()
                            .position(|&offset| offset > end)
                            .map(|p| p.saturating_sub(1))
                            .unwrap_or(line_offsets.len().saturating_sub(1));

                        for line_idx in start_line..=end_line {
                            if line_idx >= num_lines {
                                break;
                            }

                            let line_start = line_offsets[line_idx];
                            let line_end = line_offsets
                                .get(line_idx + 1)
                                .map(|&o| o.saturating_sub(1))
                                .unwrap_or(code.len());

                            let span_start_in_line = start.saturating_sub(line_start);
                            let span_end_in_line = end.min(line_end).saturating_sub(line_start);

                            if span_start_in_line < span_end_in_line {
                                let mut span =
                                    HighlightSpan::new(span_start_in_line, span_end_in_line, color);
                                if bold {
                                    span = span.bold();
                                }
                                if italic {
                                    span = span.italic();
                                }
                                line_spans[line_idx].push(span);
                            }
                        }
                    }
                }
                HighlightEvent::HighlightStart(highlight) => {
                    current_highlight = Some(highlight.0);
                }
                HighlightEvent::HighlightEnd => {
                    current_highlight = None;
                }
            }
        }

        // Sort spans in each line
        for spans in &mut line_spans {
            spans.sort_by_key(|s| s.start);
        }

        line_spans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_sitter_rust_highlight() {
        let mut hl = TreeSitterHighlighter::new(Language::Rust);
        let spans = hl.highlight_line("fn main() {");
        // Should have at least the keyword 'fn' highlighted
        assert!(!spans.is_empty());
    }

    #[test]
    fn test_tree_sitter_python_highlight() {
        let mut hl = TreeSitterHighlighter::new(Language::Python);
        let spans = hl.highlight_line("def hello():");
        assert!(!spans.is_empty());
    }

    #[test]
    fn test_tree_sitter_javascript_highlight() {
        let mut hl = TreeSitterHighlighter::new(Language::JavaScript);
        let spans = hl.highlight_line("const x = 42;");
        assert!(!spans.is_empty());
    }

    #[test]
    fn test_tree_sitter_json_highlight() {
        let mut hl = TreeSitterHighlighter::new(Language::Json);
        let spans = hl.highlight_line("{\"key\": \"value\"}");
        assert!(!spans.is_empty());
    }

    #[test]
    fn test_tree_sitter_no_language() {
        let mut hl = TreeSitterHighlighter::new(Language::None);
        let spans = hl.highlight_line("fn main() {}");
        assert!(spans.is_empty());
    }

    #[test]
    fn test_tree_sitter_multiline() {
        let mut hl = TreeSitterHighlighter::new(Language::Rust);
        let code = "fn main() {\n    println!(\"hello\");\n}";
        let line_spans = hl.highlight_code(code);
        assert_eq!(line_spans.len(), 3);
    }

    #[test]
    fn test_is_supported() {
        assert!(TreeSitterHighlighter::is_supported(Language::Rust));
        assert!(TreeSitterHighlighter::is_supported(Language::Python));
        assert!(!TreeSitterHighlighter::is_supported(Language::None));
    }
}
