//! Grid layout calculations

use super::core::Grid;
use super::types::{GridPlacement, TrackSize};

/// Maximum grid dimensions to prevent unbounded memory allocation
const MAX_GRID_SIZE: usize = 1000;

impl Grid {
    /// Calculate track sizes
    pub(crate) fn calculate_tracks(
        &self,
        available: u16,
        tracks: &[TrackSize],
        _auto_size: TrackSize,
        gap: u16,
    ) -> Vec<u16> {
        if tracks.is_empty() {
            return vec![];
        }

        let total_gaps = (tracks.len().saturating_sub(1)) as u16 * gap;
        let available = available.saturating_sub(total_gaps);

        // First pass: calculate fixed sizes and collect fr units
        let mut sizes: Vec<u16> = vec![0; tracks.len()];
        let mut total_fr = 0.0f32;
        let mut remaining = available;

        for (i, track) in tracks.iter().enumerate() {
            match track {
                TrackSize::Fixed(size) => {
                    sizes[i] = *size;
                    remaining = remaining.saturating_sub(*size);
                }
                TrackSize::Percent(pct) => {
                    let size = ((available as f32) * pct / 100.0) as u16;
                    sizes[i] = size;
                    remaining = remaining.saturating_sub(size);
                }
                TrackSize::Fr(fr) => {
                    total_fr += fr;
                }
                TrackSize::Auto | TrackSize::MinContent | TrackSize::MaxContent => {
                    // For now, treat auto as 1fr
                    total_fr += 1.0;
                }
            }
        }

        // Second pass: distribute remaining space to fr units
        if total_fr > 0.0 {
            let per_fr = (remaining as f32) / total_fr;
            for (i, track) in tracks.iter().enumerate() {
                match track {
                    TrackSize::Fr(fr) => {
                        sizes[i] = (per_fr * fr) as u16;
                    }
                    TrackSize::Auto | TrackSize::MinContent | TrackSize::MaxContent => {
                        sizes[i] = per_fr as u16;
                    }
                    _ => {}
                }
            }
        }

        sizes
    }

    /// Get track positions (cumulative)
    pub(crate) fn track_positions(&self, sizes: &[u16], gap: u16) -> Vec<u16> {
        let mut positions = Vec::with_capacity(sizes.len() + 1);
        let mut pos = 0u16;
        positions.push(pos);

        for (i, &size) in sizes.iter().enumerate() {
            pos += size;
            if i < sizes.len() - 1 {
                pos += gap;
            }
            positions.push(pos);
        }

        positions
    }

