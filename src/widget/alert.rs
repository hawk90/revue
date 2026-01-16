//! Alert widget for persistent in-place feedback messages
//!
//! Unlike Toast (ephemeral notifications), Alert stays visible until dismissed.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{Alert, AlertLevel, alert};
//!
//! // Basic alert
//! Alert::new("Operation completed successfully")
//!     .level(AlertLevel::Success);
//!
//! // With title and dismiss button
//! alert("Connection failed")
//!     .title("Network Error")
//!     .level(AlertLevel::Error)
//!     .dismissible(true);
//!
//! // Info alert with custom styling
//! Alert::info("Press Ctrl+S to save your work")
//!     .title("Tip");
//! ```

use super::border::{draw_border, BorderType};
use super::traits::{RenderContext, View, WidgetProps, WidgetState};
use crate::event::Key;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::{impl_props_builders, impl_state_builders, impl_styled_view};

/// Alert severity level
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AlertLevel {
    /// Informational message (blue)
    #[default]
    Info,
    /// Success message (green)
    Success,
    /// Warning message (yellow/orange)
    Warning,
    /// Error message (red)
    Error,
}

impl AlertLevel {
    /// Get the icon for this level
    pub fn icon(&self) -> char {
        match self {
            AlertLevel::Info => 'ℹ',
            AlertLevel::Success => '✓',
            AlertLevel::Warning => '⚠',
            AlertLevel::Error => '✗',
        }
    }

    /// Get the accent color for this level
    pub fn color(&self) -> Color {
        match self {
            AlertLevel::Info => Color::CYAN,
            AlertLevel::Success => Color::GREEN,
            AlertLevel::Warning => Color::YELLOW,
            AlertLevel::Error => Color::RED,
        }
    }

    /// Get the background color for this level
    pub fn bg_color(&self) -> Color {
        match self {
            AlertLevel::Info => Color::rgb(0, 30, 50),
            AlertLevel::Success => Color::rgb(0, 35, 0),
            AlertLevel::Warning => Color::rgb(50, 35, 0),
            AlertLevel::Error => Color::rgb(50, 0, 0),
        }
    }

    /// Get the border color for this level
    pub fn border_color(&self) -> Color {
        match self {
            AlertLevel::Info => Color::rgb(0, 100, 150),
            AlertLevel::Success => Color::rgb(0, 120, 0),
            AlertLevel::Warning => Color::rgb(180, 120, 0),
            AlertLevel::Error => Color::rgb(150, 0, 0),
        }
    }
}

/// Alert variant style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AlertVariant {
    /// Filled background with subtle color
    #[default]
    Filled,
    /// Only left border accent
    Outlined,
    /// Minimal style with just icon color
    Minimal,
}

/// A persistent alert/notification widget
///
/// Displays important messages that require user attention.
/// Unlike Toast, Alert stays visible until explicitly dismissed.
pub struct Alert {
    /// Alert message
    message: String,
    /// Optional title
    title: Option<String>,
    /// Severity level
    level: AlertLevel,
    /// Visual variant
    variant: AlertVariant,
    /// Show icon
    show_icon: bool,
    /// Allow dismissing
    dismissible: bool,
    /// Whether alert is dismissed
    dismissed: bool,
    /// Custom icon override
    custom_icon: Option<char>,
    /// Widget state
    state: WidgetState,
    /// Widget properties
    props: WidgetProps,
}

