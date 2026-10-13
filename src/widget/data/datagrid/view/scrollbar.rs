//! DataGrid scrollbar and row position

use crate::render::Cell;
use crate::style::Color;

use crate::layout::Rect;
use crate::widget::data::datagrid::DataGrid;
use crate::widget::theme::LIGHT_GRAY;
use crate::widget::traits::RenderContext;

impl DataGrid {
    /// Render scrollbar and row indicator
    pub(crate) fn render_scrollbar(
        &self,
        ctx: &mut RenderContext,
        total_rows: usize,
        visible_height: usize,
        area: Rect,
        content_y: u16,
        footer_y: u16,
    ) {
        if total_rows <= visible_height {
            return;
        }

        let scrollbar_x = area.width - 1;
        let scrollbar_height = visible_height as f64;
        let thumb_height =
            (scrollbar_height * visible_height as f64 / total_rows as f64).max(1.0) as u16;
        let thumb_pos = (self.scroll_row as f64 / (total_rows - visible_height) as f64
            * (scrollbar_height - thumb_height as f64)) as u16;

        for i in 0..visible_height {
            let scrollbar_y = content_y + i as u16;
            let i_u16 = i as u16;
            let is_thumb = i_u16 >= thumb_pos && i_u16 < (thumb_pos + thumb_height);

            let mut cell = if is_thumb {
                Cell::new('█')
            } else {
                Cell::new('░')
            };
            cell.fg = Some(Color::rgb(100, 100, 120));
            ctx.set(scrollbar_x, scrollbar_y, cell);
        }

        // Draw row indicator
        let indicator = format!(" {}/{} ", self.selected_row + 1, total_rows);
        let indicator_x = area.width.saturating_sub(indicator.len() as u16 + 1);
        let indicator_y = footer_y.saturating_sub(1);

        for (j, ch) in indicator.chars().enumerate() {
            let mut cell = Cell::new(ch);
            cell.fg = Some(LIGHT_GRAY);
            cell.bg = Some(Color::rgb(40, 40, 50));
            let cell_x = indicator_x + (j as u16);
            if cell_x < area.width {
                ctx.set(cell_x, indicator_y, cell);
            }
        }
    }
}
