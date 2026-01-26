//! Buffer diff algorithm

use super::{Buffer, Cell};
use crate::layout::Rect;

/// A change to be applied to the terminal
///
/// Represents a single cell change for efficient screen updates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Change {
    /// X coordinate (column)
    pub x: u16,
    /// Y coordinate (row)
    pub y: u16,
    /// Cell to render
    pub cell: Cell,
}

/// Compute the differences between two buffers within specified dirty regions.
pub fn diff(old: &Buffer, new: &Buffer, dirty_rects: &[Rect]) -> Vec<Change> {
    let mut changes = Vec::new();

    // If there are no dirty regions, there are no changes.
    if dirty_rects.is_empty() {
        // As a fallback for now, let's compare the whole screen if no rects are given.
        // This makes sure we don't break rendering logic that doesn't yet produce dirty rects.
        // The ideal state is that dirty_rects is never empty for a real change.
        let full_screen = Rect {
            x: 0,
            y: 0,
            width: new.width(),
            height: new.height(),
        };
        return diff(old, new, &[full_screen]);
    }

    // Optimization: skip HashSet for single rect (no overlap possible)
    // For 2+ rects, we need deduplication for overlapping regions
    if dirty_rects.len() == 1 {
        for rect in dirty_rects {
            // Use saturating_add to prevent overflow near u16::MAX
            let y_end = rect.y.saturating_add(rect.height).min(new.height());
            let x_end = rect.x.saturating_add(rect.width).min(new.width());

            for y in rect.y..y_end {
                for x in rect.x..x_end {
                    let old_cell = old.get(x, y);
                    let new_cell = new.get(x, y);

                    if old_cell != new_cell {
                        if let Some(new) = new_cell {
                            changes.push(Change { x, y, cell: *new });
                        }
                    }
                }
            }
        }
        return changes;
    }

    // A set to keep track of checked cells to avoid redundant comparisons from overlapping rects.
    // This is more efficient than creating a huge list of changes and then deduping.
    let mut checked_cells = std::collections::HashSet::with_capacity(dirty_rects.len() * 10);

    for rect in dirty_rects {
        // Use saturating_add to prevent overflow near u16::MAX
        let y_end = rect.y.saturating_add(rect.height).min(new.height());
        let x_end = rect.x.saturating_add(rect.width).min(new.width());

        for y in rect.y..y_end {
            for x in rect.x..x_end {
                if checked_cells.insert((x, y)) {
                    let old_cell = old.get(x, y);
                    let new_cell = new.get(x, y);

                    if old_cell != new_cell {
                        if let Some(new) = new_cell {
                            changes.push(Change { x, y, cell: *new });
                        }
                    }
                }
            }
        }
    }
    changes
}

// Tests moved to tests/render_tests.rs
