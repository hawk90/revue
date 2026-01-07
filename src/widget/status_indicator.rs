//! Status Indicator widget for displaying online/offline/busy states
//!
//! Provides visual feedback for connection status, user availability, or system health.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{StatusIndicator, Status, status_indicator};
//!
//! // Basic online indicator
//! StatusIndicator::online();
//!
//! // With label
//! StatusIndicator::busy().label("Do not disturb");
//!
//! // Custom status
//! status_indicator(Status::Away)
//!     .size(StatusSize::Large)
//!     .pulsing(true);
//! ```

use super::traits::{RenderContext, View, WidgetProps, WidgetState};
use crate::render::Cell;
use crate::style::Color;
use crate::{impl_props_builders, impl_state_builders, impl_styled_view};

/// Predefined status states
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Status {
    /// Online/available (green)
    #[default]
    Online,
    /// Offline/disconnected (gray)
    Offline,
    /// Busy/do not disturb (red)
    Busy,
    /// Away/idle (yellow)
    Away,
    /// Unknown/connecting (gray with question)
    Unknown,
    /// Error state (red with warning)
    Error,
    /// Custom status with a color
    Custom(Color),
}

impl Status {
    /// Get the color for this status
    pub fn color(&self) -> Color {
        match self {
            Status::Online => Color::rgb(34, 197, 94),    // Green
            Status::Offline => Color::rgb(107, 114, 128), // Gray
            Status::Busy => Color::rgb(239, 68, 68),      // Red
            Status::Away => Color::rgb(234, 179, 8),      // Yellow
            Status::Unknown => Color::rgb(156, 163, 175), // Light gray
            Status::Error => Color::rgb(220, 38, 38),     // Darker red
            Status::Custom(color) => *color,
        }
    }

    /// Get the default label for this status
    pub fn label(&self) -> &'static str {
        match self {
            Status::Online => "Online",
            Status::Offline => "Offline",
            Status::Busy => "Busy",
            Status::Away => "Away",
            Status::Unknown => "Unknown",
            Status::Error => "Error",
            Status::Custom(_) => "Custom",
        }
    }

    /// Get the icon for this status
    pub fn icon(&self) -> char {
        match self {
            Status::Online => '●',
            Status::Offline => '○',
            Status::Busy => '⊘',
            Status::Away => '◐',
            Status::Unknown => '?',
            Status::Error => '!',
            Status::Custom(_) => '●',
        }
    }
}

/// Size variants for the status indicator
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StatusSize {
    /// Small dot (1 char)
    Small,
    /// Medium dot (default)
    #[default]
    Medium,
    /// Large dot with more visual presence
    Large,
}

impl StatusSize {
    /// Get the dot character for this size
    pub fn dot(&self) -> char {
        match self {
            StatusSize::Small => '•',
            StatusSize::Medium => '●',
            StatusSize::Large => '⬤',
        }
    }

    /// Get the width for this size (for rendering with label)
    pub fn width(&self) -> u16 {
        match self {
            StatusSize::Small => 1,
            StatusSize::Medium => 1,
            StatusSize::Large => 2,
        }
    }
}

/// Status indicator style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StatusStyle {
    /// Just the dot indicator
    #[default]
    Dot,
    /// Dot with text label
    DotWithLabel,
    /// Text label only
    LabelOnly,
    /// Badge style (filled background)
    Badge,
}

/// A status indicator widget for displaying availability/connection states
///
/// Shows online/offline/busy states with consistent visual styling.
#[derive(Clone)]
pub struct StatusIndicator {
    /// Current status
    status: Status,
    /// Size variant
    size: StatusSize,
    /// Display style
    style: StatusStyle,
    /// Custom label (overrides default)
    custom_label: Option<String>,
    /// Enable pulsing animation
    pulsing: bool,
    /// Animation frame counter
    frame: usize,
    /// Widget state
    state: WidgetState,
    /// Widget properties
    props: WidgetProps,
}

impl StatusIndicator {
    /// Create a new status indicator with the given status
    pub fn new(status: Status) -> Self {
        Self {
            status,
            size: StatusSize::default(),
            style: StatusStyle::default(),
            custom_label: None,
            pulsing: false,
            frame: 0,
            state: WidgetState::new(),
            props: WidgetProps::new(),
        }
    }

