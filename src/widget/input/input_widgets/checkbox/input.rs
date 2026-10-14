//! Keys: Enter and Space toggle an enabled checkbox; the Interactive impl

use super::Checkbox;
use crate::event::{Key, KeyEvent};
use crate::widget::traits::{EventResult, Interactive, ToggleWidget};

impl Checkbox {
    /// Handle key input, returns true if state changed
    pub fn handle_key(&mut self, key: &Key) -> bool {
        if self.state.disabled {
            return false;
        }

        if matches!(key, Key::Enter | Key::Char(' ')) {
            ToggleWidget::toggle(self);
            true
        } else {
            false
        }
    }
}

impl Interactive for Checkbox {
    fn handle_key(&mut self, event: &KeyEvent) -> EventResult {
        self.handle_toggle_key(event)
    }

    fn focusable(&self) -> bool {
        self.toggle_focusable()
    }

    fn on_focus(&mut self) {
        self.set_toggle_focused(true);
    }

    fn on_blur(&mut self) {
        self.set_toggle_focused(false);
    }
}
