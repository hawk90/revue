//! Table and column formatting utilities
//!
//! Provides easy column alignment for TUI tables.
//!
//! # Example
//! ```ignore
//! let table = Table::new()
//!     .col("HOST", 8, Align::Left)
//!     .col("PING", 7, Align::Right)
//!     .col("CPU", 12, Align::Right)
//!     .spacing(1);
//!
//! let header = table.header();
//! let row = table.row(&["bm05", "3ms", "41%"]);
//! ```

use crate::utils::unicode::display_width;

/// Text alignment
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Align {
    /// Left-aligned (pad right)
    #[default]
    Left,
    /// Center-aligned
    Center,
    /// Right-aligned (pad left)
    Right,
}

/// Column definition
#[derive(Clone, Debug)]
pub struct Column {
    /// Column header text
    pub header: String,
    /// Column width in characters
    pub width: usize,
    /// Text alignment
    pub align: Align,
}

impl Column {
    /// Create a new column
    pub fn new(header: impl Into<String>, width: usize, align: Align) -> Self {
        Self {
            header: header.into(),
            width,
            align,
        }
    }

    /// Format a value according to this column's width and alignment
    pub fn format(&self, value: &str) -> String {
        align_text(value, self.width, self.align)
    }

    /// Format the header according to this column's width and alignment
    pub fn format_header(&self) -> String {
        align_text(&self.header, self.width, self.align)
    }
}

/// Table formatter for consistent column alignment
#[derive(Clone, Debug, Default)]
pub struct Table {
    /// Column definitions
    columns: Vec<Column>,
    /// Spacing between columns
    spacing: usize,
    /// Prefix before first column
    prefix: String,
}

impl Table {
    /// Create a new table formatter
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            spacing: 1,
            prefix: String::new(),
        }
    }

    /// Add a column
    pub fn col(mut self, header: impl Into<String>, width: usize, align: Align) -> Self {
        self.columns.push(Column::new(header, width, align));
        self
    }

    /// Add a left-aligned column
    pub fn col_left(self, header: impl Into<String>, width: usize) -> Self {
        self.col(header, width, Align::Left)
    }

    /// Add a right-aligned column
    pub fn col_right(self, header: impl Into<String>, width: usize) -> Self {
        self.col(header, width, Align::Right)
    }

    /// Add a center-aligned column
    pub fn col_center(self, header: impl Into<String>, width: usize) -> Self {
        self.col(header, width, Align::Center)
    }

    /// Set spacing between columns
    pub fn spacing(mut self, spacing: usize) -> Self {
        self.spacing = spacing;
        self
    }

    /// Set prefix before first column
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = prefix.into();
        self
    }

    /// Generate the header row
    pub fn header(&self) -> String {
        let mut result = self.prefix.clone();
        for (i, col) in self.columns.iter().enumerate() {
            if i > 0 {
                result.push_str(&" ".repeat(self.spacing));
            }
            result.push_str(&col.format_header());
        }
        result
    }

    /// Generate a data row
    pub fn row(&self, values: &[&str]) -> String {
        let mut result = self.prefix.clone();
        for (i, col) in self.columns.iter().enumerate() {
            if i > 0 {
                result.push_str(&" ".repeat(self.spacing));
            }
            let value = values.get(i).unwrap_or(&"");
            result.push_str(&col.format(value));
        }
        result
    }

    /// Generate a row with owned strings
    pub fn row_owned(&self, values: &[String]) -> String {
        let refs: Vec<&str> = values.iter().map(|s| s.as_str()).collect();
        self.row(&refs)
    }

    /// Get the total width of the table
    pub fn total_width(&self) -> usize {
        let col_widths: usize = self.columns.iter().map(|c| c.width).sum();
        let spacing_width = if self.columns.len() > 1 {
            self.spacing * (self.columns.len() - 1)
        } else {
            0
        };
        self.prefix.len() + col_widths + spacing_width
    }

    /// Get column count
    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    /// Get a column by index
    pub fn get_column(&self, index: usize) -> Option<&Column> {
        self.columns.get(index)
    }

    /// Get the starting position of a column (0-indexed offset from start)
    pub fn column_offset(&self, index: usize) -> usize {
        let mut offset = self.prefix.len();
        for (i, col) in self.columns.iter().enumerate() {
            if i == index {
                return offset;
            }
            offset += col.width;
            if i < self.columns.len() - 1 {
                offset += self.spacing;
            }
        }
        offset
    }
}

/// Align text within a given width
///
/// # Arguments
/// * `text` - Text to align
/// * `width` - Target width
/// * `align` - Alignment type
///
/// # Returns
/// Aligned string padded/truncated to exact width
pub fn align_text(text: &str, width: usize, align: Align) -> String {
    let text_width = display_width(text);

    if text_width >= width {
        // Truncate if too long
        let mut result = String::new();
        let mut current_width = 0;
        for ch in text.chars() {
            let ch_width = crate::utils::unicode::char_width(ch);
            if current_width + ch_width > width {
                break;
            }
            result.push(ch);
            current_width += ch_width;
        }
        // Pad remaining space if needed
        while current_width < width {
            result.push(' ');
            current_width += 1;
        }
        result
    } else {
        let padding = width - text_width;
        match align {
            Align::Left => format!("{}{}", text, " ".repeat(padding)),
            Align::Right => format!("{}{}", " ".repeat(padding), text),
            Align::Center => {
                let left_pad = padding / 2;
                let right_pad = padding - left_pad;
                format!("{}{}{}", " ".repeat(left_pad), text, " ".repeat(right_pad))
            }
        }
    }
}

