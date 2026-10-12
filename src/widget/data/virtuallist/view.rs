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
                let y = y_offset + row;
                if y >= viewport_height {
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
                        // An ordinary item takes `color`; the selected one
                        // keeps its highlight.
                        cell.fg = Some(
                            this.item_fg
                                .unwrap_or_else(|| ctx.css_color(crate::style::Color::WHITE)),
                        );
                    }

                    ctx.set(x as u16, y, cell);
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

impl<T: ToString + Clone> VirtualList<T> {
    /// Render item text
    pub fn render_item(&self, item: &T, index: usize, is_selected: bool) -> String {
        if let Some(ref renderer) = self.renderer {
            renderer(item, index, is_selected)
        } else {
            item.to_string()
        }
    }

    /// Render scrollbar
    pub fn render_scrollbar(&self, ctx: &mut RenderContext, viewport_height: u16) {
        let area = ctx.area;
        let scrollbar_x = area.width - 1;

        // Calculate thumb position and size
        let total = self.items.len() as f32;
        let visible = (viewport_height / self.item_height) as f32;

        if total <= visible {
            // No scrollbar needed
            return;
        }

        let thumb_size = ((visible / total) * viewport_height as f32).max(1.0) as u16;
        let scroll_range = viewport_height.saturating_sub(thumb_size);
        let thumb_pos =
            ((self.scroll_offset as f32 / (total - visible)) * scroll_range as f32) as u16;

        // Draw scrollbar track
        for y in 0..viewport_height {
            if y < area.height {
                let in_thumb = y >= thumb_pos && y < thumb_pos + thumb_size;
                let ch = if in_thumb { '█' } else { '░' };
                let color = if in_thumb {
                    self.scrollbar_fg
                } else {
                    self.scrollbar_bg
                };
                ctx.set(scrollbar_x, y, Cell::new(ch).fg(color));
            }
        }
    }
}
