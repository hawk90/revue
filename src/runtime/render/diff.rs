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
    // Calculate capacity based on actual dirty region area to minimize reallocations.
    let total_area: usize = dirty_rects
        .iter()
        .map(|r| (r.width as usize) * (r.height as usize))
        .sum();
    let mut checked_cells = std::collections::HashSet::with_capacity(total_area);

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;

    #[test]
    fn test_change_struct() {
        let cell = Cell::new('A').fg(Color::RED);
        let change = Change { x: 10, y: 20, cell };
        assert_eq!(change.x, 10);
        assert_eq!(change.y, 20);
        assert_eq!(change.cell.symbol, 'A');
    }

    #[test]
    fn test_diff_empty_dirty_rects() {
        let old = Buffer::new(80, 24);
        let new = Buffer::new(80, 24);
        let changes = diff(&old, &new, &[]);
        // Should treat as full screen diff (no changes in empty buffers)
        assert!(changes.is_empty());
    }

    #[test]
    fn test_diff_single_rect() {
        let old = Buffer::new(80, 24);
        let mut new = Buffer::new(80, 24);

        // Make a change at (5, 5)
        new.get_mut(5, 5).map(|c| *c = Cell::new('X'));

        let rect = Rect::new(0, 0, 80, 24);
        let changes = diff(&old, &new, &[rect]);

        // Should have one change
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].x, 5);
        assert_eq!(changes[0].y, 5);
    }

    #[test]
    fn test_diff_no_changes() {
        let old = Buffer::new(80, 24);
        let new = Buffer::new(80, 24);

        let rect = Rect::new(0, 0, 80, 24);
        let changes = diff(&old, &new, &[rect]);

        assert!(changes.is_empty());
    }

    #[test]
    fn test_diff_multiple_rects() {
        let old = Buffer::new(80, 24);
        let mut new = Buffer::new(80, 24);

        // Make changes in different regions
        new.get_mut(10, 10).map(|c| *c = Cell::new('A'));
        new.get_mut(50, 15).map(|c| *c = Cell::new('B'));

        let rect1 = Rect::new(0, 0, 20, 20);
        let rect2 = Rect::new(40, 10, 20, 20);
        let changes = diff(&old, &new, &[rect1, rect2]);

        assert_eq!(changes.len(), 2);
    }

    #[test]
    fn test_diff_overlapping_rects() {
        let old = Buffer::new(80, 24);
        let mut new = Buffer::new(80, 24);

        // Make a single change in the overlap region
        new.get_mut(15, 15).map(|c| *c = Cell::new('X'));

        let rect1 = Rect::new(0, 0, 20, 20);
        let rect2 = Rect::new(10, 10, 20, 20);
        let changes = diff(&old, &new, &[rect1, rect2]);

        // Should only report the change once despite overlapping rects
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].x, 15);
        assert_eq!(changes[0].y, 15);
    }

    #[test]
    fn test_change_partial_eq() {
        let cell = Cell::new('A');
        let change1 = Change { x: 5, y: 10, cell };
        let change2 = Change { x: 5, y: 10, cell };
        assert_eq!(change1, change2);
    }
}
