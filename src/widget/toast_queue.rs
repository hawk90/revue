//! Toast queue manager for centralized toast notifications
//!
//! Manages a queue of toasts with deduplication, positioning, and stacking control.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{ToastQueue, ToastEntry, ToastLevel, QueuePosition};
//!
//! // Create a toast queue
//! let mut queue = ToastQueue::new()
//!     .position(QueuePosition::TopRight)
//!     .max_visible(3)
//!     .stack_direction(StackDirection::Down);
//!
//! // Add toasts
//! queue.push("File saved", ToastLevel::Success);
//! queue.push_with_id("error-1", "Connection failed", ToastLevel::Error);
//!
//! // In tick handler
//! queue.tick();
//! ```

use super::toast::{ToastLevel, ToastPosition};
use super::traits::{RenderContext, View, WidgetProps};
use crate::render::Cell;
use crate::style::Color;
use crate::{impl_props_builders, impl_styled_view};
use std::time::{Duration, Instant};

/// Stack direction for toasts
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StackDirection {
    /// New toasts appear below existing ones
    #[default]
    Down,
    /// New toasts appear above existing ones
    Up,
}

/// Priority level for toasts (higher = more important)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum ToastPriority {
    /// Low priority (can be suppressed)
    Low = 0,
    /// Normal priority
    #[default]
    Normal = 1,
    /// High priority (shows immediately)
    High = 2,
    /// Critical priority (cannot be dismissed)
    Critical = 3,
}

/// A toast entry in the queue
#[derive(Clone, Debug)]
pub struct ToastEntry {
    /// Unique ID for deduplication
    pub id: Option<String>,
    /// Toast message
    pub message: String,
    /// Toast level
    pub level: ToastLevel,
    /// Priority
    pub priority: ToastPriority,
    /// Duration to show (None = use default)
    pub duration: Option<Duration>,
    /// Time when toast was created
    pub created_at: Instant,
    /// Time when toast was shown
    pub shown_at: Option<Instant>,
    /// Whether toast is dismissible
    pub dismissible: bool,
}

impl ToastEntry {
    /// Create a new toast entry
    pub fn new(message: impl Into<String>, level: ToastLevel) -> Self {
        Self {
            id: None,
            message: message.into(),
            level,
            priority: ToastPriority::Normal,
            duration: None,
            created_at: Instant::now(),
            shown_at: None,
            dismissible: true,
        }
    }

    /// Set an ID for deduplication
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the priority
    pub fn with_priority(mut self, priority: ToastPriority) -> Self {
        self.priority = priority;
        self
    }

    /// Set custom duration
    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Set whether dismissible
    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    /// Check if this toast has expired
    fn is_expired(&self, default_duration: Duration) -> bool {
        if let Some(shown) = self.shown_at {
            let duration = self.duration.unwrap_or(default_duration);
            shown.elapsed() >= duration
        } else {
            false
        }
    }
}

/// Centralized toast queue manager
pub struct ToastQueue {
    /// Queue of pending toasts
    queue: Vec<ToastEntry>,
    /// Currently visible toasts
    visible: Vec<ToastEntry>,
    /// Queue position
    position: ToastPosition,
    /// Stack direction
    stack_direction: StackDirection,
    /// Maximum visible toasts
    max_visible: usize,
    /// Default duration for toasts
    default_duration: Duration,
    /// Gap between toasts
    gap: u16,
    /// Toast width
    toast_width: u16,
    /// Enable deduplication
    deduplicate: bool,
    /// Pause on hover (not yet implemented, placeholder)
    #[allow(dead_code)]
    pause_on_hover: bool,
    /// Widget properties
    props: WidgetProps,
}

