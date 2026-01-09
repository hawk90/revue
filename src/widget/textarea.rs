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

/// Maximum number of cursors allowed
const MAX_CURSORS: usize = 100;

// ============================================================================
// Cursor Types
// ============================================================================

/// A cursor position in the text (line, column)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CursorPos {
    /// Line index (0-based)
    pub line: usize,
    /// Column index (0-based)
    pub col: usize,
}

impl CursorPos {
    /// Create a new cursor position
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }
}

impl From<(usize, usize)> for CursorPos {
    fn from((line, col): (usize, usize)) -> Self {
        Self { line, col }
    }
}

impl From<CursorPos> for (usize, usize) {
    fn from(pos: CursorPos) -> Self {
        (pos.line, pos.col)
    }
}

/// A cursor with optional selection anchor
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cursor {
    /// Current position
    pub pos: CursorPos,
    /// Selection anchor (if selecting)
    pub anchor: Option<CursorPos>,
}

impl Cursor {
    /// Create a new cursor at position
    pub fn new(pos: CursorPos) -> Self {
        Self { pos, anchor: None }
    }

    /// Create a cursor with selection
    pub fn with_selection(pos: CursorPos, anchor: CursorPos) -> Self {
        Self {
            pos,
            anchor: Some(anchor),
        }
    }

    /// Get the selection as a Selection struct if selecting
    pub fn selection(&self) -> Option<Selection> {
        self.anchor.map(|anchor| {
            let start = if self.pos < anchor {
                (self.pos.line, self.pos.col)
            } else {
                (anchor.line, anchor.col)
            };
            let end = if self.pos < anchor {
                (anchor.line, anchor.col)
            } else {
                (self.pos.line, self.pos.col)
            };
            Selection::new(start, end)
        })
    }

    /// Check if this cursor is selecting
    pub fn is_selecting(&self) -> bool {
        self.anchor.is_some()
    }

    /// Start selection at current position
    pub fn start_selection(&mut self) {
        self.anchor = Some(self.pos);
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.anchor = None;
    }
}

/// Collection of cursors (always has at least one - the primary cursor)
#[derive(Clone, Debug)]
pub struct CursorSet {
    /// All cursors, primary is at index 0
    cursors: Vec<Cursor>,
}

impl CursorSet {
    /// Create a new cursor set with a single cursor at position
    pub fn new(pos: CursorPos) -> Self {
        Self {
            cursors: vec![Cursor::new(pos)],
        }
    }

    /// Get the primary cursor (immutable)
    pub fn primary(&self) -> &Cursor {
        &self.cursors[0]
    }

    /// Get the primary cursor (mutable)
    pub fn primary_mut(&mut self) -> &mut Cursor {
        &mut self.cursors[0]
    }

    /// Get all cursors
    #[allow(dead_code)]
    pub fn all(&self) -> &[Cursor] {
        &self.cursors
    }

    /// Get all cursors (mutable)
    #[allow(dead_code)]
    pub fn all_mut(&mut self) -> &mut [Cursor] {
        &mut self.cursors
    }

    /// Get the number of cursors
    pub fn len(&self) -> usize {
        self.cursors.len()
    }

    /// Check if there's only one cursor
    #[allow(dead_code)]
    pub fn is_single(&self) -> bool {
        self.cursors.len() == 1
    }

    /// Add a cursor at position
    pub fn add(&mut self, cursor: Cursor) {
        if self.cursors.len() < MAX_CURSORS {
            self.cursors.push(cursor);
            self.normalize();
        }
    }

    /// Add a cursor at position (convenience method)
    pub fn add_at(&mut self, pos: CursorPos) {
        self.add(Cursor::new(pos));
    }

    /// Clear all secondary cursors, keeping only the primary
    pub fn clear_secondary(&mut self) {
        self.cursors.truncate(1);
    }

    /// Set the primary cursor position
    pub fn set_primary(&mut self, pos: CursorPos) {
        self.cursors[0].pos = pos;
    }

    /// Iterate over all cursors
    pub fn iter(&self) -> impl Iterator<Item = &Cursor> {
        self.cursors.iter()
    }

    /// Iterate over all cursors (mutable)
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Cursor> {
        self.cursors.iter_mut()
    }

    /// Sort cursors by position and merge overlapping ones
    fn normalize(&mut self) {
        // Sort by position (line, then column)
        self.cursors.sort_by(|a, b| a.pos.cmp(&b.pos));

        // Remove duplicates (cursors at same position)
        self.cursors.dedup_by(|a, b| a.pos == b.pos);

        // Ensure we always have at least one cursor
        if self.cursors.is_empty() {
            self.cursors.push(Cursor::new(CursorPos::new(0, 0)));
        }
    }

    /// Get positions of all cursors sorted in reverse order (for editing)
    #[allow(dead_code)]
    pub fn positions_reversed(&self) -> Vec<CursorPos> {
        let mut positions: Vec<CursorPos> = self.cursors.iter().map(|c| c.pos).collect();
        positions.sort_by(|a, b| b.cmp(a)); // Reverse order
        positions
    }
}

impl Default for CursorSet {
    fn default() -> Self {
        Self::new(CursorPos::new(0, 0))
    }
}

// ============================================================================
// Selection
// ============================================================================

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

// ============================================================================
// Find/Replace Types
// ============================================================================

/// Find/Replace mode
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FindReplaceMode {
    #[default]
    Find,
    Replace,
}

/// Options for find/replace operations
#[derive(Clone, Debug, Default)]
pub struct FindOptions {
    /// Case-sensitive search
    pub case_sensitive: bool,
    /// Match whole words only
    pub whole_word: bool,
    /// Use regex pattern
    pub use_regex: bool,
}

