//! Grid row and cell definitions

/// A row in the grid
#[derive(Clone, Debug)]
pub struct GridRow {
    /// Row data (key -> value)
    pub data: Vec<(String, String)>,
    /// Row is selected
    pub selected: bool,
    /// Row is expanded (for tree grids)
    pub expanded: bool,
    /// Child rows
    pub children: Vec<GridRow>,
}

impl GridRow {
    /// Create a new row
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            selected: false,
            expanded: false,
            children: Vec::new(),
        }
    }

    /// Add cell data
    pub fn cell(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.data.push((key.into(), value.into()));
        self
    }

    /// Get cell value by key
    pub fn get(&self, key: &str) -> Option<&str> {
        self.data
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }

    /// Add a child row (for tree grid)
    pub fn child(mut self, row: GridRow) -> Self {
        self.children.push(row);
        self
    }

    /// Add multiple child rows
    pub fn children(mut self, rows: Vec<GridRow>) -> Self {
        self.children.extend(rows);
        self
    }

    /// Set expanded state
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Check if row has children
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }
}

impl Default for GridRow {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Constructor tests
    // =========================================================================

    #[test]
    fn test_grid_row_new() {
        let row = GridRow::new();
        assert!(row.data.is_empty());
        assert!(!row.selected);
        assert!(!row.expanded);
        assert!(row.children.is_empty());
    }

    #[test]
    fn test_grid_row_default() {
        let row = GridRow::default();
        assert!(row.data.is_empty());
        assert!(!row.selected);
        assert!(!row.expanded);
        assert!(row.children.is_empty());
    }

    // =========================================================================
    // Builder method tests - cell
    // =========================================================================

    #[test]
    fn test_grid_row_cell_single() {
        let row = GridRow::new().cell("name", "Alice");
        assert_eq!(row.data.len(), 1);
        assert_eq!(row.data[0], (String::from("name"), String::from("Alice")));
    }

    #[test]
    fn test_grid_row_cell_multiple_chained() {
        let row = GridRow::new()
            .cell("id", "1")
            .cell("name", "Bob")
            .cell("age", "30");

        assert_eq!(row.data.len(), 3);
        assert_eq!(row.data[0], (String::from("id"), String::from("1")));
        assert_eq!(row.data[1], (String::from("name"), String::from("Bob")));
        assert_eq!(row.data[2], (String::from("age"), String::from("30")));
    }

    #[test]
    fn test_grid_row_cell_with_string() {
        let row = GridRow::new().cell(String::from("key"), String::from("value"));
        assert_eq!(row.data.len(), 1);
        assert_eq!(row.data[0].0, "key");
        assert_eq!(row.data[0].1, "value");
    }

    #[test]
    fn test_grid_row_cell_with_numbers() {
        let row = GridRow::new().cell("count", "42").cell("price", "100");

        assert_eq!(row.data.len(), 2);
        assert_eq!(row.data[0].1, "42");
        assert_eq!(row.data[1].1, "100");
    }

    // =========================================================================
    // Getter method tests - get
    // =========================================================================

    #[test]
    fn test_grid_row_get_existing_key() {
        let row = GridRow::new().cell("name", "Alice").cell("age", "30");

        assert_eq!(row.get("name"), Some("Alice"));
        assert_eq!(row.get("age"), Some("30"));
    }

    #[test]
    fn test_grid_row_get_nonexistent_key() {
        let row = GridRow::new().cell("name", "Alice");
        assert_eq!(row.get("nonexistent"), None);
    }

    #[test]
    fn test_grid_row_get_empty_key() {
        let row = GridRow::new().cell("", "value");
        assert_eq!(row.get(""), Some("value"));
    }

    #[test]
    fn test_grid_row_get_from_empty_row() {
        let row = GridRow::new();
        assert_eq!(row.get("anything"), None);
    }

