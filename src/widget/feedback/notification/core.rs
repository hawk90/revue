//! Notification Center implementation

use super::types::{Notification, NotificationPosition};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Notification Center widget
pub struct NotificationCenter {
    /// Active notifications
    notifications: Vec<Notification>,
    /// Maximum visible notifications
    max_visible: usize,
    /// Position on screen
    position: NotificationPosition,
    /// Notification width
    width: u16,
    /// Show icons
    show_icons: bool,
    /// Show progress timer
    show_timer: bool,
    /// Spacing between notifications
    spacing: u16,
    /// Current tick counter
    tick_counter: u64,
    /// Selected notification (for dismissal)
    selected: Option<usize>,
    /// Focused state
    focused: bool,
    /// Widget properties
    props: WidgetProps,
}

impl NotificationCenter {
    /// Create a new notification center
    pub fn new() -> Self {
        Self {
            notifications: Vec::new(),
            max_visible: 5,
            position: NotificationPosition::TopRight,
            width: 40,
            show_icons: true,
            show_timer: true,
            spacing: 1,
            tick_counter: 0,
            selected: None,
            focused: false,
            props: WidgetProps::new(),
        }
    }

    /// Set position
    pub fn position(mut self, position: NotificationPosition) -> Self {
        self.position = position;
        self
    }

    /// Set max visible
    pub fn max_visible(mut self, max: usize) -> Self {
        self.max_visible = max.max(1);
        self
    }

    /// Set width
    pub fn width(mut self, width: u16) -> Self {
        self.width = width.max(20);
        self
    }

    /// Show/hide icons
    pub fn show_icons(mut self, show: bool) -> Self {
        self.show_icons = show;
        self
    }

    /// Show/hide timer
    pub fn show_timer(mut self, show: bool) -> Self {
        self.show_timer = show;
        self
    }

    /// Set spacing
    pub fn spacing(mut self, spacing: u16) -> Self {
        self.spacing = spacing;
        self
    }

    /// Set focused state
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Push a new notification
    pub fn push(&mut self, mut notification: Notification) {
        notification.created_at = self.tick_counter;
        self.notifications.push(notification);
    }

    /// Push info notification
    pub fn info(&mut self, message: impl Into<String>) {
        self.push(Notification::info(message));
    }

    /// Push success notification
    pub fn success(&mut self, message: impl Into<String>) {
        self.push(Notification::success(message));
    }

    /// Push warning notification
    pub fn warning(&mut self, message: impl Into<String>) {
        self.push(Notification::warning(message));
    }

    /// Push error notification
    pub fn error(&mut self, message: impl Into<String>) {
        self.push(Notification::error(message));
    }

    /// Dismiss notification by ID
    pub fn dismiss(&mut self, id: u64) {
        self.notifications.retain(|n| n.id != id);
        if self.selected.is_some_and(|s| s >= self.notifications.len()) {
            self.selected = if self.notifications.is_empty() {
                None
            } else {
                Some(self.notifications.len() - 1)
            };
        }
    }

    /// Dismiss selected notification
    pub fn dismiss_selected(&mut self) {
        if let Some(idx) = self.selected {
            if idx < self.notifications.len() {
                let id = self.notifications[idx].id;
                self.dismiss(id);
            }
        }
    }

    /// Clear all notifications
    pub fn clear(&mut self) {
        self.notifications.clear();
        self.selected = None;
    }

    /// Get notification count
    pub fn count(&self) -> usize {
        self.notifications.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.notifications.is_empty()
    }

    /// Tick - update timers and remove expired
    pub fn tick(&mut self) {
        self.tick_counter += 1;

        for notification in &mut self.notifications {
            notification.tick += 1;
        }

        self.notifications.retain(|n| !n.is_expired());

        // Adjust selection
        if self.selected.is_some_and(|s| s >= self.notifications.len()) {
            self.selected = if self.notifications.is_empty() {
                None
            } else {
                Some(self.notifications.len() - 1)
            };
        }
    }

    /// Select next notification
    pub fn select_next(&mut self) {
        if self.notifications.is_empty() {
            self.selected = None;
            return;
        }

        self.selected = Some(match self.selected {
            Some(idx) => (idx + 1) % self.notifications.len(),
            None => 0,
        });
    }