impl Alert {
    /// Create a new alert with a message
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            title: None,
            level: AlertLevel::default(),
            variant: AlertVariant::default(),
            show_icon: true,
            dismissible: false,
            dismissed: false,
            custom_icon: None,
            state: WidgetState::new(),
            props: WidgetProps::new(),
        }
    }

    /// Create an info alert
    pub fn info(message: impl Into<String>) -> Self {
        Self::new(message).level(AlertLevel::Info)
    }

    /// Create a success alert
    pub fn success(message: impl Into<String>) -> Self {
        Self::new(message).level(AlertLevel::Success)
    }

    /// Create a warning alert
    pub fn warning(message: impl Into<String>) -> Self {
        Self::new(message).level(AlertLevel::Warning)
    }

    /// Create an error alert
    pub fn error(message: impl Into<String>) -> Self {
        Self::new(message).level(AlertLevel::Error)
    }

    /// Set the alert title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the severity level
    pub fn level(mut self, level: AlertLevel) -> Self {
        self.level = level;
        self
    }

    /// Set the visual variant
    pub fn variant(mut self, variant: AlertVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Show/hide the icon
    pub fn icon(mut self, show: bool) -> Self {
        self.show_icon = show;
        self
    }

    /// Set a custom icon
    pub fn custom_icon(mut self, icon: char) -> Self {
        self.custom_icon = Some(icon);
        self.show_icon = true;
        self
    }

    /// Make the alert dismissible
    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    /// Check if alert is dismissed
    pub fn is_dismissed(&self) -> bool {
        self.dismissed
    }

    /// Dismiss the alert
    pub fn dismiss(&mut self) {
        self.dismissed = true;
    }

    /// Reset dismissed state (show again)
    pub fn reset(&mut self) {
        self.dismissed = false;
    }

    /// Get the icon to display
    fn get_icon(&self) -> char {
        self.custom_icon.unwrap_or_else(|| self.level.icon())
    }

    /// Calculate the height needed for this alert
    pub fn height(&self) -> u16 {
        if self.dismissed {
            return 0;
        }
        let has_title = self.title.is_some();
        match self.variant {
            AlertVariant::Filled | AlertVariant::Outlined => {
                if has_title {
                    4 // border + title + message + border
                } else {
                    3 // border + message + border
                }
            }
            AlertVariant::Minimal => {
                if has_title {
                    2
                } else {
                    1
                }
            }
        }
    }

    /// Handle keyboard input
    ///
    /// Returns `true` if the key was handled.
    pub fn handle_key(&mut self, key: &Key) -> bool {
        if self.dismissed || !self.dismissible {
            return false;
        }

        match key {
            Key::Char('x') | Key::Char('X') | Key::Escape => {
                self.dismiss();
                true
            }
            _ => false,
        }
    }
}

impl Default for Alert {
    fn default() -> Self {
        Self::new("Alert")
    }
}

impl View for Alert {
    crate::impl_view_meta!("Alert");

    fn render(&self, ctx: &mut RenderContext) {
        if self.dismissed {
            return;
        }

        let area = ctx.area;
        if area.width < 5 || area.height < 1 {
            return;
        }

        let accent_color = self.level.color();
        let bg_color = self.level.bg_color();
        let border_color = self.level.border_color();

        match self.variant {
            AlertVariant::Filled => {
                self.render_filled(ctx, accent_color, bg_color, border_color);
            }
            AlertVariant::Outlined => {
                self.render_outlined(ctx, accent_color, border_color);
            }
            AlertVariant::Minimal => {
                self.render_minimal(ctx, accent_color);
            }
        }
    }
}

