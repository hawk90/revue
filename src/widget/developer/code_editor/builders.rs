//! Code editor builders: content, language, theme, configuration and colors

use std::path::Path;

use super::types::{EditorConfig, IndentStyle};
use super::CodeEditor;
use crate::style::Color;
use crate::widget::syntax::{Language, SyntaxHighlighter, SyntaxTheme};

impl CodeEditor {
    /// Set content
    pub fn content(mut self, text: impl Into<String>) -> Self {
        let text = text.into();
        self.reset_text(&text);
        self
    }

    /// Set language for syntax highlighting
    pub fn language(mut self, lang: Language) -> Self {
        self.set_language(lang);
        self
    }

    /// Detect language from file extension
    pub fn detect_language(mut self, filename: &str) -> Self {
        if let Some(ext) = Path::new(filename).extension().and_then(|e| e.to_str()) {
            let lang = Language::from_extension(ext);
            self = self.language(lang);
        }
        self
    }

    /// Set syntax theme
    pub fn theme(mut self, theme: SyntaxTheme) -> Self {
        self.theme = theme.clone();
        if let Some(ref mut hl) = self.highlighter {
            *hl = SyntaxHighlighter::with_theme(self.language, theme);
        }
        self
    }

    /// Set editor configuration
    pub fn config(mut self, config: EditorConfig) -> Self {
        self.config = config;
        self
    }

    /// Enable/disable line numbers
    pub fn line_numbers(mut self, show: bool) -> Self {
        self.show_line_numbers = show;
        self
    }

    /// Enable/disable read-only mode
    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    /// Set focused state
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Set indent size
    pub fn indent_size(mut self, size: usize) -> Self {
        self.config.indent_size = size.max(1);
        self
    }

    /// Set indent style
    pub fn indent_style(mut self, style: IndentStyle) -> Self {
        self.config.indent_style = style;
        self
    }

    /// Enable/disable auto-indent
    pub fn auto_indent(mut self, enable: bool) -> Self {
        self.config.auto_indent = enable;
        self
    }

    /// Enable/disable bracket matching
    pub fn bracket_matching(mut self, enable: bool) -> Self {
        self.config.bracket_matching = enable;
        self
    }

    /// Enable/disable current line highlight
    pub fn highlight_current_line(mut self, enable: bool) -> Self {
        self.config.highlight_current_line = enable;
        self
    }

    /// Enable/disable minimap
    pub fn minimap(mut self, show: bool) -> Self {
        self.config.show_minimap = show;
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Set foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }
}
