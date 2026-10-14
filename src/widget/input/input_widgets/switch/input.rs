//! Keys and the mouse: Enter and Space toggle while focused, a left
//! click toggles, and the Interactive impl

use super::Switch;
use crate::event::{KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use crate::layout::Rect;
use crate::widget::traits::{EventResult, Interactive, ToggleWidget};

impl Switch {
    /// Handle key input
    pub fn handle_key(&mut self, key: &crate::event::Key) -> bool {
        use crate::event::Key;

        if self.disabled || !self.focused {
            return false;
        }

        match key {
            Key::Enter | Key::Char(' ') => {
                ToggleWidget::toggle(self);
                true
            }
            _ => false,
        }
    }
}

impl Interactive for Switch {
    fn handle_key(&mut self, event: &KeyEvent) -> EventResult {
        self.handle_toggle_key(event)
    }

    fn handle_mouse(&mut self, event: &MouseEvent, _area: Rect) -> EventResult {
        if self.disabled {
            return EventResult::Ignored;
        }

        match event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                ToggleWidget::toggle(self);
                EventResult::ConsumedAndRender
            }
            _ => EventResult::Ignored,
        }
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