impl ToastQueue {
    /// Create a new toast queue
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
            visible: Vec::new(),
            position: ToastPosition::TopRight,
            stack_direction: StackDirection::Down,
            max_visible: 5,
            default_duration: Duration::from_secs(4),
            gap: 1,
            toast_width: 40,
            deduplicate: true,
            pause_on_hover: false,
            props: WidgetProps::new(),
        }
    }

    /// Set the position
    pub fn position(mut self, position: ToastPosition) -> Self {
        self.position = position;
        self
    }

    /// Set stack direction
    pub fn stack_direction(mut self, direction: StackDirection) -> Self {
        self.stack_direction = direction;
        self
    }

    /// Set maximum visible toasts
    pub fn max_visible(mut self, max: usize) -> Self {
        self.max_visible = max;
        self
    }

    /// Set default duration
    pub fn default_duration(mut self, duration: Duration) -> Self {
        self.default_duration = duration;
        self
    }

    /// Set gap between toasts
    pub fn gap(mut self, gap: u16) -> Self {
        self.gap = gap;
        self
    }

    /// Set toast width
    pub fn toast_width(mut self, width: u16) -> Self {
        self.toast_width = width;
        self
    }

    /// Enable/disable deduplication
    pub fn deduplicate(mut self, deduplicate: bool) -> Self {
        self.deduplicate = deduplicate;
        self
    }

    /// Push a simple toast
    pub fn push(&mut self, message: impl Into<String>, level: ToastLevel) {
        self.push_entry(ToastEntry::new(message, level));
    }

    /// Push a toast with an ID for deduplication
    pub fn push_with_id(
        &mut self,
        id: impl Into<String>,
        message: impl Into<String>,
        level: ToastLevel,
    ) {
        self.push_entry(ToastEntry::new(message, level).with_id(id));
    }

    /// Push an info toast
    pub fn info(&mut self, message: impl Into<String>) {
        self.push(message, ToastLevel::Info);
    }

    /// Push a success toast
    pub fn success(&mut self, message: impl Into<String>) {
        self.push(message, ToastLevel::Success);
    }

    /// Push a warning toast
    pub fn warning(&mut self, message: impl Into<String>) {
        self.push(message, ToastLevel::Warning);
    }

    /// Push an error toast
    pub fn error(&mut self, message: impl Into<String>) {
        self.push(message, ToastLevel::Error);
    }

    /// Push a toast entry
    pub fn push_entry(&mut self, entry: ToastEntry) {
        // Check for duplicates
        if self.deduplicate {
            if let Some(ref id) = entry.id {
                // Check if ID already exists
                let exists = self.visible.iter().any(|t| t.id.as_ref() == Some(id))
                    || self.queue.iter().any(|t| t.id.as_ref() == Some(id));
                if exists {
                    return;
                }
            }
        }

        // Insert based on priority
        let pos = self
            .queue
            .iter()
            .position(|t| t.priority < entry.priority)
            .unwrap_or(self.queue.len());
        self.queue.insert(pos, entry);
    }

    /// Update the queue (call on each tick)
    pub fn tick(&mut self) {
        // Remove expired toasts
        self.visible
            .retain(|t| !t.is_expired(self.default_duration));

        // Move toasts from queue to visible
        while self.visible.len() < self.max_visible && !self.queue.is_empty() {
            let mut entry = self.queue.remove(0);
            entry.shown_at = Some(Instant::now());
            self.visible.push(entry);
        }
    }

    /// Dismiss a specific toast by ID
    pub fn dismiss(&mut self, id: &str) {
        self.visible.retain(|t| t.id.as_deref() != Some(id));
        self.queue.retain(|t| t.id.as_deref() != Some(id));
    }

    /// Dismiss the first visible toast
    pub fn dismiss_first(&mut self) {
        if !self.visible.is_empty() {
            let first = &self.visible[0];
            if first.dismissible {
                self.visible.remove(0);
            }
        }
    }

    /// Dismiss all toasts
    pub fn dismiss_all(&mut self) {
        self.visible.retain(|t| !t.dismissible);
        self.queue.retain(|t| !t.dismissible);
    }

    /// Clear all toasts (including non-dismissible)
    pub fn clear(&mut self) {
        self.visible.clear();
        self.queue.clear();
    }

    /// Get count of visible toasts
    pub fn visible_count(&self) -> usize {
        self.visible.len()
    }

    /// Get count of pending toasts
    pub fn pending_count(&self) -> usize {
        self.queue.len()
    }

    /// Get total toast count
    pub fn total_count(&self) -> usize {
        self.visible.len() + self.queue.len()
    }

    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        self.visible.is_empty() && self.queue.is_empty()
    }

    /// Calculate toast height
    fn toast_height(&self) -> u16 {
        3 // border + content
    }

    /// Calculate base position
    fn calculate_base_position(&self, area_width: u16, area_height: u16) -> (u16, u16) {
        let margin = 1u16;
        let toast_w = self.toast_width;
        let total_height = (self.visible.len() as u16) * (self.toast_height() + self.gap);

        let x = match self.position {
            ToastPosition::TopLeft | ToastPosition::BottomLeft => margin,
            ToastPosition::TopCenter | ToastPosition::BottomCenter => {
                area_width.saturating_sub(toast_w) / 2
            }
            ToastPosition::TopRight | ToastPosition::BottomRight => {
                area_width.saturating_sub(toast_w + margin)
            }
        };

        let y = match self.position {
            ToastPosition::TopLeft | ToastPosition::TopCenter | ToastPosition::TopRight => margin,
            ToastPosition::BottomLeft
            | ToastPosition::BottomCenter
            | ToastPosition::BottomRight => area_height.saturating_sub(total_height + margin),
        };

        (x, y)
    }

    /// Render a single toast
    fn render_toast(&self, ctx: &mut RenderContext, entry: &ToastEntry, x: u16, y: u16) {
        let area = ctx.area;
        let toast_w = self.toast_width.min(area.width.saturating_sub(x));
        let toast_h = self.toast_height();

        if x >= area.width || y >= area.height {
            return;
        }

        let color = entry.level.color();
        let bg = entry.level.bg_color();

        // Draw border
        // Top
        let mut top_left = Cell::new('╭');
        top_left.fg = Some(color);
        top_left.bg = Some(bg);
        ctx.buffer.set(x, y, top_left);

        for i in 1..toast_w.saturating_sub(1) {
            let mut cell = Cell::new('─');
            cell.fg = Some(color);
            cell.bg = Some(bg);
            ctx.buffer.set(x + i, y, cell);
        }

        let mut top_right = Cell::new('╮');
        top_right.fg = Some(color);
        top_right.bg = Some(bg);
        ctx.buffer.set(x + toast_w - 1, y, top_right);

        // Bottom
        let mut bottom_left = Cell::new('╰');
        bottom_left.fg = Some(color);
        bottom_left.bg = Some(bg);
        ctx.buffer.set(x, y + toast_h - 1, bottom_left);

        for i in 1..toast_w.saturating_sub(1) {
            let mut cell = Cell::new('─');
            cell.fg = Some(color);
            cell.bg = Some(bg);
            ctx.buffer.set(x + i, y + toast_h - 1, cell);
        }

        let mut bottom_right = Cell::new('╯');
        bottom_right.fg = Some(color);
        bottom_right.bg = Some(bg);
        ctx.buffer
            .set(x + toast_w - 1, y + toast_h - 1, bottom_right);

        // Sides and fill
        for row in 1..toast_h.saturating_sub(1) {
            let mut left = Cell::new('│');
            left.fg = Some(color);
            left.bg = Some(bg);
            ctx.buffer.set(x, y + row, left);

            let mut right = Cell::new('│');
            right.fg = Some(color);
            right.bg = Some(bg);
            ctx.buffer.set(x + toast_w - 1, y + row, right);

            for col in 1..toast_w.saturating_sub(1) {
                let mut fill = Cell::new(' ');
                fill.bg = Some(bg);
                ctx.buffer.set(x + col, y + row, fill);
            }
        }

        // Content
        let content_x = x + 2;
        let content_y = y + 1;

        // Icon
        let mut icon_cell = Cell::new(entry.level.icon());
        icon_cell.fg = Some(color);
        icon_cell.bg = Some(bg);
        ctx.buffer.set(content_x, content_y, icon_cell);

        // Message
        let msg_x = content_x + 2;
        let max_msg_len = (toast_w.saturating_sub(5)) as usize;
        for (i, ch) in entry.message.chars().take(max_msg_len).enumerate() {
            let mut cell = Cell::new(ch);
            cell.fg = Some(Color::WHITE);
            cell.bg = Some(bg);
            ctx.buffer.set(msg_x + i as u16, content_y, cell);
        }

        // Dismiss hint for dismissible toasts
        if entry.dismissible && toast_w > 10 {
            let dismiss_x = x + toast_w - 3;
            let mut dismiss = Cell::new('×');
            dismiss.fg = Some(Color::rgb(100, 100, 100));
            dismiss.bg = Some(bg);
            ctx.buffer.set(dismiss_x, content_y, dismiss);
        }
    }
}

