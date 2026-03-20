//! Core types and enums for the Input widget

use crate::style::Color;
use crate::widget::WidgetProps;

/// Maximum undo history size
pub const MAX_UNDO_HISTORY: usize = 100;

/// An edit operation for undo/redo (single-line input)
#[derive(Clone, Debug)]
pub enum EditOperation {
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

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}

/// A text input widget with cursor, selection, clipboard, and undo/redo support
///
/// All cursor positions are character-based (not byte-based) to properly
/// handle UTF-8 multi-byte characters like emoji and CJK characters.
///
/// # Keyboard Shortcuts
///
/// ## Basic editing
///
/// | Key | Action |
/// |-----|--------|
/// | `Char` | Insert character at cursor (or replace selection) |
/// | `Backspace` | Delete character before cursor (or delete selection) |
/// | `Delete` | Delete character at cursor (or delete selection) |
/// | `Left` | Move cursor left (clears selection) |
/// | `Right` | Move cursor right (clears selection) |
/// | `Home` | Move cursor to start of input (clears selection) |
/// | `End` | Move cursor to end of input (clears selection) |
///
/// ## Ctrl combinations (via `handle_key_event`)
///
/// | Key | Action |
/// |-----|--------|
/// | `Ctrl+A` | Select all text |
/// | `Ctrl+C` | Copy selection to clipboard |
/// | `Ctrl+X` | Cut selection to clipboard |
/// | `Ctrl+V` | Paste from clipboard |
/// | `Ctrl+Z` | Undo last edit |
/// | `Ctrl+Y` | Redo last undone edit |
/// | `Ctrl+Left` | Move cursor to previous word |
/// | `Ctrl+Right` | Move cursor to next word |
/// | `Ctrl+Backspace` | Delete word to the left of cursor |
///
/// ## Shift combinations (via `handle_key_event`)
///
/// | Key | Action |
/// |-----|--------|
/// | `Shift+Left` | Extend selection one character to the left |
/// | `Shift+Right` | Extend selection one character to the right |
/// | `Shift+Home` | Extend selection to start of input |
/// | `Shift+End` | Extend selection to end of input |
#[derive(Clone, Debug)]
pub struct Input {
    pub(super) value: String,
    /// Cursor position in CHARACTER index (not byte index)
    pub(super) cursor: usize,
    /// Selection anchor in CHARACTER index (where selection started)
    pub(super) selection_anchor: Option<usize>,
    pub(super) placeholder: String,
    pub(super) fg: Option<Color>,
    pub(super) bg: Option<Color>,
    pub(super) cursor_fg: Option<Color>,
    pub(super) cursor_bg: Option<Color>,
    pub(super) selection_bg: Option<Color>,
    pub(super) focused: bool,
    /// Internal clipboard (also syncs with system clipboard if available)
    pub(super) clipboard: Option<String>,
    /// Undo history
    pub(super) undo_stack: Vec<EditOperation>,
    /// Redo history
    pub(super) redo_stack: Vec<EditOperation>,
    /// CSS styling properties (id, classes)
    pub(super) props: WidgetProps,
}

// KEEP HERE: All public API tests extracted to tests/widget/input/input_types.rs
