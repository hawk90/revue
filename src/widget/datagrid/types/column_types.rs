//! Column type definitions

/// Column data type
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ColumnType {
    #[default]
    Text,
    Number,
    Date,
    Boolean,
    Custom,
}

/// Sort direction
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SortDirection {
    /// Sort ascending (A-Z, 0-9)
    Ascending,
    /// Sort descending (Z-A, 9-0)
    Descending,
}

impl SortDirection {
    /// Toggle between ascending and descending
    pub fn toggle(&self) -> Self {
        match self {
            SortDirection::Ascending => SortDirection::Descending,
            SortDirection::Descending => SortDirection::Ascending,
        }
    }

    /// Get the icon character for this sort direction
    pub fn icon(&self) -> char {
        match self {
            SortDirection::Ascending => '▲',
            SortDirection::Descending => '▼',
        }
    }
}

/// Text alignment
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Alignment {
    #[default]
    Left,
    Center,
    Right,
}
