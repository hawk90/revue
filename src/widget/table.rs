//! Table widget for displaying tabular data

use super::traits::{RenderContext, View, WidgetProps};
use crate::render::Cell;
use crate::style::Color;
use crate::{impl_props_builders, impl_styled_view};

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
    selected: usize,
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
            selected: 0,
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
        self
    }

    /// Set rows data
    pub fn rows(mut self, rows: Vec<Vec<String>>) -> Self {
        self.rows = rows;
        self
    }

    /// Set selected row index
    pub fn selected(mut self, index: usize) -> Self {
        self.selected = index.min(self.rows.len().saturating_sub(1));
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
        self.selected
    }

    /// Get number of rows
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Select next row
    pub fn select_next(&mut self) {
        if !self.rows.is_empty() {
            self.selected = (self.selected + 1).min(self.rows.len() - 1);
        }
    }

    /// Select previous row
    pub fn select_prev(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    /// Select first row
    pub fn select_first(&mut self) {
        self.selected = 0;
    }

    /// Select last row
    pub fn select_last(&mut self) {
        if !self.rows.is_empty() {
            self.selected = self.rows.len() - 1;
        }
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
            self.render_border_line(ctx, area.x, y, &widths, '┌', '┬', '┐', '─');
            y += 1;
        }

        // Header row
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
            self.header_fg,
            self.header_bg,
            true,
        );
        y += 1;

        if self.border {
            // Header separator
            self.render_border_line(ctx, area.x, y, &widths, '├', '┼', '┤', '─');
            y += 1;
        }

        // Data rows
        for (i, row) in self.rows.iter().enumerate() {
            if y >= area.y + area.height - if self.border { 1 } else { 0 } {
                break;
            }

            let is_selected = i == self.selected;
            let (fg, bg) = if is_selected {
                (self.selected_fg, self.selected_bg)
            } else {
                (None, None)
            };

            self.render_row(ctx, area.x, y, &widths, row, fg, bg, false);
            y += 1;
        }

        if self.border {
            // Bottom border
            self.render_border_line(ctx, area.x, y, &widths, '└', '┴', '┘', '─');
        }
    }
}

impl Table {
    #[allow(clippy::too_many_arguments)]
    fn render_row(
        &self,
        ctx: &mut RenderContext,
        x: u16,
        y: u16,
        widths: &[u16],
        cells: &[String],
        fg: Option<Color>,
        bg: Option<Color>,
        bold: bool,
    ) {
        let mut cx = x;

        if self.border {
            let mut cell = Cell::new('│');
            cell.fg = fg;
            cell.bg = bg;
            ctx.buffer.set(cx, y, cell);
            cx += 1;
        }

        for (i, width) in widths.iter().enumerate() {
            let content = cells.get(i).map(|s| s.as_str()).unwrap_or("");
            let truncated: String = content.chars().take(*width as usize).collect();

            for (j, ch) in truncated.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = fg;
                cell.bg = bg;
                if bold {
                    cell.modifier |= crate::render::Modifier::BOLD;
                }
                ctx.buffer.set(cx + j as u16, y, cell);
            }

            // Fill remaining space
            for j in truncated.len()..(*width as usize) {
                let mut cell = Cell::new(' ');
                cell.fg = fg;
                cell.bg = bg;
                ctx.buffer.set(cx + j as u16, y, cell);
            }

            cx += width;

            if self.border {
                let mut cell = Cell::new('│');
                cell.fg = fg;
                cell.bg = bg;
                ctx.buffer.set(cx, y, cell);
                cx += 1;
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_border_line(
        &self,
        ctx: &mut RenderContext,
        x: u16,
        y: u16,
        widths: &[u16],
        left: char,
        mid: char,
        right: char,
        horiz: char,
    ) {
        let mut cx = x;

        ctx.buffer.set(cx, y, Cell::new(left));
        cx += 1;

        for (i, width) in widths.iter().enumerate() {
            for _ in 0..*width {
                ctx.buffer.set(cx, y, Cell::new(horiz));
                cx += 1;
            }
            if i < widths.len() - 1 {
                ctx.buffer.set(cx, y, Cell::new(mid));
                cx += 1;
            }
        }

        ctx.buffer.set(cx, y, Cell::new(right));
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
}
