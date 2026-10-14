//! Text input widget with selection, clipboard, and undo/redo support
//!
//! Note: All cursor and selection positions are in CHARACTER indices, not byte indices.
//! This ensures correct handling of multi-byte UTF-8 characters (emoji, CJK, etc).

mod builders;
mod editing;
mod handler;
mod render;
mod selection;
mod types;
mod undo;
mod utf8;

#[cfg(test)]
mod tests;

use crate::style::Color;
use crate::widget::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};
use types::EditOperation;

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
/// | `Ctrl+C` | Copy selection to clipboard (with no selection the key is left to the app) |
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
    /// Horizontal scroll in terminal COLUMNS, kept between renders so the
    /// view only moves when the cursor would leave it
    scroll_x: std::cell::Cell<usize>,
}

impl_styled_view!(Input);
impl_props_builders!(Input);

/// Helper function to create an input widget
pub fn input() -> Input {
    Input::new()
}
