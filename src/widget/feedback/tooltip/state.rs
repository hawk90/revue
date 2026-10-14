//! Visibility, the show delay, the anchor, and the getters

use super::{Tooltip, TooltipArrow, TooltipPosition, TooltipStyle};

impl Tooltip {
    /// Show the tooltip
    pub fn show(&mut self) {
        self.visible = true;
        self.delay_counter = 0;
    }

    /// Hide the tooltip
    pub fn hide(&mut self) {
        self.visible = false;
    }

    /// Toggle visibility
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    /// Check if visible
    pub fn is_visible(&self) -> bool {
        self.visible && self.delay_counter >= self.delay
    }

    /// Tick for delay animation
    pub fn tick(&mut self) {
        if self.delay_counter < self.delay {
            self.delay_counter += 1;
        }
    }

    /// Set anchor position
    pub fn set_anchor(&mut self, x: u16, y: u16) {
        self.anchor = (x, y);
    }

    // Getters for testing
    #[doc(hidden)]
    pub fn get_text(&self) -> &str {
        &self.text
    }

    #[doc(hidden)]
    pub fn get_position(&self) -> TooltipPosition {
        self.position
    }

    #[doc(hidden)]
    pub fn get_anchor(&self) -> (u16, u16) {
        self.anchor
    }

    #[doc(hidden)]
    pub fn get_style(&self) -> TooltipStyle {
        self.style
    }

    #[doc(hidden)]
    pub fn get_arrow(&self) -> TooltipArrow {
        self.arrow
    }

    #[doc(hidden)]
    pub fn get_max_width(&self) -> u16 {
        self.max_width
    }

    #[doc(hidden)]
    pub fn get_delay(&self) -> u16 {
        self.delay
    }

    #[doc(hidden)]
    pub fn get_delay_counter(&self) -> u16 {
        self.delay_counter
    }

    #[doc(hidden)]
    pub fn get_title(&self) -> Option<&str> {
        self.title.as_deref()
    }
}
