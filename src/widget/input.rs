//! Text input widget with selection, clipboard, and undo/redo support
//!
//! Note: All cursor and selection positions are in CHARACTER indices, not byte indices.
//! This ensures correct handling of multi-byte UTF-8 characters (emoji, CJK, etc).

use super::traits::{RenderContext, View, WidgetProps};
use crate::event::{Key, KeyEvent};
use crate::render::Cell;
use crate::style::Color;
use crate::{impl_props_builders, impl_styled_view};

/// Maximum undo history size
const MAX_UNDO_HISTORY: usize = 100;

/// An edit operation for undo/redo (single-line input)
#[derive(Clone, Debug)]
enum EditOperation {
    /// Insert text at position
    Insert { pos: usize, text: String },
    /// Delete text at position
    Delete { pos: usize, text: String },
    /// Replace entire value (for paste over selection, etc.)
    Replace {
        old_value: String,
        old_cursor: usize,
        new_value: String,
        new_cursor: usize,
    },
}

/// A text input widget with cursor, selection, clipboard, and undo/redo support
///
/// All cursor positions are character-based (not byte-based) to properly
/// handle UTF-8 multi-byte characters like emoji and CJK characters.
#[derive(Clone, Debug)]
pub struct Input {
    value: String,
    /// Cursor position in CHARACTER index (not byte index)
    cursor: usize,
    /// Selection anchor in CHARACTER index (where selection started)
    selection_anchor: Option<usize>,
    placeholder: String,
    fg: Option<Color>,
    bg: Option<Color>,
    cursor_fg: Option<Color>,
    cursor_bg: Option<Color>,
    selection_bg: Option<Color>,
    focused: bool,
    /// Internal clipboard (also syncs with system clipboard if available)
    clipboard: Option<String>,
    /// Undo history
    undo_stack: Vec<EditOperation>,
    /// Redo history
    redo_stack: Vec<EditOperation>,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
}

impl Input {
    // ─────────────────────────────────────────────────────────────────────────
    // UTF-8 helper methods
    // ─────────────────────────────────────────────────────────────────────────

    /// Get byte index from character index
    fn char_to_byte_index(&self, char_idx: usize) -> usize {
        self.value
            .char_indices()
            .nth(char_idx)
            .map(|(i, _)| i)
            .unwrap_or(self.value.len())
    }

    /// Get character count
    fn char_count(&self) -> usize {
        self.value.chars().count()
    }

    /// Insert string at character position (returns new cursor position)
    fn insert_at_char(&mut self, char_idx: usize, s: &str) -> usize {
        let byte_idx = self.char_to_byte_index(char_idx);
        self.value.insert_str(byte_idx, s);
        char_idx + s.chars().count()
    }

    /// Remove character at character position
    fn remove_char_at(&mut self, char_idx: usize) {
        let byte_idx = self.char_to_byte_index(char_idx);
        if let Some((_, ch)) = self.value.char_indices().nth(char_idx) {
            self.value.drain(byte_idx..byte_idx + ch.len_utf8());
        }
    }

    /// Remove range of characters (start..end in char indices)
    fn remove_char_range(&mut self, start: usize, end: usize) {
        let start_byte = self.char_to_byte_index(start);
        let end_byte = self.char_to_byte_index(end);
        self.value.drain(start_byte..end_byte);
    }

    /// Get substring by character range
    fn substring(&self, start: usize, end: usize) -> &str {
        let start_byte = self.char_to_byte_index(start);
        let end_byte = self.char_to_byte_index(end);
        &self.value[start_byte..end_byte]
    }
}

impl Input {
    /// Create a new input widget
    pub fn new() -> Self {
        Self {
            value: String::new(),
            cursor: 0,
            selection_anchor: None,
            placeholder: String::new(),
            fg: None,
            bg: None,
            cursor_fg: Some(Color::BLACK),
            cursor_bg: Some(Color::WHITE),
            selection_bg: Some(Color::rgb(70, 130, 180)), // Steel blue
            focused: true,
            clipboard: None,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            props: WidgetProps::new(),
        }
    }

