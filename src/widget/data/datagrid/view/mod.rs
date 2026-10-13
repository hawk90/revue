//! DataGrid rendering: the frame plan, then header, rows, footer and scrollbar

mod body;
mod footer;
mod header;
mod scrollbar;

use crate::layout::Rect;
use crate::style::Color;
use crate::widget::data::datagrid::columns::slots::ColumnSlot;
use crate::widget::data::datagrid::types::FooterRow;
use crate::widget::data::datagrid::DataGrid;

use crate::widget::traits::{RenderContext, View};

/// Cell position for rendering
pub(crate) struct CellPos {
    pub x: u16,
    pub y: u16,
    pub width: u16,
}

/// Cell state for rendering
pub(crate) struct CellState {
    pub row_bg: Color,
    pub is_selected: bool,
    pub is_editing: bool,
}

/// Row rendering parameters
pub(crate) struct RowRenderParams<'a, 'b> {
    pub slots: &'b [ColumnSlot<'a>],
    pub area_x: u16,
    pub content_end: u16,
    pub start_y: u16,
    pub row_num_width: u16,
    pub visible_height: usize,
}

impl View for DataGrid {
    crate::impl_view_meta!("DataGrid");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 10 || area.height < 3 {
            return;
        }

        let row_num_width = self.row_number_gutter_width();
        let header_height: u16 = if self.options.show_header { 1 } else { 0 };
        let footer_rows = self.footer_rows_that_fit(area.height - header_height);
        let footer_height = footer_rows.len() as u16;

        // Position columns for this viewport, applying column freeze and
        // horizontal scroll. Both the header and the rows draw from this plan so
        // they stay aligned, and mouse hit-testing uses the same plan.
        let content_end = area.width;
        let visible_cols = self.visible_columns_in_order();
        let slots =
            self.layout_column_slots(&visible_cols, Rect::new(0, 0, area.width, area.height));

        let mut y = 0u16;

        // Draw header
        if self.options.show_header {
            self.render_header(ctx, &slots, content_end, row_num_width, y);
            y += 1;
        }

        // Calculate visible range with virtual scrolling
        let total_rows = self.filtered_count();
        let visible_height = (area.height - header_height - footer_height) as usize
            / self.options.row_height.max(1) as usize;
        self.last_viewport_height.set(visible_height);

        let (render_start, render_end) = self.render_range(total_rows, visible_height);

        let params = RowRenderParams {
            slots: &slots,
            area_x: 0,
            content_end,
            start_y: y,
            row_num_width,
            visible_height,
        };

        // Render rows using index-based access (no allocation)
        self.render_rows_virtual(ctx, render_start, render_end, &params);

        // Draw the footer rows at the bottom
        let footer_y = area.height - footer_height;
        for (i, footer) in footer_rows.iter().enumerate() {
            self.render_footer(ctx, footer, &slots, content_end, footer_y + i as u16);
        }

        // Draw scrollbar if needed
        self.render_scrollbar(ctx, total_rows, visible_height, area, y, footer_y);
    }
}

impl DataGrid {
    /// The footer rows to draw: they take rows from the bottom of the
    /// viewport, all of them or none, as long as at least one data row still
    /// fits above them. `height` excludes the header.
    fn footer_rows_that_fit(&self, height: u16) -> &[FooterRow] {
        if self.show_footer && (self.footer_rows.len() as u16) < height {
            &self.footer_rows
        } else {
            &[]
        }
    }

    /// Filtered-row positions to walk for this frame: the viewport plus
    /// `overscan` rows on each side with virtual scrolling, every row without
    fn render_range(&self, total_rows: usize, visible_height: usize) -> (usize, usize) {
        if self.options.virtual_scroll {
            let overscan = self.options.overscan;
            let start = self.scroll_row.saturating_sub(overscan);
            let end = (self.scroll_row + visible_height + overscan).min(total_rows);
            (start, end)
        } else {
            (0, total_rows)
        }
    }
}