    #[test]
    fn test_grid_row_get_duplicate_keys_returns_first() {
        let row = GridRow::new().cell("key", "first").cell("key", "second");

        assert_eq!(row.get("key"), Some("first"));
    }

    // =========================================================================
    // Builder method tests - child
    // =========================================================================

    #[test]
    fn test_grid_row_child_single() {
        let child = GridRow::new().cell("name", "Child");
        let row = GridRow::new().child(child);

        assert_eq!(row.children.len(), 1);
        assert_eq!(row.children[0].data[0].1, "Child");
    }

    #[test]
    fn test_grid_row_child_multiple_chained() {
        let row = GridRow::new()
            .child(GridRow::new().cell("name", "Child1"))
            .child(GridRow::new().cell("name", "Child2"))
            .child(GridRow::new().cell("name", "Child3"));

        assert_eq!(row.children.len(), 3);
    }

    #[test]
    fn test_grid_row_child_nested() {
        let nested = GridRow::new()
            .cell("name", "Grandchild")
            .child(GridRow::new().cell("name", "GreatGrandchild"));

        let row = GridRow::new().cell("name", "Parent").child(nested);

        assert_eq!(row.children.len(), 1);
        assert_eq!(row.children[0].data[0].1, "Grandchild");
        assert_eq!(row.children[0].children.len(), 1);
        assert_eq!(row.children[0].children[0].data[0].1, "GreatGrandchild");
    }

    // =========================================================================
    // Builder method tests - children
    // =========================================================================

    #[test]
    fn test_grid_row_children_empty_vec() {
        let row = GridRow::new().children(vec![]);
        assert!(row.children.is_empty());
    }

    #[test]
    fn test_grid_row_children_single() {
        let children = vec![GridRow::new().cell("name", "Only")];
        let row = GridRow::new().children(children);

        assert_eq!(row.children.len(), 1);
        assert_eq!(row.children[0].data[0].1, "Only");
    }

    #[test]
    fn test_grid_row_children_multiple() {
        let children = vec![
            GridRow::new().cell("id", "1"),
            GridRow::new().cell("id", "2"),
            GridRow::new().cell("id", "3"),
        ];
        let row = GridRow::new().children(children);

        assert_eq!(row.children.len(), 3);
    }

    #[test]
    fn test_grid_row_children_adds_to_existing() {
        let row = GridRow::new()
            .child(GridRow::new().cell("id", "1"))
            .children(vec![
                GridRow::new().cell("id", "2"),
                GridRow::new().cell("id", "3"),
            ]);

        assert_eq!(row.children.len(), 3);
    }

    // =========================================================================
    // Builder method tests - expanded
    // =========================================================================

    #[test]
    fn test_grid_row_expanded_true() {
        let row = GridRow::new().expanded(true);
        assert!(row.expanded);
    }

    #[test]
    fn test_grid_row_expanded_false() {
        let row = GridRow::new().expanded(false);
        assert!(!row.expanded);
    }

    #[test]
    fn test_grid_row_expanded_default_is_false() {
        let row = GridRow::new();
        assert!(!row.expanded);
    }

    // =========================================================================
    // Getter method tests - has_children
    // =========================================================================

    #[test]
    fn test_grid_row_has_children_true() {
        let row = GridRow::new().child(GridRow::new());
        assert!(row.has_children());
    }

    #[test]
    fn test_grid_row_has_children_false() {
        let row = GridRow::new();
        assert!(!row.has_children());
    }

    #[test]
    fn test_grid_row_has_children_empty_vec() {
        let row = GridRow::new().children(vec![]);
        assert!(!row.has_children());
    }

    #[test]
    fn test_grid_row_has_children_multiple() {
        let row = GridRow::new().child(GridRow::new()).child(GridRow::new());
        assert!(row.has_children());
        assert_eq!(row.children.len(), 2);
    }

    // =========================================================================
    // Clone tests
    // =========================================================================

