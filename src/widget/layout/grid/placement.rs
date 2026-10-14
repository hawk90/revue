//! Item placement: explicit cells, the auto-placement cursor for both flow
//! directions, and the occupancy map that keeps items apart

use super::core::Grid;
use super::types::GridPlacement;

/// Maximum grid dimensions to prevent unbounded memory allocation
const MAX_GRID_SIZE: usize = 1000;

/// Which cells are taken, by row then column (0-indexed)
type Occupied = Vec<Vec<bool>>;

impl Grid {
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

        let mut grid: Occupied = vec![vec![false; col_count]; row_count];
        let mut auto_col = 0usize;
        let mut auto_row = 0usize;

        for (idx, item) in self.items.iter().enumerate() {
            let placement = &item.placement;

            if starts_beyond_max_grid(placement) {
                continue; // Skip items placed beyond max grid size
            }

            // Determine actual placement
            let (col_start, col_end, row_start, row_end) =
                if placement.col_start > 0 && placement.row_start > 0 {
                    explicit_cell(placement)
                } else {
                    let (col_span, row_span) = auto_spans(placement, col_count);
                    if !self.find_auto_slot(
                        &mut grid,
                        (&mut auto_col, &mut auto_row),
                        col_count,
                        (col_span, row_span),
                    ) {
                        continue; // Skip this item
                    }
                    (auto_col, auto_col + col_span, auto_row, auto_row + row_span)
                };

            mark_occupied(
                &mut grid,
                col_count,
                (col_start, col_end),
                (row_start, row_end),
            );

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

    /// Move the auto-placement cursor to the next slot, in the grid's flow
    /// direction, where a `span` item may go. `false` when no slot turns up
    /// within the iteration bound (the item is then skipped).
    fn find_auto_slot(
        &self,
        grid: &mut Occupied,
        (auto_col, auto_row): (&mut usize, &mut usize),
        col_count: usize,
        span: (usize, usize),
    ) -> bool {
        // Find next available slot with bounded iterations
        let max_iterations = MAX_GRID_SIZE * MAX_GRID_SIZE;
        let mut iterations = 0;
        loop {
            iterations += 1;
            if iterations > max_iterations {
                // Couldn't find slot within bounds, skip item
                return false;
            }

            let done = if self.auto_flow_row {
                row_flow_step(grid, auto_col, auto_row, col_count, span)
            } else {
                col_flow_step(grid, auto_col, auto_row, col_count, span)
            };
            if done {
                return true;
            }
        }
    }

    // Test accessor methods
    #[doc(hidden)]
    pub fn test_auto_place_items(
        &self,
        col_count: usize,
        row_count: usize,
    ) -> Vec<(usize, GridPlacement)> {
        self.auto_place_items(col_count, row_count)
    }
}

/// Whether an explicit start line lies past `MAX_GRID_SIZE`
fn starts_beyond_max_grid(placement: &GridPlacement) -> bool {
    // Validate explicit placement is within bounds
    (placement.col_start > 0 && placement.col_start as usize > MAX_GRID_SIZE)
        || (placement.row_start > 0 && placement.row_start as usize > MAX_GRID_SIZE)
}

/// The 0-indexed (col_start, col_end, row_start, row_end) of an explicitly
/// placed item, clamped to the max grid size
fn explicit_cell(placement: &GridPlacement) -> (usize, usize, usize, usize) {
    (
        ((placement.col_start - 1) as usize).min(MAX_GRID_SIZE - 1),
        ((placement.col_end - 1) as usize).min(MAX_GRID_SIZE),
        ((placement.row_start - 1) as usize).min(MAX_GRID_SIZE - 1),
        ((placement.row_end - 1) as usize).min(MAX_GRID_SIZE),
    )
}

/// The (columns, rows) an auto-placed item spans
fn auto_spans(placement: &GridPlacement, col_count: usize) -> (usize, usize) {
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
    (col_span, row_span)
}

/// One step of row-major auto-placement: `true` when the cursor is at a
/// free slot or the grid cannot grow further, otherwise advance the cursor
/// along the row (wrapping to the next row, which is added if missing).
fn row_flow_step(
    grid: &mut Occupied,
    auto_col: &mut usize,
    auto_row: &mut usize,
    col_count: usize,
    (col_span, row_span): (usize, usize),
) -> bool {
    if *auto_col + col_span <= col_count {
        let fits = (*auto_row..*auto_row + row_span).all(|r| {
            r < grid.len()
                && (*auto_col..*auto_col + col_span).all(|c| c < grid[r].len() && !grid[r][c])
        });
        if fits {
            return true;
        }
    }
    *auto_col += 1;
    if *auto_col >= col_count {
        *auto_col = 0;
        *auto_row += 1;
        // Expand grid if needed (with bounds check)
        while grid.len() <= *auto_row + row_span && grid.len() < MAX_GRID_SIZE {
            grid.push(vec![false; col_count]);
        }
        if *auto_row + row_span > MAX_GRID_SIZE {
            return true; // Can't expand further
        }
    }
    false
}

/// One step of column-major auto-placement: `true` when the cursor is at a
/// free slot or the grid cannot grow further, otherwise advance the cursor
/// down the column (wrapping to the next column, which is added if missing).
fn col_flow_step(
    grid: &mut Occupied,
    auto_col: &mut usize,
    auto_row: &mut usize,
    col_count: usize,
    (col_span, row_span): (usize, usize),
) -> bool {
    if *auto_row + row_span <= grid.len() {
        let fits = (*auto_col..*auto_col + col_span).all(|c| {
            (*auto_row..*auto_row + row_span)
                .all(|r| r < grid.len() && c < grid[r].len() && !grid[r][c])
        });
        if fits {
            return true;
        }
    }
    *auto_row += 1;
    if *auto_row >= grid.len() {
        *auto_row = 0;
        *auto_col += 1;
        if *auto_col >= col_count && col_count < MAX_GRID_SIZE {
            // Expand grid columns
            for row in grid.iter_mut() {
                if row.len() < MAX_GRID_SIZE {
                    row.push(false);
                }
            }
        } else if *auto_col >= MAX_GRID_SIZE {
            return true; // Can't expand further
        }
    }
    false
}

/// Mark the cells of a placed item as taken, growing the map as needed
/// (with bounds checking)
fn mark_occupied(
    grid: &mut Occupied,
    col_count: usize,
    (col_start, col_end): (usize, usize),
    (row_start, row_end): (usize, usize),
) {
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
}
