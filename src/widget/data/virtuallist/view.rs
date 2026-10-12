use super::core::VirtualList;
use crate::render::Cell;
use crate::style::Color;
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
            area.width - 1
        } else {
            area.width
        };

        // Worked out per frame from `self`; the stored offset is not moved
        let offset = self.offset_showing_selection(viewport_height);
        let visible = self.visible_count(viewport_height);
        let end = (offset + visible).min(self.items.len());
        for (slot, index) in (offset..end).enumerate() {
            let top = (slot as u16).saturating_mul(self.item_height);
            self.draw_item(ctx, index, top, content_width);
        }

        if self.show_scrollbar && self.items.len() > visible {
            self.draw_scrollbar(ctx, viewport_height, offset);
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

    /// Draw item `index` from row `top`: the text on its first row, and the
    /// row colors on all of them
    fn draw_item(&self, ctx: &mut RenderContext, index: usize, top: u16, width: u16) {
        let is_selected = self.selected == Some(index);
        let (fg, bg) = if is_selected {
            (self.selected_fg, Some(self.selected_bg))
        } else {
            // An ordinary item takes `color`; the selected one keeps its
            // highlight.
            (
                self.item_fg.unwrap_or_else(|| ctx.css_color(Color::WHITE)),
                None,
            )
        };
        let text = self.render_item(&self.items[index], index, is_selected);

        for row in 0..self.item_height {
            let y = top.saturating_add(row);
            if y >= ctx.area.height {
                break;
            }
            ctx.fill_row(y, width, Some(fg), bg);
        }
        match bg {
            Some(bg) => ctx.draw_text_clipped_bg(0, top, &text, fg, bg, width),
            None => ctx.draw_text_clipped(0, top, &text, fg, width),
        }
    }

    /// Render scrollbar
    pub fn render_scrollbar(&self, ctx: &mut RenderContext, viewport_height: u16) {
        self.draw_scrollbar(ctx, viewport_height, self.scroll_offset);
    }

    /// Draw the scrollbar in the last column for a view starting at `offset`
    fn draw_scrollbar(&self, ctx: &mut RenderContext, viewport_height: u16, offset: usize) {
        let area = ctx.area;
        let Some(scrollbar_x) = area.width.checked_sub(1) else {
            return;
        };

        // Calculate thumb position and size
        let total = self.items.len() as f32;
        let visible = self.visible_count(viewport_height) as f32;

        if total <= visible {
            // No scrollbar needed
            return;
        }

        let thumb_size = ((visible / total) * viewport_height as f32).max(1.0) as u16;
        let scroll_range = viewport_height.saturating_sub(thumb_size);
        // A jump can leave the offset past the last full page; the thumb
        // still stops at the bottom.
        let progress = (offset as f32 / (total - visible)).min(1.0);
        let thumb_pos = (progress * scroll_range as f32) as u16;

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
