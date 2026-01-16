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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;

    // Helper to create a test rect
    fn rect(x: u16, y: u16, width: u16, height: u16) -> Rect {
        Rect {
            x,
            y,
            width,
            height,
        }
    }

    #[test]
    fn test_diff_empty_rects_fallbacks_to_full_diff() {
        let buf1 = Buffer::new(10, 10);
        let mut buf2 = Buffer::new(10, 10);
        buf2.set(5, 5, Cell::new('X'));

        // No dirty rects should behave like a full diff for now
        let changes = diff(&buf1, &buf2, &[]);
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].x, 5);
    }

    #[test]
    fn test_diff_single_dirty_rect() {
        let buf1 = Buffer::new(10, 10);
        let mut buf2 = Buffer::new(10, 10);
        buf2.set(5, 5, Cell::new('X')); // Change is inside the rect

        let changes = diff(&buf1, &buf2, &[rect(5, 5, 1, 1)]);
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].x, 5);
        assert_eq!(changes[0].y, 5);
    }

    #[test]
    fn test_diff_change_outside_dirty_rect() {
        let buf1 = Buffer::new(10, 10);
        let mut buf2 = Buffer::new(10, 10);
        buf2.set(5, 5, Cell::new('X')); // Change is outside the rect

        let changes = diff(&buf1, &buf2, &[rect(0, 0, 1, 1)]);
        assert!(changes.is_empty());
    }

    #[test]
    fn test_diff_multiple_dirty_rects() {
        let buf1 = Buffer::new(10, 10);
        let mut buf2 = Buffer::new(10, 10);
        buf2.set(1, 1, Cell::new('A'));
        buf2.set(8, 8, Cell::new('B'));

        let dirty_rects = vec![rect(1, 1, 1, 1), rect(8, 8, 1, 1)];
        let changes = diff(&buf1, &buf2, &dirty_rects);
        assert_eq!(changes.len(), 2);
    }

    #[test]
    fn test_diff_overlapping_dirty_rects() {
        let buf1 = Buffer::new(10, 10);
        let mut buf2 = Buffer::new(10, 10);
        buf2.set(2, 2, Cell::new('C'));

        // Overlapping rects, both containing the change
        let dirty_rects = vec![rect(0, 0, 5, 5), rect(2, 2, 5, 5)];
        let changes = diff(&buf1, &buf2, &dirty_rects);

        // HashSet should ensure we only get one change
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].x, 2);
        assert_eq!(changes[0].y, 2);
    }

    #[test]
    fn test_original_diff_logic_with_full_rect() {
        let full_rect = rect(0, 0, 10, 10);

        // test_diff_identical_buffers
        let buf1 = Buffer::new(10, 10);
        let buf2 = Buffer::new(10, 10);
        let changes = diff(&buf1, &buf2, &[full_rect]);
        assert!(changes.is_empty());

        // test_diff_single_change
        let mut buf2_single = buf1.clone();
        buf2_single.set(5, 5, Cell::new('X'));
        let changes_single = diff(&buf1, &buf2_single, &[full_rect]);
        assert_eq!(changes_single.len(), 1);

        // test_diff_multiple_changes
        let mut buf2_multi = buf1.clone();
        buf2_multi.put_str(0, 0, "Hello");
        let changes_multi = diff(&buf1, &buf2_multi, &[full_rect]);
        assert_eq!(changes_multi.len(), 5);

        // test_diff_no_change_same_content
        let mut buf1_same = buf1.clone();
        let mut buf2_same = buf1.clone();
        buf1_same.set(5, 5, Cell::new('X'));
        buf2_same.set(5, 5, Cell::new('X'));
        let changes_same = diff(&buf1_same, &buf2_same, &[full_rect]);
        assert!(changes_same.is_empty());
    }

    #[test]
    fn test_diff_no_overflow_near_u16_max() {
        // Test that diff doesn't panic with rects that would overflow u16::MAX
        // This is the fix for issue #145
        let buf1 = Buffer::new(100, 100);
        let buf2 = Buffer::new(100, 100);

        // Rect where x + width would overflow
        let overflow_x = rect(u16::MAX - 5, 0, 10, 1);
        let changes = diff(&buf1, &buf2, &[overflow_x]);
        assert!(changes.is_empty()); // No changes, but importantly no panic

        // Rect where y + height would overflow
        let overflow_y = rect(0, u16::MAX - 5, 1, 10);
        let changes = diff(&buf1, &buf2, &[overflow_y]);
        assert!(changes.is_empty());

        // Rect where both would overflow
        let overflow_both = rect(u16::MAX - 5, u16::MAX - 5, 10, 10);
        let changes = diff(&buf1, &buf2, &[overflow_both]);
        assert!(changes.is_empty());

        // Rect at exact u16::MAX
        let at_max = rect(u16::MAX, u16::MAX, 1, 1);
        let changes = diff(&buf1, &buf2, &[at_max]);
        assert!(changes.is_empty());
    }

    #[test]
    fn test_diff_rect_exceeds_buffer() {
        // Test that rects larger than the buffer are handled correctly
        let buf1 = Buffer::new(10, 10);
        let mut buf2 = Buffer::new(10, 10);
        buf2.set(5, 5, Cell::new('X'));

        // Rect larger than buffer
        let large_rect = rect(0, 0, 1000, 1000);
        let changes = diff(&buf1, &buf2, &[large_rect]);
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].x, 5);
        assert_eq!(changes[0].y, 5);
    }
}
