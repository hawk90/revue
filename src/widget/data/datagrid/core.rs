//! DataGrid core structure and builders

use crate::widget::data::datagrid::input::editing::EditState;
use crate::widget::data::datagrid::rows::tree::TreeNodeInfo;
use crate::widget::data::datagrid::types::{
    FooterRow, GridColors, GridColumn, GridOptions, GridRow, SortDirection,
};
use crate::{impl_props_builders, impl_styled_view};

/// DataGrid widget
pub struct DataGrid {
    // ─────────────────────────────────────────────────────────────────────────
    // Data
    // ─────────────────────────────────────────────────────────────────────────
    /// Columns
    pub columns: Vec<GridColumn>,
    /// Rows
    pub rows: Vec<GridRow>,

    // ─────────────────────────────────────────────────────────────────────────
    // Sorting & Filtering
    // ─────────────────────────────────────────────────────────────────────────
    /// Current sort column
    pub sort_column: Option<usize>,
    /// Sort direction
    pub sort_direction: SortDirection,
    /// Multi-column sort stack: (column_index, direction) in priority order
    pub sort_columns: Vec<(usize, SortDirection)>,
    /// Filter text
    pub filter: String,
    /// Filter column (None = all columns)
    pub filter_column: Option<usize>,
    /// Cached filtered row indices (eagerly computed on mutation)
    pub filtered_cache: Vec<usize>,

    // ─────────────────────────────────────────────────────────────────────────
    // Selection & Navigation
    // ─────────────────────────────────────────────────────────────────────────
    /// Selected row
    pub selected_row: usize,
    /// Selected column
    pub selected_col: usize,
    /// Scroll offset
    pub scroll_row: usize,
    /// Unused horizontal scroll offset (reserved for future use)
    pub _scroll_col: usize,

    // ─────────────────────────────────────────────────────────────────────────
    // Display Options & Colors (extracted structs)
    // ─────────────────────────────────────────────────────────────────────────
    /// Display options
    pub options: GridOptions,
    /// Color scheme
    pub colors: GridColors,

    // ─────────────────────────────────────────────────────────────────────────
    // Editing
    // ─────────────────────────────────────────────────────────────────────────
    /// Cell editing state
    pub edit_state: EditState,

    // ─────────────────────────────────────────────────────────────────────────
    // Column Resize State
    // ─────────────────────────────────────────────────────────────────────────
    /// Column being resized (index)
    pub resizing_col: Option<usize>,
    /// X position when resize started
    pub resize_start_x: u16,
    /// Column width when resize started
    pub resize_start_width: u16,
    /// Column resize handle being hovered
    pub hovered_resize: Option<usize>,
    /// User-set column widths (overrides auto calculation), indexed like
    /// `columns`
    pub column_widths: Vec<u16>,
    /// Callback when column is resized
    pub on_column_resize: Option<Box<dyn FnMut(usize, u16)>>,

    // ─────────────────────────────────────────────────────────────────────────
    // Column Reorder State
    // ─────────────────────────────────────────────────────────────────────────
    /// Column being dragged (index into `columns`)
    pub dragging_col: Option<usize>,
    /// Drop target: the display position (among visible columns) the dragged
    /// column will be inserted before; the visible column count means "at
    /// the end"
    pub drop_target_col: Option<usize>,
    /// Column display order (maps display index to actual column index).
    /// Empty means the order of `columns`; drag reorder permutes this and
    /// leaves `columns` in place.
    pub column_order: Vec<usize>,
    /// Whether columns can be reordered
    pub reorderable: bool,
    /// Callback when column is reordered
    pub on_column_reorder: Option<Box<dyn FnMut(usize, usize)>>,

    // ─────────────────────────────────────────────────────────────────────────
    // Column Freeze State
    // ─────────────────────────────────────────────────────────────────────────
    /// Number of columns frozen on the left
    pub frozen_left: usize,
    /// Number of columns frozen on the right
    pub frozen_right: usize,
    /// Horizontal scroll offset (column index)
    pub scroll_col: usize,

    // ─────────────────────────────────────────────────────────────────────────
    // Tree Grid State
    // ─────────────────────────────────────────────────────────────────────────
    /// Enable tree grid mode (hierarchical display)
    pub tree_mode: bool,
    /// Flattened tree cache for display
    pub tree_cache: Vec<TreeNodeInfo>,

    // ─────────────────────────────────────────────────────────────────────────
    // Export & Aggregation State
    // ─────────────────────────────────────────────────────────────────────────
    /// Footer rows for aggregation display
    pub footer_rows: Vec<FooterRow>,
    /// Show aggregation footer
    pub show_footer: bool,

    /// Last known viewport height (rows visible), updated during render
    pub(crate) last_viewport_height: std::cell::Cell<usize>,

    /// Widget props for CSS integration
    pub props: crate::widget::traits::WidgetProps,
}

// Test helpers
impl DataGrid {
    /// Get column width for testing
    #[doc(hidden)]
    pub fn get_column_widths(&self) -> &Vec<u16> {
        &self.column_widths
    }

    /// Set column width for testing
    #[doc(hidden)]
    pub fn set_column_widths(&mut self, widths: Vec<u16>) {
        self.column_widths = widths;
    }

    /// Get options for testing
    #[doc(hidden)]
    pub fn get_options(&self) -> &GridOptions {
        &self.options
    }

    /// Set show_row_numbers for testing
    #[doc(hidden)]
    pub fn set_row_numbers(&mut self, show: bool) {
        self.options.show_row_numbers = show;
    }
}

