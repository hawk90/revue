//! Core CSV Viewer implementation

use super::parse::{detect_delimiter, parse_csv};
use super::types::{Delimiter, SortOrder};
use crate::style::Color;
use crate::widget::theme::{DARK_GRAY, PLACEHOLDER_FG};
use crate::widget::traits::WidgetProps;

/// CSV Viewer widget
#[derive(Clone, Debug)]
pub struct CsvViewer {
    /// Raw CSV data
    pub data: Vec<Vec<String>>,
    /// Whether first row is header
    pub has_header: bool,
    /// Column widths (0 = auto)
    pub column_widths: Vec<u16>,
    /// Selected row index
    pub selected_row: usize,
    /// Selected column index
    pub selected_col: usize,
    /// Scroll offset (row)
    pub scroll_row: usize,
    /// Scroll offset (column)
    pub scroll_col: usize,
    /// Sort column index
    pub sort_column: Option<usize>,
    /// Sort order
    pub sort_order: SortOrder,
    /// Sorted row indices
    pub sorted_indices: Vec<usize>,
    /// Search query
    pub search_query: String,
    /// Search matches (row, col)
    pub search_matches: Vec<(usize, usize)>,
    /// Current search match index
    pub current_match: usize,
    /// Show row numbers
    pub show_row_numbers: bool,
    /// Show column separators
    pub show_separators: bool,
    /// Delimiter used
    pub delimiter: Delimiter,
    // Styling
    /// Header foreground color
    pub header_fg: Option<Color>,
    /// Header background color
    pub header_bg: Option<Color>,
    /// Selected cell foreground color
    pub selected_fg: Option<Color>,
    /// Selected cell background color
    pub selected_bg: Option<Color>,
    /// Search match foreground color
    pub match_fg: Option<Color>,
    /// Search match background color
    pub match_bg: Option<Color>,
    /// Separator foreground color
    pub separator_fg: Option<Color>,
    /// Row number foreground color
    pub row_number_fg: Option<Color>,
    /// Default foreground color
    pub fg: Option<Color>,
    /// Default background color
    pub bg: Option<Color>,
    /// CSS props
    pub props: WidgetProps,
}

impl Default for CsvViewer {
    fn default() -> Self {
        Self::new()
    }
}

