//! Popover widget for anchor-positioned interactive overlays
//!
//! Unlike Tooltip (hover-only), Popover supports click triggers, focus trapping,
//! and interactive content. Essential for DatePicker, Combobox, etc.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{Popover, PopoverPosition, popover};
//!
//! // Basic popover
//! Popover::new("Click me for details")
//!     .anchor(10, 5)
//!     .position(PopoverPosition::Bottom);
//!
//! // Interactive popover with trigger
//! popover("Menu content")
//!     .trigger(PopoverTrigger::Click)
//!     .close_on_escape(true)
//!     .close_on_click_outside(true);
//! ```

use super::traits::{RenderContext, View, WidgetProps, WidgetState};
use crate::event::Key;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::border::BorderChars;
use crate::{impl_styled_view, impl_widget_builders};

/// Popover position relative to anchor
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PopoverPosition {
    /// Above the anchor
    Top,
    /// Below the anchor
    #[default]
    Bottom,
    /// To the left of anchor
    Left,
    /// To the right of anchor
    Right,
    /// Auto-detect best position
    Auto,
}

/// Popover trigger type
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PopoverTrigger {
    /// Triggered by click
    #[default]
    Click,
    /// Triggered by hover
    Hover,
    /// Triggered by focus
    Focus,
    /// Manually controlled (no automatic trigger)
    Manual,
}

/// Popover arrow style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PopoverArrow {
    /// No arrow
    #[default]
    None,
    /// Simple ASCII arrow
    Simple,
    /// Unicode arrow
    Unicode,
}

impl PopoverArrow {
    fn chars(&self, position: PopoverPosition) -> char {
        match (self, position) {
            (PopoverArrow::None, _) => ' ',
            (PopoverArrow::Simple, PopoverPosition::Top) => 'v',
            (PopoverArrow::Simple, PopoverPosition::Bottom) => '^',
            (PopoverArrow::Simple, PopoverPosition::Left) => '>',
            (PopoverArrow::Simple, PopoverPosition::Right) => '<',
            (PopoverArrow::Simple, PopoverPosition::Auto) => 'v',
            (PopoverArrow::Unicode, PopoverPosition::Top) => '▼',
            (PopoverArrow::Unicode, PopoverPosition::Bottom) => '▲',
            (PopoverArrow::Unicode, PopoverPosition::Left) => '▶',
            (PopoverArrow::Unicode, PopoverPosition::Right) => '◀',
            (PopoverArrow::Unicode, PopoverPosition::Auto) => '▼',
        }
    }
}

/// Popover visual style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PopoverStyle {
    /// Default bordered style
    #[default]
    Default,
    /// Rounded corners
    Rounded,
    /// Minimal without border
    Minimal,
    /// Elevated with shadow effect
    Elevated,
}

impl PopoverStyle {
    fn colors(&self) -> (Color, Color, Color) {
        // (fg, bg, border)
        match self {
            PopoverStyle::Default => (Color::WHITE, Color::rgb(30, 30, 35), Color::rgb(70, 70, 80)),
            PopoverStyle::Rounded => (Color::WHITE, Color::rgb(35, 35, 40), Color::rgb(80, 80, 90)),
            PopoverStyle::Minimal => (Color::WHITE, Color::rgb(40, 40, 45), Color::rgb(40, 40, 45)),
            PopoverStyle::Elevated => {
                (Color::WHITE, Color::rgb(25, 25, 30), Color::rgb(60, 60, 70))
            }
        }
    }

    fn border_chars(&self) -> Option<BorderChars> {
        match self {
            PopoverStyle::Minimal => None,
            PopoverStyle::Rounded => Some(BorderChars::ROUNDED),
            _ => Some(BorderChars::SINGLE),
        }
    }
}

/// A popover widget for interactive overlays
///
/// Popovers are positioned relative to an anchor point and can contain
/// interactive content. They support various triggers and can be configured
/// to close on escape or click outside.
pub struct Popover {
    /// Content text (for simple popovers)
    content: String,
    /// Anchor point (x, y)
    anchor: (u16, u16),
    /// Position relative to anchor
    position: PopoverPosition,
    /// Trigger type
    trigger: PopoverTrigger,
    /// Visual style
    popover_style: PopoverStyle,
    /// Arrow style
    arrow: PopoverArrow,
    /// Whether popover is open
    open: bool,
    /// Close on escape key
    close_on_escape: bool,
    /// Close when clicking outside
    close_on_click_outside: bool,
    /// Title (optional)
    title: Option<String>,
    /// Max width (0 = auto)
    max_width: u16,
    /// Custom border color
    border_color: Option<Color>,
    /// Widget state
    state: WidgetState,
    /// Widget properties
    props: WidgetProps,
}

