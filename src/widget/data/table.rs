//! Table widget for displaying tabular data

use crate::render::Cell;
use crate::style::Color;
use crate::utils::Selection;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Row rendering style configuration
struct RowStyle {
    fg: Option<Color>,
    bg: Option<Color>,
    bold: bool,
}

/// Border line characters
struct BorderChars {
    left: char,
    mid: char,
    right: char,
    horiz: char,
}

/// Column definition
#[derive(Clone)]
pub struct Column {
    /// Column title
    pub title: String,
    /// Column width (0 = auto)
    pub width: u16,
}

impl Column {
    /// Create a new column
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            width: 0,
        }
    }

    /// Set fixed width
    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }
}

/// A table widget for displaying rows and columns
pub struct Table {
    columns: Vec<Column>,
    rows: Vec<Vec<String>>,
    selection: Selection,
    header_fg: Option<Color>,
    header_bg: Option<Color>,
    selected_fg: Option<Color>,
    selected_bg: Option<Color>,
    border: bool,
    props: WidgetProps,
}

impl Table {
    /// Create a new table with columns
    pub fn new(columns: Vec<Column>) -> Self {
        Self {
            columns,
            rows: Vec::new(),
            selection: Selection::new(0),
            header_fg: Some(Color::WHITE),
            header_bg: None,
            selected_fg: Some(Color::WHITE),
            selected_bg: Some(Color::BLUE),
            border: true,
            props: WidgetProps::new(),
        }
    }

    /// Add a row of data
    pub fn row(mut self, cells: Vec<impl Into<String>>) -> Self {
        self.rows
            .push(cells.into_iter().map(|c| c.into()).collect());
        self.selection.set_len(self.rows.len());
        self
    }

    /// Set rows data
    pub fn rows(mut self, rows: Vec<Vec<String>>) -> Self {
        self.rows = rows;
        self.selection.set_len(self.rows.len());
        self
    }

    /// Set selected row index
    pub fn selected(mut self, index: usize) -> Self {
        self.selection.set(index);
        self
    }

    /// Set header colors
    pub fn header_style(mut self, fg: Color, bg: Option<Color>) -> Self {
        self.header_fg = Some(fg);
        self.header_bg = bg;
        self
    }

    /// Set selected row colors
    pub fn selected_style(mut self, fg: Color, bg: Color) -> Self {
        self.selected_fg = Some(fg);
        self.selected_bg = Some(bg);
        self
    }

    /// Enable/disable border
    pub fn border(mut self, enabled: bool) -> Self {
        self.border = enabled;
        self
    }

    /// Get selected index
    pub fn selected_index(&self) -> usize {
        self.selection.index
    }

    /// Get number of rows
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Select next row (no wrap)
    pub fn select_next(&mut self) {
        self.selection.down();
    }

    /// Select previous row (no wrap)
    pub fn select_prev(&mut self) {
        self.selection.up();
    }

    /// Select first row
    pub fn select_first(&mut self) {
        self.selection.first();
    }

    /// Select last row
    pub fn select_last(&mut self) {
        self.selection.last();
    }