    /// Select previous notification
    pub fn select_prev(&mut self) {
        if self.notifications.is_empty() {
            self.selected = None;
            return;
        }

        self.selected = Some(match self.selected {
            Some(0) => self.notifications.len() - 1,
            Some(idx) => idx - 1,
            None => self.notifications.len() - 1,
        });
    }

    /// Handle key input
    pub fn handle_key(&mut self, key: &crate::event::Key) -> bool {
        use crate::event::Key;

        if !self.focused || self.notifications.is_empty() {
            return false;
        }

        match key {
            Key::Up | Key::Char('k') => {
                self.select_prev();
                true
            }
            Key::Down | Key::Char('j') => {
                self.select_next();
                true
            }
            Key::Char('d') | Key::Delete => {
                self.dismiss_selected();
                true
            }
            Key::Char('c') => {
                self.clear();
                true
            }
            _ => false,
        }
    }

    /// Calculate notification height
    fn notification_height(&self, notification: &Notification) -> u16 {
        let mut height = 1; // Message line
        if notification.title.is_some() {
            height += 1;
        }
        if notification.progress.is_some() {
            height += 1;
        }
        if notification.action.is_some() {
            height += 1;
        }
        height + 2 // Border top and bottom
    }
}

impl Default for NotificationCenter {
    fn default() -> Self {
        Self::new()
    }
}

impl View for NotificationCenter {
    crate::impl_view_meta!("NotificationCenter");

    fn render(&self, ctx: &mut RenderContext) {
        if self.notifications.is_empty() {
            return;
        }

        let area = ctx.area;
        let visible = self
            .notifications
            .iter()
            .rev()
            .take(self.max_visible)
            .collect::<Vec<_>>();

        // Calculate starting position based on notification position
        let (start_x, mut current_y, direction): (u16, u16, i16) = match self.position {
            NotificationPosition::TopRight => {
                (area.x + area.width.saturating_sub(self.width), area.y, 1)
            }
            NotificationPosition::TopLeft => (area.x, area.y, 1),
            NotificationPosition::TopCenter => (
                area.x + (area.width.saturating_sub(self.width)) / 2,
                area.y,
                1,
            ),
            NotificationPosition::BottomRight => (
                area.x + area.width.saturating_sub(self.width),
                area.y + area.height,
                -1,
            ),
            NotificationPosition::BottomLeft => (area.x, area.y + area.height, -1),
            NotificationPosition::BottomCenter => (
                area.x + (area.width.saturating_sub(self.width)) / 2,
                area.y + area.height,
                -1,
            ),
        };

        // Render each notification
        for (idx, notification) in visible.iter().enumerate() {
            let height = self.notification_height(notification);
            let is_selected = self.selected == Some(self.notifications.len() - 1 - idx);

            // Adjust Y position for bottom positions
            let y = if direction < 0 {
                current_y.saturating_sub(height)
            } else {
                current_y
            };

            if y >= area.y + area.height || y + height > area.y + area.height {
                continue;
            }

            self.render_notification(ctx, notification, start_x, y, is_selected);

            if direction < 0 {
                current_y = y.saturating_sub(self.spacing);
            } else {
                current_y = y + height + self.spacing;
            }
        }
    }
}

