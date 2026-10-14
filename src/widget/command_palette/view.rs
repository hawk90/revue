//! Drawing: the frame, the title and search lines, and one row per command.

mod frame;
mod header;
mod row;

use crate::render::Cell;
use crate::widget::traits::RenderContext;
use crate::widget::View;

use super::CommandPalette;

/// Where the palette sits in the area, computed once per render.
struct Layout {
    /// Left column of the frame
    x: u16,
    /// Top row of the frame
    y: u16,
    /// Frame width
    width: u16,
    /// Frame height
    height: u16,
    /// Right border column
    right_x: u16,
    /// End of the text columns
    text_end: u16,
    /// End of the label column
    label_end: u16,
}

impl View for CommandPalette {
    crate::impl_view_meta!("CommandPalette");

    fn render(&self, ctx: &mut RenderContext) {
        if !self.visible {
            return;
        }

        let Some(layout) = self.layout(ctx) else {
            return;
        };

        self.draw_background(ctx, &layout);

        let mut current_y = layout.y;

        // Top border
        self.draw_rule(ctx, &layout, current_y, '╭', '╮');
        current_y += 1;

        // Title (if present)
        if let Some(ref title) = self.title {
            self.draw_title(ctx, &layout, current_y, title);
            current_y += 1;
        }

        // Search input line
        self.draw_search(ctx, &layout, current_y);
        current_y += 1;

        // Separator
        self.draw_rule(ctx, &layout, current_y, '├', '┤');
        current_y += 1;

        // Command items
        let scroll_offset = self.selection.offset();
        let visible_items: Vec<_> = self
            .filtered
            .iter()
            .skip(scroll_offset)
            .take(self.max_visible as usize)
            .collect();

        for (i, &cmd_idx) in visible_items.iter().enumerate() {
            let is_selected = self.selection.is_selected(scroll_offset + i);
            let item_y = current_y + i as u16;
            self.draw_row(ctx, &layout, item_y, &self.commands[*cmd_idx], is_selected);
        }

        // Fill remaining space if fewer items than max_visible
        for i in visible_items.len()..self.max_visible as usize {
            let item_y = current_y + i as u16;
            if item_y >= layout.y + layout.height - 1 {
                break;
            }
            self.draw_sides(ctx, &layout, item_y);
        }

        // Bottom border
        let bottom_y = layout.y + layout.height - 1;
        self.draw_rule(ctx, &layout, bottom_y, '╰', '╯');
        self.draw_count(ctx, &layout, bottom_y);
    }
}

impl CommandPalette {
    /// Centered frame geometry, or `None` when the frame does not fit the
    /// area's height.
    fn layout(&self, ctx: &RenderContext) -> Option<Layout> {
        let area = ctx.area;
        let width = self.width.min(area.width);
        let x = (area.width.saturating_sub(width)) / 2;
        let has_title = self.title.is_some();

        // Calculate height
        let visible_count = self.filtered.len().min(self.max_visible as usize);
        let height = 3 + visible_count as u16 + if has_title { 1 } else { 0 }; // border + input + items
        let y: u16 = 2; // Offset from top

        if y + height > area.height {
            return None;
        }

        // Right border column, end of the text columns and end of the label
        // column, saturating for a palette narrower than its frame.
        Some(Layout {
            x,
            y,
            width,
            height,
            right_x: (x + width).saturating_sub(1),
            text_end: (x + width).saturating_sub(2),
            label_end: (x + width).saturating_sub(15),
        })
    }

    /// A border glyph in the border color.
    fn border_cell(&self, ch: char) -> Cell {
        let mut cell = Cell::new(ch);
        cell.fg = Some(self.border_color);
        cell
    }
}