    /// Calculate column widths
    fn calculate_widths(&self, available_width: u16) -> Vec<u16> {
        let col_count = self.columns.len();
        if col_count == 0 {
            return Vec::new();
        }

        // Calculate fixed and auto columns
        let mut widths: Vec<u16> = self.columns.iter().map(|c| c.width).collect();
        let fixed_total: u16 = widths.iter().filter(|&&w| w > 0).sum();
        let auto_count = widths.iter().filter(|&&w| w == 0).count() as u16;

        // Distribute remaining space to auto columns
        let border_space = if self.border { col_count as u16 + 1 } else { 0 };
        let remaining = available_width
            .saturating_sub(fixed_total)
            .saturating_sub(border_space);

        if auto_count > 0 {
            let auto_width = remaining / auto_count;
            for w in &mut widths {
                if *w == 0 {
                    *w = auto_width.max(1);
                }
            }
        }

        widths
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl View for Table {
    crate::impl_view_meta!("Table");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 3 || area.height < 2 || self.columns.is_empty() {
            return;
        }

        let widths = self.calculate_widths(area.width);
        let mut y = area.y;

        // Render header
        if self.border {
            // Top border
            let top_border = BorderChars {
                left: '┌',
                mid: '┬',
                right: '┐',
                horiz: '─',
            };
            self.render_border_line(ctx, area.x, y, &widths, &top_border);
            y += 1;
        }

        // Header row
        let header_style = RowStyle {
            fg: self.header_fg,
            bg: self.header_bg,
            bold: true,
        };
        self.render_row(
            ctx,
            area.x,
            y,
            &widths,
            &self
                .columns
                .iter()
                .map(|c| c.title.clone())
                .collect::<Vec<_>>(),
            &header_style,
        );
        y += 1;

        if self.border {
            // Header separator
            let sep_border = BorderChars {
                left: '├',
                mid: '┼',
                right: '┤',
                horiz: '─',
            };
            self.render_border_line(ctx, area.x, y, &widths, &sep_border);
            y += 1;
        }

        // Data rows
        for (i, row) in self.rows.iter().enumerate() {
            if y >= area.y + area.height - if self.border { 1 } else { 0 } {
                break;
            }

            let is_selected = self.selection.is_selected(i);
            let (fg, bg) = if is_selected {
                (self.selected_fg, self.selected_bg)
            } else {
                (None, None)
            };

            let row_style = RowStyle {
                fg,
                bg,
                bold: false,
            };
            self.render_row(ctx, area.x, y, &widths, row, &row_style);
            y += 1;
        }

        if self.border {
            // Bottom border
            let bottom_border = BorderChars {
                left: '└',
                mid: '┴',
                right: '┘',
                horiz: '─',
            };
            self.render_border_line(ctx, area.x, y, &widths, &bottom_border);
        }
    }
}

impl Table {
    fn render_row(
        &self,
        ctx: &mut RenderContext,
        x: u16,
        y: u16,
        widths: &[u16],
        cells: &[String],
        style: &RowStyle,
    ) {
        let mut cx = x;

        if self.border {
            let mut cell = Cell::new('│');
            cell.fg = style.fg;
            cell.bg = style.bg;
            ctx.buffer.set(cx, y, cell);
            cx += 1;
        }

        for (i, width) in widths.iter().enumerate() {
            let content = cells.get(i).map(|s| s.as_str()).unwrap_or("");
            let truncated: String = content.chars().take(*width as usize).collect();

            for (j, ch) in truncated.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = style.fg;
                cell.bg = style.bg;
                if style.bold {
                    cell.modifier |= crate::render::Modifier::BOLD;
                }
                ctx.buffer.set(cx + j as u16, y, cell);
            }

            // Fill remaining space
            for j in truncated.len()..(*width as usize) {
                let mut cell = Cell::new(' ');
                cell.fg = style.fg;
                cell.bg = style.bg;
                ctx.buffer.set(cx + j as u16, y, cell);
            }

            cx += width;

            if self.border {
                let mut cell = Cell::new('│');
                cell.fg = style.fg;
                cell.bg = style.bg;
                ctx.buffer.set(cx, y, cell);
                cx += 1;
            }
        }
    }

    fn render_border_line(
        &self,
        ctx: &mut RenderContext,
        x: u16,
        y: u16,
        widths: &[u16],
        chars: &BorderChars,
    ) {
        let mut cx = x;

        ctx.buffer.set(cx, y, Cell::new(chars.left));
        cx += 1;

        for (i, width) in widths.iter().enumerate() {
            for _ in 0..*width {
                ctx.buffer.set(cx, y, Cell::new(chars.horiz));
                cx += 1;
            }
            if i < widths.len() - 1 {
                ctx.buffer.set(cx, y, Cell::new(chars.mid));
                cx += 1;
            }
        }

        ctx.buffer.set(cx, y, Cell::new(chars.right));
    }
}

impl_styled_view!(Table);
impl_props_builders!(Table);

/// Helper function to create a table
pub fn table(columns: Vec<Column>) -> Table {
    Table::new(columns)
}

