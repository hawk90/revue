//! Code editor key handling

use crate::event::{Key, KeyEvent};
use crate::utils::EditOutcome;

impl super::CodeEditor {
    // =========================================================================
    // Key Handling
    // =========================================================================

    /// Handle a key without modifiers, as
    /// [`handle_key_event`](Self::handle_key_event) does
    pub fn handle_key(&mut self, key: &Key) -> bool {
        self.handle_key_event(&KeyEvent::new(*key))
    }

    /// Handle a key with its modifiers. The go-to-line and find boxes take
    /// keys first while open. Then the editor's own keys:
    /// - a character: typed, with its closing bracket or quote after it
    ///   when bracket matching is on
    /// - `Enter`: a new line, auto-indented
    /// - `Tab`: one indent
    /// - `Home`: the first non-blank character, then the line's start (with
    ///   Shift, select)
    /// - `PageUp` / `PageDown`: scroll the view and the cursor a page (with
    ///   Shift, select)
    ///
    /// The others are the standard editing keys every text field shares
    /// (see [`EditState::edit_key`](crate::utils::EditState::edit_key)):
    /// Backspace and Delete (with Ctrl, a word), moves by character, word
    /// (Ctrl), line and text (Ctrl+Home/End), Shift with any move to select,
    /// Ctrl+A, Ctrl+Z and Ctrl+Y or Ctrl+Shift+Z. An unbound Ctrl or Alt
    /// letter is left to the app. Returns whether the key was the editor's.
    pub fn handle_key_event(&mut self, event: &KeyEvent) -> bool {
        // Handle modal modes first
        if self.goto_line_mode {
            return self.handle_goto_input(event);
        }
        if self.find_mode {
            return self.handle_find_input(event);
        }
        // In selection mode the moving keys extend the selection, as with
        // Shift
        let moving = matches!(
            event.key,
            Key::Left
                | Key::Right
                | Key::Up
                | Key::Down
                | Key::Home
                | Key::End
                | Key::PageUp
                | Key::PageDown
        );
        let mut event = event.clone();
        if self.selecting && moving {
            event.shift = true;
        }
        let event = &event;

        let plain = !event.ctrl && !event.alt;
        match event.key {
            // Ctrl+Alt with a letter is text, as Windows reports AltGr
            Key::Char(ch) if event.ctrl == event.alt && !ch.is_control() => {
                self.insert_char(ch);
                return true;
            }
            Key::Enter if plain => {
                self.insert_char('\n');
                return true;
            }
            Key::Tab if plain => {
                self.insert_char('\t');
                return true;
            }
            Key::Home if plain => {
                self.smart_home(event.shift);
                return true;
            }
            Key::PageUp | Key::PageDown if plain => {
                let page = self.page_height.get().max(1);
                self.page(page, event.key == Key::PageDown, event.shift);
                return true;
            }
            _ => {}
        }

        let page = self.page_height.get().max(1);
        let outcome = self.edit.edit_key(event, page);
        if outcome == EditOutcome::Edited {
            // An edit ends selection mode
            self.selecting = false;
            self.text_changed();
        } else {
            self.ensure_cursor_visible();
        }
        outcome != EditOutcome::Unhandled
    }
}
