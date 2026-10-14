//! Keys: arrows and vim keys along the orientation, Home and End

use super::{Slider, SliderOrientation};

impl Slider {
    /// Handle key input
    pub fn handle_key(&mut self, key: &crate::event::Key) -> bool {
        use crate::event::Key;

        if self.disabled || !self.focused {
            return false;
        }

        match (&self.orientation, key) {
            (SliderOrientation::Horizontal, Key::Right | Key::Char('l'))
            | (SliderOrientation::Vertical, Key::Up | Key::Char('k')) => {
                self.increment();
                true
            }
            (SliderOrientation::Horizontal, Key::Left | Key::Char('h'))
            | (SliderOrientation::Vertical, Key::Down | Key::Char('j')) => {
                self.decrement();
                true
            }
            (_, Key::Home) => {
                self.set_min();
                true
            }
            (_, Key::End) => {
                self.set_max();
                true
            }
            _ => false,
        }
    }
}
