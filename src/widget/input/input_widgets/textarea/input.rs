//! Keys: plain keys edit and move; modifier combinations open find and
//! replace, add cursors, undo, and select

use crate::event::{Key, KeyEvent};
use crate::utils::EditOutcome;
use crate::widget::traits::{EventResult, Interactive};

use super::TextArea;

impl TextArea {
    /// Handle a key without modifiers, as
    /// [`handle_key_event`](Self::handle_key_event) does
    pub fn handle_key(&mut self, key: &Key) -> bool {
        self.handle_key_event(&KeyEvent::new(*key))
    }

    /// Handle a key event, including modifier combinations.
    ///
    /// The TextArea's own keys come first; the others are the standard
    /// editing keys every text field shares (see
    /// [`EditState::edit_key`](crate::utils::EditState::edit_key)): typing,
    /// Enter, Backspace and Delete (with Ctrl, a word), moves by character,
    /// word (Ctrl), line and text (Ctrl+Home/End), Shift with any move to
    /// select, Ctrl+A, Ctrl+Z and Ctrl+Y or Ctrl+Shift+Z. Edits apply at every
    /// cursor.
    ///
    /// Bindings of its own:
    /// - `Tab` — type [`tab_width`](Self::tab_width) spaces
    /// - `PageUp` / `PageDown` — page the view and the cursor (with Shift,
    ///   select)
    /// - `Ctrl+Shift+K` — delete the lines the cursors touch
    /// - `Ctrl+F` — open the find panel
    /// - `Ctrl+H` — open the find/replace panel
    /// - `Ctrl+D` — select the next occurrence of the word/selection
    /// - `F3` / `Shift+F3` — jump to the next / previous match
    /// - `Ctrl+Alt+Up` / `Ctrl+Alt+Down` — add a cursor above / below
    /// - `Esc` — close the find panel, or collapse to a single cursor
    ///
    /// Returns `true` if the event was handled (a redraw may be needed).
    pub fn handle_key_event(&mut self, event: &KeyEvent) -> bool {
        if !self.focused {
            return false;
        }
        // A key ends a selection started by `start_selection`; Shift
        // selects on its own
        self.selecting = false;

        // Ctrl+Alt cursor stacking (multi-cursor).
        if event.ctrl && event.alt {
            match event.key {
                Key::Up => {
                    self.add_cursor_above();
                    return true;
                }
                Key::Down => {
                    self.add_cursor_below();
                    return true;
                }
                _ => {}
            }
        }

        // Ctrl combinations: find/replace, multi-cursor selection, undo and
        // line commands.
        if event.ctrl && !event.alt {
            match event.key {
                Key::Char('f') => {
                    self.open_find();
                    return true;
                }
                Key::Char('h') => {
                    self.open_replace();
                    return true;
                }
                Key::Char('d') => {
                    self.select_next_occurrence();
                    return true;
                }
                // Shift may come as the flag, as the capital, or both
                Key::Char('K') | Key::Char('k') if event.shift || event.key == Key::Char('K') => {
                    self.delete_line();
                    return true;
                }
                _ => {}
            }
        }

        // Find navigation (works whether or not the panel is focused).
        if event.key == Key::F(3) {
            if event.shift {
                self.find_previous();
            } else {
                self.find_next();
            }
            return true;
        }

        // Escape: close the find panel first, then collapse to a single cursor.
        if event.key == Key::Escape {
            if self.is_find_open() {
                self.close_find();
                return true;
            }
            if self.edit.cursors().len() > 1 {
                self.clear_secondary_cursors();
                return true;
            }
            return false;
        }

        match event.key {
            Key::Tab if !event.ctrl && !event.alt => {
                self.insert_char('\t');
                return true;
            }
            Key::PageUp | Key::PageDown if !event.ctrl && !event.alt => {
                let page = self.last_viewport_height.get().max(1);
                self.page(page, event.key == Key::PageDown, event.shift);
                return true;
            }
            _ => {}
        }

        // Ctrl or Alt with an unbound letter is left to the app
        let page = self.last_viewport_height.get().max(1);
        self.edit.edit_key(event, page) != EditOutcome::Unhandled
    }
}

impl Interactive for TextArea {
    fn handle_key(&mut self, event: &KeyEvent) -> EventResult {
        EventResult::from(self.handle_key_event(event))
    }

    fn on_focus(&mut self) {
        self.focused = true;
    }

    fn on_blur(&mut self) {
        self.focused = false;
    }
}
