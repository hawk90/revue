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

use super::syntax::{Language, SyntaxHighlighter, SyntaxTheme};
use super::WidgetProps;
use crate::event::Key;
use crate::style::Color;
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
            selection_bg: None,
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
