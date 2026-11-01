//! Editing the value: inserting, deleting, moving the cursor and the peek countdown

use super::{MaskStyle, MaskedInput};
use crate::event::{Key, KeyEvent};
use crate::utils::{EditMotion, EditOutcome};
use crate::widget::traits::{EventResult, Interactive};

impl MaskedInput {
    /// Handle a key without modifiers, as
    /// [`handle_key_event`](Self::handle_key_event) does
    pub fn handle_key(&mut self, key: &Key) -> bool {
        self.handle_key_event(&KeyEvent::new(*key))
    }

    /// Handle a key: the standard editing keys every single-line field
    /// shares (#985), typing only up to `max_length`, and the word keys
    /// going to the start or end of the value (see the module docs).
    /// Returns whether the key was the field's; a disabled input handles no
    /// keys.
    pub fn handle_key_event(&mut self, event: &KeyEvent) -> bool {
        if self.disabled {
            return false;
        }
        let typing =
            matches!(event.key, Key::Char(c) if event.ctrl == event.alt && !c.is_control());
        if typing && self.is_full() {
            return true;
        }
        // Masked, Backspace takes a whole character: taking an accent alone
        // would change nothing the mask shows
        if event.key == Key::Backspace && !event.ctrl && !event.alt && self.masks() {
            self.delete_backward();
            return true;
        }
        if event.ctrl && !event.alt {
            let whole = match event.key {
                Key::Left => Some(EditMotion::LineStart),
                Key::Right => Some(EditMotion::LineEnd),
                Key::Backspace => Some(EditMotion::LineStart),
                Key::Delete => Some(EditMotion::LineEnd),
                _ => None,
            };
            if let Some(motion) = whole {
                self.end_peek();
                if matches!(event.key, Key::Backspace | Key::Delete) {
                    // What a selection holds, else up to the start or end
                    if !self.state.has_selection() {
                        self.state.move_cursors(motion, true);
                    }
                    self.state.delete_selection();
                } else {
                    self.state.move_cursors(motion, event.shift);
                }
                return true;
            }
        }
        match self.state.edit_key(event, 1) {
            EditOutcome::Edited if typing => self.start_peek(),
            EditOutcome::Unhandled => return false,
            _ => self.end_peek(),
        }
        true
    }

    /// Insert character at cursor (up to `max_length`)
    pub fn insert_char(&mut self, c: char) {
        if self.disabled || self.is_full() {
            return;
        }
        if self.state.insert(c.encode_utf8(&mut [0; 4])) {
            self.start_peek();
        }
    }

    /// Delete the character before the cursor (while masked, the whole
    /// character as seen; shown, as Backspace does in any field)
    pub fn delete_backward(&mut self) {
        self.end_peek();
        if self.disabled {
            return;
        }
        if self.masks() && !self.state.has_selection() {
            self.state.move_cursors(EditMotion::Left, true);
            self.state.delete_selection();
        } else {
            self.state.backspace();
        }
    }

    /// Whether the value is shown masked
    fn masks(&self) -> bool {
        !self.revealing
    }

    /// Delete character at cursor
    pub fn delete_forward(&mut self) {
        self.end_peek();
        if !self.disabled {
            self.state.delete();
        }
    }

    /// Move cursor left
    pub fn move_left(&mut self) {
        self.move_to(EditMotion::Left);
    }

    /// Move cursor right
    pub fn move_right(&mut self) {
        self.move_to(EditMotion::Right);
    }

    /// Move cursor to start
    pub fn move_start(&mut self) {
        self.move_to(EditMotion::LineStart);
    }

    /// Move cursor to end
    pub fn move_end(&mut self) {
        self.move_to(EditMotion::LineEnd);
    }

    fn move_to(&mut self, motion: EditMotion) {
        self.end_peek();
        self.state.move_cursors(motion, false);
    }

    /// Whether typing a char would go past `max_length`: the value is at
    /// it and nothing selected would make room
    fn is_full(&self) -> bool {
        if self.max_length == 0 {
            return false;
        }
        let (from, to) = self.state.primary().range();
        self.len() - (to.col - from.col) >= self.max_length
    }

    /// Show the char just typed for a while (the peek style)
    fn start_peek(&mut self) {
        if matches!(self.mask_style, MaskStyle::Peek) {
            self.peek_countdown = self.peek_timeout;
        }
    }

    /// End the peek: it shows the char before the cursor, which is the one
    /// just typed only until the cursor moves or the value changes, after
    /// which it would reveal another char of the secret (#1073)
    pub(super) fn end_peek(&mut self) {
        self.peek_countdown = 0;
    }

    /// Update (call each frame for peek mode)
    pub fn update(&mut self) {
        if self.peek_countdown > 0 {
            self.peek_countdown -= 1;
        }
    }
}

/// The runtime does not route keys to widgets: an app forwards them, and this
/// is the same call every interactive widget answers.
impl Interactive for MaskedInput {
    fn handle_key(&mut self, event: &KeyEvent) -> EventResult {
        if self.handle_key_event(event) {
            EventResult::ConsumedAndRender
        } else {
            EventResult::Ignored
        }
    }

    fn focusable(&self) -> bool {
        !self.disabled
    }

    fn on_focus(&mut self) {
        self.focused = true;
    }

    fn on_blur(&mut self) {
        self.focused = false;
    }
}
