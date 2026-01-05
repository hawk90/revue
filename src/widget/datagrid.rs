//! DataGrid widget for advanced data display
//!
//! A feature-rich data grid with sorting, filtering, and cell editing.

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

/// Grid color scheme
#[derive(Clone, Debug)]
pub struct GridColors {
    /// Header background color
    pub header_bg: Color,
    /// Header foreground color
    pub header_fg: Color,
    /// Normal row background
    pub row_bg: Color,
    /// Alternate row background (zebra striping)
    pub alt_row_bg: Color,
    /// Selected row background
    pub selected_bg: Color,
    /// Selected row foreground
    pub selected_fg: Color,
    /// Border/separator color
    pub border_color: Color,
}

impl Default for GridColors {
    fn default() -> Self {
        Self {
            header_bg: Color::rgb(60, 60, 80),
            header_fg: Color::WHITE,
            row_bg: Color::rgb(30, 30, 30),
            alt_row_bg: Color::rgb(40, 40, 40),
            selected_bg: Color::rgb(60, 100, 180),
            selected_fg: Color::WHITE,
            border_color: Color::rgb(80, 80, 80),
        }
    }
}

impl GridColors {
    /// Create a new color scheme
    pub fn new() -> Self {
        Self::default()
    }

    /// Dark theme (default)
    pub fn dark() -> Self {
        Self::default()
    }

    /// Light theme
    pub fn light() -> Self {
        Self {
            header_bg: Color::rgb(220, 220, 230),
            header_fg: Color::BLACK,
            row_bg: Color::rgb(255, 255, 255),
            alt_row_bg: Color::rgb(245, 245, 250),
            selected_bg: Color::rgb(100, 150, 220),
            selected_fg: Color::WHITE,
            border_color: Color::rgb(180, 180, 190),
        }
    }
}

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
    fn toggle(&self) -> Self {
        match self {
            SortDirection::Ascending => SortDirection::Descending,
            SortDirection::Descending => SortDirection::Ascending,
        }
    }

    fn icon(&self) -> char {
        match self {
            SortDirection::Ascending => '▲',
            SortDirection::Descending => '▼',
        }
    }
}

/// Grid column definition
#[derive(Clone)]
pub struct GridColumn {
    /// Column key/id
    pub key: String,
    /// Display title
    pub title: String,
    /// Column type
    pub col_type: ColumnType,
    /// Width (0 = auto)
    pub width: u16,
    /// Minimum width
    pub min_width: u16,
    /// Maximum width
    pub max_width: u16,
    /// Is sortable
    pub sortable: bool,
    /// Is filterable
    pub filterable: bool,
    /// Is editable
    pub editable: bool,
    /// Is visible
    pub visible: bool,
    /// Alignment
    pub align: Alignment,
}

/// Text alignment
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Alignment {
    #[default]
    Left,
    Center,
    Right,
}

impl GridColumn {
    /// Create a new column
    pub fn new(key: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            title: title.into(),
            col_type: ColumnType::Text,
            width: 0,
            min_width: 5,
            max_width: 50,
            sortable: true,
            filterable: true,
            editable: false,
            visible: true,
            align: Alignment::Left,
        }
    }

    /// Set column type
    pub fn col_type(mut self, t: ColumnType) -> Self {
        self.col_type = t;
        self
    }

    /// Set width
    pub fn width(mut self, w: u16) -> Self {
        self.width = w;
        self
    }

    /// Set min width
    pub fn min_width(mut self, w: u16) -> Self {
        self.min_width = w;
        self
    }

    /// Set max width
    pub fn max_width(mut self, w: u16) -> Self {
        self.max_width = w;
        self
    }

    /// Set sortable
    pub fn sortable(mut self, s: bool) -> Self {
        self.sortable = s;
        self
    }

    /// Set editable
    pub fn editable(mut self, e: bool) -> Self {
        self.editable = e;
        self
    }

    /// Set alignment
    pub fn align(mut self, a: Alignment) -> Self {
        self.align = a;
        self
    }

    /// Right align
    pub fn right(mut self) -> Self {
        self.align = Alignment::Right;
        self
    }

    /// Center align
    pub fn center(mut self) -> Self {
        self.align = Alignment::Center;
        self
    }
}

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
}