    /// Create an online status indicator
    pub fn online() -> Self {
        Self::new(Status::Online)
    }

    /// Create an offline status indicator
    pub fn offline() -> Self {
        Self::new(Status::Offline)
    }

    /// Create a busy status indicator
    pub fn busy() -> Self {
        Self::new(Status::Busy)
    }

    /// Create an away status indicator
    pub fn away() -> Self {
        Self::new(Status::Away)
    }

    /// Create an unknown status indicator
    pub fn unknown() -> Self {
        Self::new(Status::Unknown)
    }

    /// Create an error status indicator
    pub fn error() -> Self {
        Self::new(Status::Error)
    }

    /// Create a custom status indicator with a specific color
    pub fn custom(color: Color) -> Self {
        Self::new(Status::Custom(color))
    }

    /// Set the status
    pub fn status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }

    /// Set the size
    pub fn size(mut self, size: StatusSize) -> Self {
        self.size = size;
        self
    }

    /// Set the display style
    pub fn indicator_style(mut self, style: StatusStyle) -> Self {
        self.style = style;
        self
    }

    /// Set a custom label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.custom_label = Some(label.into());
        self
    }

    /// Enable/disable pulsing animation
    pub fn pulsing(mut self, pulsing: bool) -> Self {
        self.pulsing = pulsing;
        self
    }

    /// Update animation frame
    pub fn tick(&mut self) {
        self.frame = self.frame.wrapping_add(1);
    }

    /// Get current status
    pub fn get_status(&self) -> Status {
        self.status
    }

    /// Set status mutably
    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    /// Get the label to display
    fn get_label(&self) -> &str {
        self.custom_label
            .as_deref()
            .unwrap_or_else(|| self.status.label())
    }

    /// Check if currently visible (for pulsing animation)
    fn is_visible(&self) -> bool {
        if !self.pulsing {
            return true;
        }
        // Pulse every 8 frames (visible for 6, hidden for 2)
        (self.frame % 8) < 6
    }

    /// Calculate total width needed
    pub fn width(&self) -> u16 {
        match self.style {
            StatusStyle::Dot => self.size.width(),
            StatusStyle::DotWithLabel => {
                let label_len = self.get_label().chars().count() as u16;
                self.size.width() + 1 + label_len // dot + space + label
            }
            StatusStyle::LabelOnly => self.get_label().chars().count() as u16,
            StatusStyle::Badge => {
                let label_len = self.get_label().chars().count() as u16;
                label_len + 4 // padding + dot + space + label + padding
            }
        }
    }
}

impl Default for StatusIndicator {
    fn default() -> Self {
        Self::new(Status::Online)
    }
}

impl View for StatusIndicator {
    crate::impl_view_meta!("StatusIndicator");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 1 || area.height < 1 {
            return;
        }

        let color = self.status.color();
        let visible = self.is_visible();

        match self.style {
            StatusStyle::Dot => {
                self.render_dot(ctx, color, visible);
            }
            StatusStyle::DotWithLabel => {
                self.render_dot_with_label(ctx, color, visible);
            }
            StatusStyle::LabelOnly => {
                self.render_label_only(ctx, color);
            }
            StatusStyle::Badge => {
                self.render_badge(ctx, color, visible);
            }
        }
    }
}

impl StatusIndicator {
    fn render_dot(&self, ctx: &mut RenderContext, color: Color, visible: bool) {
        let area = ctx.area;
        let dot = if visible { self.size.dot() } else { ' ' };

        let mut cell = Cell::new(dot);
        cell.fg = Some(color);
        ctx.buffer.set(area.x, area.y, cell);

        // For large size, add extra visual
        if self.size == StatusSize::Large && area.width > 1 {
            let mut cell2 = Cell::new(' ');
            cell2.bg = Some(color);
            ctx.buffer.set(area.x + 1, area.y, cell2);
        }
    }

