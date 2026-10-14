//! Keys: open, move, apply and close

use super::ThemePicker;
use crate::event::{Key, KeyEvent};
use crate::widget::traits::{EventResult, Interactive};

impl Interactive for ThemePicker {
    fn handle_key(&mut self, event: &KeyEvent) -> EventResult {
        match event.key {
            Key::Enter | Key::Char(' ') => {
                if self.open {
                    self.apply_selected();
                    self.close();
                } else {
                    self.open();
                }
                EventResult::Consumed
            }
            Key::Up | Key::Char('k') if self.open => {
                self.select_prev();
                EventResult::Consumed
            }
            Key::Down | Key::Char('j') if self.open => {
                self.select_next();
                EventResult::Consumed
            }
            Key::Escape if self.open => {
                self.close();
                EventResult::Consumed
            }
            Key::Tab => {
                // Apply next theme without opening dropdown
                self.select_next();
                if self.selected_index == 0 && !self.themes.is_empty() {
                    // Wrapped around, go to first
                }
                self.apply_selected();
                EventResult::Consumed
            }
            _ => EventResult::Ignored,
        }
    }
}
