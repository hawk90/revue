use super::types::PopoverTrigger;
use super::Popover;
use crate::event::Key;

impl Popover {
    /// Open the popover
    pub fn show(&mut self) {
        self.open = true;
    }

    /// Close the popover
    pub fn hide(&mut self) {
        self.open = false;
    }

    /// Toggle the popover
    pub fn toggle(&mut self) {
        self.open = !self.open;
    }

    /// Check if the popover is open
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Set anchor position
    pub fn set_anchor(&mut self, x: u16, y: u16) {
        self.anchor = (x, y);
    }

    /// Handle keyboard input
    ///
    /// Returns `true` if the key was handled.
    pub fn handle_key(&mut self, key: &Key) -> bool {
        if !self.open {
            return false;
        }

        if self.close_on_escape && matches!(key, Key::Escape) {
            self.hide();
            return true;
        }

        false
    }

    /// Handle click at position
    ///
    /// Returns `true` if the click was handled.
    pub fn handle_click(&mut self, x: u16, y: u16, area_width: u16, area_height: u16) -> bool {
        if !self.open {
            // Check if click is on anchor (toggle open)
            if matches!(self.trigger, PopoverTrigger::Click)
                && x == self.anchor.0
                && y == self.anchor.1
            {
                self.show();
                return true;
            }
            return false;
        }

        // Check if click is inside popover
        let (popup_x, popup_y, _) = self.calculate_position(area_width, area_height);
        let (popup_w, popup_h) = self.calculate_dimensions(area_width);

        let inside = x >= popup_x && x < popup_x + popup_w && y >= popup_y && y < popup_y + popup_h;

        if inside {
            // Click inside popover - could be handled by content
            true
        } else if self.close_on_click_outside {
            self.hide();
            true
        } else {
            false
        }
    }
}
