//! Syntax highlighter main module

use super::types::{HighlightSpan, Language, SyntaxTheme};

/// Syntax highlighter
#[derive(Clone, Debug)]
pub struct SyntaxHighlighter {
    /// Language to highlight
    pub language: Language,
    /// Color theme
    pub theme: SyntaxTheme,
}

impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self {
            language: Language::None,
            theme: SyntaxTheme::default(),
        }
    }
}

impl SyntaxHighlighter {
    /// Create a new highlighter for the given language
    pub fn new(language: Language) -> Self {
        Self {
            language,
            theme: SyntaxTheme::default(),
        }
    }

    /// Create a new highlighter with a custom theme
    pub fn with_theme(language: Language, theme: SyntaxTheme) -> Self {
        Self { language, theme }
    }

    /// Set the color theme
    pub fn theme(mut self, theme: SyntaxTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Set the language
    pub fn language(mut self, language: Language) -> Self {
        self.language = language;
        self
    }

    /// Get the current language
    pub fn get_language(&self) -> Language {
        self.language
    }

    /// Highlight a line of code
    pub fn highlight_line(&self, line: &str) -> Vec<HighlightSpan> {
        match self.language {
            Language::None => Vec::new(),
            Language::Rust => self.highlight_rust(line),
            Language::Python => self.highlight_python(line),
            Language::JavaScript => self.highlight_javascript(line),
            Language::Json => self.highlight_json(line),
            Language::Toml => self.highlight_toml(line),
            Language::Yaml => self.highlight_yaml(line),
            Language::Markdown => self.highlight_markdown(line),
            Language::Shell => self.highlight_shell(line),
            Language::Sql => self.highlight_sql(line),
            Language::Html => self.highlight_html(line),
            Language::Css => self.highlight_css(line),
            Language::Go => self.highlight_go(line),
        }
    }

    // Delegate to language-specific modules
    fn highlight_rust(&self, line: &str) -> Vec<HighlightSpan> {
        self.highlight_with(line, super::rust::highlight_rust)
    }

    fn highlight_python(&self, line: &str) -> Vec<HighlightSpan> {
        self.highlight_with(line, super::python::highlight_python)
    }

    fn highlight_javascript(&self, line: &str) -> Vec<HighlightSpan> {
        self.highlight_with(line, super::javascript::highlight_javascript)
    }

    fn highlight_json(&self, line: &str) -> Vec<HighlightSpan> {
        self.highlight_with(line, super::json::highlight_json)
    }

    fn highlight_toml(&self, line: &str) -> Vec<HighlightSpan> {
        self.highlight_with(line, super::toml::highlight_toml)
    }

    fn highlight_yaml(&self, line: &str) -> Vec<HighlightSpan> {
        self.highlight_with(line, super::yaml::highlight_yaml)
    }

    fn highlight_markdown(&self, line: &str) -> Vec<HighlightSpan> {
        self.highlight_with(line, super::markdown::highlight_markdown)
    }

    fn highlight_shell(&self, line: &str) -> Vec<HighlightSpan> {
        self.highlight_with(line, super::shell::highlight_shell)
    }

    fn highlight_sql(&self, line: &str) -> Vec<HighlightSpan> {
        self.highlight_with(line, super::sql::highlight_sql)
    }

    fn highlight_html(&self, line: &str) -> Vec<HighlightSpan> {
        self.highlight_with(line, super::html::highlight_html)
    }

    fn highlight_css(&self, line: &str) -> Vec<HighlightSpan> {
        self.highlight_with(line, super::css::highlight_css)
    }

    fn highlight_go(&self, line: &str) -> Vec<HighlightSpan> {
        self.highlight_with(line, super::go::highlight_go)
    }

    fn highlight_with(
        &self,
        line: &str,
        highlight_fn: fn(&str, &SyntaxTheme) -> Vec<HighlightSpan>,
    ) -> Vec<HighlightSpan> {
        highlight_fn(line, &self.theme)
    }
}
