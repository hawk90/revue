//! Grid layout algorithm
//!
//! CSS Grid-like layout implementation optimized for TUI.

use super::node::{ComputedLayout, LayoutNode};
use super::tree::LayoutTree;
use crate::style::GridTrack;

/// Maximum grid dimensions to prevent unbounded memory allocation
const MAX_GRID_SIZE: usize = 1000;

/// Compute grid layout for a node and its children
pub fn compute_grid(
    tree: &mut LayoutTree,
    node_id: u64,
    available_width: u16,
    available_height: u16,
) {
    let node = match tree.get(node_id) {
        Some(n) => n,
        None => return,
    };

    let padding = node.spacing.padding;
    let col_gap = node.flex.column_gap.unwrap_or(node.flex.gap);
    let row_gap = node.flex.row_gap.unwrap_or(node.flex.gap);
    let template_columns = node.grid.template_columns.clone();
    let template_rows = node.grid.template_rows.clone();
    let children: Vec<u64> = node.children.clone();

    if children.is_empty() {
        return;
    }

    // Calculate content area
    let content_width = available_width
        .saturating_sub(padding.left)
        .saturating_sub(padding.right);
    let content_height = available_height
        .saturating_sub(padding.top)
        .saturating_sub(padding.bottom);

    // Determine grid dimensions
    let num_cols = if template_columns.is_empty() {
        // Auto-detect: use square root of children count, minimum 1
        ((children.len() as f32).sqrt().ceil() as usize).clamp(1, MAX_GRID_SIZE)
    } else {
        template_columns.len().min(MAX_GRID_SIZE)
    };

    let num_rows = if template_rows.is_empty() {
        // Auto-detect: calculate needed rows
        let needed = children.len().div_ceil(num_cols);
        needed.clamp(1, MAX_GRID_SIZE)
    } else {
        template_rows.len().min(MAX_GRID_SIZE)
    };

    // Calculate track sizes
    let col_sizes = calculate_track_sizes(content_width, &template_columns, num_cols, col_gap);
    let row_sizes = calculate_track_sizes(content_height, &template_rows, num_rows, row_gap);

    // Calculate track positions
    let col_positions = track_positions(&col_sizes, col_gap);
    let row_positions = track_positions(&row_sizes, row_gap);

    // Place children
    for (i, &child_id) in children.iter().enumerate() {
        let child = match tree.get(child_id) {
            Some(c) => c,
            None => continue,
        };

        // Get placement or auto-place
        let (col, row, col_span, row_span) = get_placement(child, i, num_cols);

        // Clamp to grid bounds
        let col = col.min(num_cols.saturating_sub(1));
        let row = row.min(num_rows.saturating_sub(1));
        let col_end = (col + col_span).min(num_cols);
        let row_end = (row + row_span).min(num_rows);

        // Calculate position and size from tracks
        let x = col_positions.get(col).copied().unwrap_or(0);
        let y = row_positions.get(row).copied().unwrap_or(0);

        // Width spans multiple columns (including gaps between them)
        let x_end = col_positions.get(col_end).copied().unwrap_or(x);
        let w = if col_end > col && col_end <= col_positions.len() {
            x_end
                .saturating_sub(x)
                .saturating_sub(if col_end < num_cols { col_gap } else { 0 })
        } else {
            col_sizes.get(col).copied().unwrap_or(0)
        };

        // Height spans multiple rows
        let y_end = row_positions.get(row_end).copied().unwrap_or(y);
        let h = if row_end > row && row_end <= row_positions.len() {
            y_end
                .saturating_sub(y)
                .saturating_sub(if row_end < num_rows { row_gap } else { 0 })
        } else {
            row_sizes.get(row).copied().unwrap_or(0)
        };

        // Update child's computed layout
        if let Some(child_mut) = tree.get_mut(child_id) {
            child_mut.computed = ComputedLayout::new(
                padding.left.saturating_add(x),
                padding.top.saturating_add(y),
                w,
                h,
            );
        }
    }
}

/// Calculate track sizes from template
fn calculate_track_sizes(
    available: u16,
    template: &[GridTrack],
    count: usize,
    gap: u16,
) -> Vec<u16> {
    if count == 0 {
        return vec![];
    }

    let total_gaps = gap.saturating_mul(count.saturating_sub(1) as u16);
    let available = available.saturating_sub(total_gaps);

    let mut sizes: Vec<u16> = vec![0; count];
    let mut total_fr = 0.0f32;
    let mut remaining = available;

    // If template is empty, treat all as 1fr
    let default_track = GridTrack::Fr(1.0);
    let tracks: Vec<&GridTrack> = if template.is_empty() {
        vec![&default_track; count]
    } else {
        // Extend template to cover all tracks
        (0..count)
            .map(|i| template.get(i).unwrap_or(&default_track))
            .collect()
    };

    // First pass: calculate fixed sizes and collect fr units
    for (i, track) in tracks.iter().enumerate() {
        match track {
            GridTrack::Fixed(size) => {
                sizes[i] = *size;
                remaining = remaining.saturating_sub(*size);
            }
            GridTrack::Fr(fr) => {
                total_fr += fr;
            }
            GridTrack::Auto | GridTrack::MinContent | GridTrack::MaxContent => {
                // Treat as 1fr for simplicity
                total_fr += 1.0;
            }
        }
    }

    // Second pass: distribute remaining space to fr units
    if total_fr > 0.0 {
        let per_fr = (remaining as f32) / total_fr;
        for (i, track) in tracks.iter().enumerate() {
            match track {
                GridTrack::Fr(fr) => {
                    sizes[i] = (per_fr * fr) as u16;
                }
                GridTrack::Auto | GridTrack::MinContent | GridTrack::MaxContent => {
                    sizes[i] = per_fr as u16;
                }
                _ => {}
            }
        }
    }

    sizes
}

