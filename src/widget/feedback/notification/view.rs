//! Where each notification goes: the stack's corner, direction and heights

use super::{Notification, NotificationCenter, NotificationPosition};
use crate::layout::Rect;
use crate::widget::traits::{RenderContext, View};

impl NotificationCenter {
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

    /// The left column of every box, the row the stack starts from, and the
    /// direction it grows in (1 down from the top, -1 up from the bottom),
    /// relative to `area`
    fn stack_origin(&self, area: Rect) -> (u16, u16, i16) {
        match self.position {
            NotificationPosition::TopRight => (area.width.saturating_sub(self.width), 0, 1),
            NotificationPosition::TopLeft => (0, 0, 1),
            NotificationPosition::TopCenter => ((area.width.saturating_sub(self.width)) / 2, 0, 1),
            NotificationPosition::BottomRight => {
                (area.width.saturating_sub(self.width), area.height, -1)
            }
            NotificationPosition::BottomLeft => (0, area.height, -1),
            NotificationPosition::BottomCenter => {
                ((area.width.saturating_sub(self.width)) / 2, area.height, -1)
            }
        }
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

        // Calculate starting position based on notification position (relative coordinates)
        let (start_x, mut current_y, direction) = self.stack_origin(area);

        // Render each notification
        for (idx, notification) in visible.iter().enumerate() {
            let height = self.notification_height(notification);
            let is_selected = self.selected.is_selected(idx);

            // Adjust Y position for bottom positions
            let y = if direction < 0 {
                current_y.saturating_sub(height)
            } else {
                current_y
            };

            if y >= area.height || y + height > area.height {
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