impl Popover {
    /// Create a new popover with content
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            anchor: (0, 0),
            position: PopoverPosition::Bottom,
            trigger: PopoverTrigger::Click,
            popover_style: PopoverStyle::Default,
            arrow: PopoverArrow::None,
            open: false,
            close_on_escape: true,
            close_on_click_outside: true,
            title: None,
            max_width: 40,
            border_color: None,
            state: WidgetState::new(),
            props: WidgetProps::new(),
        }
    }

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

    // State management

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
        let (popup_w, popup_h) = self.calculate_dimensions();

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

    /// Word wrap content
    fn wrap_content(&self) -> Vec<String> {
        let max_width = if self.max_width > 0 {
            self.max_width as usize
        } else {
            40
        };

        let mut lines = Vec::new();
        for line in self.content.lines() {
            if line.len() <= max_width {
                lines.push(line.to_string());
            } else {
                let mut current_line = String::new();
                for word in line.split_whitespace() {
                    if current_line.is_empty() {
                        current_line = word.to_string();
                    } else if current_line.len() + 1 + word.len() <= max_width {
                        current_line.push(' ');
                        current_line.push_str(word);
                    } else {
                        lines.push(current_line);
                        current_line = word.to_string();
                    }
                }
                if !current_line.is_empty() {
                    lines.push(current_line);
                }
            }
        }

        if lines.is_empty() {
            lines.push(String::new());
        }

        lines
    }

    /// Calculate popover dimensions
    fn calculate_dimensions(&self) -> (u16, u16) {
        let lines = self.wrap_content();
        let has_border = self.popover_style.border_chars().is_some();
        let has_title = self.title.is_some();

        let content_width = lines.iter().map(|l| l.len()).max().unwrap_or(0) as u16;
        let title_width = self.title.as_ref().map(|t| t.len() as u16 + 2).unwrap_or(0);
        let text_width = content_width.max(title_width);

        let width = text_width + if has_border { 4 } else { 2 };
        let height = lines.len() as u16
            + if has_border { 2 } else { 0 }
            + if has_title && has_border { 1 } else { 0 };

        (width.max(10), height.max(3))
    }

    /// Calculate position based on anchor and available space
    fn calculate_position(&self, area_width: u16, area_height: u16) -> (u16, u16, PopoverPosition) {
        let (popup_w, popup_h) = self.calculate_dimensions();
        let (anchor_x, anchor_y) = self.anchor;
        let arrow_offset: u16 = if matches!(self.arrow, PopoverArrow::None) {
            0
        } else {
            1
        };

        let position = if matches!(self.position, PopoverPosition::Auto) {
            let space_above = anchor_y;
            let space_below = area_height.saturating_sub(anchor_y + 1);
            let space_left = anchor_x;
            let space_right = area_width.saturating_sub(anchor_x + 1);

            if space_below >= popup_h + arrow_offset {
                PopoverPosition::Bottom
            } else if space_above >= popup_h + arrow_offset {
                PopoverPosition::Top
            } else if space_right >= popup_w + arrow_offset {
                PopoverPosition::Right
            } else if space_left >= popup_w + arrow_offset {
                PopoverPosition::Left
            } else {
                PopoverPosition::Bottom
            }
        } else {
            self.position
        };

        let (x, y) = match position {
            PopoverPosition::Top => {
                let x = anchor_x.saturating_sub(popup_w / 2);
                let y = anchor_y.saturating_sub(popup_h + arrow_offset);
                (x, y)
            }
            PopoverPosition::Bottom => {
                let x = anchor_x.saturating_sub(popup_w / 2);
                let y = anchor_y + 1 + arrow_offset;
                (x, y)
            }
            PopoverPosition::Left => {
                let x = anchor_x.saturating_sub(popup_w + arrow_offset);
                let y = anchor_y.saturating_sub(popup_h / 2);
                (x, y)
            }
            PopoverPosition::Right => {
                let x = anchor_x + 1 + arrow_offset;
                let y = anchor_y.saturating_sub(popup_h / 2);
                (x, y)
            }
            PopoverPosition::Auto => unreachable!(),
        };

        let x = x.min(area_width.saturating_sub(popup_w));
        let y = y.min(area_height.saturating_sub(popup_h));

        (x, y, position)
    }
}

