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

#[cfg(test)]
mod tests {
    use super::super::types::{GridAlign, GridItem, TrackSize};
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::Text;

    // =========================================================================
    // Render edge case tests
    // =========================================================================

    #[test]
    fn test_grid_render_empty() {
        let grid = Grid::new();
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should not crash with empty grid
        grid.render(&mut ctx);
    }

    #[test]
    fn test_grid_render_zero_width() {
        let grid = Grid::new().child(Text::new("Test"));
        let mut buffer = Buffer::new(0, 10);
        let area = Rect::new(0, 0, 0, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should not crash with zero width
        grid.render(&mut ctx);
    }

    #[test]
    fn test_grid_render_zero_height() {
        let grid = Grid::new().child(Text::new("Test"));
        let mut buffer = Buffer::new(10, 0);
        let area = Rect::new(0, 0, 10, 0);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should not crash with zero height
        grid.render(&mut ctx);
    }

    #[test]
    fn test_grid_render_single_item() {
        let grid = Grid::new().child(Text::new("A"));
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        grid.render(&mut ctx);

        // Item should be rendered
        let cell = buffer.get(0, 0);
        assert!(cell.is_some());
    }

    #[test]
    fn test_grid_render_multiple_items() {
        let grid = Grid::new()
            .cols(2)
            .child(Text::new("A"))
            .child(Text::new("B"))
            .child(Text::new("C"))
            .child(Text::new("D"));
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        grid.render(&mut ctx);

        // Items should be rendered
        let cell = buffer.get(0, 0);
        assert!(cell.is_some());
    }

    #[test]
    fn test_grid_render_with_gaps() {
        let grid = Grid::new()
            .cols(2)
            .gap(2)
            .child(Text::new("A"))
            .child(Text::new("B"));
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should not crash with gaps
        grid.render(&mut ctx);
    }

    #[test]
    fn test_grid_render_explicit_placement() {
        let grid = Grid::new()
            .item(GridItem::new(Text::new("A")).at(1, 1))
            .item(GridItem::new(Text::new("B")).at(2, 2));
        let mut buffer = Buffer::new(20, 20);
        let area = Rect::new(0, 0, 20, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should not crash with explicit placement
        grid.render(&mut ctx);
    }

    #[test]
    fn test_grid_render_with_span() {
        let grid = Grid::new()
            .cols(3)
            .item(GridItem::new(Text::new("A")).col_span(2))
            .child(Text::new("B"));
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should not crash with column span
        grid.render(&mut ctx);
    }

    #[test]
    fn test_grid_render_auto_dimensions() {
        let grid = Grid::new().child(Text::new("A")).child(Text::new("B"));
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should auto-detect dimensions
        grid.render(&mut ctx);
    }

    #[test]
    fn test_grid_render_fixed_tracks() {
        let grid = Grid::new()
            .columns(vec![TrackSize::Fixed(5), TrackSize::Fixed(10)])
            .rows(vec![TrackSize::Fixed(3), TrackSize::Fixed(7)])
            .child(Text::new("A"))
            .child(Text::new("B"));
        let mut buffer = Buffer::new(20, 20);
        let area = Rect::new(0, 0, 20, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should not crash with fixed tracks
        grid.render(&mut ctx);
    }

    #[test]
    fn test_grid_render_fr_tracks() {
        let grid = Grid::new()
            .columns(vec![TrackSize::Fr(1.0), TrackSize::Fr(2.0)])
            .child(Text::new("A"))
            .child(Text::new("B"));
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should not crash with fr tracks
        grid.render(&mut ctx);
    }

    #[test]
    fn test_grid_render_mixed_tracks() {
        let grid = Grid::new()
            .columns(vec![TrackSize::Fixed(10), TrackSize::Fr(1.0)])
            .child(Text::new("A"))
            .child(Text::new("B"));
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should not crash with mixed tracks
        grid.render(&mut ctx);
    }

    #[test]
    fn test_grid_render_out_of_bounds_item() {
        let grid = Grid::new().item(GridItem::new(Text::new("A")).at(100, 100)); // Way out of bounds
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should handle out-of bounds gracefully
        grid.render(&mut ctx);
    }

    #[test]
    fn test_grid_render_zero_size_cell() {
        let grid = Grid::new()
            .columns(vec![TrackSize::Fixed(0), TrackSize::Fixed(10)])
            .child(Text::new("A"))
            .child(Text::new("B"));
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should handle zero-size cells gracefully
        grid.render(&mut ctx);
    }

    #[test]
    fn test_grid_render_alignment() {
        let grid = Grid::new()
            .justify_items(GridAlign::Center)
            .align_items(GridAlign::Start)
            .child(Text::new("A"));
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should not crash with alignment settings
        grid.render(&mut ctx);
    }
}
