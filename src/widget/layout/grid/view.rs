//! Grid widget View implementation

use super::core::Grid;
use crate::widget::traits::{RenderContext, View};

impl View for Grid {
    crate::impl_view_meta!("Grid");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 || self.items.is_empty() {
            return;
        }

        // Determine grid dimensions
        let col_count = if self.columns.is_empty() {
            // Auto-detect from placements
            self.items
                .iter()
                .map(|item| item.placement.col_end.max(item.placement.col_start + 1) as usize)
                .max()
                .unwrap_or(1)
        } else {
            self.columns.len()
        };

        let row_count = if self.rows.is_empty() {
            self.items
                .iter()
                .map(|item| item.placement.row_end.max(item.placement.row_start + 1) as usize)
                .max()
                .unwrap_or(1)
        } else {
            self.rows.len()
        };

        // Build track definitions
        let col_tracks: Vec<super::types::TrackSize> = if self.columns.is_empty() {
            vec![self.auto_cols; col_count]
        } else {
            self.columns.clone()
        };

        let row_tracks: Vec<super::types::TrackSize> = if self.rows.is_empty() {
            vec![self.auto_rows; row_count]
        } else {
            self.rows.clone()
        };

        // Calculate track sizes
        let col_sizes =
            self.calculate_tracks(area.width, &col_tracks, self.auto_cols, self.col_gap);
        let row_sizes =
            self.calculate_tracks(area.height, &row_tracks, self.auto_rows, self.row_gap);

        // Get track positions
        let col_positions = self.track_positions(&col_sizes, self.col_gap);
        let row_positions = self.track_positions(&row_sizes, self.row_gap);

        // Auto-place items
        let placements = self.auto_place_items(col_count, row_count);

        // Render each item
        for (idx, placement) in placements {
            if idx >= self.items.len() {
                continue;
            }

            let item = &self.items[idx];

            // Get cell bounds
            let col_start = (placement.col_start as usize).saturating_sub(1);
            let col_end = (placement.col_end as usize).saturating_sub(1);
            let row_start = (placement.row_start as usize).saturating_sub(1);
            let row_end = (placement.row_end as usize).saturating_sub(1);

            if col_start >= col_positions.len() || row_start >= row_positions.len() {
                continue;
            }

            let x = area.x + col_positions.get(col_start).copied().unwrap_or(0);
            let y = area.y + row_positions.get(row_start).copied().unwrap_or(0);

            let end_x = col_positions.get(col_end).copied().unwrap_or(area.width);
            let end_y = row_positions.get(row_end).copied().unwrap_or(area.height);

            let width = end_x.saturating_sub(col_positions.get(col_start).copied().unwrap_or(0));
            let height = end_y.saturating_sub(row_positions.get(row_start).copied().unwrap_or(0));

            if width == 0 || height == 0 {
                continue;
            }

            let cell_rect = crate::layout::Rect::new(x, y, width, height);

            let mut child_ctx = RenderContext::new(ctx.buffer, cell_rect);

            item.widget.render(&mut child_ctx);
        }
    }
}