impl Alert {
    fn render_filled(
        &self,
        ctx: &mut RenderContext,
        accent_color: Color,
        bg_color: Color,
        border_color: Color,
    ) {
        let area = ctx.area;

        // Fill background
        for y in area.y..area.y + area.height {
            for x in area.x..area.x + area.width {
                let mut cell = Cell::new(' ');
                cell.bg = Some(bg_color);
                ctx.buffer.set(x, y, cell);
            }
        }

        // Draw border
        self.draw_alert_border(ctx, border_color, bg_color);

        // Content area
        let content_x = area.x + 2;
        let content_width = area.width.saturating_sub(4);
        let mut y = area.y + 1;

        // Icon and title/message
        let icon_offset = if self.show_icon {
            let icon = self.get_icon();
            let mut icon_cell = Cell::new(icon);
            icon_cell.fg = Some(accent_color);
            icon_cell.bg = Some(bg_color);
            ctx.buffer.set(content_x, y, icon_cell);
            2
        } else {
            0
        };

        // Title (if present)
        if let Some(ref title) = self.title {
            let title_x = content_x + icon_offset;
            for (i, ch) in title.chars().enumerate() {
                if i as u16 >= content_width - icon_offset {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::WHITE);
                cell.bg = Some(bg_color);
                cell.modifier |= Modifier::BOLD;
                ctx.buffer.set(title_x + i as u16, y, cell);
            }
            y += 1;

            // Message on next line (indented to align with title)
            let msg_x = content_x + icon_offset;
            for (i, ch) in self.message.chars().enumerate() {
                if i as u16 >= content_width - icon_offset {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::rgb(200, 200, 200));
                cell.bg = Some(bg_color);
                ctx.buffer.set(msg_x + i as u16, y, cell);
            }
        } else {
            // Message only (same line as icon)
            let msg_x = content_x + icon_offset;
            for (i, ch) in self.message.chars().enumerate() {
                if i as u16 >= content_width - icon_offset {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::WHITE);
                cell.bg = Some(bg_color);
                ctx.buffer.set(msg_x + i as u16, y, cell);
            }
        }

        // Dismiss button
        if self.dismissible {
            let dismiss_x = area.x + area.width - 3;
            let mut x_cell = Cell::new('×');
            x_cell.fg = Some(Color::rgb(150, 150, 150));
            x_cell.bg = Some(bg_color);
            ctx.buffer.set(dismiss_x, area.y + 1, x_cell);
        }
    }

    fn render_outlined(&self, ctx: &mut RenderContext, accent_color: Color, _border_color: Color) {
        let area = ctx.area;

        // Draw left accent border
        for y in area.y..area.y + area.height {
            let mut cell = Cell::new('┃');
            cell.fg = Some(accent_color);
            ctx.buffer.set(area.x, y, cell);
        }

        // Content
        let content_x = area.x + 2;
        let content_width = area.width.saturating_sub(3);
        let mut y = area.y;

        // Icon
        let icon_offset = if self.show_icon {
            let icon = self.get_icon();
            let mut icon_cell = Cell::new(icon);
            icon_cell.fg = Some(accent_color);
            ctx.buffer.set(content_x, y, icon_cell);
            2
        } else {
            0
        };

        // Title
        if let Some(ref title) = self.title {
            let title_x = content_x + icon_offset;
            for (i, ch) in title.chars().enumerate() {
                if i as u16 >= content_width - icon_offset {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::WHITE);
                cell.modifier |= Modifier::BOLD;
                ctx.buffer.set(title_x + i as u16, y, cell);
            }
            y += 1;

            // Message
            let msg_x = content_x + icon_offset;
            for (i, ch) in self.message.chars().enumerate() {
                if i as u16 >= content_width - icon_offset {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::rgb(180, 180, 180));
                ctx.buffer.set(msg_x + i as u16, y, cell);
            }
        } else {
            let msg_x = content_x + icon_offset;
            for (i, ch) in self.message.chars().enumerate() {
                if i as u16 >= content_width - icon_offset {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::WHITE);
                ctx.buffer.set(msg_x + i as u16, y, cell);
            }
        }

        // Dismiss button
        if self.dismissible {
            let dismiss_x = area.x + area.width - 2;
            let mut x_cell = Cell::new('×');
            x_cell.fg = Some(Color::rgb(150, 150, 150));
            ctx.buffer.set(dismiss_x, area.y, x_cell);
        }
    }

