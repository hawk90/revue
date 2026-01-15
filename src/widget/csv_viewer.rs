//! CSV Viewer widget for displaying tabular CSV data
//!
//! Features:
//! - Auto-detect delimiters (comma, tab, semicolon, pipe)
//! - Header row detection
//! - Column sorting (ascending/descending)
//! - Search across all cells
//! - Virtual scrolling for large files
//! - Column width auto-sizing
//! - Row numbering

use super::traits::{RenderContext, View, WidgetProps};
use crate::render::Cell;
use crate::style::Color;
use crate::utils::natural_cmp;
use crate::{impl_props_builders, impl_styled_view};

/// CSV delimiter options
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Delimiter {
    /// Auto-detect delimiter
    #[default]
    Auto,
    /// Comma (,)
    Comma,
    /// Tab (\t)
    Tab,
    /// Semicolon (;)
    Semicolon,
    /// Pipe (|)
    Pipe,
    /// Custom delimiter
    Custom(char),
}

impl Delimiter {
    /// Get the actual character for parsing
    pub fn char(&self) -> Option<char> {
        match self {
            Delimiter::Auto => None,
            Delimiter::Comma => Some(','),
            Delimiter::Tab => Some('\t'),
            Delimiter::Semicolon => Some(';'),
            Delimiter::Pipe => Some('|'),
            Delimiter::Custom(c) => Some(*c),
        }
    }
}

/// Sort direction for columns
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SortOrder {
    /// No sorting applied
    #[default]
    None,
    /// Ascending order (A-Z, 0-9)
    Ascending,
    /// Descending order (Z-A, 9-0)
    Descending,
}

