//! CodeEditor widget for code editing with syntax highlighting
//!
//! A lightweight code editor with syntax highlighting, line numbers,
//! bracket matching, auto-indent, and other code-specific features.

use super::syntax::{HighlightSpan, Language, SyntaxHighlighter, SyntaxTheme};
use super::traits::{RenderContext, View, WidgetProps};
use crate::event::Key;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::{impl_props_builders, impl_styled_view};

/// Maximum undo history size
const MAX_UNDO_HISTORY: usize = 100;

/// Bracket pair for matching
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BracketPair {
    /// Opening bracket position (line, col)
    pub open: (usize, usize),
    /// Closing bracket position (line, col)
    pub close: (usize, usize),
}

/// A bracket match result
#[derive(Clone, Copy, Debug)]
pub struct BracketMatch {
    /// Position of the matching bracket
    pub position: (usize, usize),
    /// The matching character
    pub char: char,
}

/// Indent style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IndentStyle {
    /// Use spaces for indentation
    #[default]
    Spaces,
    /// Use tabs for indentation
    Tabs,
}

/// Code editor configuration
#[derive(Clone, Debug)]
pub struct EditorConfig {
    /// Indent style (spaces or tabs)
    pub indent_style: IndentStyle,
    /// Indent size (number of spaces or tab width)
    pub indent_size: usize,
    /// Enable auto-indent on newline
    pub auto_indent: bool,
    /// Enable bracket matching
    pub bracket_matching: bool,
    /// Enable current line highlight
    pub highlight_current_line: bool,
    /// Enable minimap
    pub show_minimap: bool,
    /// Minimap width
    pub minimap_width: u16,
    /// Show whitespace characters
    pub show_whitespace: bool,
    /// Enable word wrap
    pub word_wrap: bool,
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            indent_style: IndentStyle::Spaces,
            indent_size: 4,
            auto_indent: true,
            bracket_matching: true,
            highlight_current_line: true,
            show_minimap: false,
            minimap_width: 10,
            show_whitespace: false,
            word_wrap: false,
        }
    }
}

/// Edit operation for undo/redo
#[derive(Clone, Debug)]
enum EditOp {
    Insert {
        line: usize,
        col: usize,
        text: String,
    },
    Delete {
        line: usize,
        col: usize,
        text: String,
    },
    SplitLine {
        line: usize,
        col: usize,
    },
    MergeLine {
        line: usize,
        col: usize,
    },
}