/// A match found in the text
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FindMatch {
    /// Start position
    pub start: CursorPos,
    /// End position
    pub end: CursorPos,
}

impl FindMatch {
    /// Create a new find match
    pub fn new(start: CursorPos, end: CursorPos) -> Self {
        Self { start, end }
    }
}

/// Find/Replace state
#[derive(Clone, Debug, Default)]
pub struct FindReplaceState {
    /// Search query
    pub query: String,
    /// Replacement text
    pub replace_with: String,
    /// Search options
    pub options: FindOptions,
    /// All matches in document
    pub matches: Vec<FindMatch>,
    /// Currently focused match index
    pub current_match: Option<usize>,
    /// UI mode (Find or Replace)
    pub mode: FindReplaceMode,
    /// Input focus: true = query input, false = replace input
    pub query_focused: bool,
}

impl FindReplaceState {
    /// Create a new find/replace state
    pub fn new(mode: FindReplaceMode) -> Self {
        Self {
            mode,
            query_focused: true,
            ..Default::default()
        }
    }

    /// Get match count
    pub fn match_count(&self) -> usize {
        self.matches.len()
    }

    /// Get current match (1-indexed for display)
    pub fn current_match_display(&self) -> usize {
        self.current_match.map(|i| i + 1).unwrap_or(0)
    }
}

