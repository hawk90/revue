//! Grid display options

/// Grid display options
#[derive(Clone, Debug)]
pub struct GridOptions {
    /// Show header row
    pub show_header: bool,
    /// Show row numbers column
    pub show_row_numbers: bool,
    /// Enable multi-row selection
    pub multi_select: bool,
    /// Enable zebra striping (alternating row colors)
    pub zebra: bool,
    /// Use natural sorting for text (file2 < file10)
    pub use_natural_sort: bool,
    /// Enable virtual scrolling for large datasets
    pub virtual_scroll: bool,
    /// Row height in lines (for virtual scroll calculations)
    pub row_height: u16,
    /// Overscan rows (extra rows rendered above/below viewport for smooth scrolling)
    pub overscan: usize,
}

impl Default for GridOptions {
    fn default() -> Self {
        Self {
            show_header: true,
            show_row_numbers: false,
            multi_select: false,
            zebra: true,
            use_natural_sort: true,
            virtual_scroll: true,
            row_height: 1,
            overscan: 5,
        }
    }
}

impl GridOptions {
    /// Create new options with defaults
    pub fn new() -> Self {
        Self::default()
    }
}
