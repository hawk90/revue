//! Builder methods and the preset constructors

use super::{Tooltip, TooltipArrow, TooltipPosition, TooltipStyle};
use crate::style::Color;

impl Tooltip {
    /// Set tooltip text
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    /// Set position
    pub fn position(mut self, position: TooltipPosition) -> Self {
        self.position = position;
        self
    }

    /// Set anchor point
    pub fn anchor(mut self, x: u16, y: u16) -> Self {
        self.anchor = (x, y);
        self
    }

    /// Set style
    pub fn style(mut self, style: TooltipStyle) -> Self {
        self.style = style;
        self
    }

    /// Set arrow style
    pub fn arrow(mut self, arrow: TooltipArrow) -> Self {
        self.arrow = arrow;
        self
    }

    /// Set max width
    pub fn max_width(mut self, width: u16) -> Self {
        self.max_width = width;
        self
    }

    /// Set visibility
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Set foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Set title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set show delay
    pub fn delay(mut self, frames: u16) -> Self {
        self.delay = frames;
        self
    }

    // Preset styles

    /// Create info tooltip
    pub fn info(text: impl Into<String>) -> Self {
        Self::new(text).style(TooltipStyle::Info)
    }

    /// Create warning tooltip
    pub fn warning(text: impl Into<String>) -> Self {
        Self::new(text).style(TooltipStyle::Warning)
    }

    /// Create error tooltip
    pub fn error(text: impl Into<String>) -> Self {
        Self::new(text).style(TooltipStyle::Error)
    }

    /// Create success tooltip
    pub fn success(text: impl Into<String>) -> Self {
        Self::new(text).style(TooltipStyle::Success)
    }
}
