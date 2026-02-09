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
#[derive(Clone, Debug)]
pub struct Table {
    /// Column definitions
    columns: Vec<Column>,
    /// Spacing between columns
    spacing: usize,
    /// Prefix before first column
    prefix: String,
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
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

    // =========================================================================
    // Align enum trait tests
    // =========================================================================

    #[test]
    fn test_align_default() {
        assert_eq!(Align::default(), Align::Left);
    }

    #[test]
    fn test_align_clone() {
        let align1 = Align::Center;
        let align2 = align1.clone();
        assert_eq!(align1, align2);
    }

    #[test]
    fn test_align_copy() {
        let a1 = Align::Right;
        let a2 = a1;
        assert_eq!(a1, Align::Right);
        assert_eq!(a2, Align::Right);
    }

    #[test]
    fn test_align_partial_eq() {
        assert_eq!(Align::Left, Align::Left);
        assert_eq!(Align::Center, Align::Center);
        assert_eq!(Align::Right, Align::Right);
        assert_ne!(Align::Left, Align::Right);
    }

    #[test]
    fn test_align_debug() {
        let debug_str = format!("{:?}", Align::Center);
        assert!(debug_str.contains("Center"));
    }

    // =========================================================================
    // Column Clone trait tests
    // =========================================================================

    #[test]
    fn test_column_clone_basic() {
        let col1 = Column::new("Test", 10, Align::Left);
        let col2 = col1.clone();

        assert_eq!(col1.header, col2.header);
        assert_eq!(col1.width, col2.width);
        assert_eq!(col1.align, col2.align);
    }

    #[test]
    fn test_column_clone_with_header() {
        let col1 = Column::new("Original", 15, Align::Center);
        let col2 = col1.clone();

        assert_eq!(col2.header, "Original");
    }

    #[test]
    fn test_column_clone_with_width() {
        let col1 = Column::new("Test", 20, Align::Right);
        let col2 = col1.clone();

        assert_eq!(col2.width, 20);
    }

    #[test]
    fn test_column_clone_with_align() {
        let col1 = Column::new("Test", 10, Align::Center);
        let col2 = col1.clone();

        assert_eq!(col2.align, Align::Center);
    }

    #[test]
    fn test_column_clone_all_alignments() {
        let col_left = Column::new("L", 5, Align::Left).clone();
        let col_center = Column::new("C", 5, Align::Center).clone();
        let col_right = Column::new("R", 5, Align::Right).clone();

        assert_eq!(col_left.align, Align::Left);
        assert_eq!(col_center.align, Align::Center);
        assert_eq!(col_right.align, Align::Right);
    }

    #[test]
    fn test_column_debug() {
        let col = Column::new("Debug", 10, Align::Left);
        let debug_str = format!("{:?}", col);
        assert!(debug_str.contains("Debug"));
    }

    // =========================================================================
    // Table Clone trait tests
    // =========================================================================

    #[test]
    fn test_table_clone_basic() {
        let table1 = Table::new().col_left("A", 5).col_right("B", 5).spacing(1);

        let table2 = table1.clone();

        assert_eq!(table1.column_count(), table2.column_count());
        assert_eq!(table1.spacing, table2.spacing);
    }

    #[test]
    fn test_table_clone_with_prefix() {
        let table1 = Table::new().prefix(">>").col_left("X", 4);

        let table2 = table1.clone();
        assert_eq!(table2.prefix, ">>");
    }

    #[test]
    fn test_table_clone_with_columns() {
        let table1 = Table::new()
            .col_left("Name", 10)
            .col_right("Value", 8)
            .spacing(2);

        let table2 = table1.clone();

        assert_eq!(table2.column_count(), 2);
        assert_eq!(table2.spacing, 2);
    }

    #[test]
    fn test_table_clone_preserves_state() {
        let table1 = Table::new().prefix("  ").col_center("Test", 10).spacing(1);

        let table2 = table1.clone();

        assert_eq!(table1.total_width(), table2.total_width());
        assert_eq!(table1.column_count(), table2.column_count());
    }

    #[test]
    fn test_table_debug() {
        let table = Table::new().col_left("A", 5);
        let debug_str = format!("{:?}", table);
        // Debug output should contain something meaningful
        assert!(!debug_str.is_empty());
    }

    // =========================================================================
    // Table Default trait tests
    // =========================================================================

    #[test]
    fn test_table_default() {
        let table = Table::default();
        assert_eq!(table.column_count(), 0);
        assert_eq!(table.spacing, 1);
        assert_eq!(table.prefix, "");
    }

