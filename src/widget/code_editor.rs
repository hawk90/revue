//! CodeEditor widget for code editing with syntax highlighting
//!
//! A lightweight code editor with syntax highlighting, line numbers,
//! bracket matching, auto-indent, and other code-specific features.

mod bracket;
mod editing;
mod key_handling;
mod modes;
mod navigation;
mod render;
mod selection;
mod types;

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_code_editor_new() {
        let editor = CodeEditor::new();
        assert_eq!(editor.lines.len(), 1);
        assert_eq!(editor.cursor, (0, 0));
    }

    #[test]
    fn test_code_editor_content() {
        let editor = CodeEditor::new().content("Hello\nWorld");
        assert_eq!(editor.lines.len(), 2);
        assert_eq!(editor.lines[0], "Hello");
        assert_eq!(editor.lines[1], "World");
    }

    #[test]
    fn test_code_editor_insert_char() {
        let mut editor = CodeEditor::new();
        editor.insert_char('H');
        editor.insert_char('i');
        assert_eq!(editor.get_content(), "Hi");
    }

    #[test]
    fn test_code_editor_movement() {
        let mut editor = CodeEditor::new().content("Hello\nWorld");
        editor.move_right();
        assert_eq!(editor.cursor, (0, 1));
        editor.move_down();
        assert_eq!(editor.cursor, (1, 1));
        editor.move_left();
        assert_eq!(editor.cursor, (1, 0));
        editor.move_up();
        assert_eq!(editor.cursor, (0, 0));
    }

    #[test]
    fn test_bracket_matching() {
        let editor = CodeEditor::new()
            .content("fn main() {}")
            .bracket_matching(true);
        // Cursor at opening brace
        let mut ed = editor;
        ed.set_cursor(0, 10);
        let m = ed.find_matching_bracket();
        assert!(m.is_some());
        assert_eq!(m.unwrap().position, (0, 11));
    }
}

use std::path::Path;

use super::syntax::{Language, SyntaxHighlighter, SyntaxTheme};
use super::traits::WidgetProps;
use crate::style::Color;
use crate::{impl_props_builders, impl_styled_view};

// Public exports
pub use types::{BracketMatch, BracketPair, EditOp, EditorConfig, IndentStyle};

/// Code editor widget
pub struct CodeEditor {
    /// Lines of code
    pub(super) lines: Vec<String>,
    /// Cursor position (line, column)
    pub(super) cursor: (usize, usize),
    /// Selection anchor (if selecting)
    pub(super) anchor: Option<(usize, usize)>,
    /// Scroll offset (line, column)
    pub(super) scroll: (usize, usize),
    /// Undo stack
    pub(super) undo_stack: Vec<EditOp>,
    /// Redo stack
    pub(super) redo_stack: Vec<EditOp>,
    /// Language for syntax highlighting
    pub(super) language: Language,
    /// Syntax highlighter
    pub(super) highlighter: Option<SyntaxHighlighter>,
    /// Syntax theme
    pub(super) theme: SyntaxTheme,
    /// Editor configuration
    pub(super) config: EditorConfig,
    /// Show line numbers
    pub(super) show_line_numbers: bool,
    /// Read-only mode
    pub(super) read_only: bool,
    /// Focused state
    pub(super) focused: bool,
    /// Go-to-line mode active
    pub(super) goto_line_mode: bool,
    /// Go-to-line input buffer
    pub(super) goto_line_input: String,
    /// Find mode active
    pub(super) find_mode: bool,
    /// Find query
    pub(super) find_query: String,
    /// Find matches
    pub(super) find_matches: Vec<(usize, usize, usize)>, // (line, start, end)
    /// Current find match index
    pub(super) find_index: usize,
    /// Colors
    /// Background color
    pub bg: Option<Color>,
    /// Foreground color
    pub fg: Option<Color>,
    /// Cursor background color
    pub cursor_bg: Color,
    /// Selection background color
    pub selection_bg: Color,
    /// Line number foreground color
    pub line_number_fg: Color,
    /// Current line background color
    pub current_line_bg: Color,
    /// Bracket match background color
    pub bracket_match_bg: Color,
    /// Find match background color
    pub find_match_bg: Color,
    /// Current find match background color
    pub current_find_bg: Color,
    /// Minimap background color
    pub minimap_bg: Color,
    /// Minimap visible area background color
    pub minimap_visible_bg: Color,
    /// Widget props
    pub props: WidgetProps,
}

impl CodeEditor {
    /// Create a new code editor
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()],
            cursor: (0, 0),
            anchor: None,
            scroll: (0, 0),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            language: Language::None,
            highlighter: None,
            theme: SyntaxTheme::dark(),
            config: EditorConfig::default(),
            show_line_numbers: true,
            read_only: false,
            focused: true,
            goto_line_mode: false,
            goto_line_input: String::new(),
            find_mode: false,
            find_query: String::new(),
            find_matches: Vec::new(),
            find_index: 0,
            bg: Some(Color::rgb(30, 30, 46)),
            fg: Some(Color::rgb(205, 214, 244)),
            cursor_bg: Color::rgb(166, 227, 161),
            selection_bg: Color::rgb(69, 71, 90),
            line_number_fg: Color::rgb(88, 91, 112),
            current_line_bg: Color::rgb(49, 50, 68),
            bracket_match_bg: Color::rgb(137, 180, 250),
            find_match_bg: Color::rgb(249, 226, 175),
            current_find_bg: Color::rgb(250, 179, 135),
            minimap_bg: Color::rgb(24, 24, 37),
            minimap_visible_bg: Color::rgb(49, 50, 68),
            props: WidgetProps::new(),
        }
    }

    /// Set content
    pub fn content(mut self, text: impl Into<String>) -> Self {
        let text = text.into();
        self.lines = text.lines().map(String::from).collect();
        if self.lines.is_empty() {
            self.lines.push(String::new());
        }
        self.cursor = (0, 0);
        self.scroll = (0, 0);
        self
    }

    /// Set content (mutable)
    pub fn set_content(&mut self, text: &str) {
        self.lines = text.lines().map(String::from).collect();
        if self.lines.is_empty() {
            self.lines.push(String::new());
        }
        self.cursor = (0, 0);
        self.scroll = (0, 0);
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// Get content
    pub fn get_content(&self) -> String {
        self.lines.join("\n")
    }

    /// Set language for syntax highlighting
    pub fn language(mut self, lang: Language) -> Self {
        self.language = lang;
        if lang != Language::None {
            self.highlighter = Some(SyntaxHighlighter::with_theme(lang, self.theme.clone()));
        } else {
            self.highlighter = None;
        }
        self
    }

    /// Set language (mutable)
    pub fn set_language(&mut self, lang: Language) {
        self.language = lang;
        if lang != Language::None {
            self.highlighter = Some(SyntaxHighlighter::with_theme(lang, self.theme.clone()));
        } else {
            self.highlighter = None;
        }
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

impl Default for CodeEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl_styled_view!(CodeEditor);
impl_props_builders!(CodeEditor);

/// Create a new code editor
pub fn code_editor() -> CodeEditor {
    CodeEditor::new()
}