impl Default for Popover {
    fn default() -> Self {
        Self::new("")
    }
}

impl View for Popover {
    crate::impl_view_meta!("Popover");

    fn render(&self, ctx: &mut RenderContext) {
        if !self.open {
            return;
        }

        let area = ctx.area;
        let (popup_w, popup_h) = self.calculate_dimensions();
        let (popup_x, popup_y, actual_position) = self.calculate_position(area.width, area.height);

        // Get colors
        let (default_fg, default_bg, default_border) = self.popover_style.colors();
        let fg = self.state.fg.unwrap_or(default_fg);
        let bg = self.state.bg.unwrap_or(default_bg);
        let border_fg = self.border_color.unwrap_or(default_border);

        // Draw shadow for elevated style
        if matches!(self.popover_style, PopoverStyle::Elevated) {
            for dy in 1..=popup_h {
                for dx in 1..=popup_w {
                    let x = popup_x + dx;
                    let y = popup_y + dy;
                    if x < area.width && y < area.height {
                        let mut cell = Cell::new(' ');
                        cell.bg = Some(Color::rgb(15, 15, 15));
                        ctx.buffer.set(x, y, cell);
                    }
                }
            }
        }

        // Draw background
        for dy in 0..popup_h {
            for dx in 0..popup_w {
                let x = popup_x + dx;
                let y = popup_y + dy;
                if x < area.width && y < area.height {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(bg);
                    ctx.buffer.set(x, y, cell);
                }
            }
        }

        // Draw border
        let content_start_x;
        let content_start_y;

        if let Some(border) = self.popover_style.border_chars() {
            content_start_x = popup_x + 2;
            content_start_y = popup_y + 1;

            // Top border
            if popup_y < area.height {
                let mut tl = Cell::new(border.top_left);
                tl.fg = Some(border_fg);
                tl.bg = Some(bg);
                ctx.buffer.set(popup_x, popup_y, tl);

                for dx in 1..popup_w - 1 {
                    let mut h = Cell::new(border.horizontal);
                    h.fg = Some(border_fg);
                    h.bg = Some(bg);
                    ctx.buffer.set(popup_x + dx, popup_y, h);
                }

                let mut tr = Cell::new(border.top_right);
                tr.fg = Some(border_fg);
                tr.bg = Some(bg);
                ctx.buffer.set(popup_x + popup_w - 1, popup_y, tr);
            }

            // Title if present
            if let Some(ref title) = self.title {
                let title_x = popup_x + 2;
                let title_y = popup_y + 1;
                for (i, ch) in title.chars().enumerate() {
                    let x = title_x + i as u16;
                    if x < popup_x + popup_w - 2 && x < area.width {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(fg);
                        cell.bg = Some(bg);
                        cell.modifier |= Modifier::BOLD;
                        ctx.buffer.set(x, title_y, cell);
                    }
                }
            }

            // Side borders
            for dy in 1..popup_h - 1 {
                let y = popup_y + dy;
                if y < area.height {
                    let mut left = Cell::new(border.vertical);
                    left.fg = Some(border_fg);
                    left.bg = Some(bg);
                    ctx.buffer.set(popup_x, y, left);

                    let mut right = Cell::new(border.vertical);
                    right.fg = Some(border_fg);
                    right.bg = Some(bg);
                    ctx.buffer.set(popup_x + popup_w - 1, y, right);
                }
            }

            // Bottom border
            let bottom_y = popup_y + popup_h - 1;
            if bottom_y < area.height {
                let mut bl = Cell::new(border.bottom_left);
                bl.fg = Some(border_fg);
                bl.bg = Some(bg);
                ctx.buffer.set(popup_x, bottom_y, bl);

                for dx in 1..popup_w - 1 {
                    let mut h = Cell::new(border.horizontal);
                    h.fg = Some(border_fg);
                    h.bg = Some(bg);
                    ctx.buffer.set(popup_x + dx, bottom_y, h);
                }

                let mut br = Cell::new(border.bottom_right);
                br.fg = Some(border_fg);
                br.bg = Some(bg);
                ctx.buffer.set(popup_x + popup_w - 1, bottom_y, br);
            }
        } else {
            content_start_x = popup_x + 1;
            content_start_y = popup_y;
        }

        // Draw content
        let lines = self.wrap_content();
        let text_y_offset = if self.title.is_some() && self.popover_style.border_chars().is_some() {
            1
        } else {
            0
        };

        for (i, line) in lines.iter().enumerate() {
            let y = content_start_y + text_y_offset + i as u16;
            if y >= area.height || y >= popup_y + popup_h - 1 {
                break;
            }

            for (j, ch) in line.chars().enumerate() {
                let x = content_start_x + j as u16;
                if x < popup_x + popup_w - 1 && x < area.width {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(fg);
                    cell.bg = Some(bg);
                    ctx.buffer.set(x, y, cell);
                }
            }
        }

        // Draw arrow
        if !matches!(self.arrow, PopoverArrow::None) {
            let arrow_char = self.arrow.chars(actual_position);
            let (arrow_x, arrow_y) = match actual_position {
                PopoverPosition::Top => (self.anchor.0, popup_y + popup_h),
                PopoverPosition::Bottom => (self.anchor.0, popup_y.saturating_sub(1)),
                PopoverPosition::Left => (popup_x + popup_w, self.anchor.1),
                PopoverPosition::Right => (popup_x.saturating_sub(1), self.anchor.1),
                PopoverPosition::Auto => unreachable!(),
            };

            if arrow_x < area.width && arrow_y < area.height {
                let mut cell = Cell::new(arrow_char);
                cell.fg = Some(border_fg);
                ctx.buffer.set(arrow_x, arrow_y, cell);
            }
        }
    }
}

