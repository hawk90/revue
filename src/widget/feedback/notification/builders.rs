//! NotificationCenter settings

use super::{NotificationCenter, NotificationPosition};

impl NotificationCenter {
    /// Set position
    pub fn position(mut self, position: NotificationPosition) -> Self {
        self.position = position;
        self
    }

    /// Set max visible
    pub fn max_visible(mut self, max: usize) -> Self {
        self.max_visible = max.max(1);
        self
    }

    /// Set width
    pub fn width(mut self, width: u16) -> Self {
        self.width = width.max(20);
        self
    }

    /// Show/hide icons
    pub fn show_icons(mut self, show: bool) -> Self {
        self.show_icons = show;
        self
    }

    /// Show/hide timer
    pub fn show_timer(mut self, show: bool) -> Self {
        self.show_timer = show;
        self
    }

    /// Set spacing
    pub fn spacing(mut self, spacing: u16) -> Self {
        self.spacing = spacing;
        self
    }

    /// Set focused state
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }
}