    /// Auto-place items without explicit placement
    ///
    /// Grid dimensions are limited to `MAX_GRID_SIZE` to prevent unbounded memory allocation.
    pub(crate) fn auto_place_items(
        &self,
        col_count: usize,
        row_count: usize,
    ) -> Vec<(usize, GridPlacement)> {
        let mut placements = Vec::new();

        // Clamp initial dimensions to prevent excessive allocation
        let col_count = col_count.clamp(1, MAX_GRID_SIZE);
        let row_count = row_count.clamp(1, MAX_GRID_SIZE);

        let mut grid: Vec<Vec<bool>> = vec![vec![false; col_count]; row_count];
        let mut auto_col = 0usize;
        let mut auto_row = 0usize;

        for (idx, item) in self.items.iter().enumerate() {
            let placement = &item.placement;

            // Validate explicit placement is within bounds
            if placement.col_start > 0 && placement.col_start as usize > MAX_GRID_SIZE {
                continue; // Skip items placed beyond max grid size
            }
            if placement.row_start > 0 && placement.row_start as usize > MAX_GRID_SIZE {
                continue;
            }

            // Determine actual placement
            let (col_start, col_end, row_start, row_end) = if placement.col_start > 0
                && placement.row_start > 0
            {
                // Explicit placement - clamp to max grid size
                (
                    ((placement.col_start - 1) as usize).min(MAX_GRID_SIZE - 1),
                    ((placement.col_end - 1) as usize).min(MAX_GRID_SIZE),
                    ((placement.row_start - 1) as usize).min(MAX_GRID_SIZE - 1),
                    ((placement.row_end - 1) as usize).min(MAX_GRID_SIZE),
                )
            } else {
                // Auto-placement
                let col_span = if placement.col_end > placement.col_start {
                    ((placement.col_end - placement.col_start) as usize).min(col_count)
                } else {
                    1
                };
                let row_span = if placement.row_end > placement.row_start {
                    ((placement.row_end - placement.row_start) as usize).min(MAX_GRID_SIZE)
                } else {
                    1
                };

                // Find next available slot with bounded iterations
                let max_iterations = MAX_GRID_SIZE * MAX_GRID_SIZE;
                let mut iterations = 0;
                loop {
                    iterations += 1;
                    if iterations > max_iterations {
                        // Couldn't find slot within bounds, skip item
                        break;
                    }

                    if self.auto_flow_row {
                        if auto_col + col_span <= col_count {
                            let fits = (auto_row..auto_row + row_span).all(|r| {
                                r < grid.len()
                                    && (auto_col..auto_col + col_span)
                                        .all(|c| c < grid[r].len() && !grid[r][c])
                            });
                            if fits {
                                break;
                            }
                        }
                        auto_col += 1;
                        if auto_col >= col_count {
                            auto_col = 0;
                            auto_row += 1;
                            // Expand grid if needed (with bounds check)
                            while grid.len() <= auto_row + row_span && grid.len() < MAX_GRID_SIZE {
                                grid.push(vec![false; col_count]);
                            }
                            if auto_row + row_span > MAX_GRID_SIZE {
                                break; // Can't expand further
                            }
                        }
                    } else {
                        if auto_row + row_span <= grid.len() {
                            let fits = (auto_col..auto_col + col_span).all(|c| {
                                (auto_row..auto_row + row_span)
                                    .all(|r| r < grid.len() && c < grid[r].len() && !grid[r][c])
                            });
                            if fits {
                                break;
                            }
                        }
                        auto_row += 1;
                        if auto_row >= grid.len() {
                            auto_row = 0;
                            auto_col += 1;
                            if auto_col >= col_count && col_count < MAX_GRID_SIZE {
                                // Expand grid columns
                                for row in &mut grid {
                                    if row.len() < MAX_GRID_SIZE {
                                        row.push(false);
                                    }
                                }
                            } else if auto_col >= MAX_GRID_SIZE {
                                break; // Can't expand further
                            }
                        }
                    }
                }

                if iterations > max_iterations {
                    continue; // Skip this item
                }

                (auto_col, auto_col + col_span, auto_row, auto_row + row_span)
            };

            // Mark cells as occupied (with bounds checking)
            for r in row_start..row_end.min(MAX_GRID_SIZE) {
                while grid.len() <= r && grid.len() < MAX_GRID_SIZE {
                    grid.push(vec![false; col_count]);
                }
                if r >= grid.len() {
                    break;
                }
                for c in col_start..col_end.min(MAX_GRID_SIZE) {
                    while grid[r].len() <= c && grid[r].len() < MAX_GRID_SIZE {
                        grid[r].push(false);
                    }
                    if c < grid[r].len() {
                        grid[r][c] = true;
                    }
                }
            }

            placements.push((
                idx,
                GridPlacement {
                    col_start: (col_start + 1) as u16,
                    col_end: (col_end + 1) as u16,
                    row_start: (row_start + 1) as u16,
                    row_end: (row_end + 1) as u16,
                },
            ));
        }

        placements
    }

    // Test accessor methods
    #[doc(hidden)]
    pub fn test_calculate_tracks(
        &self,
        available: u16,
        tracks: &[TrackSize],
        auto_size: TrackSize,
        gap: u16,
    ) -> Vec<u16> {
        self.calculate_tracks(available, tracks, auto_size, gap)
    }

    #[doc(hidden)]
    pub fn test_track_positions(&self, sizes: &[u16], gap: u16) -> Vec<u16> {
        self.track_positions(sizes, gap)
    }

    #[doc(hidden)]
    pub fn test_auto_place_items(
        &self,
        col_count: usize,
        row_count: usize,
    ) -> Vec<(usize, GridPlacement)> {
        self.auto_place_items(col_count, row_count)
    }
}

#[cfg(test)]
mod tests {
    use super::super::types::GridItem;
    use super::*;
    use crate::widget::Text;

    // =========================================================================
    // calculate_tracks tests
    // =========================================================================

    #[test]
    fn test_calculate_tracks_empty() {
        let grid = Grid::new();
        let tracks = grid.calculate_tracks(100, &[], TrackSize::Fr(1.0), 0);
        assert!(tracks.is_empty());
    }

    #[test]
    fn test_calculate_tracks_all_fixed() {
        let grid = Grid::new();
        let tracks = &[
            TrackSize::Fixed(10),
            TrackSize::Fixed(20),
            TrackSize::Fixed(30),
        ];
        let result = grid.calculate_tracks(100, tracks, TrackSize::Fr(1.0), 0);
        assert_eq!(result, vec![10, 20, 30]);
    }

