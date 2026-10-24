//! CodeEditor widget for code editing with syntax highlighting
//!
//! A lightweight code editor with syntax highlighting, line numbers,
//! bracket matching, auto-indent, and other code-specific features.

mod bracket;
mod builders;
mod editing;
mod key_handling;
mod modes;
mod navigation;
mod render;
mod selection;
mod types;

#[cfg(test)]
mod tests;

use crate::style::Color;
use crate::utils::EditState;
use crate::widget::syntax::{Language, SyntaxHighlighter, SyntaxTheme};
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

// Public exports
// EditOp is deprecated (3.12), re-exported until 4.0
#[allow(deprecated)]
pub use types::{BracketMatch, BracketPair, EditOp, EditorConfig, IndentStyle};

/// Lines PageUp and PageDown scroll the view (and move the cursor) before
/// the editor has been rendered and so knows how many rows it shows
const DEFAULT_PAGE_HEIGHT: usize = 20;

/// Code editor widget
#[derive(Clone)]
pub struct CodeEditor {
    /// The text, its cursor with the selection, and undo (#1261)
    edit: EditState,
    /// Selection mode, from [`start_selection`](Self::start_selection) or
    /// [`select_all`](Self::select_all) until the selection is cleared or
    /// the text is edited: every move extends the selection
    selecting: bool,
    /// First visible line, kept between renders so the view only moves
    /// when the cursor would leave it
    scroll: std::cell::Cell<usize>,
    /// Horizontal scroll in terminal COLUMNS, kept between renders so the
    /// view only moves when the cursor would leave it
    scroll_x: std::cell::Cell<usize>,
    /// Rows showing lines at the last render, the step for PageUp and
    /// PageDown ([`DEFAULT_PAGE_HEIGHT`] before the first render)
    page_height: std::cell::Cell<usize>,
    /// Language for syntax highlighting
    language: Language,
    /// Syntax highlighter
    highlighter: Option<SyntaxHighlighter>,
    /// Syntax theme
    theme: SyntaxTheme,
    /// Editor configuration
    config: EditorConfig,
    /// Show line numbers
    show_line_numbers: bool,
    /// Focused state
    focused: bool,
    /// Go-to-line mode active
    goto_line_mode: bool,
    /// Go-to-line input buffer
    goto_line_input: EditState,
    /// Horizontal scroll of the go-to-line box's text, in columns
    goto_scroll: std::cell::Cell<usize>,
    /// Find mode active
    find_mode: bool,
    /// Find query
    find_query: EditState,
    /// Horizontal scroll of the find box's text, in columns
    find_scroll: std::cell::Cell<usize>,
    /// Find matches
    find_matches: Vec<(usize, usize, usize)>, // (line, start, end)
    /// Current find match index
    find_index: usize,
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
            edit: EditState::multi_line(),
            selecting: false,
            scroll: std::cell::Cell::new(0),
            scroll_x: std::cell::Cell::new(0),
            page_height: std::cell::Cell::new(DEFAULT_PAGE_HEIGHT),
            language: Language::None,
            highlighter: None,
            theme: SyntaxTheme::dark(),
            config: EditorConfig::default(),
            show_line_numbers: true,
            focused: true,
            goto_line_mode: false,
            goto_line_input: EditState::single_line(),
            goto_scroll: std::cell::Cell::new(0),
            find_mode: false,
            find_query: EditState::single_line(),
            find_scroll: std::cell::Cell::new(0),
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

    /// Set content (mutable)
    pub fn set_content(&mut self, text: &str) {
        self.reset_text(text);
        // Matches of the find query are positions in the old text
        self.refresh_find_matches();
    }

    /// Get content
    pub fn get_content(&self) -> String {
        self.edit.text()
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

    /// Get line at index
    #[doc(hidden)]
    pub fn get_line(&self, index: usize) -> Option<String> {
        self.edit.line(index).map(String::from)
    }

    /// Replace the text, with the cursor and the view at its start and no
    /// undo history. A final line break is kept, as an empty last line, so
    /// [`get_content`](Self::get_content) gives the text back.
    fn reset_text(&mut self, text: &str) {
        // A selection in the old text means nothing in the new one (#975)
        self.edit.set_text(text);
        self.selecting = false;
        self.scroll.set(0);
        self.scroll_x.set(0);
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
