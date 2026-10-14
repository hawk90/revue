use super::types::{PopoverArrow, PopoverPosition, PopoverStyle, PopoverTrigger};
use super::Popover;
use crate::style::Color;

impl Popover {
    /// Set the content text
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }

    /// Set the anchor point
    pub fn anchor(mut self, x: u16, y: u16) -> Self {
        self.anchor = (x, y);
        self
    }

    /// Set the position relative to anchor
    pub fn position(mut self, position: PopoverPosition) -> Self {
        self.position = position;
        self
    }

    /// Set the trigger type
    pub fn trigger(mut self, trigger: PopoverTrigger) -> Self {
        self.trigger = trigger;
        self
    }

    /// Set the visual style
    pub fn popover_style(mut self, style: PopoverStyle) -> Self {
        self.popover_style = style;
        self
    }

    /// Set the arrow style
    pub fn arrow(mut self, arrow: PopoverArrow) -> Self {
        self.arrow = arrow;
        self
    }

    /// Set whether the popover is open
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Set whether to close on escape key
    pub fn close_on_escape(mut self, close: bool) -> Self {
        self.close_on_escape = close;
        self
    }

    /// Set whether to close on click outside
    pub fn close_on_click_outside(mut self, close: bool) -> Self {
        self.close_on_click_outside = close;
        self
    }

    /// Set an optional title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set max width
    pub fn max_width(mut self, width: u16) -> Self {
        self.max_width = width;
        self
    }

    /// Set border color
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }
}
