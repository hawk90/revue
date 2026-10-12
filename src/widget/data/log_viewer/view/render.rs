//! LogViewer rendering: the `View` impl

use super::LogViewer;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::{char_width, display_width, truncate_to_width};
use crate::widget::data::log_viewer::entry::LogEntry;
use crate::widget::theme::{DARK_GRAY, DISABLED_FG};
use crate::widget::traits::{RenderContext, View};

/// Column widths of one row: the optional prefix columns, then the message
struct Columns {
    line_number: u16,
    bookmark: u16,
    timestamp: u16,
    level: u16,
    source: u16,
    message: u16,
}

impl Columns {
    fn new(viewer: &LogViewer, width: u16) -> Self {
        let line_number = if viewer.show_line_numbers { 6 } else { 0 };
        let bookmark = 2;
        let timestamp = if viewer.show_timestamps { 12 } else { 0 };
        let level = if viewer.show_levels { 5 } else { 0 };
        let source = if viewer.show_source { 12 } else { 0 };
        let prefix = line_number + bookmark + timestamp + level + source;
        Self {
            line_number,
            bookmark,
            timestamp,
            level,
            source,
            message: width.saturating_sub(prefix),
        }
    }

    fn timestamp_x(&self) -> u16 {
        self.line_number + self.bookmark
    }

    fn level_x(&self) -> u16 {
        self.timestamp_x() + self.timestamp
    }

    fn source_x(&self) -> u16 {
        self.level_x() + self.level
    }

    fn message_x(&self) -> u16 {
        self.source_x() + self.source
    }
}

/// One entry being drawn: its row, its index in `entries`, and its colors
struct Row<'a> {
    y: u16,
    entry_idx: usize,
    entry: &'a LogEntry,
    is_selected: bool,
    bg: Option<Color>,
}

/// A cell with `fg` and `bg`
fn cell(ch: char, fg: Color, bg: Option<Color>) -> Cell {
    let mut cell = Cell::new(ch);
    cell.fg = Some(fg);
    cell.bg = bg;
    cell
}

impl View for LogViewer {
    crate::impl_view_meta!("LogViewer");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let filtered: Vec<_> = self.filtered_entries().collect();

        if filtered.is_empty() {
            self.render_empty(ctx);
            return;
        }

        let columns = Columns::new(self, area.width);

        // Visible range
        let visible_height = area.height as usize;
        let start = self.scroll.min(filtered.len().saturating_sub(1));
        let end = (start + visible_height).min(filtered.len());

        for (view_idx, (entry_idx, entry)) in
            filtered.iter().enumerate().skip(start).take(end - start)
        {
            let y = (view_idx - start) as u16;
            if y >= area.height {
                break;
            }
            let is_selected = view_idx == self.selected;
            let row = Row {
                y,
                entry_idx: *entry_idx,
                entry,
                is_selected,
                bg: if is_selected {
                    Some(self.selected_bg)
                } else {
                    self.bg
                },
            };
            self.render_row(ctx, &row, &columns);
        }

        if filtered.len() > visible_height && area.width > 0 && area.height > 0 {
            self.draw_scroll_indicator(ctx, filtered.len());
        }

        if self.tail_mode {
            self.draw_tail_indicator(ctx);
        }
    }
}

impl LogViewer {
    /// "No log entries", centered
    fn render_empty(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let msg = "No log entries";
        let x = (area.width.saturating_sub(display_width(msg) as u16)) / 2;
        let y = area.height / 2;
        let mut dx: u16 = 0;
        for ch in msg.chars() {
            let cw = char_width(ch) as u16;
            ctx.set(x + dx, y, cell(ch, DISABLED_FG, self.bg));
            dx += cw;
        }
    }

