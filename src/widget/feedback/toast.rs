//! Toast notification widget
//!
//! Displays temporary notification messages with different severity levels.

use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Toast notification level
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToastLevel {
    /// Informational message (blue)
    #[default]
    Info,
    /// Success message (green)
    Success,
    /// Warning message (yellow)
    Warning,
    /// Error message (red)
    Error,
}

impl ToastLevel {
    /// Get the icon for this level
    pub fn icon(&self) -> char {
        match self {
            ToastLevel::Info => 'ℹ',
            ToastLevel::Success => '✓',
            ToastLevel::Warning => '⚠',
            ToastLevel::Error => '✗',
        }
    }

    /// Get the color for this level
    pub fn color(&self) -> Color {
        match self {
            ToastLevel::Info => Color::CYAN,
            ToastLevel::Success => Color::GREEN,
            ToastLevel::Warning => Color::YELLOW,
            ToastLevel::Error => Color::RED,
        }
    }

    /// Get the background color for this level
    pub fn bg_color(&self) -> Color {
        match self {
            ToastLevel::Info => Color::rgb(0, 40, 60),
            ToastLevel::Success => Color::rgb(0, 40, 0),
            ToastLevel::Warning => Color::rgb(60, 40, 0),
            ToastLevel::Error => Color::rgb(60, 0, 0),
        }
    }
}

/// Toast position on screen
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToastPosition {
    /// Top-left corner
    TopLeft,
    /// Top-center
    TopCenter,
    /// Top-right corner
    #[default]
    TopRight,
    /// Bottom-left corner
    BottomLeft,
    /// Bottom-center
    BottomCenter,
    /// Bottom-right corner
    BottomRight,
}

/// A toast notification widget
///
/// # Example
///
/// ```rust,ignore
/// use revue::prelude::*;
///
/// let toast = Toast::new("File saved successfully!")
///     .level(ToastLevel::Success)
///     .position(ToastPosition::TopRight);
/// ```
pub struct Toast {
    message: String,
    level: ToastLevel,
    position: ToastPosition,
    width: Option<u16>,
    show_icon: bool,
    show_border: bool,
    props: WidgetProps,
}

impl Toast {
    /// Create a new toast with a message
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            level: ToastLevel::default(),
            position: ToastPosition::default(),
            width: None,
            show_icon: true,
            show_border: true,
            props: WidgetProps::new(),
        }
    }

    /// Create an info toast
    pub fn info(message: impl Into<String>) -> Self {
        Self::new(message).level(ToastLevel::Info)
    }

    /// Create a success toast
    pub fn success(message: impl Into<String>) -> Self {
        Self::new(message).level(ToastLevel::Success)
    }

    /// Create a warning toast
    pub fn warning(message: impl Into<String>) -> Self {
        Self::new(message).level(ToastLevel::Warning)
    }

    /// Create an error toast
    pub fn error(message: impl Into<String>) -> Self {
        Self::new(message).level(ToastLevel::Error)
    }

    /// Set the toast level
    pub fn level(mut self, level: ToastLevel) -> Self {
        self.level = level;
        self
    }

    /// Set the toast position
    pub fn position(mut self, position: ToastPosition) -> Self {
        self.position = position;
        self
    }

    /// Set fixed width
    pub fn width(mut self, width: u16) -> Self {
        self.width = Some(width);
        self
    }

    /// Show or hide the icon
    pub fn show_icon(mut self, show: bool) -> Self {
        self.show_icon = show;
        self
    }

    /// Show or hide the border
    pub fn show_border(mut self, show: bool) -> Self {
        self.show_border = show;
        self
    }

    /// Calculate toast dimensions
    fn calculate_size(&self, max_width: u16) -> (u16, u16) {
        let icon_width = if self.show_icon { 2 } else { 0 };
        let border_width = if self.show_border { 2 } else { 0 };
        let padding = 2; // 1 char padding on each side

        let content_width = self.message.len() as u16 + icon_width;
        let total_width = self.width.unwrap_or(content_width + border_width + padding);
        let width = total_width.min(max_width);

        // Calculate height based on message wrapping
        let inner_width = width.saturating_sub(border_width + padding + icon_width);
        let lines = if inner_width > 0 {
            (self.message.len() as u16).div_ceil(inner_width)
        } else {
            1
        };
        let height = lines + if self.show_border { 2 } else { 0 };

        (width, height.max(if self.show_border { 3 } else { 1 }))
    }

    /// Calculate toast position
    fn calculate_position(
        &self,
        area_width: u16,
        area_height: u16,
        toast_width: u16,
        toast_height: u16,
    ) -> (u16, u16) {
        let margin = 1u16;

        let x = match self.position {
            ToastPosition::TopLeft | ToastPosition::BottomLeft => margin,
            ToastPosition::TopCenter | ToastPosition::BottomCenter => {
                area_width.saturating_sub(toast_width) / 2
            }
            ToastPosition::TopRight | ToastPosition::BottomRight => {
                area_width.saturating_sub(toast_width + margin)
            }
        };

        let y = match self.position {
            ToastPosition::TopLeft | ToastPosition::TopCenter | ToastPosition::TopRight => margin,
            ToastPosition::BottomLeft
            | ToastPosition::BottomCenter
            | ToastPosition::BottomRight => area_height.saturating_sub(toast_height + margin),
        };

        (x, y)
    }
}