impl Default for ToastQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl View for ToastQueue {
    crate::impl_view_meta!("ToastQueue");

    fn render(&self, ctx: &mut RenderContext) {
        if self.visible.is_empty() {
            return;
        }

        let area = ctx.area;
        let (base_x, base_y) = self.calculate_base_position(area.width, area.height);

        for (i, entry) in self.visible.iter().enumerate() {
            let offset = (i as u16) * (self.toast_height() + self.gap);
            let y = match self.stack_direction {
                StackDirection::Down => base_y + offset,
                StackDirection::Up => base_y.saturating_sub(offset),
            };

            if y < area.height {
                self.render_toast(ctx, entry, base_x, y);
            }
        }
    }
}

impl_styled_view!(ToastQueue);
impl_props_builders!(ToastQueue);

/// Create a new toast queue
pub fn toast_queue() -> ToastQueue {
    ToastQueue::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;
    use std::thread::sleep;

    #[test]
    fn test_toast_queue_new() {
        let queue = ToastQueue::new();
        assert!(queue.is_empty());
        assert_eq!(queue.max_visible, 5);
    }

    #[test]
    fn test_toast_queue_push() {
        let mut queue = ToastQueue::new();
        queue.push("Test message", ToastLevel::Info);
        assert_eq!(queue.pending_count(), 1);
    }

    #[test]
    fn test_toast_queue_tick() {
        let mut queue = ToastQueue::new();
        queue.push("Test", ToastLevel::Info);
        queue.tick();
        assert_eq!(queue.visible_count(), 1);
        assert_eq!(queue.pending_count(), 0);
    }

    #[test]
    fn test_toast_queue_max_visible() {
        let mut queue = ToastQueue::new().max_visible(2);
        for i in 0..5 {
            queue.push(format!("Toast {}", i), ToastLevel::Info);
        }
        queue.tick();
        assert_eq!(queue.visible_count(), 2);
        assert_eq!(queue.pending_count(), 3);
    }

    #[test]
    fn test_toast_queue_deduplication() {
        let mut queue = ToastQueue::new();
        queue.push_with_id("test-1", "Message 1", ToastLevel::Info);
        queue.push_with_id("test-1", "Message 1 duplicate", ToastLevel::Info);
        assert_eq!(queue.pending_count(), 1);
    }

    #[test]
    fn test_toast_queue_deduplication_disabled() {
        let mut queue = ToastQueue::new().deduplicate(false);
        queue.push_with_id("test-1", "Message 1", ToastLevel::Info);
        queue.push_with_id("test-1", "Message 1 duplicate", ToastLevel::Info);
        assert_eq!(queue.pending_count(), 2);
    }

    #[test]
    fn test_toast_queue_priority() {
        let mut queue = ToastQueue::new();
        queue
            .push_entry(ToastEntry::new("Low", ToastLevel::Info).with_priority(ToastPriority::Low));
        queue.push_entry(
            ToastEntry::new("High", ToastLevel::Info).with_priority(ToastPriority::High),
        );
        queue.push_entry(
            ToastEntry::new("Normal", ToastLevel::Info).with_priority(ToastPriority::Normal),
        );

        // High priority should be first
        assert_eq!(queue.queue[0].message, "High");
        assert_eq!(queue.queue[1].message, "Normal");
        assert_eq!(queue.queue[2].message, "Low");
    }

    #[test]
    fn test_toast_queue_dismiss() {
        let mut queue = ToastQueue::new();
        queue.push_with_id("test-1", "Message", ToastLevel::Info);
        queue.tick();
        assert_eq!(queue.visible_count(), 1);

        queue.dismiss("test-1");
        assert_eq!(queue.visible_count(), 0);
    }

    #[test]
    fn test_toast_queue_dismiss_all() {
        let mut queue = ToastQueue::new();
        queue.push("Toast 1", ToastLevel::Info);
        queue.push("Toast 2", ToastLevel::Info);
        queue.tick();
        assert_eq!(queue.visible_count(), 2);

        queue.dismiss_all();
        assert_eq!(queue.visible_count(), 0);
    }

    #[test]
    fn test_toast_queue_clear() {
        let mut queue = ToastQueue::new();
        queue.push_entry(ToastEntry::new("Non-dismissible", ToastLevel::Error).dismissible(false));
        queue.push("Dismissible", ToastLevel::Info);
        queue.tick();

        queue.dismiss_all();
        // Non-dismissible should remain
        assert_eq!(queue.visible_count(), 1);

        queue.clear();
        assert_eq!(queue.visible_count(), 0);
    }

    #[test]
    fn test_toast_queue_expiry() {
        let mut queue = ToastQueue::new().default_duration(Duration::from_millis(50));
        queue.push("Short lived", ToastLevel::Info);
        queue.tick();
        assert_eq!(queue.visible_count(), 1);

        sleep(Duration::from_millis(60));
        queue.tick();
        assert_eq!(queue.visible_count(), 0);
    }

    #[test]
    fn test_toast_queue_helpers() {
        let mut queue = ToastQueue::new();
        queue.info("Info");
        queue.success("Success");
        queue.warning("Warning");
        queue.error("Error");
        assert_eq!(queue.pending_count(), 4);
    }

    #[test]
    fn test_toast_queue_render() {
        let mut queue = ToastQueue::new();
        queue.push("Test toast", ToastLevel::Success);
        queue.tick();

        let mut buffer = Buffer::new(50, 20);
        let area = Rect::new(0, 0, 50, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);
        queue.render(&mut ctx);
    }

    #[test]
    fn test_toast_queue_render_empty() {
        let queue = ToastQueue::new();
        let mut buffer = Buffer::new(50, 20);
        let area = Rect::new(0, 0, 50, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);
        queue.render(&mut ctx);
    }

    #[test]
    fn test_toast_queue_positions() {
        let positions = [
            ToastPosition::TopLeft,
            ToastPosition::TopCenter,
            ToastPosition::TopRight,
            ToastPosition::BottomLeft,
            ToastPosition::BottomCenter,
            ToastPosition::BottomRight,
        ];

        for pos in positions {
            let mut queue = ToastQueue::new().position(pos);
            queue.push("Test", ToastLevel::Info);
            queue.tick();

            let mut buffer = Buffer::new(50, 20);
            let area = Rect::new(0, 0, 50, 20);
            let mut ctx = RenderContext::new(&mut buffer, area);
            queue.render(&mut ctx);
        }
    }

    #[test]
    fn test_toast_queue_stack_directions() {
        for dir in [StackDirection::Down, StackDirection::Up] {
            let mut queue = ToastQueue::new().stack_direction(dir);
            queue.push("Toast 1", ToastLevel::Info);
            queue.push("Toast 2", ToastLevel::Info);
            queue.tick();

            let mut buffer = Buffer::new(50, 20);
            let area = Rect::new(0, 0, 50, 20);
            let mut ctx = RenderContext::new(&mut buffer, area);
            queue.render(&mut ctx);
        }
    }

    #[test]
    fn test_toast_entry_builder() {
        let entry = ToastEntry::new("Test", ToastLevel::Info)
            .with_id("test-id")
            .with_priority(ToastPriority::High)
            .with_duration(Duration::from_secs(10))
            .dismissible(false);

        assert_eq!(entry.id, Some("test-id".to_string()));
        assert_eq!(entry.priority, ToastPriority::High);
        assert_eq!(entry.duration, Some(Duration::from_secs(10)));
        assert!(!entry.dismissible);
    }

    #[test]
    fn test_toast_queue_helper() {
        let queue = toast_queue();
        assert!(queue.is_empty());
    }

    #[test]
    fn test_toast_queue_default() {
        let queue = ToastQueue::default();
        assert!(queue.is_empty());
    }

    #[test]
    fn test_toast_queue_total_count() {
        let mut queue = ToastQueue::new().max_visible(1);
        queue.push("Toast 1", ToastLevel::Info);
        queue.push("Toast 2", ToastLevel::Info);
        queue.tick();

        assert_eq!(queue.visible_count(), 1);
        assert_eq!(queue.pending_count(), 1);
        assert_eq!(queue.total_count(), 2);
    }

    #[test]
    fn test_toast_queue_dismiss_first() {
        let mut queue = ToastQueue::new();
        queue.push("Toast 1", ToastLevel::Info);
        queue.push("Toast 2", ToastLevel::Info);
        queue.tick();

        queue.dismiss_first();
        assert_eq!(queue.visible_count(), 1);
    }
}
