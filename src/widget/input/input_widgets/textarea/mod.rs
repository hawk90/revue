//! TextArea widget for multi-line text editing
//!
//! A full-featured text editor widget with:
//! - Multi-line editing
//! - Cursor navigation
//! - Text selection
//! - Undo/redo history
//! - Line numbers
//! - Word wrap
//! - Scrolling
//! - Multi-cursor support
//! - Find/replace functionality

mod content;
mod cursor;
mod edit;
mod editing;
mod find_impl;
mod find_replace;
mod multi_cursor;
mod navigation;
mod selection;
mod undo;
mod view;

pub use cursor::{Cursor, CursorPos, CursorSet};
pub use find_replace::{FindMatch, FindOptions, FindReplaceMode, FindReplaceState};
pub use selection::Selection;

use crate::event::Key;
use crate::style::Color;
use crate::widget::syntax::{Language, SyntaxHighlighter, SyntaxTheme};
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// Maximum undo history size
pub(super) const MAX_UNDO_HISTORY: usize = 100;

/// A multi-line text editor widget
///
/// # Example
///
/// ```rust,ignore
/// use revue::prelude::*;
///
/// let mut editor = TextArea::new()
///     .content("Hello, World!\nLine 2")
///     .line_numbers(true)
///     .wrap(true);
///
/// // Handle key events
/// editor.handle_key(&Key::Char('a'));
/// ```
pub struct TextArea {
    /// Lines of text
    pub(super) lines: Vec<String>,
    /// Multiple cursors (primary cursor is at index 0)
    pub(super) cursors: CursorSet,
    /// Scroll offset (line, column)
    pub(super) scroll: (usize, usize),
    /// Undo history
    pub(super) undo_stack: Vec<edit::EditOperation>,
    /// Redo history
    pub(super) redo_stack: Vec<edit::EditOperation>,
    /// Show line numbers
    pub(super) show_line_numbers: bool,
    /// Enable word wrap
    pub(super) wrap: bool,
    /// Read-only mode
    pub(super) read_only: bool,
    /// Focused state
    pub(super) focused: bool,
    /// Tab width
    pub(super) tab_width: usize,
    /// Placeholder text
    pub(super) placeholder: Option<String>,
    /// Maximum lines (0 = unlimited)
    pub(super) max_lines: usize,
    /// Text color
    pub(super) fg: Option<Color>,
    /// Background color
    pub(super) bg: Option<Color>,
    /// Cursor color
    pub(super) cursor_fg: Option<Color>,
    /// Selection color
    pub(super) selection_bg: Option<Color>,
    /// Line number color
    pub(super) line_number_fg: Option<Color>,
    /// Syntax highlighter for code coloring
    pub(super) highlighter: Option<SyntaxHighlighter>,
    /// Find/Replace state
    pub(super) find_replace: Option<FindReplaceState>,
    /// Match highlight color
    pub(super) match_highlight_bg: Option<Color>,
    /// Current match highlight color
    pub(super) current_match_bg: Option<Color>,
    /// CSS styling properties (id, classes)
    pub(super) props: WidgetProps,
}

