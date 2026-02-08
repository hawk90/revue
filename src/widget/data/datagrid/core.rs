//! DataGrid core structure and builders

use super::editing::EditState;
use super::types::{
    AggregationType, FooterRow, GridColors, GridColumn, GridOptions, GridRow, SortDirection,
};
use crate::style::Color;
use crate::{impl_props_builders, impl_styled_view};

/// Tree node info for flattened display
#[derive(Clone, Debug)]
pub struct TreeNodeInfo {
    /// Original row index in tree (path from root)
    pub path: Vec<usize>,
    /// Nesting depth (0 = root level)
    pub depth: usize,
    /// Has child rows
    pub has_children: bool,
    /// Currently expanded
    pub is_expanded: bool,
    /// Is last child at this level (for tree line rendering)
    pub is_last_child: bool,
}

/// Cell position for rendering
pub(super) struct CellPos {
    pub x: u16,
    pub y: u16,
    pub width: u16,
}

/// Cell state for rendering
pub(super) struct CellState {
    pub row_bg: Color,
    pub is_selected: bool,
    pub is_editing: bool,
}

/// Row rendering parameters
pub(super) struct RowRenderParams<'a> {
    pub visible_cols: &'a [(usize, &'a GridColumn)],
    pub widths: &'a [u16],
    pub area_x: u16,
    pub start_y: u16,
    pub row_num_width: u16,
    pub visible_height: usize,
}

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
    /// User-set column widths (overrides auto calculation)
    pub column_widths: Vec<u16>,
    /// Callback when column is resized
    pub on_column_resize: Option<Box<dyn FnMut(usize, u16)>>,

    // ─────────────────────────────────────────────────────────────────────────
    // Column Reorder State
    // ─────────────────────────────────────────────────────────────────────────
    /// Column being dragged (index)
    pub dragging_col: Option<usize>,
    /// Drop target column (index)
    pub drop_target_col: Option<usize>,
    /// Column display order (maps display index to actual column index)
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

    /// Widget props for CSS integration
    pub props: crate::widget::traits::WidgetProps,
}

impl DataGrid {
    /// Create a new data grid
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            rows: Vec::new(),
            sort_column: None,
            sort_direction: SortDirection::Ascending,
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

    /// Recompute the filtered rows cache (called on mutation)
    pub fn recompute_cache(&mut self) {
        self.filtered_cache = self.compute_filtered_indices();
    }

    /// Add a column
    pub fn column(mut self, col: GridColumn) -> Self {
        self.columns.push(col);
        self
    }