    #[test]
    fn test_calculate_tracks_all_fr() {
        let grid = Grid::new();
        let tracks = &[TrackSize::Fr(1.0), TrackSize::Fr(2.0), TrackSize::Fr(1.0)];
        let result = grid.calculate_tracks(100, tracks, TrackSize::Fr(1.0), 0);
        // Total fr = 4, so: 100/4=25 per fr unit
        // 1fr=25, 2fr=50, 1fr=25
        assert_eq!(result, vec![25, 50, 25]);
    }

    #[test]
    fn test_calculate_tracks_mixed_fixed_and_fr() {
        let grid = Grid::new();
        let tracks = &[
            TrackSize::Fixed(20),
            TrackSize::Fr(1.0),
            TrackSize::Fixed(30),
        ];
        let result = grid.calculate_tracks(100, tracks, TrackSize::Fr(1.0), 0);
        // Fixed: 20+30=50, remaining: 50, fr: 1
        assert_eq!(result[0], 20);
        assert_eq!(result[1], 50);
        assert_eq!(result[2], 30);
    }

    #[test]
    fn test_calculate_tracks_with_gap() {
        let grid = Grid::new();
        let tracks = &[TrackSize::Fr(1.0), TrackSize::Fr(1.0)];
        let result = grid.calculate_tracks(100, tracks, TrackSize::Fr(1.0), 10);
        // Total gap: 10, available: 90, each: 45
        assert_eq!(result, vec![45, 45]);
    }

    #[test]
    fn test_calculate_tracks_percent() {
        let grid = Grid::new();
        let tracks = &[TrackSize::Percent(50.0), TrackSize::Percent(50.0)];
        let result = grid.calculate_tracks(100, tracks, TrackSize::Fr(1.0), 0);
        // 50% of 100 = 50 each
        assert_eq!(result, vec![50, 50]);
    }

    #[test]
    fn test_calculate_tracks_auto_as_fr() {
        let grid = Grid::new();
        let tracks = &[TrackSize::Auto, TrackSize::Auto];
        let result = grid.calculate_tracks(100, tracks, TrackSize::Fr(1.0), 0);
        // Auto treated as 1fr
        assert_eq!(result, vec![50, 50]);
    }

    #[test]
    fn test_calculate_tracks_min_content_as_fr() {
        let grid = Grid::new();
        let tracks = &[TrackSize::MinContent, TrackSize::MinContent];
        let result = grid.calculate_tracks(100, tracks, TrackSize::Fr(1.0), 0);
        // MinContent treated as 1fr
        assert_eq!(result, vec![50, 50]);
    }

    #[test]
    fn test_calculate_tracks_max_content_as_fr() {
        let grid = Grid::new();
        let tracks = &[TrackSize::MaxContent, TrackSize::MaxContent];
        let result = grid.calculate_tracks(100, tracks, TrackSize::Fr(1.0), 0);
        // MaxContent treated as 1fr
        assert_eq!(result, vec![50, 50]);
    }

    #[test]
    fn test_calculate_tracks_insufficient_space() {
        let grid = Grid::new();
        let tracks = &[TrackSize::Fixed(80), TrackSize::Fr(1.0)];
        let result = grid.calculate_tracks(100, tracks, TrackSize::Fr(1.0), 0);
        // Fixed uses 80, fr gets remaining 20
        assert_eq!(result, vec![80, 20]);
    }

    #[test]
    fn test_calculate_tracks_zero_fr() {
        let grid = Grid::new();
        let tracks = &[TrackSize::Fixed(10), TrackSize::Fixed(20)];
        let result = grid.calculate_tracks(100, tracks, TrackSize::Fr(1.0), 0);
        // No fr tracks, sizes are just the fixed values
        assert_eq!(result, vec![10, 20]);
    }

    #[test]
    fn test_calculate_tracks_overflow_protection() {
        let grid = Grid::new();
        let tracks = &[TrackSize::Fixed(150), TrackSize::Fixed(150)];
        let result = grid.calculate_tracks(100, tracks, TrackSize::Fr(1.0), 0);
        // Saturating subtraction should handle this
        assert_eq!(result[0], 150);
        // Second value also set even though it exceeds available
        assert_eq!(result[1], 150);
    }

    // =========================================================================
    // track_positions tests
    // =========================================================================

    #[test]
    fn test_track_positions_empty() {
        let grid = Grid::new();
        let positions = grid.track_positions(&[], 0);
        assert_eq!(positions, vec![0]);
    }

