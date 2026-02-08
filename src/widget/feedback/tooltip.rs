//! Tooltip widget for displaying contextual information
//!
//! Provides hover-style tooltips and help text displays.

use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::border::BorderChars;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Tooltip position relative to anchor
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TooltipPosition {
    /// Above the anchor
    #[default]
    Top,
    /// Below the anchor
    Bottom,
    /// To the left of anchor
    Left,
    /// To the right of anchor
    Right,
    /// Auto-detect best position
    Auto,
}

/// Tooltip arrow style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TooltipArrow {
    /// No arrow
    #[default]
    None,
    /// Simple arrow
    Simple,
    /// Unicode arrow
    Unicode,
}

impl TooltipArrow {
    fn chars(&self, position: TooltipPosition) -> (char, char) {
        match (self, position) {
            (TooltipArrow::None, _) => (' ', ' '),
            (TooltipArrow::Simple, TooltipPosition::Top) => ('v', 'v'),
            (TooltipArrow::Simple, TooltipPosition::Bottom) => ('^', '^'),
            (TooltipArrow::Simple, TooltipPosition::Left) => ('>', '>'),
            (TooltipArrow::Simple, TooltipPosition::Right) => ('<', '<'),
            (TooltipArrow::Simple, TooltipPosition::Auto) => ('v', 'v'),
            (TooltipArrow::Unicode, TooltipPosition::Top) => ('▼', '▽'),
            (TooltipArrow::Unicode, TooltipPosition::Bottom) => ('▲', '△'),
            (TooltipArrow::Unicode, TooltipPosition::Left) => ('▶', '▷'),
            (TooltipArrow::Unicode, TooltipPosition::Right) => ('◀', '◁'),
            (TooltipArrow::Unicode, TooltipPosition::Auto) => ('▼', '▽'),
        }
    }
}

/// Tooltip style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TooltipStyle {
    /// Simple text
    #[default]
    Plain,
    /// With border
    Bordered,
    /// Rounded corners
    Rounded,
    /// Info style (cyan)
    Info,
    /// Warning style (yellow)
    Warning,
    /// Error style (red)
    Error,
    /// Success style (green)
    Success,
}

impl TooltipStyle {
    fn colors(&self) -> (Color, Color) {
        match self {
            TooltipStyle::Plain => (Color::WHITE, Color::rgb(40, 40, 40)),
            TooltipStyle::Bordered => (Color::WHITE, Color::rgb(30, 30, 30)),
            TooltipStyle::Rounded => (Color::WHITE, Color::rgb(30, 30, 30)),
            TooltipStyle::Info => (Color::WHITE, Color::rgb(30, 80, 100)),
            TooltipStyle::Warning => (Color::BLACK, Color::rgb(180, 150, 0)),
            TooltipStyle::Error => (Color::WHITE, Color::rgb(150, 30, 30)),
            TooltipStyle::Success => (Color::WHITE, Color::rgb(30, 100, 50)),
        }
    }

    fn border_chars(&self) -> Option<BorderChars> {
        match self {
            TooltipStyle::Plain => None,
            TooltipStyle::Bordered
            | TooltipStyle::Info
            | TooltipStyle::Warning
            | TooltipStyle::Error
            | TooltipStyle::Success => Some(BorderChars::SINGLE),
            TooltipStyle::Rounded => Some(BorderChars::ROUNDED),
        }
    }
}

