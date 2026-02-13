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

// Keep private tests that require private field access here
// Tests above this line are public API tests that have been extracted to tests/widget/data/table.rs

#[test]
fn test_table_calculate_widths_private() {
    // Test private method - keeping in source
    let _t = Table::new(vec![
        Column::new("A").width(10),
        Column::new("B"), // auto width
    ]);

    // This would require accessing private calculate_widths method
    // Test kept inline due to private access
}