    fn render_dot_with_label(&self, ctx: &mut RenderContext, color: Color, visible: bool) {
        let area = ctx.area;

        // Render dot
        let dot = if visible { self.size.dot() } else { ' ' };
        let mut dot_cell = Cell::new(dot);
        dot_cell.fg = Some(color);
        ctx.buffer.set(area.x, area.y, dot_cell);

        // Render label
        let label = self.get_label();
        let label_start = area.x + self.size.width() + 1;
        let max_label_width = area.width.saturating_sub(self.size.width() + 1);

        for (i, ch) in label.chars().enumerate() {
            if i as u16 >= max_label_width {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(Color::rgb(200, 200, 200));
            ctx.buffer.set(label_start + i as u16, area.y, cell);
        }
    }

    fn render_label_only(&self, ctx: &mut RenderContext, color: Color) {
        let area = ctx.area;
        let label = self.get_label();

        for (i, ch) in label.chars().enumerate() {
            if i as u16 >= area.width {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(color);
            ctx.buffer.set(area.x + i as u16, area.y, cell);
        }
    }

    fn render_badge(&self, ctx: &mut RenderContext, color: Color, visible: bool) {
        let area = ctx.area;
        let label = self.get_label();

        // Background
        let bg_color = Color::rgb(40, 40, 40);
        let total_width = self.width().min(area.width);

        for i in 0..total_width {
            let mut cell = Cell::new(' ');
            cell.bg = Some(bg_color);
            ctx.buffer.set(area.x + i, area.y, cell);
        }

        // Dot
        let dot = if visible { '●' } else { ' ' };
        let mut dot_cell = Cell::new(dot);
        dot_cell.fg = Some(color);
        dot_cell.bg = Some(bg_color);
        ctx.buffer.set(area.x + 1, area.y, dot_cell);

        // Label
        let label_start = area.x + 3;
        for (i, ch) in label.chars().enumerate() {
            if label_start + i as u16 >= area.x + total_width - 1 {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(Color::WHITE);
            cell.bg = Some(bg_color);
            ctx.buffer.set(label_start + i as u16, area.y, cell);
        }
    }
}

impl_styled_view!(StatusIndicator);
impl_state_builders!(StatusIndicator);
impl_props_builders!(StatusIndicator);

/// Helper function to create a StatusIndicator
pub fn status_indicator(status: Status) -> StatusIndicator {
    StatusIndicator::new(status)
}

/// Helper function to create an online indicator
pub fn online() -> StatusIndicator {
    StatusIndicator::online()
}

/// Helper function to create an offline indicator
pub fn offline() -> StatusIndicator {
    StatusIndicator::offline()
}

/// Helper function to create a busy indicator
pub fn busy_indicator() -> StatusIndicator {
    StatusIndicator::busy()
}

/// Helper function to create an away indicator
pub fn away_indicator() -> StatusIndicator {
    StatusIndicator::away()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_status_indicator_new() {
        let s = StatusIndicator::new(Status::Online);
        assert_eq!(s.status, Status::Online);
        assert_eq!(s.size, StatusSize::Medium);
        assert_eq!(s.style, StatusStyle::Dot);
    }

    #[test]
    fn test_status_helpers() {
        assert_eq!(StatusIndicator::online().status, Status::Online);
        assert_eq!(StatusIndicator::offline().status, Status::Offline);
        assert_eq!(StatusIndicator::busy().status, Status::Busy);
        assert_eq!(StatusIndicator::away().status, Status::Away);
        assert_eq!(StatusIndicator::unknown().status, Status::Unknown);
        assert_eq!(StatusIndicator::error().status, Status::Error);
    }

    #[test]
    fn test_status_custom() {
        let custom = StatusIndicator::custom(Color::MAGENTA);
        assert!(matches!(custom.status, Status::Custom(_)));
    }

    #[test]
    fn test_status_builders() {
        let s = StatusIndicator::new(Status::Online)
            .size(StatusSize::Large)
            .indicator_style(StatusStyle::DotWithLabel)
            .label("Available")
            .pulsing(true);

        assert_eq!(s.size, StatusSize::Large);
        assert_eq!(s.style, StatusStyle::DotWithLabel);
        assert_eq!(s.custom_label, Some("Available".to_string()));
        assert!(s.pulsing);
    }

    #[test]
    fn test_status_colors() {
        assert_eq!(Status::Online.color(), Color::rgb(34, 197, 94));
        assert_eq!(Status::Offline.color(), Color::rgb(107, 114, 128));
        assert_eq!(Status::Busy.color(), Color::rgb(239, 68, 68));
        assert_eq!(Status::Away.color(), Color::rgb(234, 179, 8));
    }

    #[test]
    fn test_status_labels() {
        assert_eq!(Status::Online.label(), "Online");
        assert_eq!(Status::Offline.label(), "Offline");
        assert_eq!(Status::Busy.label(), "Busy");
        assert_eq!(Status::Away.label(), "Away");
        assert_eq!(Status::Unknown.label(), "Unknown");
        assert_eq!(Status::Error.label(), "Error");
    }

    #[test]
    fn test_status_icons() {
        assert_eq!(Status::Online.icon(), '●');
        assert_eq!(Status::Offline.icon(), '○');
        assert_eq!(Status::Busy.icon(), '⊘');
        assert_eq!(Status::Away.icon(), '◐');
    }

    #[test]
    fn test_size_dots() {
        assert_eq!(StatusSize::Small.dot(), '•');
        assert_eq!(StatusSize::Medium.dot(), '●');
        assert_eq!(StatusSize::Large.dot(), '⬤');
    }

    #[test]
    fn test_status_width() {
        let dot_only = StatusIndicator::online();
        assert_eq!(dot_only.width(), 1);

        let with_label = StatusIndicator::online().indicator_style(StatusStyle::DotWithLabel);
        assert!(with_label.width() > 1);

        let label_only = StatusIndicator::online().indicator_style(StatusStyle::LabelOnly);
        assert_eq!(label_only.width(), "Online".len() as u16);
    }

    #[test]
    fn test_status_tick() {
        let mut s = StatusIndicator::online().pulsing(true);
        assert_eq!(s.frame, 0);
        s.tick();
        assert_eq!(s.frame, 1);
    }

    #[test]
    fn test_status_pulsing_visibility() {
        let mut s = StatusIndicator::online().pulsing(true);
        assert!(s.is_visible()); // frame 0 is visible

        for _ in 0..6 {
            s.tick();
        }
        assert!(!s.is_visible()); // frame 6 is not visible

        s.tick();
        s.tick();
        assert!(s.is_visible()); // frame 8 wraps to visible again
    }

    #[test]
    fn test_status_set_get() {
        let mut s = StatusIndicator::online();
        assert_eq!(s.get_status(), Status::Online);

        s.set_status(Status::Busy);
        assert_eq!(s.get_status(), Status::Busy);
    }

    #[test]
    fn test_status_render_dot() {
        let mut buffer = Buffer::new(10, 1);
        let area = Rect::new(0, 0, 10, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let s = StatusIndicator::online();
        s.render(&mut ctx);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, '●');
    }

    #[test]
    fn test_status_render_with_label() {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let s = StatusIndicator::online().indicator_style(StatusStyle::DotWithLabel);
        s.render(&mut ctx);

        // Should have dot and label
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '●');
        assert_eq!(buffer.get(2, 0).unwrap().symbol, 'O'); // "Online" starts
    }

    #[test]
    fn test_status_render_label_only() {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let s = StatusIndicator::busy().indicator_style(StatusStyle::LabelOnly);
        s.render(&mut ctx);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, 'B'); // "Busy" starts
    }

    #[test]
    fn test_status_render_badge() {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let s = StatusIndicator::online().indicator_style(StatusStyle::Badge);
        s.render(&mut ctx);

        // Badge has background and dot
        assert_eq!(buffer.get(1, 0).unwrap().symbol, '●');
    }

    #[test]
    fn test_helper_functions() {
        let s = status_indicator(Status::Away);
        assert_eq!(s.status, Status::Away);

        let o = online();
        assert_eq!(o.status, Status::Online);

        let off = offline();
        assert_eq!(off.status, Status::Offline);

        let b = busy_indicator();
        assert_eq!(b.status, Status::Busy);

        let a = away_indicator();
        assert_eq!(a.status, Status::Away);
    }

    #[test]
    fn test_status_default() {
        let s = StatusIndicator::default();
        assert_eq!(s.status, Status::Online);
    }

    #[test]
    fn test_custom_label() {
        let s = StatusIndicator::online().label("Available now");
        assert_eq!(s.get_label(), "Available now");

        let s2 = StatusIndicator::online();
        assert_eq!(s2.get_label(), "Online");
    }
}