    /// Set initial value
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self.cursor = self.char_count();
        self
    }

    /// Set placeholder text
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = text.into();
        self
    }

    /// Set foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Set cursor colors
    pub fn cursor_style(mut self, fg: Color, bg: Color) -> Self {
        self.cursor_fg = Some(fg);
        self.cursor_bg = Some(bg);
        self
    }

    /// Set selection background color
    pub fn selection_bg(mut self, color: Color) -> Self {
        self.selection_bg = Some(color);
        self
    }

    /// Set focused state
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Get current text content
    pub fn text(&self) -> &str {
        &self.value
    }

    /// Get cursor position
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Get selection range (start, end) if there is a selection
    pub fn selection(&self) -> Option<(usize, usize)> {
        self.selection_anchor.map(|anchor| {
            if anchor < self.cursor {
                (anchor, self.cursor)
            } else {
                (self.cursor, anchor)
            }
        })
    }

    /// Get selected text
    pub fn selected_text(&self) -> Option<&str> {
        self.selection()
            .map(|(start, end)| self.substring(start, end))
    }

    /// Check if there's an active selection
    pub fn has_selection(&self) -> bool {
        self.selection_anchor.is_some() && self.selection_anchor != Some(self.cursor)
    }

    /// Start selection at current cursor position
    pub fn start_selection(&mut self) {
        if self.selection_anchor.is_none() {
            self.selection_anchor = Some(self.cursor);
        }
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.selection_anchor = None;
    }

    /// Select all text
    pub fn select_all(&mut self) {
        self.selection_anchor = Some(0);
        self.cursor = self.char_count();
    }

    /// Delete selected text with undo support
    fn delete_selection_with_undo(&mut self) -> bool {
        if let Some((start, end)) = self.selection() {
            let deleted = self.substring(start, end).to_string();
            self.push_undo(EditOperation::Delete {
                pos: start,
                text: deleted,
            });
            self.remove_char_range(start, end);
            self.cursor = start;
            self.clear_selection();
            true
        } else {
            false
        }
    }

    /// Copy selection to clipboard
    pub fn copy(&mut self) {
        if let Some(text) = self.selected_text().map(|s| s.to_string()) {
            self.clipboard = Some(text.clone());
            // Try to copy to system clipboard
            #[cfg(feature = "clipboard")]
            if let Ok(mut ctx) = arboard::Clipboard::new() {
                let _ = ctx.set_text(&text);
            }
        }
    }

    /// Cut selection to clipboard
    pub fn cut(&mut self) -> bool {
        self.copy();
        self.delete_selection_with_undo()
    }

    /// Paste from clipboard (O(n) using insert_str)
    pub fn paste(&mut self) -> bool {
        // Try system clipboard first
        #[cfg(feature = "clipboard")]
        if let Ok(mut ctx) = arboard::Clipboard::new() {
            if let Ok(text) = ctx.get_text() {
                self.paste_text(&text);
                return true;
            }
        }

        // Fall back to internal clipboard
        if let Some(text) = self.clipboard.clone() {
            self.paste_text(&text);
            true
        } else {
            false
        }
    }

    /// Paste text with undo support (atomic operation using Replace)
    fn paste_text(&mut self, text: &str) {
        if let Some((start, end)) = self.selection() {
            // Paste over selection - use Replace for atomic undo
            let old_value = self.value.clone();
            let old_cursor = self.cursor;

            self.remove_char_range(start, end);
            self.cursor = start;
            self.clear_selection();
            self.cursor = self.insert_at_char(self.cursor, text);

            self.push_undo(EditOperation::Replace {
                old_value,
                old_cursor,
                new_value: self.value.clone(),
                new_cursor: self.cursor,
            });
        } else {
            // No selection - simple insert
            let pos = self.cursor;
            self.push_undo(EditOperation::Insert {
                pos,
                text: text.to_string(),
            });
            self.cursor = self.insert_at_char(self.cursor, text);
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Undo/Redo
    // ─────────────────────────────────────────────────────────────────────────

    /// Push an operation to the undo stack
    fn push_undo(&mut self, op: EditOperation) {
        self.undo_stack.push(op);
        if self.undo_stack.len() >= MAX_UNDO_HISTORY {
            self.undo_stack.remove(0);
        }
        self.redo_stack.clear();
    }

    /// Undo the last operation
    pub fn undo(&mut self) -> bool {
        if let Some(op) = self.undo_stack.pop() {
            match op {
                EditOperation::Insert { pos, ref text } => {
                    // Undo insert by deleting the inserted text
                    let end = pos + text.chars().count();
                    self.remove_char_range(pos, end);
                    self.cursor = pos;
                    self.redo_stack.push(op);
                }
                EditOperation::Delete { pos, ref text } => {
                    // Undo delete by inserting the deleted text
                    self.insert_at_char(pos, text);
                    self.cursor = pos + text.chars().count();
                    self.redo_stack.push(op);
                }
                EditOperation::Replace {
                    ref old_value,
                    old_cursor,
                    ref new_value,
                    new_cursor,
                } => {
                    // Undo replace by restoring old value
                    self.value = old_value.clone();
                    self.cursor = old_cursor;
                    self.redo_stack.push(EditOperation::Replace {
                        old_value: new_value.clone(),
                        old_cursor: new_cursor,
                        new_value: old_value.clone(),
                        new_cursor: old_cursor,
                    });
                }
            }
            self.clear_selection();
            true
        } else {
            false
        }
    }

    /// Redo the last undone operation
    pub fn redo(&mut self) -> bool {
        if let Some(op) = self.redo_stack.pop() {
            match op {
                EditOperation::Insert { pos, ref text } => {
                    // Redo insert
                    self.insert_at_char(pos, text);
                    self.cursor = pos + text.chars().count();
                    self.undo_stack.push(op);
                }
                EditOperation::Delete { pos, ref text } => {
                    // Redo delete
                    let end = pos + text.chars().count();
                    self.remove_char_range(pos, end);
                    self.cursor = pos;
                    self.undo_stack.push(op);
                }
                EditOperation::Replace {
                    ref old_value,
                    old_cursor,
                    ref new_value,
                    new_cursor,
                } => {
                    // Redo replace
                    self.value = new_value.clone();
                    self.cursor = new_cursor;
                    self.undo_stack.push(EditOperation::Replace {
                        old_value: old_value.clone(),
                        old_cursor,
                        new_value: new_value.clone(),
                        new_cursor,
                    });
                }
            }
            self.clear_selection();
            true
        } else {
            false
        }
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Clear undo/redo history
    pub fn clear_history(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// Handle Ctrl+key combinations
    ///
    /// Returns `Some(true)` if handled and needs redraw,
    /// `Some(false)` if handled but no redraw needed,
    /// `None` if not handled.
    fn handle_ctrl_key(&mut self, event: &KeyEvent) -> Option<bool> {
        match event.key {
            Key::Char('a') => {
                self.select_all();
                Some(true)
            }
            Key::Char('c') => {
                self.copy();
                Some(true)
            }
            Key::Char('x') => Some(self.cut()),
            Key::Char('v') => Some(self.paste()),
            Key::Char('z') => Some(self.undo()),
            Key::Char('y') => Some(self.redo()),
            Key::Left => {
                // Move to previous word (with optional selection)
                if event.shift {
                    self.start_selection();
                } else {
                    self.clear_selection();
                }
                self.move_word_left();
                Some(true)
            }
            Key::Right => {
                // Move to next word (with optional selection)
                if event.shift {
                    self.start_selection();
                } else {
                    self.clear_selection();
                }
                self.move_word_right();
                Some(true)
            }
            Key::Backspace => {
                self.delete_word_left();
                Some(true)
            }
            _ => None,
        }
    }

    /// Handle Shift+key combinations for selection
    ///
    /// Returns `Some(true)` if handled and needs redraw, `None` if not handled.
    fn handle_shift_key(&mut self, key: &Key) -> Option<bool> {
        let char_len = self.char_count();
        match key {
            Key::Left => {
                self.start_selection();
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
                Some(true)
            }
            Key::Right => {
                self.start_selection();
                if self.cursor < char_len {
                    self.cursor += 1;
                }
                Some(true)
            }
            Key::Home => {
                self.start_selection();
                self.cursor = 0;
                Some(true)
            }
            Key::End => {
                self.start_selection();
                self.cursor = char_len;
                Some(true)
            }
            _ => None,
        }
    }

    /// Handle key event with modifiers, returns true if needs redraw
    pub fn handle_key_event(&mut self, event: &KeyEvent) -> bool {
        // Try Ctrl combinations first
        if event.ctrl {
            if let Some(handled) = self.handle_ctrl_key(event) {
                return handled;
            }
        }

        // Try Shift combinations for selection
        if event.shift {
            if let Some(handled) = self.handle_shift_key(&event.key) {
                return handled;
            }
        }

        // Regular key handling (clears selection on most actions)
        self.handle_key(&event.key)
    }

    /// Handle key input (without modifiers), returns true if value changed
    pub fn handle_key(&mut self, key: &Key) -> bool {
        let char_len = self.char_count();

        match key {
            Key::Char(c) => {
                self.delete_selection_with_undo();
                // Create a string from the character and insert at cursor
                let s = c.to_string();
                let pos = self.cursor;
                self.push_undo(EditOperation::Insert {
                    pos,
                    text: s.clone(),
                });
                self.cursor = self.insert_at_char(self.cursor, &s);
                true
            }
            Key::Backspace => {
                if self.has_selection() {
                    self.delete_selection_with_undo()
                } else if self.cursor > 0 {
                    self.cursor -= 1;
                    // Get the character to be deleted for undo
                    let deleted = self.substring(self.cursor, self.cursor + 1).to_string();
                    self.push_undo(EditOperation::Delete {
                        pos: self.cursor,
                        text: deleted,
                    });
                    self.remove_char_at(self.cursor);
                    true
                } else {
                    false
                }
            }
            Key::Delete => {
                if self.has_selection() {
                    self.delete_selection_with_undo()
                } else if self.cursor < char_len {
                    // Get the character to be deleted for undo
                    let deleted = self.substring(self.cursor, self.cursor + 1).to_string();
                    self.push_undo(EditOperation::Delete {
                        pos: self.cursor,
                        text: deleted,
                    });
                    self.remove_char_at(self.cursor);
                    true
                } else {
                    false
                }
            }
            Key::Left => {
                self.clear_selection();
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
                true
            }
            Key::Right => {
                self.clear_selection();
                if self.cursor < char_len {
                    self.cursor += 1;
                }
                true
            }
            Key::Home => {
                self.clear_selection();
                self.cursor = 0;
                true
            }
            Key::End => {
                self.clear_selection();
                self.cursor = char_len;
                true
            }
            _ => false,
        }
    }

    /// Move cursor to the left by one word (zero allocation)
    fn move_word_left(&mut self) {
        if self.cursor == 0 {
            return;
        }

        // Get substring before cursor and iterate in reverse (no allocation)
        let byte_pos = self.char_to_byte_index(self.cursor);
        let before_cursor = &self.value[..byte_pos];

        let mut new_pos = self.cursor;

        // Skip whitespace going backwards
        for ch in before_cursor.chars().rev() {
            if ch.is_whitespace() {
                new_pos -= 1;
            } else {
                break;
            }
        }

        // Skip word characters going backwards
        let byte_pos = self.char_to_byte_index(new_pos);
        let before_new_pos = &self.value[..byte_pos];
        for ch in before_new_pos.chars().rev() {
            if !ch.is_whitespace() {
                new_pos -= 1;
            } else {
                break;
            }
        }

        self.cursor = new_pos;
    }

    /// Move cursor to the right by one word (zero allocation)
    fn move_word_right(&mut self) {
        let char_len = self.char_count();
        if self.cursor >= char_len {
            return;
        }

        // Get substring after cursor (no allocation)
        let byte_pos = self.char_to_byte_index(self.cursor);
        let after_cursor = &self.value[byte_pos..];

        let mut advance = 0;

        // Skip current word characters
        for ch in after_cursor.chars() {
            if !ch.is_whitespace() {
                advance += 1;
            } else {
                break;
            }
        }

        // Get remaining substring and skip whitespace
        let new_byte_pos = self.char_to_byte_index(self.cursor + advance);
        let remaining = &self.value[new_byte_pos..];
        for ch in remaining.chars() {
            if ch.is_whitespace() {
                advance += 1;
            } else {
                break;
            }
        }

        self.cursor = (self.cursor + advance).min(char_len);
    }

    /// Delete word to the left with undo support
    fn delete_word_left(&mut self) {
        if self.cursor == 0 {
            return;
        }

        let end = self.cursor;
        self.move_word_left();
        let start = self.cursor;

        // Get deleted text for undo
        let deleted = self.substring(start, end).to_string();
        self.push_undo(EditOperation::Delete {
            pos: start,
            text: deleted,
        });

        // Delete characters between new cursor position and old cursor position
        self.remove_char_range(start, end);
    }

    /// Clear the input (also clears undo history)
    pub fn clear(&mut self) {
        self.value.clear();
        self.cursor = 0;
        self.clear_selection();
        self.clear_history();
    }

    /// Set value programmatically (also clears undo history)
    pub fn set_value(&mut self, value: impl Into<String>) {
        self.value = value.into();
        self.cursor = self.char_count();
        self.clear_selection();
        self.clear_history();
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Input {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 {
            return;
        }

        let display_text = if self.value.is_empty() && !self.focused {
            &self.placeholder
        } else {
            &self.value
        };

        let is_placeholder = self.value.is_empty() && !self.focused;
        let selection = self.selection();

        // Get CSS colors with priority: inline > CSS > default
        let css_fg = self.fg.or_else(|| {
            ctx.style.and_then(|s| {
                let c = s.visual.color;
                if c != Color::default() {
                    Some(c)
                } else {
                    None
                }
            })
        });
        let css_bg = self.bg.or_else(|| {
            ctx.style.and_then(|s| {
                let c = s.visual.background;
                if c != Color::default() {
                    Some(c)
                } else {
                    None
                }
            })
        });

        let mut x = area.x;
        for (i, ch) in display_text.chars().enumerate() {
            if x >= area.x + area.width {
                break;
            }

            let is_cursor = self.focused && i == self.cursor;
            let is_selected = selection.map_or(false, |(start, end)| i >= start && i < end);
            let mut cell = Cell::new(ch);

            if is_cursor {
                cell.fg = self.cursor_fg;
                cell.bg = self.cursor_bg;
            } else if is_selected {
                cell.fg = Some(Color::WHITE);
                cell.bg = self.selection_bg;
            } else if is_placeholder {
                cell.fg = Some(Color::rgb(128, 128, 128)); // Gray for placeholder
            } else {
                cell.fg = css_fg;
                cell.bg = css_bg;
            }

            ctx.buffer.set(x, area.y, cell);

            let char_width = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(1) as u16;
            x += char_width;
        }

        // Draw cursor at end if cursor is at the end of text
        if self.focused && self.cursor >= display_text.len() && x < area.x + area.width {
            let mut cursor_cell = Cell::new(' ');
            cursor_cell.fg = self.cursor_fg;
            cursor_cell.bg = self.cursor_bg;
            ctx.buffer.set(x, area.y, cursor_cell);
        }
    }

    crate::impl_view_meta!("Input");
}

impl_styled_view!(Input);
impl_props_builders!(Input);

/// Helper function to create an input widget
pub fn input() -> Input {
    Input::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::StyledView;

    #[test]
    fn test_input_new() {
        let input = Input::new();
        assert_eq!(input.text(), "");
        assert_eq!(input.cursor(), 0);
    }

    #[test]
    fn test_input_with_value() {
        let input = Input::new().value("hello");
        assert_eq!(input.text(), "hello");
        assert_eq!(input.cursor(), 5);
    }

    #[test]
    fn test_input_type_char() {
        let mut input = Input::new();
        input.handle_key(&Key::Char('a'));
        input.handle_key(&Key::Char('b'));
        input.handle_key(&Key::Char('c'));
        assert_eq!(input.text(), "abc");
        assert_eq!(input.cursor(), 3);
    }

    #[test]
    fn test_input_backspace() {
        let mut input = Input::new().value("abc");
        input.handle_key(&Key::Backspace);
        assert_eq!(input.text(), "ab");
        assert_eq!(input.cursor(), 2);
    }

    #[test]
    fn test_input_delete() {
        let mut input = Input::new().value("abc");
        input.cursor = 1; // Position after 'a'
        input.handle_key(&Key::Delete);
        assert_eq!(input.text(), "ac");
    }

    #[test]
    fn test_input_cursor_movement() {
        let mut input = Input::new().value("hello");
        assert_eq!(input.cursor(), 5);

        input.handle_key(&Key::Left);
        assert_eq!(input.cursor(), 4);

        input.handle_key(&Key::Home);
        assert_eq!(input.cursor(), 0);

        input.handle_key(&Key::End);
        assert_eq!(input.cursor(), 5);

        input.handle_key(&Key::Right);
        assert_eq!(input.cursor(), 5); // Can't go past end
    }

    #[test]
    fn test_input_insert_middle() {
        let mut input = Input::new().value("ac");
        input.cursor = 1;
        input.handle_key(&Key::Char('b'));
        assert_eq!(input.text(), "abc");
        assert_eq!(input.cursor(), 2);
    }

    #[test]
    fn test_input_clear() {
        let mut input = Input::new().value("hello");
        input.clear();
        assert_eq!(input.text(), "");
        assert_eq!(input.cursor(), 0);
    }

    #[test]
    fn test_input_render() {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let input = Input::new().value("Hi").focused(true);
        input.render(&mut ctx);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, 'H');
        assert_eq!(buffer.get(1, 0).unwrap().symbol, 'i');
        // Cursor at position 2
        assert_eq!(buffer.get(2, 0).unwrap().bg, Some(Color::WHITE));
    }

    #[test]
    fn test_input_selection() {
        let mut input = Input::new().value("hello world");
        input.cursor = 0;

        // Select "hello" using shift+right simulation
        input.start_selection();
        input.cursor = 5;

        assert!(input.has_selection());
        assert_eq!(input.selection(), Some((0, 5)));
        assert_eq!(input.selected_text(), Some("hello"));
    }

    #[test]
    fn test_input_select_all() {
        let mut input = Input::new().value("hello world");
        input.select_all();

        assert!(input.has_selection());
        assert_eq!(input.selection(), Some((0, 11)));
        assert_eq!(input.selected_text(), Some("hello world"));
    }

    #[test]
    fn test_input_delete_selection() {
        let mut input = Input::new().value("hello world");
        input.selection_anchor = Some(0);
        input.cursor = 6; // Select "hello "

        input.handle_key(&Key::Backspace);

        assert_eq!(input.text(), "world");
        assert!(!input.has_selection());
    }

    #[test]
    fn test_input_copy_paste() {
        let mut input = Input::new().value("hello world");
        input.selection_anchor = Some(0);
        input.cursor = 5; // Select "hello"

        input.copy();
        // Verify internal clipboard was set
        assert_eq!(input.clipboard, Some("hello".to_string()));

        input.clear_selection();
        input.cursor = input.value.len();

        // Use paste_text directly to avoid system clipboard access in tests
        if let Some(text) = input.clipboard.clone() {
            input.paste_text(&text);
        }
        assert_eq!(input.text(), "hello worldhello");
    }

    #[test]
    fn test_input_cut() {
        let mut input = Input::new().value("hello world");
        input.selection_anchor = Some(0);
        input.cursor = 6; // Select "hello "

        input.cut();
        assert_eq!(input.text(), "world");
        // Verify internal clipboard was set
        assert_eq!(input.clipboard, Some("hello ".to_string()));

        // Paste back using internal clipboard directly
        input.cursor = 0;
        if let Some(text) = input.clipboard.clone() {
            input.paste_text(&text);
        }
        assert_eq!(input.text(), "hello world");
    }

    #[test]
    fn test_input_word_navigation() {
        let mut input = Input::new().value("hello world test");
        input.cursor = 0;

        input.move_word_right();
        assert_eq!(input.cursor, 6); // After "hello "

        input.move_word_right();
        assert_eq!(input.cursor, 12); // After "world "

        input.move_word_left();
        assert_eq!(input.cursor, 6); // Back to "world"
    }

    #[test]
    fn test_input_key_event_shift_selection() {
        let mut input = Input::new().value("hello");
        input.cursor = 0;

        // Shift+Right
        let event = KeyEvent {
            key: Key::Right,
            ctrl: false,
            alt: false,
            shift: true,
        };
        input.handle_key_event(&event);
        input.handle_key_event(&event);
        input.handle_key_event(&event);

        assert!(input.has_selection());
        assert_eq!(input.selection(), Some((0, 3)));
        assert_eq!(input.selected_text(), Some("hel"));
    }

    #[test]
    fn test_input_ctrl_a_select_all() {
        let mut input = Input::new().value("hello");

        let event = KeyEvent {
            key: Key::Char('a'),
            ctrl: true,
            alt: false,
            shift: false,
        };
        input.handle_key_event(&event);

        assert!(input.has_selection());
        assert_eq!(input.selected_text(), Some("hello"));
    }

    #[test]
    fn test_input_utf8_emoji() {
        // Test with emoji (multi-byte UTF-8)
        let mut input = Input::new().value("Hello 🎉 World");
        assert_eq!(input.cursor(), 13); // 13 characters, not 16 bytes

        // Move cursor left
        input.handle_key(&Key::Left);
        assert_eq!(input.cursor(), 12);

        // Select all should work correctly
        input.select_all();
        assert_eq!(input.selected_text(), Some("Hello 🎉 World"));

        // Delete emoji
        let mut input2 = Input::new().value("A🎉B");
        assert_eq!(input2.char_count(), 3); // 3 characters
        input2.cursor = 2; // After emoji
        input2.handle_key(&Key::Backspace);
        assert_eq!(input2.text(), "AB");
    }

    #[test]
    fn test_input_utf8_korean() {
        // Test with Korean (multi-byte UTF-8)
        let mut input = Input::new().value("안녕하세요");
        assert_eq!(input.cursor(), 5); // 5 characters
        assert_eq!(input.char_count(), 5);

        input.cursor = 2;
        input.start_selection();
        input.cursor = 4;
        assert_eq!(input.selected_text(), Some("하세"));

        // Insert at position
        input.clear_selection();
        input.cursor = 2;
        input.handle_key(&Key::Char('!'));
        assert_eq!(input.text(), "안녕!하세요");
    }

    #[test]
    fn test_input_paste_utf8() {
        let mut input = Input::new().value("AB");
        input.cursor = 1;
        // Use paste_text directly to avoid system clipboard interference
        input.paste_text("🎉한글");
        assert_eq!(input.text(), "A🎉한글B");
        assert_eq!(input.cursor(), 4); // After "A🎉한글"
    }

    #[test]
    fn test_input_undo_redo_insert() {
        let mut input = Input::new();

        // Type "abc"
        input.handle_key(&Key::Char('a'));
        input.handle_key(&Key::Char('b'));
        input.handle_key(&Key::Char('c'));
        assert_eq!(input.text(), "abc");
        assert!(input.can_undo());

        // Undo last character
        input.undo();
        assert_eq!(input.text(), "ab");
        assert!(input.can_redo());

        // Undo all
        input.undo();
        input.undo();
        assert_eq!(input.text(), "");

        // Redo
        input.redo();
        assert_eq!(input.text(), "a");
        input.redo();
        assert_eq!(input.text(), "ab");
    }

    #[test]
    fn test_input_undo_redo_delete() {
        let mut input = Input::new().value("hello");
        input.clear_history(); // Start fresh

        // Delete last char with backspace
        input.handle_key(&Key::Backspace);
        assert_eq!(input.text(), "hell");

        // Undo
        input.undo();
        assert_eq!(input.text(), "hello");

        // Redo
        input.redo();
        assert_eq!(input.text(), "hell");
    }

    #[test]
    fn test_input_undo_selection_delete() {
        let mut input = Input::new().value("hello world");
        input.clear_history();

        // Select "hello "
        input.selection_anchor = Some(0);
        input.cursor = 6;

        // Delete selection
        input.handle_key(&Key::Backspace);
        assert_eq!(input.text(), "world");

        // Undo
        input.undo();
        assert_eq!(input.text(), "hello world");
    }

    #[test]
    fn test_input_undo_ctrl_z() {
        let mut input = Input::new();

        input.handle_key(&Key::Char('a'));
        input.handle_key(&Key::Char('b'));
        assert_eq!(input.text(), "ab");

        // Ctrl+Z
        let event = KeyEvent {
            key: Key::Char('z'),
            ctrl: true,
            alt: false,
            shift: false,
        };
        input.handle_key_event(&event);
        assert_eq!(input.text(), "a");

        // Ctrl+Y (redo)
        let event = KeyEvent {
            key: Key::Char('y'),
            ctrl: true,
            alt: false,
            shift: false,
        };
        input.handle_key_event(&event);
        assert_eq!(input.text(), "ab");
    }

    #[test]
    fn test_input_clear_history() {
        let mut input = Input::new();
        input.handle_key(&Key::Char('a'));
        assert!(input.can_undo());

        input.clear_history();
        assert!(!input.can_undo());
        assert!(!input.can_redo());
    }

    // CSS integration tests
    #[test]
    fn test_input_css_id() {
        use crate::widget::View;

        let input = Input::new().element_id("email-input");
        assert_eq!(View::id(&input), Some("email-input"));

        let meta = input.meta();
        assert_eq!(meta.id, Some("email-input".to_string()));
    }

    #[test]
    fn test_input_css_classes() {
        let input = Input::new().class("form-control").class("required");

        assert!(input.has_class("form-control"));
        assert!(input.has_class("required"));
        assert!(!input.has_class("optional"));

        let meta = input.meta();
        assert!(meta.classes.contains("form-control"));
        assert!(meta.classes.contains("required"));
    }

    #[test]
    fn test_input_styled_view() {
        use crate::widget::View;

        let mut input = Input::new();

        input.set_id("test-input");
        assert_eq!(View::id(&input), Some("test-input"));

        input.add_class("focused");
        assert!(input.has_class("focused"));

        input.toggle_class("focused");
        assert!(!input.has_class("focused"));

        input.toggle_class("error");
        assert!(input.has_class("error"));

        input.remove_class("error");
        assert!(!input.has_class("error"));
    }

    #[test]
    fn test_input_css_colors_from_context() {
        use crate::style::{Style, VisualStyle};

        let input = Input::new().value("test");
        let mut buffer = Buffer::new(30, 3);
        let area = Rect::new(1, 1, 25, 1);

        let mut style = Style::default();
        style.visual = VisualStyle {
            color: Color::CYAN,
            background: Color::rgb(40, 40, 40),
            ..VisualStyle::default()
        };

        let mut ctx = RenderContext::with_style(&mut buffer, area, &style);
        input.render(&mut ctx);
        // Input should use CSS colors for non-cursor/non-selected text
    }
}