impl NotificationCenter {
    fn render_notification(
        &self,
        ctx: &mut RenderContext,
        notification: &Notification,
        x: u16,
        y: u16,
        is_selected: bool,
    ) {
        let width = self.width;
        let color = notification.level.color();
        let bg = notification.level.bg_color();
        let border_color = if is_selected { Color::WHITE } else { color };

        // Top border
        let mut tl = Cell::new('╭');
        tl.fg = Some(border_color);
        ctx.buffer.set(x, y, tl);

        for dx in 1..width - 1 {
            let mut h = Cell::new('─');
            h.fg = Some(border_color);
            ctx.buffer.set(x + dx, y, h);
        }

        let mut tr = Cell::new('╮');
        tr.fg = Some(border_color);
        ctx.buffer.set(x + width - 1, y, tr);

        let mut current_y = y + 1;

        // Title line (if present)
        if let Some(ref title) = notification.title {
            let mut left = Cell::new('│');
            left.fg = Some(border_color);
            ctx.buffer.set(x, current_y, left);

            // Fill background
            for dx in 1..width - 1 {
                let mut cell = Cell::new(' ');
                cell.bg = Some(bg);
                ctx.buffer.set(x + dx, current_y, cell);
            }

            // Icon
            let mut content_x = x + 1;
            if self.show_icons {
                let mut icon = Cell::new(notification.level.icon());
                icon.fg = Some(color);
                icon.bg = Some(bg);
                ctx.buffer.set(content_x, current_y, icon);
                content_x += 2;
            }

            // Title text
            for (i, ch) in title.chars().enumerate() {
                if content_x + i as u16 >= x + width - 2 {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::WHITE);
                cell.bg = Some(bg);
                cell.modifier |= Modifier::BOLD;
                ctx.buffer.set(content_x + i as u16, current_y, cell);
            }

            let mut right = Cell::new('│');
            right.fg = Some(border_color);
            ctx.buffer.set(x + width - 1, current_y, right);

            current_y += 1;
        }

        // Message line
        {
            let mut left = Cell::new('│');
            left.fg = Some(border_color);
            ctx.buffer.set(x, current_y, left);

            // Fill background
            for dx in 1..width - 1 {
                let mut cell = Cell::new(' ');
                cell.bg = Some(bg);
                ctx.buffer.set(x + dx, current_y, cell);
            }

            // Icon (if no title)
            let mut content_x = x + 1;
            if self.show_icons && notification.title.is_none() {
                let mut icon = Cell::new(notification.level.icon());
                icon.fg = Some(color);
                icon.bg = Some(bg);
                ctx.buffer.set(content_x, current_y, icon);
                content_x += 2;
            }

            // Message text
            for (i, ch) in notification.message.chars().enumerate() {
                if content_x + i as u16 >= x + width - 2 {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::WHITE);
                cell.bg = Some(bg);
                ctx.buffer.set(content_x + i as u16, current_y, cell);
            }

            let mut right = Cell::new('│');
            right.fg = Some(border_color);
            ctx.buffer.set(x + width - 1, current_y, right);

            current_y += 1;
        }

        // Progress line (if present)
        if let Some(progress) = notification.progress {
            let mut left = Cell::new('│');
            left.fg = Some(border_color);
            ctx.buffer.set(x, current_y, left);

            // Fill background
            for dx in 1..width - 1 {
                let mut cell = Cell::new(' ');
                cell.bg = Some(bg);
                ctx.buffer.set(x + dx, current_y, cell);
            }

            // Progress bar
            let bar_width = width - 4;
            let filled = (progress * bar_width as f64).round() as u16;
            for dx in 0..bar_width {
                let ch = if dx < filled { '█' } else { '░' };
                let fg = if dx < filled {
                    color
                } else {
                    Color::rgb(60, 60, 60)
                };
                let mut cell = Cell::new(ch);
                cell.fg = Some(fg);
                cell.bg = Some(bg);
                ctx.buffer.set(x + 2 + dx, current_y, cell);
            }

            let mut right = Cell::new('│');
            right.fg = Some(border_color);
            ctx.buffer.set(x + width - 1, current_y, right);

            current_y += 1;
        }

        // Bottom border with timer
        let mut bl = Cell::new('╰');
        bl.fg = Some(border_color);
        ctx.buffer.set(x, current_y, bl);

        // Timer indicator
        if self.show_timer && notification.duration > 0 {
            let remaining = notification.remaining();
            let timer_width = (width - 4) as f64;
            let timer_filled = (remaining * timer_width).round() as u16;

            for dx in 1..width - 1 {
                let ch = if dx <= timer_filled { '━' } else { '─' };
                let fg = if dx <= timer_filled {
                    color
                } else {
                    border_color
                };
                let mut cell = Cell::new(ch);
                cell.fg = Some(fg);
                ctx.buffer.set(x + dx, current_y, cell);
            }
        } else {
            for dx in 1..width - 1 {
                let mut h = Cell::new('─');
                h.fg = Some(border_color);
                ctx.buffer.set(x + dx, current_y, h);
            }
        }

        let mut br = Cell::new('╯');
        br.fg = Some(border_color);
        ctx.buffer.set(x + width - 1, current_y, br);
    }
}

impl_styled_view!(NotificationCenter);
impl_props_builders!(NotificationCenter);