    fn render_minimal(&self, ctx: &mut RenderContext, accent_color: Color) {
        let area = ctx.area;
        let mut x = area.x;
        let y = area.y;

        // Icon
        if self.show_icon {
            let icon = self.get_icon();
            let mut icon_cell = Cell::new(icon);
            icon_cell.fg = Some(accent_color);
            ctx.buffer.set(x, y, icon_cell);
            x += 2;
        }

        // Title or message
        if let Some(ref title) = self.title {
            // Title on first line
            for (i, ch) in title.chars().enumerate() {
                if x + i as u16 >= area.x + area.width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(accent_color);
                cell.modifier |= Modifier::BOLD;
                ctx.buffer.set(x + i as u16, y, cell);
            }

            // Message on second line
            if area.height > 1 {
                let msg_x = if self.show_icon { area.x + 2 } else { area.x };
                for (i, ch) in self.message.chars().enumerate() {
                    if msg_x + i as u16 >= area.x + area.width {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(Color::rgb(180, 180, 180));
                    ctx.buffer.set(msg_x + i as u16, y + 1, cell);
                }
            }
        } else {
            // Just message
            for (i, ch) in self.message.chars().enumerate() {
                if x + i as u16 >= area.x + area.width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::WHITE);
                ctx.buffer.set(x + i as u16, y, cell);
            }
        }

        // Dismiss button
        if self.dismissible {
            let dismiss_x = area.x + area.width - 1;
            let mut x_cell = Cell::new('×');
            x_cell.fg = Some(Color::rgb(100, 100, 100));
            ctx.buffer.set(dismiss_x, y, x_cell);
        }
    }

    fn draw_alert_border(&self, ctx: &mut RenderContext, border_color: Color, bg_color: Color) {
        // Use centralized border drawing utility
        draw_border(
            ctx.buffer,
            ctx.area,
            BorderType::Rounded,
            Some(border_color),
            Some(bg_color),
        );
    }
}

impl_styled_view!(Alert);
impl_state_builders!(Alert);
impl_props_builders!(Alert);

/// Helper function to create an Alert
pub fn alert(message: impl Into<String>) -> Alert {
    Alert::new(message)
}

/// Helper function to create an info Alert
pub fn info_alert(message: impl Into<String>) -> Alert {
    Alert::info(message)
}

/// Helper function to create a success Alert
pub fn success_alert(message: impl Into<String>) -> Alert {
    Alert::success(message)
}

/// Helper function to create a warning Alert
pub fn warning_alert(message: impl Into<String>) -> Alert {
    Alert::warning(message)
}

