//! Tooltip widget for displaying contextual information
//!
//! Provides hover-style tooltips and help text displays.

mod builders;
mod layout;
mod render;
mod state;
mod types;

pub use types::{TooltipArrow, TooltipPosition, TooltipStyle};

use crate::style::Color;
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// Tooltip widget
#[derive(Clone)]
pub struct Tooltip {
    /// Tooltip text (supports multiple lines)
    text: String,
    /// Position relative to anchor
    position: TooltipPosition,
    /// Anchor point (x, y)
    anchor: (u16, u16),
    /// Visual style
    style: TooltipStyle,
    /// Arrow style
    arrow: TooltipArrow,
    /// Max width (0 = auto)
    max_width: u16,
    /// Visible
    visible: bool,
    /// Custom colors
    fg: Option<Color>,
    bg: Option<Color>,
    /// Title (optional)
    title: Option<String>,
    /// Show delay in frames (for animated appearance)
    delay: u16,
    /// Current delay counter
    delay_counter: u16,
    /// Widget properties
    props: WidgetProps,
}

impl Tooltip {
    /// Create a new tooltip
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            position: TooltipPosition::Top,
            anchor: (0, 0),
            style: TooltipStyle::Bordered,
            arrow: TooltipArrow::Unicode,
            max_width: 40,
            visible: true,
            fg: None,
            bg: None,
            title: None,
            delay: 0,
            delay_counter: 0,
            props: WidgetProps::new(),
        }
    }
}

impl Default for Tooltip {
    fn default() -> Self {
        Self::new("")
    }
}

impl_styled_view!(Tooltip);
impl_props_builders!(Tooltip);

/// Helper to create a tooltip
pub fn tooltip(text: impl Into<String>) -> Tooltip {
    Tooltip::new(text)
}

// KEEP HERE - accesses private fields
// Tests for private methods that cannot be extracted

#[cfg(test)]
mod tests {
    use super::*;

    // These tests access private methods and must stay inline

    #[test]
    fn test_tooltip_wrap_text() {
        let t = Tooltip::new("This is a very long text that should be wrapped").max_width(20);
        let lines = t.wrap_text();
        assert!(lines.len() > 1);
        assert!(lines.iter().all(|l| l.len() <= 20));
    }

    #[test]
    fn test_tooltip_calculate_dimensions() {
        let t = Tooltip::new("Short").style(TooltipStyle::Bordered);
        let (w, h) = t.calculate_dimensions();
        assert!(w > 5);
        assert!(h >= 3); // At least border + 1 line
    }

    #[test]
    fn test_tooltip_with_title() {
        let t = Tooltip::new("Content")
            .title("Title")
            .style(TooltipStyle::Bordered);

        let (_, h) = t.calculate_dimensions();
        assert!(h >= 4); // border + title + content
    }

    #[test]
    fn test_tooltip_auto_position() {
        let t = Tooltip::new("Test")
            .position(TooltipPosition::Auto)
            .anchor(5, 5);

        let (_, _, pos) = t.calculate_position(40, 20);
        // Should choose a valid position
        assert!(!matches!(pos, TooltipPosition::Auto));
    }
}