impl View for Toast {
    crate::impl_view_meta!("Toast");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 5 || area.height < 3 {
            return;
        }

        let (toast_width, toast_height) = self.calculate_size(area.width);
        let (x, y) = self.calculate_position(area.width, area.height, toast_width, toast_height);

        let color = self.level.color();
        let bg = self.level.bg_color();

        // Draw border
        if self.show_border {
            // Top border
            let mut top_left = Cell::new('╭');
            top_left.fg = Some(color);
            top_left.bg = Some(bg);
            ctx.buffer.set(area.x + x, area.y + y, top_left);

            for i in 1..toast_width.saturating_sub(1) {
                let mut cell = Cell::new('─');
                cell.fg = Some(color);
                cell.bg = Some(bg);
                ctx.buffer.set(area.x + x + i, area.y + y, cell);
            }

            let mut top_right = Cell::new('╮');
            top_right.fg = Some(color);
            top_right.bg = Some(bg);
            ctx.buffer
                .set(area.x + x + toast_width - 1, area.y + y, top_right);

            // Bottom border
            let mut bottom_left = Cell::new('╰');
            bottom_left.fg = Some(color);
            bottom_left.bg = Some(bg);
            ctx.buffer
                .set(area.x + x, area.y + y + toast_height - 1, bottom_left);

            for i in 1..toast_width.saturating_sub(1) {
                let mut cell = Cell::new('─');
                cell.fg = Some(color);
                cell.bg = Some(bg);
                ctx.buffer
                    .set(area.x + x + i, area.y + y + toast_height - 1, cell);
            }

            let mut bottom_right = Cell::new('╯');
            bottom_right.fg = Some(color);
            bottom_right.bg = Some(bg);
            ctx.buffer.set(
                area.x + x + toast_width - 1,
                area.y + y + toast_height - 1,
                bottom_right,
            );

            // Side borders
            for row in 1..toast_height.saturating_sub(1) {
                let mut left = Cell::new('│');
                left.fg = Some(color);
                left.bg = Some(bg);
                ctx.buffer.set(area.x + x, area.y + y + row, left);

                let mut right = Cell::new('│');
                right.fg = Some(color);
                right.bg = Some(bg);
                ctx.buffer
                    .set(area.x + x + toast_width - 1, area.y + y + row, right);

                // Fill background
                for col in 1..toast_width.saturating_sub(1) {
                    let mut fill = Cell::new(' ');
                    fill.bg = Some(bg);
                    ctx.buffer.set(area.x + x + col, area.y + y + row, fill);
                }
            }
        }

