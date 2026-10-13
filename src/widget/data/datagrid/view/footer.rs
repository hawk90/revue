//! DataGrid footer rows

use crate::render::{Cell, Modifier};

use crate::utils::{char_width, display_width, truncate_to_width};

use crate::widget::data::datagrid::columns::slots::ColumnSlot;
use crate::widget::data::datagrid::types::{Alignment, FooterRow};
use crate::widget::data::datagrid::DataGrid;
use crate::widget::traits::RenderContext;

impl DataGrid {
    /// Render one footer row: each aggregation in its column, and the row
    /// label in the first column when that column has no aggregation.
    pub(crate) fn render_footer(
        &self,
        ctx: &mut RenderContext,
        footer: &FooterRow,
        slots: &[ColumnSlot<'_>],
        content_end: u16,
        y: u16,
    ) {
        let bg = self.colors.header_bg;
        ctx.fill_bg(0, y, content_end, 1, bg);

        let values = self.get_footer_values(footer);
        for (i, slot) in slots.iter().enumerate() {
            let text = values
                .iter()
                .find(|(key, _)| *key == slot.col.key)
                .map(|(_, text)| text.as_str())
                .or((i == 0).then_some(footer.label.as_str()))
                .unwrap_or("");

            let truncated = truncate_to_width(text, (slot.width as usize).saturating_sub(1));
            let dw = display_width(truncated) as u16;
            let start_x = match slot.col.align {
                Alignment::Left => slot.x,
                Alignment::Center => slot.x + (slot.width.saturating_sub(dw)) / 2,
                Alignment::Right => slot.x + slot.width.saturating_sub(dw + 1),
            };
            let mut dx: u16 = 0;
            for ch in truncated.chars() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(self.colors.header_fg);
                cell.bg = Some(bg);
                cell.modifier |= Modifier::BOLD;
                ctx.set(start_x + dx, y, cell);
                dx += char_width(ch) as u16;
            }

            let mut sep = Cell::new('│');
            sep.fg = Some(self.colors.border_color);
            sep.bg = Some(bg);
            ctx.set(slot.x + slot.width, y, sep);
        }
    }
}