impl_styled_view!(Popover);
impl_widget_builders!(Popover);

/// Helper function to create a popover
pub fn popover(content: impl Into<String>) -> Popover {
    Popover::new(content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_popover_new() {
        let p = Popover::new("Test content");
        assert_eq!(p.content, "Test content");
        assert!(!p.is_open());
        assert!(p.close_on_escape);
        assert!(p.close_on_click_outside);
    }

    #[test]
    fn test_popover_builder() {
        let p = Popover::new("Content")
            .anchor(10, 5)
            .position(PopoverPosition::Top)
            .trigger(PopoverTrigger::Hover)
            .popover_style(PopoverStyle::Rounded)
            .arrow(PopoverArrow::Unicode)
            .title("Title")
            .max_width(30)
            .close_on_escape(false)
            .close_on_click_outside(false);

        assert_eq!(p.anchor, (10, 5));
        assert!(matches!(p.position, PopoverPosition::Top));
        assert!(matches!(p.trigger, PopoverTrigger::Hover));
        assert!(matches!(p.popover_style, PopoverStyle::Rounded));
        assert!(matches!(p.arrow, PopoverArrow::Unicode));
        assert_eq!(p.title, Some("Title".to_string()));
        assert_eq!(p.max_width, 30);
        assert!(!p.close_on_escape);
        assert!(!p.close_on_click_outside);
    }

    #[test]
    fn test_popover_visibility() {
        let mut p = Popover::new("Test");
        assert!(!p.is_open());

        p.show();
        assert!(p.is_open());

        p.hide();
        assert!(!p.is_open());

        p.toggle();
        assert!(p.is_open());

        p.toggle();
        assert!(!p.is_open());
    }

    #[test]
    fn test_popover_handle_escape() {
        let mut p = Popover::new("Test").open(true);
        assert!(p.is_open());

        assert!(p.handle_key(&Key::Escape));
        assert!(!p.is_open());
    }

    #[test]
    fn test_popover_handle_escape_disabled() {
        let mut p = Popover::new("Test").open(true).close_on_escape(false);
        assert!(!p.handle_key(&Key::Escape));
        assert!(p.is_open());
    }

    #[test]
    fn test_popover_handle_key_closed() {
        let mut p = Popover::new("Test");
        assert!(!p.handle_key(&Key::Escape));
    }

    #[test]
    fn test_popover_calculate_dimensions() {
        let p = Popover::new("Short content");
        let (w, h) = p.calculate_dimensions();
        assert!(w >= 10);
        assert!(h >= 3);
    }

    #[test]
    fn test_popover_calculate_dimensions_with_title() {
        let p = Popover::new("Content").title("Title");
        let (_, h) = p.calculate_dimensions();
        assert!(h >= 4);
    }

    #[test]
    fn test_popover_wrap_content() {
        let p = Popover::new("This is a very long text that should be wrapped").max_width(20);
        let lines = p.wrap_content();
        assert!(lines.len() > 1);
        assert!(lines.iter().all(|l| l.len() <= 20));
    }

    #[test]
    fn test_popover_position_auto() {
        let p = Popover::new("Test")
            .position(PopoverPosition::Auto)
            .anchor(20, 20);

        let (_, _, pos) = p.calculate_position(40, 40);
        assert!(!matches!(pos, PopoverPosition::Auto));
    }

    #[test]
    fn test_popover_render_closed() {
        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let p = Popover::new("Content");
        p.render(&mut ctx);
        // Should not render when closed
    }

    #[test]
    fn test_popover_render_open() {
        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let p = Popover::new("Hello World").anchor(20, 10).open(true);
        p.render(&mut ctx);
        // Smoke test - renders without panic
    }

    #[test]
    fn test_popover_render_elevated() {
        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let p = Popover::new("Elevated")
            .popover_style(PopoverStyle::Elevated)
            .anchor(20, 10)
            .open(true);
        p.render(&mut ctx);
    }

    #[test]
    fn test_popover_render_minimal() {
        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let p = Popover::new("Minimal")
            .popover_style(PopoverStyle::Minimal)
            .anchor(20, 10)
            .open(true);
        p.render(&mut ctx);
    }

    #[test]
    fn test_popover_render_with_arrow() {
        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let p = Popover::new("Arrow")
            .arrow(PopoverArrow::Unicode)
            .anchor(20, 10)
            .open(true);
        p.render(&mut ctx);
    }

    #[test]
    fn test_popover_styles() {
        let styles = [
            PopoverStyle::Default,
            PopoverStyle::Rounded,
            PopoverStyle::Minimal,
            PopoverStyle::Elevated,
        ];

        for style in styles {
            let (fg, bg, border) = style.colors();
            let _ = (fg.r, bg.r, border.r);
        }
    }

    #[test]
    fn test_popover_arrow_chars() {
        let arrow = PopoverArrow::Unicode;
        assert_eq!(arrow.chars(PopoverPosition::Top), '▼');
        assert_eq!(arrow.chars(PopoverPosition::Bottom), '▲');
        assert_eq!(arrow.chars(PopoverPosition::Left), '▶');
        assert_eq!(arrow.chars(PopoverPosition::Right), '◀');
    }

    #[test]
    fn test_popover_helper() {
        let p = popover("Quick popover");
        assert_eq!(p.content, "Quick popover");
    }

    #[test]
    fn test_popover_default() {
        let p = Popover::default();
        assert_eq!(p.content, "");
    }

    #[test]
    fn test_popover_set_anchor() {
        let mut p = Popover::new("Test");
        p.set_anchor(15, 25);
        assert_eq!(p.anchor, (15, 25));
    }

    #[test]
    fn test_popover_trigger_types() {
        let _ = Popover::new("Test").trigger(PopoverTrigger::Click);
        let _ = Popover::new("Test").trigger(PopoverTrigger::Hover);
        let _ = Popover::new("Test").trigger(PopoverTrigger::Focus);
        let _ = Popover::new("Test").trigger(PopoverTrigger::Manual);
    }

    #[test]
    fn test_popover_custom_colors() {
        let p = Popover::new("Test")
            .fg(Color::RED)
            .bg(Color::BLUE)
            .border_color(Color::GREEN);

        assert_eq!(p.state.fg, Some(Color::RED));
        assert_eq!(p.state.bg, Some(Color::BLUE));
        assert_eq!(p.border_color, Some(Color::GREEN));
    }

    #[test]
    fn test_popover_handle_click_inside() {
        let mut p = Popover::new("Test").anchor(20, 10).open(true);

        // Click inside the popover area
        let handled = p.handle_click(20, 12, 40, 20);
        assert!(handled);
        assert!(p.is_open()); // Should stay open
    }

    #[test]
    fn test_popover_handle_click_outside() {
        let mut p = Popover::new("Test").anchor(20, 10).open(true);

        // Click outside the popover
        let handled = p.handle_click(0, 0, 40, 20);
        assert!(handled);
        assert!(!p.is_open()); // Should close
    }

    #[test]
    fn test_popover_handle_click_outside_disabled() {
        let mut p = Popover::new("Test")
            .anchor(20, 10)
            .open(true)
            .close_on_click_outside(false);

        let handled = p.handle_click(0, 0, 40, 20);
        assert!(!handled);
        assert!(p.is_open()); // Should stay open
    }
}
