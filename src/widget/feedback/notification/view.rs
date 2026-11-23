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

    /// The width of every box in `area`: the set width, but no wider than
    /// the area, so a narrow area does not cut off the right border, the end
    /// of the text and the right-aligned action (#1048)
    pub(super) fn box_width(&self, area: Rect) -> u16 {
        self.width.min(area.width)
    }

    /// The left column of every box, the row the stack starts from, and the
    /// direction it grows in (1 down from the top, -1 up from the bottom),
    /// relative to `area`
    fn stack_origin(&self, area: Rect) -> (u16, u16, i16) {
        let free = area.width.saturating_sub(self.box_width(area));
        match self.position {
            NotificationPosition::TopRight => (free, 0, 1),
            NotificationPosition::TopLeft => (0, 0, 1),
            NotificationPosition::TopCenter => (free / 2, 0, 1),
            NotificationPosition::BottomRight => (free, area.height, -1),
            NotificationPosition::BottomLeft => (0, area.height, -1),
            NotificationPosition::BottomCenter => (free / 2, area.height, -1),
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
        // A box needs its two borders and two inner columns
        let width = self.box_width(area);
        if width < 4 {
            self.drawn.set(Vec::new());
            return;
        }
        let visible = self
            .notifications
            .iter()
            .rev()
            .take(self.max_visible)
            .collect::<Vec<_>>();
        let mut drawn = Vec::with_capacity(visible.len());

        // Calculate starting position based on notification position (relative coordinates)
        let (start_x, mut current_y, direction) = self.stack_origin(area);

        // Render each notification
        for notification in visible.iter() {
            let height = self.notification_height(notification);
            // By id: the keys move over the drawn ones, not over every
            // shown one, so an index in `visible` would not match (#1516)
            let is_selected = self.selected.key() == Some(&notification.id);

            // A box that does not fit is left out and the next one tries the
            // same place: growing down, one past the bottom of the area;
            // growing up, one with no room above the last box (it was pinned
            // at row 0 and drawn over the others, #1048)
            let y = if direction < 0 {
                match current_y.checked_sub(height) {
                    Some(y) => y,
                    None => continue,
                }
            } else {
                current_y
            };
            if u32::from(y) + u32::from(height) > u32::from(area.height) {
                continue;
            }

            self.render_notification(ctx, notification, start_x, y, width, is_selected);
            drawn.push(notification.id);

            if direction < 0 {
                // At row 0 no box fits above: every box is at least 3 rows
                current_y = y.saturating_sub(self.spacing);
            } else {
                current_y = y.saturating_add(height).saturating_add(self.spacing);
            }
        }
        self.drawn.set(drawn);
    }
}
