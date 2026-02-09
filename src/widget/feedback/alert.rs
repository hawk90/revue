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

use crate::event::Key;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::layout::border::{draw_border, BorderType};
use crate::widget::traits::{RenderContext, View, WidgetProps, WidgetState};
use crate::{impl_styled_view, impl_widget_builders};

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
impl_widget_builders!(Alert);

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

// Most tests moved to tests/widget_tests.rs
// Tests below access private fields and must stay inline

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_alert_custom_icon() {
        let a = Alert::new("Test").custom_icon('★');
        assert_eq!(a.get_icon(), '★');
        assert!(a.show_icon);
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

    // =========================================================================
    // AlertLevel enum tests
    // =========================================================================

    #[test]
    fn test_alert_level_default() {
        let level = AlertLevel::default();
        assert_eq!(level, AlertLevel::Info);
    }

    #[test]
    fn test_alert_level_clone() {
        let level = AlertLevel::Warning;
        let cloned = level.clone();
        assert_eq!(level, cloned);
    }

    #[test]
    fn test_alert_level_copy() {
        let level1 = AlertLevel::Error;
        let level2 = level1;
        assert_eq!(level1, AlertLevel::Error);
        assert_eq!(level2, AlertLevel::Error);
    }

    #[test]
    fn test_alert_level_partial_eq() {
        assert_eq!(AlertLevel::Info, AlertLevel::Info);
        assert_ne!(AlertLevel::Info, AlertLevel::Success);
    }

    #[test]
    fn test_alert_level_debug() {
        let level = AlertLevel::Warning;
        assert!(format!("{:?}", level).contains("Warning"));
    }

    #[test]
    fn test_alert_level_icon_info() {
        assert_eq!(AlertLevel::Info.icon(), 'ℹ');
    }

    #[test]
    fn test_alert_level_icon_success() {
        assert_eq!(AlertLevel::Success.icon(), '✓');
    }

    #[test]
    fn test_alert_level_icon_warning() {
        assert_eq!(AlertLevel::Warning.icon(), '⚠');
    }

    #[test]
    fn test_alert_level_icon_error() {
        assert_eq!(AlertLevel::Error.icon(), '✗');
    }

    #[test]
    fn test_alert_level_color_info() {
        assert_eq!(AlertLevel::Info.color(), Color::CYAN);
    }

    #[test]
    fn test_alert_level_color_success() {
        assert_eq!(AlertLevel::Success.color(), Color::GREEN);
    }

    #[test]
    fn test_alert_level_color_warning() {
        assert_eq!(AlertLevel::Warning.color(), Color::YELLOW);
    }

    #[test]
    fn test_alert_level_color_error() {
        assert_eq!(AlertLevel::Error.color(), Color::RED);
    }

    #[test]
    fn test_alert_level_bg_color_info() {
        assert_eq!(AlertLevel::Info.bg_color(), Color::rgb(0, 30, 50));
    }

    #[test]
    fn test_alert_level_bg_color_success() {
        assert_eq!(AlertLevel::Success.bg_color(), Color::rgb(0, 35, 0));
    }

    #[test]
    fn test_alert_level_bg_color_warning() {
        assert_eq!(AlertLevel::Warning.bg_color(), Color::rgb(50, 35, 0));
    }

    #[test]
    fn test_alert_level_bg_color_error() {
        assert_eq!(AlertLevel::Error.bg_color(), Color::rgb(50, 0, 0));
    }

    #[test]
    fn test_alert_level_border_color_info() {
        assert_eq!(AlertLevel::Info.border_color(), Color::rgb(0, 100, 150));
    }

    #[test]
    fn test_alert_level_border_color_success() {
        assert_eq!(AlertLevel::Success.border_color(), Color::rgb(0, 120, 0));
    }

    #[test]
    fn test_alert_level_border_color_warning() {
        assert_eq!(AlertLevel::Warning.border_color(), Color::rgb(180, 120, 0));
    }

    #[test]
    fn test_alert_level_border_color_error() {
        assert_eq!(AlertLevel::Error.border_color(), Color::rgb(150, 0, 0));
    }

    // =========================================================================
    // AlertVariant enum tests
    // =========================================================================

    #[test]
    fn test_alert_variant_default() {
        let variant = AlertVariant::default();
        assert_eq!(variant, AlertVariant::Filled);
    }

    #[test]
    fn test_alert_variant_clone() {
        let variant = AlertVariant::Outlined;
        let cloned = variant.clone();
        assert_eq!(variant, cloned);
    }

    #[test]
    fn test_alert_variant_copy() {
        let variant1 = AlertVariant::Minimal;
        let variant2 = variant1;
        assert_eq!(variant1, AlertVariant::Minimal);
        assert_eq!(variant2, AlertVariant::Minimal);
    }

    #[test]
    fn test_alert_variant_partial_eq() {
        assert_eq!(AlertVariant::Filled, AlertVariant::Filled);
        assert_ne!(AlertVariant::Filled, AlertVariant::Outlined);
    }

    #[test]
    fn test_alert_variant_debug() {
        let variant = AlertVariant::Minimal;
        assert!(format!("{:?}", variant).contains("Minimal"));
    }

    // =========================================================================
    // Alert builder method tests
    // =========================================================================

    #[test]
    fn test_alert_title() {
        let a = Alert::new("Message").title("My Title");
        assert_eq!(a.title, Some("My Title".to_string()));
    }

    #[test]
    fn test_alert_level_builder() {
        let a = Alert::new("Message").level(AlertLevel::Error);
        assert_eq!(a.level, AlertLevel::Error);
    }

    #[test]
    fn test_alert_variant_builder() {
        let a = Alert::new("Message").variant(AlertVariant::Minimal);
        assert_eq!(a.variant, AlertVariant::Minimal);
    }

    #[test]
    fn test_alert_icon_show() {
        let a = Alert::new("Message").icon(true);
        assert!(a.show_icon);
    }

    #[test]
    fn test_alert_icon_hide() {
        let a = Alert::new("Message").icon(false);
        assert!(!a.show_icon);
    }

    #[test]
    fn test_alert_dismissible_true() {
        let a = Alert::new("Message").dismissible(true);
        assert!(a.dismissible);
    }

    #[test]
    fn test_alert_dismissible_false() {
        let a = Alert::new("Message").dismissible(false);
        assert!(!a.dismissible);
    }

    // =========================================================================
    // Alert state method tests
    // =========================================================================

    #[test]
    fn test_alert_is_dismissed_initial() {
        let a = Alert::new("Message");
        assert!(!a.is_dismissed());
    }

    #[test]
    fn test_alert_dismiss() {
        let mut a = Alert::new("Message");
        a.dismiss();
        assert!(a.is_dismissed());
    }

    #[test]
    fn test_alert_reset() {
        let mut a = Alert::new("Message");
        a.dismiss();
        a.reset();
        assert!(!a.is_dismissed());
    }

    #[test]
    fn test_alert_dismiss_then_reset() {
        let mut a = Alert::new("Message");
        assert!(!a.is_dismissed());
        a.dismiss();
        assert!(a.is_dismissed());
        a.reset();
        assert!(!a.is_dismissed());
    }

    #[test]
    fn test_alert_height_dismissed() {
        let mut a = Alert::new("Message");
        a.dismiss();
        assert_eq!(a.height(), 0);
    }

    #[test]
    fn test_alert_height_filled_no_title() {
        let a = Alert::new("Message").variant(AlertVariant::Filled);
        assert_eq!(a.height(), 3);
    }

    #[test]
    fn test_alert_height_filled_with_title() {
        let a = Alert::new("Message")
            .title("Title")
            .variant(AlertVariant::Filled);
        assert_eq!(a.height(), 4);
    }

    #[test]
    fn test_alert_height_outlined_no_title() {
        let a = Alert::new("Message").variant(AlertVariant::Outlined);
        assert_eq!(a.height(), 3);
    }

    #[test]
    fn test_alert_height_outlined_with_title() {
        let a = Alert::new("Message")
            .title("Title")
            .variant(AlertVariant::Outlined);
        assert_eq!(a.height(), 4);
    }

    #[test]
    fn test_alert_height_minimal_no_title() {
        let a = Alert::new("Message").variant(AlertVariant::Minimal);
        assert_eq!(a.height(), 1);
    }

    #[test]
    fn test_alert_height_minimal_with_title() {
        let a = Alert::new("Message")
            .title("Title")
            .variant(AlertVariant::Minimal);
        assert_eq!(a.height(), 2);
    }

    // =========================================================================
    // Alert::handle_key tests
    // =========================================================================

    #[test]
    fn test_handle_key_dismissed() {
        let mut a = Alert::new("Message").dismissible(true);
        a.dismiss();
        assert!(!a.handle_key(&Key::Char('x')));
    }

    #[test]
    fn test_handle_key_not_dismissible() {
        let mut a = Alert::new("Message");
        assert!(!a.handle_key(&Key::Char('x')));
    }

    #[test]
    fn test_handle_key_lowercase_x() {
        let mut a = Alert::new("Message").dismissible(true);
        assert!(a.handle_key(&Key::Char('x')));
        assert!(a.is_dismissed());
    }

    #[test]
    fn test_handle_key_uppercase_x() {
        let mut a = Alert::new("Message").dismissible(true);
        assert!(a.handle_key(&Key::Char('X')));
        assert!(a.is_dismissed());
    }

    #[test]
    fn test_handle_key_escape() {
        let mut a = Alert::new("Message").dismissible(true);
        assert!(a.handle_key(&Key::Escape));
        assert!(a.is_dismissed());
    }

    #[test]
    fn test_handle_key_other_key() {
        let mut a = Alert::new("Message").dismissible(true);
        assert!(!a.handle_key(&Key::Char('a')));
        assert!(!a.is_dismissed());
    }

    // =========================================================================
    // Builder chain tests
    // =========================================================================

    #[test]
    fn test_alert_builder_chain() {
        let a = Alert::new("Chain message")
            .title("Chain Title")
            .level(AlertLevel::Warning)
            .variant(AlertVariant::Outlined)
            .dismissible(true)
            .icon(true);
        assert_eq!(a.message, "Chain message");
        assert_eq!(a.title, Some("Chain Title".to_string()));
        assert_eq!(a.level, AlertLevel::Warning);
        assert_eq!(a.variant, AlertVariant::Outlined);
        assert!(a.dismissible);
        assert!(a.show_icon);
    }

    #[test]
    fn test_alert_info_chain() {
        let a = Alert::info("Info message")
            .title("Info Title")
            .dismissible(true);
        assert_eq!(a.level, AlertLevel::Info);
        assert_eq!(a.title, Some("Info Title".to_string()));
        assert!(a.dismissible);
    }

    #[test]
    fn test_alert_error_chain() {
        let a = Alert::error("Error message")
            .variant(AlertVariant::Minimal)
            .icon(false);
        assert_eq!(a.level, AlertLevel::Error);
        assert_eq!(a.variant, AlertVariant::Minimal);
        assert!(!a.show_icon);
    }

    // =========================================================================
    // Helper function tests
    // =========================================================================

    #[test]
    fn test_alert_helper_fn() {
        let a = alert("Helper message");
        assert_eq!(a.message, "Helper message");
        assert_eq!(a.level, AlertLevel::Info);
    }

    #[test]
    fn test_info_alert_fn() {
        let a = info_alert("Info");
        assert_eq!(a.level, AlertLevel::Info);
    }

    #[test]
    fn test_success_alert_fn() {
        let a = success_alert("Success");
        assert_eq!(a.level, AlertLevel::Success);
    }

    #[test]
    fn test_warning_alert_fn() {
        let a = warning_alert("Warning");
        assert_eq!(a.level, AlertLevel::Warning);
    }

    #[test]
    fn test_error_alert_fn() {
        let a = error_alert("Error");
        assert_eq!(a.level, AlertLevel::Error);
    }
}
