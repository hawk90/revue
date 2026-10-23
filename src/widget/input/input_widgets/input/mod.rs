//! Text input widget with selection, clipboard, and undo/redo support
//!
//! The text, cursor, selection and undo are a single-line
//! [`EditState`], the edit model every text field
//! shares (#1261). The public methods give positions in CHARACTER indices,
//! as they always have; the cursor itself only stops between grapheme
//! clusters.

mod builders;
mod editing;
mod handler;
mod render;
mod selection;
mod utf8;

#[cfg(test)]
mod tests;

use crate::style::Color;
use crate::utils::EditState;
use crate::widget::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}

/// A text input widget with cursor, selection, clipboard, and undo/redo support
///
/// Editing is the shared [`EditState`] in its single-line form: the cursor
/// moves over grapheme clusters (an emoji sequence or a letter with its
/// accent is one step), word moves stop at runs of letters, punctuation and
/// spaces, and typing close together undoes as one step. Positions given by
/// the methods are character indices.
///
/// # Keyboard Shortcuts
///
/// The standard editing keys of [`EditState::edit_key`], plus:
///
/// | Key | Action |
/// |-----|--------|
/// | `Ctrl+C` | Copy the selection (with no selection the key is left to the app) |
/// | `Ctrl+X` | Cut the selection |
/// | `Ctrl+V` | Paste from the clipboard |
///
/// Among the standard keys: `Ctrl+A` selects all, `Ctrl+Z` undoes,
/// `Ctrl+Y` and `Ctrl+Shift+Z` redo, `Ctrl+←/→` move by word,
/// `Ctrl+Backspace/Delete` delete a word, and `Shift` with a move selects.
/// Enter, Up, Down, Tab and other Ctrl or Alt letters are left to the app.
#[derive(Clone, Debug)]
pub struct Input {
    /// The text, cursor, selection and undo
    state: EditState,
    placeholder: String,
    fg: Option<Color>,
    bg: Option<Color>,
    cursor_fg: Option<Color>,
    cursor_bg: Option<Color>,
    selection_bg: Option<Color>,
    focused: bool,
    /// Internal clipboard (also syncs with system clipboard if available)
    clipboard: Option<String>,
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