    #[test]
    fn test_track_positions_single_track() {
        let grid = Grid::new();
        let positions = grid.track_positions(&[100], 0);
        assert_eq!(positions, vec![0, 100]);
    }

    #[test]
    fn test_track_positions_multiple_tracks_no_gap() {
        let grid = Grid::new();
        let positions = grid.track_positions(&[10, 20, 30], 0);
        assert_eq!(positions, vec![0, 10, 30, 60]);
    }

    #[test]
    fn test_track_positions_with_gap() {
        let grid = Grid::new();
        let positions = grid.track_positions(&[10, 20, 30], 5);
        assert_eq!(positions, vec![0, 15, 40, 70]);
        // 0 + 10 + 5 = 15 (gap added after track 0)
        // 15 + 20 + 5 = 40 (gap added after track 1)
        // 40 + 30 = 70 (no gap after last track)
    }

    #[test]
    fn test_track_positions_cumulative() {
        let grid = Grid::new();
        let positions = grid.track_positions(&[15, 25, 35], 2);
        // 0
        // 0 + 15 + gap(2) = 17
        // 17 + 25 + gap(2) = 44
        // 44 + 35 = 79 (no gap after last track)
        assert_eq!(positions, vec![0, 17, 44, 79]);
    }

    #[test]
    fn test_track_positions_zero_size_tracks() {
        let grid = Grid::new();
        let positions = grid.track_positions(&[0, 0, 0], 1);
        assert_eq!(positions, vec![0, 1, 2, 2]);
        // 0 + 0 + 1 = 1 (gap added after track 0)
        // 1 + 0 + 1 = 2 (gap added after track 1)
        // 2 + 0 = 2 (no gap after last track)
    }

    #[test]
    fn test_track_positions_large_gap() {
        let grid = Grid::new();
        let positions = grid.track_positions(&[10, 10], 50);
        assert_eq!(positions, vec![0, 60, 70]);
        // 0 + 10 + 50 = 60 (gap added after track 0)
        // 60 + 10 = 70 (no gap after last track)
    }

    // =========================================================================
    // auto_place_items tests
    // =========================================================================

    #[test]
    fn test_auto_place_items_empty() {
        let grid = Grid::new();
        let placements = grid.auto_place_items(3, 3);
        assert!(placements.is_empty());
    }

    #[test]
    fn test_auto_place_items_single_item_auto() {
        let grid = Grid::new().child(Text::new("A"));
        let placements = grid.auto_place_items(2, 2);
        assert_eq!(placements.len(), 1);
        assert_eq!(placements[0].0, 0); // index
        assert_eq!(placements[0].1.col_start, 1); // 0-indexed + 1
        assert_eq!(placements[0].1.col_end, 2);
        assert_eq!(placements[0].1.row_start, 1);
        assert_eq!(placements[0].1.row_end, 2);
    }

    #[test]
    fn test_auto_place_items_explicit_placement() {
        let grid = Grid::new().item(GridItem::new(Text::new("A")).at(2, 3));
        let placements = grid.auto_place_items(5, 5);
        assert_eq!(placements.len(), 1);
        // GridItem uses 1-indexed, auto_place_items returns 1-indexed
        assert_eq!(placements[0].1.col_start, 2);
        assert_eq!(placements[0].1.col_end, 3);
        assert_eq!(placements[0].1.row_start, 3);
        assert_eq!(placements[0].1.row_end, 4);
    }

    #[test]
    fn test_auto_place_items_row_flow() {
        let grid = Grid::new()
            .auto_flow_row()
            .child(Text::new("A"))
            .child(Text::new("B"))
            .child(Text::new("C"));
        let placements = grid.auto_place_items(2, 2);
        assert_eq!(placements.len(), 3);
        // Row flow fills columns first
        // A: (0,0), B: (1,0), C: (0,1)
        assert_eq!(placements[0].1.col_start, 1);
        assert_eq!(placements[0].1.row_start, 1);
        assert_eq!(placements[1].1.col_start, 2);
        assert_eq!(placements[1].1.row_start, 1);
        assert_eq!(placements[2].1.col_start, 1);
        assert_eq!(placements[2].1.row_start, 2);
    }

    #[test]
    fn test_auto_place_items_col_flow() {
        let grid = Grid::new()
            .auto_flow_col()
            .child(Text::new("A"))
            .child(Text::new("B"))
            .child(Text::new("C"));
        let placements = grid.auto_place_items(2, 2);
        assert_eq!(placements.len(), 3);
        // Column flow fills rows first
        assert_eq!(placements[0].1.col_start, 1);
        assert_eq!(placements[0].1.row_start, 1);
        assert_eq!(placements[1].1.col_start, 1);
        assert_eq!(placements[1].1.row_start, 2);
        assert_eq!(placements[2].1.col_start, 2);
        assert_eq!(placements[2].1.row_start, 1);
    }