/// Convenience function: left-align text
pub fn align_left(text: &str, width: usize) -> String {
    align_text(text, width, Align::Left)
}

/// Convenience function: right-align text
pub fn align_right(text: &str, width: usize) -> String {
    align_text(text, width, Align::Right)
}

/// Convenience function: center-align text
pub fn align_center(text: &str, width: usize) -> String {
    align_text(text, width, Align::Center)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align_left() {
        assert_eq!(align_left("hi", 5), "hi   ");
        assert_eq!(align_left("hello", 5), "hello");
        assert_eq!(align_left("hello world", 5), "hello");
    }

    #[test]
    fn test_align_right() {
        assert_eq!(align_right("hi", 5), "   hi");
        assert_eq!(align_right("hello", 5), "hello");
    }

    #[test]
    fn test_align_center() {
        assert_eq!(align_center("hi", 6), "  hi  ");
        assert_eq!(align_center("hi", 5), " hi  ");
    }

    #[test]
    fn test_table_header() {
        let table = Table::new()
            .col_left("NAME", 10)
            .col_right("VALUE", 8)
            .spacing(1);

        // NAME(4) + 6 spaces = 10, spacing = 1, 3 spaces + VALUE(5) = 8
        assert_eq!(table.header(), "NAME          VALUE");
    }

    #[test]
    fn test_table_row() {
        let table = Table::new()
            .col_left("NAME", 10)
            .col_right("VALUE", 8)
            .spacing(1);

        // test(4) + 6 spaces = 10, spacing = 1, 5 spaces + 123(3) = 8
        assert_eq!(table.row(&["test", "123"]), "test            123");
    }

    #[test]
    fn test_table_with_prefix() {
        let table = Table::new()
            .prefix("  ")
            .col_left("A", 4)
            .col_right("B", 4)
            .spacing(1);

        assert_eq!(table.header(), "  A       B");
        assert_eq!(table.row(&["x", "y"]), "  x       y");
    }

    #[test]
    fn test_table_total_width() {
        let table = Table::new()
            .prefix(">>")
            .col_left("A", 5)
            .col_right("B", 5)
            .spacing(2);

        // prefix(2) + col_a(5) + spacing(2) + col_b(5) = 14
        assert_eq!(table.total_width(), 14);
    }

    #[test]
    fn test_column_offset() {
        let table = Table::new()
            .prefix("  ")
            .col_left("A", 4)
            .col_right("B", 6)
            .col_right("C", 8)
            .spacing(1);

        assert_eq!(table.column_offset(0), 2); // after prefix
        assert_eq!(table.column_offset(1), 7); // 2 + 4 + 1
        assert_eq!(table.column_offset(2), 14); // 2 + 4 + 1 + 6 + 1
    }

    #[test]
    fn test_multiple_rows() {
        let table = Table::new()
            .col_left("HOST", 8)
            .col_right("PING", 6)
            .spacing(1);

        let header = table.header();
        let row1 = table.row(&["server1", "3ms"]);
        let row2 = table.row(&["server2", "15ms"]);

        // HOST(4) + 4 spaces = 8, spacing = 1, 2 spaces + PING(4) = 6
        assert_eq!(header, "HOST       PING");
        // server1(7) + 1 space = 8, spacing = 1, 3 spaces + 3ms(3) = 6
        assert_eq!(row1, "server1     3ms");
        // server2(7) + 1 space = 8, spacing = 1, 2 spaces + 15ms(4) = 6
        assert_eq!(row2, "server2    15ms");
    }

    #[test]
    fn test_table_row_with_missing_values() {
        let table = Table::new()
            .col_left("NAME", 10)
            .col_right("VALUE", 8)
            .spacing(1);

        // Missing values should be filled with empty strings (no panic)
        // Empty values: 10 spaces + 1 space + 8 spaces = 19 spaces
        assert_eq!(table.row(&[]), "                   ");
        // One value: "only_one" padded to 10, spacing 1, 8 spaces for empty VALUE
        assert_eq!(table.row(&["only_one"]), "only_one           ");
        // Extra values are ignored: only first 2 values used
        assert_eq!(table.row(&["one", "two", "three"]), "one             two");
    }

    #[test]
    fn test_table_row_empty_values() {
        let table = Table::new().col_left("A", 4).col_right("B", 4).spacing(1);

        // Empty values array should not panic
        // Column 0: 4 spaces, spacing: 1 space, Column 1: 4 spaces = 9 total
        assert_eq!(table.row(&[]), "         ");
    }
}
