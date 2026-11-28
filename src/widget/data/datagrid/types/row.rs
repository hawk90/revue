//! Grid row and cell definitions

/// A row in the grid
#[derive(Clone, Debug)]
pub struct GridRow {
    /// Row data (key -> value)
    pub data: Vec<(String, String)>,
    /// Row is selected
    pub selected: bool,
    /// Row is expanded (for tree grids; child rows are not drawn in 3.x)
    #[deprecated(
        since = "3.17.0",
        note = "DataGrid does not draw child rows in 3.x; tree mode comes in 4.0 with row keys (#1539)"
    )]
    pub expanded: bool,
    /// Child rows (not drawn in 3.x; tree mode is planned for 4.0, #1539)
    #[deprecated(
        since = "3.17.0",
        note = "DataGrid does not draw child rows in 3.x; tree mode comes in 4.0 with row keys (#1539)"
    )]
    pub children: Vec<GridRow>,
}

#[allow(deprecated)]
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

    /// Add a child row (for tree grid; child rows are not drawn in 3.x)
    #[deprecated(
        since = "3.17.0",
        note = "DataGrid does not draw child rows in 3.x; tree mode comes in 4.0 with row keys (#1539)"
    )]
    pub fn child(mut self, row: GridRow) -> Self {
        self.children.push(row);
        self
    }

    /// Add multiple child rows
    #[deprecated(
        since = "3.17.0",
        note = "DataGrid does not draw child rows in 3.x; tree mode comes in 4.0 with row keys (#1539)"
    )]
    pub fn children(mut self, rows: Vec<GridRow>) -> Self {
        self.children.extend(rows);
        self
    }

    /// Set expanded state
    #[deprecated(
        since = "3.17.0",
        note = "DataGrid does not draw child rows in 3.x; tree mode comes in 4.0 with row keys (#1539)"
    )]
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Check if row has children
    #[deprecated(
        since = "3.17.0",
        note = "DataGrid does not draw child rows in 3.x; tree mode comes in 4.0 with row keys (#1539)"
    )]
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }
}

impl Default for GridRow {
    fn default() -> Self {
        Self::new()
    }
}