    /// One entry on row `y`: the background, the prefix columns, the message
    fn render_row(&self, ctx: &mut RenderContext, row: &Row, columns: &Columns) {
        let (y, entry, row_bg) = (row.y, row.entry, row.bg);

        for x in 0..ctx.area.width {
            let mut blank = Cell::new(' ');
            blank.bg = row_bg;
            ctx.set(x, y, blank);
        }

        let mut x = 0u16;
        if self.show_line_numbers {
            let num_str = format!("{:>5}", entry.line_number);
            for ch in num_str.chars() {
                ctx.set(x, y, cell(ch, self.line_number_fg, row_bg));
                x += 1;
            }
            x += 1; // Space after line number
        }

        let bookmark_char = if entry.bookmarked { '★' } else { ' ' };
        ctx.set(x, y, cell(bookmark_char, self.bookmark_fg, row_bg));
        x += columns.bookmark;

        if self.show_timestamps {
            if let Some(ref ts) = entry.timestamp {
                let ts_display = truncate_to_width(ts, columns.timestamp as usize - 1);
                self.draw_chars(ctx, x, y, ts_display, self.timestamp_fg, row_bg);
            }
            x = columns.level_x();
        }

        if self.show_levels {
            let level_color = entry.level.color();
            ctx.set(x, y, cell(entry.level.icon(), level_color, row_bg));
            x += 1;
            for ch in entry.level.label().chars().take(3) {
                let mut label = cell(ch, level_color, row_bg);
                label.modifier |= Modifier::BOLD;
                ctx.set(x, y, label);
                x += 1;
            }
            x = columns.source_x();
        }

        if self.show_source {
            if let Some(ref src) = entry.source {
                let src_display = truncate_to_width(src, columns.source as usize - 1);
                self.draw_chars(ctx, x, y, src_display, self.source_fg, row_bg);
            }
            x = columns.message_x();
        }

        self.draw_message(ctx, x, row, columns.message);
    }

    /// `text` from `x`, one cell per character, advancing by its width
    fn draw_chars(
        &self,
        ctx: &mut RenderContext,
        x: u16,
        y: u16,
        text: &str,
        fg: Color,
        bg: Option<Color>,
    ) {
        let mut x = x;
        for ch in text.chars() {
            ctx.set(x, y, cell(ch, fg, bg));
            x += char_width(ch) as u16;
        }
    }

    /// The message from `x`, cut at `width`, with the search matches
    /// highlighted
    fn draw_message(&self, ctx: &mut RenderContext, x: u16, row: &Row, width: u16) {
        let (y, entry, is_selected, row_bg) = (row.y, row.entry, row.is_selected, row.bg);
        let level_color = entry.level.color();
        let matches_for_entry: Vec<_> = self
            .search_matches
            .iter()
            .filter(|m| m.entry_index == row.entry_idx)
            .collect();

        // Search match indices are byte offsets from the find() call in update_search
        let mut col_dx: u16 = 0;
        let mut byte_pos: usize = 0;
        for ch in entry.message.chars() {
            let cw = char_width(ch) as u16;
            if col_dx + cw > width {
                break;
            }
            let in_match = matches_for_entry
                .iter()
                .any(|m| byte_pos >= m.start && byte_pos < m.end);

            let mut cell = Cell::new(ch);
            cell.fg = Some(if is_selected {
                Color::WHITE
            } else {
                level_color
            });
            cell.bg = if in_match {
                Some(self.search_highlight_bg)
            } else {
                row_bg
            };
            if in_match {
                cell.fg = Some(Color::BLACK);
                cell.modifier |= Modifier::BOLD;
            }
            if is_selected {
                cell.modifier |= Modifier::BOLD;
            }
            ctx.set(x + col_dx, y, cell);
            col_dx += cw;
            byte_pos += ch.len_utf8();
        }
    }

    /// The one-cell scroll position marker in the last column
    fn draw_scroll_indicator(&self, ctx: &mut RenderContext, total: usize) {
        let area = ctx.area;
        let visible_height = area.height as usize;
        let scroll_ratio = self.scroll as f32 / (total - visible_height) as f32;
        let indicator_pos = (scroll_ratio * (area.height as f32 - 1.0)) as u16;
        let indicator_y = indicator_pos.min(area.height - 1);

        let mut marker = Cell::new('█');
        marker.fg = Some(DARK_GRAY);
        ctx.set(area.width - 1, indicator_y, marker);
    }

    /// "◉ TAIL" at the top right
    fn draw_tail_indicator(&self, ctx: &mut RenderContext) {
        let indicator = "◉ TAIL";
        let indicator_w = display_width(indicator) as u16;
        let x = ctx.area.width.saturating_sub(indicator_w + 2);
        let mut dx: u16 = 0;
        for ch in indicator.chars() {
            let cw = char_width(ch) as u16;
            ctx.set(x + dx, 0, cell(ch, Color::GREEN, self.bg));
            dx += cw;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_render_in_zero_sized_area() {
        let mut viewer = LogViewer::new();
        viewer.push("first");
        viewer.push("second");
        for (w, h) in [(40, 0), (0, 1), (0, 0)] {
            let mut buffer = Buffer::new(40, 5);
            let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
            viewer.render(&mut ctx);
        }
    }
}
