//! Ctrl+C, Ctrl+X and Ctrl+V for every field that edits with [`EditState`]
//!
//! The system clipboard with the `clipboard` feature, and always a copy kept
//! on the thread too, so text still moves between fields where there is no system
//! clipboard to reach.

use std::cell::RefCell;

use super::{EditOutcome, EditState};
use crate::event::{Key, KeyEvent};

thread_local! {
    /// What was last copied or cut on this thread: the one the app's keys come
    /// in on
    static LAST_COPIED: RefCell<Option<String>> = const { RefCell::new(None) };
}

/// Put `text` on the clipboard.
pub(crate) fn set_clipboard_text(text: &str) {
    LAST_COPIED.with(|last| *last.borrow_mut() = Some(text.to_string()));
    #[cfg(feature = "clipboard")]
    if let Ok(mut clipboard) = arboard::Clipboard::new() {
        let _ = clipboard.set_text(text);
    }
}

/// The clipboard's text: the system's if it has any, else what was last copied
/// here.
pub(crate) fn clipboard_text() -> Option<String> {
    #[cfg(feature = "clipboard")]
    if let Ok(mut clipboard) = arboard::Clipboard::new() {
        if let Ok(text) = clipboard.get_text() {
            if !text.is_empty() {
                return Some(text);
            }
        }
    }
    LAST_COPIED.with(|last| last.borrow().clone())
}

impl EditState {
    /// Copy what every cursor selects, in text order, one per line; returns
    /// whether anything was selected.
    pub fn copy_selection(&self) -> bool {
        if !self.has_selection() {
            return false;
        }
        set_clipboard_text(&self.selected_text());
        true
    }

    /// Copy what every cursor selects and delete it; returns whether the text
    /// changed.
    pub fn cut_selection(&mut self) -> bool {
        self.copy_selection() && self.delete_selection()
    }

    /// Replace the selection with the clipboard's text, as one undo step;
    /// returns whether the text changed.
    pub fn paste_clipboard(&mut self) -> bool {
        clipboard_text().is_some_and(|text| self.paste(&text))
    }

    /// Ctrl+C with a selection, Ctrl+X and Ctrl+V. Anything else, and Ctrl+C with
    /// nothing selected, is left alone: it quits an app by default.
    pub(super) fn clipboard_key(&mut self, key: &KeyEvent) -> Option<EditOutcome> {
        if !key.ctrl || key.alt {
            return None;
        }
        match key.key {
            Key::Char('c') if self.copy_selection() => Some(EditOutcome::Moved),
            Key::Char('x') if self.cut_selection() => Some(EditOutcome::Edited),
            Key::Char('v') if self.paste_clipboard() => Some(EditOutcome::Edited),
            _ => None,
        }
    }
}
