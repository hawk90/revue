//! DataGrid widget for advanced data display
//!
//! A feature-rich data grid with sorting, filtering, and cell editing.

mod types;

#[cfg(test)]
mod tests;

// Re-export all types
pub use types::*;

use super::traits::{RenderContext, View, WidgetProps};
use crate::event::Key;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::natural_cmp;

/// Cell position for rendering
struct CellPos {
    x: u16,
    y: u16,
    width: u16,
}

/// Cell state for rendering
struct CellState {
    row_bg: Color,
    is_selected: bool,
    is_editing: bool,
}

/// Row rendering parameters
struct RowRenderParams<'a> {
    visible_cols: &'a [(usize, &'a GridColumn)],
    widths: &'a [u16],
    area_x: u16,
    start_y: u16,
    row_num_width: u16,
    visible_height: usize,
}
use crate::{impl_props_builders, impl_styled_view};
use std::cmp::Ordering;

/// Cell editing state
#[derive(Clone, Debug, Default)]
struct EditState {
    /// Currently editing
    active: bool,
    /// Row being edited (actual index, not filtered)
    row: usize,
    /// Column being edited
    col: usize,
    /// Edit buffer
    buffer: String,
    /// Cursor position in buffer
    cursor: usize,
}

// ═══════════════════════════════════════════════════════════════════════════
// Tree Grid Types
// ═══════════════════════════════════════════════════════════════════════════

/// Tree node info for flattened display
#[derive(Clone, Debug)]
#[allow(dead_code)] // Fields used for tree rendering
struct TreeNodeInfo {
    /// Original row index in tree (path from root)
    path: Vec<usize>,
    /// Nesting depth (0 = root level)
    depth: usize,
    /// Has child rows
    has_children: bool,
    /// Currently expanded
    is_expanded: bool,
    /// Is last child at this level (for tree line rendering)
    is_last_child: bool,
}

// Types (GridColors, GridOptions, ColumnType, SortDirection, Alignment,
// GridColumn, GridRow, ExportFormat, ExportOptions, AggregationType,
// ColumnAggregation, FooterRow) are defined in types.rs

/// DataGrid widget
pub struct DataGrid {
    // ─────────────────────────────────────────────────────────────────────────
    // Data
    // ─────────────────────────────────────────────────────────────────────────
    /// Columns
    columns: Vec<GridColumn>,
    /// Rows
    rows: Vec<GridRow>,

    // ─────────────────────────────────────────────────────────────────────────
    // Sorting & Filtering
    // ─────────────────────────────────────────────────────────────────────────
    /// Current sort column
    sort_column: Option<usize>,
    /// Sort direction
    sort_direction: SortDirection,
    /// Filter text
    filter: String,
    /// Filter column (None = all columns)
    filter_column: Option<usize>,
    /// Cached filtered row indices (eagerly computed on mutation)
    filtered_cache: Vec<usize>,

    // ─────────────────────────────────────────────────────────────────────────
    // Selection & Navigation
    // ─────────────────────────────────────────────────────────────────────────
    /// Selected row
    selected_row: usize,
    /// Selected column
    selected_col: usize,
    /// Scroll offset
    scroll_row: usize,
    _scroll_col: usize,

    // ─────────────────────────────────────────────────────────────────────────
    // Display Options & Colors (extracted structs)
    // ─────────────────────────────────────────────────────────────────────────
    /// Display options
    options: GridOptions,
    /// Color scheme
    colors: GridColors,

    // ─────────────────────────────────────────────────────────────────────────
    // Editing
    // ─────────────────────────────────────────────────────────────────────────
    /// Cell editing state
    edit_state: EditState,

    // ─────────────────────────────────────────────────────────────────────────
    // Column Resize State
    // ─────────────────────────────────────────────────────────────────────────
    /// Column being resized (index)
    resizing_col: Option<usize>,
    /// X position when resize started
    resize_start_x: u16,
    /// Column width when resize started
    resize_start_width: u16,
    /// Column resize handle being hovered
    hovered_resize: Option<usize>,
    /// User-set column widths (overrides auto calculation)
    column_widths: Vec<u16>,
    /// Callback when column is resized
    on_column_resize: Option<Box<dyn FnMut(usize, u16)>>,

    // ─────────────────────────────────────────────────────────────────────────
    // Column Reorder State
    // ─────────────────────────────────────────────────────────────────────────
    /// Column being dragged (index)
    dragging_col: Option<usize>,
    /// Drop target column (index)
    drop_target_col: Option<usize>,
    /// Column display order (maps display index to actual column index)
    column_order: Vec<usize>,
    /// Whether columns can be reordered
    reorderable: bool,
    /// Callback when column is reordered
    on_column_reorder: Option<Box<dyn FnMut(usize, usize)>>,

    // ─────────────────────────────────────────────────────────────────────────
    // Column Freeze State
    // ─────────────────────────────────────────────────────────────────────────
    /// Number of columns frozen on the left
    frozen_left: usize,
    /// Number of columns frozen on the right
    frozen_right: usize,
    /// Horizontal scroll offset (column index)
    scroll_col: usize,

    // ─────────────────────────────────────────────────────────────────────────
    // Tree Grid State
    // ─────────────────────────────────────────────────────────────────────────
    /// Enable tree grid mode (hierarchical display)
    tree_mode: bool,
    /// Flattened tree cache for display
    tree_cache: Vec<TreeNodeInfo>,

    // ─────────────────────────────────────────────────────────────────────────
    // Export & Aggregation State
    // ─────────────────────────────────────────────────────────────────────────
    /// Footer rows for aggregation display
    footer_rows: Vec<FooterRow>,
    /// Show aggregation footer
    show_footer: bool,