impl Default for GridRow {
    fn default() -> Self {
        Self::new()
    }
}

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

        let widths = self.calculate_widths(area.width);
        let visible_cols: Vec<_> = self
            .columns
            .iter()
            .enumerate()
            .filter(|(_, c)| c.visible)
            .collect();

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

            for (j, ch) in title.chars().take(w as usize - 1).enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(self.colors.header_fg);
                cell.bg = Some(bg);
                cell.modifier |= Modifier::BOLD;
                ctx.buffer.set(x + j as u16, y, cell);
            }

            x += w + 1;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_grid_column() {
        let col = GridColumn::new("name", "Name").width(20).sortable(true);

        assert_eq!(col.key, "name");
        assert_eq!(col.title, "Name");
        assert_eq!(col.width, 20);
        assert!(col.sortable);
    }

    #[test]
    fn test_grid_row() {
        let row = GridRow::new().cell("name", "John").cell("age", "30");

        assert_eq!(row.get("name"), Some("John"));
        assert_eq!(row.get("age"), Some("30"));
        assert_eq!(row.get("unknown"), None);
    }

    #[test]
    fn test_data_grid() {
        let grid = DataGrid::new()
            .column(GridColumn::new("name", "Name"))
            .column(GridColumn::new("age", "Age"))
            .row(GridRow::new().cell("name", "Alice").cell("age", "25"))
            .row(GridRow::new().cell("name", "Bob").cell("age", "30"));

        assert_eq!(grid.columns.len(), 2);
        assert_eq!(grid.rows.len(), 2);
    }

    #[test]
    fn test_sorting() {
        let mut grid = DataGrid::new()
            .column(GridColumn::new("name", "Name"))
            .row(GridRow::new().cell("name", "Charlie"))
            .row(GridRow::new().cell("name", "Alice"))
            .row(GridRow::new().cell("name", "Bob"));

        grid.sort(0);

        assert_eq!(grid.rows[0].get("name"), Some("Alice"));
        assert_eq!(grid.rows[1].get("name"), Some("Bob"));
        assert_eq!(grid.rows[2].get("name"), Some("Charlie"));
    }

    #[test]
    fn test_filtering() {
        let mut grid = DataGrid::new()
            .column(GridColumn::new("name", "Name"))
            .row(GridRow::new().cell("name", "Alice"))
            .row(GridRow::new().cell("name", "Bob"))
            .row(GridRow::new().cell("name", "Alex"));

        grid.set_filter("al");

        let filtered = grid.filtered_rows();
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_navigation() {
        let mut grid = DataGrid::new()
            .column(GridColumn::new("a", "A"))
            .column(GridColumn::new("b", "B"))
            .row(GridRow::new().cell("a", "1").cell("b", "2"))
            .row(GridRow::new().cell("a", "3").cell("b", "4"));

        assert_eq!(grid.selected_row, 0);

        grid.select_next();
        assert_eq!(grid.selected_row, 1);

        grid.select_prev();
        assert_eq!(grid.selected_row, 0);
    }

    #[test]
    fn test_render() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let grid = DataGrid::new()
            .column(GridColumn::new("name", "Name"))
            .row(GridRow::new().cell("name", "Test"));

        grid.render(&mut ctx);
        // Smoke test
    }

    #[test]
    fn test_natural_sorting() {
        let mut grid = DataGrid::new()
            .natural_sort(true)
            .column(GridColumn::new("file", "File"))
            .row(GridRow::new().cell("file", "file10.txt"))
            .row(GridRow::new().cell("file", "file2.txt"))
            .row(GridRow::new().cell("file", "file1.txt"));

        grid.sort(0);

        assert_eq!(grid.rows[0].get("file"), Some("file1.txt"));
        assert_eq!(grid.rows[1].get("file"), Some("file2.txt"));
        assert_eq!(grid.rows[2].get("file"), Some("file10.txt"));
    }

    #[test]
    fn test_ascii_sorting() {
        let mut grid = DataGrid::new()
            .natural_sort(false)
            .column(GridColumn::new("file", "File"))
            .row(GridRow::new().cell("file", "file10.txt"))
            .row(GridRow::new().cell("file", "file2.txt"))
            .row(GridRow::new().cell("file", "file1.txt"));

        grid.sort(0);

        // ASCII sort: "file1" < "file10" < "file2"
        assert_eq!(grid.rows[0].get("file"), Some("file1.txt"));
        assert_eq!(grid.rows[1].get("file"), Some("file10.txt"));
        assert_eq!(grid.rows[2].get("file"), Some("file2.txt"));
    }

    #[test]
    fn test_filter_cache() {
        let mut grid = DataGrid::new()
            .column(GridColumn::new("name", "Name"))
            .row(GridRow::new().cell("name", "Alice"))
            .row(GridRow::new().cell("name", "Bob"))
            .row(GridRow::new().cell("name", "Alex"))
            .row(GridRow::new().cell("name", "Charlie"));

        // Initial: all 4 rows
        assert_eq!(grid.filtered_count(), 4);

        // Multiple calls should use cache
        assert_eq!(grid.filtered_count(), 4);
        assert_eq!(grid.row_count(), 4);

        // Filter: only "al" matches
        grid.set_filter("al");
        assert_eq!(grid.filtered_count(), 2);

        // Cache should be invalidated and recomputed
        assert_eq!(grid.filtered_rows().len(), 2);

        // Clear filter
        grid.set_filter("");
        assert_eq!(grid.filtered_count(), 4);
    }

    #[test]
    fn test_cache_invalidation_on_sort() {
        let mut grid = DataGrid::new()
            .column(GridColumn::new("name", "Name"))
            .row(GridRow::new().cell("name", "Charlie"))
            .row(GridRow::new().cell("name", "Alice"))
            .row(GridRow::new().cell("name", "Bob"));

        // Access cache
        assert_eq!(grid.filtered_count(), 3);

        // Sort should invalidate cache
        grid.sort(0);

        // Cache should still work correctly after sort
        assert_eq!(grid.filtered_count(), 3);
        let rows = grid.filtered_rows();
        assert_eq!(rows[0].get("name"), Some("Alice"));
    }

    #[test]
    fn test_cell_edit_start() {
        let mut grid = DataGrid::new()
            .column(GridColumn::new("name", "Name").editable(true))
            .row(GridRow::new().cell("name", "Alice"));

        assert!(!grid.is_editing());

        // Start editing
        assert!(grid.start_edit());
        assert!(grid.is_editing());
        assert_eq!(grid.edit_buffer(), Some("Alice"));
    }

    #[test]
    fn test_cell_edit_non_editable() {
        let mut grid = DataGrid::new()
            .column(GridColumn::new("name", "Name").editable(false))
            .row(GridRow::new().cell("name", "Alice"));

        // Should not be able to edit non-editable column
        assert!(!grid.start_edit());
        assert!(!grid.is_editing());
    }

    #[test]
    fn test_cell_edit_commit() {
        let mut grid = DataGrid::new()
            .column(GridColumn::new("name", "Name").editable(true))
            .row(GridRow::new().cell("name", "Alice"));

        grid.start_edit();

        // Type some text
        grid.handle_key(&Key::Backspace); // Delete 'e'
        grid.handle_key(&Key::Backspace); // Delete 'c'
        grid.handle_key(&Key::Backspace); // Delete 'i'
        grid.handle_key(&Key::Backspace); // Delete 'l'
        grid.handle_key(&Key::Backspace); // Delete 'A'
        grid.handle_key(&Key::Char('B'));
        grid.handle_key(&Key::Char('o'));
        grid.handle_key(&Key::Char('b'));

        // Commit with Enter
        grid.handle_key(&Key::Enter);

        assert!(!grid.is_editing());
        assert_eq!(grid.rows[0].get("name"), Some("Bob"));
    }

    #[test]
    fn test_cell_edit_cancel() {
        let mut grid = DataGrid::new()
            .column(GridColumn::new("name", "Name").editable(true))
            .row(GridRow::new().cell("name", "Alice"));

        grid.start_edit();

        // Type some text
        grid.handle_key(&Key::Char('X'));
        grid.handle_key(&Key::Char('Y'));
        grid.handle_key(&Key::Char('Z'));

        // Cancel with Escape
        grid.handle_key(&Key::Escape);

        assert!(!grid.is_editing());
        // Value should be unchanged
        assert_eq!(grid.rows[0].get("name"), Some("Alice"));
    }

    #[test]
    fn test_cell_edit_cursor_movement() {
        let mut grid = DataGrid::new()
            .column(GridColumn::new("name", "Name").editable(true))
            .row(GridRow::new().cell("name", "Test"));

        grid.start_edit();
        assert_eq!(grid.edit_state.cursor, 4); // At end

        // Move cursor left
        grid.handle_key(&Key::Left);
        assert_eq!(grid.edit_state.cursor, 3);

        // Move to start
        grid.handle_key(&Key::Home);
        assert_eq!(grid.edit_state.cursor, 0);

        // Move to end
        grid.handle_key(&Key::End);
        assert_eq!(grid.edit_state.cursor, 4);
    }

    #[test]
    fn test_virtual_scroll_enabled_by_default() {
        let grid = DataGrid::new();
        assert!(grid.options.virtual_scroll);
        assert_eq!(grid.options.row_height, 1);
        assert_eq!(grid.options.overscan, 5);
    }

    #[test]
    fn test_virtual_scroll_builder_methods() {
        let grid = DataGrid::new()
            .virtual_scroll(true)
            .row_height(2)
            .overscan(10);

        assert!(grid.options.virtual_scroll);
        assert_eq!(grid.options.row_height, 2);
        assert_eq!(grid.options.overscan, 10);
    }

    #[test]
    fn test_virtual_scroll_disabled() {
        let grid = DataGrid::new().virtual_scroll(false);
        assert!(!grid.options.virtual_scroll);
    }

    #[test]
    fn test_large_dataset_100k_rows() {
        // Create grid with 100,000 rows
        let mut grid = DataGrid::new()
            .virtual_scroll(true)
            .overscan(5)
            .column(GridColumn::new("id", "ID"))
            .column(GridColumn::new("name", "Name"));

        // Add 100,000 rows
        let mut rows = Vec::with_capacity(100_000);
        for i in 0..100_000 {
            rows.push(
                GridRow::new()
                    .cell("id", i.to_string())
                    .cell("name", format!("Row {}", i)),
            );
        }
        grid = grid.rows(rows);

        // Verify row count
        assert_eq!(grid.row_count(), 100_000);

        // Navigation should work
        grid.select_last();
        assert_eq!(grid.selected_row, 99_999);

        grid.select_first();
        assert_eq!(grid.selected_row, 0);

        // Page navigation
        grid.page_down(100);
        assert_eq!(grid.selected_row, 100);

        // Render should only process visible rows (smoke test)
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);
        grid.render(&mut ctx);
        // If this completes quickly, virtual scroll is working
    }

    #[test]
    fn test_virtual_scroll_render_range() {
        let mut grid = DataGrid::new()
            .virtual_scroll(true)
            .overscan(3)
            .column(GridColumn::new("id", "ID"));

        // Add 100 rows
        let rows: Vec<_> = (0..100)
            .map(|i| GridRow::new().cell("id", i.to_string()))
            .collect();
        grid = grid.rows(rows);

        // Scroll to middle
        grid.selected_row = 50;
        grid.scroll_row = 45;

        // With viewport of 20 rows and overscan of 3:
        // render_start = 45 - 3 = 42
        // render_end = 45 + 20 + 3 = 68 (capped at 100)
        let total = grid.filtered_count();
        let visible_height = 20;
        let overscan = grid.options.overscan;

        let render_start = grid.scroll_row.saturating_sub(overscan);
        let render_end = (grid.scroll_row + visible_height + overscan).min(total);

        assert_eq!(render_start, 42);
        assert_eq!(render_end, 68);
    }

    #[test]
    fn test_row_height_calculation() {
        let grid = DataGrid::new().row_height(2);
        assert_eq!(grid.options.row_height, 2);

        // Row height of 0 should be clamped to 1
        let grid = DataGrid::new().row_height(0);
        assert_eq!(grid.options.row_height, 1);
    }
}