impl TextArea {
    /// Create a new empty text area
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()],
            cursors: CursorSet::default(),
            scroll: (0, 0),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            show_line_numbers: false,
            wrap: false,
            read_only: false,
            focused: false,
            tab_width: 4,
            placeholder: None,
            max_lines: 0,
            fg: None,
            bg: None,
            cursor_fg: None,
            selection_bg: Some(Color::rgb(50, 50, 150)),
            line_number_fg: None,
            highlighter: None,
            find_replace: None,
            match_highlight_bg: None,
            current_match_bg: None,
            props: WidgetProps::new(),
        }
    }

    /// Set initial content
    pub fn content(mut self, text: impl Into<String>) -> Self {
        self.set_content(&text.into());
        self
    }

    /// Show/hide line numbers
    pub fn line_numbers(mut self, show: bool) -> Self {
        self.show_line_numbers = show;
        self
    }

    /// Enable/disable word wrap
    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    /// Set read-only mode
    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    /// Set focused state
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Set tab width
    pub fn tab_width(mut self, width: usize) -> Self {
        self.tab_width = width;
        self
    }

    /// Set placeholder text
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(text.into());
        self
    }

    /// Set maximum lines (0 = unlimited)
    pub fn max_lines(mut self, max: usize) -> Self {
        self.max_lines = max;
        self
    }

    /// Set text color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Set cursor color
    pub fn cursor_fg(mut self, color: Color) -> Self {
        self.cursor_fg = Some(color);
        self
    }

    /// Set selection background color
    pub fn selection_bg(mut self, color: Color) -> Self {
        self.selection_bg = Some(color);
        self
    }

    /// Set line number color
    pub fn line_number_fg(mut self, color: Color) -> Self {
        self.line_number_fg = Some(color);
        self
    }

    /// Set match highlight background color
    pub fn match_highlight_bg(mut self, color: Color) -> Self {
        self.match_highlight_bg = Some(color);
        self
    }

    /// Set current match highlight background color
    pub fn current_match_bg(mut self, color: Color) -> Self {
        self.current_match_bg = Some(color);
        self
    }

    /// Enable syntax highlighting for a language
    pub fn syntax(mut self, language: Language) -> Self {
        self.highlighter = Some(SyntaxHighlighter::new(language));
        self
    }

    /// Enable syntax highlighting with a custom theme
    pub fn syntax_with_theme(mut self, language: Language, theme: SyntaxTheme) -> Self {
        self.highlighter = Some(SyntaxHighlighter::with_theme(language, theme));
        self
    }

    // =========================================================================
    // Key Handling
    // =========================================================================

    /// Handle key event
    pub fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Char(ch) => {
                self.insert_char(*ch);
                true
            }
            Key::Enter => {
                self.insert_char('\n');
                true
            }
            Key::Tab => {
                self.insert_char('\t');
                true
            }
            Key::Backspace => {
                self.delete_char_before();
                true
            }
            Key::Delete => {
                self.delete_char_at();
                true
            }
            Key::Left => {
                self.clear_selection();
                self.move_left();
                true
            }
            Key::Right => {
                self.clear_selection();
                self.move_right();
                true
            }
            Key::Up => {
                self.clear_selection();
                self.move_up();
                true
            }
            Key::Down => {
                self.clear_selection();
                self.move_down();
                true
            }
            Key::Home => {
                self.clear_selection();
                self.move_home();
                true
            }
            Key::End => {
                self.clear_selection();
                self.move_end();
                true
            }
            Key::PageUp => {
                self.page_up(10);
                true
            }
            Key::PageDown => {
                self.page_down(10);
                true
            }
            _ => false,
        }
    }
}

impl Default for TextArea {
    fn default() -> Self {
        Self::new()
    }
}

impl_styled_view!(TextArea);
impl_props_builders!(TextArea);