/// Helper function to create an error Alert
pub fn error_alert(message: impl Into<String>) -> Alert {
    Alert::error(message)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_alert_new() {
        let a = Alert::new("Test message");
        assert_eq!(a.message, "Test message");
        assert_eq!(a.level, AlertLevel::Info);
        assert!(a.title.is_none());
        assert!(!a.dismissible);
        assert!(!a.dismissed);
    }

    #[test]
    fn test_alert_builders() {
        let a = Alert::new("Message")
            .title("Title")
            .level(AlertLevel::Error)
            .variant(AlertVariant::Outlined)
            .dismissible(true)
            .icon(false);

        assert_eq!(a.title, Some("Title".to_string()));
        assert_eq!(a.level, AlertLevel::Error);
        assert_eq!(a.variant, AlertVariant::Outlined);
        assert!(a.dismissible);
        assert!(!a.show_icon);
    }

    #[test]
    fn test_alert_level_helpers() {
        assert_eq!(Alert::info("msg").level, AlertLevel::Info);
        assert_eq!(Alert::success("msg").level, AlertLevel::Success);
        assert_eq!(Alert::warning("msg").level, AlertLevel::Warning);
        assert_eq!(Alert::error("msg").level, AlertLevel::Error);
    }

    #[test]
    fn test_alert_dismiss() {
        let mut a = Alert::new("Test").dismissible(true);
        assert!(!a.is_dismissed());

        a.dismiss();
        assert!(a.is_dismissed());

        a.reset();
        assert!(!a.is_dismissed());
    }

    #[test]
    fn test_alert_handle_key() {
        let mut a = Alert::new("Test").dismissible(true);

        assert!(a.handle_key(&Key::Char('x')));
        assert!(a.is_dismissed());

        a.reset();
        assert!(a.handle_key(&Key::Escape));
        assert!(a.is_dismissed());
    }

    #[test]
    fn test_alert_handle_key_not_dismissible() {
        let mut a = Alert::new("Test").dismissible(false);
        assert!(!a.handle_key(&Key::Char('x')));
        assert!(!a.is_dismissed());
    }

    #[test]
    fn test_alert_height() {
        let minimal = Alert::new("msg").variant(AlertVariant::Minimal);
        assert_eq!(minimal.height(), 1);

        let minimal_title = Alert::new("msg")
            .title("Title")
            .variant(AlertVariant::Minimal);
        assert_eq!(minimal_title.height(), 2);

        let filled = Alert::new("msg").variant(AlertVariant::Filled);
        assert_eq!(filled.height(), 3);

        let filled_title = Alert::new("msg")
            .title("Title")
            .variant(AlertVariant::Filled);
        assert_eq!(filled_title.height(), 4);

        let dismissed = Alert::new("msg").dismissible(true);
        let mut dismissed = dismissed;
        dismissed.dismiss();
        assert_eq!(dismissed.height(), 0);
    }

    #[test]
    fn test_alert_level_colors() {
        assert_eq!(AlertLevel::Info.color(), Color::CYAN);
        assert_eq!(AlertLevel::Success.color(), Color::GREEN);
        assert_eq!(AlertLevel::Warning.color(), Color::YELLOW);
        assert_eq!(AlertLevel::Error.color(), Color::RED);
    }

    #[test]
    fn test_alert_level_icons() {
        assert_eq!(AlertLevel::Info.icon(), 'ℹ');
        assert_eq!(AlertLevel::Success.icon(), '✓');
        assert_eq!(AlertLevel::Warning.icon(), '⚠');
        assert_eq!(AlertLevel::Error.icon(), '✗');
    }

    #[test]
    fn test_alert_custom_icon() {
        let a = Alert::new("Test").custom_icon('★');
        assert_eq!(a.get_icon(), '★');
        assert!(a.show_icon);
    }

    #[test]
    fn test_alert_render_filled() {
        let mut buffer = Buffer::new(40, 5);
        let area = Rect::new(0, 0, 40, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let a = Alert::new("Test message").variant(AlertVariant::Filled);
        a.render(&mut ctx);

        // Check border corners
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '╭');
        assert_eq!(buffer.get(39, 0).unwrap().symbol, '╮');
    }

    #[test]
    fn test_alert_render_outlined() {
        let mut buffer = Buffer::new(40, 3);
        let area = Rect::new(0, 0, 40, 3);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let a = Alert::new("Test").variant(AlertVariant::Outlined);
        a.render(&mut ctx);

        // Check left accent border
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '┃');
        assert_eq!(buffer.get(0, 1).unwrap().symbol, '┃');
    }

    #[test]
    fn test_alert_render_minimal() {
        let mut buffer = Buffer::new(40, 2);
        let area = Rect::new(0, 0, 40, 2);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let a = Alert::new("Test").variant(AlertVariant::Minimal);
        a.render(&mut ctx);

        // Check icon
        assert_eq!(buffer.get(0, 0).unwrap().symbol, 'ℹ');
    }

    #[test]
    fn test_alert_render_dismissed() {
        let mut buffer = Buffer::new(40, 5);
        let area = Rect::new(0, 0, 40, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut a = Alert::new("Test");
        a.dismiss();
        a.render(&mut ctx);

        // Should not render anything (buffer should be default)
        assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
    }

    #[test]
    fn test_alert_render_with_title() {
        let mut buffer = Buffer::new(40, 5);
        let area = Rect::new(0, 0, 40, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let a = Alert::new("Message body")
            .title("Alert Title")
            .variant(AlertVariant::Filled);
        a.render(&mut ctx);

        // Smoke test - just ensure it doesn't panic
    }

    #[test]
    fn test_alert_helpers() {
        let a = alert("msg");
        assert_eq!(a.message, "msg");

        let i = info_alert("info");
        assert_eq!(i.level, AlertLevel::Info);

        let s = success_alert("success");
        assert_eq!(s.level, AlertLevel::Success);

        let w = warning_alert("warning");
        assert_eq!(w.level, AlertLevel::Warning);

        let e = error_alert("error");
        assert_eq!(e.level, AlertLevel::Error);
    }

    #[test]
    fn test_alert_default() {
        let a = Alert::default();
        assert_eq!(a.message, "Alert");
    }
}