    #[test]
    fn test_table_default_empty() {
        let table = Table::default();
        assert!(table.columns.is_empty());
    }

    #[test]
    fn test_table_default_spacing() {
        let table = Table::default();
        assert_eq!(table.spacing, 1);
    }

    #[test]
    fn test_table_default_prefix() {
        let table = Table::default();
        assert_eq!(table.prefix, "");
    }

    #[test]
    fn test_table_default_total_width() {
        let table = Table::default();
        assert_eq!(table.total_width(), 0);
    }

    // =========================================================================
    // Column::new tests
    // =========================================================================

    #[test]
    fn test_column_new_with_str() {
        let col = Column::new("Header", 10, Align::Left);
        assert_eq!(col.header, "Header");
        assert_eq!(col.width, 10);
        assert_eq!(col.align, Align::Left);
    }

    #[test]
    fn test_column_new_with_string() {
        let col = Column::new(String::from("Owned"), 15, Align::Center);
        assert_eq!(col.header, "Owned");
        assert_eq!(col.width, 15);
        assert_eq!(col.align, Align::Center);
    }

    #[test]
    fn test_column_new_zero_width() {
        let col = Column::new("Test", 0, Align::Right);
        assert_eq!(col.width, 0);
    }

    #[test]
    fn test_column_new_large_width() {
        let col = Column::new("Wide", 1000, Align::Left);
        assert_eq!(col.width, 1000);
    }

    #[test]
    fn test_column_new_all_alignments() {
        let col_left = Column::new("L", 5, Align::Left);
        let col_center = Column::new("C", 5, Align::Center);
        let col_right = Column::new("R", 5, Align::Right);

        assert_eq!(col_left.align, Align::Left);
        assert_eq!(col_center.align, Align::Center);
        assert_eq!(col_right.align, Align::Right);
    }

    // =========================================================================
    // Column::format tests
    // =========================================================================

    #[test]
    fn test_column_format_short_text() {
        let col = Column::new("Test", 10, Align::Left);
        assert_eq!(col.format("hi"), "hi        ");
    }

    #[test]
    fn test_column_format_exact_width() {
        let col = Column::new("Test", 5, Align::Left);
        assert_eq!(col.format("hello"), "hello");
    }

    #[test]
    fn test_column_format_long_text() {
        let col = Column::new("Test", 5, Align::Left);
        assert_eq!(col.format("hello world"), "hello");
    }

    #[test]
    fn test_column_format_right_align() {
        let col = Column::new("Test", 10, Align::Right);
        assert_eq!(col.format("hi"), "        hi");
    }

    #[test]
    fn test_column_format_center_align() {
        let col = Column::new("Test", 10, Align::Center);
        assert_eq!(col.format("hi"), "    hi    ");
    }

    // =========================================================================
    // Column::format_header tests
    // =========================================================================

    #[test]
    fn test_column_format_header_left() {
        let col = Column::new("Name", 10, Align::Left);
        assert_eq!(col.format_header(), "Name      ");
    }

    #[test]
    fn test_column_format_header_right() {
        let col = Column::new("Value", 10, Align::Right);
        assert_eq!(col.format_header(), "     Value");
    }

    #[test]
    fn test_column_format_header_center() {
        let col = Column::new("Title", 10, Align::Center);
        assert_eq!(col.format_header(), "  Title   ");
    }

    #[test]
    fn test_column_format_header_truncated() {
        let col = Column::new("Very Long Header", 5, Align::Left);
        assert_eq!(col.format_header(), "Very ");
    }

    // =========================================================================
    // Table::new tests
    // =========================================================================

    #[test]
    fn test_table_new_empty() {
        let table = Table::new();
        assert_eq!(table.column_count(), 0);
    }

    #[test]
    fn test_table_new_default_spacing() {
        let table = Table::new();
        assert_eq!(table.spacing, 1);
    }

    #[test]
    fn test_table_new_default_prefix() {
        let table = Table::new();
        assert_eq!(table.prefix, "");
    }

    // =========================================================================
    // Table::col tests
    // =========================================================================

    #[test]
    fn test_table_col_left() {
        let table = Table::new().col_left("A", 5);
        assert_eq!(table.column_count(), 1);
    }

    #[test]
    fn test_table_col_right() {
        let table = Table::new().col_right("B", 5);
        assert_eq!(table.column_count(), 1);
    }

    #[test]
    fn test_table_col_center() {
        let table = Table::new().col_center("C", 5);
        assert_eq!(table.column_count(), 1);
    }

