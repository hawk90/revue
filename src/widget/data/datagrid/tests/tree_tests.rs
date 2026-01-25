#![allow(unused_imports)]

use super::super::*;

#[test]
fn test_tree_grid_basic() {
    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(
            GridRow::new()
                .cell("name", "Parent")
                .expanded(true)
                .child(GridRow::new().cell("name", "Child 1"))
                .child(GridRow::new().cell("name", "Child 2")),
        )
        .tree_mode(true);

    assert!(grid.is_tree_mode());
    // Tree cache should have 3 items: Parent + 2 children (expanded)
    assert_eq!(grid.tree_cache.len(), 3);
}

#[test]
fn test_tree_grid_collapsed() {
    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(
            GridRow::new()
                .cell("name", "Parent")
                .expanded(false)
                .child(GridRow::new().cell("name", "Child 1"))
                .child(GridRow::new().cell("name", "Child 2")),
        )
        .tree_mode(true);

    // Tree cache should have 1 item: only Parent (collapsed)
    assert_eq!(grid.tree_cache.len(), 1);
}

#[test]
fn test_tree_grid_toggle_expand() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(
            GridRow::new()
                .cell("name", "Parent")
                .expanded(false)
                .child(GridRow::new().cell("name", "Child")),
        )
        .tree_mode(true);

    // Initially collapsed
    assert_eq!(grid.tree_cache.len(), 1);

    // Toggle expand
    grid.toggle_expand();

    // Now expanded
    assert_eq!(grid.tree_cache.len(), 2);
}

#[test]
fn test_tree_grid_expand_collapse_all() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(
            GridRow::new()
                .cell("name", "A")
                .expanded(false)
                .child(GridRow::new().cell("name", "A1")),
        )
        .row(
            GridRow::new()
                .cell("name", "B")
                .expanded(false)
                .child(GridRow::new().cell("name", "B1")),
        )
        .tree_mode(true);

    // Initially collapsed (2 parents only)
    assert_eq!(grid.tree_cache.len(), 2);

    // Expand all
    grid.expand_all();
    assert_eq!(grid.tree_cache.len(), 4); // 2 parents + 2 children

    // Collapse all
    grid.collapse_all();
    assert_eq!(grid.tree_cache.len(), 2); // 2 parents only
}

#[test]
fn test_tree_indent_depth_zero() {
    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(GridRow::new().cell("name", "Root"))
        .tree_mode(true);

    // Root level node (depth 0) should have no indent
    let node = &grid.tree_cache[0];
    let indent = grid.get_tree_indent(node);
    assert!(indent.is_empty());
}

#[test]
fn test_tree_indent_nested() {
    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(
            GridRow::new().cell("name", "Parent").expanded(true).child(
                GridRow::new()
                    .cell("name", "Child")
                    .expanded(true)
                    .child(GridRow::new().cell("name", "Grandchild")),
            ),
        )
        .tree_mode(true);

    // Check that we have 3 nodes
    assert_eq!(grid.tree_cache.len(), 3);

    // Child (depth 1) should have branch
    let child_node = &grid.tree_cache[1];
    let indent = grid.get_tree_indent(child_node);
    assert!(indent.contains('└') || indent.contains('├'));
}

#[test]
fn test_tree_indicator() {
    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(
            GridRow::new()
                .cell("name", "Parent")
                .expanded(true)
                .child(GridRow::new().cell("name", "Child")),
        )
        .row(GridRow::new().cell("name", "Leaf"))
        .tree_mode(true);

    // Parent (expanded, has children) -> ▼
    let parent = &grid.tree_cache[0];
    assert_eq!(grid.get_tree_indicator(parent), "▼ ");

    // Leaf (no children) -> spaces
    let leaf = &grid.tree_cache[2];
    assert_eq!(grid.get_tree_indicator(leaf), "  ");
}

#[test]
fn test_tree_indicator_collapsed() {
    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(
            GridRow::new()
                .cell("name", "Parent")
                .expanded(false)
                .child(GridRow::new().cell("name", "Child")),
        )
        .tree_mode(true);

    // Collapsed parent -> ▶
    let parent = &grid.tree_cache[0];
    assert_eq!(grid.get_tree_indicator(parent), "▶ ");
}

#[test]
fn test_get_row_by_path() {
    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(
            GridRow::new()
                .cell("name", "Parent")
                .child(GridRow::new().cell("name", "Child")),
        )
        .tree_mode(true);

    // Get root row
    let root = grid.get_row_by_path(&[0]);
    assert!(root.is_some());
    assert_eq!(root.unwrap().get("name"), Some("Parent"));

    // Get child row
    let child = grid.get_row_by_path(&[0, 0]);
    assert!(child.is_some());
    assert_eq!(child.unwrap().get("name"), Some("Child"));

    // Invalid path
    let invalid = grid.get_row_by_path(&[99]);
    assert!(invalid.is_none());

    // Empty path
    let empty = grid.get_row_by_path(&[]);
    assert!(empty.is_none());
}

#[test]
fn test_expand_on_leaf_node() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(GridRow::new().cell("name", "Leaf"))
        .tree_mode(true);

    // Expand on leaf should do nothing
    let count_before = grid.tree_cache.len();
    grid.expand();
    assert_eq!(grid.tree_cache.len(), count_before);
}

#[test]
fn test_collapse_on_leaf_node() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(GridRow::new().cell("name", "Leaf"))
        .tree_mode(true);

    // Collapse on leaf should do nothing
    let count_before = grid.tree_cache.len();
    grid.collapse();
    assert_eq!(grid.tree_cache.len(), count_before);
}

#[test]
fn test_tree_mode_disabled_operations() {
    let mut grid = DataGrid::new().column(GridColumn::new("name", "Name")).row(
        GridRow::new()
            .cell("name", "Parent")
            .child(GridRow::new().cell("name", "Child")),
    );
    // Tree mode is disabled by default

    // These should be no-ops
    grid.toggle_expand();
    grid.expand();
    grid.collapse();
    grid.expand_all();
    grid.collapse_all();

    // Tree cache should be empty
    assert!(grid.tree_cache.is_empty());
}

#[test]
fn test_tree_grid_deep_nesting() {
    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(
            GridRow::new().cell("name", "L1").expanded(true).child(
                GridRow::new().cell("name", "L2").expanded(true).child(
                    GridRow::new()
                        .cell("name", "L3")
                        .expanded(true)
                        .child(GridRow::new().cell("name", "L4")),
                ),
            ),
        )
        .tree_mode(true);

    // Should have 4 nodes (all expanded)
    assert_eq!(grid.tree_cache.len(), 4);

    // Check depths
    assert_eq!(grid.tree_cache[0].depth, 0);
    assert_eq!(grid.tree_cache[1].depth, 1);
    assert_eq!(grid.tree_cache[2].depth, 2);
    assert_eq!(grid.tree_cache[3].depth, 3);
}

#[test]
fn test_toggle_expand_out_of_bounds() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(GridRow::new().cell("name", "Only"))
        .tree_mode(true);

    // Select out of bounds
    grid.selected_row = 999;

    // Should not panic
    grid.toggle_expand();
}
