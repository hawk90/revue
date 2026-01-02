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

use super::syntax::{Language, SyntaxHighlighter, SyntaxTheme};
use super::traits::{RenderContext, View, WidgetProps};
use crate::event::Key;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::{impl_props_builders, impl_styled_view};

/// Maximum undo history size
const MAX_UNDO_HISTORY: usize = 100;

/// A text selection range
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Selection {
    /// Start position (line, column)
    pub start: (usize, usize),
    /// End position (line, column)
    pub end: (usize, usize),
}

impl Selection {
    /// Create a new selection
    pub fn new(start: (usize, usize), end: (usize, usize)) -> Self {
        Self { start, end }
    }

    /// Get the normalized selection (start before end)
    pub fn normalized(&self) -> Self {
        if self.start.0 > self.end.0 || (self.start.0 == self.end.0 && self.start.1 > self.end.1) {
            Self {
                start: self.end,
                end: self.start,
            }
        } else {
            *self
        }
    }

    /// Check if a position is within the selection
    pub fn contains(&self, line: usize, col: usize) -> bool {
        let norm = self.normalized();
        if line < norm.start.0 || line > norm.end.0 {
            return false;
        }
        if line == norm.start.0 && line == norm.end.0 {
            col >= norm.start.1 && col < norm.end.1
        } else if line == norm.start.0 {
            col >= norm.start.1
        } else if line == norm.end.0 {
            col < norm.end.1
        } else {
            true
        }
    }
}

/// An edit operation for undo/redo
#[derive(Clone, Debug)]
enum EditOperation {
    /// Insert text at position
    Insert {
        line: usize,
        col: usize,
        text: String,
    },
    /// Delete text at position
    Delete {
        line: usize,
        col: usize,
        text: String,
    },
    /// Insert a new line
    InsertLine { line: usize, content: String },
    /// Delete a line
    DeleteLine { line: usize, content: String },
    /// Merge with previous line
    MergeLines { line: usize, col: usize },
    /// Split line at position
    SplitLine { line: usize, col: usize },
}

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
    lines: Vec<String>,
    /// Cursor position (line, column)
    cursor: (usize, usize),
    /// Scroll offset (line, column)
    scroll: (usize, usize),
    /// Current selection
    selection: Option<Selection>,
    /// Whether selecting text
    selecting: bool,
    /// Undo history
    undo_stack: Vec<EditOperation>,
    /// Redo history
    redo_stack: Vec<EditOperation>,
    /// Show line numbers
    show_line_numbers: bool,
    /// Enable word wrap
    wrap: bool,
    /// Read-only mode
    read_only: bool,
    /// Focused state
    focused: bool,
    /// Tab width
    tab_width: usize,
    /// Placeholder text
    placeholder: Option<String>,
    /// Maximum lines (0 = unlimited)
    max_lines: usize,
    /// Text color
    fg: Option<Color>,
    /// Background color
    bg: Option<Color>,
    /// Cursor color
    cursor_fg: Option<Color>,
    /// Selection color
    selection_bg: Option<Color>,
    /// Line number color
    line_number_fg: Option<Color>,
    /// Syntax highlighter for code coloring
    highlighter: Option<SyntaxHighlighter>,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
}

