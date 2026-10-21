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

        // The rows in view: the selection kept in view, or in the middle in
        // Center mode (#971)
        let total = self.total_height() as usize;
        self.adopt_offsets();
        self.view().set_viewport(usize::from(viewport_height));
        self.track_selection();
        let shown = self.view().visible();

        if !self.items.is_empty() && !shown.is_empty() {
            let mut index = self.index_at_row(shown.start as u32);
            while index < self.items.len() && (self.row_of_index(index) as usize) < shown.end {
                let rows = self.rows_of(index);
                self.draw_item(ctx, index, rows, shown.start, content_width);
                index += 1;
            }
        }

        if self.show_scrollbar && total > usize::from(viewport_height) {
            self.draw_scrollbar(ctx, viewport_height, self.view().offset());
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

    /// Draw item `index`, which takes `rows` of the list, in a view that
    /// starts at row `first`: the row colors on each of its rows in view,
    /// and the text on its first row when that is in view
    fn draw_item(
        &self,
        ctx: &mut RenderContext,
        index: usize,
        rows: std::ops::Range<usize>,
        first: usize,
        width: u16,
    ) {
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
        for row in rows.clone() {
            let Some(y) = row.checked_sub(first) else {
                continue;
            };
            if y >= usize::from(ctx.area.height) {
                break;
            }
            ctx.fill_row(y as u16, width, Some(fg), bg);
        }
        let Some(top) = rows.start.checked_sub(first) else {
            return;
        };
        let top = top as u16;
        let text = self.render_item(&self.items[index], index, is_selected);
        match bg {
            Some(bg) => ctx.draw_text_clipped_bg(0, top, &text, fg, bg, width),
            None => ctx.draw_text_clipped(0, top, &text, fg, width),
        }
    }

    /// Render scrollbar
    pub fn render_scrollbar(&self, ctx: &mut RenderContext, viewport_height: u16) {
        self.draw_scrollbar(ctx, viewport_height, self.view().offset());
    }

    /// Draw the scrollbar in the last column for a view starting at row
    /// `offset`
    fn draw_scrollbar(&self, ctx: &mut RenderContext, viewport_height: u16, offset: usize) {
        let area = ctx.area;
        let Some(scrollbar_x) = area.width.checked_sub(1) else {
            return;
        };

        // Thumb position and size, in rows
        let total = self.total_height() as f32;
        let visible = f32::from(viewport_height);
        if total <= visible {
            // No scrollbar needed
            return;
        }

        let thumb_size = ((visible / total) * visible).max(1.0) as u16;
        let scroll_range = viewport_height.saturating_sub(thumb_size);
        let progress = (offset as f32 / (total - visible)).min(1.0);
        let thumb_pos = (progress * f32::from(scroll_range)) as u16;

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