/// Create a new text area
pub fn textarea() -> TextArea {
    TextArea::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::Key;

    #[test]
    fn test_textarea_new_creates_empty_editor() {
        let textarea = TextArea::new();
        assert_eq!(textarea.lines.len(), 1);
        assert_eq!(textarea.lines[0], "");
        assert_eq!(textarea.scroll, (0, 0));
        assert!(!textarea.show_line_numbers);
        assert!(!textarea.wrap);
        assert!(!textarea.read_only);
        assert!(!textarea.focused);
        assert_eq!(textarea.tab_width, 4);
        assert!(textarea.placeholder.is_none());
        assert_eq!(textarea.max_lines, 0);
    }

    #[test]
    fn test_textarea_default_trait() {
        let textarea = TextArea::default();
        assert_eq!(textarea.lines.len(), 1);
        assert_eq!(textarea.tab_width, 4);
    }

    #[test]
    fn test_textarea_content_builder() {
        let textarea = TextArea::new().content("Hello\nWorld");
        assert_eq!(textarea.lines.len(), 2);
        assert_eq!(textarea.lines[0], "Hello");
        assert_eq!(textarea.lines[1], "World");
    }

    #[test]
    fn test_textarea_content_builder_single_line() {
        let textarea = TextArea::new().content("Single line");
        assert_eq!(textarea.lines.len(), 1);
        assert_eq!(textarea.lines[0], "Single line");
    }

    #[test]
    fn test_textarea_line_numbers_builder() {
        let textarea = TextArea::new().line_numbers(true);
        assert!(textarea.show_line_numbers);

        let textarea = TextArea::new().line_numbers(false);
        assert!(!textarea.show_line_numbers);
    }

    #[test]
    fn test_textarea_wrap_builder() {
        let textarea = TextArea::new().wrap(true);
        assert!(textarea.wrap);

        let textarea = TextArea::new().wrap(false);
        assert!(!textarea.wrap);
    }

    #[test]
    fn test_textarea_read_only_builder() {
        let textarea = TextArea::new().read_only(true);
        assert!(textarea.read_only);

        let textarea = TextArea::new().read_only(false);
        assert!(!textarea.read_only);
    }

    #[test]
    fn test_textarea_focused_builder() {
        let textarea = TextArea::new().focused(true);
        assert!(textarea.focused);

        let textarea = TextArea::new().focused(false);
        assert!(!textarea.focused);
    }

    #[test]
    fn test_textarea_tab_width_builder() {
        let textarea = TextArea::new().tab_width(8);
        assert_eq!(textarea.tab_width, 8);

        let textarea = TextArea::new().tab_width(2);
        assert_eq!(textarea.tab_width, 2);
    }

    #[test]
    fn test_textarea_placeholder_builder() {
        let textarea = TextArea::new().placeholder("Enter text here");
        assert_eq!(textarea.placeholder, Some("Enter text here".to_string()));
    }

    #[test]
    fn test_textarea_max_lines_builder() {
        let textarea = TextArea::new().max_lines(100);
        assert_eq!(textarea.max_lines, 100);

        let textarea = TextArea::new().max_lines(0);
        assert_eq!(textarea.max_lines, 0);
    }

    #[test]
    fn test_textarea_fg_builder() {
        let textarea = TextArea::new().fg(Color::RED);
        assert_eq!(textarea.fg, Some(Color::RED));
    }

    #[test]
    fn test_textarea_bg_builder() {
        let textarea = TextArea::new().bg(Color::BLUE);
        assert_eq!(textarea.bg, Some(Color::BLUE));
    }

    #[test]
    fn test_textarea_cursor_fg_builder() {
        let textarea = TextArea::new().cursor_fg(Color::GREEN);
        assert_eq!(textarea.cursor_fg, Some(Color::GREEN));
    }

    #[test]
    fn test_textarea_selection_bg_builder() {
        let textarea = TextArea::new().selection_bg(Color::YELLOW);
        assert_eq!(textarea.selection_bg, Some(Color::YELLOW));
    }

    #[test]
    fn test_textarea_line_number_fg_builder() {
        let textarea = TextArea::new().line_number_fg(Color::CYAN);
        assert_eq!(textarea.line_number_fg, Some(Color::CYAN));
    }

    #[test]
    fn test_textarea_match_highlight_bg_builder() {
        let textarea = TextArea::new().match_highlight_bg(Color::rgb(255, 255, 0));
        assert_eq!(textarea.match_highlight_bg, Some(Color::rgb(255, 255, 0)));
    }

    #[test]
    fn test_textarea_current_match_bg_builder() {
        let textarea = TextArea::new().current_match_bg(Color::rgb(0, 255, 255));
        assert_eq!(textarea.current_match_bg, Some(Color::rgb(0, 255, 255)));
    }

    #[test]
    fn test_textarea_syntax_builder() {
        let textarea = TextArea::new().syntax(Language::Rust);
        assert!(textarea.highlighter.is_some());
    }

    #[test]
    fn test_textarea_syntax_with_theme_builder() {
        let textarea = TextArea::new().syntax_with_theme(Language::Rust, SyntaxTheme::monokai());
        assert!(textarea.highlighter.is_some());
    }

    #[test]
    fn test_textarea_builder_chaining() {
        let textarea = TextArea::new()
            .content("Test content")
            .line_numbers(true)
            .wrap(true)
            .read_only(false)
            .focused(true)
            .tab_width(4)
            .placeholder("Placeholder")
            .max_lines(100)
            .fg(Color::WHITE)
            .bg(Color::BLACK);

        assert_eq!(textarea.lines[0], "Test content");
        assert!(textarea.show_line_numbers);
        assert!(textarea.wrap);
        assert!(!textarea.read_only);
        assert!(textarea.focused);
        assert_eq!(textarea.tab_width, 4);
        assert_eq!(textarea.placeholder, Some("Placeholder".to_string()));
        assert_eq!(textarea.max_lines, 100);
        assert_eq!(textarea.fg, Some(Color::WHITE));
        assert_eq!(textarea.bg, Some(Color::BLACK));
    }

    #[test]
    fn test_textarea_handle_key_char() {
        let mut textarea = TextArea::new();
        let handled = textarea.handle_key(&Key::Char('a'));
        assert!(handled);
    }

    #[test]
    fn test_textarea_handle_key_enter() {
        let mut textarea = TextArea::new();
        let handled = textarea.handle_key(&Key::Enter);
        assert!(handled);
    }

    #[test]
    fn test_textarea_handle_key_tab() {
        let mut textarea = TextArea::new();
        let handled = textarea.handle_key(&Key::Tab);
        assert!(handled);
    }

    #[test]
    fn test_textarea_handle_key_backspace() {
        let mut textarea = TextArea::new();
        let handled = textarea.handle_key(&Key::Backspace);
        assert!(handled);
    }

    #[test]
    fn test_textarea_handle_key_delete() {
        let mut textarea = TextArea::new();
        let handled = textarea.handle_key(&Key::Delete);
        assert!(handled);
    }

    #[test]
    fn test_textarea_handle_key_left() {
        let mut textarea = TextArea::new();
        let handled = textarea.handle_key(&Key::Left);
        assert!(handled);
    }

    #[test]
    fn test_textarea_handle_key_right() {
        let mut textarea = TextArea::new();
        let handled = textarea.handle_key(&Key::Right);
        assert!(handled);
    }

    #[test]
    fn test_textarea_handle_key_up() {
        let mut textarea = TextArea::new();
        let handled = textarea.handle_key(&Key::Up);
        assert!(handled);
    }

    #[test]
    fn test_textarea_handle_key_down() {
        let mut textarea = TextArea::new();
        let handled = textarea.handle_key(&Key::Down);
        assert!(handled);
    }

    #[test]
    fn test_textarea_handle_key_home() {
        let mut textarea = TextArea::new();
        let handled = textarea.handle_key(&Key::Home);
        assert!(handled);
    }

    #[test]
    fn test_textarea_handle_key_end() {
        let mut textarea = TextArea::new();
        let handled = textarea.handle_key(&Key::End);
        assert!(handled);
    }

    #[test]
    fn test_textarea_handle_key_page_up() {
        let mut textarea = TextArea::new();
        let handled = textarea.handle_key(&Key::PageUp);
        assert!(handled);
    }

    #[test]
    fn test_textarea_handle_key_page_down() {
        let mut textarea = TextArea::new();
        let handled = textarea.handle_key(&Key::PageDown);
        assert!(handled);
    }

    #[test]
    fn test_textarea_handle_key_unknown() {
        let mut textarea = TextArea::new();
        let handled = textarea.handle_key(&Key::Escape);
        assert!(!handled);
    }

    #[test]
    fn test_textarea_handle_key_f1() {
        let mut textarea = TextArea::new();
        let handled = textarea.handle_key(&Key::F(1));
        assert!(!handled);
    }

    #[test]
    fn test_textarea_default_selection_bg() {
        let textarea = TextArea::new();
        assert_eq!(textarea.selection_bg, Some(Color::rgb(50, 50, 150)));
    }

    #[test]
    fn test_textarea_empty_undo_stack() {
        let textarea = TextArea::new();
        assert_eq!(textarea.undo_stack.len(), 0);
    }

    #[test]
    fn test_textarea_empty_redo_stack() {
        let textarea = TextArea::new();
        assert_eq!(textarea.redo_stack.len(), 0);
    }

    #[test]
    fn test_textarea_no_find_replace_by_default() {
        let textarea = TextArea::new();
        assert!(textarea.find_replace.is_none());
    }

    #[test]
    fn test_textarea_no_highlighter_by_default() {
        let textarea = TextArea::new();
        assert!(textarea.highlighter.is_none());
    }
}
