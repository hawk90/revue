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