/// Code editor widget
pub struct CodeEditor {
    /// Lines of code
    lines: Vec<String>,
    /// Cursor position (line, column)
    cursor: (usize, usize),
    /// Selection anchor (if selecting)
    anchor: Option<(usize, usize)>,
    /// Scroll offset (line, column)
    scroll: (usize, usize),
    /// Undo stack
    undo_stack: Vec<EditOp>,
    /// Redo stack
    redo_stack: Vec<EditOp>,
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
    /// Read-only mode
    read_only: bool,
    /// Focused state
    focused: bool,
    /// Go-to-line mode active
    goto_line_mode: bool,
    /// Go-to-line input buffer
    goto_line_input: String,
    /// Find mode active
    find_mode: bool,
    /// Find query
    find_query: String,
    /// Find matches
    find_matches: Vec<(usize, usize, usize)>, // (line, start, end)
    /// Current find match index
    find_index: usize,
    /// Colors
    bg: Option<Color>,
    fg: Option<Color>,
    cursor_bg: Color,
    selection_bg: Color,
    line_number_fg: Color,
    current_line_bg: Color,
    bracket_match_bg: Color,
    find_match_bg: Color,
    current_find_bg: Color,
    minimap_bg: Color,
    minimap_visible_bg: Color,
    /// Widget props
    props: WidgetProps,
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
        if let Some(ext) = filename.rsplit('.').next() {
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

    // =========================================================================
    // Cursor and Navigation
    // =========================================================================

    /// Get cursor position
    pub fn cursor_position(&self) -> (usize, usize) {
        self.cursor
    }

    /// Set cursor position
    pub fn set_cursor(&mut self, line: usize, col: usize) {
        let line = line.min(self.lines.len().saturating_sub(1));
        let col = col.min(self.line_len(line));
        self.cursor = (line, col);
        self.ensure_cursor_visible();
    }

    /// Get line count
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    /// Get line length
    fn line_len(&self, line: usize) -> usize {
        self.lines.get(line).map(|l| l.len()).unwrap_or(0)
    }

    /// Move cursor left
    pub fn move_left(&mut self) {
        if self.cursor.1 > 0 {
            self.cursor.1 -= 1;
        } else if self.cursor.0 > 0 {
            self.cursor.0 -= 1;
            self.cursor.1 = self.line_len(self.cursor.0);
        }
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Move cursor right
    pub fn move_right(&mut self) {
        let line_len = self.line_len(self.cursor.0);
        if self.cursor.1 < line_len {
            self.cursor.1 += 1;
        } else if self.cursor.0 + 1 < self.lines.len() {
            self.cursor.0 += 1;
            self.cursor.1 = 0;
        }
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Move cursor up
    pub fn move_up(&mut self) {
        if self.cursor.0 > 0 {
            self.cursor.0 -= 1;
            self.cursor.1 = self.cursor.1.min(self.line_len(self.cursor.0));
        }
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Move cursor down
    pub fn move_down(&mut self) {
        if self.cursor.0 + 1 < self.lines.len() {
            self.cursor.0 += 1;
            self.cursor.1 = self.cursor.1.min(self.line_len(self.cursor.0));
        }
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Move to start of line
    pub fn move_home(&mut self) {
        // Smart home: first go to first non-whitespace, then to column 0
        let line = &self.lines[self.cursor.0];
        let first_non_ws = line.chars().position(|c| !c.is_whitespace()).unwrap_or(0);

        if self.cursor.1 == first_non_ws || self.cursor.1 == 0 {
            self.cursor.1 = if self.cursor.1 == 0 { first_non_ws } else { 0 };
        } else {
            self.cursor.1 = first_non_ws;
        }
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Move to end of line
    pub fn move_end(&mut self) {
        self.cursor.1 = self.line_len(self.cursor.0);
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Move to start of document
    pub fn move_document_start(&mut self) {
        self.cursor = (0, 0);
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Move to end of document
    pub fn move_document_end(&mut self) {
        let last_line = self.lines.len().saturating_sub(1);
        self.cursor = (last_line, self.line_len(last_line));
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Move by word left
    pub fn move_word_left(&mut self) {
        if self.cursor.1 == 0 {
            if self.cursor.0 > 0 {
                self.cursor.0 -= 1;
                self.cursor.1 = self.line_len(self.cursor.0);
            }
            return;
        }

        let line = &self.lines[self.cursor.0];
        let chars: Vec<char> = line.chars().collect();
        let mut col = self.cursor.1.min(chars.len());

        // Skip whitespace
        while col > 0 && chars[col - 1].is_whitespace() {
            col -= 1;
        }
        // Skip word
        while col > 0 && !chars[col - 1].is_whitespace() {
            col -= 1;
        }

        self.cursor.1 = col;
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Move by word right
    pub fn move_word_right(&mut self) {
        let line = &self.lines[self.cursor.0];
        let chars: Vec<char> = line.chars().collect();
        let mut col = self.cursor.1;

        if col >= chars.len() {
            if self.cursor.0 + 1 < self.lines.len() {
                self.cursor.0 += 1;
                self.cursor.1 = 0;
            }
            return;
        }

        // Skip current word
        while col < chars.len() && !chars[col].is_whitespace() {
            col += 1;
        }
        // Skip whitespace
        while col < chars.len() && chars[col].is_whitespace() {
            col += 1;
        }

        self.cursor.1 = col;
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Page up
    pub fn page_up(&mut self, page_size: usize) {
        self.cursor.0 = self.cursor.0.saturating_sub(page_size);
        self.cursor.1 = self.cursor.1.min(self.line_len(self.cursor.0));
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Page down
    pub fn page_down(&mut self, page_size: usize) {
        self.cursor.0 = (self.cursor.0 + page_size).min(self.lines.len().saturating_sub(1));
        self.cursor.1 = self.cursor.1.min(self.line_len(self.cursor.0));
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Ensure cursor is visible
    fn ensure_cursor_visible(&mut self) {
        // Adjust vertical scroll
        if self.cursor.0 < self.scroll.0 {
            self.scroll.0 = self.cursor.0;
        }
        // Horizontal scroll adjustment would need view width
    }

    // =========================================================================
    // Selection
    // =========================================================================

    /// Start selection
    pub fn start_selection(&mut self) {
        self.anchor = Some(self.cursor);
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.anchor = None;
    }

    /// Check if there's a selection
    pub fn has_selection(&self) -> bool {
        self.anchor.is_some()
    }

    /// Get selected text
    pub fn get_selection(&self) -> Option<String> {
        let anchor = self.anchor?;
        let (start, end) = if anchor < self.cursor {
            (anchor, self.cursor)
        } else {
            (self.cursor, anchor)
        };

        let mut result = String::new();
        for line_idx in start.0..=end.0 {
            if line_idx >= self.lines.len() {
                break;
            }
            let line = &self.lines[line_idx];
            let start_col = if line_idx == start.0 { start.1 } else { 0 };
            let end_col = if line_idx == end.0 { end.1 } else { line.len() };

            if start_col < line.len() {
                result.push_str(&line[start_col..end_col.min(line.len())]);
            }
            if line_idx < end.0 {
                result.push('\n');
            }
        }

        Some(result)
    }

    /// Delete selection
    pub fn delete_selection(&mut self) {
        let anchor = match self.anchor {
            Some(a) => a,
            None => return,
        };

        let (start, end) = if anchor < self.cursor {
            (anchor, self.cursor)
        } else {
            (self.cursor, anchor)
        };

        if start.0 == end.0 {
            // Single line
            if let Some(line) = self.lines.get_mut(start.0) {
                let deleted: String = line.drain(start.1..end.1.min(line.len())).collect();
                self.push_undo(EditOp::Delete {
                    line: start.0,
                    col: start.1,
                    text: deleted,
                });
            }
        } else {
            // Multi-line
            let before: String = self.lines[start.0].chars().take(start.1).collect();
            let after: String = self.lines[end.0].chars().skip(end.1).collect();

            for _ in start.0..=end.0 {
                if start.0 < self.lines.len() {
                    self.lines.remove(start.0);
                }
            }

            self.lines.insert(start.0, format!("{}{}", before, after));
        }

        self.cursor = start;
        self.anchor = None;
    }

    /// Select all
    pub fn select_all(&mut self) {
        self.anchor = Some((0, 0));
        let last_line = self.lines.len().saturating_sub(1);
        self.cursor = (last_line, self.line_len(last_line));
    }

    // =========================================================================
    // Editing
    // =========================================================================

    /// Push undo operation
    fn push_undo(&mut self, op: EditOp) {
        self.undo_stack.push(op);
        if self.undo_stack.len() > MAX_UNDO_HISTORY {
            self.undo_stack.remove(0);
        }
        self.redo_stack.clear();
    }

    /// Insert character
    pub fn insert_char(&mut self, ch: char) {
        if self.read_only {
            return;
        }

        if self.has_selection() {
            self.delete_selection();
        }

        if ch == '\n' {
            self.insert_newline();
            return;
        }

        if ch == '\t' {
            self.insert_indent();
            return;
        }

        if let Some(line) = self.lines.get_mut(self.cursor.0) {
            let col = self.cursor.1.min(line.len());
            line.insert(col, ch);
            self.push_undo(EditOp::Insert {
                line: self.cursor.0,
                col,
                text: ch.to_string(),
            });
            self.cursor.1 = col + 1;
        }

        // Auto-close brackets
        if self.config.bracket_matching {
            let close = match ch {
                '(' => Some(')'),
                '[' => Some(']'),
                '{' => Some('}'),
                '"' => Some('"'),
                '\'' => Some('\''),
                _ => None,
            };
            if let Some(close_ch) = close {
                if let Some(line) = self.lines.get_mut(self.cursor.0) {
                    let col = self.cursor.1.min(line.len());
                    line.insert(col, close_ch);
                }
            }
        }
    }

    /// Insert string
    pub fn insert_str(&mut self, s: &str) {
        if self.read_only {
            return;
        }

        if self.has_selection() {
            self.delete_selection();
        }

        for ch in s.chars() {
            if ch == '\n' {
                self.insert_newline();
            } else if let Some(line) = self.lines.get_mut(self.cursor.0) {
                let col = self.cursor.1.min(line.len());
                line.insert(col, ch);
                self.cursor.1 = col + 1;
            }
        }
    }

    /// Insert newline with auto-indent
    fn insert_newline(&mut self) {
        if self.read_only {
            return;
        }

        let (line_idx, col) = self.cursor;

        // Get current line's indentation
        let indent = if self.config.auto_indent {
            let current_line = &self.lines[line_idx];
            let leading_ws: String = current_line
                .chars()
                .take_while(|c| c.is_whitespace())
                .collect();

            // Check if we should add extra indent (after opening bracket)
            let trimmed = current_line.trim_end();
            let extra_indent = if !trimmed.is_empty() {
                let last_char = trimmed.chars().last().unwrap();
                matches!(last_char, '{' | '[' | '(' | ':')
            } else {
                false
            };

            let base = leading_ws;
            if extra_indent {
                let indent_str = match self.config.indent_style {
                    IndentStyle::Spaces => " ".repeat(self.config.indent_size),
                    IndentStyle::Tabs => "\t".to_string(),
                };
                format!("{}{}", base, indent_str)
            } else {
                base
            }
        } else {
            String::new()
        };

        // Split line
        if let Some(current) = self.lines.get_mut(line_idx) {
            let rest: String = current.drain(col.min(current.len())..).collect();
            let new_line = format!("{}{}", indent, rest);
            self.lines.insert(line_idx + 1, new_line);
            self.push_undo(EditOp::SplitLine {
                line: line_idx,
                col,
            });
            self.cursor = (line_idx + 1, indent.len());
        }

        self.ensure_cursor_visible();
    }

    /// Insert indent
    fn insert_indent(&mut self) {
        if self.read_only {
            return;
        }

        let indent = match self.config.indent_style {
            IndentStyle::Spaces => " ".repeat(self.config.indent_size),
            IndentStyle::Tabs => "\t".to_string(),
        };

        if let Some(line) = self.lines.get_mut(self.cursor.0) {
            let col = self.cursor.1.min(line.len());
            line.insert_str(col, &indent);
            self.push_undo(EditOp::Insert {
                line: self.cursor.0,
                col,
                text: indent.clone(),
            });
            self.cursor.1 = col + indent.len();
        }
    }

    /// Delete character before cursor (backspace)
    pub fn delete_char_before(&mut self) {
        if self.read_only {
            return;
        }

        if self.has_selection() {
            self.delete_selection();
            return;
        }

        let (line_idx, col) = self.cursor;
        if col > 0 {
            if let Some(line) = self.lines.get_mut(line_idx) {
                if col <= line.len() {
                    let deleted = line.remove(col - 1);
                    self.push_undo(EditOp::Delete {
                        line: line_idx,
                        col: col - 1,
                        text: deleted.to_string(),
                    });
                    self.cursor.1 = col - 1;
                }
            }
        } else if line_idx > 0 {
            // Merge with previous line
            let current = self.lines.remove(line_idx);
            let prev_len = self.lines[line_idx - 1].len();
            self.lines[line_idx - 1].push_str(&current);
            self.push_undo(EditOp::MergeLine {
                line: line_idx - 1,
                col: prev_len,
            });
            self.cursor = (line_idx - 1, prev_len);
        }

        self.ensure_cursor_visible();
    }

    /// Delete character at cursor (delete key)
    pub fn delete_char_at(&mut self) {
        if self.read_only {
            return;
        }

        if self.has_selection() {
            self.delete_selection();
            return;
        }

        let (line_idx, col) = self.cursor;
        if let Some(line) = self.lines.get_mut(line_idx) {
            if col < line.len() {
                let deleted = line.remove(col);
                self.push_undo(EditOp::Delete {
                    line: line_idx,
                    col,
                    text: deleted.to_string(),
                });
            } else if line_idx + 1 < self.lines.len() {
                // Merge with next line
                let next = self.lines.remove(line_idx + 1);
                self.lines[line_idx].push_str(&next);
                self.push_undo(EditOp::MergeLine {
                    line: line_idx,
                    col,
                });
            }
        }
    }

    /// Delete current line
    pub fn delete_line(&mut self) {
        if self.read_only || self.lines.len() <= 1 {
            return;
        }

        let line_idx = self.cursor.0;
        self.lines.remove(line_idx);
        self.cursor.0 = line_idx.min(self.lines.len().saturating_sub(1));
        self.cursor.1 = 0;
        self.ensure_cursor_visible();
    }

    /// Duplicate current line
    pub fn duplicate_line(&mut self) {
        if self.read_only {
            return;
        }

        let line_idx = self.cursor.0;
        let content = self.lines[line_idx].clone();
        self.lines.insert(line_idx + 1, content);
        self.cursor.0 = line_idx + 1;
        self.ensure_cursor_visible();
    }

    /// Undo
    pub fn undo(&mut self) {
        if let Some(op) = self.undo_stack.pop() {
            match &op {
                EditOp::Insert { line, col, text } => {
                    if let Some(l) = self.lines.get_mut(*line) {
                        let end = (*col + text.len()).min(l.len());
                        l.drain(*col..end);
                    }
                    self.cursor = (*line, *col);
                }
                EditOp::Delete { line, col, text } => {
                    if let Some(l) = self.lines.get_mut(*line) {
                        l.insert_str(*col, text);
                    }
                    self.cursor = (*line, *col + text.len());
                }
                EditOp::SplitLine { line, col } => {
                    if *line + 1 < self.lines.len() {
                        let next = self.lines.remove(*line + 1);
                        if let Some(l) = self.lines.get_mut(*line) {
                            l.push_str(&next);
                        }
                    }
                    self.cursor = (*line, *col);
                }
                EditOp::MergeLine { line, col } => {
                    if let Some(l) = self.lines.get_mut(*line) {
                        let rest: String = l.drain(*col..).collect();
                        self.lines.insert(*line + 1, rest);
                    }
                    self.cursor = (*line + 1, 0);
                }
            }
            self.redo_stack.push(op);
            self.ensure_cursor_visible();
        }
    }

    /// Redo
    pub fn redo(&mut self) {
        if let Some(op) = self.redo_stack.pop() {
            match &op {
                EditOp::Insert { line, col, text } => {
                    if let Some(l) = self.lines.get_mut(*line) {
                        l.insert_str(*col, text);
                    }
                    self.cursor = (*line, *col + text.len());
                }
                EditOp::Delete { line, col, text } => {
                    if let Some(l) = self.lines.get_mut(*line) {
                        let end = (*col + text.len()).min(l.len());
                        l.drain(*col..end);
                    }
                    self.cursor = (*line, *col);
                }
                EditOp::SplitLine { line, col } => {
                    if let Some(l) = self.lines.get_mut(*line) {
                        let rest: String = l.drain(*col..).collect();
                        self.lines.insert(*line + 1, rest);
                    }
                    self.cursor = (*line + 1, 0);
                }
                EditOp::MergeLine { line, col } => {
                    if *line + 1 < self.lines.len() {
                        let next = self.lines.remove(*line + 1);
                        if let Some(l) = self.lines.get_mut(*line) {
                            l.push_str(&next);
                        }
                    }
                    self.cursor = (*line, *col);
                }
            }
            self.undo_stack.push(op);
            self.ensure_cursor_visible();
        }
    }

    // =========================================================================
    // Bracket Matching
    // =========================================================================

    /// Find matching bracket at cursor
    pub fn find_matching_bracket(&self) -> Option<BracketMatch> {
        if !self.config.bracket_matching {
            return None;
        }

        let line = self.lines.get(self.cursor.0)?;
        let chars: Vec<char> = line.chars().collect();
        let col = self.cursor.1;

        if col >= chars.len() {
            return None;
        }

        let ch = chars[col];
        let (open, close, forward) = match ch {
            '(' => ('(', ')', true),
            ')' => ('(', ')', false),
            '[' => ('[', ']', true),
            ']' => ('[', ']', false),
            '{' => ('{', '}', true),
            '}' => ('{', '}', false),
            _ => return None,
        };

        if forward {
            self.find_matching_bracket_forward(open, close, self.cursor.0, col)
        } else {
            self.find_matching_bracket_backward(open, close, self.cursor.0, col)
        }
    }

    /// Find matching bracket forward
    fn find_matching_bracket_forward(
        &self,
        open: char,
        close: char,
        start_line: usize,
        start_col: usize,
    ) -> Option<BracketMatch> {
        let mut depth = 1;
        let mut line_idx = start_line;
        let mut col_idx = start_col + 1;

        while line_idx < self.lines.len() {
            let line = &self.lines[line_idx];
            let chars: Vec<char> = line.chars().collect();

            while col_idx < chars.len() {
                let ch = chars[col_idx];
                if ch == open {
                    depth += 1;
                } else if ch == close {
                    depth -= 1;
                    if depth == 0 {
                        return Some(BracketMatch {
                            position: (line_idx, col_idx),
                            char: ch,
                        });
                    }
                }
                col_idx += 1;
            }

            line_idx += 1;
            col_idx = 0;
        }

        None
    }

    /// Find matching bracket backward
    fn find_matching_bracket_backward(
        &self,
        open: char,
        close: char,
        start_line: usize,
        start_col: usize,
    ) -> Option<BracketMatch> {
        let mut depth = 1;
        let mut line_idx = start_line;
        let mut col_idx = start_col;

        loop {
            let line = &self.lines[line_idx];
            let chars: Vec<char> = line.chars().collect();

            while col_idx > 0 {
                col_idx -= 1;
                let ch = chars[col_idx];
                if ch == close {
                    depth += 1;
                } else if ch == open {
                    depth -= 1;
                    if depth == 0 {
                        return Some(BracketMatch {
                            position: (line_idx, col_idx),
                            char: ch,
                        });
                    }
                }
            }

            if line_idx == 0 {
                break;
            }
            line_idx -= 1;
            col_idx = self.lines[line_idx].len();
        }

        None
    }

    // =========================================================================
    // Go-to-Line
    // =========================================================================

    /// Open go-to-line dialog
    pub fn open_goto_line(&mut self) {
        self.goto_line_mode = true;
        self.goto_line_input.clear();
    }

    /// Close go-to-line dialog
    pub fn close_goto_line(&mut self) {
        self.goto_line_mode = false;
        self.goto_line_input.clear();
    }

    /// Check if go-to-line is active
    pub fn is_goto_line_active(&self) -> bool {
        self.goto_line_mode
    }

    /// Go to specific line
    pub fn goto_line(&mut self, line: usize) {
        let target = line
            .saturating_sub(1)
            .min(self.lines.len().saturating_sub(1));
        self.cursor = (target, 0);
        self.clear_selection();
        self.ensure_cursor_visible();
        self.close_goto_line();
    }

    /// Handle go-to-line input
    fn handle_goto_input(&mut self, key: &Key) -> bool {
        match key {
            Key::Char(ch) if ch.is_ascii_digit() => {
                self.goto_line_input.push(*ch);
                true
            }
            Key::Backspace => {
                self.goto_line_input.pop();
                true
            }
            Key::Enter => {
                if let Ok(line) = self.goto_line_input.parse::<usize>() {
                    self.goto_line(line);
                }
                self.close_goto_line();
                true
            }
            Key::Escape => {
                self.close_goto_line();
                true
            }
            _ => false,
        }
    }

    // =========================================================================
    // Find
    // =========================================================================

    /// Open find dialog
    pub fn open_find(&mut self) {
        self.find_mode = true;
        self.find_query.clear();
        self.find_matches.clear();
        self.find_index = 0;
    }

    /// Close find dialog
    pub fn close_find(&mut self) {
        self.find_mode = false;
    }

    /// Check if find is active
    pub fn is_find_active(&self) -> bool {
        self.find_mode
    }

    /// Set find query
    pub fn set_find_query(&mut self, query: &str) {
        self.find_query = query.to_string();
        self.refresh_find_matches();
    }

    /// Refresh find matches
    fn refresh_find_matches(&mut self) {
        self.find_matches.clear();

        if self.find_query.is_empty() {
            return;
        }

        let query_lower = self.find_query.to_lowercase();
        for (line_idx, line) in self.lines.iter().enumerate() {
            let line_lower = line.to_lowercase();
            let mut start = 0;
            while let Some(pos) = line_lower[start..].find(&query_lower) {
                let match_start = start + pos;
                let match_end = match_start + self.find_query.len();
                self.find_matches.push((line_idx, match_start, match_end));
                start = match_start + 1;
            }
        }

        if !self.find_matches.is_empty() && self.find_index >= self.find_matches.len() {
            self.find_index = 0;
        }
    }

    /// Find next match
    pub fn find_next(&mut self) {
        if self.find_matches.is_empty() {
            return;
        }

        self.find_index = (self.find_index + 1) % self.find_matches.len();
        self.jump_to_current_match();
    }

    /// Find previous match
    pub fn find_previous(&mut self) {
        if self.find_matches.is_empty() {
            return;
        }

        self.find_index = if self.find_index == 0 {
            self.find_matches.len() - 1
        } else {
            self.find_index - 1
        };
        self.jump_to_current_match();
    }

    /// Jump to current match
    fn jump_to_current_match(&mut self) {
        if let Some(&(line, col, _)) = self.find_matches.get(self.find_index) {
            self.cursor = (line, col);
            self.ensure_cursor_visible();
        }
    }

    /// Get find match count
    pub fn find_match_count(&self) -> usize {
        self.find_matches.len()
    }

    /// Get current find index (1-based)
    pub fn current_find_index(&self) -> usize {
        if self.find_matches.is_empty() {
            0
        } else {
            self.find_index + 1
        }
    }

    /// Handle find input
    fn handle_find_input(&mut self, key: &Key) -> bool {
        match key {
            Key::Char(ch) => {
                self.find_query.push(*ch);
                self.refresh_find_matches();
                if !self.find_matches.is_empty() {
                    self.jump_to_current_match();
                }
                true
            }
            Key::Backspace => {
                self.find_query.pop();
                self.refresh_find_matches();
                true
            }
            Key::Enter => {
                self.find_next();
                true
            }
            Key::Escape => {
                self.close_find();
                true
            }
            _ => false,
        }
    }

    // =========================================================================
    // Key Handling
    // =========================================================================

    /// Handle key event
    pub fn handle_key(&mut self, key: &Key) -> bool {
        // Handle modal modes first
        if self.goto_line_mode {
            return self.handle_goto_input(key);
        }

        if self.find_mode {
            return self.handle_find_input(key);
        }

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
                self.move_left();
                true
            }
            Key::Right => {
                self.move_right();
                true
            }
            Key::Up => {
                self.move_up();
                true
            }
            Key::Down => {
                self.move_down();
                true
            }
            Key::Home => {
                self.move_home();
                true
            }
            Key::End => {
                self.move_end();
                true
            }
            Key::PageUp => {
                self.page_up(20);
                true
            }
            Key::PageDown => {
                self.page_down(20);
                true
            }
            _ => false,
        }
    }

    // =========================================================================
    // Rendering Helpers
    // =========================================================================

    /// Get line number width
    fn line_number_width(&self) -> u16 {
        if self.show_line_numbers {
            let digits = format!("{}", self.lines.len()).len();
            (digits + 2) as u16
        } else {
            0
        }
    }

    /// Get syntax highlights for a line
    fn get_highlights(&self, line: &str) -> Vec<HighlightSpan> {
        self.highlighter
            .as_ref()
            .map(|h| h.highlight_line(line))
            .unwrap_or_default()
    }

    /// Check if position is in selection
    fn is_selected(&self, line: usize, col: usize) -> bool {
        let anchor = match self.anchor {
            Some(a) => a,
            None => return false,
        };

        let (start, end) = if anchor < self.cursor {
            (anchor, self.cursor)
        } else {
            (self.cursor, anchor)
        };

        if line < start.0 || line > end.0 {
            return false;
        }
        if line == start.0 && line == end.0 {
            col >= start.1 && col < end.1
        } else if line == start.0 {
            col >= start.1
        } else if line == end.0 {
            col < end.1
        } else {
            true
        }
    }

    /// Check if position is in a find match
    fn get_find_match_at(&self, line: usize, col: usize) -> Option<(bool, usize)> {
        for (idx, &(m_line, m_start, m_end)) in self.find_matches.iter().enumerate() {
            if m_line == line && col >= m_start && col < m_end {
                return Some((idx == self.find_index, idx));
            }
        }
        None
    }
}

impl Default for CodeEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl View for CodeEditor {
    crate::impl_view_meta!("CodeEditor");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 {
            return;
        }

        let line_num_width = self.line_number_width();
        let minimap_width = if self.config.show_minimap {
            self.config.minimap_width
        } else {
            0
        };
        let text_width = area.width.saturating_sub(line_num_width + minimap_width);
        let visible_lines = area.height as usize;

        // Find matching bracket
        let bracket_match = self.find_matching_bracket();

        // Draw background
        if let Some(bg) = self.bg {
            for y in 0..area.height {
                for x in 0..area.width {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(bg);
                    ctx.buffer.set(area.x + x, area.y + y, cell);
                }
            }
        }

        // Render visible lines
        let start_line = self.scroll.0;
        let end_line = (start_line + visible_lines).min(self.lines.len());

        for (view_row, line_idx) in (start_line..end_line).enumerate() {
            let y = area.y + view_row as u16;
            let line = &self.lines[line_idx];
            let is_current_line = line_idx == self.cursor.0;

            // Current line highlight
            if self.config.highlight_current_line && is_current_line && self.focused {
                for x in line_num_width..line_num_width + text_width {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(self.current_line_bg);
                    ctx.buffer.set(area.x + x, y, cell);
                }
            }

            // Draw line numbers
            if self.show_line_numbers {
                let num_str = format!(
                    "{:>width$} ",
                    line_idx + 1,
                    width = (line_num_width - 2) as usize
                );
                for (i, ch) in num_str.chars().enumerate() {
                    if (i as u16) < line_num_width {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(if is_current_line && self.focused {
                            Color::WHITE
                        } else {
                            self.line_number_fg
                        });
                        cell.bg = self.bg;
                        ctx.buffer.set(area.x + i as u16, y, cell);
                    }
                }
            }

            // Get syntax highlights
            let highlights = self.get_highlights(line);

            // Draw text
            let chars: Vec<char> = line.chars().collect();
            let scroll_col = self.scroll.1;

            for (view_col, char_idx) in (scroll_col..scroll_col + text_width as usize).enumerate() {
                let x = area.x + line_num_width + view_col as u16;
                if x >= area.x + area.width - minimap_width {
                    break;
                }

                let ch = chars.get(char_idx).copied().unwrap_or(' ');
                let mut cell = Cell::new(ch);

                // Check cursor position
                let is_cursor =
                    self.focused && line_idx == self.cursor.0 && char_idx == self.cursor.1;

                // Check selection
                let is_selected = self.is_selected(line_idx, char_idx);

                // Check bracket match
                let is_bracket_match = bracket_match
                    .as_ref()
                    .map(|m| m.position == (line_idx, char_idx))
                    .unwrap_or(false);

                // Check find match
                let find_match = self.get_find_match_at(line_idx, char_idx);

                if is_cursor {
                    cell.bg = Some(self.cursor_bg);
                    cell.fg = Some(Color::BLACK);
                } else if is_selected {
                    cell.bg = Some(self.selection_bg);
                    cell.fg = self.fg;
                } else if is_bracket_match {
                    cell.bg = Some(self.bracket_match_bg);
                    cell.fg = Some(Color::BLACK);
                    cell.modifier |= Modifier::BOLD;
                } else if let Some((is_current, _)) = find_match {
                    cell.bg = Some(if is_current {
                        self.current_find_bg
                    } else {
                        self.find_match_bg
                    });
                    cell.fg = Some(Color::BLACK);
                } else {
                    // Apply syntax highlighting
                    let mut fg_set = false;
                    for span in &highlights {
                        if char_idx >= span.start && char_idx < span.end {
                            cell.fg = Some(span.fg);
                            if span.bold {
                                cell.modifier |= Modifier::BOLD;
                            }
                            if span.italic {
                                cell.modifier |= Modifier::ITALIC;
                            }
                            fg_set = true;
                            break;
                        }
                    }
                    if !fg_set {
                        cell.fg = self.fg;
                    }
                    if self.config.highlight_current_line && is_current_line && self.focused {
                        cell.bg = Some(self.current_line_bg);
                    } else {
                        cell.bg = self.bg;
                    }
                }

                ctx.buffer.set(x, y, cell);
            }

            // Draw cursor at end of line if needed
            if self.focused && line_idx == self.cursor.0 && self.cursor.1 >= chars.len() {
                let cursor_x = area.x + line_num_width + (self.cursor.1 - scroll_col) as u16;
                if cursor_x < area.x + area.width - minimap_width {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(self.cursor_bg);
                    ctx.buffer.set(cursor_x, y, cell);
                }
            }
        }

        // Draw minimap if enabled
        if self.config.show_minimap && minimap_width > 0 {
            let minimap_x = area.x + area.width - minimap_width;

            // Minimap background
            for y in 0..area.height {
                for x in 0..minimap_width {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(self.minimap_bg);
                    ctx.buffer.set(minimap_x + x, area.y + y, cell);
                }
            }

            // Calculate minimap scale
            let total_lines = self.lines.len();
            let minimap_height = area.height as usize;
            let lines_per_row = (total_lines as f32 / minimap_height as f32).max(1.0);

            // Draw minimap content
            for row in 0..minimap_height {
                let start_line = (row as f32 * lines_per_row) as usize;
                let y = area.y + row as u16;

                // Highlight visible area
                if start_line >= self.scroll.0 && start_line < self.scroll.0 + visible_lines {
                    for x in 0..minimap_width {
                        let mut cell = Cell::new(' ');
                        cell.bg = Some(self.minimap_visible_bg);
                        ctx.buffer.set(minimap_x + x, y, cell);
                    }
                }

                // Draw condensed line representation
                if let Some(line) = self.lines.get(start_line) {
                    let chars: Vec<char> = line.chars().collect();
                    for (i, &ch) in chars.iter().take(minimap_width as usize).enumerate() {
                        if !ch.is_whitespace() {
                            let mut cell = Cell::new('▪');
                            cell.fg = Some(Color::rgb(128, 128, 128));
                            ctx.buffer.set(minimap_x + i as u16, y, cell);
                        }
                    }
                }
            }
        }

        // Draw go-to-line dialog
        if self.goto_line_mode {
            let dialog_width = 20u16;
            let dialog_x = area.x + (area.width.saturating_sub(dialog_width)) / 2;
            let dialog_y = area.y;

            // Background
            for x in 0..dialog_width {
                let mut cell = Cell::new(' ');
                cell.bg = Some(Color::rgb(49, 50, 68));
                ctx.buffer.set(dialog_x + x, dialog_y, cell);
            }

            // Label
            let label = "Go to line: ";
            for (i, ch) in label.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::WHITE);
                cell.bg = Some(Color::rgb(49, 50, 68));
                ctx.buffer.set(dialog_x + i as u16, dialog_y, cell);
            }

            // Input
            for (i, ch) in self.goto_line_input.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::rgb(166, 227, 161));
                cell.bg = Some(Color::rgb(49, 50, 68));
                ctx.buffer
                    .set(dialog_x + label.len() as u16 + i as u16, dialog_y, cell);
            }
        }

        // Draw find dialog
        if self.find_mode {
            let dialog_width = 30u16;
            let dialog_x = area.x + (area.width.saturating_sub(dialog_width)) / 2;
            let dialog_y = area.y;

            // Background
            for x in 0..dialog_width {
                let mut cell = Cell::new(' ');
                cell.bg = Some(Color::rgb(49, 50, 68));
                ctx.buffer.set(dialog_x + x, dialog_y, cell);
            }

            // Label
            let label = "Find: ";
            for (i, ch) in label.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::WHITE);
                cell.bg = Some(Color::rgb(49, 50, 68));
                ctx.buffer.set(dialog_x + i as u16, dialog_y, cell);
            }

            // Query
            for (i, ch) in self.find_query.chars().enumerate() {
                if (label.len() + i) < dialog_width as usize - 8 {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(Color::rgb(166, 227, 161));
                    cell.bg = Some(Color::rgb(49, 50, 68));
                    ctx.buffer
                        .set(dialog_x + label.len() as u16 + i as u16, dialog_y, cell);
                }
            }

            // Match count
            let count = format!(" {}/{}", self.current_find_index(), self.find_match_count());
            let count_x = dialog_x + dialog_width - count.len() as u16;
            for (i, ch) in count.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::rgb(180, 190, 254));
                cell.bg = Some(Color::rgb(49, 50, 68));
                ctx.buffer.set(count_x + i as u16, dialog_y, cell);
            }
        }
    }
}

impl_styled_view!(CodeEditor);
impl_props_builders!(CodeEditor);

/// Create a new code editor
pub fn code_editor() -> CodeEditor {
    CodeEditor::new()
}

#[cfg(test)]
mod tests {
    use super::*;

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
