//! Grid widget View implementation

use super::core::Grid;
use super::types::{GridItem, GridPlacement, TrackSize};
use crate::layout::Rect;
use crate::widget::traits::{RenderContext, View};

impl View for Grid {
    crate::impl_view_meta!("Grid");

    fn render(&self, ctx: &mut RenderContext) {
        let area = self.apply_constraints(ctx.area);
        if area.width == 0 || area.height == 0 || self.items.is_empty() {
            return;
        }

        // Determine grid dimensions
        let col_count = track_count(&self.columns, &self.items, |p| {
            p.col_end.max(p.col_start + 1)
        });
        let row_count = track_count(&self.rows, &self.items, |p| p.row_end.max(p.row_start + 1));

        // Build track definitions
        let col_tracks = track_defs(&self.columns, self.auto_cols, col_count);
        let row_tracks = track_defs(&self.rows, self.auto_rows, row_count);

        // CSS wins when it specified a gap; `column-gap`/`row-gap` fall back to
        // `gap`, which falls back to the builder's own value.
        let col_gap = ctx.column_gap_or(self.col_gap);
        let row_gap = ctx.row_gap_or(self.row_gap);

        // Calculate track sizes
        let col_sizes = self.calculate_tracks(area.width, &col_tracks, self.auto_cols, col_gap);
        let row_sizes = self.calculate_tracks(area.height, &row_tracks, self.auto_rows, row_gap);

        // Get track positions
        let col_positions = self.track_positions(&col_sizes, col_gap);
        let row_positions = self.track_positions(&row_sizes, row_gap);

        // Auto-place items
        let placements = self.auto_place_items(col_count, row_count);

        // Render each item
        for (idx, placement) in placements {
            if idx >= self.items.len() {
                continue;
            }

            let item = &self.items[idx];

            let Some((rel_x, rel_y, width, height)) =
                cell_bounds(&placement, &col_positions, &row_positions, area)
            else {
                continue;
            };

            let cell_rect = ctx.sub_area(rel_x, rel_y, width, height);

            ctx.render_child(item.widget.as_ref(), cell_rect);
        }
    }
}

/// The number of tracks on one axis: the defined ones, or, with none
/// defined, auto-detected from the items' `line` (end line or start + 1)
fn track_count(
    defined: &[TrackSize],
    items: &[GridItem],
    line: impl Fn(&GridPlacement) -> u16,
) -> usize {
    if defined.is_empty() {
        // Auto-detect from placements
        items
            .iter()
            .map(|item| line(&item.placement) as usize)
            .max()
            .unwrap_or(1)
    } else {
        defined.len()
    }
}

/// The defined tracks, or `count` auto-sized ones when none are defined
fn track_defs(defined: &[TrackSize], auto: TrackSize, count: usize) -> Vec<TrackSize> {
    if defined.is_empty() {
        vec![auto; count]
    } else {
        defined.to_vec()
    }
}

/// The relative (x, y, width, height) of a placed item's cell, from the
/// track start positions; `None` when it starts past the tracks or is empty
fn cell_bounds(
    placement: &GridPlacement,
    col_positions: &[u16],
    row_positions: &[u16],
    area: Rect,
) -> Option<(u16, u16, u16, u16)> {
    // Get cell bounds
    let col_start = (placement.col_start as usize).saturating_sub(1);
    let col_end = (placement.col_end as usize).saturating_sub(1);
    let row_start = (placement.row_start as usize).saturating_sub(1);
    let row_end = (placement.row_end as usize).saturating_sub(1);

    if col_start >= col_positions.len() || row_start >= row_positions.len() {
        return None;
    }

    let rel_x = col_positions.get(col_start).copied().unwrap_or(0);
    let rel_y = row_positions.get(row_start).copied().unwrap_or(0);

    let end_x = col_positions.get(col_end).copied().unwrap_or(area.width);
    let end_y = row_positions.get(row_end).copied().unwrap_or(area.height);

    let width = end_x.saturating_sub(rel_x);
    let height = end_y.saturating_sub(rel_y);

    if width == 0 || height == 0 {
        return None;
    }
    Some((rel_x, rel_y, width, height))
}