    #[test]
    fn test_table_col_multiple() {
        let table = Table::new()
            .col("A", 5, Align::Left)
            .col("B", 5, Align::Right);

        assert_eq!(table.column_count(), 2);
    }

    #[test]
    fn test_table_col_with_string() {
        let table = Table::new().col(String::from("Owned"), 10, Align::Left);
        assert_eq!(table.column_count(), 1);
    }

    // =========================================================================
    // Table::spacing tests
    // =========================================================================

    #[test]
    fn test_table_spacing_zero() {
        let table = Table::new().spacing(0);
        assert_eq!(table.spacing, 0);
    }

    #[test]
    fn test_table_spacing_multiple() {
        let table = Table::new().spacing(5);
        assert_eq!(table.spacing, 5);
    }

    #[test]
    fn test_table_spacing_chain() {
        let table = Table::new().spacing(2).col("A", 5, Align::Left);

        assert_eq!(table.spacing, 2);
    }

    // =========================================================================
    // Table::prefix tests
    // =========================================================================

    #[test]
    fn test_table_prefix_empty() {
        let table = Table::new().prefix("");
        assert_eq!(table.prefix, "");
    }

    #[test]
    fn test_table_prefix_spaces() {
        let table = Table::new().prefix("  ");
        assert_eq!(table.prefix, "  ");
    }

    #[test]
    fn test_table_prefix_string() {
        let table = Table::new().prefix(String::from(">>"));
        assert_eq!(table.prefix, ">>");
    }

    #[test]
    fn test_table_prefix_unicode() {
        let table = Table::new().prefix("→");
        assert_eq!(table.prefix, "→");
    }

    // =========================================================================
    // Table::column_count tests
    // =========================================================================

    #[test]
    fn test_table_column_count_empty() {
        let table = Table::new();
        assert_eq!(table.column_count(), 0);
    }

    #[test]
    fn test_table_column_count_single() {
        let table = Table::new().col("A", 5, Align::Left);
        assert_eq!(table.column_count(), 1);
    }

    #[test]
    fn test_table_column_count_many() {
        let table = Table::new()
            .col("A", 1, Align::Left)
            .col("B", 1, Align::Left)
            .col("C", 1, Align::Left)
            .col("D", 1, Align::Left)
            .col("E", 1, Align::Left);

        assert_eq!(table.column_count(), 5);
    }

    // =========================================================================
    // Table::get_column tests
    // =========================================================================

    #[test]
    fn test_table_get_column_first() {
        let table = Table::new().col("First", 10, Align::Left);
        let col = table.get_column(0);

        assert!(col.is_some());
        assert_eq!(col.unwrap().header, "First");
    }

    #[test]
    fn test_table_get_column_middle() {
        let table = Table::new()
            .col("A", 5, Align::Left)
            .col("Middle", 10, Align::Center)
            .col("B", 5, Align::Left);

        let col = table.get_column(1);
        assert!(col.is_some());
        assert_eq!(col.unwrap().header, "Middle");
    }

    #[test]
    fn test_table_get_column_out_of_bounds() {
        let table = Table::new().col("A", 5, Align::Left);
        assert!(table.get_column(10).is_none());
    }

    #[test]
    fn test_table_get_column_empty_table() {
        let table = Table::new();
        assert!(table.get_column(0).is_none());
    }

    // =========================================================================
    // Table::column_offset tests
    // =========================================================================

    #[test]
    fn test_table_column_offset_first_no_prefix() {
        let table = Table::new().col_left("A", 5);
        assert_eq!(table.column_offset(0), 0);
    }

    #[test]
    fn test_table_column_offset_first_with_prefix() {
        let table = Table::new().prefix(">>").col_left("A", 5);
        assert_eq!(table.column_offset(0), 2);
    }

    #[test]
    fn test_table_column_offset_second() {
        let table = Table::new().col_left("A", 5).col_left("B", 5).spacing(1);

        // First column: 5 chars + 1 spacing = 6
        assert_eq!(table.column_offset(1), 6);
    }

    #[test]
    fn test_table_column_offset_out_of_bounds() {
        let table = Table::new().col_left("A", 5);
        // Returns offset after last column
        let offset = table.column_offset(10);
        assert!(offset >= 5);
    }

    #[test]
    fn test_table_column_offset_with_prefix_and_spacing() {
        let table = Table::new()
            .prefix("  ")
            .col_left("A", 4)
            .col_left("B", 6)
            .col_left("C", 8)
            .spacing(2);

        assert_eq!(table.column_offset(0), 2); // after prefix
        assert_eq!(table.column_offset(1), 8); // 2 + 4 + 2
        assert_eq!(table.column_offset(2), 16); // 2 + 4 + 2 + 6 + 2
    }