    /// Add columns
    pub fn columns(mut self, cols: Vec<GridColumn>) -> Self {
        self.columns.extend(cols);
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
                if let Some(col) = self.columns.get(i) {
                    row.data.push((col.key.clone(), value));
                }
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

    /// Set callback for when a column is resized
    pub fn on_column_resize<F>(mut self, callback: F) -> Self
    where
        F: FnMut(usize, u16) + 'static,
    {
        self.on_column_resize = Some(Box::new(callback));
        self
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Column Reorder API
    // ─────────────────────────────────────────────────────────────────────────

    /// Enable or disable column reordering via drag and drop
    pub fn reorderable(mut self, enable: bool) -> Self {
        self.reorderable = enable;
        self
    }

    /// Set callback for when columns are reordered
    pub fn on_column_reorder<F>(mut self, callback: F) -> Self
    where
        F: FnMut(usize, usize) + 'static,
    {
        self.on_column_reorder = Some(Box::new(callback));
        self
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Column Freeze API
    // ─────────────────────────────────────────────────────────────────────────

    /// Freeze N columns on the left (they stay visible during horizontal scroll)
    pub fn freeze_columns_left(mut self, count: usize) -> Self {
        self.frozen_left = count;
        self
    }

    /// Freeze N columns on the right (they stay visible during horizontal scroll)
    pub fn freeze_columns_right(mut self, count: usize) -> Self {
        self.frozen_right = count;
        self
    }

    /// Compute aggregation value for a column
    pub(super) fn compute_aggregation(
        &self,
        column_key: &str,
        agg_type: AggregationType,
    ) -> Option<f64> {
        let values: Vec<f64> = self
            .filtered_indices()
            .iter()
            .filter_map(|&idx| {
                self.rows
                    .get(idx)
                    .and_then(|r| r.get(column_key))
                    .and_then(|v| v.parse::<f64>().ok())
            })
            .collect();

        if values.is_empty() {
            return None;
        }

        Some(match agg_type {
            AggregationType::Sum => values.iter().sum(),
            AggregationType::Average => values.iter().sum::<f64>() / values.len() as f64,
            AggregationType::Count => values.len() as f64,
            AggregationType::Min => values.iter().cloned().fold(f64::INFINITY, f64::min),
            AggregationType::Max => values.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
        })
    }

    /// Get computed footer values for rendering
    #[allow(dead_code)] // Used for footer rendering
    pub(super) fn get_footer_values(&self, footer: &FooterRow) -> Vec<(String, String)> {
        let mut values = Vec::new();

        for agg in &footer.aggregations {
            let label = agg
                .label
                .clone()
                .unwrap_or_else(|| agg.agg_type.label().to_string());

            let value = self
                .compute_aggregation(&agg.column_key, agg.agg_type)
                .map(|v| {
                    if v.fract() == 0.0 {
                        format!("{:.0}", v)
                    } else {
                        format!("{:.2}", v)
                    }
                })
                .unwrap_or_else(|| "—".to_string());

            values.push((agg.column_key.clone(), format!("{}: {}", label, value)));
        }

        values
    }

    /// Compute filtered row indices (internal)
    pub(super) fn compute_filtered_indices(&self) -> Vec<usize> {
        if self.filter.is_empty() {
            (0..self.rows.len()).collect()
        } else {
            self.rows
                .iter()
                .enumerate()
                .filter(|(_, row)| match self.filter_column {
                    Some(col_idx) => {
                        if let Some(col) = self.columns.get(col_idx) {
                            row.get(&col.key)
                                .map(|v| v.to_lowercase().contains(&self.filter))
                                .unwrap_or(false)
                        } else {
                            false
                        }
                    }
                    None => row
                        .data
                        .iter()
                        .any(|(_, v)| v.to_lowercase().contains(&self.filter)),
                })
                .map(|(i, _)| i)
                .collect()
        }
    }

    /// Get cached filtered row indices (zero-cost, no allocation)
    #[inline]
    pub fn filtered_indices(&self) -> &[usize] {
        &self.filtered_cache
    }

    /// Get filtered rows count (uses cache)
    pub fn filtered_count(&self) -> usize {
        self.filtered_indices().len()
    }

    /// Get filtered rows (uses cached indices)
    /// Note: For large datasets, prefer using filtered_indices() with index-based access
    #[allow(dead_code)]
    pub fn filtered_rows(&self) -> Vec<&GridRow> {
        self.filtered_indices()
            .iter()
            .filter_map(|&i| self.rows.get(i))
            .collect()
    }
}

impl Default for DataGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl_styled_view!(DataGrid);
impl_props_builders!(DataGrid);

#[cfg(test)]
mod tests {
    use super::super::types::GridColors;
    use super::*;

    // =========================================================================
    // Constructor tests
    // =========================================================================

    #[test]
    fn test_datagrid_new() {
        let grid = DataGrid::new();
        assert!(grid.columns.is_empty());
        assert!(grid.rows.is_empty());
        assert!(grid.filtered_cache.is_empty());
        assert_eq!(grid.selected_row, 0);
        assert_eq!(grid.selected_col, 0);
        assert_eq!(grid.scroll_row, 0);
        assert!(!grid.edit_state.active);
        assert!(grid.footer_rows.is_empty());
        assert!(!grid.show_footer);
        assert!(!grid.tree_mode);
        assert_eq!(grid.frozen_left, 0);
        assert_eq!(grid.frozen_right, 0);
        assert!(!grid.reorderable);
    }

    #[test]
    fn test_datagrid_default() {
        let grid = DataGrid::default();
        assert!(grid.columns.is_empty());
        assert!(grid.rows.is_empty());
    }

    // =========================================================================
    // Builder tests - colors
    // =========================================================================

    #[test]
    fn test_datagrid_colors() {
        let colors = GridColors::new();
        let header_bg = colors.header_bg;
        let grid = DataGrid::new().colors(colors);
        assert_eq!(grid.colors.header_bg, header_bg);
    }

    #[test]
    fn test_datagrid_colors_mut() {
        let mut grid = DataGrid::new();
        let colors = grid.colors_mut();
        colors.header_bg = crate::style::Color::RED;
        assert_eq!(grid.colors.header_bg, crate::style::Color::RED);
    }

    // =========================================================================
    // Builder tests - options
    // =========================================================================

    #[test]
    fn test_datagrid_options() {
        let grid = DataGrid::new().zebra(false);
        assert!(!grid.options.zebra);
    }

    #[test]
    fn test_datagrid_options_mut() {
        let mut grid = DataGrid::new();
        let opts = grid.options_mut();
        opts.zebra = false;
        assert!(!grid.options.zebra);
    }

    // =========================================================================
    // Builder tests - columns
    // =========================================================================

    #[test]
    fn test_datagrid_column_single() {
        let grid = DataGrid::new().column(GridColumn::new("a", "A"));

        assert_eq!(grid.columns.len(), 1);
        assert_eq!(grid.columns[0].key, "a");
    }

    #[test]
    fn test_datagrid_column_multiple() {
        let grid = DataGrid::new()
            .column(GridColumn::new("a", "A"))
            .column(GridColumn::new("b", "B"))
            .column(GridColumn::new("c", "C"));

        assert_eq!(grid.columns.len(), 3);
    }

    #[test]
    fn test_datagrid_columns_vec() {
        let cols = vec![GridColumn::new("x", "X"), GridColumn::new("y", "Y")];
        let grid = DataGrid::new().columns(cols);

        assert_eq!(grid.columns.len(), 2);
    }

    // =========================================================================
    // Builder tests - rows
    // =========================================================================

    #[test]
    fn test_datagrid_row_single() {
        let grid = DataGrid::new()
            .column(GridColumn::new("name", "Name"))
            .row(GridRow::new().cell("name", "Alice"));

        assert_eq!(grid.rows.len(), 1);
        assert_eq!(grid.rows[0].get("name"), Some("Alice"));
    }

    #[test]
    fn test_datagrid_row_multiple() {
        let grid = DataGrid::new()
            .column(GridColumn::new("a", "A"))
            .row(GridRow::new().cell("a", "1"))
            .row(GridRow::new().cell("a", "2"))
            .row(GridRow::new().cell("a", "3"));

        assert_eq!(grid.rows.len(), 3);
    }

    #[test]
    fn test_datagrid_rows_vec() {
        let rows = vec![GridRow::new().cell("x", "a"), GridRow::new().cell("x", "b")];
        let grid = DataGrid::new().column(GridColumn::new("x", "X")).rows(rows);

        assert_eq!(grid.rows.len(), 2);
    }

    #[test]
    fn test_datagrid_data_2d() {
        let data = vec![
            vec![String::from("Alice"), String::from("25")],
            vec![String::from("Bob"), String::from("30")],
        ];
        let grid = DataGrid::new()
            .column(GridColumn::new("name", "Name"))
            .column(GridColumn::new("age", "Age"))
            .data(data);

        assert_eq!(grid.rows.len(), 2);
        assert_eq!(grid.rows[0].get("name"), Some("Alice"));
        assert_eq!(grid.rows[0].get("age"), Some("25"));
    }

    // =========================================================================
    // Builder tests - display options
    // =========================================================================

    #[test]
    fn test_datagrid_header_true() {
        let grid = DataGrid::new().header(true);
        assert!(grid.options.show_header);
    }

    #[test]
    fn test_datagrid_header_false() {
        let grid = DataGrid::new().header(false);
        assert!(!grid.options.show_header);
    }

    #[test]
    fn test_datagrid_row_numbers_true() {
        let grid = DataGrid::new().row_numbers(true);
        assert!(grid.options.show_row_numbers);
    }

    #[test]
    fn test_datagrid_row_numbers_false() {
        let grid = DataGrid::new().row_numbers(false);
        assert!(!grid.options.show_row_numbers);
    }

    #[test]
    fn test_datagrid_zebra_true() {
        let grid = DataGrid::new().zebra(true);
        assert!(grid.options.zebra);
    }

    #[test]
    fn test_datagrid_zebra_false() {
        let grid = DataGrid::new().zebra(false);
        assert!(!grid.options.zebra);
    }

    #[test]
    fn test_datagrid_multi_select_true() {
        let grid = DataGrid::new().multi_select(true);
        assert!(grid.options.multi_select);
    }

    #[test]
    fn test_datagrid_multi_select_false() {
        let grid = DataGrid::new().multi_select(false);
        assert!(!grid.options.multi_select);
    }

    #[test]
    fn test_datagrid_natural_sort_true() {
        let grid = DataGrid::new().natural_sort(true);
        assert!(grid.options.use_natural_sort);
    }

    #[test]
    fn test_datagrid_natural_sort_false() {
        let grid = DataGrid::new().natural_sort(false);
        assert!(!grid.options.use_natural_sort);
    }

    #[test]
    fn test_datagrid_virtual_scroll_true() {
        let grid = DataGrid::new().virtual_scroll(true);
        assert!(grid.options.virtual_scroll);
    }

    #[test]
    fn test_datagrid_virtual_scroll_false() {
        let grid = DataGrid::new().virtual_scroll(false);
        assert!(!grid.options.virtual_scroll);
    }

    #[test]
    fn test_datagrid_row_height() {
        let grid = DataGrid::new().row_height(2);
        assert_eq!(grid.options.row_height, 2);
    }

    #[test]
    fn test_datagrid_row_height_minimum() {
        let grid = DataGrid::new().row_height(0);
        assert_eq!(grid.options.row_height, 1); // Clamped to 1
    }

    #[test]
    fn test_datagrid_overscan() {
        let grid = DataGrid::new().overscan(10);
        assert_eq!(grid.options.overscan, 10);
    }

    // =========================================================================
    // Column resize tests
    // =========================================================================

    #[test]
    fn test_datagrid_on_column_resize() {
        let grid = DataGrid::new().on_column_resize(|col, width| {
            assert_eq!(col, 0);
            assert_eq!(width, 20);
        });
        assert!(grid.on_column_resize.is_some());
    }

    // =========================================================================
    // Column reorder tests
    // =========================================================================

    #[test]
    fn test_datagrid_reorderable_true() {
        let grid = DataGrid::new().reorderable(true);
        assert!(grid.reorderable);
    }

    #[test]
    fn test_datagrid_reorderable_false() {
        let grid = DataGrid::new().reorderable(false);
        assert!(!grid.reorderable);
    }

    #[test]
    fn test_datagrid_on_column_reorder() {
        let grid = DataGrid::new().on_column_reorder(|from, to| {
            assert_eq!(from, 0);
            assert_eq!(to, 1);
        });
        assert!(grid.on_column_reorder.is_some());
    }

    // =========================================================================
    // Column freeze tests
    // =========================================================================

    #[test]
    fn test_datagrid_freeze_columns_left() {
        let grid = DataGrid::new().freeze_columns_left(2);
        assert_eq!(grid.frozen_left, 2);
    }

    #[test]
    fn test_datagrid_freeze_columns_right() {
        let grid = DataGrid::new().freeze_columns_right(1);
        assert_eq!(grid.frozen_right, 1);
    }

    #[test]
    fn test_datagrid_freeze_both_sides() {
        let grid = DataGrid::new()
            .freeze_columns_left(1)
            .freeze_columns_right(1);
        assert_eq!(grid.frozen_left, 1);
        assert_eq!(grid.frozen_right, 1);
    }

    // =========================================================================
    // Filtered cache tests
    // =========================================================================

    #[test]
    fn test_datagrid_recompute_cache_initializes() {
        let mut grid = DataGrid::new()
            .column(GridColumn::new("a", "A"))
            .row(GridRow::new().cell("a", "1"))
            .row(GridRow::new().cell("a", "2"));

        grid.recompute_cache();
        assert_eq!(grid.filtered_cache, vec![0, 1]);
    }

    #[test]
    fn test_datagrid_filtered_indices() {
        let grid = DataGrid::new()
            .column(GridColumn::new("a", "A"))
            .row(GridRow::new().cell("a", "1"))
            .row(GridRow::new().cell("a", "2"));

        assert_eq!(grid.filtered_indices(), &[0, 1]);
    }

    #[test]
    fn test_datagrid_filtered_count() {
        let grid = DataGrid::new()
            .column(GridColumn::new("a", "A"))
            .row(GridRow::new().cell("a", "1"))
            .row(GridRow::new().cell("a", "2"))
            .row(GridRow::new().cell("a", "3"));

        assert_eq!(grid.filtered_count(), 3);
    }

    #[test]
    fn test_datagrid_filtered_rows() {
        let grid = DataGrid::new()
            .column(GridColumn::new("a", "A"))
            .row(GridRow::new().cell("a", "x"))
            .row(GridRow::new().cell("a", "y"));

        let rows = grid.filtered_rows();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].get("a"), Some("x"));
        assert_eq!(rows[1].get("a"), Some("y"));
    }

    // =========================================================================
    // Builder chain tests
    // =========================================================================

    #[test]
    fn test_datagrid_full_builder_chain() {
        let grid = DataGrid::new()
            .column(GridColumn::new("name", "Name"))
            .row(GridRow::new().cell("name", "Test"))
            .header(true)
            .row_numbers(true)
            .zebra(true)
            .multi_select(false)
            .row_height(1)
            .overscan(5)
            .freeze_columns_left(1)
            .reorderable(false);

        assert_eq!(grid.columns.len(), 1);
        assert_eq!(grid.rows.len(), 1);
        assert!(grid.options.show_header);
        assert!(grid.options.show_row_numbers);
        assert!(grid.options.zebra);
        assert_eq!(grid.frozen_left, 1);
    }

    // =========================================================================
    // Public field access tests
    // =========================================================================

    #[test]
    fn test_datagrid_public_fields_accessible() {
        let mut grid = DataGrid::new();

        grid.selected_row = 5;
        grid.selected_col = 2;
        grid.scroll_row = 1;

        assert_eq!(grid.selected_row, 5);
        assert_eq!(grid.selected_col, 2);
        assert_eq!(grid.scroll_row, 1);
    }

    #[test]
    fn test_datagrid_sort_fields() {
        let mut grid = DataGrid::new();

        grid.sort_column = Some(1);
        grid.sort_direction = SortDirection::Descending;

        assert_eq!(grid.sort_column, Some(1));
        assert_eq!(grid.sort_direction, SortDirection::Descending);
    }

    #[test]
    fn test_datagrid_edit_state_fields() {
        let grid = DataGrid::new();

        assert!(!grid.edit_state.active);
        assert_eq!(grid.edit_state.row, 0);
        assert_eq!(grid.edit_state.col, 0);
        assert!(grid.edit_state.buffer.is_empty());
        assert_eq!(grid.edit_state.cursor, 0);
    }

    // =========================================================================
    // Edge case tests
    // =========================================================================

    #[test]
    fn test_datagrid_empty_with_options() {
        let grid = DataGrid::new().header(false).row_numbers(true).zebra(false);

        assert!(grid.columns.is_empty());
        assert!(grid.rows.is_empty());
        assert!(!grid.options.show_header);
        assert!(grid.options.show_row_numbers);
    }

    #[test]
    fn test_datagrid_multiple_freeze_operations() {
        let grid = DataGrid::new()
            .freeze_columns_left(1)
            .freeze_columns_left(2)
            .freeze_columns_right(1)
            .freeze_columns_right(2);

        assert_eq!(grid.frozen_left, 2);
        assert_eq!(grid.frozen_right, 2);
    }
}
