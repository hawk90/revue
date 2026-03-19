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

    // Getters for testing
    #[doc(hidden)]
    pub fn get_message(&self) -> &str {
        &self.message
    }

    #[doc(hidden)]
    pub fn get_level(&self) -> ToastLevel {
        self.level
    }

    #[doc(hidden)]
    pub fn get_position(&self) -> ToastPosition {
        self.position
    }

    #[doc(hidden)]
    pub fn get_width(&self) -> Option<u16> {
        self.width
    }

    #[doc(hidden)]
    pub fn get_show_icon(&self) -> bool {
        self.show_icon
    }

    #[doc(hidden)]
    pub fn get_show_border(&self) -> bool {
        self.show_border
    }

    /// Calculate toast dimensions
    fn calculate_size(&self, max_width: u16) -> (u16, u16) {
        let icon_width = if self.show_icon { 2 } else { 0 };
        let border_width = if self.show_border { 2 } else { 0 };
        let padding = 2; // 1 char padding on each side

        let content_width = crate::utils::unicode::display_width(&self.message) as u16 + icon_width;
        let total_width = self.width.unwrap_or(content_width + border_width + padding);
        let width = total_width.min(max_width);

        // Calculate height based on message wrapping
        let inner_width = width.saturating_sub(border_width + padding + icon_width);
        let msg_cols = crate::utils::unicode::display_width(&self.message) as u16;
        let lines = if inner_width > 0 {
            msg_cols.saturating_add(inner_width - 1) / inner_width
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

        // Use buffer (screen) dimensions for positioning, not parent area
        let screen_w = ctx.buffer.width();
        let screen_h = ctx.buffer.height();
        let (toast_width, toast_height) = self.calculate_size(screen_w);
        let (abs_x, abs_y) = self.calculate_position(screen_w, screen_h, toast_width, toast_height);

        let color = self.level.color();
        let bg = self.level.bg_color();

        // Build overlay entry for toast (renders on top of everything)
        let overlay_area = crate::layout::Rect::new(abs_x, abs_y, toast_width, toast_height);
        let mut entry = crate::widget::traits::OverlayEntry::new(200, overlay_area); // z=200: above dropdowns

        // Helper to push cell
        let mut push = |rx: u16, ry: u16, ch: char, fg: Color, bg_c: Color| {
            let mut cell = Cell::new(ch);
            cell.fg = Some(fg);
            cell.bg = Some(bg_c);
            entry.push(rx, ry, cell);
        };

        // Draw border (relative to overlay area)
        if self.show_border {
            push(0, 0, '╭', color, bg);
            for i in 1..toast_width.saturating_sub(1) {
                push(i, 0, '─', color, bg);
            }
            push(toast_width - 1, 0, '╮', color, bg);

            push(0, toast_height - 1, '╰', color, bg);
            for i in 1..toast_width.saturating_sub(1) {
                push(i, toast_height - 1, '─', color, bg);
            }
            push(toast_width - 1, toast_height - 1, '╯', color, bg);

            for row in 1..toast_height.saturating_sub(1) {
                push(0, row, '│', color, bg);
                push(toast_width - 1, row, '│', color, bg);
                for col in 1..toast_width.saturating_sub(1) {
                    push(col, row, ' ', color, bg);
                }
            }
        }

        // Content position (relative)
        let cx = if self.show_border { 2 } else { 0 };
        let cy = if self.show_border { 1 } else { 0 };

        if self.show_icon {
            push(cx, cy, self.level.icon(), color, bg);
        }

        // Message text
        let msg_x = cx + if self.show_icon { 2 } else { 0 };
        let max_text = toast_width.saturating_sub(msg_x + if self.show_border { 1 } else { 0 });
        let truncated = crate::utils::truncate_to_width(&self.message, max_text as usize);
        let mut tx = msg_x;
        for ch in truncated.chars() {
            let cw = crate::utils::char_width(ch) as u16;
            if tx + cw > msg_x + max_text {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(Color::WHITE);
            cell.bg = Some(bg);
            entry.push(tx, cy, cell);
            tx += cw;
        }

        // Queue as overlay; fallback to inline
        if !ctx.queue_overlay(entry.clone()) {
            for oc in &entry.cells {
                ctx.set(oc.x, oc.y, oc.cell);
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
