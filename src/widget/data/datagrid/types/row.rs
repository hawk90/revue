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

// # KEEP HERE - Private tests for internal functionality
