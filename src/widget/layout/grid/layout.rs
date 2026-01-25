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
}
