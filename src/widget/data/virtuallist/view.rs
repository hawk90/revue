use super::core::VirtualList;
use crate::render::Cell;
use crate::widget::traits::{RenderContext, View};

impl<T: ToString + Clone> View for VirtualList<T> {
    crate::impl_view_meta!("VirtualList");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 2 || area.height < 1 {
            return;
        }

        let viewport_height = area.height;
        let content_width = if self.show_scrollbar {
            area.width.saturating_sub(1)
        } else {
            area.width
        };

        // Ensure selected item is visible
        let mut this = self.clone();
        this.ensure_visible(viewport_height);

        // Get visible range
        let visible_range = this.visible_range(viewport_height);

        // Render visible items
        for item_idx in visible_range {
            let item = &this.items[item_idx];
            let is_selected = this.selected == Some(item_idx);

            // Calculate Y position (accounting for scroll offset)
            let relative_idx = item_idx.saturating_sub(this.scroll_offset);
            let y_offset = (relative_idx as u16) * this.item_height;

            if y_offset >= viewport_height {
                break;
            }

            // Get item text
            let text = this.render_item(item, item_idx, is_selected);

            // Render item rows
            for row in 0..this.item_height {
                let y = area.y + y_offset + row;
                if y >= area.y + viewport_height {
                    break;
                }

                // Get the line for this row (for multi-row items)
                let line = if row == 0 { &text } else { "" };

                // Render each character
                // Iterate directly for O(n) instead of O(n²) with .chars().nth(x) in loop
                for (x, ch) in line
                    .chars()
                    .chain(std::iter::repeat(' '))
                    .take(content_width as usize)
                    .enumerate()
                {
                    let mut cell = Cell::new(ch);

                    if is_selected {
                        cell.fg = Some(this.selected_fg);
                        cell.bg = Some(this.selected_bg);
                    } else {
                        cell.fg = Some(this.item_fg);
                    }

                    ctx.buffer.set(area.x + x as u16, y, cell);
                }
            }
        }

        // Render scrollbar
        if this.show_scrollbar && this.items.len() > (viewport_height / this.item_height) as usize {
            // Use a mutable reference for scrollbar rendering
            let this_clone = this.clone();
            this_clone.render_scrollbar(ctx, viewport_height);
        }
    }
}