impl TextArea {
    /// Create a new empty text area
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()],
            cursor: (0, 0),
            scroll: (0, 0),
            selection: None,
            selecting: false,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            show_line_numbers: false,
            wrap: false,
            read_only: false,
            focused: true,
            tab_width: 4,
            placeholder: None,
            max_lines: 0,
            fg: None,
            bg: None,
            cursor_fg: Some(Color::BLACK),
            selection_bg: Some(Color::rgb(70, 130, 180)),
            line_number_fg: Some(Color::rgb(128, 128, 128)),
            highlighter: None,
            props: WidgetProps::new(),
        }
    }

    /// Set initial content
    pub fn content(mut self, text: impl Into<String>) -> Self {
        let text = text.into();
        self.lines = text.lines().map(String::from).collect();
        if self.lines.is_empty() {
            self.lines.push(String::new());
        }
        self
    }

    /// Enable or disable line numbers
    pub fn line_numbers(mut self, show: bool) -> Self {
        self.show_line_numbers = show;
        self
    }

    /// Enable or disable word wrap
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
        self.tab_width = width.max(1);
        self
    }

    /// Set placeholder text
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(text.into());
        self
    }

    /// Set maximum number of lines
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

    /// Set the syntax highlighting language (mutable)
    pub fn set_language(&mut self, language: Language) {
        if language == Language::None {
            self.highlighter = None;
        } else {
            self.highlighter = Some(SyntaxHighlighter::new(language));
        }
    }

    /// Get the current highlighting language
    pub fn get_syntax_language(&self) -> Language {
        self.highlighter
            .as_ref()
            .map(|h| h.get_language())
            .unwrap_or(Language::None)
    }

    /// Get the current text content
    pub fn get_content(&self) -> String {
        self.lines.join("\n")
    }

    /// Set the text content
    pub fn set_content(&mut self, text: &str) {
        self.lines = text.lines().map(String::from).collect();
        if self.lines.is_empty() {
            self.lines.push(String::new());
        }
        self.cursor = (0, 0);
        self.scroll = (0, 0);
        self.selection = None;
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// Get the number of lines
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    /// Get the cursor position
    pub fn cursor_position(&self) -> (usize, usize) {
        self.cursor
    }

    /// Set the cursor position
    pub fn set_cursor(&mut self, line: usize, col: usize) {
        self.cursor.0 = line.min(self.lines.len().saturating_sub(1));
        self.cursor.1 = col.min(self.current_line_len());
    }

    /// Get current line length
    fn current_line_len(&self) -> usize {
        self.lines.get(self.cursor.0).map(|l| l.len()).unwrap_or(0)
    }

    /// Get selected text
    pub fn get_selection(&self) -> Option<String> {
        let sel = self.selection?.normalized();
        let mut result = String::new();

        for line_idx in sel.start.0..=sel.end.0 {
            if line_idx >= self.lines.len() {
                break;
            }
            let line = &self.lines[line_idx];
            let start_col = if line_idx == sel.start.0 {
                sel.start.1
            } else {
                0
            };
            let end_col = if line_idx == sel.end.0 {
                sel.end.1
            } else {
                line.len()
            };

            if start_col < line.len() {
                result.push_str(&line[start_col..end_col.min(line.len())]);
            }
            if line_idx < sel.end.0 {
                result.push('\n');
            }
        }

        Some(result)
    }

    /// Delete selected text
    pub fn delete_selection(&mut self) {
        if let Some(sel) = self.selection.take() {
            let sel = sel.normalized();

            if sel.start.0 == sel.end.0 {
                // Single line selection
                if let Some(line) = self.lines.get_mut(sel.start.0) {
                    let deleted: String =
                        line.drain(sel.start.1..sel.end.1.min(line.len())).collect();
                    self.push_undo(EditOperation::Delete {
                        line: sel.start.0,
                        col: sel.start.1,
                        text: deleted,
                    });
                }
            } else {
                // Multi-line selection
                // Get the content before and after selection
                let before: String = self.lines[sel.start.0].chars().take(sel.start.1).collect();
                let after: String = self.lines[sel.end.0].chars().skip(sel.end.1).collect();

                // Remove lines between start and end
                for _ in sel.start.0..=sel.end.0 {
                    if sel.start.0 < self.lines.len() {
                        self.lines.remove(sel.start.0);
                    }
                }

                // Insert merged line
                self.lines
                    .insert(sel.start.0, format!("{}{}", before, after));
            }

            self.cursor = sel.start;
            self.selection = None;
        }
    }

    /// Start selection at current cursor
    pub fn start_selection(&mut self) {
        self.selecting = true;
        self.selection = Some(Selection::new(self.cursor, self.cursor));
    }

    /// Update selection to current cursor
    fn update_selection(&mut self) {
        if self.selecting {
            if let Some(ref mut sel) = self.selection {
                sel.end = self.cursor;
            }
        }
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.selection = None;
        self.selecting = false;
    }

    /// Push an operation to the undo stack
    fn push_undo(&mut self, op: EditOperation) {
        self.undo_stack.push(op);
        if self.undo_stack.len() > MAX_UNDO_HISTORY {
            self.undo_stack.remove(0);
        }
        self.redo_stack.clear();
    }

    /// Undo the last operation
    pub fn undo(&mut self) {
        if let Some(op) = self.undo_stack.pop() {
            match &op {
                EditOperation::Insert { line, col, text } => {
                    if let Some(l) = self.lines.get_mut(*line) {
                        let end = (*col + text.len()).min(l.len());
                        l.drain(*col..end);
                    }
                    self.cursor = (*line, *col);
                }
                EditOperation::Delete { line, col, text } => {
                    if let Some(l) = self.lines.get_mut(*line) {
                        l.insert_str(*col, text);
                    }
                    self.cursor = (*line, *col + text.len());
                }
                EditOperation::InsertLine { line, .. } => {
                    if *line < self.lines.len() {
                        self.lines.remove(*line);
                    }
                    self.cursor = (line.saturating_sub(1), 0);
                }
                EditOperation::DeleteLine { line, content } => {
                    self.lines.insert(*line, content.clone());
                    self.cursor = (*line, 0);
                }
                EditOperation::SplitLine { line, col } => {
                    // Merge lines back
                    if *line + 1 < self.lines.len() {
                        let next = self.lines.remove(*line + 1);
                        if let Some(l) = self.lines.get_mut(*line) {
                            l.push_str(&next);
                        }
                    }
                    self.cursor = (*line, *col);
                }
                EditOperation::MergeLines { line, col } => {
                    // Split line again
                    if let Some(l) = self.lines.get_mut(*line) {
                        let rest: String = l.drain(*col..).collect();
                        self.lines.insert(*line + 1, rest);
                    }
                    self.cursor = (*line + 1, 0);
                }
            }
            self.redo_stack.push(op);
        }
    }

    /// Redo the last undone operation
    pub fn redo(&mut self) {
        if let Some(op) = self.redo_stack.pop() {
            match &op {
                EditOperation::Insert { line, col, text } => {
                    if let Some(l) = self.lines.get_mut(*line) {
                        l.insert_str(*col, text);
                    }
                    self.cursor = (*line, *col + text.len());
                }
                EditOperation::Delete { line, col, text } => {
                    if let Some(l) = self.lines.get_mut(*line) {
                        let end = (*col + text.len()).min(l.len());
                        l.drain(*col..end);
                    }
                    self.cursor = (*line, *col);
                }
                EditOperation::InsertLine { line, content } => {
                    self.lines.insert(*line, content.clone());
                    self.cursor = (*line, 0);
                }
                EditOperation::DeleteLine { line, .. } => {
                    if *line < self.lines.len() {
                        self.lines.remove(*line);
                    }
                    self.cursor = (*line.min(&self.lines.len().saturating_sub(1)), 0);
                }
                EditOperation::SplitLine { line, col } => {
                    if let Some(l) = self.lines.get_mut(*line) {
                        let rest: String = l.drain(*col..).collect();
                        self.lines.insert(*line + 1, rest);
                    }
                    self.cursor = (*line + 1, 0);
                }
                EditOperation::MergeLines { line, col } => {
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
        }
    }

    /// Insert a character at cursor
    pub fn insert_char(&mut self, ch: char) {
        if self.read_only {
            return;
        }

        self.delete_selection();

        if ch == '\n' {
            self.insert_newline();
            return;
        }

        if ch == '\t' {
            // Insert spaces for tab
            let spaces = " ".repeat(self.tab_width);
            self.insert_str(&spaces);
            return;
        }

        if let Some(line) = self.lines.get_mut(self.cursor.0) {
            let col = self.cursor.1.min(line.len());
            line.insert(col, ch);
            self.push_undo(EditOperation::Insert {
                line: self.cursor.0,
                col,
                text: ch.to_string(),
            });
            self.cursor.1 = col + 1;
        }
    }

    /// Insert a string at cursor
    pub fn insert_str(&mut self, s: &str) {
        if self.read_only {
            return;
        }

        self.delete_selection();

        // Handle multi-line inserts
        let parts: Vec<&str> = s.split('\n').collect();
        if parts.len() == 1 {
            // Single line insert
            if let Some(line) = self.lines.get_mut(self.cursor.0) {
                let col = self.cursor.1.min(line.len());
                line.insert_str(col, s);
                self.push_undo(EditOperation::Insert {
                    line: self.cursor.0,
                    col,
                    text: s.to_string(),
                });
                self.cursor.1 = col + s.len();
            }
        } else {
            // Multi-line insert
            for (i, part) in parts.iter().enumerate() {
                if i == 0 {
                    if let Some(line) = self.lines.get_mut(self.cursor.0) {
                        line.insert_str(self.cursor.1, part);
                    }
                    self.cursor.1 += part.len();
                } else {
                    self.insert_newline();
                    if let Some(line) = self.lines.get_mut(self.cursor.0) {
                        line.insert_str(0, part);
                    }
                    self.cursor.1 = part.len();
                }
            }
        }
    }

    /// Insert a newline at cursor
    fn insert_newline(&mut self) {
        if self.read_only {
            return;
        }

        if self.max_lines > 0 && self.lines.len() >= self.max_lines {
            return;
        }

        let (line, col) = self.cursor;
        if let Some(current) = self.lines.get_mut(line) {
            let rest: String = current.drain(col.min(current.len())..).collect();
            self.lines.insert(line + 1, rest);
            self.push_undo(EditOperation::SplitLine { line, col });
            self.cursor = (line + 1, 0);
        }
    }

    /// Delete character before cursor (backspace)
    pub fn delete_char_before(&mut self) {
        if self.read_only {
            return;
        }

        if self.selection.is_some() {
            self.delete_selection();
            return;
        }

        let (line, col) = self.cursor;
        if col > 0 {
            // Delete character in current line
            if let Some(l) = self.lines.get_mut(line) {
                if col <= l.len() {
                    let deleted = l.remove(col - 1);
                    self.push_undo(EditOperation::Delete {
                        line,
                        col: col - 1,
                        text: deleted.to_string(),
                    });
                    self.cursor.1 = col - 1;
                }
            }
        } else if line > 0 {
            // Merge with previous line
            let current = self.lines.remove(line);
            let prev_len = self.lines[line - 1].len();
            self.lines[line - 1].push_str(&current);
            self.push_undo(EditOperation::MergeLines {
                line: line - 1,
                col: prev_len,
            });
            self.cursor = (line - 1, prev_len);
        }
    }

    /// Delete character at cursor (delete key)
    pub fn delete_char_at(&mut self) {
        if self.read_only {
            return;
        }

        if self.selection.is_some() {
            self.delete_selection();
            return;
        }

        let (line, col) = self.cursor;
        if let Some(l) = self.lines.get_mut(line) {
            if col < l.len() {
                let deleted = l.remove(col);
                self.push_undo(EditOperation::Delete {
                    line,
                    col,
                    text: deleted.to_string(),
                });
            } else if line + 1 < self.lines.len() {
                // Merge with next line
                let next = self.lines.remove(line + 1);
                self.lines[line].push_str(&next);
                self.push_undo(EditOperation::MergeLines { line, col });
            }
        }
    }

    /// Delete the current line
    pub fn delete_line(&mut self) {
        if self.read_only || self.lines.len() <= 1 {
            return;
        }

        let line = self.cursor.0;
        let content = self.lines.remove(line);
        self.push_undo(EditOperation::DeleteLine { line, content });

        if self.cursor.0 >= self.lines.len() {
            self.cursor.0 = self.lines.len() - 1;
        }
        self.cursor.1 = 0;
    }

    /// Duplicate the current line
    pub fn duplicate_line(&mut self) {
        if self.read_only {
            return;
        }

        let line = self.cursor.0;
        let content = self.lines[line].clone();
        self.lines.insert(line + 1, content.clone());
        self.push_undo(EditOperation::InsertLine {
            line: line + 1,
            content,
        });
        self.cursor.0 += 1;
    }

    /// Move cursor left
    pub fn move_left(&mut self) {
        if self.cursor.1 > 0 {
            self.cursor.1 -= 1;
        } else if self.cursor.0 > 0 {
            self.cursor.0 -= 1;
            self.cursor.1 = self.current_line_len();
        }
        self.update_selection();
    }

    /// Move cursor right
    pub fn move_right(&mut self) {
        let line_len = self.current_line_len();
        if self.cursor.1 < line_len {
            self.cursor.1 += 1;
        } else if self.cursor.0 + 1 < self.lines.len() {
            self.cursor.0 += 1;
            self.cursor.1 = 0;
        }
        self.update_selection();
    }

    /// Move cursor up
    pub fn move_up(&mut self) {
        if self.cursor.0 > 0 {
            self.cursor.0 -= 1;
            self.cursor.1 = self.cursor.1.min(self.current_line_len());
        }
        self.update_selection();
    }

    /// Move cursor down
    pub fn move_down(&mut self) {
        if self.cursor.0 + 1 < self.lines.len() {
            self.cursor.0 += 1;
            self.cursor.1 = self.cursor.1.min(self.current_line_len());
        }
        self.update_selection();
    }

    /// Move to start of line
    pub fn move_home(&mut self) {
        self.cursor.1 = 0;
        self.update_selection();
    }

    /// Move to end of line
    pub fn move_end(&mut self) {
        self.cursor.1 = self.current_line_len();
        self.update_selection();
    }

    /// Move to start of document
    pub fn move_document_start(&mut self) {
        self.cursor = (0, 0);
        self.update_selection();
    }

    /// Move to end of document
    pub fn move_document_end(&mut self) {
        self.cursor.0 = self.lines.len().saturating_sub(1);
        self.cursor.1 = self.current_line_len();
        self.update_selection();
    }

    /// Move cursor by word to the left
    pub fn move_word_left(&mut self) {
        if self.cursor.1 == 0 {
            if self.cursor.0 > 0 {
                self.cursor.0 -= 1;
                self.cursor.1 = self.current_line_len();
            }
            return;
        }

        let line = &self.lines[self.cursor.0];
        let chars: Vec<char> = line.chars().collect();
        let mut col = self.cursor.1.min(chars.len());

        // Skip spaces
        while col > 0 && chars[col - 1].is_whitespace() {
            col -= 1;
        }
        // Skip word
        while col > 0 && !chars[col - 1].is_whitespace() {
            col -= 1;
        }

        self.cursor.1 = col;
        self.update_selection();
    }

    /// Move cursor by word to the right
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
        // Skip spaces
        while col < chars.len() && chars[col].is_whitespace() {
            col += 1;
        }

        self.cursor.1 = col;
        self.update_selection();
    }

    /// Page up
    pub fn page_up(&mut self, page_size: usize) {
        self.cursor.0 = self.cursor.0.saturating_sub(page_size);
        self.cursor.1 = self.cursor.1.min(self.current_line_len());
        self.update_selection();
    }

    /// Page down
    pub fn page_down(&mut self, page_size: usize) {
        self.cursor.0 = (self.cursor.0 + page_size).min(self.lines.len().saturating_sub(1));
        self.cursor.1 = self.cursor.1.min(self.current_line_len());
        self.update_selection();
    }

    /// Select all text
    pub fn select_all(&mut self) {
        let last_line = self.lines.len().saturating_sub(1);
        let last_col = self.lines.last().map(|l| l.len()).unwrap_or(0);
        self.selection = Some(Selection::new((0, 0), (last_line, last_col)));
    }

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

    /// Get line number width
    fn line_number_width(&self) -> u16 {
        if self.show_line_numbers {
            let max_line = self.lines.len();
            let digits = format!("{}", max_line).len();
            (digits + 2) as u16 // digits + space + separator
        } else {
            0
        }
    }
}

impl Default for TextArea {
    fn default() -> Self {
        Self::new()
    }
}

impl View for TextArea {
    crate::impl_view_meta!("TextArea");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 {
            return;
        }

        let line_num_width = self.line_number_width();
        let text_start_x = line_num_width;
        let text_width = area.width.saturating_sub(line_num_width);
        let visible_lines = area.height as usize;

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

        // Show placeholder if empty
        if self.lines.len() == 1 && self.lines[0].is_empty() {
            if let Some(ref placeholder) = self.placeholder {
                for (i, ch) in placeholder.chars().enumerate() {
                    if (i as u16) < text_width {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(Color::rgb(128, 128, 128));
                        cell.modifier = Modifier::ITALIC;
                        ctx.buffer
                            .set(area.x + text_start_x + i as u16, area.y, cell);
                    }
                }
            }
        }

        // Render visible lines
        for (view_row, line_idx) in (self.scroll.0..self.scroll.0 + visible_lines).enumerate() {
            if line_idx >= self.lines.len() {
                break;
            }

            let y = area.y + view_row as u16;

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
                        cell.fg = self.line_number_fg;
                        ctx.buffer.set(area.x + i as u16, y, cell);
                    }
                }
            }

            // Draw text
            let line = &self.lines[line_idx];
            let chars: Vec<char> = line.chars().collect();
            let scroll_col = if self.wrap { 0 } else { self.scroll.1 };

            // Get syntax highlighting spans for this line
            let highlights = self.highlighter.as_ref().map(|h| h.highlight_line(line));

            for (view_col, char_idx) in (scroll_col..scroll_col + text_width as usize).enumerate() {
                let x = area.x + text_start_x + view_col as u16;
                if x >= area.x + area.width {
                    break;
                }

                let ch = chars.get(char_idx).copied().unwrap_or(' ');
                let mut cell = Cell::new(ch);

                // Check if this position is selected
                let is_selected = self
                    .selection
                    .map(|s| s.contains(line_idx, char_idx))
                    .unwrap_or(false);

                // Check if this is cursor position
                let is_cursor =
                    self.focused && line_idx == self.cursor.0 && char_idx == self.cursor.1;

                if is_cursor {
                    cell.fg = self.cursor_fg;
                    cell.bg = Some(Color::WHITE);
                    cell.modifier = Modifier::BOLD;
                } else if is_selected {
                    cell.fg = Some(Color::WHITE);
                    cell.bg = self.selection_bg;
                } else {
                    // Apply syntax highlighting if available
                    let mut highlight_applied = false;
                    if let Some(ref spans) = highlights {
                        for span in spans {
                            if char_idx >= span.start && char_idx < span.end {
                                cell.fg = Some(span.fg);
                                if span.bold {
                                    cell.modifier |= Modifier::BOLD;
                                }
                                if span.italic {
                                    cell.modifier |= Modifier::ITALIC;
                                }
                                highlight_applied = true;
                                break;
                            }
                        }
                    }
                    if !highlight_applied {
                        cell.fg = self.fg;
                    }
                    cell.bg = self.bg;
                }

                ctx.buffer.set(x, y, cell);
            }

            // Draw cursor at end of line if needed
            if self.focused && line_idx == self.cursor.0 && self.cursor.1 >= chars.len() {
                let cursor_x = area.x + text_start_x + (self.cursor.1 - scroll_col) as u16;
                if cursor_x < area.x + area.width {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(Color::WHITE);
                    ctx.buffer.set(cursor_x, y, cell);
                }
            }
        }
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
    use super::super::syntax::{Language, SyntaxTheme};
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_textarea_new() {
        let ta = TextArea::new();
        assert_eq!(ta.lines.len(), 1);
        assert_eq!(ta.lines[0], "");
    }

    #[test]
    fn test_textarea_content() {
        let ta = TextArea::new().content("Hello\nWorld");
        assert_eq!(ta.lines.len(), 2);
        assert_eq!(ta.lines[0], "Hello");
        assert_eq!(ta.lines[1], "World");
    }

    #[test]
    fn test_textarea_insert_char() {
        let mut ta = TextArea::new();
        ta.insert_char('H');
        ta.insert_char('i');
        assert_eq!(ta.get_content(), "Hi");
        assert_eq!(ta.cursor, (0, 2));
    }

    #[test]
    fn test_textarea_insert_newline() {
        let mut ta = TextArea::new().content("Hello");
        ta.cursor = (0, 5);
        ta.insert_char('\n');
        assert_eq!(ta.lines.len(), 2);
        assert_eq!(ta.cursor, (1, 0));
    }

    #[test]
    fn test_textarea_delete_char_before() {
        let mut ta = TextArea::new().content("Hello");
        ta.cursor = (0, 3);
        ta.delete_char_before();
        assert_eq!(ta.get_content(), "Helo");
        assert_eq!(ta.cursor, (0, 2));
    }

    #[test]
    fn test_textarea_delete_char_at() {
        let mut ta = TextArea::new().content("Hello");
        ta.cursor = (0, 2);
        ta.delete_char_at();
        assert_eq!(ta.get_content(), "Helo");
    }

    #[test]
    fn test_textarea_movement() {
        let mut ta = TextArea::new().content("Hello\nWorld");
        ta.cursor = (0, 2);

        ta.move_right();
        assert_eq!(ta.cursor, (0, 3));

        ta.move_left();
        assert_eq!(ta.cursor, (0, 2));

        ta.move_down();
        assert_eq!(ta.cursor, (1, 2));

        ta.move_up();
        assert_eq!(ta.cursor, (0, 2));

        ta.move_home();
        assert_eq!(ta.cursor, (0, 0));

        ta.move_end();
        assert_eq!(ta.cursor, (0, 5));
    }

    #[test]
    fn test_textarea_undo_redo() {
        let mut ta = TextArea::new();
        ta.insert_char('A');
        ta.insert_char('B');
        assert_eq!(ta.get_content(), "AB");

        ta.undo();
        assert_eq!(ta.get_content(), "A");

        ta.undo();
        assert_eq!(ta.get_content(), "");

        ta.redo();
        assert_eq!(ta.get_content(), "A");

        ta.redo();
        assert_eq!(ta.get_content(), "AB");
    }

    #[test]
    fn test_textarea_selection() {
        let mut ta = TextArea::new().content("Hello World");
        ta.selection = Some(Selection::new((0, 0), (0, 5)));

        let selected = ta.get_selection();
        assert_eq!(selected, Some("Hello".to_string()));
    }

    #[test]
    fn test_textarea_delete_selection() {
        let mut ta = TextArea::new().content("Hello World");
        ta.selection = Some(Selection::new((0, 0), (0, 6)));
        ta.delete_selection();
        assert_eq!(ta.get_content(), "World");
    }

    #[test]
    fn test_textarea_render() {
        let ta = TextArea::new()
            .content("Line 1\nLine 2\nLine 3")
            .line_numbers(true);

        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        ta.render(&mut ctx);
    }

    #[test]
    fn test_textarea_word_movement() {
        let mut ta = TextArea::new().content("Hello World Test");
        ta.cursor = (0, 0);

        ta.move_word_right();
        assert_eq!(ta.cursor.1, 6); // After "Hello "

        ta.move_word_right();
        assert_eq!(ta.cursor.1, 12); // After "World "

        ta.move_word_left();
        assert_eq!(ta.cursor.1, 6);
    }

    #[test]
    fn test_textarea_handle_key() {
        let mut ta = TextArea::new();

        assert!(ta.handle_key(&Key::Char('A')));
        assert_eq!(ta.get_content(), "A");

        assert!(ta.handle_key(&Key::Enter));
        assert_eq!(ta.lines.len(), 2);

        assert!(ta.handle_key(&Key::Backspace));
        assert_eq!(ta.lines.len(), 1);
    }

    #[test]
    fn test_textarea_delete_line() {
        let mut ta = TextArea::new().content("Line 1\nLine 2\nLine 3");
        ta.cursor = (1, 0);
        ta.delete_line();
        assert_eq!(ta.line_count(), 2);
        assert_eq!(ta.get_content(), "Line 1\nLine 3");
    }

    #[test]
    fn test_textarea_duplicate_line() {
        let mut ta = TextArea::new().content("Line 1\nLine 2");
        ta.cursor = (0, 0);
        ta.duplicate_line();
        assert_eq!(ta.line_count(), 3);
        assert_eq!(ta.get_content(), "Line 1\nLine 1\nLine 2");
    }

    #[test]
    fn test_textarea_select_all() {
        let mut ta = TextArea::new().content("Hello\nWorld");
        ta.select_all();
        assert!(ta.selection.is_some());
        let sel = ta.selection.unwrap();
        assert_eq!(sel.start, (0, 0));
        assert_eq!(sel.end, (1, 5));
    }

    #[test]
    fn test_selection_contains() {
        let sel = Selection::new((0, 5), (0, 10));
        assert!(!sel.contains(0, 4));
        assert!(sel.contains(0, 5));
        assert!(sel.contains(0, 7));
        assert!(!sel.contains(0, 10));
    }

    #[test]
    fn test_textarea_helper() {
        let ta = textarea();
        assert_eq!(ta.line_count(), 1);
    }

    #[test]
    fn test_textarea_read_only() {
        let mut ta = TextArea::new().read_only(true);
        ta.insert_char('A');
        assert_eq!(ta.get_content(), "");
    }

    #[test]
    fn test_textarea_max_lines() {
        let mut ta = TextArea::new().max_lines(2);
        ta.insert_char('A');
        ta.insert_char('\n');
        ta.insert_char('B');
        ta.insert_char('\n'); // Should not add third line
        ta.insert_char('C');
        assert_eq!(ta.line_count(), 2);
    }

    #[test]
    fn test_textarea_syntax_highlighting() {
        let ta = TextArea::new()
            .content("fn main() {\n    println!(\"Hello\");\n}")
            .syntax(Language::Rust);

        assert_eq!(ta.get_syntax_language(), Language::Rust);
    }

    #[test]
    fn test_textarea_syntax_with_theme() {
        let ta = TextArea::new()
            .content("def hello():\n    print('Hi')")
            .syntax_with_theme(Language::Python, SyntaxTheme::monokai());

        assert_eq!(ta.get_syntax_language(), Language::Python);
    }

    #[test]
    fn test_textarea_set_language() {
        let mut ta = TextArea::new().content("console.log('Hello');");

        ta.set_language(Language::JavaScript);
        assert_eq!(ta.get_syntax_language(), Language::JavaScript);

        ta.set_language(Language::None);
        assert_eq!(ta.get_syntax_language(), Language::None);
    }

    #[test]
    fn test_textarea_syntax_render() {
        let ta = TextArea::new()
            .content("fn test() {}")
            .syntax(Language::Rust);

        let mut buffer = Buffer::new(40, 5);
        let area = Rect::new(0, 0, 40, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should not panic when rendering with syntax highlighting
        ta.render(&mut ctx);
    }
}