/// Tooltip widget
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

    /// Word wrap text
    fn wrap_text(&self) -> Vec<String> {
        let max_width = if self.max_width > 0 {
            self.max_width as usize
        } else {
            40
        };

        let mut lines = Vec::new();
        for line in self.text.lines() {
            if line.len() <= max_width {
                lines.push(line.to_string());
            } else {
                // Simple word wrap
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

    /// Calculate tooltip dimensions
    fn calculate_dimensions(&self) -> (u16, u16) {
        let lines = self.wrap_text();
        let has_border = self.style.border_chars().is_some();
        let has_title = self.title.is_some();

        let content_width = lines.iter().map(|l| l.len()).max().unwrap_or(0) as u16;
        let title_width = self.title.as_ref().map(|t| t.len() as u16 + 2).unwrap_or(0);
        let text_width = content_width.max(title_width);

        let width = text_width + if has_border { 4 } else { 2 }; // padding + border
        let height = lines.len() as u16
            + if has_border { 2 } else { 0 }
            + if has_title && has_border { 1 } else { 0 };

        (width, height)
    }

    /// Calculate position based on anchor and available space
    fn calculate_position(&self, area_width: u16, area_height: u16) -> (u16, u16, TooltipPosition) {
        let (tooltip_w, tooltip_h) = self.calculate_dimensions();
        let (anchor_x, anchor_y) = self.anchor;
        let arrow_offset: u16 = if matches!(self.arrow, TooltipArrow::None) {
            0
        } else {
            1
        };

        let (x, y, position) = match self.position {
            TooltipPosition::Auto => {
                // Auto-detect best position based on available space
                let space_above = anchor_y;
                let space_below = area_height.saturating_sub(anchor_y + 1);
                let space_left = anchor_x;
                let space_right = area_width.saturating_sub(anchor_x + 1);

                let pos = if space_above >= tooltip_h + arrow_offset {
                    TooltipPosition::Top
                } else if space_below >= tooltip_h + arrow_offset {
                    TooltipPosition::Bottom
                } else if space_right >= tooltip_w + arrow_offset {
                    TooltipPosition::Right
                } else if space_left >= tooltip_w + arrow_offset {
                    TooltipPosition::Left
                } else {
                    TooltipPosition::Top // Default fallback
                };

                // Calculate position for the auto-detected position
                // Note: pos is guaranteed to be Top/Bottom/Left/Right (never Auto)
                // because Auto was resolved to a concrete position above
                let (x, y) = match pos {
                    TooltipPosition::Top => {
                        let x = anchor_x.saturating_sub(tooltip_w / 2);
                        let y = anchor_y.saturating_sub(tooltip_h + arrow_offset);
                        (x, y)
                    }
                    TooltipPosition::Bottom => {
                        let x = anchor_x.saturating_sub(tooltip_w / 2);
                        let y = anchor_y + 1 + arrow_offset;
                        (x, y)
                    }
                    TooltipPosition::Left => {
                        let x = anchor_x.saturating_sub(tooltip_w + arrow_offset);
                        let y = anchor_y.saturating_sub(tooltip_h / 2);
                        (x, y)
                    }
                    TooltipPosition::Right => {
                        let x = anchor_x + 1 + arrow_offset;
                        let y = anchor_y.saturating_sub(tooltip_h / 2);
                        (x, y)
                    }
                    // Auto is handled above and never reaches here
                    TooltipPosition::Auto => {
                        unreachable!("Auto position resolved to concrete position above")
                    }
                };
                (x, y, pos)
            }
            TooltipPosition::Top => {
                let x = anchor_x.saturating_sub(tooltip_w / 2);
                let y = anchor_y.saturating_sub(tooltip_h + arrow_offset);
                (x, y, TooltipPosition::Top)
            }
            TooltipPosition::Bottom => {
                let x = anchor_x.saturating_sub(tooltip_w / 2);
                let y = anchor_y + 1 + arrow_offset;
                (x, y, TooltipPosition::Bottom)
            }
            TooltipPosition::Left => {
                let x = anchor_x.saturating_sub(tooltip_w + arrow_offset);
                let y = anchor_y.saturating_sub(tooltip_h / 2);
                (x, y, TooltipPosition::Left)
            }
            TooltipPosition::Right => {
                let x = anchor_x + 1 + arrow_offset;
                let y = anchor_y.saturating_sub(tooltip_h / 2);
                (x, y, TooltipPosition::Right)
            }
        };

        // Clamp to screen bounds
        let x = x.min(area_width.saturating_sub(tooltip_w));
        let y = y.min(area_height.saturating_sub(tooltip_h));

        (x, y, position)
    }
}

impl Default for Tooltip {
    fn default() -> Self {
        Self::new("")
    }
}

impl View for Tooltip {
    crate::impl_view_meta!("Tooltip");

    fn render(&self, ctx: &mut RenderContext) {
        if !self.visible || (self.delay > 0 && self.delay_counter < self.delay) {
            return;
        }

        let area = ctx.area;
        let (tooltip_w, tooltip_h) = self.calculate_dimensions();
        let (tooltip_x, tooltip_y, actual_position) =
            self.calculate_position(area.width, area.height);

        // Get colors
        let (default_fg, default_bg) = self.style.colors();
        let fg = self.fg.unwrap_or(default_fg);
        let bg = self.bg.unwrap_or(default_bg);

        // Draw background
        for dy in 0..tooltip_h {
            for dx in 0..tooltip_w {
                let x = tooltip_x + dx;
                let y = tooltip_y + dy;
                if x < area.width && y < area.height {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(bg);
                    ctx.buffer.set(x, y, cell);
                }
            }
        }

        // Draw border if applicable
        let content_start_x;
        let content_start_y;

        if let Some(border) = self.style.border_chars() {
            content_start_x = tooltip_x + 2;
            content_start_y = tooltip_y + 1;

            // Top border
            if tooltip_y < area.height {
                let mut tl = Cell::new(border.top_left);
                tl.fg = Some(fg);
                tl.bg = Some(bg);
                ctx.buffer.set(tooltip_x, tooltip_y, tl);

                for dx in 1..tooltip_w - 1 {
                    let mut h = Cell::new(border.horizontal);
                    h.fg = Some(fg);
                    h.bg = Some(bg);
                    ctx.buffer.set(tooltip_x + dx, tooltip_y, h);
                }

                let mut tr = Cell::new(border.top_right);
                tr.fg = Some(fg);
                tr.bg = Some(bg);
                ctx.buffer.set(tooltip_x + tooltip_w - 1, tooltip_y, tr);
            }

            // Title if present
            if let Some(ref title) = self.title {
                let title_x = tooltip_x + 2;
                let title_y = tooltip_y + 1;
                for (i, ch) in title.chars().enumerate() {
                    let x = title_x + i as u16;
                    if x < tooltip_x + tooltip_w - 2 {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(fg);
                        cell.bg = Some(bg);
                        cell.modifier |= Modifier::BOLD;
                        ctx.buffer.set(x, title_y, cell);
                    }
                }
            }

            // Left and right borders
            let _text_start_y = if self.title.is_some() {
                tooltip_y + 2
            } else {
                tooltip_y + 1
            };
            for dy in 1..tooltip_h - 1 {
                let y = tooltip_y + dy;
                if y < area.height {
                    let mut left = Cell::new(border.vertical);
                    left.fg = Some(fg);
                    left.bg = Some(bg);
                    ctx.buffer.set(tooltip_x, y, left);

                    let mut right = Cell::new(border.vertical);
                    right.fg = Some(fg);
                    right.bg = Some(bg);
                    ctx.buffer.set(tooltip_x + tooltip_w - 1, y, right);
                }
            }

            // Bottom border
            let bottom_y = tooltip_y + tooltip_h - 1;
            if bottom_y < area.height {
                let mut bl = Cell::new(border.bottom_left);
                bl.fg = Some(fg);
                bl.bg = Some(bg);
                ctx.buffer.set(tooltip_x, bottom_y, bl);

                for dx in 1..tooltip_w - 1 {
                    let mut h = Cell::new(border.horizontal);
                    h.fg = Some(fg);
                    h.bg = Some(bg);
                    ctx.buffer.set(tooltip_x + dx, bottom_y, h);
                }

                let mut br = Cell::new(border.bottom_right);
                br.fg = Some(fg);
                br.bg = Some(bg);
                ctx.buffer.set(tooltip_x + tooltip_w - 1, bottom_y, br);
            }
        } else {
            content_start_x = tooltip_x + 1;
            content_start_y = tooltip_y;
        }

        // Draw text content
        let lines = self.wrap_text();
        let text_y_offset = if self.title.is_some() && self.style.border_chars().is_some() {
            1
        } else {
            0
        };

        for (i, line) in lines.iter().enumerate() {
            let y = content_start_y + text_y_offset + i as u16;
            if y >= area.height || y >= tooltip_y + tooltip_h - 1 {
                break;
            }

            for (j, ch) in line.chars().enumerate() {
                let x = content_start_x + j as u16;
                if x < tooltip_x + tooltip_w - 1 {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(fg);
                    cell.bg = Some(bg);
                    ctx.buffer.set(x, y, cell);
                }
            }
        }

        // Draw arrow
        if !matches!(self.arrow, TooltipArrow::None) {
            let (arrow_char, _) = self.arrow.chars(actual_position);
            let (arrow_x, arrow_y) = match actual_position {
                TooltipPosition::Top => (self.anchor.0, tooltip_y + tooltip_h),
                TooltipPosition::Bottom => (self.anchor.0, tooltip_y.saturating_sub(1)),
                TooltipPosition::Left => (tooltip_x + tooltip_w, self.anchor.1),
                TooltipPosition::Right => (tooltip_x.saturating_sub(1), self.anchor.1),
                // Auto is already resolved to a concrete position in calculate_position
                // This case should never be reached, but use Top as fallback
                TooltipPosition::Auto => (self.anchor.0, tooltip_y + tooltip_h),
            };

            if arrow_x < area.width && arrow_y < area.height {
                let mut cell = Cell::new(arrow_char);
                cell.fg = Some(fg);
                ctx.buffer.set(arrow_x, arrow_y, cell);
            }
        }
    }
}

impl_styled_view!(Tooltip);
impl_props_builders!(Tooltip);

/// Helper to create a tooltip
pub fn tooltip(text: impl Into<String>) -> Tooltip {
    Tooltip::new(text)
}

// Most tests moved to tests/widget_tests.rs
// Tests below access private fields and must stay inline

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tooltip_new() {
        let t = Tooltip::new("Test tooltip");
        assert_eq!(t.text, "Test tooltip");
        assert!(t.visible);
    }

    #[test]
    fn test_tooltip_builder() {
        let t = Tooltip::new("Hello")
            .position(TooltipPosition::Bottom)
            .anchor(10, 5)
            .style(TooltipStyle::Info)
            .arrow(TooltipArrow::Unicode)
            .max_width(30);

        assert!(matches!(t.position, TooltipPosition::Bottom));
        assert_eq!(t.anchor, (10, 5));
        assert!(matches!(t.style, TooltipStyle::Info));
        assert_eq!(t.max_width, 30);
    }

    #[test]
    fn test_tooltip_presets() {
        let info = Tooltip::info("Info message");
        assert!(matches!(info.style, TooltipStyle::Info));

        let warning = Tooltip::warning("Warning!");
        assert!(matches!(warning.style, TooltipStyle::Warning));

        let error = Tooltip::error("Error!");
        assert!(matches!(error.style, TooltipStyle::Error));

        let success = Tooltip::success("Success!");
        assert!(matches!(success.style, TooltipStyle::Success));
    }

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

    #[test]
    fn test_tooltip_helper_text() {
        let t = tooltip("Quick tooltip");
        assert_eq!(t.text, "Quick tooltip");
    }

    #[test]
    fn test_tooltip_styles_colors() {
        let styles = [
            TooltipStyle::Plain,
            TooltipStyle::Bordered,
            TooltipStyle::Rounded,
            TooltipStyle::Info,
            TooltipStyle::Warning,
            TooltipStyle::Error,
            TooltipStyle::Success,
        ];

        for style in styles {
            let (fg, bg) = style.colors();
            // All should return valid colors (checking they exist)
            let _ = (fg.r, fg.g, fg.b);
            let _ = (bg.r, bg.g, bg.b);
        }
    }

    #[test]
    fn test_arrow_chars() {
        let arrow = TooltipArrow::Unicode;
        let (top, _) = arrow.chars(TooltipPosition::Top);
        assert_eq!(top, '▼');

        let (bottom, _) = arrow.chars(TooltipPosition::Bottom);
        assert_eq!(bottom, '▲');
    }

    // =========================================================================
    // TooltipPosition enum tests
    // =========================================================================

    #[test]
    fn test_tooltip_position_default() {
        assert_eq!(TooltipPosition::default(), TooltipPosition::Top);
    }

    #[test]
    fn test_tooltip_position_clone() {
        let pos1 = TooltipPosition::Bottom;
        let pos2 = pos1.clone();
        assert_eq!(pos1, pos2);
    }

    #[test]
    fn test_tooltip_position_copy() {
        let pos1 = TooltipPosition::Left;
        let pos2 = pos1;
        assert_eq!(pos2, TooltipPosition::Left);
        // pos1 is still valid because of Copy
        assert_eq!(pos1, TooltipPosition::Left);
    }

    #[test]
    fn test_tooltip_position_partial_eq() {
        assert_eq!(TooltipPosition::Top, TooltipPosition::Top);
        assert_eq!(TooltipPosition::Bottom, TooltipPosition::Bottom);
        assert_eq!(TooltipPosition::Left, TooltipPosition::Left);
        assert_eq!(TooltipPosition::Right, TooltipPosition::Right);
        assert_eq!(TooltipPosition::Auto, TooltipPosition::Auto);

        assert_ne!(TooltipPosition::Top, TooltipPosition::Bottom);
        assert_ne!(TooltipPosition::Left, TooltipPosition::Right);
        assert_ne!(TooltipPosition::Auto, TooltipPosition::Top);
    }

    #[test]
    fn test_tooltip_position_all_variants() {
        let positions = [
            TooltipPosition::Top,
            TooltipPosition::Bottom,
            TooltipPosition::Left,
            TooltipPosition::Right,
            TooltipPosition::Auto,
        ];

        // Verify all variants are distinct
        for (i, pos1) in positions.iter().enumerate() {
            for (j, pos2) in positions.iter().enumerate() {
                if i == j {
                    assert_eq!(pos1, pos2);
                } else {
                    assert_ne!(pos1, pos2);
                }
            }
        }
    }

    // =========================================================================
    // TooltipArrow enum tests
    // =========================================================================

    #[test]
    fn test_tooltip_arrow_default() {
        assert_eq!(TooltipArrow::default(), TooltipArrow::None);
    }

    #[test]
    fn test_tooltip_arrow_clone() {
        let arrow1 = TooltipArrow::Unicode;
        let arrow2 = arrow1.clone();
        assert_eq!(arrow1, arrow2);
    }

    #[test]
    fn test_tooltip_arrow_copy() {
        let arrow1 = TooltipArrow::Simple;
        let arrow2 = arrow1;
        assert_eq!(arrow2, TooltipArrow::Simple);
        // arrow1 is still valid because of Copy
        assert_eq!(arrow1, TooltipArrow::Simple);
    }

    #[test]
    fn test_tooltip_arrow_partial_eq() {
        assert_eq!(TooltipArrow::None, TooltipArrow::None);
        assert_eq!(TooltipArrow::Simple, TooltipArrow::Simple);
        assert_eq!(TooltipArrow::Unicode, TooltipArrow::Unicode);

        assert_ne!(TooltipArrow::None, TooltipArrow::Simple);
        assert_ne!(TooltipArrow::Simple, TooltipArrow::Unicode);
        assert_ne!(TooltipArrow::Unicode, TooltipArrow::None);
    }

    #[test]
    fn test_tooltip_arrow_all_variants() {
        let arrows = [
            TooltipArrow::None,
            TooltipArrow::Simple,
            TooltipArrow::Unicode,
        ];

        // Verify all variants are distinct
        for (i, arrow1) in arrows.iter().enumerate() {
            for (j, arrow2) in arrows.iter().enumerate() {
                if i == j {
                    assert_eq!(arrow1, arrow2);
                } else {
                    assert_ne!(arrow1, arrow2);
                }
            }
        }
    }

    #[test]
    fn test_tooltip_arrow_chars_all_combinations() {
        let arrows = [
            TooltipArrow::None,
            TooltipArrow::Simple,
            TooltipArrow::Unicode,
        ];
        let positions = [
            TooltipPosition::Top,
            TooltipPosition::Bottom,
            TooltipPosition::Left,
            TooltipPosition::Right,
            TooltipPosition::Auto,
        ];

        for arrow in arrows {
            for pos in positions {
                let (char1, char2) = arrow.chars(pos);
                // Verify chars are valid (no panics) and are valid Unicode chars
                assert!(char1.len_utf8() >= 1);
                assert!(char2.len_utf8() >= 1);
            }
        }
    }

    // =========================================================================
    // TooltipStyle enum tests
    // =========================================================================

    #[test]
    fn test_tooltip_style_default() {
        assert_eq!(TooltipStyle::default(), TooltipStyle::Plain);
    }

    #[test]
    fn test_tooltip_style_clone() {
        let style1 = TooltipStyle::Info;
        let style2 = style1.clone();
        assert_eq!(style1, style2);
    }

    #[test]
    fn test_tooltip_style_copy() {
        let style1 = TooltipStyle::Warning;
        let style2 = style1;
        assert_eq!(style2, TooltipStyle::Warning);
        // style1 is still valid because of Copy
        assert_eq!(style1, TooltipStyle::Warning);
    }

    #[test]
    fn test_tooltip_style_partial_eq() {
        assert_eq!(TooltipStyle::Plain, TooltipStyle::Plain);
        assert_eq!(TooltipStyle::Bordered, TooltipStyle::Bordered);
        assert_eq!(TooltipStyle::Rounded, TooltipStyle::Rounded);
        assert_eq!(TooltipStyle::Info, TooltipStyle::Info);
        assert_eq!(TooltipStyle::Warning, TooltipStyle::Warning);
        assert_eq!(TooltipStyle::Error, TooltipStyle::Error);
        assert_eq!(TooltipStyle::Success, TooltipStyle::Success);

        assert_ne!(TooltipStyle::Plain, TooltipStyle::Bordered);
        assert_ne!(TooltipStyle::Info, TooltipStyle::Warning);
        assert_ne!(TooltipStyle::Error, TooltipStyle::Success);
    }

    #[test]
    fn test_tooltip_style_all_variants() {
        let styles = [
            TooltipStyle::Plain,
            TooltipStyle::Bordered,
            TooltipStyle::Rounded,
            TooltipStyle::Info,
            TooltipStyle::Warning,
            TooltipStyle::Error,
            TooltipStyle::Success,
        ];

        // Verify all variants are distinct
        for (i, style1) in styles.iter().enumerate() {
            for (j, style2) in styles.iter().enumerate() {
                if i == j {
                    assert_eq!(style1, style2);
                } else {
                    assert_ne!(style1, style2);
                }
            }
        }
    }

    #[test]
    fn test_tooltip_style_colors_all_variants() {
        let styles = [
            TooltipStyle::Plain,
            TooltipStyle::Bordered,
            TooltipStyle::Rounded,
            TooltipStyle::Info,
            TooltipStyle::Warning,
            TooltipStyle::Error,
            TooltipStyle::Success,
        ];

        for style in styles {
            let (fg, bg) = style.colors();
            // u8 values are always valid 0-255, just verify colors exist
            let _ = (fg.r, fg.g, fg.b, bg.r, bg.g, bg.b);
        }
    }

    #[test]
    fn test_tooltip_style_border_chars_all_variants() {
        let styles = [
            TooltipStyle::Plain,
            TooltipStyle::Bordered,
            TooltipStyle::Rounded,
            TooltipStyle::Info,
            TooltipStyle::Warning,
            TooltipStyle::Error,
            TooltipStyle::Success,
        ];

        for style in styles {
            let border = style.border_chars();
            // Plain should have no border
            if matches!(style, TooltipStyle::Plain) {
                assert!(border.is_none());
            } else {
                assert!(border.is_some());
            }
        }
    }

    // =========================================================================
    // Tooltip Default trait tests
    // =========================================================================

    #[test]
    fn test_tooltip_default_trait() {
        let tooltip = Tooltip::default();
        assert_eq!(tooltip.text, "");
        assert!(tooltip.visible);
        assert_eq!(tooltip.position, TooltipPosition::Top);
        assert_eq!(tooltip.anchor, (0, 0));
        assert_eq!(tooltip.style, TooltipStyle::Bordered);
        assert_eq!(tooltip.arrow, TooltipArrow::Unicode);
        assert_eq!(tooltip.max_width, 40);
        assert_eq!(tooltip.fg, None);
        assert_eq!(tooltip.bg, None);
        assert_eq!(tooltip.title, None);
        assert_eq!(tooltip.delay, 0);
        assert_eq!(tooltip.delay_counter, 0);
    }

    #[test]
    fn test_tooltip_default_vs_new_empty() {
        let default_tooltip = Tooltip::default();
        let new_tooltip = Tooltip::new("");

        assert_eq!(default_tooltip.text, new_tooltip.text);
        assert_eq!(default_tooltip.visible, new_tooltip.visible);
    }
}