        // Draw content
        let content_x = x + if self.show_border { 2 } else { 0 };
        let content_y = y + if self.show_border { 1 } else { 0 };

        // Draw icon
        if self.show_icon {
            let mut icon_cell = Cell::new(self.level.icon());
            icon_cell.fg = Some(color);
            icon_cell.bg = Some(bg);
            ctx.buffer
                .set(area.x + content_x, area.y + content_y, icon_cell);
        }

        // Draw message
        let msg_x = content_x + if self.show_icon { 2 } else { 0 };
        for (i, ch) in self.message.chars().enumerate() {
            let pos = msg_x + i as u16;
            if pos < x + toast_width - if self.show_border { 1 } else { 0 } {
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::WHITE);
                cell.bg = Some(bg);
                ctx.buffer.set(area.x + pos, area.y + content_y, cell);
            }
        }
    }
}

impl_styled_view!(Toast);
impl_props_builders!(Toast);

/// Create a toast notification
pub fn toast(message: impl Into<String>) -> Toast {
    Toast::new(message)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_toast_new() {
        let t = Toast::new("Test message");
        assert_eq!(t.message, "Test message");
        assert_eq!(t.level, ToastLevel::Info);
    }

    #[test]
    fn test_toast_levels() {
        let info = Toast::info("Info");
        assert_eq!(info.level, ToastLevel::Info);

        let success = Toast::success("Success");
        assert_eq!(success.level, ToastLevel::Success);

        let warning = Toast::warning("Warning");
        assert_eq!(warning.level, ToastLevel::Warning);

        let error = Toast::error("Error");
        assert_eq!(error.level, ToastLevel::Error);
    }

    #[test]
    fn test_toast_position() {
        let t = Toast::new("Test").position(ToastPosition::BottomLeft);
        assert_eq!(t.position, ToastPosition::BottomLeft);
    }

    #[test]
    fn test_toast_level_icon() {
        assert_eq!(ToastLevel::Info.icon(), 'ℹ');
        assert_eq!(ToastLevel::Success.icon(), '✓');
        assert_eq!(ToastLevel::Warning.icon(), '⚠');
        assert_eq!(ToastLevel::Error.icon(), '✗');
    }

    #[test]
    fn test_toast_level_color() {
        assert_eq!(ToastLevel::Info.color(), Color::CYAN);
        assert_eq!(ToastLevel::Success.color(), Color::GREEN);
        assert_eq!(ToastLevel::Warning.color(), Color::YELLOW);
        assert_eq!(ToastLevel::Error.color(), Color::RED);
    }

    #[test]
    fn test_toast_render() {
        let t = Toast::new("Hello World")
            .level(ToastLevel::Success)
            .position(ToastPosition::TopRight);

        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        t.render(&mut ctx);
    }

    #[test]
    fn test_toast_no_border() {
        let t = Toast::new("No border").show_border(false);

        let mut buffer = Buffer::new(30, 5);
        let area = Rect::new(0, 0, 30, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        t.render(&mut ctx);
    }

    #[test]
    fn test_toast_no_icon() {
        let t = Toast::new("No icon").show_icon(false);

        let mut buffer = Buffer::new(30, 5);
        let area = Rect::new(0, 0, 30, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        t.render(&mut ctx);
    }

    #[test]
    fn test_toast_helper() {
        let t = toast("Quick toast");
        assert_eq!(t.message, "Quick toast");
    }

    #[test]
    fn test_toast_all_positions() {
        let positions = [
            ToastPosition::TopLeft,
            ToastPosition::TopCenter,
            ToastPosition::TopRight,
            ToastPosition::BottomLeft,
            ToastPosition::BottomCenter,
            ToastPosition::BottomRight,
        ];

        for pos in positions {
            let t = Toast::new("Test").position(pos);
            let mut buffer = Buffer::new(40, 20);
            let area = Rect::new(0, 0, 40, 20);
            let mut ctx = RenderContext::new(&mut buffer, area);
            t.render(&mut ctx);
        }
    }
}