    #[test]
    fn test_auto_place_items_with_span() {
        let grid = Grid::new()
            .item(GridItem::new(Text::new("A")).col_span(2))
            .child(Text::new("B"));
        let placements = grid.auto_place_items(3, 2);
        assert_eq!(placements.len(), 2);
        // A spans 2 columns: (0,0) to (2,0)
        // B goes to (0,1) after A
    }

    #[test]
    fn test_auto_place_items_bounds_protection() {
        // Create a grid with items that would exceed MAX_GRID_SIZE
        let mut grid = Grid::new();
        for _ in 0..5 {
            grid.items.push(GridItem::new(Text::new("Test")));
        }
        // Should not panic even with large coordinates
        let placements = grid.auto_place_items(MAX_GRID_SIZE, MAX_GRID_SIZE);
        assert!(!placements.is_empty());
    }

    #[test]
    fn test_auto_place_items_explicit_out_of_bounds() {
        let grid = Grid::new().item(GridItem::new(Text::new("A")).at(MAX_GRID_SIZE as u16 + 10, 1));
        let placements = grid.auto_place_items(10, 10);
        // Item beyond max grid size should be skipped
        assert_eq!(placements.len(), 0);
    }

    #[test]
    fn test_auto_place_items_many_items() {
        let mut grid = Grid::new().auto_flow_row();
        for i in 0..10 {
            grid.items
                .push(GridItem::new(Text::new(format!("Item{}", i))));
        }
        let placements = grid.auto_place_items(3, 5);
        assert_eq!(placements.len(), 10);
    }

    #[test]
    fn test_auto_place_items_with_column_span() {
        let grid = Grid::new()
            .item(GridItem::new(Text::new("A")).at(1, 1).col_span(2))
            .child(Text::new("B"));
        let placements = grid.auto_place_items(3, 2);
        assert_eq!(placements.len(), 2);
        // A at (1,1) spanning 2 cols: col_start=1, col_end=3
        assert_eq!(placements[0].1.col_start, 1);
        assert_eq!(placements[0].1.col_end, 3);
        // B should be placed at (1,2) since (2,1) is occupied by A's span
    }

    #[test]
    fn test_auto_place_items_with_row_span() {
        let grid = Grid::new()
            .item(GridItem::new(Text::new("A")).at(1, 1).row_span(2))
            .child(Text::new("B"));
        let placements = grid.auto_place_items(2, 3);
        assert_eq!(placements.len(), 2);
        // A at (1,1) spanning 2 rows: row_start=1, row_end=3
        assert_eq!(placements[0].1.row_start, 1);
        assert_eq!(placements[0].1.row_end, 3);
    }

    // =========================================================================
    // Edge case tests
    // =========================================================================

    #[test]
    fn test_calculate_tracks_zero_available() {
        let grid = Grid::new();
        let tracks = &[TrackSize::Fr(1.0), TrackSize::Fr(1.0)];
        let result = grid.calculate_tracks(0, tracks, TrackSize::Fr(1.0), 0);
        assert_eq!(result, vec![0, 0]);
    }

    #[test]
    fn test_track_positions_empty_sizes() {
        let grid = Grid::new();
        let positions = grid.track_positions(&[], 5);
        assert_eq!(positions, vec![0]);
    }

    #[test]
    fn test_auto_place_items_zero_dimensions() {
        let grid = Grid::new().child(Text::new("Test"));
        // Even with 0 columns/rows, should clamp to minimum 1
        let placements = grid.auto_place_items(0, 0);
        assert!(!placements.is_empty());
    }

    #[test]
    fn test_calculate_tracks_single_fr() {
        let grid = Grid::new();
        let tracks = &[TrackSize::Fr(1.0)];
        let result = grid.calculate_tracks(50, tracks, TrackSize::Fr(1.0), 0);
        assert_eq!(result, vec![50]);
    }

    #[test]
    fn test_calculate_tracks_gap_exceeds_available() {
        let grid = Grid::new();
        let tracks = &[TrackSize::Fr(1.0), TrackSize::Fr(1.0)];
        let result = grid.calculate_tracks(5, tracks, TrackSize::Fr(1.0), 10);
        // Gap of 10 exceeds available 5, should saturate to 0
        // Total gaps would be 10, available becomes 0
        assert_eq!(result[0], 0);
        assert_eq!(result[1], 0);
    }
}