impl CsvViewer {
    /// Create a new CSV viewer
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            has_header: true,
            column_widths: Vec::new(),
            selected_row: 0,
            selected_col: 0,
            scroll_row: 0,
            scroll_col: 0,
            sort_column: None,
            sort_order: SortOrder::None,
            sorted_indices: Vec::new(),
            search_query: String::new(),
            search_matches: Vec::new(),
            current_match: 0,
            show_row_numbers: true,
            show_separators: true,
            delimiter: Delimiter::Auto,
            header_fg: Some(Color::WHITE),
            header_bg: Some(Color::rgb(60, 60, 80)),
            selected_fg: Some(Color::WHITE),
            selected_bg: Some(Color::BLUE),
            match_fg: Some(Color::BLACK),
            match_bg: Some(Color::YELLOW),
            separator_fg: Some(DARK_GRAY),
            row_number_fg: Some(PLACEHOLDER_FG),
            fg: None,
            bg: None,
            props: WidgetProps::new(),
        }
    }

    /// Parse CSV from string content
    pub fn from_content(content: &str) -> Self {
        let mut viewer = Self::new();
        viewer.parse(content);
        viewer
    }

    /// Parse CSV content
    pub fn parse(&mut self, content: &str) {
        self.data = parse_csv(content, detect_delimiter(content, self.delimiter));
        self.calculate_column_widths();
        self.reset_sort();
        self.reset_view();
    }

    /// Set CSV data directly
    pub fn data(mut self, data: Vec<Vec<String>>) -> Self {
        self.data = data;
        self.calculate_column_widths();
        self.reset_sort();
        self.reset_view();
        self
    }

    /// New data: move the selection and scroll back to the first cell and
    /// look for the search query in the new rows.
    fn reset_view(&mut self) {
        self.selected_row = 0;
        self.selected_col = 0;
        self.scroll_row = 0;
        self.scroll_col = 0;
        self.rerun_search();
    }

    /// Set whether first row is header
    ///
    /// The rows already loaded are re-read: the first line joins or leaves
    /// the data rows, and the sort and the search are applied again.
    pub fn has_header(mut self, has_header: bool) -> Self {
        if self.has_header != has_header {
            self.has_header = has_header;
            self.selected_row = 0;
            self.apply_sort();
        }
        self
    }

    /// Set delimiter
    pub fn delimiter(mut self, delimiter: Delimiter) -> Self {
        self.delimiter = delimiter;
        self
    }

    /// Show/hide row numbers
    pub fn show_row_numbers(mut self, show: bool) -> Self {
        self.show_row_numbers = show;
        self
    }

    /// Show/hide column separators
    pub fn show_separators(mut self, show: bool) -> Self {
        self.show_separators = show;
        self
    }

    /// Set header style
    pub fn header_style(mut self, fg: Color, bg: Color) -> Self {
        self.header_fg = Some(fg);
        self.header_bg = Some(bg);
        self
    }

    /// Set selected cell style
    pub fn selected_style(mut self, fg: Color, bg: Color) -> Self {
        self.selected_fg = Some(fg);
        self.selected_bg = Some(bg);
        self
    }

    /// Set search match style
    pub fn match_style(mut self, fg: Color, bg: Color) -> Self {
        self.match_fg = Some(fg);
        self.match_bg = Some(bg);
        self
    }

    /// Set foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    // ─────────────────────────────────────────────────────────────────────────
    // State getters
    // ─────────────────────────────────────────────────────────────────────────

    /// Get number of rows (excluding header if present)
    pub fn row_count(&self) -> usize {
        if self.has_header && !self.data.is_empty() {
            self.data.len() - 1
        } else {
            self.data.len()
        }
    }

    /// Get number of columns
    pub fn column_count(&self) -> usize {
        self.data.first().map(|r| r.len()).unwrap_or(0)
    }

    /// Get selected row index
    ///
    /// This is the row's position on screen, counted from the first data row
    /// in the current sort order. Without sorting it is also its index for
    /// [`get_cell`](Self::get_cell).
    pub fn selected_row(&self) -> usize {
        self.selected_row
    }

    /// Get selected column index
    pub fn selected_col(&self) -> usize {
        self.selected_col
    }

    /// Get cell value at position
    pub fn get_cell(&self, row: usize, col: usize) -> Option<&str> {
        let actual_row = row + self.first_data_row();
        self.data
            .get(actual_row)
            .and_then(|r| r.get(col))
            .map(|s| s.as_str())
    }

    /// Get header value at column
    pub fn get_header(&self, col: usize) -> Option<&str> {
        if self.has_header {
            self.data
                .first()
                .and_then(|r| r.get(col))
                .map(|s| s.as_str())
        } else {
            None
        }
    }

    /// Get selected cell value
    pub fn selected_value(&self) -> Option<&str> {
        let row = self.record_at(self.selected_row)?;
        self.get_cell(row, self.selected_col)
    }

    /// Index in `data` of the first data row: 1 past a header, else 0
    pub(super) fn first_data_row(&self) -> usize {
        usize::from(self.has_header)
    }

    /// Data row (as [`get_cell`](Self::get_cell) counts) shown at `position`
    pub(super) fn record_at(&self, position: usize) -> Option<usize> {
        self.sorted_indices
            .get(position)
            .map(|&i| i - self.first_data_row())
    }

    /// Position on screen of data row `row`
    pub(super) fn position_of(&self, row: usize) -> Option<usize> {
        let data_idx = row + self.first_data_row();
        self.sorted_indices.iter().position(|&i| i == data_idx)
    }

    /// Check if search is active
    pub fn is_searching(&self) -> bool {
        !self.search_query.is_empty()
    }

    /// Get search match count
    pub fn match_count(&self) -> usize {
        self.search_matches.len()
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Navigation
    // ─────────────────────────────────────────────────────────────────────────

    /// Move selection down
    pub fn select_down(&mut self) {
        let max_row = self.row_count().saturating_sub(1);
        self.selected_row = (self.selected_row + 1).min(max_row);
    }

    /// Move selection up
    pub fn select_up(&mut self) {
        self.selected_row = self.selected_row.saturating_sub(1);
    }

    /// Move selection right
    pub fn select_right(&mut self) {
        let max_col = self.column_count().saturating_sub(1);
        self.selected_col = (self.selected_col + 1).min(max_col);
    }

    /// Move selection left
    pub fn select_left(&mut self) {
        self.selected_col = self.selected_col.saturating_sub(1);
    }

    /// Select first row
    pub fn select_first_row(&mut self) {
        self.selected_row = 0;
    }

    /// Select last row
    pub fn select_last_row(&mut self) {
        self.selected_row = self.row_count().saturating_sub(1);
    }

    /// Page down
    pub fn page_down(&mut self, page_size: usize) {
        let max_row = self.row_count().saturating_sub(1);
        self.selected_row = (self.selected_row + page_size).min(max_row);
    }

    /// Page up
    pub fn page_up(&mut self, page_size: usize) {
        self.selected_row = self.selected_row.saturating_sub(page_size);
    }

    /// Calculate optimal column widths
    fn calculate_column_widths(&mut self) {
        let col_count = self.column_count();
        self.column_widths = vec![0; col_count];

        for row in &self.data {
            for (col, cell) in row.iter().enumerate() {
                if col < self.column_widths.len() {
                    let width = crate::utils::display_width(cell).min(u16::MAX as usize) as u16;
                    self.column_widths[col] = self.column_widths[col].max(width);
                }
            }
        }

        // Cap widths at reasonable maximum
        for w in &mut self.column_widths {
            *w = (*w).clamp(3, 40);
        }
    }

    /// Get row number width
    pub fn row_number_width(&self) -> u16 {
        if self.show_row_numbers {
            let digits = (self.row_count().max(1) as f64).log10().floor() as u16 + 1;
            digits.max(2) + 1 // +1 for padding
        } else {
            0
        }
    }
}