    /// Widget props for CSS integration
    props: WidgetProps,
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
            props: WidgetProps::new(),
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
    fn recompute_cache(&mut self) {
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

    // ─────────────────────────────────────────────────────────────────────────
    // Tree Grid Methods
    // ─────────────────────────────────────────────────────────────────────────

    /// Enable tree grid mode for hierarchical data display
    pub fn tree_mode(mut self, enabled: bool) -> Self {
        self.tree_mode = enabled;
        if enabled {
            self.rebuild_tree_cache();
        }
        self
    }

    /// Check if tree mode is enabled
    pub fn is_tree_mode(&self) -> bool {
        self.tree_mode
    }

    /// Rebuild the flattened tree cache from rows
    fn rebuild_tree_cache(&mut self) {
        self.tree_cache.clear();
        self.flatten_rows(&self.rows.clone(), 0, &mut vec![], &[]);
    }

    /// Recursively flatten rows into tree_cache
    fn flatten_rows(
        &mut self,
        rows: &[GridRow],
        depth: usize,
        path: &mut Vec<usize>,
        parent_is_last: &[bool],
    ) {
        let count = rows.len();
        for (i, row) in rows.iter().enumerate() {
            let is_last = i == count - 1;
            path.push(i);

            self.tree_cache.push(TreeNodeInfo {
                path: path.clone(),
                depth,
                has_children: !row.children.is_empty(),
                is_expanded: row.expanded,
                is_last_child: is_last,
            });

            // Recurse into expanded children
            if row.expanded && !row.children.is_empty() {
                let mut new_parent_is_last = parent_is_last.to_vec();
                new_parent_is_last.push(is_last);
                self.flatten_rows(&row.children, depth + 1, path, &new_parent_is_last);
            }

            path.pop();
        }
    }

    /// Get row by path through tree
    #[allow(dead_code)] // Used for tree rendering
    fn get_row_by_path(&self, path: &[usize]) -> Option<&GridRow> {
        if path.is_empty() {
            return None;
        }

        let mut current_rows = &self.rows;
        let mut row: Option<&GridRow> = None;

        for &idx in path {
            if idx >= current_rows.len() {
                return None;
            }
            row = Some(&current_rows[idx]);
            current_rows = &current_rows[idx].children;
        }

        row
    }

    /// Get mutable row by path through tree
    fn get_row_by_path_mut(&mut self, path: &[usize]) -> Option<&mut GridRow> {
        if path.is_empty() {
            return None;
        }

        let mut current_rows = &mut self.rows;

        for (i, &idx) in path.iter().enumerate() {
            if idx >= current_rows.len() {
                return None;
            }
            if i == path.len() - 1 {
                return Some(&mut current_rows[idx]);
            }
            current_rows = &mut current_rows[idx].children;
        }

        None
    }

    /// Toggle expand/collapse of selected row in tree mode
    pub fn toggle_expand(&mut self) {
        if !self.tree_mode {
            return;
        }

        let visible_rows = if self.tree_mode {
            self.tree_cache.len()
        } else {
            self.filtered_count()
        };

        if self.selected_row >= visible_rows {
            return;
        }

        if let Some(node) = self.tree_cache.get(self.selected_row).cloned() {
            if node.has_children {
                if let Some(row) = self.get_row_by_path_mut(&node.path) {
                    row.expanded = !row.expanded;
                    self.rebuild_tree_cache();
                }
            }
        }
    }

    /// Expand selected row in tree mode
    pub fn expand(&mut self) {
        if !self.tree_mode {
            return;
        }

        if let Some(node) = self.tree_cache.get(self.selected_row).cloned() {
            if node.has_children && !node.is_expanded {
                if let Some(row) = self.get_row_by_path_mut(&node.path) {
                    row.expanded = true;
                    self.rebuild_tree_cache();
                }
            }
        }
    }

    /// Collapse selected row in tree mode
    pub fn collapse(&mut self) {
        if !self.tree_mode {
            return;
        }

        if let Some(node) = self.tree_cache.get(self.selected_row).cloned() {
            if node.has_children && node.is_expanded {
                if let Some(row) = self.get_row_by_path_mut(&node.path) {
                    row.expanded = false;
                    self.rebuild_tree_cache();
                }
            }
        }
    }

    /// Expand all rows in tree mode
    pub fn expand_all(&mut self) {
        if !self.tree_mode {
            return;
        }
        Self::set_expanded_recursive(&mut self.rows, true);
        self.rebuild_tree_cache();
    }

    /// Collapse all rows in tree mode
    pub fn collapse_all(&mut self) {
        if !self.tree_mode {
            return;
        }
        Self::set_expanded_recursive(&mut self.rows, false);
        self.rebuild_tree_cache();
    }

    /// Recursively set expanded state for all rows
    fn set_expanded_recursive(rows: &mut [GridRow], expanded: bool) {
        for row in rows.iter_mut() {
            if !row.children.is_empty() {
                row.expanded = expanded;
                Self::set_expanded_recursive(&mut row.children, expanded);
            }
        }
    }

    /// Get tree indent string for rendering
    #[allow(dead_code)] // Used for tree rendering
    fn get_tree_indent(&self, node: &TreeNodeInfo) -> String {
        if node.depth == 0 {
            return String::new();
        }

        let mut indent = String::new();

        // Add vertical lines for parent levels
        for _ in 0..node.depth.saturating_sub(1) {
            indent.push_str("│ ");
        }

        // Add branch character for this level
        if node.is_last_child {
            indent.push_str("└─");
        } else {
            indent.push_str("├─");
        }

        indent
    }

    /// Get expand/collapse indicator for tree node
    #[allow(dead_code)] // Used for tree rendering
    fn get_tree_indicator(&self, node: &TreeNodeInfo) -> &'static str {
        if !node.has_children {
            "  "
        } else if node.is_expanded {
            "▼ "
        } else {
            "▶ "
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Export Methods
    // ─────────────────────────────────────────────────────────────────────────

    /// Export grid data with options
    pub fn export(&self, options: &ExportOptions) -> String {
        let separator = match options.format {
            ExportFormat::Csv => ',',
            ExportFormat::Tsv => '\t',
            ExportFormat::PlainText => ' ',
        };

        let mut output = String::new();

        // Get visible columns
        let visible_cols: Vec<_> = if options.visible_columns_only {
            self.columns
                .iter()
                .enumerate()
                .filter(|(_, c)| c.visible)
                .collect()
        } else {
            self.columns.iter().enumerate().collect()
        };

        // Export headers
        if options.include_headers {
            let headers: Vec<_> = visible_cols
                .iter()
                .map(|(_, c)| self.escape_value(&c.title, options.format))
                .collect();
            output.push_str(&headers.join(&separator.to_string()));
            output.push('\n');
        }

        // Get rows to export
        let row_indices: Vec<usize> = if options.selected_only {
            self.filtered_indices()
                .iter()
                .enumerate()
                .filter(|(_, &idx)| self.rows.get(idx).is_some_and(|r| r.selected))
                .map(|(i, _)| i)
                .collect()
        } else {
            (0..self.filtered_count()).collect()
        };

        // Export rows
        for row_idx in row_indices {
            if let Some(&actual_idx) = self.filtered_indices().get(row_idx) {
                if let Some(row) = self.rows.get(actual_idx) {
                    let values: Vec<_> = visible_cols
                        .iter()
                        .map(|(_, c)| {
                            let value = row.get(&c.key).unwrap_or("");
                            self.escape_value(value, options.format)
                        })
                        .collect();
                    output.push_str(&values.join(&separator.to_string()));
                    output.push('\n');
                }
            }
        }

        output
    }

    /// Export as CSV
    pub fn export_csv(&self) -> String {
        self.export(&ExportOptions::new().format(ExportFormat::Csv))
    }

    /// Export as TSV
    pub fn export_tsv(&self) -> String {
        self.export(&ExportOptions::new().format(ExportFormat::Tsv))
    }

    /// Copy current cell value
    pub fn copy_cell(&self) -> String {
        let visible_cols: Vec<_> = self.columns.iter().filter(|c| c.visible).collect();

        if let Some(col) = visible_cols.get(self.selected_col) {
            if let Some(&actual_idx) = self.filtered_indices().get(self.selected_row) {
                if let Some(row) = self.rows.get(actual_idx) {
                    return row.get(&col.key).unwrap_or("").to_string();
                }
            }
        }
        String::new()
    }

    /// Copy selected rows as CSV
    pub fn copy_selected(&self) -> String {
        self.export(&ExportOptions::new().selected_only(true))
    }

    /// Escape value for export format
    fn escape_value(&self, value: &str, format: ExportFormat) -> String {
        match format {
            ExportFormat::Csv => {
                if value.contains(',') || value.contains('"') || value.contains('\n') {
                    format!("\"{}\"", value.replace('"', "\"\""))
                } else {
                    value.to_string()
                }
            }
            ExportFormat::Tsv => {
                if value.contains('\t') || value.contains('\n') {
                    value
                        .chars()
                        .map(|c| if c == '\t' || c == '\n' { ' ' } else { c })
                        .collect()
                } else {
                    value.to_string()
                }
            }
            ExportFormat::PlainText => value.to_string(),
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Aggregation Footer Methods
    // ─────────────────────────────────────────────────────────────────────────

    /// Add a footer row
    pub fn footer(mut self, row: FooterRow) -> Self {
        self.footer_rows.push(row);
        self.show_footer = true;
        self
    }

    /// Show/hide footer
    pub fn show_footer(mut self, show: bool) -> Self {
        self.show_footer = show;
        self
    }

    /// Add a quick sum aggregation
    pub fn add_sum(mut self, column_key: impl Into<String>) -> Self {
        let key = column_key.into();
        if self.footer_rows.is_empty() {
            self.footer_rows.push(FooterRow::new("Total"));
        }
        if let Some(footer) = self.footer_rows.first_mut() {
            footer
                .aggregations
                .push(ColumnAggregation::new(key, AggregationType::Sum));
        }
        self.show_footer = true;
        self
    }

    /// Add a quick average aggregation
    pub fn add_average(mut self, column_key: impl Into<String>) -> Self {
        let key = column_key.into();
        if self.footer_rows.is_empty() {
            self.footer_rows.push(FooterRow::new("Average"));
        }
        if let Some(footer) = self.footer_rows.first_mut() {
            footer
                .aggregations
                .push(ColumnAggregation::new(key, AggregationType::Average));
        }
        self.show_footer = true;
        self
    }

    /// Compute aggregation value for a column
    fn compute_aggregation(&self, column_key: &str, agg_type: AggregationType) -> Option<f64> {
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
    fn get_footer_values(&self, footer: &FooterRow) -> Vec<(String, String)> {
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

    /// Sort by column (cancels any active edit)
    pub fn sort(&mut self, column: usize) {
        if column >= self.columns.len() || !self.columns[column].sortable {
            return;
        }

        // Cancel any active edit before sorting (row indices will change)
        if self.edit_state.active {
            self.cancel_edit();
        }

        if self.sort_column == Some(column) {
            self.sort_direction = self.sort_direction.toggle();
        } else {
            self.sort_column = Some(column);
            self.sort_direction = SortDirection::Ascending;
        }

        let key = &self.columns[column].key;
        let col_type = self.columns[column].col_type;
        let ascending = self.sort_direction == SortDirection::Ascending;
        let use_natural = self.options.use_natural_sort;

        self.rows.sort_by(|a, b| {
            let va = a.get(key).unwrap_or("");
            let vb = b.get(key).unwrap_or("");

            let cmp = match col_type {
                ColumnType::Number => {
                    let na: f64 = va.parse().unwrap_or(0.0);
                    let nb: f64 = vb.parse().unwrap_or(0.0);
                    na.partial_cmp(&nb).unwrap_or(Ordering::Equal)
                }
                ColumnType::Text | ColumnType::Custom => {
                    if use_natural {
                        natural_cmp(va, vb)
                    } else {
                        va.cmp(vb)
                    }
                }
                _ => va.cmp(vb),
            };

            if ascending {
                cmp
            } else {
                cmp.reverse()
            }
        });
        self.recompute_cache();
    }

    /// Set filter (cancels any active edit)
    pub fn set_filter(&mut self, filter: impl Into<String>) {
        // Cancel any active edit before filtering (row visibility will change)
        if self.edit_state.active {
            self.cancel_edit();
        }
        self.filter = filter.into().to_lowercase();
        self.recompute_cache();
    }

    /// Compute filtered row indices (internal)
    fn compute_filtered_indices(&self) -> Vec<usize> {
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
    fn filtered_indices(&self) -> &[usize] {
        &self.filtered_cache
    }

    /// Get filtered rows count (uses cache)
    fn filtered_count(&self) -> usize {
        self.filtered_indices().len()
    }

    /// Get filtered rows (uses cached indices)
    /// Note: For large datasets, prefer using filtered_indices() with index-based access
    #[allow(dead_code)]
    fn filtered_rows(&self) -> Vec<&GridRow> {
        self.filtered_indices()
            .iter()
            .filter_map(|&i| self.rows.get(i))
            .collect()
    }

    /// Select next row (with auto-scroll)
    pub fn select_next(&mut self) {
        let count = self.filtered_count();
        if self.selected_row < count.saturating_sub(1) {
            self.selected_row += 1;
            self.ensure_visible();
        }
    }

    /// Select previous row (with auto-scroll)
    pub fn select_prev(&mut self) {
        if self.selected_row > 0 {
            self.selected_row -= 1;
            self.ensure_visible();
        }
    }

    /// Page down
    pub fn page_down(&mut self, page_size: usize) {
        let count = self.filtered_count();
        self.selected_row = (self.selected_row + page_size).min(count.saturating_sub(1));
        self.ensure_visible();
    }

    /// Page up
    pub fn page_up(&mut self, page_size: usize) {
        self.selected_row = self.selected_row.saturating_sub(page_size);
        self.ensure_visible();
    }

    /// Go to first row
    pub fn select_first(&mut self) {
        self.selected_row = 0;
        self.ensure_visible();
    }

    /// Go to last row
    pub fn select_last(&mut self) {
        let count = self.filtered_count();
        self.selected_row = count.saturating_sub(1);
        self.ensure_visible();
    }

    /// Ensure selected row is visible (auto-scroll)
    pub fn ensure_visible(&mut self) {
        // This will be called with viewport_height from render
        // For now, use a reasonable default
        self.ensure_visible_with_height(20);
    }

    /// Ensure selected row is visible with specific viewport height
    pub fn ensure_visible_with_height(&mut self, viewport_height: usize) {
        if self.selected_row < self.scroll_row {
            // Scroll up to show selected row
            self.scroll_row = self.selected_row;
        } else if self.selected_row >= self.scroll_row + viewport_height {
            // Scroll down to show selected row
            self.scroll_row = self.selected_row.saturating_sub(viewport_height - 1);
        }
    }

    /// Set viewport height (called during render)
    pub fn set_viewport_height(&mut self, height: usize) {
        self.ensure_visible_with_height(height);
    }

    /// Get scroll position info (current, total, viewport)
    pub fn scroll_info(&self) -> (usize, usize, usize) {
        let total = self.filtered_count();
        (self.scroll_row, total, 20) // Default viewport, will be updated in render
    }

    /// Get total row count
    pub fn row_count(&self) -> usize {
        self.filtered_count()
    }

    /// Get visible row count
    pub fn visible_row_count(&self) -> usize {
        self.filtered_count()
    }

    /// Select next column
    pub fn select_next_col(&mut self) {
        let visible_cols: Vec<_> = self
            .columns
            .iter()
            .enumerate()
            .filter(|(_, c)| c.visible)
            .collect();
        if let Some(pos) = visible_cols
            .iter()
            .position(|(i, _)| *i == self.selected_col)
        {
            if pos < visible_cols.len() - 1 {
                self.selected_col = visible_cols[pos + 1].0;
            }
        }
    }

    /// Select previous column
    pub fn select_prev_col(&mut self) {
        let visible_cols: Vec<_> = self
            .columns
            .iter()
            .enumerate()
            .filter(|(_, c)| c.visible)
            .collect();
        if let Some(pos) = visible_cols
            .iter()
            .position(|(i, _)| *i == self.selected_col)
        {
            if pos > 0 {
                self.selected_col = visible_cols[pos - 1].0;
            }
        }
    }

    /// Toggle row selection
    pub fn toggle_selection(&mut self) {
        if self.options.multi_select && self.selected_row < self.rows.len() {
            self.rows[self.selected_row].selected = !self.rows[self.selected_row].selected;
        }
    }

    /// Get selected rows
    pub fn selected_rows(&self) -> Vec<&GridRow> {
        self.rows.iter().filter(|r| r.selected).collect()
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Cell Editing
    // ─────────────────────────────────────────────────────────────────────────

    /// Check if currently editing a cell
    pub fn is_editing(&self) -> bool {
        self.edit_state.active
    }

    /// Start editing the selected cell
    pub fn start_edit(&mut self) -> bool {
        // Early bounds check - no clone needed, just copy indices
        let selected_row = self.selected_row;
        let selected_col = self.selected_col;

        if selected_col >= self.columns.len() {
            return false;
        }

        if !self.columns[selected_col].editable {
            return false;
        }

        // Get actual row index from filtered cache (zero-copy access)
        let row_idx = match self.filtered_cache.get(selected_row) {
            Some(&idx) => idx,
            None => return false,
        };

        if row_idx >= self.rows.len() {
            return false;
        }

        // Get current cell value
        let col_key = &self.columns[selected_col].key;
        let value = self.rows[row_idx].get(col_key).unwrap_or("").to_string();

        self.edit_state = EditState {
            active: true,
            row: row_idx,
            col: selected_col,
            cursor: value.chars().count(),
            buffer: value,
        };
        true
    }

    /// Commit the current edit
    pub fn commit_edit(&mut self) -> bool {
        if !self.edit_state.active {
            return false;
        }

        // Validate bounds before accessing
        if self.edit_state.col >= self.columns.len() {
            self.edit_state.active = false;
            return false;
        }
        if self.edit_state.row >= self.rows.len() {
            self.edit_state.active = false;
            return false;
        }

        let col_key = self.columns[self.edit_state.col].key.clone();
        let row = &mut self.rows[self.edit_state.row];

        // Update the cell value
        if let Some(cell) = row.data.iter_mut().find(|(k, _)| k == &col_key) {
            cell.1 = self.edit_state.buffer.clone();
        } else {
            row.data.push((col_key, self.edit_state.buffer.clone()));
        }

        self.edit_state.active = false;
        self.recompute_cache();
        true
    }

    /// Cancel the current edit
    pub fn cancel_edit(&mut self) {
        self.edit_state.active = false;
    }

    /// Get the current edit buffer
    pub fn edit_buffer(&self) -> Option<&str> {
        if self.edit_state.active {
            Some(&self.edit_state.buffer)
        } else {
            None
        }
    }

    /// Handle key input in edit mode
    fn handle_edit_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Escape => {
                self.cancel_edit();
                true
            }
            Key::Enter => {
                self.commit_edit();
                true
            }
            Key::Char(c) => {
                let pos = self.edit_state.cursor;
                self.edit_state.buffer.insert(
                    self.edit_state
                        .buffer
                        .char_indices()
                        .nth(pos)
                        .map(|(i, _)| i)
                        .unwrap_or(self.edit_state.buffer.len()),
                    *c,
                );
                self.edit_state.cursor += 1;
                true
            }
            Key::Backspace => {
                if self.edit_state.cursor > 0 {
                    self.edit_state.cursor -= 1;
                    let byte_pos = self
                        .edit_state
                        .buffer
                        .char_indices()
                        .nth(self.edit_state.cursor)
                        .map(|(i, _)| i)
                        .unwrap_or(0);
                    if let Some((_, ch)) = self
                        .edit_state
                        .buffer
                        .char_indices()
                        .nth(self.edit_state.cursor)
                    {
                        self.edit_state
                            .buffer
                            .drain(byte_pos..byte_pos + ch.len_utf8());
                    }
                }
                true
            }
            Key::Delete => {
                let char_count = self.edit_state.buffer.chars().count();
                if self.edit_state.cursor < char_count {
                    let byte_pos = self
                        .edit_state
                        .buffer
                        .char_indices()
                        .nth(self.edit_state.cursor)
                        .map(|(i, _)| i)
                        .unwrap_or(0);
                    if let Some((_, ch)) = self
                        .edit_state
                        .buffer
                        .char_indices()
                        .nth(self.edit_state.cursor)
                    {
                        self.edit_state
                            .buffer
                            .drain(byte_pos..byte_pos + ch.len_utf8());
                    }
                }
                true
            }
            Key::Left => {
                if self.edit_state.cursor > 0 {
                    self.edit_state.cursor -= 1;
                }
                true
            }
            Key::Right => {
                let char_count = self.edit_state.buffer.chars().count();
                if self.edit_state.cursor < char_count {
                    self.edit_state.cursor += 1;
                }
                true
            }
            Key::Home => {
                self.edit_state.cursor = 0;
                true
            }
            Key::End => {
                self.edit_state.cursor = self.edit_state.buffer.chars().count();
                true
            }
            _ => false,
        }
    }

    /// Handle key input
    pub fn handle_key(&mut self, key: &Key) -> bool {
        // If editing, delegate to edit handler
        if self.edit_state.active {
            return self.handle_edit_key(key);
        }

        match key {
            Key::Up | Key::Char('k') => {
                self.select_prev();
                true
            }
            Key::Down | Key::Char('j') => {
                self.select_next();
                true
            }
            Key::Left | Key::Char('h') => {
                self.select_prev_col();
                true
            }
            Key::Right | Key::Char('l') => {
                self.select_next_col();
                true
            }
            Key::PageDown => {
                self.page_down(10);
                true
            }
            Key::PageUp => {
                self.page_up(10);
                true
            }
            Key::Home | Key::Char('g') => {
                self.select_first();
                true
            }
            Key::End | Key::Char('G') => {
                self.select_last();
                true
            }
            Key::Enter => {
                // Try to start editing, fall back to sort
                if !self.start_edit() {
                    self.sort(self.selected_col);
                }
                true
            }
            Key::Char(' ') if self.options.multi_select => {
                self.toggle_selection();
                true
            }
            _ => false,
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Mouse Event Handling
    // ─────────────────────────────────────────────────────────────────────────

    /// Handle mouse event for column resize, reorder, etc.
    ///
    /// Returns true if the event was handled.
    pub fn handle_mouse(
        &mut self,
        kind: crate::event::MouseEventKind,
        x: u16,
        y: u16,
        area: crate::layout::Rect,
    ) -> bool {
        use crate::event::{MouseButton, MouseEventKind};

        match kind {
            MouseEventKind::Down(MouseButton::Left) => {
                // Check for resize handle first (higher priority)
                if let Some(col) = self.hit_test_resize_handle(x, y, area) {
                    self.start_resize(col, x, area);
                    return true;
                }
                // Check for column header drag (reorder)
                if self.reorderable {
                    if let Some(col) = self.hit_test_header(x, y, area) {
                        self.start_column_drag(col);
                        return true;
                    }
                }
                false
            }
            MouseEventKind::Drag(MouseButton::Left) => {
                if self.resizing_col.is_some() {
                    self.apply_resize_delta(x);
                    return true;
                }
                if self.dragging_col.is_some() {
                    self.update_drop_target(x, area);
                    return true;
                }
                false
            }
            MouseEventKind::Up(MouseButton::Left) => {
                if self.resizing_col.is_some() {
                    self.end_resize();
                    return true;
                }
                if self.dragging_col.is_some() {
                    self.end_column_drag();
                    return true;
                }
                false
            }
            MouseEventKind::Move => {
                // Update hover state for resize handles
                let prev = self.hovered_resize;
                self.hovered_resize = self.hit_test_resize_handle(x, y, area);
                prev != self.hovered_resize
            }
            _ => false,
        }
    }

    /// Test if position is on a column resize handle
    fn hit_test_resize_handle(&self, x: u16, y: u16, area: crate::layout::Rect) -> Option<usize> {
        // Only detect in header row
        if !self.options.show_header || y != area.y {
            return None;
        }

        let row_num_width = if self.options.show_row_numbers { 5 } else { 0 };
        let mut col_x = area.x + row_num_width;

        let widths = self.get_display_widths(area.width);

        for (i, col) in self.columns.iter().enumerate() {
            if !col.visible {
                continue;
            }
            let width = widths.get(i).copied().unwrap_or(col.min_width);
            col_x += width + 1; // +1 for separator

            // Check if x is within ±1 of column boundary
            if x >= col_x.saturating_sub(1) && x <= col_x && col.resizable {
                return Some(i);
            }
        }
        None
    }

    /// Test if position is on a column header
    fn hit_test_header(&self, x: u16, y: u16, area: crate::layout::Rect) -> Option<usize> {
        // Only detect in header row
        if !self.options.show_header || y != area.y {
            return None;
        }

        let row_num_width = if self.options.show_row_numbers { 5 } else { 0 };
        let mut col_x = area.x + row_num_width;

        let widths = self.get_display_widths(area.width);

        for (i, col) in self.columns.iter().enumerate() {
            if !col.visible {
                continue;
            }
            let width = widths.get(i).copied().unwrap_or(col.min_width);
            let next_x = col_x + width;

            if x >= col_x && x < next_x {
                return Some(i);
            }
            col_x = next_x + 1; // +1 for separator
        }
        None
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Column Resize Implementation
    // ─────────────────────────────────────────────────────────────────────────

    /// Start resizing a column
    fn start_resize(&mut self, col: usize, x: u16, area: crate::layout::Rect) {
        if col >= self.columns.len() || !self.columns[col].resizable {
            return;
        }

        // Ensure column_widths is populated
        if self.column_widths.is_empty() {
            self.column_widths = self.get_display_widths(area.width);
        }

        self.resizing_col = Some(col);
        self.resize_start_x = x;
        self.resize_start_width = self.column_widths.get(col).copied().unwrap_or(10);
    }

    /// Apply resize delta
    fn apply_resize_delta(&mut self, current_x: u16) {
        let col_idx = match self.resizing_col {
            Some(idx) => idx,
            None => return,
        };

        let delta = current_x as i16 - self.resize_start_x as i16;
        let new_width = (self.resize_start_width as i16 + delta).max(1) as u16;

        let col = &self.columns[col_idx];
        let constrained = new_width.max(col.min_width).min(if col.max_width > 0 {
            col.max_width
        } else {
            u16::MAX
        });

        // Update column width
        if col_idx < self.column_widths.len() {
            let old_width = self.column_widths[col_idx];
            if old_width != constrained {
                self.column_widths[col_idx] = constrained;

                // Call callback
                if let Some(ref mut cb) = self.on_column_resize {
                    cb(col_idx, constrained);
                }
            }
        }
    }

    /// End resizing
    fn end_resize(&mut self) {
        self.resizing_col = None;
    }

    /// Check if currently resizing
    pub fn is_resizing(&self) -> bool {
        self.resizing_col.is_some()
    }

    /// Get the current width of a column
    pub fn column_width(&self, col: usize) -> Option<u16> {
        self.column_widths.get(col).copied()
    }

    /// Set a column width programmatically
    pub fn set_column_width(&mut self, col: usize, width: u16) {
        // Ensure column_widths is populated
        while self.column_widths.len() <= col {
            self.column_widths.push(10); // Default width
        }

        let col_def = self.columns.get(col);
        let constrained = if let Some(c) = col_def {
            width.max(c.min_width).min(if c.max_width > 0 {
                c.max_width
            } else {
                u16::MAX
            })
        } else {
            width
        };

        self.column_widths[col] = constrained;

        if let Some(ref mut cb) = self.on_column_resize {
            cb(col, constrained);
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Column Reorder Implementation
    // ─────────────────────────────────────────────────────────────────────────

    /// Start dragging a column
    fn start_column_drag(&mut self, col: usize) {
        if !self.reorderable || col >= self.columns.len() {
            return;
        }

        // Initialize column order if not set
        if self.column_order.is_empty() {
            self.column_order = (0..self.columns.len()).collect();
        }

        self.dragging_col = Some(col);
        self.drop_target_col = Some(col);
    }

    /// Update drop target during drag
    fn update_drop_target(&mut self, x: u16, area: crate::layout::Rect) {
        if self.dragging_col.is_none() {
            return;
        }

        let row_num_width = if self.options.show_row_numbers { 5 } else { 0 };
        let mut col_x = area.x + row_num_width;

        let widths = self.get_display_widths(area.width);

        for (i, col) in self.columns.iter().enumerate() {
            if !col.visible {
                continue;
            }
            let width = widths.get(i).copied().unwrap_or(col.min_width);
            let mid = col_x + width / 2;

            if x < mid {
                self.drop_target_col = Some(i);
                return;
            }
            col_x += width + 1;
        }

        // If past all columns, drop at the end
        self.drop_target_col = Some(self.columns.len());
    }

    /// End column drag and perform reorder
    fn end_column_drag(&mut self) {
        if let (Some(from), Some(to)) = (self.dragging_col, self.drop_target_col) {
            if from != to && to != from + 1 {
                // Initialize column order if not set
                if self.column_order.is_empty() {
                    self.column_order = (0..self.columns.len()).collect();
                }

                // Perform reorder on column_order
                let col_idx = self.column_order.remove(from);
                let insert_idx = if to > from { to - 1 } else { to };
                let insert_idx = insert_idx.min(self.column_order.len());
                self.column_order.insert(insert_idx, col_idx);

                // Also reorder column_widths if set
                if !self.column_widths.is_empty() {
                    let width = self.column_widths.remove(from);
                    self.column_widths.insert(insert_idx, width);
                }

                // Reorder the actual columns vector
                let col = self.columns.remove(from);
                self.columns.insert(insert_idx, col);

                // Call callback
                if let Some(ref mut cb) = self.on_column_reorder {
                    cb(from, insert_idx);
                }
            }
        }

        self.dragging_col = None;
        self.drop_target_col = None;
    }

    /// Check if currently dragging a column
    pub fn is_dragging_column(&self) -> bool {
        self.dragging_col.is_some()
    }

    /// Move selected column left (keyboard reorder)
    pub fn move_column_left(&mut self) {
        if !self.reorderable || self.selected_col == 0 {
            return;
        }

        let from = self.selected_col;
        let to = self.selected_col - 1;

        self.columns.swap(from, to);

        if !self.column_widths.is_empty() && from < self.column_widths.len() {
            self.column_widths.swap(from, to);
        }

        self.selected_col = to;

        if let Some(ref mut cb) = self.on_column_reorder {
            cb(from, to);
        }
    }

    /// Move selected column right (keyboard reorder)
    pub fn move_column_right(&mut self) {
        if !self.reorderable || self.selected_col >= self.columns.len().saturating_sub(1) {
            return;
        }

        let from = self.selected_col;
        let to = self.selected_col + 1;

        self.columns.swap(from, to);

        if !self.column_widths.is_empty() && to < self.column_widths.len() {
            self.column_widths.swap(from, to);
        }

        self.selected_col = to;

        if let Some(ref mut cb) = self.on_column_reorder {
            cb(from, to);
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Column Freeze Implementation
    // ─────────────────────────────────────────────────────────────────────────

    /// Scroll columns left
    pub fn scroll_col_left(&mut self) {
        if self.scroll_col > 0 {
            self.scroll_col -= 1;
        }
    }

    /// Scroll columns right
    pub fn scroll_col_right(&mut self) {
        let scrollable = self
            .columns
            .len()
            .saturating_sub(self.frozen_left + self.frozen_right);
        if self.scroll_col < scrollable.saturating_sub(1) {
            self.scroll_col += 1;
        }
    }

    /// Get frozen left column count
    pub fn frozen_left(&self) -> usize {
        self.frozen_left
    }

    /// Get frozen right column count
    pub fn frozen_right(&self) -> usize {
        self.frozen_right
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Width Calculation Helpers
    // ─────────────────────────────────────────────────────────────────────────

    /// Get display widths, using user-set widths if available
    fn get_display_widths(&self, available: u16) -> Vec<u16> {
        if !self.column_widths.is_empty() {
            self.column_widths.clone()
        } else {
            self.calculate_widths(available)
        }
    }

    /// Calculate column widths
    fn calculate_widths(&self, available: u16) -> Vec<u16> {
        let visible_cols: Vec<_> = self.columns.iter().filter(|c| c.visible).collect();

        if visible_cols.is_empty() {
            return vec![];
        }

        let row_num_width = if self.options.show_row_numbers { 5 } else { 0 };
        let borders = visible_cols.len() as u16 + 1;
        let available = available.saturating_sub(row_num_width + borders);

        // Start with fixed or min widths
        let mut widths: Vec<u16> = visible_cols
            .iter()
            .map(|c| if c.width > 0 { c.width } else { c.min_width })
            .collect();

        let total: u16 = widths.iter().sum();

        if total < available {
            // Distribute extra space
            let extra = available - total;
            let per_col = extra / visible_cols.len() as u16;
            for (i, col) in visible_cols.iter().enumerate() {
                let new_width = widths[i] + per_col;
                widths[i] = new_width.min(col.max_width);
            }
        }

        widths
    }
}

impl Default for DataGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl View for DataGrid {
    crate::impl_view_meta!("DataGrid");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 10 || area.height < 3 {
            return;
        }

        let widths = self.get_display_widths(area.width);

        // Get visible columns in display order (respecting column_order)
        let visible_cols: Vec<_> = if self.column_order.is_empty() {
            self.columns
                .iter()
                .enumerate()
                .filter(|(_, c)| c.visible)
                .collect()
        } else {
            self.column_order
                .iter()
                .filter_map(|&orig_idx| {
                    self.columns
                        .get(orig_idx)
                        .filter(|c| c.visible)
                        .map(|c| (orig_idx, c))
                })
                .collect()
        };

        let row_num_width: u16 = if self.options.show_row_numbers { 5 } else { 0 };
        let header_height: u16 = if self.options.show_header { 1 } else { 0 };

        let mut y = area.y;

        // Draw header
        if self.options.show_header {
            self.render_header(ctx, &visible_cols, &widths, area.x + row_num_width, y);
            y += 1;
        }

        // Calculate visible range with virtual scrolling
        let total_rows = self.filtered_count();
        let visible_height =
            (area.height - header_height) as usize / self.options.row_height.max(1) as usize;

        // Virtual scroll: calculate render range with overscan
        let (render_start, render_end) = if self.options.virtual_scroll {
            let overscan = self.options.overscan;
            let start = self.scroll_row.saturating_sub(overscan);
            let end = (self.scroll_row + visible_height + overscan).min(total_rows);
            (start, end)
        } else {
            (0, total_rows)
        };

        let params = RowRenderParams {
            visible_cols: &visible_cols,
            widths: &widths,
            area_x: area.x,
            start_y: y,
            row_num_width,
            visible_height,
        };

        // Render rows using index-based access (no allocation)
        self.render_rows_virtual(ctx, render_start, render_end, &params);

        // Draw scrollbar if needed
        self.render_scrollbar(ctx, total_rows, visible_height, area, y);
    }
}

impl_styled_view!(DataGrid);
impl_props_builders!(DataGrid);

impl DataGrid {
    /// Render rows using virtual scrolling (index-based, no allocation)
    fn render_rows_virtual(
        &self,
        ctx: &mut RenderContext,
        render_start: usize,
        render_end: usize,
        params: &RowRenderParams<'_>,
    ) {
        let filtered_indices = self.filtered_indices();

        for render_idx in render_start..render_end {
            // Skip rows above viewport (but within overscan)
            if render_idx < self.scroll_row.saturating_sub(self.options.overscan) {
                continue;
            }

            // Get actual row index from filtered cache
            let Some(&actual_row_idx) = filtered_indices.get(render_idx) else {
                continue;
            };

            let Some(row) = self.rows.get(actual_row_idx) else {
                continue;
            };

            // Calculate Y position relative to viewport
            let viewport_offset = render_idx.saturating_sub(self.scroll_row);
            if viewport_offset >= params.visible_height {
                continue;
            }

            let row_y = params.start_y + (viewport_offset as u16 * self.options.row_height);
            let is_selected = render_idx == self.selected_row;
            let is_alt = self.options.zebra && render_idx % 2 == 1;

            let row_bg = if is_selected {
                self.colors.selected_bg
            } else if is_alt {
                self.colors.alt_row_bg
            } else {
                self.colors.row_bg
            };

            // Draw row number
            if self.options.show_row_numbers {
                self.render_row_number(ctx, params.area_x, row_y, render_idx + 1, row_bg);
            }

            // Draw cells
            let mut x = params.area_x + params.row_num_width;
            for (col_idx, (orig_col_idx, col)) in params.visible_cols.iter().enumerate() {
                if col_idx >= params.widths.len() {
                    break;
                }
                let w = params.widths[col_idx];
                let is_editing = self.edit_state.active
                    && render_idx == self.selected_row
                    && *orig_col_idx == self.edit_state.col;

                let pos = CellPos {
                    x,
                    y: row_y,
                    width: w,
                };
                let state = CellState {
                    row_bg,
                    is_selected,
                    is_editing,
                };
                self.render_cell(ctx, row, col, &pos, &state);

                // Draw separator
                let mut sep = Cell::new('│');
                sep.fg = Some(self.colors.border_color);
                sep.bg = Some(row_bg);
                ctx.buffer.set(x + w, row_y, sep);

                x += w + 1;
            }
        }
    }

    /// Render the header row
    fn render_header(
        &self,
        ctx: &mut RenderContext,
        visible_cols: &[(usize, &GridColumn)],
        widths: &[u16],
        start_x: u16,
        y: u16,
    ) {
        let mut x = start_x;

        for (col_idx, (orig_idx, col)) in visible_cols.iter().enumerate() {
            if col_idx >= widths.len() {
                break;
            }
            let w = widths[col_idx];
            let is_sort_col = self.sort_column == Some(*orig_idx);
            let is_selected = *orig_idx == self.selected_col;
            let is_dragging = self.dragging_col == Some(col_idx);

            // Draw drop indicator before this column
            if self.drop_target_col == Some(col_idx) && self.dragging_col.is_some() {
                let mut cell = Cell::new('│');
                cell.fg = Some(Color::CYAN);
                cell.modifier |= Modifier::BOLD;
                ctx.buffer.set(x.saturating_sub(1), y, cell);
            }

            // Draw header cell background
            let bg = if is_selected {
                self.colors.selected_bg
            } else {
                self.colors.header_bg
            };
            for dx in 0..w {
                let mut cell = Cell::new(' ');
                cell.bg = Some(bg);
                ctx.buffer.set(x + dx, y, cell);
            }

            // Draw title with sort indicator
            let mut title = col.title.clone();
            if is_sort_col {
                title.push(' ');
                title.push(self.sort_direction.icon());
            }

            // Dim text if this column is being dragged
            let fg = if is_dragging {
                Color::rgb(100, 100, 100)
            } else {
                self.colors.header_fg
            };

            for (j, ch) in title.chars().take(w as usize - 1).enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(fg);
                cell.bg = Some(bg);
                if !is_dragging {
                    cell.modifier |= Modifier::BOLD;
                } else {
                    cell.modifier |= Modifier::DIM;
                }
                ctx.buffer.set(x + j as u16, y, cell);
            }

            // Draw separator with resize indicator
            let is_resize_hover = self.hovered_resize == Some(col_idx);
            let is_resizing = self.resizing_col == Some(col_idx);
            let sep_char = if is_resize_hover || is_resizing {
                '⇔'
            } else {
                '│'
            };
            let sep_color = if is_resizing {
                Color::CYAN
            } else if is_resize_hover {
                Color::YELLOW
            } else {
                self.colors.border_color
            };

            let mut sep = Cell::new(sep_char);
            sep.fg = Some(sep_color);
            sep.bg = Some(bg);
            ctx.buffer.set(x + w, y, sep);

            x += w + 1;
        }

        // Draw drop indicator at the end if dropping after last column
        if let Some(target) = self.drop_target_col {
            if target >= visible_cols.len() && self.dragging_col.is_some() {
                let mut cell = Cell::new('│');
                cell.fg = Some(Color::CYAN);
                cell.modifier |= Modifier::BOLD;
                ctx.buffer.set(x.saturating_sub(1), y, cell);
            }
        }
    }

    /// Render all visible rows (legacy, non-virtual scroll)
    #[allow(dead_code)]
    fn render_rows(
        &self,
        ctx: &mut RenderContext,
        filtered: &[&GridRow],
        params: &RowRenderParams<'_>,
    ) {
        for (i, row) in filtered
            .iter()
            .skip(self.scroll_row)
            .take(params.visible_height)
            .enumerate()
        {
            let row_y = params.start_y + i as u16;
            let is_selected = self.scroll_row + i == self.selected_row;
            let is_alt = self.options.zebra && i % 2 == 1;

            let row_bg = if is_selected {
                self.colors.selected_bg
            } else if is_alt {
                self.colors.alt_row_bg
            } else {
                self.colors.row_bg
            };

            // Draw row number
            if self.options.show_row_numbers {
                self.render_row_number(ctx, params.area_x, row_y, self.scroll_row + i + 1, row_bg);
            }

            // Draw cells
            let mut x = params.area_x + params.row_num_width;
            for (col_idx, (orig_col_idx, col)) in params.visible_cols.iter().enumerate() {
                if col_idx >= params.widths.len() {
                    break;
                }
                let w = params.widths[col_idx];
                let is_editing = self.edit_state.active
                    && self.scroll_row + i == self.selected_row
                    && *orig_col_idx == self.edit_state.col;

                let pos = CellPos {
                    x,
                    y: row_y,
                    width: w,
                };
                let state = CellState {
                    row_bg,
                    is_selected,
                    is_editing,
                };
                self.render_cell(ctx, row, col, &pos, &state);

                // Draw separator
                let mut sep = Cell::new('│');
                sep.fg = Some(self.colors.border_color);
                sep.bg = Some(row_bg);
                ctx.buffer.set(x + w, row_y, sep);

                x += w + 1;
            }
        }
    }

    /// Render row number column
    fn render_row_number(&self, ctx: &mut RenderContext, x: u16, y: u16, num: usize, bg: Color) {
        let num_str = format!("{:>4}", num);
        for (j, ch) in num_str.chars().enumerate() {
            let mut cell = Cell::new(ch);
            cell.fg = Some(Color::rgb(100, 100, 100));
            cell.bg = Some(bg);
            ctx.buffer.set(x + j as u16, y, cell);
        }
    }

    /// Render a single cell
    fn render_cell(
        &self,
        ctx: &mut RenderContext,
        row: &GridRow,
        col: &GridColumn,
        pos: &CellPos,
        state: &CellState,
    ) {
        let cell_bg = if state.is_editing {
            Color::rgb(50, 50, 80) // Edit mode background
        } else {
            state.row_bg
        };

        // Fill background
        for dx in 0..pos.width {
            let mut cell = Cell::new(' ');
            cell.bg = Some(cell_bg);
            ctx.buffer.set(pos.x + dx, pos.y, cell);
        }

        // Draw value or edit buffer
        if state.is_editing {
            self.render_edit_cell(ctx, pos.x, pos.y, pos.width, cell_bg);
        } else if let Some(value) = row.get(&col.key) {
            self.render_value_cell(ctx, value, col, pos, state.row_bg, state.is_selected);
        }
    }

    /// Render cell in edit mode with cursor
    fn render_edit_cell(&self, ctx: &mut RenderContext, x: u16, y: u16, width: u16, bg: Color) {
        let display: String = self
            .edit_state
            .buffer
            .chars()
            .take(width as usize - 1)
            .collect();
        for (j, ch) in display.chars().enumerate() {
            let is_cursor = j == self.edit_state.cursor;
            let mut cell = Cell::new(ch);
            cell.fg = Some(if is_cursor {
                Color::BLACK
            } else {
                Color::WHITE
            });
            cell.bg = Some(if is_cursor { Color::WHITE } else { bg });
            ctx.buffer.set(x + j as u16, y, cell);
        }
        // Draw cursor at end if needed
        if self.edit_state.cursor >= display.chars().count()
            && self.edit_state.cursor < width as usize
        {
            let mut cursor_cell = Cell::new(' ');
            cursor_cell.bg = Some(Color::WHITE);
            ctx.buffer
                .set(x + self.edit_state.cursor as u16, y, cursor_cell);
        }
    }

    /// Render cell with value (respecting alignment)
    fn render_value_cell(
        &self,
        ctx: &mut RenderContext,
        value: &str,
        col: &GridColumn,
        pos: &CellPos,
        row_bg: Color,
        is_selected: bool,
    ) {
        let display: String = value.chars().take(pos.width as usize - 1).collect();
        let start_x = match col.align {
            Alignment::Left => pos.x,
            Alignment::Center => pos.x + (pos.width.saturating_sub(display.len() as u16)) / 2,
            Alignment::Right => pos.x + pos.width.saturating_sub(display.len() as u16 + 1),
        };

        for (j, ch) in display.chars().enumerate() {
            let mut cell = Cell::new(ch);
            cell.fg = Some(if is_selected {
                self.colors.selected_fg
            } else {
                Color::WHITE
            });
            cell.bg = Some(row_bg);
            ctx.buffer.set(start_x + j as u16, pos.y, cell);
        }
    }

    /// Render scrollbar and row indicator
    fn render_scrollbar(
        &self,
        ctx: &mut RenderContext,
        total_rows: usize,
        visible_height: usize,
        area: crate::layout::Rect,
        content_y: u16,
    ) {
        if total_rows <= visible_height {
            return;
        }

        let scrollbar_x = area.x + area.width - 1;
        let scrollbar_height = visible_height as f64;
        let thumb_height =
            (scrollbar_height * visible_height as f64 / total_rows as f64).max(1.0) as u16;
        let thumb_pos = (self.scroll_row as f64 / (total_rows - visible_height) as f64
            * (scrollbar_height - thumb_height as f64)) as u16;

        for i in 0..visible_height {
            let scrollbar_y = content_y + i as u16;
            let i_u16 = i as u16;
            let is_thumb = i_u16 >= thumb_pos && i_u16 < (thumb_pos + thumb_height);

            let mut cell = if is_thumb {
                Cell::new('█')
            } else {
                Cell::new('░')
            };
            cell.fg = Some(Color::rgb(100, 100, 120));
            ctx.buffer.set(scrollbar_x, scrollbar_y, cell);
        }

        // Draw row indicator
        let indicator = format!(" {}/{} ", self.selected_row + 1, total_rows);
        let indicator_x = area.x + area.width.saturating_sub(indicator.len() as u16 + 1);
        let indicator_y = area.y + area.height - 1;

        for (j, ch) in indicator.chars().enumerate() {
            let mut cell = Cell::new(ch);
            cell.fg = Some(Color::rgb(150, 150, 150));
            cell.bg = Some(Color::rgb(40, 40, 50));
            let cell_x = indicator_x + (j as u16);
            if cell_x < area.x + area.width {
                ctx.buffer.set(cell_x, indicator_y, cell);
            }
        }
    }
}

// Helper functions

/// Create a new data grid
pub fn datagrid() -> DataGrid {
    DataGrid::new()
}

/// Create a new grid column with key and title
pub fn grid_column(key: impl Into<String>, title: impl Into<String>) -> GridColumn {
    GridColumn::new(key, title)
}

/// Create a new grid row
pub fn grid_row() -> GridRow {
    GridRow::new()
}