    // =========================================================================
    // align_text function edge cases
    // =========================================================================

    #[test]
    fn test_align_text_empty_string() {
        assert_eq!(align_text("", 5, Align::Left), "     ");
    }

    #[test]
    fn test_align_text_zero_width() {
        assert_eq!(align_text("test", 0, Align::Left), "");
    }

    #[test]
    fn test_align_text_unicode_wide_chars() {
        // Test with wide unicode characters
        let result = align_text("ab", 4, Align::Left);
        // Should pad to width 4
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_align_text_truncates_long_text() {
        let result = align_text("hello world", 5, Align::Left);
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_align_text_exact_fit() {
        assert_eq!(align_text("hello", 5, Align::Left), "hello");
    }

    // =========================================================================
    // Convenience function tests
    // =========================================================================

    #[test]
    fn test_align_left_empty() {
        assert_eq!(align_left("", 5), "     ");
    }

    #[test]
    fn test_align_right_empty() {
        assert_eq!(align_right("", 5), "     ");
    }

    #[test]
    fn test_align_center_empty() {
        assert_eq!(align_center("", 5), "     ");
    }

    #[test]
    fn test_align_left_unicode() {
        // "テスト" has 3 wide chars = 6 display columns, 10 - 6 = 4 spaces
        assert_eq!(align_left("テスト", 10), "テスト    ");
    }

    #[test]
    fn test_align_right_unicode() {
        // "テスト" has 3 wide chars = 6 display columns, 10 - 6 = 4 spaces
        assert_eq!(align_right("テスト", 10), "    テスト");
    }

    #[test]
    fn test_align_center_unicode() {
        // "テ" has 1 wide char = 2 columns, width 5, so padding = 3
        // left_pad = 3/2 = 1, right_pad = 3-1 = 2
        assert_eq!(align_center("テ", 5), " テ  ");
    }

    // =========================================================================
    // Table::row_owned tests
    // =========================================================================

    #[test]
    fn test_table_row_owned_basic() {
        let table = Table::new().col_left("A", 5).col_right("B", 5);
        let result = table.row_owned(&vec![String::from("x"), String::from("y")]);
        // Column A (width 5, left): "x    " (x + 4 spaces)
        // Spacing (1): " " (1 space)
        // Column B (width 5, right): "    y" (4 spaces + y)
        // Total: "x    " + " " + "    y" = "x         y" (11 chars)
        assert_eq!(result, "x         y");
    }

    #[test]
    fn test_table_row_owned_empty() {
        let table = Table::new().col_left("A", 5);
        let result = table.row_owned(&vec![]);
        assert_eq!(result, "     ");
    }

    #[test]
    fn test_table_row_owned_extra_values() {
        let table = Table::new().col_left("A", 5);
        let result = table.row_owned(&vec![
            String::from("x"),
            String::from("y"),
            String::from("z"),
        ]);
        // Extra values should be ignored
        // Column A (width 5, left): "x    " (x + 4 spaces = 5 chars)
        assert_eq!(result, "x    ");
    }

    // =========================================================================
    // Edge case tests
    // =========================================================================

    #[test]
    fn test_table_total_width_with_prefix() {
        let table = Table::new()
            .prefix(">>>")
            .col_left("A", 5)
            .col_left("B", 5)
            .spacing(2);

        // prefix(3) + col_a(5) + spacing(2) + col_b(5) = 15
        assert_eq!(table.total_width(), 15);
    }

    #[test]
    fn test_table_total_width_no_spacing_single_col() {
        let table = Table::new().col_left("A", 10);
        assert_eq!(table.total_width(), 10);
    }

    #[test]
    fn test_table_with_all_alignment_types() {
        let table = Table::new()
            .col_left("Left", 10)
            .col_center("Center", 10)
            .col_right("Right", 10)
            .spacing(1);

        assert_eq!(table.column_count(), 3);
        assert_eq!(table.total_width(), 32); // 10 + 1 + 10 + 1 + 10 = 32
    }

    #[test]
    fn test_column_with_unicode_header() {
        let col = Column::new("🎉 Celebrate", 15, Align::Left);
        assert_eq!(col.header, "🎉 Celebrate");
    }

    #[test]
    fn test_table_row_with_unicode_values() {
        let table = Table::new()
            .col("Name", 10, Align::Left)
            .col("Value", 10, Align::Right);

        let result = table.row(&["テスト", "値"]);
        // Should not panic and produce some output
        assert!(!result.is_empty());
    }
}