/// Get cumulative track positions
fn track_positions(sizes: &[u16], gap: u16) -> Vec<u16> {
    let mut positions = Vec::with_capacity(sizes.len() + 1);
    let mut pos = 0u16;
    positions.push(pos);

    for (i, &size) in sizes.iter().enumerate() {
        pos = pos.saturating_add(size);
        if i < sizes.len() - 1 {
            pos = pos.saturating_add(gap);
        }
        positions.push(pos);
    }

    positions
}

/// Get placement for a child (explicit or auto)
fn get_placement(
    child: &LayoutNode,
    index: usize,
    num_cols: usize,
) -> (usize, usize, usize, usize) {
    let grid = &child.grid;

    // Check for explicit column placement (1-indexed in style)
    let col = if grid.column.start > 0 {
        (grid.column.start - 1) as usize
    } else {
        index % num_cols
    };

    // Check for explicit row placement
    let row = if grid.row.start > 0 {
        (grid.row.start - 1) as usize
    } else {
        index / num_cols
    };

    // Calculate spans
    let col_span = if grid.column.end < 0 {
        // Negative end means span
        (-grid.column.end) as usize
    } else if grid.column.end > grid.column.start {
        (grid.column.end - grid.column.start) as usize
    } else {
        1
    };

    let row_span = if grid.row.end < 0 {
        (-grid.row.end) as usize
    } else if grid.row.end > grid.row.start {
        (grid.row.end - grid.row.start) as usize
    } else {
        1
    };

    (col, row, col_span.max(1), row_span.max(1))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Display;

    fn setup_grid_tree(
        cols: Vec<GridTrack>,
        rows: Vec<GridTrack>,
        child_count: usize,
    ) -> (LayoutTree, u64, Vec<u64>) {
        let mut tree = LayoutTree::new();

        let mut parent = LayoutNode::default();
        parent.id = 1;
        parent.display = Display::Grid;
        parent.grid.template_columns = cols;
        parent.grid.template_rows = rows;

        let mut child_ids = Vec::new();
        for i in 0..child_count {
            let mut child = LayoutNode::default();
            child.id = (i + 2) as u64;
            child_ids.push(child.id);
            tree.insert(child);
        }

        parent.children = child_ids.clone();
        tree.insert(parent);
        tree.set_root(1);

        (tree, 1, child_ids)
    }

    #[test]
    fn test_grid_equal_columns() {
        let (mut tree, parent_id, child_ids) = setup_grid_tree(
            vec![GridTrack::Fr(1.0), GridTrack::Fr(1.0)],
            vec![GridTrack::Fr(1.0)],
            2,
        );

        compute_grid(&mut tree, parent_id, 100, 50);

        let child1 = tree.get(child_ids[0]).unwrap();
        assert_eq!(child1.computed.x, 0);
        assert_eq!(child1.computed.width, 50);

        let child2 = tree.get(child_ids[1]).unwrap();
        assert_eq!(child2.computed.x, 50);
        assert_eq!(child2.computed.width, 50);
    }

    #[test]
    fn test_grid_fixed_columns() {
        let (mut tree, parent_id, child_ids) = setup_grid_tree(
            vec![GridTrack::Fixed(30), GridTrack::Fr(1.0)],
            vec![GridTrack::Fr(1.0)],
            2,
        );

        compute_grid(&mut tree, parent_id, 100, 50);

        let child1 = tree.get(child_ids[0]).unwrap();
        assert_eq!(child1.computed.width, 30);

        let child2 = tree.get(child_ids[1]).unwrap();
        assert_eq!(child2.computed.width, 70); // Remaining space
    }

    #[test]
    fn test_grid_with_gap() {
        let (mut tree, parent_id, child_ids) = setup_grid_tree(
            vec![GridTrack::Fr(1.0), GridTrack::Fr(1.0)],
            vec![GridTrack::Fr(1.0)],
            2,
        );

        if let Some(parent) = tree.get_mut(parent_id) {
            parent.flex.gap = 10;
        }

        compute_grid(&mut tree, parent_id, 100, 50);

        // 100 - 10 gap = 90 / 2 = 45 each
        let child1 = tree.get(child_ids[0]).unwrap();
        assert_eq!(child1.computed.width, 45);

        let child2 = tree.get(child_ids[1]).unwrap();
        assert_eq!(child2.computed.x, 55); // 45 + 10 gap
    }

    #[test]
    fn test_grid_auto_rows() {
        let (mut tree, parent_id, child_ids) = setup_grid_tree(
            vec![GridTrack::Fr(1.0), GridTrack::Fr(1.0)],
            vec![], // Auto rows
            4,
        );

        compute_grid(&mut tree, parent_id, 100, 100);

        // Should create 2 rows
        let child1 = tree.get(child_ids[0]).unwrap();
        assert_eq!(child1.computed.y, 0);

        let child3 = tree.get(child_ids[2]).unwrap();
        assert_eq!(child3.computed.y, 50); // Second row
    }

    #[test]
    fn test_track_sizes_calculation() {
        let sizes = calculate_track_sizes(
            100,
            &[GridTrack::Fixed(20), GridTrack::Fr(1.0), GridTrack::Fr(2.0)],
            3,
            0,
        );

        assert_eq!(sizes[0], 20); // Fixed
        assert_eq!(sizes[1], 26); // 80 / 3 = 26.67
        assert_eq!(sizes[2], 53); // 80 * 2 / 3 = 53.33
    }

    #[test]
    fn test_track_positions() {
        let sizes = vec![30, 40, 30];
        let positions = track_positions(&sizes, 5);

        assert_eq!(positions, vec![0, 35, 80, 110]);
    }
}
