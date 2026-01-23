//! Helper functions for CSV Viewer widget

use super::types::Delimiter;
use super::CsvViewer;

impl CsvViewer {
    /// Parse CSV from string content
    pub fn from_content(content: &str) -> Self {
        let mut viewer = Self::new();
        viewer.parse(content);
        viewer
    }

    /// Parse CSV content
    pub fn parse(&mut self, content: &str) {
        let delimiter = super::parser::detect_delimiter(content, self.delimiter);
        self.data = super::parser::parse_csv(content, delimiter);
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
    pub fn header_style(mut self, fg: crate::style::Color, bg: crate::style::Color) -> Self {
        self.header_fg = Some(fg);
        self.header_bg = Some(bg);
        self
    }

    /// Set selected cell style
    pub fn selected_style(mut self, fg: crate::style::Color, bg: crate::style::Color) -> Self {
        self.selected_fg = Some(fg);
        self.selected_bg = Some(bg);
        self
    }

    /// Set search match style
    pub fn match_style(mut self, fg: crate::style::Color, bg: crate::style::Color) -> Self {
        self.match_fg = Some(fg);
        self.match_bg = Some(bg);
        self
    }

    /// Set foreground color
    pub fn fg(mut self, color: crate::style::Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set background color
    pub fn bg(mut self, color: crate::style::Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Calculate optimal column widths
    fn calculate_column_widths(&mut self) {
        self.column_widths = super::parser::calculate_column_widths(&self.data);
    }
}

/// Helper function to create a CSV viewer
pub fn csv_viewer() -> CsvViewer {
    CsvViewer::new()
}