/// Helper to create a notification center
pub fn notification_center() -> NotificationCenter {
    NotificationCenter::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::NotificationLevel;

    #[test]
    fn test_notification_new() {
        let n = Notification::new("Test message");
        assert_eq!(n.message, "Test message");
        assert!(matches!(n.level, NotificationLevel::Info));
    }

    #[test]
    fn test_notification_levels() {
        let info = Notification::info("Info");
        let success = Notification::success("Success");
        let warning = Notification::warning("Warning");
        let error = Notification::error("Error");

        assert!(matches!(info.level, NotificationLevel::Info));
        assert!(matches!(success.level, NotificationLevel::Success));
        assert!(matches!(warning.level, NotificationLevel::Warning));
        assert!(matches!(error.level, NotificationLevel::Error));
    }

    #[test]
    fn test_notification_builder() {
        let n = Notification::new("Test")
            .title("Title")
            .level(NotificationLevel::Warning)
            .duration(50)
            .dismissible(false)
            .progress(0.5)
            .action("Retry");

        assert_eq!(n.title, Some("Title".to_string()));
        assert!(matches!(n.level, NotificationLevel::Warning));
        assert_eq!(n.duration, 50);
        assert!(!n.dismissible);
        assert_eq!(n.progress, Some(0.5));
        assert_eq!(n.action, Some("Retry".to_string()));
    }

    #[test]
    fn test_notification_expired() {
        let mut n = Notification::new("Test").duration(10);
        assert!(!n.is_expired());

        n.tick = 10;
        assert!(n.is_expired());
    }

    #[test]
    fn test_notification_remaining() {
        let mut n = Notification::new("Test").duration(100);
        assert_eq!(n.remaining(), 1.0);

        n.tick = 50;
        assert_eq!(n.remaining(), 0.5);
    }

    #[test]
    fn test_center_new() {
        let c = NotificationCenter::new();
        assert!(c.is_empty());
        assert_eq!(c.count(), 0);
    }

    #[test]
    fn test_center_push() {
        let mut c = NotificationCenter::new();
        c.push(Notification::info("Test"));
        assert_eq!(c.count(), 1);
    }

    #[test]
    fn test_center_shortcuts() {
        let mut c = NotificationCenter::new();
        c.info("Info");
        c.success("Success");
        c.warning("Warning");
        c.error("Error");
        assert_eq!(c.count(), 4);
    }

    #[test]
    fn test_center_dismiss() {
        let mut c = NotificationCenter::new();
        c.info("Test");
        let id = c.notifications[0].id;
        c.dismiss(id);
        assert!(c.is_empty());
    }

    #[test]
    fn test_center_clear() {
        let mut c = NotificationCenter::new();
        c.info("1");
        c.info("2");
        c.info("3");
        c.clear();
        assert!(c.is_empty());
    }

    #[test]
    fn test_center_tick() {
        let mut c = NotificationCenter::new();
        c.push(Notification::info("Test").duration(2));

        c.tick();
        assert_eq!(c.count(), 1);

        c.tick();
        assert!(c.is_empty());
    }

    #[test]
    fn test_center_selection() {
        let mut c = NotificationCenter::new();
        c.info("1");
        c.info("2");
        c.info("3");

        c.select_next();
        assert_eq!(c.selected, Some(0));

        c.select_next();
        assert_eq!(c.selected, Some(1));

        c.select_prev();
        assert_eq!(c.selected, Some(0));
    }

    #[test]
    fn test_center_render() {
        let mut buffer = Buffer::new(50, 20);
        let area = Rect::new(0, 0, 50, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut c = NotificationCenter::new();
        c.info("Test notification");
        c.render(&mut ctx);
        // Smoke test
    }

    #[test]
    fn test_level_icon() {
        assert_eq!(NotificationLevel::Info.icon(), 'ℹ');
        assert_eq!(NotificationLevel::Success.icon(), '✓');
        assert_eq!(NotificationLevel::Warning.icon(), '⚠');
        assert_eq!(NotificationLevel::Error.icon(), '✗');
    }

    #[test]
    fn test_center_positions() {
        let positions = [
            NotificationPosition::TopRight,
            NotificationPosition::TopLeft,
            NotificationPosition::TopCenter,
            NotificationPosition::BottomRight,
            NotificationPosition::BottomLeft,
            NotificationPosition::BottomCenter,
        ];

        for pos in positions {
            let mut buffer = Buffer::new(80, 24);
            let area = Rect::new(0, 0, 80, 24);
            let mut ctx = RenderContext::new(&mut buffer, area);

            let mut c = NotificationCenter::new().position(pos);
            c.info("Test");
            c.render(&mut ctx);
        }
    }

    #[test]
    fn test_helper() {
        let c = notification_center().max_visible(3);
        assert_eq!(c.max_visible, 3);
    }

    // =========================================================================
    // NotificationCenter builder method tests
    // =========================================================================

    #[test]
    fn test_notification_center_position_builder() {
        let center = NotificationCenter::new().position(NotificationPosition::TopLeft);
        assert_eq!(center.position, NotificationPosition::TopLeft);
    }

    #[test]
    fn test_notification_center_max_visible_builder() {
        let center = NotificationCenter::new().max_visible(10);
        assert_eq!(center.max_visible, 10);
    }

    #[test]
    fn test_notification_center_max_visible_minimum() {
        let center = NotificationCenter::new().max_visible(0);
        assert_eq!(center.max_visible, 1); // Minimum is 1
    }

    #[test]
    fn test_notification_center_width_builder() {
        let center = NotificationCenter::new().width(60);
        assert_eq!(center.width, 60);
    }

    #[test]
    fn test_notification_center_width_minimum() {
        let center = NotificationCenter::new().width(10);
        assert_eq!(center.width, 20); // Minimum is 20
    }

    #[test]
    fn test_notification_center_show_icons_builder() {
        let center = NotificationCenter::new().show_icons(false);
        assert!(!center.show_icons);
    }

    #[test]
    fn test_notification_center_show_timer_builder() {
        let center = NotificationCenter::new().show_timer(false);
        assert!(!center.show_timer);
    }

    #[test]
    fn test_notification_center_spacing_builder() {
        let center = NotificationCenter::new().spacing(3);
        assert_eq!(center.spacing, 3);
    }

    #[test]
    fn test_notification_center_focused_builder() {
        let center = NotificationCenter::new().focused(true);
        assert!(center.focused);
    }

    // =========================================================================
    // NotificationCenter builder chain tests
    // =========================================================================

    #[test]
    fn test_notification_center_builder_chain() {
        let center = NotificationCenter::new()
            .position(NotificationPosition::BottomLeft)
            .max_visible(7)
            .width(50)
            .show_icons(true)
            .show_timer(false)
            .spacing(2)
            .focused(true);

        assert_eq!(center.position, NotificationPosition::BottomLeft);
        assert_eq!(center.max_visible, 7);
        assert_eq!(center.width, 50);
        assert!(center.show_icons);
        assert!(!center.show_timer);
        assert_eq!(center.spacing, 2);
        assert!(center.focused);
    }

    // =========================================================================
    // NotificationCenter edge case tests
    // =========================================================================

    #[test]
    fn test_center_dismiss_selected_empty() {
        let mut c = NotificationCenter::new();
        c.dismiss_selected(); // Should not panic
        assert_eq!(c.count(), 0);
    }

    #[test]
    fn test_center_dismiss_selected_no_selection() {
        let mut c = NotificationCenter::new();
        c.info("Test");
        c.dismiss_selected(); // Should not panic (no selection)
        assert_eq!(c.count(), 1);
    }

    #[test]
    fn test_center_dismiss_selected_out_of_bounds() {
        let mut c = NotificationCenter::new();
        c.info("Test");
        c.selected = Some(5); // Out of bounds
        c.dismiss_selected(); // Should not panic
        assert_eq!(c.count(), 1);
    }

    #[test]
    fn test_center_tick_empty() {
        let mut c = NotificationCenter::new();
        c.tick(); // Should not panic
        assert!(c.is_empty());
    }

    #[test]
    fn test_center_tick_updates_tick_counter() {
        let mut c = NotificationCenter::new();
        assert_eq!(c.tick_counter, 0);
        c.tick();
        assert_eq!(c.tick_counter, 1);
        c.tick();
        assert_eq!(c.tick_counter, 2);
    }

    #[test]
    fn test_center_tick_updates_notification_ticks() {
        let mut c = NotificationCenter::new();
        c.push(Notification::new("Test").duration(10));
        c.tick();
        assert_eq!(c.notifications[0].tick, 1);
        c.tick();
        assert_eq!(c.notifications[0].tick, 2);
    }

    #[test]
    fn test_center_select_next_empty() {
        let mut c = NotificationCenter::new();
        c.select_next(); // Should not panic
        assert_eq!(c.selected, None);
    }

    #[test]
    fn test_center_select_prev_empty() {
        let mut c = NotificationCenter::new();
        c.select_prev(); // Should not panic
        assert_eq!(c.selected, None);
    }

    #[test]
    fn test_center_select_next_wraps() {
        let mut c = NotificationCenter::new();
        c.info("1");
        c.info("2");
        c.selected = Some(1);
        c.select_next();
        assert_eq!(c.selected, Some(0)); // Wraps to start
    }

    #[test]
    fn test_center_select_prev_wraps() {
        let mut c = NotificationCenter::new();
        c.info("1");
        c.info("2");
        c.selected = Some(0);
        c.select_prev();
        assert_eq!(c.selected, Some(1)); // Wraps to end
    }

    #[test]
    fn test_center_handle_key_not_focused() {
        use crate::event::Key;
        let mut c = NotificationCenter::new();
        c.info("Test");
        let result = c.handle_key(&Key::Down);
        assert!(!result); // Should not handle when not focused
    }

    #[test]
    fn test_center_handle_key_empty() {
        use crate::event::Key;
        let mut c = NotificationCenter::new().focused(true);
        let result = c.handle_key(&Key::Down);
        assert!(!result); // Should not handle when empty
    }

    #[test]
    fn test_center_handle_key_up() {
        use crate::event::Key;
        let mut c = NotificationCenter::new().focused(true);
        c.info("1");
        c.info("2");
        let result = c.handle_key(&Key::Up);
        assert!(result);
        assert_eq!(c.selected, Some(1));
    }

    #[test]
    fn test_center_handle_key_down() {
        use crate::event::Key;
        let mut c = NotificationCenter::new().focused(true);
        c.info("1");
        c.info("2");
        let result = c.handle_key(&Key::Down);
        assert!(result);
        assert_eq!(c.selected, Some(0));
    }

    #[test]
    fn test_center_handle_key_vim_keys() {
        use crate::event::Key;
        let mut c = NotificationCenter::new().focused(true);
        c.info("1");
        c.info("2");
        c.handle_key(&Key::Char('k'));
        assert_eq!(c.selected, Some(1));
        c.handle_key(&Key::Char('j'));
        assert_eq!(c.selected, Some(0));
    }

    #[test]
    fn test_center_handle_key_dismiss() {
        use crate::event::Key;
        let mut c = NotificationCenter::new().focused(true);
        c.info("Test");
        c.select_next();
        let result = c.handle_key(&Key::Char('d'));
        assert!(result);
        assert!(c.is_empty());
    }

    #[test]
    fn test_center_handle_key_delete() {
        use crate::event::Key;
        let mut c = NotificationCenter::new().focused(true);
        c.info("Test");
        c.select_next();
        let result = c.handle_key(&Key::Delete);
        assert!(result);
        assert!(c.is_empty());
    }

    #[test]
    fn test_center_handle_key_clear() {
        use crate::event::Key;
        let mut c = NotificationCenter::new().focused(true);
        c.info("1");
        c.info("2");
        c.info("3");
        let result = c.handle_key(&Key::Char('c'));
        assert!(result);
        assert!(c.is_empty());
        assert_eq!(c.selected, None);
    }

    #[test]
    fn test_center_handle_key_unknown() {
        use crate::event::Key;
        let mut c = NotificationCenter::new().focused(true);
        c.info("Test");
        let result = c.handle_key(&Key::Char('x'));
        assert!(!result);
    }

    // =========================================================================
    // NotificationCenter Default trait tests
    // =========================================================================

    #[test]
    fn test_notification_center_default() {
        let center = NotificationCenter::default();
        assert!(center.is_empty());
        assert_eq!(center.count(), 0);
        assert_eq!(center.max_visible, 5);
        assert_eq!(center.position, NotificationPosition::TopRight);
        assert_eq!(center.width, 40);
        assert!(center.show_icons);
        assert!(center.show_timer);
        assert_eq!(center.spacing, 1);
        assert_eq!(center.tick_counter, 0);
        assert_eq!(center.selected, None);
        assert!(!center.focused);
    }

    #[test]
    fn test_notification_center_default_vs_new() {
        let default_center = NotificationCenter::default();
        let new_center = NotificationCenter::new();

        assert_eq!(default_center.max_visible, new_center.max_visible);
        assert_eq!(default_center.position, new_center.position);
        assert_eq!(default_center.width, new_center.width);
    }

    // =========================================================================
    // NotificationCenter render tests
    // =========================================================================

    #[test]
    fn test_center_render_with_title() {
        let mut buffer = Buffer::new(50, 20);
        let area = Rect::new(0, 0, 50, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut c = NotificationCenter::new();
        c.push(Notification::new("Test message").title("Title"));
        c.render(&mut ctx);
        // Smoke test - should render without panic
    }

    #[test]
    fn test_center_render_with_progress() {
        let mut buffer = Buffer::new(50, 20);
        let area = Rect::new(0, 0, 50, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut c = NotificationCenter::new();
        c.push(Notification::new("Test message").progress(0.5));
        c.render(&mut ctx);
    }

    #[test]
    fn test_center_render_with_action() {
        let mut buffer = Buffer::new(50, 20);
        let area = Rect::new(0, 0, 50, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut c = NotificationCenter::new();
        c.push(Notification::new("Test message").action("Retry"));
        c.render(&mut ctx);
    }

    #[test]
    fn test_center_render_without_icons() {
        let mut buffer = Buffer::new(50, 20);
        let area = Rect::new(0, 0, 50, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut c = NotificationCenter::new().show_icons(false);
        c.info("Test");
        c.render(&mut ctx);
    }

    #[test]
    fn test_center_render_without_timer() {
        let mut buffer = Buffer::new(50, 20);
        let area = Rect::new(0, 0, 50, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut c = NotificationCenter::new().show_timer(false);
        c.info("Test");
        c.render(&mut ctx);
    }

    #[test]
    fn test_center_render_max_visible() {
        let mut buffer = Buffer::new(50, 20);
        let area = Rect::new(0, 0, 50, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut c = NotificationCenter::new().max_visible(2);
        c.info("1");
        c.info("2");
        c.info("3");
        c.render(&mut ctx);
        // Only 2 should be visible
    }

    #[test]
    fn test_center_render_selected() {
        let mut buffer = Buffer::new(50, 20);
        let area = Rect::new(0, 0, 50, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut c = NotificationCenter::new();
        c.info("1");
        c.info("2");
        c.selected = Some(0);
        c.render(&mut ctx);
        // First notification should render as selected
    }

    // =========================================================================
    // NotificationCenter dismissal tests
    // =========================================================================

    #[test]
    fn test_center_dismiss_adjusts_selection() {
        let mut c = NotificationCenter::new();
        c.info("1");
        c.info("2");
        c.info("3");
        c.selected = Some(2);

        // Dismiss the selected one
        let id = c.notifications[2].id;
        c.dismiss(id);

        // Selection should be adjusted
        assert_eq!(c.selected, Some(1));
    }

    #[test]
    fn test_center_dismiss_clears_selection_if_empty() {
        let mut c = NotificationCenter::new();
        c.info("Test");
        c.selected = Some(0);

        let id = c.notifications[0].id;
        c.dismiss(id);

        assert!(c.is_empty());
        assert_eq!(c.selected, None);
    }

    #[test]
    fn test_center_tick_adjusts_selection() {
        let mut c = NotificationCenter::new();
        c.push(Notification::new("Test").duration(1));
        c.selected = Some(0);

        c.tick(); // Notification expires

        assert!(c.is_empty());
        assert_eq!(c.selected, None);
    }

    #[test]
    fn test_center_push_sets_created_at() {
        let mut c = NotificationCenter::new();
        assert_eq!(c.tick_counter, 0);

        c.info("Test 1");
        assert_eq!(c.notifications[0].created_at, 0);

        c.tick();
        assert_eq!(c.tick_counter, 1);

        c.info("Test 2");
        assert_eq!(c.notifications[1].created_at, 1);
    }
}