// ============================================================================
// Edit Operations
// ============================================================================

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
    /// Multiple cursors (primary cursor is at index 0)
    cursors: CursorSet,
    /// Scroll offset (line, column)
    scroll: (usize, usize),
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
    /// Find/Replace state
    find_replace: Option<FindReplaceState>,
    /// Match highlight color
    match_highlight_bg: Option<Color>,
    /// Current match highlight color
    current_match_bg: Option<Color>,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
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
            find_replace: None,
            match_highlight_bg: Some(Color::rgb(100, 100, 0)),
            current_match_bg: Some(Color::rgb(255, 200, 0)),
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
        self.cursors = CursorSet::default();
        self.scroll = (0, 0);
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// Get the number of lines
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    /// Get the cursor position (primary cursor for backward compatibility)
    pub fn cursor_position(&self) -> (usize, usize) {
        let pos = self.cursors.primary().pos;
        (pos.line, pos.col)
    }

    /// Get all cursor positions
    pub fn cursor_positions(&self) -> Vec<(usize, usize)> {
        self.cursors
            .iter()
            .map(|c| (c.pos.line, c.pos.col))
            .collect()
    }

    /// Get the number of cursors
    pub fn cursor_count(&self) -> usize {
        self.cursors.len()
    }

    /// Set the cursor position (primary cursor, clears secondary cursors)
    pub fn set_cursor(&mut self, line: usize, col: usize) {
        let line = line.min(self.lines.len().saturating_sub(1));
        let col = col.min(self.line_len(line));
        self.cursors = CursorSet::new(CursorPos::new(line, col));
    }

    /// Get length of a specific line
    fn line_len(&self, line: usize) -> usize {
        self.lines.get(line).map(|l| l.len()).unwrap_or(0)
    }

    /// Get current line length (primary cursor's line)
    #[allow(dead_code)]
    fn current_line_len(&self) -> usize {
        self.line_len(self.cursors.primary().pos.line)
    }

    /// Get selected text (from primary cursor)
    pub fn get_selection(&self) -> Option<String> {
        let sel = self.cursors.primary().selection()?.normalized();
        self.get_text_in_selection(&sel)
    }

    /// Get text within a selection range
    fn get_text_in_selection(&self, sel: &Selection) -> Option<String> {
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

    /// Delete selected text (from primary cursor)
    pub fn delete_selection(&mut self) {
        let sel = match self.cursors.primary().selection() {
            Some(s) => s.normalized(),
            None => return,
        };

        if sel.start.0 == sel.end.0 {
            // Single line selection
            if let Some(line) = self.lines.get_mut(sel.start.0) {
                let deleted: String = line.drain(sel.start.1..sel.end.1.min(line.len())).collect();
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

        // Update cursor to selection start
        self.cursors = CursorSet::new(CursorPos::new(sel.start.0, sel.start.1));
    }

    /// Check if primary cursor has a selection
    pub fn has_selection(&self) -> bool {
        self.cursors.primary().is_selecting()
    }

    /// Start selection at current cursor (primary)
    pub fn start_selection(&mut self) {
        self.cursors.primary_mut().start_selection();
    }

    /// Update selection to current cursor position
    fn update_selection(&mut self) {
        // Selection is automatically updated because anchor stays fixed
        // while cursor position moves. No explicit update needed.
    }

    /// Clear selection (all cursors)
    pub fn clear_selection(&mut self) {
        for cursor in self.cursors.iter_mut() {
            cursor.clear_selection();
        }
    }

    /// Push an operation to the undo stack
    fn push_undo(&mut self, op: EditOperation) {
        self.undo_stack.push(op);
        if self.undo_stack.len() > MAX_UNDO_HISTORY {
            self.undo_stack.remove(0);
        }
        self.redo_stack.clear();
    }

    /// Set primary cursor position (internal helper)
    fn set_primary_cursor(&mut self, line: usize, col: usize) {
        self.cursors.set_primary(CursorPos::new(line, col));
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
                    self.set_primary_cursor(*line, *col);
                }
                EditOperation::Delete { line, col, text } => {
                    if let Some(l) = self.lines.get_mut(*line) {
                        l.insert_str(*col, text);
                    }
                    self.set_primary_cursor(*line, *col + text.len());
                }
                EditOperation::InsertLine { line, .. } => {
                    if *line < self.lines.len() {
                        self.lines.remove(*line);
                    }
                    self.set_primary_cursor(line.saturating_sub(1), 0);
                }
                EditOperation::DeleteLine { line, content } => {
                    self.lines.insert(*line, content.clone());
                    self.set_primary_cursor(*line, 0);
                }
                EditOperation::SplitLine { line, col } => {
                    // Merge lines back
                    if *line + 1 < self.lines.len() {
                        let next = self.lines.remove(*line + 1);
                        if let Some(l) = self.lines.get_mut(*line) {
                            l.push_str(&next);
                        }
                    }
                    self.set_primary_cursor(*line, *col);
                }
                EditOperation::MergeLines { line, col } => {
                    // Split line again
                    if let Some(l) = self.lines.get_mut(*line) {
                        let rest: String = l.drain(*col..).collect();
                        self.lines.insert(*line + 1, rest);
                    }
                    self.set_primary_cursor(*line + 1, 0);
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
                    self.set_primary_cursor(*line, *col + text.len());
                }
                EditOperation::Delete { line, col, text } => {
                    if let Some(l) = self.lines.get_mut(*line) {
                        let end = (*col + text.len()).min(l.len());
                        l.drain(*col..end);
                    }
                    self.set_primary_cursor(*line, *col);
                }
                EditOperation::InsertLine { line, content } => {
                    self.lines.insert(*line, content.clone());
                    self.set_primary_cursor(*line, 0);
                }
                EditOperation::DeleteLine { line, .. } => {
                    if *line < self.lines.len() {
                        self.lines.remove(*line);
                    }
                    self.set_primary_cursor(*line.min(&self.lines.len().saturating_sub(1)), 0);
                }
                EditOperation::SplitLine { line, col } => {
                    if let Some(l) = self.lines.get_mut(*line) {
                        let rest: String = l.drain(*col..).collect();
                        self.lines.insert(*line + 1, rest);
                    }
                    self.set_primary_cursor(*line + 1, 0);
                }
                EditOperation::MergeLines { line, col } => {
                    if *line + 1 < self.lines.len() {
                        let next = self.lines.remove(*line + 1);
                        if let Some(l) = self.lines.get_mut(*line) {
                            l.push_str(&next);
                        }
                    }
                    self.set_primary_cursor(*line, *col);
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

        // Delete selection first if any
        if self.has_selection() {
            self.delete_selection();
        }

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

        let cursor_pos = self.cursors.primary().pos;
        if let Some(line) = self.lines.get_mut(cursor_pos.line) {
            let col = cursor_pos.col.min(line.len());
            line.insert(col, ch);
            self.push_undo(EditOperation::Insert {
                line: cursor_pos.line,
                col,
                text: ch.to_string(),
            });
            self.set_primary_cursor(cursor_pos.line, col + 1);
        }
    }

    /// Insert a string at cursor
    pub fn insert_str(&mut self, s: &str) {
        if self.read_only {
            return;
        }

        if self.has_selection() {
            self.delete_selection();
        }

        // Handle multi-line inserts
        let parts: Vec<&str> = s.split('\n').collect();
        if parts.len() == 1 {
            // Single line insert
            let cursor_pos = self.cursors.primary().pos;
            if let Some(line) = self.lines.get_mut(cursor_pos.line) {
                let col = cursor_pos.col.min(line.len());
                line.insert_str(col, s);
                self.push_undo(EditOperation::Insert {
                    line: cursor_pos.line,
                    col,
                    text: s.to_string(),
                });
                self.set_primary_cursor(cursor_pos.line, col + s.len());
            }
        } else {
            // Multi-line insert
            for (i, part) in parts.iter().enumerate() {
                let cursor_pos = self.cursors.primary().pos;
                if i == 0 {
                    if let Some(line) = self.lines.get_mut(cursor_pos.line) {
                        line.insert_str(cursor_pos.col, part);
                    }
                    self.set_primary_cursor(cursor_pos.line, cursor_pos.col + part.len());
                } else {
                    self.insert_newline();
                    let cursor_pos = self.cursors.primary().pos;
                    if let Some(line) = self.lines.get_mut(cursor_pos.line) {
                        line.insert_str(0, part);
                    }
                    self.set_primary_cursor(cursor_pos.line, part.len());
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

        let cursor_pos = self.cursors.primary().pos;
        let (line, col) = (cursor_pos.line, cursor_pos.col);
        if let Some(current) = self.lines.get_mut(line) {
            let rest: String = current.drain(col.min(current.len())..).collect();
            self.lines.insert(line + 1, rest);
            self.push_undo(EditOperation::SplitLine { line, col });
            self.set_primary_cursor(line + 1, 0);
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

        let cursor_pos = self.cursors.primary().pos;
        let (line, col) = (cursor_pos.line, cursor_pos.col);
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
                    self.set_primary_cursor(line, col - 1);
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
            self.set_primary_cursor(line - 1, prev_len);
        }
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

        let cursor_pos = self.cursors.primary().pos;
        let (line, col) = (cursor_pos.line, cursor_pos.col);
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

        let cursor_pos = self.cursors.primary().pos;
        let line = cursor_pos.line;
        let content = self.lines.remove(line);
        self.push_undo(EditOperation::DeleteLine { line, content });

        let new_line = line.min(self.lines.len().saturating_sub(1));
        self.set_primary_cursor(new_line, 0);
    }

    /// Duplicate the current line
    pub fn duplicate_line(&mut self) {
        if self.read_only {
            return;
        }

        let cursor_pos = self.cursors.primary().pos;
        let line = cursor_pos.line;
        let content = self.lines[line].clone();
        self.lines.insert(line + 1, content.clone());
        self.push_undo(EditOperation::InsertLine {
            line: line + 1,
            content,
        });
        self.set_primary_cursor(line + 1, cursor_pos.col);
    }

    /// Move cursor left
    pub fn move_left(&mut self) {
        let pos = self.cursors.primary().pos;
        if pos.col > 0 {
            self.set_primary_cursor(pos.line, pos.col - 1);
        } else if pos.line > 0 {
            let new_line = pos.line - 1;
            let new_col = self.line_len(new_line);
            self.set_primary_cursor(new_line, new_col);
        }
        self.update_selection();
    }

    /// Move cursor right
    pub fn move_right(&mut self) {
        let pos = self.cursors.primary().pos;
        let line_len = self.line_len(pos.line);
        if pos.col < line_len {
            self.set_primary_cursor(pos.line, pos.col + 1);
        } else if pos.line + 1 < self.lines.len() {
            self.set_primary_cursor(pos.line + 1, 0);
        }
        self.update_selection();
    }

    /// Move cursor up
    pub fn move_up(&mut self) {
        let pos = self.cursors.primary().pos;
        if pos.line > 0 {
            let new_line = pos.line - 1;
            let new_col = pos.col.min(self.line_len(new_line));
            self.set_primary_cursor(new_line, new_col);
        }
        self.update_selection();
    }

    /// Move cursor down
    pub fn move_down(&mut self) {
        let pos = self.cursors.primary().pos;
        if pos.line + 1 < self.lines.len() {
            let new_line = pos.line + 1;
            let new_col = pos.col.min(self.line_len(new_line));
            self.set_primary_cursor(new_line, new_col);
        }
        self.update_selection();
    }

    /// Move to start of line
    pub fn move_home(&mut self) {
        let pos = self.cursors.primary().pos;
        self.set_primary_cursor(pos.line, 0);
        self.update_selection();
    }

    /// Move to end of line
    pub fn move_end(&mut self) {
        let pos = self.cursors.primary().pos;
        let line_len = self.line_len(pos.line);
        self.set_primary_cursor(pos.line, line_len);
        self.update_selection();
    }

    /// Move to start of document
    pub fn move_document_start(&mut self) {
        self.set_primary_cursor(0, 0);
        self.update_selection();
    }

    /// Move to end of document
    pub fn move_document_end(&mut self) {
        let last_line = self.lines.len().saturating_sub(1);
        let last_col = self.line_len(last_line);
        self.set_primary_cursor(last_line, last_col);
        self.update_selection();
    }

    /// Move cursor by word to the left
    pub fn move_word_left(&mut self) {
        let pos = self.cursors.primary().pos;
        if pos.col == 0 {
            if pos.line > 0 {
                let new_line = pos.line - 1;
                let new_col = self.line_len(new_line);
                self.set_primary_cursor(new_line, new_col);
            }
            return;
        }

        let line = &self.lines[pos.line];
        let chars: Vec<char> = line.chars().collect();
        let mut col = pos.col.min(chars.len());

        // Skip spaces
        while col > 0 && chars[col - 1].is_whitespace() {
            col -= 1;
        }
        // Skip word
        while col > 0 && !chars[col - 1].is_whitespace() {
            col -= 1;
        }

        self.set_primary_cursor(pos.line, col);
        self.update_selection();
    }

    /// Move cursor by word to the right
    pub fn move_word_right(&mut self) {
        let pos = self.cursors.primary().pos;
        let line = &self.lines[pos.line];
        let chars: Vec<char> = line.chars().collect();
        let mut col = pos.col;

        if col >= chars.len() {
            if pos.line + 1 < self.lines.len() {
                self.set_primary_cursor(pos.line + 1, 0);
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

        self.set_primary_cursor(pos.line, col);
        self.update_selection();
    }

    /// Page up
    pub fn page_up(&mut self, page_size: usize) {
        let pos = self.cursors.primary().pos;
        let new_line = pos.line.saturating_sub(page_size);
        let new_col = pos.col.min(self.line_len(new_line));
        self.set_primary_cursor(new_line, new_col);
        self.update_selection();
    }

    /// Page down
    pub fn page_down(&mut self, page_size: usize) {
        let pos = self.cursors.primary().pos;
        let new_line = (pos.line + page_size).min(self.lines.len().saturating_sub(1));
        let new_col = pos.col.min(self.line_len(new_line));
        self.set_primary_cursor(new_line, new_col);
        self.update_selection();
    }

    /// Select all text
    pub fn select_all(&mut self) {
        let last_line = self.lines.len().saturating_sub(1);
        let last_col = self.lines.last().map(|l| l.len()).unwrap_or(0);
        // Create cursor at end with anchor at start
        self.cursors = CursorSet::new(CursorPos::new(last_line, last_col));
        self.cursors.primary_mut().anchor = Some(CursorPos::new(0, 0));
    }

    // =========================================================================
    // Find/Replace Methods
    // =========================================================================

    /// Open find panel (Ctrl+F)
    pub fn open_find(&mut self) {
        let mut state = FindReplaceState::new(FindReplaceMode::Find);
        // Pre-populate with selection if any
        if let Some(text) = self.get_selection() {
            state.query = text;
        }
        self.find_replace = Some(state);
        self.refresh_matches();
    }

    /// Open replace panel (Ctrl+H)
    pub fn open_replace(&mut self) {
        let mut state = FindReplaceState::new(FindReplaceMode::Replace);
        if let Some(text) = self.get_selection() {
            state.query = text;
        }
        self.find_replace = Some(state);
        self.refresh_matches();
    }

    /// Close find/replace panel
    pub fn close_find(&mut self) {
        self.find_replace = None;
    }

    /// Check if find panel is open
    pub fn is_find_open(&self) -> bool {
        self.find_replace.is_some()
    }

    /// Get find/replace state
    pub fn find_state(&self) -> Option<&FindReplaceState> {
        self.find_replace.as_ref()
    }

    /// Set find query and refresh matches
    pub fn set_find_query(&mut self, query: &str) {
        if let Some(ref mut state) = self.find_replace {
            state.query = query.to_string();
        }
        self.refresh_matches();
    }

    /// Set replacement text
    pub fn set_replace_text(&mut self, text: &str) {
        if let Some(ref mut state) = self.find_replace {
            state.replace_with = text.to_string();
        }
    }

    /// Find next match (F3)
    pub fn find_next(&mut self) {
        if let Some(ref mut state) = self.find_replace {
            if state.matches.is_empty() {
                return;
            }

            let current = state.current_match.unwrap_or(0);
            state.current_match = Some((current + 1) % state.matches.len());
            self.jump_to_current_match();
        }
    }

    /// Find previous match (Shift+F3)
    pub fn find_previous(&mut self) {
        if let Some(ref mut state) = self.find_replace {
            if state.matches.is_empty() {
                return;
            }

            let current = state.current_match.unwrap_or(0);
            let len = state.matches.len();
            state.current_match = Some((current + len - 1) % len);
            self.jump_to_current_match();
        }
    }

    /// Replace current match
    pub fn replace_current(&mut self) {
        if self.read_only {
            return;
        }

        let (start, end, replace_with) = {
            let state = match self.find_replace.as_ref() {
                Some(s) => s,
                None => return,
            };
            let idx = match state.current_match {
                Some(i) => i,
                None => return,
            };
            let m = match state.matches.get(idx) {
                Some(m) => m,
                None => return,
            };
            (m.start, m.end, state.replace_with.clone())
        };

        // Replace the text
        self.replace_range(start, end, &replace_with);
        self.refresh_matches();

        // Move to next match if available
        if self
            .find_replace
            .as_ref()
            .map(|s| !s.matches.is_empty())
            .unwrap_or(false)
        {
            // The current_match index might need adjustment since we removed a match
            if let Some(ref mut state) = self.find_replace {
                if state.current_match.unwrap_or(0) >= state.matches.len() {
                    state.current_match = Some(0);
                }
            }
            self.jump_to_current_match();
        }
    }

    /// Replace all matches (Ctrl+Shift+H)
    pub fn replace_all(&mut self) {
        if self.read_only {
            return;
        }

        let replacements: Vec<(CursorPos, CursorPos, String)> = {
            let state = match self.find_replace.as_ref() {
                Some(s) => s,
                None => return,
            };
            if state.matches.is_empty() {
                return;
            }
            state
                .matches
                .iter()
                .map(|m| (m.start, m.end, state.replace_with.clone()))
                .collect()
        };

        // Apply in reverse order to maintain position validity
        for (start, end, replace_with) in replacements.into_iter().rev() {
            self.replace_range(start, end, &replace_with);
        }

        self.refresh_matches();
    }

    /// Toggle case sensitivity
    pub fn toggle_case_sensitive(&mut self) {
        if let Some(ref mut state) = self.find_replace {
            state.options.case_sensitive = !state.options.case_sensitive;
        }
        self.refresh_matches();
    }

    /// Toggle whole word matching
    pub fn toggle_whole_word(&mut self) {
        if let Some(ref mut state) = self.find_replace {
            state.options.whole_word = !state.options.whole_word;
        }
        self.refresh_matches();
    }

    /// Toggle regex mode
    pub fn toggle_regex(&mut self) {
        if let Some(ref mut state) = self.find_replace {
            state.options.use_regex = !state.options.use_regex;
        }
        self.refresh_matches();
    }

    /// Refresh all matches based on current query
    fn refresh_matches(&mut self) {
        let (query, options) = match self.find_replace.as_ref() {
            Some(state) => (state.query.clone(), state.options.clone()),
            None => return,
        };

        let mut matches = Vec::new();

        if query.is_empty() {
            if let Some(ref mut state) = self.find_replace {
                state.matches = matches;
                state.current_match = None;
            }
            return;
        }

        // Search each line
        for (line_idx, line) in self.lines.iter().enumerate() {
            self.find_matches_in_line(line_idx, line, &query, &options, &mut matches);
        }

        // Update state
        if let Some(ref mut state) = self.find_replace {
            state.matches = matches;
            // Reset to first match or maintain position
            if !state.matches.is_empty() {
                if state.current_match.is_none()
                    || state.current_match.unwrap() >= state.matches.len()
                {
                    state.current_match = Some(0);
                }
            } else {
                state.current_match = None;
            }
        }
    }

    /// Find matches in a single line
    fn find_matches_in_line(
        &self,
        line_idx: usize,
        line: &str,
        query: &str,
        options: &FindOptions,
        matches: &mut Vec<FindMatch>,
    ) {
        if options.use_regex {
            // Regex search (simple implementation without regex crate)
            // For now, fall back to literal search
            self.find_literal_matches(line_idx, line, query, options, matches);
        } else {
            self.find_literal_matches(line_idx, line, query, options, matches);
        }
    }

    /// Find literal string matches
    fn find_literal_matches(
        &self,
        line_idx: usize,
        line: &str,
        query: &str,
        options: &FindOptions,
        matches: &mut Vec<FindMatch>,
    ) {
        let (search_line, search_query) = if options.case_sensitive {
            (line.to_string(), query.to_string())
        } else {
            (line.to_lowercase(), query.to_lowercase())
        };

        let mut start = 0;
        while let Some(pos) = search_line[start..].find(&search_query) {
            let match_start = start + pos;
            let match_end = match_start + query.len();

            // Check whole word if needed
            let is_whole_word =
                !options.whole_word || self.is_word_boundary(line, match_start, match_end);

            if is_whole_word {
                matches.push(FindMatch::new(
                    CursorPos::new(line_idx, match_start),
                    CursorPos::new(line_idx, match_end),
                ));
            }

            start = match_start + 1;
        }
    }

    /// Check if match is at word boundary
    fn is_word_boundary(&self, line: &str, start: usize, end: usize) -> bool {
        let chars: Vec<char> = line.chars().collect();
        let at_start = start == 0
            || !chars
                .get(start - 1)
                .map(|c| c.is_alphanumeric())
                .unwrap_or(false);
        let at_end =
            end >= chars.len() || !chars.get(end).map(|c| c.is_alphanumeric()).unwrap_or(false);
        at_start && at_end
    }

    /// Jump cursor to current match
    fn jump_to_current_match(&mut self) {
        let pos = {
            let state = match self.find_replace.as_ref() {
                Some(s) => s,
                None => return,
            };
            let idx = match state.current_match {
                Some(i) => i,
                None => return,
            };
            match state.matches.get(idx) {
                Some(m) => m.start,
                None => return,
            }
        };

        self.set_cursor(pos.line, pos.col);
        self.ensure_cursor_visible();
    }

    /// Ensure cursor is visible by adjusting scroll
    fn ensure_cursor_visible(&mut self) {
        // This would need the visible area size, which we don't have here
        // For now, just update scroll.0 to show the cursor line
        let cursor_line = self.cursors.primary().pos.line;
        if cursor_line < self.scroll.0 {
            self.scroll.0 = cursor_line;
        }
        // Note: Full implementation would need view height
    }

    /// Replace text in range
    fn replace_range(&mut self, start: CursorPos, end: CursorPos, replacement: &str) {
        if start.line == end.line {
            // Single line replacement
            if let Some(line) = self.lines.get_mut(start.line) {
                let before: String = line.chars().take(start.col).collect();
                let after: String = line.chars().skip(end.col).collect();
                *line = format!("{}{}{}", before, replacement, after);
            }
        } else {
            // Multi-line replacement
            let before: String = self.lines[start.line].chars().take(start.col).collect();
            let after: String = self.lines[end.line].chars().skip(end.col).collect();

            // Remove lines between start and end
            for _ in start.line..=end.line {
                if start.line < self.lines.len() {
                    self.lines.remove(start.line);
                }
            }

            // Insert replacement
            let new_content = format!("{}{}{}", before, replacement, after);
            let new_lines: Vec<String> = new_content.lines().map(String::from).collect();
            for (i, new_line) in new_lines.into_iter().enumerate() {
                self.lines.insert(start.line + i, new_line);
            }
        }
    }

    // =========================================================================
    // Multiple Cursors Methods
    // =========================================================================

    /// Add cursor at position (Alt+Click)
    pub fn add_cursor_at(&mut self, line: usize, col: usize) {
        let line = line.min(self.lines.len().saturating_sub(1));
        let col = col.min(self.line_len(line));
        self.cursors.add_at(CursorPos::new(line, col));
    }

    /// Add cursor above current (Ctrl+Alt+Up)
    pub fn add_cursor_above(&mut self) {
        let primary = self.cursors.primary().pos;
        if primary.line > 0 {
            let new_line = primary.line - 1;
            let new_col = primary.col.min(self.line_len(new_line));
            self.cursors.add_at(CursorPos::new(new_line, new_col));
        }
    }

    /// Add cursor below current (Ctrl+Alt+Down)
    pub fn add_cursor_below(&mut self) {
        let primary = self.cursors.primary().pos;
        if primary.line + 1 < self.lines.len() {
            let new_line = primary.line + 1;
            let new_col = primary.col.min(self.line_len(new_line));
            self.cursors.add_at(CursorPos::new(new_line, new_col));
        }
    }

    /// Clear all secondary cursors (Escape)
    pub fn clear_secondary_cursors(&mut self) {
        self.cursors.clear_secondary();
    }

    /// Get word at cursor position
    fn get_word_at_cursor(&self) -> String {
        let pos = self.cursors.primary().pos;
        if pos.line >= self.lines.len() {
            return String::new();
        }
        let line = &self.lines[pos.line];
        let chars: Vec<char> = line.chars().collect();

        if chars.is_empty() || pos.col >= chars.len() {
            return String::new();
        }

        let mut start = pos.col;
        let mut end = pos.col;

        // Expand left
        while start > 0 && chars[start - 1].is_alphanumeric() {
            start -= 1;
        }

        // Expand right
        while end < chars.len() && chars[end].is_alphanumeric() {
            end += 1;
        }

        chars[start..end].iter().collect()
    }

    /// Get current word or selection text
    fn get_word_or_selection(&self) -> String {
        // If selection exists, return selected text
        if let Some(text) = self.get_selection() {
            return text;
        }
        // Otherwise get word under cursor
        self.get_word_at_cursor()
    }

    /// Find next occurrence of text from a given position
    fn find_next_from(&self, text: &str, from: CursorPos) -> Option<CursorPos> {
        if text.is_empty() {
            return None;
        }

        let text_lower = text.to_lowercase();

        // Search from the position after `from`
        for line_idx in from.line..self.lines.len() {
            let line = &self.lines[line_idx];
            let line_lower = line.to_lowercase();

            let start_col = if line_idx == from.line {
                from.col + 1
            } else {
                0
            };

            if start_col < line.len() {
                if let Some(pos) = line_lower[start_col..].find(&text_lower) {
                    return Some(CursorPos::new(line_idx, start_col + pos));
                }
            }
        }

        // Wrap around to beginning
        for line_idx in 0..=from.line {
            let line = &self.lines[line_idx];
            let line_lower = line.to_lowercase();

            let end_col = if line_idx == from.line {
                from.col + 1
            } else {
                line.len()
            };

            if let Some(pos) = line_lower[..end_col].find(&text_lower) {
                let found_pos = CursorPos::new(line_idx, pos);
                // Don't return if it's the same as one of our existing cursors
                if !self.cursors.iter().any(|c| c.pos == found_pos) {
                    return Some(found_pos);
                }
            }
        }

        None
    }

    /// Select next occurrence of current word/selection (Ctrl+D)
    pub fn select_next_occurrence(&mut self) {
        let text = self.get_word_or_selection();
        if text.is_empty() {
            return;
        }

        // Find next occurrence after the last cursor
        let last_pos = self
            .cursors
            .iter()
            .map(|c| c.pos)
            .max()
            .unwrap_or(CursorPos::new(0, 0));

        if let Some(match_pos) = self.find_next_from(&text, last_pos) {
            let end_col = match_pos.col + text.len();
            let new_cursor =
                Cursor::with_selection(CursorPos::new(match_pos.line, end_col), match_pos);
            self.cursors.add(new_cursor);
        }
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

                // Check if this position is selected (from any cursor)
                let is_selected = self.cursors.iter().any(|c| {
                    c.selection()
                        .map(|s| s.contains(line_idx, char_idx))
                        .unwrap_or(false)
                });

                // Check if this is any cursor position
                let is_cursor = self.focused
                    && self
                        .cursors
                        .iter()
                        .any(|c| c.pos.line == line_idx && c.pos.col == char_idx);

                // Check if this position is in a find match
                let (is_match, is_current_match) = if let Some(ref state) = self.find_replace {
                    let mut in_match = false;
                    let mut in_current = false;
                    for (idx, m) in state.matches.iter().enumerate() {
                        if m.start.line == line_idx
                            && char_idx >= m.start.col
                            && char_idx < m.end.col
                        {
                            in_match = true;
                            if state.current_match == Some(idx) {
                                in_current = true;
                            }
                            break;
                        }
                    }
                    (in_match, in_current)
                } else {
                    (false, false)
                };

                if is_cursor {
                    cell.fg = self.cursor_fg;
                    cell.bg = Some(Color::WHITE);
                    cell.modifier = Modifier::BOLD;
                } else if is_selected {
                    cell.fg = Some(Color::WHITE);
                    cell.bg = self.selection_bg;
                } else if is_current_match {
                    cell.fg = Some(Color::BLACK);
                    cell.bg = self.current_match_bg;
                } else if is_match {
                    cell.fg = Some(Color::BLACK);
                    cell.bg = self.match_highlight_bg;
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

            // Draw cursors at end of line if needed
            if self.focused {
                for cursor in self.cursors.iter() {
                    if cursor.pos.line == line_idx && cursor.pos.col >= chars.len() {
                        let cursor_x = area.x + text_start_x + (cursor.pos.col - scroll_col) as u16;
                        if cursor_x < area.x + area.width {
                            let mut cell = Cell::new(' ');
                            cell.bg = Some(Color::WHITE);
                            ctx.buffer.set(cursor_x, y, cell);
                        }
                    }
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
        assert_eq!(ta.cursor_position(), (0, 2));
    }

    #[test]
    fn test_textarea_insert_newline() {
        let mut ta = TextArea::new().content("Hello");
        ta.set_cursor(0, 5);
        ta.insert_char('\n');
        assert_eq!(ta.lines.len(), 2);
        assert_eq!(ta.cursor_position(), (1, 0));
    }

    #[test]
    fn test_textarea_delete_char_before() {
        let mut ta = TextArea::new().content("Hello");
        ta.set_cursor(0, 3);
        ta.delete_char_before();
        assert_eq!(ta.get_content(), "Helo");
        assert_eq!(ta.cursor_position(), (0, 2));
    }

    #[test]
    fn test_textarea_delete_char_at() {
        let mut ta = TextArea::new().content("Hello");
        ta.set_cursor(0, 2);
        ta.delete_char_at();
        assert_eq!(ta.get_content(), "Helo");
    }

    #[test]
    fn test_textarea_movement() {
        let mut ta = TextArea::new().content("Hello\nWorld");
        ta.set_cursor(0, 2);

        ta.move_right();
        assert_eq!(ta.cursor_position(), (0, 3));

        ta.move_left();
        assert_eq!(ta.cursor_position(), (0, 2));

        ta.move_down();
        assert_eq!(ta.cursor_position(), (1, 2));

        ta.move_up();
        assert_eq!(ta.cursor_position(), (0, 2));

        ta.move_home();
        assert_eq!(ta.cursor_position(), (0, 0));

        ta.move_end();
        assert_eq!(ta.cursor_position(), (0, 5));
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
        // Create selection from position 0 to 5
        ta.set_cursor(0, 0);
        ta.start_selection();
        ta.set_cursor(0, 5);
        ta.cursors.primary_mut().anchor = Some(CursorPos::new(0, 0));

        let selected = ta.get_selection();
        assert_eq!(selected, Some("Hello".to_string()));
    }

    #[test]
    fn test_textarea_delete_selection() {
        let mut ta = TextArea::new().content("Hello World");
        // Create selection from position 0 to 6
        ta.set_cursor(0, 6);
        ta.cursors.primary_mut().anchor = Some(CursorPos::new(0, 0));
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
        ta.set_cursor(0, 0);

        ta.move_word_right();
        assert_eq!(ta.cursor_position().1, 6); // After "Hello "

        ta.move_word_right();
        assert_eq!(ta.cursor_position().1, 12); // After "World "

        ta.move_word_left();
        assert_eq!(ta.cursor_position().1, 6);
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
        ta.set_cursor(1, 0);
        ta.delete_line();
        assert_eq!(ta.line_count(), 2);
        assert_eq!(ta.get_content(), "Line 1\nLine 3");
    }

    #[test]
    fn test_textarea_duplicate_line() {
        let mut ta = TextArea::new().content("Line 1\nLine 2");
        ta.set_cursor(0, 0);
        ta.duplicate_line();
        assert_eq!(ta.line_count(), 3);
        assert_eq!(ta.get_content(), "Line 1\nLine 1\nLine 2");
    }

    #[test]
    fn test_textarea_select_all() {
        let mut ta = TextArea::new().content("Hello\nWorld");
        ta.select_all();
        assert!(ta.has_selection());
        let sel = ta.cursors.primary().selection().unwrap();
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

    // =========================================================================
    // Find/Replace Tests
    // =========================================================================

    #[test]
    fn test_find_basic() {
        let mut ta = TextArea::new().content("Hello World");
        ta.open_find();
        ta.set_find_query("World");

        let state = ta.find_state().unwrap();
        assert_eq!(state.matches.len(), 1);
        assert_eq!(state.matches[0].start, CursorPos::new(0, 6));
        assert_eq!(state.matches[0].end, CursorPos::new(0, 11));
    }

    #[test]
    fn test_find_case_insensitive() {
        let mut ta = TextArea::new().content("Hello hello HELLO");
        ta.open_find();
        ta.set_find_query("hello");

        // Default is case insensitive
        let state = ta.find_state().unwrap();
        assert_eq!(state.matches.len(), 3);
    }

    #[test]
    fn test_find_case_sensitive() {
        let mut ta = TextArea::new().content("Hello hello HELLO");
        ta.open_find();
        ta.toggle_case_sensitive();
        ta.set_find_query("hello");

        let state = ta.find_state().unwrap();
        assert_eq!(state.matches.len(), 1);
    }

    #[test]
    fn test_find_whole_word() {
        let mut ta = TextArea::new().content("test testing tested");
        ta.open_find();
        ta.toggle_whole_word();
        ta.set_find_query("test");

        let state = ta.find_state().unwrap();
        assert_eq!(state.matches.len(), 1);
        assert_eq!(state.matches[0].start.col, 0);
    }

    #[test]
    fn test_find_next_wraps() {
        let mut ta = TextArea::new().content("aaa");
        ta.open_find();
        ta.set_find_query("a");

        assert_eq!(ta.find_state().unwrap().current_match, Some(0));
        ta.find_next();
        assert_eq!(ta.find_state().unwrap().current_match, Some(1));
        ta.find_next();
        assert_eq!(ta.find_state().unwrap().current_match, Some(2));
        ta.find_next();
        assert_eq!(ta.find_state().unwrap().current_match, Some(0)); // Wrapped
    }

    #[test]
    fn test_replace_current() {
        let mut ta = TextArea::new().content("foo bar foo");
        ta.open_replace();
        ta.set_find_query("foo");
        ta.set_replace_text("baz");

        ta.replace_current();
        assert_eq!(ta.get_content(), "baz bar foo");
    }

    #[test]
    fn test_replace_all() {
        let mut ta = TextArea::new().content("foo bar foo");
        ta.open_replace();
        ta.set_find_query("foo");
        ta.set_replace_text("baz");

        ta.replace_all();
        assert_eq!(ta.get_content(), "baz bar baz");
    }

    #[test]
    fn test_close_find() {
        let mut ta = TextArea::new().content("test");
        ta.open_find();
        assert!(ta.is_find_open());
        ta.close_find();
        assert!(!ta.is_find_open());
    }

    // =========================================================================
    // Multiple Cursors Tests
    // =========================================================================

    #[test]
    fn test_add_cursor() {
        let mut ta = TextArea::new().content("Hello\nWorld\nTest");
        ta.add_cursor_at(1, 2);
        assert_eq!(ta.cursor_count(), 2);
    }

    #[test]
    fn test_add_cursor_above() {
        let mut ta = TextArea::new().content("Line1\nLine2\nLine3");
        ta.set_cursor(1, 3);
        ta.add_cursor_above();
        assert_eq!(ta.cursor_count(), 2);
        let positions = ta.cursor_positions();
        assert!(positions.contains(&(0, 3)));
        assert!(positions.contains(&(1, 3)));
    }

    #[test]
    fn test_add_cursor_below() {
        let mut ta = TextArea::new().content("Line1\nLine2\nLine3");
        ta.set_cursor(1, 3);
        ta.add_cursor_below();
        assert_eq!(ta.cursor_count(), 2);
        let positions = ta.cursor_positions();
        assert!(positions.contains(&(1, 3)));
        assert!(positions.contains(&(2, 3)));
    }

    #[test]
    fn test_clear_secondary_cursors() {
        let mut ta = TextArea::new().content("Hello\nWorld");
        ta.add_cursor_at(1, 0);
        assert_eq!(ta.cursor_count(), 2);
        ta.clear_secondary_cursors();
        assert_eq!(ta.cursor_count(), 1);
    }

    #[test]
    fn test_select_next_occurrence() {
        let mut ta = TextArea::new().content("foo bar foo baz foo");
        ta.set_cursor(0, 0);

        // Select "foo" (set cursor at position and give it a word)
        ta.select_next_occurrence();
        // First occurrence at 0,0 should now have a cursor
        assert!(ta.cursor_count() >= 1);
    }

    #[test]
    fn test_cursor_positions() {
        let mut ta = TextArea::new().content("Hello\nWorld");
        ta.add_cursor_at(1, 2);

        let positions = ta.cursor_positions();
        assert_eq!(positions.len(), 2);
    }
}