    #[test]
    fn test_grid_row_clone_simple() {
        let row1 = GridRow::new().cell("name", "Test");
        let row2 = row1.clone();

        assert_eq!(row1.data.len(), row2.data.len());
        assert_eq!(row1.data[0], row2.data[0]);
    }

    #[test]
    fn test_grid_row_clone_with_children() {
        let row1 = GridRow::new()
            .cell("name", "Parent")
            .child(GridRow::new().cell("name", "Child"));
        let row2 = row1.clone();

        assert_eq!(row1.data.len(), row2.data.len());
        assert_eq!(row1.children.len(), row2.children.len());
        assert_eq!(row1.children[0].data[0], row2.children[0].data[0]);
    }

    #[test]
    fn test_grid_row_clone_with_expanded() {
        let row1 = GridRow::new().expanded(true);
        let row2 = row1.clone();

        assert_eq!(row1.expanded, row2.expanded);
        assert!(row2.expanded);
    }

    #[test]
    fn test_grid_row_clone_independence() {
        let row1 = GridRow::new()
            .cell("key", "value")
            .child(GridRow::new().cell("child", "data"));
        let mut row2 = row1.clone();

        row2.data[0].1 = String::from("modified");
        row2.children[0].data[0].1 = String::from("changed");

        assert_eq!(row1.data[0].1, "value");
        assert_eq!(row1.children[0].data[0].1, "data");
        assert_eq!(row2.data[0].1, "modified");
        assert_eq!(row2.children[0].data[0].1, "changed");
    }

    // =========================================================================
    // Public field access tests
    // =========================================================================

    #[test]
    fn test_grid_row_public_fields_accessible() {
        let mut row = GridRow::new();
        row.data.push(("key".to_string(), "value".to_string()));
        row.selected = true;
        row.expanded = true;

        assert_eq!(row.data.len(), 1);
        assert!(row.selected);
        assert!(row.expanded);
    }

    #[test]
    fn test_grid_row_selected_field() {
        let mut row = GridRow::new();
        assert!(!row.selected);

        row.selected = true;
        assert!(row.selected);

        row.selected = false;
        assert!(!row.selected);
    }

    // =========================================================================
    // Complex scenarios
    // =========================================================================

    #[test]
    fn test_grid_row_full_builder_chain() {
        let row = GridRow::new()
            .cell("id", "1")
            .cell("name", "Alice")
            .cell("age", "30")
            .expanded(true)
            .child(GridRow::new().cell("id", "2").cell("name", "Bob"))
            .children(vec![
                GridRow::new().cell("id", "3"),
                GridRow::new().cell("id", "4"),
            ]);

        assert_eq!(row.data.len(), 3);
        assert!(row.expanded);
        assert_eq!(row.children.len(), 3);
        assert_eq!(row.get("name"), Some("Alice"));
        assert_eq!(row.children[0].get("name"), Some("Bob"));
    }

    #[test]
    fn test_grid_row_deep_hierarchy() {
        let row = GridRow::new().cell("level", "0").child(
            GridRow::new().cell("level", "1").child(
                GridRow::new()
                    .cell("level", "2")
                    .child(GridRow::new().cell("level", "3")),
            ),
        );

        assert!(row.has_children());
        assert!(row.children[0].has_children());
        assert!(row.children[0].children[0].has_children());
        assert!(!row.children[0].children[0].children[0].has_children());
    }

    #[test]
    fn test_grid_row_many_cells() {
        let mut row = GridRow::new();
        for i in 0..10 {
            row = row.cell(&format!("col{}", i), &format!("val{}", i));
        }

        assert_eq!(row.data.len(), 10);
        assert_eq!(row.get("col0"), Some("val0"));
        assert_eq!(row.get("col9"), Some("val9"));
    }

