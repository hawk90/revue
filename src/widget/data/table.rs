//! Table widget for displaying tabular data

use crate::render::Cell;
use crate::style::Color;
use crate::utils::{char_width, truncate_to_width, Selection};
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
#[derive(Clone)]
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
    /// Explicitly enable virtual scroll
    virtual_scroll: bool,
    /// Auto-enable virtual scroll when row count exceeds this threshold
    virtual_threshold: usize,
    /// Extra rows to render above/below the visible viewport
    overscan: usize,
    /// Show scrollbar when virtual scrolling is active
    show_scrollbar: bool,
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
            virtual_scroll: false,
            virtual_threshold: 100,
            overscan: 5,
            show_scrollbar: true,
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

    /// Show a scrollbar for large datasets
    ///
    /// Every table scrolls to keep the selected row visible; this adds the
    /// scrollbar column. It switches itself on at 100 rows.
    pub fn virtual_scroll(mut self, enabled: bool) -> Self {
        self.virtual_scroll = enabled;
        self
    }

    /// Set overscan rows
    ///
    /// Stored but currently has no effect: rows outside the viewport are
    /// never drawn.
    pub fn overscan(mut self, rows: usize) -> Self {
        self.overscan = rows;
        self
    }

    /// Show/hide scrollbar when virtual scrolling
    pub fn show_scrollbar(mut self, show: bool) -> Self {
        self.show_scrollbar = show;
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

    /// Page down (move by viewport rows)
    pub fn page_down(&mut self, viewport_rows: usize) {
        let target = (self.selection.index + viewport_rows).min(self.rows.len().saturating_sub(1));
        self.selection.set(target);
    }

    /// Page up (move by viewport rows)
    pub fn page_up(&mut self, viewport_rows: usize) {
        let target = self.selection.index.saturating_sub(viewport_rows);
        self.selection.set(target);
    }

    /// Jump to a specific row index
    pub fn jump_to(&mut self, index: usize) {
        let clamped = index.min(self.rows.len().saturating_sub(1));
        self.selection.set(clamped);
    }

    /// Check if virtual scrolling is currently active
    fn is_virtual_active(&self) -> bool {
        self.virtual_scroll || self.rows.len() >= self.virtual_threshold
    }

    /// Calculate column widths
    fn calculate_widths(&self, available_width: u16) -> Vec<u16> {
        let col_count = self.columns.len();
        if col_count == 0 {
            return Vec::new();
        }

        // Reserve space for scrollbar if virtual scrolling is active
        let scrollbar_width = if self.is_virtual_active() && self.show_scrollbar {
            1u16
        } else {
            0
        };

        // Calculate fixed and auto columns
        let mut widths: Vec<u16> = self.columns.iter().map(|c| c.width).collect();
        let fixed_total = widths.iter().fold(0u16, |sum, &w| sum.saturating_add(w));
        let auto_count = widths.iter().filter(|&&w| w == 0).count() as u16;

        // Distribute remaining space to auto columns
        let border_space = if self.border {
            u16::try_from(col_count).map_or(u16::MAX, |n| n.saturating_add(1))
        } else {
            0
        };
        let remaining = available_width
            .saturating_sub(fixed_total)
            .saturating_sub(border_space)
            .saturating_sub(scrollbar_width);

        if let Some(auto_width) = remaining.checked_div(auto_count) {
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
        let mut y = self.render_header(ctx, &widths);

        let max_data_y = area.height - if self.border { 1 } else { 0 };
        let viewport_rows = max_data_y.saturating_sub(y) as usize;

        // Every table scrolls to keep the selection visible; virtual scroll
        // only adds the scrollbar.
        self.selection.set_visible(viewport_rows);
        let range = self.selection.visible_range();
        if self.is_virtual_active() && self.show_scrollbar && self.rows.len() > viewport_rows {
            self.render_scrollbar(ctx, y, viewport_rows);
        }
        for i in range {
            self.render_row(
                ctx,
                0,
                y,
                &widths,
                &self.rows[i],
                &self.data_row_style(ctx, i),
            );
            y += 1;
        }

        if self.border {
            let bottom_border = BorderChars {
                left: '└',
                mid: '┴',
                right: '┘',
                horiz: '─',
            };
            self.render_border_line(ctx, 0, y, &widths, &bottom_border);
        }
    }
}

impl Table {
    /// Draw the top border, header row and header separator; returns the
    /// first row below them.
    fn render_header(&self, ctx: &mut RenderContext, widths: &[u16]) -> u16 {
        let mut y = 0u16;
        if self.border {
            let top_border = BorderChars {
                left: '┌',
                mid: '┬',
                right: '┐',
                horiz: '─',
            };
            self.render_border_line(ctx, 0, y, widths, &top_border);
            y += 1;
        }

        let header_style = RowStyle {
            fg: self.header_fg,
            bg: self.header_bg,
            bold: true,
        };
        let titles: Vec<String> = self.columns.iter().map(|c| c.title.clone()).collect();
        self.render_row(ctx, 0, y, widths, &titles, &header_style);
        y += 1;

        if self.border {
            let sep_border = BorderChars {
                left: '├',
                mid: '┼',
                right: '┤',
                horiz: '─',
            };
            self.render_border_line(ctx, 0, y, widths, &sep_border);
            y += 1;
        }
        y
    }

    fn data_row_style(&self, ctx: &RenderContext, index: usize) -> RowStyle {
        let (fg, bg) = if self.selection.is_selected(index) {
            (self.selected_fg, self.selected_bg)
        } else {
            // These were `(None, None)`, so a rule matching the table
            // never reached its rows.
            (ctx.css_color_if_set(), ctx.css_background_if_set())
        };
        RowStyle {
            fg,
            bg,
            bold: false,
        }
    }

    fn render_scrollbar(&self, ctx: &mut RenderContext, top: u16, viewport_rows: usize) {
        let scrollbar_x = ctx.area.width - 1;
        let track_height = viewport_rows as f32;
        let total = self.rows.len() as f32;
        let thumb_size = ((track_height / total) * track_height).max(1.0) as u16;
        let scroll = self.selection.offset();
        let max_scroll = self.rows.len().saturating_sub(viewport_rows);
        let scroll_ratio = if max_scroll > 0 {
            scroll as f32 / max_scroll as f32
        } else {
            0.0
        };
        let thumb_pos =
            (scroll_ratio * (viewport_rows as u16).saturating_sub(thumb_size) as f32) as u16;

        for vy in 0..viewport_rows as u16 {
            let in_thumb = vy >= thumb_pos && vy < thumb_pos + thumb_size;
            let ch = if in_thumb { '█' } else { '░' };
            ctx.set(scrollbar_x, top + vy, Cell::new(ch));
        }
    }

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
            ctx.set(cx, y, cell);
            cx += 1;
        }

        for (i, &width) in widths.iter().enumerate() {
            // Fixed widths can add up past the area; stop at its edge.
            if cx >= ctx.area.width {
                break;
            }
            let width = width.min(ctx.area.width - cx);
            let content = cells.get(i).map(|s| s.as_str()).unwrap_or("");
            let truncated = truncate_to_width(content, width as usize);

            let mut dx: u16 = 0;
            for ch in truncated.chars() {
                let cw = char_width(ch) as u16;
                if dx + cw > width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = style.fg;
                cell.bg = style.bg;
                if style.bold {
                    cell.modifier |= crate::render::Modifier::BOLD;
                }
                ctx.set(cx + dx, y, cell);
                dx += cw;
            }

            // Fill remaining space
            for j in dx..width {
                let mut cell = Cell::new(' ');
                cell.fg = style.fg;
                cell.bg = style.bg;
                ctx.set(cx + j, y, cell);
            }

            cx += width;

            if self.border {
                let mut cell = Cell::new('│');
                cell.fg = style.fg;
                cell.bg = style.bg;
                ctx.set(cx, y, cell);
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

        ctx.set(cx, y, Cell::new(chars.left));
        cx += 1;

        for (i, &width) in widths.iter().enumerate() {
            if cx >= ctx.area.width {
                return;
            }
            for _ in 0..width.min(ctx.area.width - cx) {
                ctx.set(cx, y, Cell::new(chars.horiz));
                cx += 1;
            }
            if i < widths.len() - 1 {
                ctx.set(cx, y, Cell::new(chars.mid));
                cx += 1;
            }
        }

        ctx.set(cx, y, Cell::new(chars.right));
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

    #[test]
    fn calculate_widths_shares_the_rest_among_auto_columns() {
        let t = Table::new(vec![
            Column::new("A").width(10),
            Column::new("B"),
            Column::new("C"),
        ]);
        // 30 - 10 fixed - 4 border cells = 16, split between two auto columns
        assert_eq!(t.calculate_widths(30), [10, 8, 8]);
        assert_eq!(t.clone().border(false).calculate_widths(30), [10, 10, 10]);
    }

    #[test]
    fn calculate_widths_keeps_auto_columns_one_cell_wide_when_full() {
        let t = Table::new(vec![Column::new("A").width(u16::MAX), Column::new("B")]);
        assert_eq!(t.calculate_widths(20), [u16::MAX, 1]);
    }

    #[test]
    fn calculate_widths_reserves_the_scrollbar_column() {
        let t = Table::new(vec![Column::new("A")])
            .border(false)
            .virtual_scroll(true);
        assert_eq!(t.calculate_widths(10), [9]);
        assert_eq!(t.show_scrollbar(false).calculate_widths(10), [10]);
    }
}