impl DataGrid {
    /// Create a new data grid
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            rows: Vec::new(),
            sort_column: None,
            sort_direction: SortDirection::Ascending,
            sort_columns: Vec::new(),
            filter: String::new(),
            filter_column: None,
            filtered_cache: Vec::new(),
            selected_row: 0,
            selected_col: 0,
            scroll_row: 0,
            _scroll_col: 0,
            options: GridOptions::default(),
            colors: GridColors::default(),
            edit_state: EditState::default(),
            // Resize state
            resizing_col: None,
            resize_start_x: 0,
            resize_start_width: 0,
            hovered_resize: None,
            column_widths: Vec::new(),
            on_column_resize: None,
            // Reorder state
            dragging_col: None,
            drop_target_col: None,
            column_order: Vec::new(),
            reorderable: false,
            on_column_reorder: None,
            // Freeze state
            frozen_left: 0,
            frozen_right: 0,
            scroll_col: 0,
            // Tree grid state
            tree_mode: false,
            tree_cache: Vec::new(),
            // Footer state
            footer_rows: Vec::new(),
            show_footer: false,
            last_viewport_height: std::cell::Cell::new(10),
            props: crate::widget::traits::WidgetProps::new(),
        }
    }

    /// Set color scheme
    pub fn colors(mut self, colors: GridColors) -> Self {
        self.colors = colors;
        self
    }

    /// Set display options
    pub fn options(mut self, options: GridOptions) -> Self {
        self.options = options;
        self
    }

    /// Get color scheme (for customization)
    pub fn colors_mut(&mut self) -> &mut GridColors {
        &mut self.colors
    }

    /// Get display options (for customization)
    pub fn options_mut(&mut self) -> &mut GridOptions {
        &mut self.options
    }

    /// Add a column. Values `data` gave before there was a column at its
    /// position become this column's (#1011).
    pub fn column(mut self, col: GridColumn) -> Self {
        let position = self.columns.len();
        claim_position(&mut self.rows, position, &col.key);
        self.columns.push(col);
        self.recompute_cache();
        self
    }

    /// Add columns
    pub fn columns(mut self, cols: Vec<GridColumn>) -> Self {
        for col in cols {
            self = self.column(col);
        }
        self
    }

    /// Add a row
    pub fn row(mut self, row: GridRow) -> Self {
        self.rows.push(row);
        self.recompute_cache();
        self
    }

    /// Add rows
    pub fn rows(mut self, rows: Vec<GridRow>) -> Self {
        self.rows.extend(rows);
        self.recompute_cache();
        self
    }

    /// Set data from 2D vector
    pub fn data(mut self, data: Vec<Vec<String>>) -> Self {
        for row_data in data {
            let mut row = GridRow::new();
            for (i, value) in row_data.into_iter().enumerate() {
                // A value past the columns waits for its column (#1011)
                let key = match self.columns.get(i) {
                    Some(col) => col.key.clone(),
                    None => unclaimed_key(i),
                };
                row.data.push((key, value));
            }
            self.rows.push(row);
        }
        self.recompute_cache();
        self
    }

    /// Show/hide header
    pub fn header(mut self, show: bool) -> Self {
        self.options.show_header = show;
        self
    }

    /// Show/hide row numbers
    pub fn row_numbers(mut self, show: bool) -> Self {
        self.options.show_row_numbers = show;
        self
    }

    /// Enable/disable zebra striping
    pub fn zebra(mut self, enable: bool) -> Self {
        self.options.zebra = enable;
        self
    }

    /// Enable multi-select
    pub fn multi_select(mut self, enable: bool) -> Self {
        self.options.multi_select = enable;
        self
    }

    /// Enable natural sorting for text columns (file2 before file10)
    pub fn natural_sort(mut self, enable: bool) -> Self {
        self.options.use_natural_sort = enable;
        self
    }

    /// Enable virtual scrolling for large datasets (default: true)
    ///
    /// When enabled, only visible rows plus overscan are rendered,
    /// allowing smooth performance with 100,000+ rows.
    pub fn virtual_scroll(mut self, enable: bool) -> Self {
        self.options.virtual_scroll = enable;
        self
    }

    /// Set row height in lines (default: 1)
    pub fn row_height(mut self, height: u16) -> Self {
        self.options.row_height = height.max(1);
        self
    }

    /// Set overscan rows (extra rows rendered above/below viewport)
    ///
    /// Higher values provide smoother scrolling but use more memory.
    /// Default is 5 rows.
    pub fn overscan(mut self, rows: usize) -> Self {
        self.options.overscan = rows;
        self
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Column Resize API
    // ─────────────────────────────────────────────────────────────────────────

    // ─────────────────────────────────────────────────────────────────────────
    // Column Reorder API
    // ─────────────────────────────────────────────────────────────────────────

    // ─────────────────────────────────────────────────────────────────────────
    // Column Freeze API
    // ─────────────────────────────────────────────────────────────────────────
}

impl Default for DataGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl_styled_view!(DataGrid);
impl_props_builders!(DataGrid);

/// The key `data` gives a value at `position` before there is a column
/// there. It starts with NUL, which no column key a caller writes does.
fn unclaimed_key(position: usize) -> String {
    format!("\u{0}{position}")
}

/// Give the values waiting at `position` (in `rows` and their children) to
/// the column `key` now there
fn claim_position(rows: &mut [GridRow], position: usize, key: &str) {
    let unclaimed = unclaimed_key(position);
    for row in rows {
        for (k, _) in &mut row.data {
            if *k == unclaimed {
                *k = key.to_string();
            }
        }
        claim_position(&mut row.children, position, key);
    }
}