    #[test]
    fn test_grid_row_special_characters_in_keys() {
        let row = GridRow::new()
            .cell("key-with-dash", "value1")
            .cell("key_with_underscore", "value2")
            .cell("key.with.dots", "value3")
            .cell("key/with/slashes", "value4");

        assert_eq!(row.get("key-with-dash"), Some("value1"));
        assert_eq!(row.get("key_with_underscore"), Some("value2"));
        assert_eq!(row.get("key.with.dots"), Some("value3"));
        assert_eq!(row.get("key/with/slashes"), Some("value4"));
    }

    #[test]
    fn test_grid_row_unicode_values() {
        let row = GridRow::new()
            .cell("emoji", "😀")
            .cell("chinese", "中文")
            .cell("korean", "한글")
            .cell("arabic", "العربية");

        assert_eq!(row.get("emoji"), Some("😀"));
        assert_eq!(row.get("chinese"), Some("中文"));
        assert_eq!(row.get("korean"), Some("한글"));
        assert_eq!(row.get("arabic"), Some("العربية"));
    }

    #[test]
    fn test_grid_row_empty_string_values() {
        let row = GridRow::new().cell("empty", "").cell("spaces", "   ");

        assert_eq!(row.get("empty"), Some(""));
        assert_eq!(row.get("spaces"), Some("   "));
    }

    #[test]
    fn test_grid_row_debug_trait() {
        let row = GridRow::new()
            .cell("id", "1")
            .child(GridRow::new().cell("id", "2"));

        let debug_str = format!("{:?}", row);
        assert!(debug_str.contains("GridRow"));
    }

    // =========================================================================
    // Integration tests
    // =========================================================================

    #[test]
    fn test_grid_row_tree_structure() {
        let tree = GridRow::new()
            .cell("name", "Root")
            .expanded(true)
            .children(vec![
                GridRow::new()
                    .cell("name", "Branch1")
                    .child(GridRow::new().cell("name", "Leaf1.1"))
                    .child(GridRow::new().cell("name", "Leaf1.2")),
                GridRow::new()
                    .cell("name", "Branch2")
                    .expanded(true)
                    .child(GridRow::new().cell("name", "Leaf2.1")),
            ]);

        assert_eq!(tree.get("name"), Some("Root"));
        assert!(tree.has_children());
        assert_eq!(tree.children.len(), 2);
        assert!(tree.children[0].has_children());
        assert_eq!(tree.children[0].children.len(), 2);
        assert!(!tree.children[0].children[0].has_children());
        assert!(tree.children[1].has_children());
        assert!(tree.children[1].expanded);
    }

    #[test]
    fn test_grid_row_find_child_by_key() {
        let row = GridRow::new().children(vec![
            GridRow::new().cell("id", "1").cell("name", "Alice"),
            GridRow::new().cell("id", "2").cell("name", "Bob"),
            GridRow::new().cell("id", "3").cell("name", "Charlie"),
        ]);

        // Can access children by index
        assert_eq!(row.children[0].get("name"), Some("Alice"));
        assert_eq!(row.children[1].get("name"), Some("Bob"));
        assert_eq!(row.children[2].get("name"), Some("Charlie"));
    }

    #[test]
    fn test_grid_row_with_selected_children() {
        let row = GridRow::new().cell("name", "Parent").children(vec![
            {
                let mut r = GridRow::new().cell("name", "Child1");
                r.selected = true;
                r
            },
            {
                let mut r = GridRow::new().cell("name", "Child2");
                r.selected = false;
                r
            },
        ]);

        assert!(!row.selected);
        assert!(row.children[0].selected);
        assert!(!row.children[1].selected);
    }

    #[test]
    fn test_grid_row_mixed_expansion_states() {
        let row = GridRow::new().expanded(true).children(vec![
            GridRow::new().expanded(true).child(GridRow::new()),
            GridRow::new().expanded(false).child(GridRow::new()),
            GridRow::new(), // leaf
        ]);

        assert!(row.expanded);
        assert!(row.children[0].expanded);
        assert!(!row.children[1].expanded);
        assert!(!row.children[2].expanded);
    }
}