/// Helper to create a column
pub fn column(title: impl Into<String>) -> Column {
    Column::new(title)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_table_new() {
        let t = Table::new(vec![Column::new("Name"), Column::new("Age")]);
        assert_eq!(t.columns.len(), 2);
        assert_eq!(t.row_count(), 0);
    }

    #[test]
    fn test_table_with_rows() {
        let t = Table::new(vec![Column::new("A"), Column::new("B")])
            .row(vec!["1", "2"])
            .row(vec!["3", "4"]);

        assert_eq!(t.row_count(), 2);
    }

    #[test]
    fn test_table_selection() {
        let mut t = Table::new(vec![Column::new("X")])
            .row(vec!["a"])
            .row(vec!["b"])
            .row(vec!["c"]);

        assert_eq!(t.selected_index(), 0);

        t.select_next();
        assert_eq!(t.selected_index(), 1);

        t.select_next();
        assert_eq!(t.selected_index(), 2);

        t.select_next(); // Should stay at last
        assert_eq!(t.selected_index(), 2);

        t.select_prev();
        assert_eq!(t.selected_index(), 1);

        t.select_first();
        assert_eq!(t.selected_index(), 0);

        t.select_last();
        assert_eq!(t.selected_index(), 2);
    }

    #[test]
    fn test_table_render() {
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let t = Table::new(vec![
            Column::new("Name").width(10),
            Column::new("Value").width(10),
        ])
        .row(vec!["Alice", "100"])
        .row(vec!["Bob", "200"]);

        t.render(&mut ctx);

        // Check top-left corner
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
        // Check header text
        assert_eq!(buffer.get(1, 1).unwrap().symbol, 'N');
    }

    #[test]
    fn test_column_builder() {
        let col = Column::new("Test").width(15);
        assert_eq!(col.title, "Test");
        assert_eq!(col.width, 15);
    }

    #[test]
    fn test_table_helpers() {
        let t = table(vec![column("A"), column("B")]).row(vec!["1", "2"]);

        assert_eq!(t.columns.len(), 2);
        assert_eq!(t.row_count(), 1);
    }

    #[test]
    fn test_table_no_wrap_navigation() {
        let mut t = Table::new(vec![Column::new("X")])
            .row(vec!["a"])
            .row(vec!["b"]);

        // At start, can't go up
        assert_eq!(t.selected_index(), 0);
        t.select_prev();
        assert_eq!(t.selected_index(), 0); // Stays at 0

        // At end, can't go down
        t.select_last();
        assert_eq!(t.selected_index(), 1);
        t.select_next();
        assert_eq!(t.selected_index(), 1); // Stays at 1
    }

    #[test]
    fn test_table_navigation_comprehensive() {
        let mut t = Table::new(vec![Column::new("X")])
            .row(vec!["a"])
            .row(vec!["b"])
            .row(vec!["c"]);

        // Start at first
        assert_eq!(t.selected_index(), 0);

        // Go to last
        t.select_last();
        assert_eq!(t.selected_index(), 2);

        // Go back to first
        t.select_first();
        assert_eq!(t.selected_index(), 0);

        // Navigate down twice
        t.select_next();
        t.select_next();
        assert_eq!(t.selected_index(), 2);

        // Navigate up once
        t.select_prev();
        assert_eq!(t.selected_index(), 1);
    }

    #[test]
    fn test_table_selected_index_with_rows() {
        let t = Table::new(vec![Column::new("Name")])
            .row(vec!["Alice"])
            .row(vec!["Bob"])
            .selected(1);

        assert_eq!(t.selected_index(), 1);
        assert_eq!(t.row_count(), 2);
    }

    #[test]
    fn test_table_empty() {
        let t = Table::new(vec![Column::new("X")]);
        assert_eq!(t.row_count(), 0);
    }

    #[test]
    fn test_table_single_row() {
        let mut t = Table::new(vec![Column::new("X")]).row(vec!["only"]);

        assert_eq!(t.selected_index(), 0);

        t.select_next();
        assert_eq!(t.selected_index(), 0); // Can't go further

        t.select_prev();
        assert_eq!(t.selected_index(), 0); // Can't go back
    }

    #[test]
    fn test_table_rows_builder() {
        let t = Table::new(vec![Column::new("A")]).rows(vec![
            vec!["1".into()],
            vec!["2".into()],
            vec!["3".into()],
        ]);

        assert_eq!(t.row_count(), 3);
    }

    #[test]
    fn test_table_render_empty() {
        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let t = Table::new(vec![Column::new("Header")]);
        t.render(&mut ctx);
        // Should not crash on empty table
    }

    #[test]
    fn test_table_selection_builder() {
        let t = Table::new(vec![Column::new("X")])
            .row(vec!["a"])
            .row(vec!["b"])
            .row(vec!["c"])
            .selected(2);

        assert_eq!(t.selected_index(), 2);
    }

    #[test]
    fn test_table_selection_bounds() {
        let t = Table::new(vec![Column::new("X")])
            .row(vec!["a"])
            .row(vec!["b"])
            .selected(10); // Out of bounds

        // Should be clamped to valid range
        assert!(t.selected_index() <= 1);
    }

    #[test]
    fn test_table_selected_style() {
        let t = Table::new(vec![Column::new("X")])
            .row(vec!["a"])
            .selected_style(Color::WHITE, Color::BLUE);

        assert_eq!(t.selected_fg, Some(Color::WHITE));
        assert_eq!(t.selected_bg, Some(Color::BLUE));
    }

    #[test]
    fn test_table_header_style() {
        let t = Table::new(vec![Column::new("X")]).header_style(Color::YELLOW, Some(Color::BLACK));

        assert_eq!(t.header_fg, Some(Color::YELLOW));
        assert_eq!(t.header_bg, Some(Color::BLACK));
    }

    #[test]
    fn test_table_border_toggle() {
        let t = Table::new(vec![Column::new("X")]).border(false);
        assert!(!t.border);

        let t2 = Table::new(vec![Column::new("X")]).border(true);
        assert!(t2.border);
    }

    // =========================================================================
    // Column Clone trait tests
    // =========================================================================

    #[test]
    fn test_column_clone_basic() {
        let col1 = Column::new("Test").width(10);
        let col2 = col1.clone();

        assert_eq!(col1.title, col2.title);
        assert_eq!(col1.width, col2.width);
    }

    #[test]
    fn test_column_clone_with_title() {
        let col1 = Column::new("Original Title");
        let col2 = col1.clone();

        assert_eq!(col2.title, "Original Title");
        // Modifying clone shouldn't affect original
        let col3 = Column::new(col2.title.clone()).width(20);
        assert_eq!(col1.width, 0);
        assert_eq!(col3.width, 20);
    }

    #[test]
    fn test_column_clone_with_width() {
        let col1 = Column::new("Test").width(15);
        let col2 = col1.clone();

        assert_eq!(col2.width, 15);

        let col3 = col2.width(25);
        assert_eq!(col1.width, 15);
        assert_eq!(col3.width, 25);
    }

    #[test]
    fn test_column_clone_empty() {
        let col1 = Column::new("");
        let col2 = col1.clone();

        assert_eq!(col2.title, "");
        assert_eq!(col2.width, 0);
    }

    // =========================================================================
    // Table Default trait tests
    // =========================================================================

    #[test]
    fn test_table_default() {
        let t = Table::default();
        assert_eq!(t.columns.len(), 0);
        assert_eq!(t.row_count(), 0);
        assert_eq!(t.selected_index(), 0);
        assert!(t.border);
        assert_eq!(t.header_fg, Some(Color::WHITE));
        assert_eq!(t.selected_bg, Some(Color::BLUE));
    }

    #[test]
    fn test_table_default_empty_columns() {
        let t = Table::default();
        assert!(t.columns.is_empty());
    }

    #[test]
    fn test_table_default_has_border() {
        let t = Table::default();
        assert!(t.border);
    }

    #[test]
    fn test_table_default_colors() {
        let t = Table::default();
        assert_eq!(t.header_fg, Some(Color::WHITE));
        assert_eq!(t.header_bg, None);
        assert_eq!(t.selected_fg, Some(Color::WHITE));
        assert_eq!(t.selected_bg, Some(Color::BLUE));
    }

    // =========================================================================
    // Column public field tests
    // =========================================================================

    #[test]
    fn test_column_public_fields_accessible() {
        let col = Column::new("Field Test").width(20);

        // Direct field access
        assert_eq!(col.title, "Field Test");
        assert_eq!(col.width, 20);
    }

    #[test]
    fn test_column_title_field() {
        let col = Column::new("Custom Title");
        assert_eq!(col.title, "Custom Title");
    }

    #[test]
    fn test_column_width_field_default() {
        let col = Column::new("Test");
        assert_eq!(col.width, 0);
    }

    #[test]
    fn test_column_width_field_set() {
        let col = Column::new("Test").width(100);
        assert_eq!(col.width, 100);
    }

    // =========================================================================
    // Table builder chain tests
    // =========================================================================

    #[test]
    fn test_table_full_builder_chain() {
        let t = Table::new(vec![Column::new("A"), Column::new("B")])
            .row(vec!["1", "2"])
            .selected(0)
            .header_style(Color::YELLOW, Some(Color::BLACK))
            .selected_style(Color::WHITE, Color::BLUE)
            .border(false);

        assert_eq!(t.row_count(), 1);
        assert_eq!(t.selected_index(), 0);
        assert_eq!(t.header_fg, Some(Color::YELLOW));
        assert_eq!(t.header_bg, Some(Color::BLACK));
        assert_eq!(t.selected_fg, Some(Color::WHITE));
        assert_eq!(t.selected_bg, Some(Color::BLUE));
        assert!(!t.border);
    }

    #[test]
    fn test_table_multiple_rows_builder() {
        let t = Table::new(vec![Column::new("X")])
            .row(vec!["a"])
            .row(vec!["b"])
            .row(vec!["c"])
            .row(vec!["d"]);

        assert_eq!(t.row_count(), 4);
    }

    // =========================================================================
    // Table rows method tests
    // =========================================================================

    #[test]
    fn test_table_rows_empty() {
        let t = Table::new(vec![Column::new("A")]).rows(vec![]);
        assert_eq!(t.row_count(), 0);
    }

    #[test]
    fn test_table_rows_multiple() {
        let t = Table::new(vec![Column::new("A"), Column::new("B")]).rows(vec![
            vec!["1".into(), "2".into()],
            vec!["3".into(), "4".into()],
            vec!["5".into(), "6".into()],
        ]);

        assert_eq!(t.row_count(), 3);
    }

    #[test]
    fn test_table_rows_with_string() {
        let t = Table::new(vec![Column::new("Name")])
            .rows(vec![vec![String::from("Alice")], vec![String::from("Bob")]]);

        assert_eq!(t.row_count(), 2);
    }

    #[test]
    fn test_table_rows_then_row() {
        let t = Table::new(vec![Column::new("X")])
            .rows(vec![vec!["a".into()], vec!["b".into()]])
            .row(vec!["c"]);

        assert_eq!(t.row_count(), 3);
    }

    // =========================================================================
    // Table navigation edge cases
    // =========================================================================

    #[test]
    fn test_table_navigation_empty() {
        let mut t = Table::new(vec![Column::new("X")]);

        t.select_next();
        t.select_prev();
        t.select_first();
        t.select_last();

        // Should not panic
        assert_eq!(t.selected_index(), 0);
    }

    #[test]
    fn test_table_select_first_resets_offset() {
        let mut t = Table::new(vec![Column::new("X")])
            .row(vec!["a"])
            .row(vec!["b"])
            .row(vec!["c"])
            .selected(2);

        t.select_first();
        assert_eq!(t.selected_index(), 0);
    }

    #[test]
    fn test_table_select_last_from_start() {
        let mut t = Table::new(vec![Column::new("X")])
            .row(vec!["a"])
            .row(vec!["b"])
            .row(vec!["c"]);

        t.select_last();
        assert_eq!(t.selected_index(), 2);
    }

    // =========================================================================
    // Table render edge cases
    // =========================================================================

    #[test]
    fn test_table_render_too_narrow() {
        let mut buffer = Buffer::new(2, 5);
        let area = Rect::new(0, 0, 2, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let t = Table::new(vec![Column::new("X")]).row(vec!["a"]);
        t.render(&mut ctx); // Should return early (width < 3)
    }

    #[test]
    fn test_table_render_too_short() {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let t = Table::new(vec![Column::new("X")]).row(vec!["a"]);
        t.render(&mut ctx); // Should return early (height < 2)
    }

    #[test]
    fn test_table_render_no_columns() {
        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let t = Table::new(vec![]);
        t.render(&mut ctx); // Should return early (no columns)
    }

    // =========================================================================
    // Column method tests
    // =========================================================================

    #[test]
    fn test_column_new_with_string() {
        let col = Column::new(String::from("Owned String"));
        assert_eq!(col.title, "Owned String");
    }

    #[test]
    fn test_column_new_empty_title() {
        let col = Column::new("");
        assert_eq!(col.title, "");
    }

    #[test]
    fn test_column_new_unicode_title() {
        let col = Column::new("🎉 Celebration");
        assert_eq!(col.title, "🎉 Celebration");
    }

    #[test]
    fn test_column_width_zero() {
        let col = Column::new("Test").width(0);
        assert_eq!(col.width, 0);
    }

    #[test]
    fn test_column_width_large() {
        let col = Column::new("Test").width(1000);
        assert_eq!(col.width, 1000);
    }

    #[test]
    fn test_column_builder_chain() {
        let col = Column::new("Title").width(20).width(30);
        assert_eq!(col.title, "Title");
        assert_eq!(col.width, 30); // Last width wins
    }

    // =========================================================================
    // Helper function tests
    // =========================================================================

    #[test]
    fn test_table_helper_empty() {
        let t = table(vec![]);
        assert_eq!(t.columns.len(), 0);
    }

    #[test]
    fn test_table_helper_with_columns() {
        let t = table(vec![Column::new("A").width(5), Column::new("B").width(10)]);

        assert_eq!(t.columns.len(), 2);
    }

    #[test]
    fn test_column_helper() {
        let col = column("Test Column");
        assert_eq!(col.title, "Test Column");
        assert_eq!(col.width, 0);
    }

    #[test]
    fn test_column_helper_with_string() {
        let col = column(String::from("Owned"));
        assert_eq!(col.title, "Owned");
    }

    #[test]
    fn test_column_helper_chainable() {
        let col = column("Chained").width(25);
        assert_eq!(col.title, "Chained");
        assert_eq!(col.width, 25);
    }

    // =========================================================================
    // Edge case tests
    // =========================================================================

    #[test]
    fn test_table_with_many_columns() {
        let cols: Vec<Column> = (0..10).map(|i| Column::new(format!("Col{}", i))).collect();
        let t = Table::new(cols);
        assert_eq!(t.columns.len(), 10);
    }

    #[test]
    fn test_table_with_many_rows() {
        let rows: Vec<Vec<String>> = (0..100).map(|i| vec![format!("Row{}", i)]).collect();

        let t = Table::new(vec![Column::new("X")]).rows(rows);
        assert_eq!(t.row_count(), 100);
    }

    #[test]
    fn test_table_with_empty_cells() {
        let t = Table::new(vec![Column::new("A"), Column::new("B")])
            .row(vec!["", ""])
            .row(vec!["x", ""]);

        assert_eq!(t.row_count(), 2);
    }

    #[test]
    fn test_table_unicode_content() {
        let t = Table::new(vec![Column::new("名前"), Column::new("값")]).row(vec!["テスト", "🎉"]);

        assert_eq!(t.row_count(), 1);
    }

    #[test]
    fn test_table_selected_valid_range() {
        let t = Table::new(vec![Column::new("X")])
            .row(vec!["a"])
            .row(vec!["b"])
            .selected(1);

        assert_eq!(t.selected_index(), 1);
    }

    #[test]
    fn test_table_select_first_no_rows() {
        let mut t = Table::new(vec![Column::new("X")]);
        t.select_first();
        // Should not panic
        assert_eq!(t.selected_index(), 0);
    }

    #[test]
    fn test_table_select_last_no_rows() {
        let mut t = Table::new(vec![Column::new("X")]);
        t.select_last();
        // Should not panic
        assert_eq!(t.selected_index(), 0);
    }

    #[test]
    fn test_table_header_style_no_bg() {
        let t = Table::new(vec![Column::new("X")]).header_style(Color::CYAN, None);

        assert_eq!(t.header_fg, Some(Color::CYAN));
        assert_eq!(t.header_bg, None);
    }
}