/// CSV Viewer widget
#[derive(Clone, Debug)]
pub struct CsvViewer {
    /// Raw CSV data
    data: Vec<Vec<String>>,
    /// Whether first row is header
    has_header: bool,
    /// Column widths (0 = auto)
    column_widths: Vec<u16>,
    /// Selected row index
    selected_row: usize,
    /// Selected column index
    selected_col: usize,
    /// Scroll offset (row)
    scroll_row: usize,
    /// Scroll offset (column)
    #[allow(dead_code)]
    scroll_col: usize,
    /// Sort column index
    sort_column: Option<usize>,
    /// Sort order
    sort_order: SortOrder,
    /// Sorted row indices
    sorted_indices: Vec<usize>,
    /// Search query
    search_query: String,
    /// Search matches (row, col)
    search_matches: Vec<(usize, usize)>,
    /// Current search match index
    current_match: usize,
    /// Show row numbers
    show_row_numbers: bool,
    /// Show column separators
    show_separators: bool,
    /// Delimiter used
    delimiter: Delimiter,
    // Styling
    header_fg: Option<Color>,
    header_bg: Option<Color>,
    selected_fg: Option<Color>,
    selected_bg: Option<Color>,
    match_fg: Option<Color>,
    match_bg: Option<Color>,
    separator_fg: Option<Color>,
    row_number_fg: Option<Color>,
    fg: Option<Color>,
    bg: Option<Color>,
    /// CSS props
    props: WidgetProps,
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
            separator_fg: Some(Color::rgb(80, 80, 80)),
            row_number_fg: Some(Color::rgb(128, 128, 128)),
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
        let delimiter = self.detect_delimiter(content);
        self.data = self.parse_csv(content, delimiter);
        self.calculate_column_widths();
        self.reset_sort();
    }

    /// Set CSV data directly
    pub fn data(mut self, data: Vec<Vec<String>>) -> Self {
        self.data = data;
        self.calculate_column_widths();
        self.reset_sort();
        self
    }

    /// Set whether first row is header
    pub fn has_header(mut self, has_header: bool) -> Self {
        self.has_header = has_header;
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
    pub fn selected_row(&self) -> usize {
        self.selected_row
    }

    /// Get selected column index
    pub fn selected_col(&self) -> usize {
        self.selected_col
    }

    /// Get cell value at position
    pub fn get_cell(&self, row: usize, col: usize) -> Option<&str> {
        let actual_row = if self.has_header { row + 1 } else { row };
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
        self.get_cell(self.selected_row, self.selected_col)
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
        self.ensure_visible();
    }

    /// Move selection up
    pub fn select_up(&mut self) {
        self.selected_row = self.selected_row.saturating_sub(1);
        self.ensure_visible();
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
        self.ensure_visible();
    }

    /// Select last row
    pub fn select_last_row(&mut self) {
        self.selected_row = self.row_count().saturating_sub(1);
        self.ensure_visible();
    }

    /// Page down
    pub fn page_down(&mut self, page_size: usize) {
        let max_row = self.row_count().saturating_sub(1);
        self.selected_row = (self.selected_row + page_size).min(max_row);
        self.ensure_visible();
    }

    /// Page up
    pub fn page_up(&mut self, page_size: usize) {
        self.selected_row = self.selected_row.saturating_sub(page_size);
        self.ensure_visible();
    }

    /// Ensure selected cell is visible
    fn ensure_visible(&mut self) {
        // Vertical scrolling handled during render
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Sorting
    // ─────────────────────────────────────────────────────────────────────────

    /// Sort by column
    pub fn sort_by(&mut self, column: usize) {
        if self.sort_column == Some(column) {
            // Toggle sort order
            self.sort_order = match self.sort_order {
                SortOrder::None => SortOrder::Ascending,
                SortOrder::Ascending => SortOrder::Descending,
                SortOrder::Descending => SortOrder::None,
            };
        } else {
            self.sort_column = Some(column);
            self.sort_order = SortOrder::Ascending;
        }

        self.apply_sort();
    }

    /// Reset sorting
    pub fn reset_sort(&mut self) {
        self.sort_column = None;
        self.sort_order = SortOrder::None;
        let start = if self.has_header { 1 } else { 0 };
        self.sorted_indices = (start..self.data.len()).collect();
    }

    /// Apply current sort
    fn apply_sort(&mut self) {
        let start = if self.has_header { 1 } else { 0 };
        self.sorted_indices = (start..self.data.len()).collect();

        if let Some(col) = self.sort_column {
            match self.sort_order {
                SortOrder::None => {}
                SortOrder::Ascending => {
                    self.sorted_indices.sort_by(|&a, &b| {
                        let val_a = self
                            .data
                            .get(a)
                            .and_then(|r| r.get(col))
                            .map(|s| s.as_str())
                            .unwrap_or("");
                        let val_b = self
                            .data
                            .get(b)
                            .and_then(|r| r.get(col))
                            .map(|s| s.as_str())
                            .unwrap_or("");
                        natural_cmp(val_a, val_b)
                    });
                }
                SortOrder::Descending => {
                    self.sorted_indices.sort_by(|&a, &b| {
                        let val_a = self
                            .data
                            .get(a)
                            .and_then(|r| r.get(col))
                            .map(|s| s.as_str())
                            .unwrap_or("");
                        let val_b = self
                            .data
                            .get(b)
                            .and_then(|r| r.get(col))
                            .map(|s| s.as_str())
                            .unwrap_or("");
                        natural_cmp(val_b, val_a)
                    });
                }
            }
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Search
    // ─────────────────────────────────────────────────────────────────────────

    /// Set search query
    pub fn search(&mut self, query: &str) {
        self.search_query = query.to_lowercase();
        self.search_matches.clear();
        self.current_match = 0;

        if self.search_query.is_empty() {
            return;
        }

        let start = if self.has_header { 1 } else { 0 };
        for (row_idx, row) in self.data.iter().enumerate().skip(start) {
            for (col_idx, cell) in row.iter().enumerate() {
                if cell.to_lowercase().contains(&self.search_query) {
                    self.search_matches.push((row_idx - start, col_idx));
                }
            }
        }
    }

    /// Clear search
    pub fn clear_search(&mut self) {
        self.search_query.clear();
        self.search_matches.clear();
        self.current_match = 0;
    }

    /// Go to next search match
    pub fn next_match(&mut self) {
        if !self.search_matches.is_empty() {
            self.current_match = (self.current_match + 1) % self.search_matches.len();
            let (row, col) = self.search_matches[self.current_match];
            self.selected_row = row;
            self.selected_col = col;
        }
    }

    /// Go to previous search match
    pub fn prev_match(&mut self) {
        if !self.search_matches.is_empty() {
            self.current_match = self
                .current_match
                .checked_sub(1)
                .unwrap_or(self.search_matches.len() - 1);
            let (row, col) = self.search_matches[self.current_match];
            self.selected_row = row;
            self.selected_col = col;
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Parsing helpers
    // ─────────────────────────────────────────────────────────────────────────

    /// Detect delimiter from content
    fn detect_delimiter(&self, content: &str) -> char {
        if let Some(c) = self.delimiter.char() {
            return c;
        }

        // Count occurrences in first few lines
        let first_lines: String = content.lines().take(5).collect::<Vec<_>>().join("\n");

        let delimiters = [',', '\t', ';', '|'];
        let mut best = ',';
        let mut best_count = 0;

        for &d in &delimiters {
            let count = first_lines.matches(d).count();
            if count > best_count {
                best_count = count;
                best = d;
            }
        }

        best
    }

    /// Parse CSV with given delimiter
    fn parse_csv(&self, content: &str, delimiter: char) -> Vec<Vec<String>> {
        let mut result = Vec::new();
        let mut current_row = Vec::new();
        let mut current_field = String::new();
        let mut in_quotes = false;
        let mut chars = content.chars().peekable();

        while let Some(c) = chars.next() {
            if in_quotes {
                if c == '"' {
                    if chars.peek() == Some(&'"') {
                        // Escaped quote
                        current_field.push('"');
                        chars.next();
                    } else {
                        // End of quoted field
                        in_quotes = false;
                    }
                } else {
                    current_field.push(c);
                }
            } else if c == '"' {
                in_quotes = true;
            } else if c == delimiter {
                current_row.push(current_field.trim().to_string());
                current_field = String::new();
            } else if c == '\n' {
                current_row.push(current_field.trim().to_string());
                if !current_row.iter().all(|s| s.is_empty()) {
                    result.push(current_row);
                }
                current_row = Vec::new();
                current_field = String::new();
            } else if c != '\r' {
                current_field.push(c);
            }
        }

        // Handle last field/row
        if !current_field.is_empty() || !current_row.is_empty() {
            current_row.push(current_field.trim().to_string());
            if !current_row.iter().all(|s| s.is_empty()) {
                result.push(current_row);
            }
        }

        result
    }

    /// Calculate optimal column widths
    fn calculate_column_widths(&mut self) {
        let col_count = self.column_count();
        self.column_widths = vec![0; col_count];

        for row in &self.data {
            for (col, cell) in row.iter().enumerate() {
                if col < self.column_widths.len() {
                    let width = cell.chars().count() as u16;
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
    fn row_number_width(&self) -> u16 {
        if self.show_row_numbers {
            let digits = (self.row_count() as f64).log10().floor() as u16 + 1;
            digits.max(2) + 1 // +1 for padding
        } else {
            0
        }
    }
}

impl Default for CsvViewer {
    fn default() -> Self {
        Self::new()
    }
}

impl View for CsvViewer {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 5 || area.height < 2 {
            return;
        }

        let row_num_width = self.row_number_width();
        let content_start_x = area.x + row_num_width;
        let _content_width = area.width.saturating_sub(row_num_width);

        // Calculate visible rows
        let header_rows = if self.has_header { 1 } else { 0 };
        let visible_data_rows = (area.height as usize).saturating_sub(header_rows);

        // Adjust scroll to keep selection visible
        let mut scroll_row = self.scroll_row;
        if self.selected_row < scroll_row {
            scroll_row = self.selected_row;
        } else if self.selected_row >= scroll_row + visible_data_rows {
            scroll_row = self.selected_row.saturating_sub(visible_data_rows - 1);
        }

        let mut y = area.y;

        // Render header if present
        if self.has_header {
            if let Some(header_row) = self.data.first() {
                // Row number column header
                if self.show_row_numbers {
                    for x in area.x..content_start_x {
                        let mut cell = Cell::new(' ');
                        cell.fg = self.header_fg;
                        cell.bg = self.header_bg;
                        ctx.buffer.set(x, y, cell);
                    }
                }

                // Header cells
                let mut x = content_start_x;
                for (col_idx, cell_value) in header_row.iter().enumerate() {
                    let width = self.column_widths.get(col_idx).copied().unwrap_or(10) as usize;

                    // Sort indicator
                    let sort_indicator = if self.sort_column == Some(col_idx) {
                        match self.sort_order {
                            SortOrder::Ascending => " ▲",
                            SortOrder::Descending => " ▼",
                            SortOrder::None => "",
                        }
                    } else {
                        ""
                    };

                    let display: String = format!("{}{}", cell_value, sort_indicator)
                        .chars()
                        .take(width)
                        .collect();

                    for (i, ch) in display.chars().enumerate() {
                        if x + i as u16 >= area.x + area.width {
                            break;
                        }
                        let mut cell = Cell::new(ch).bold();
                        cell.fg = self.header_fg;
                        cell.bg = self.header_bg;
                        ctx.buffer.set(x + i as u16, y, cell);
                    }

                    // Fill remaining width
                    for i in display.chars().count()..width {
                        if x + i as u16 >= area.x + area.width {
                            break;
                        }
                        let mut cell = Cell::new(' ');
                        cell.fg = self.header_fg;
                        cell.bg = self.header_bg;
                        ctx.buffer.set(x + i as u16, y, cell);
                    }

                    x += width as u16;

                    // Separator
                    if self.show_separators && x < area.x + area.width {
                        let mut cell = Cell::new('│');
                        cell.fg = self.separator_fg;
                        cell.bg = self.header_bg;
                        ctx.buffer.set(x, y, cell);
                        x += 1;
                    }
                }

                y += 1;
            }
        }

        // Render data rows
        for &data_idx in self
            .sorted_indices
            .iter()
            .skip(scroll_row)
            .take(visible_data_rows)
        {
            if y >= area.y + area.height {
                break;
            }

            let row_idx = data_idx - if self.has_header { 1 } else { 0 };
            let is_selected_row = row_idx == self.selected_row;

            // Row number
            if self.show_row_numbers {
                let num_str = format!(
                    "{:>width$}",
                    row_idx + 1,
                    width = (row_num_width - 1) as usize
                );
                for (i, ch) in num_str.chars().enumerate() {
                    let mut cell = Cell::new(ch);
                    cell.fg = self.row_number_fg;
                    cell.bg = if is_selected_row {
                        self.selected_bg
                    } else {
                        self.bg
                    };
                    ctx.buffer.set(area.x + i as u16, y, cell);
                }
            }

            // Data cells
            let mut x = content_start_x;
            if let Some(row_data) = self.data.get(data_idx) {
                for (col_idx, cell_value) in row_data.iter().enumerate() {
                    let width = self.column_widths.get(col_idx).copied().unwrap_or(10) as usize;
                    let is_selected = is_selected_row && col_idx == self.selected_col;
                    let is_match = self.search_matches.contains(&(row_idx, col_idx));

                    let (fg, bg) = if is_selected {
                        (self.selected_fg, self.selected_bg)
                    } else if is_match {
                        (self.match_fg, self.match_bg)
                    } else {
                        (self.fg, self.bg)
                    };

                    let display: String = cell_value.chars().take(width).collect();

                    for (i, ch) in display.chars().enumerate() {
                        if x + i as u16 >= area.x + area.width {
                            break;
                        }
                        let mut cell = Cell::new(ch);
                        cell.fg = fg;
                        cell.bg = bg;
                        ctx.buffer.set(x + i as u16, y, cell);
                    }

                    // Fill remaining width
                    for i in display.chars().count()..width {
                        if x + i as u16 >= area.x + area.width {
                            break;
                        }
                        let mut cell = Cell::new(' ');
                        cell.fg = fg;
                        cell.bg = bg;
                        ctx.buffer.set(x + i as u16, y, cell);
                    }

                    x += width as u16;

                    // Separator
                    if self.show_separators && x < area.x + area.width {
                        let mut cell = Cell::new('│');
                        cell.fg = self.separator_fg;
                        cell.bg = if is_selected_row {
                            self.selected_bg
                        } else {
                            self.bg
                        };
                        ctx.buffer.set(x, y, cell);
                        x += 1;
                    }
                }
            }

            y += 1;
        }
    }

    crate::impl_view_meta!("CsvViewer");
}

impl_styled_view!(CsvViewer);
impl_props_builders!(CsvViewer);

/// Helper function to create a CSV viewer
pub fn csv_viewer() -> CsvViewer {
    CsvViewer::new()
}
