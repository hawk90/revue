//! LogViewer rendering: the `View` impl

use super::LogViewer;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::{char_width, display_width, truncate_to_width};
use crate::widget::theme::{DARK_GRAY, DISABLED_FG};
use crate::widget::traits::{RenderContext, View};

impl View for LogViewer {
    crate::impl_view_meta!("LogViewer");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let filtered: Vec<_> = self.filtered_entries().collect();

        if filtered.is_empty() {
            // Show empty message
            let msg = "No log entries";
            let x = (area.width.saturating_sub(display_width(msg) as u16)) / 2;
            let y = area.height / 2;
            let mut dx: u16 = 0;
            for ch in msg.chars() {
                let cw = char_width(ch) as u16;
                let mut cell = Cell::new(ch);
                cell.fg = Some(DISABLED_FG);
                cell.bg = self.bg;
                ctx.set(x + dx, y, cell);
                dx += cw;
            }
            return;
        }

        // Calculate column widths
        let line_num_width = if self.show_line_numbers { 6 } else { 0 };
        let timestamp_width = if self.show_timestamps { 12 } else { 0 };
        let level_width = if self.show_levels { 5 } else { 0 };
        let source_width = if self.show_source { 12 } else { 0 };
        let bookmark_width = 2;

        let prefix_width =
            line_num_width + bookmark_width + timestamp_width + level_width + source_width;
        let message_width = area.width.saturating_sub(prefix_width);

        // Visible range
        let visible_height = area.height as usize;
        let start = self.scroll.min(filtered.len().saturating_sub(1));
        let end = (start + visible_height).min(filtered.len());

        for (view_idx, (entry_idx, entry)) in
            filtered.iter().enumerate().skip(start).take(end - start)
        {
            let row = (view_idx - start) as u16;
            let y = row;

            if y >= area.height {
                break;
            }

            let is_selected = view_idx == self.selected;
            let level_color = entry.level.color();

            // Fill background
            let row_bg = if is_selected {
                Some(self.selected_bg)
            } else {
                self.bg
            };

            for x in 0..area.width {
                let mut cell = Cell::new(' ');
                cell.bg = row_bg;
                ctx.set(x, y, cell);
            }

            let mut x = 0u16;

            // Draw line number
            if self.show_line_numbers {
                let num_str = format!("{:>5}", entry.line_number);
                for ch in num_str.chars() {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(self.line_number_fg);
                    cell.bg = row_bg;
                    ctx.set(x, y, cell);
                    x += 1;
                }
                x += 1; // Space after line number
            }

            // Draw bookmark indicator
            let bookmark_char = if entry.bookmarked { '★' } else { ' ' };
            let mut cell = Cell::new(bookmark_char);
            cell.fg = Some(self.bookmark_fg);
            cell.bg = row_bg;
            ctx.set(x, y, cell);
            x += bookmark_width;

            // Draw timestamp
            if self.show_timestamps {
                if let Some(ref ts) = entry.timestamp {
                    let ts_display = truncate_to_width(ts, timestamp_width as usize - 1);
                    for ch in ts_display.chars() {
                        let cw = char_width(ch) as u16;
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(self.timestamp_fg);
                        cell.bg = row_bg;
                        ctx.set(x, y, cell);
                        x += cw;
                    }
                }
                x = line_num_width + bookmark_width + timestamp_width;
            }

            // Draw level
            if self.show_levels {
                let icon = entry.level.icon();
                let mut cell = Cell::new(icon);
                cell.fg = Some(level_color);
                cell.bg = row_bg;
                ctx.set(x, y, cell);
                x += 1;

                let label = entry.level.label();
                for ch in label.chars().take(3) {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(level_color);
                    cell.bg = row_bg;
                    cell.modifier |= Modifier::BOLD;
                    ctx.set(x, y, cell);
                    x += 1;
                }
                x = line_num_width + bookmark_width + timestamp_width + level_width;
            }

            // Draw source
            if self.show_source {
                if let Some(ref src) = entry.source {
                    let src_display = truncate_to_width(src, source_width as usize - 1);
                    for ch in src_display.chars() {
                        let cw = char_width(ch) as u16;
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(self.source_fg);
                        cell.bg = row_bg;
                        ctx.set(x, y, cell);
                        x += cw;
                    }
                }
                x = prefix_width;
            }

            // Find search matches for this entry
            let matches_for_entry: Vec<_> = self
                .search_matches
                .iter()
                .filter(|m| m.entry_index == *entry_idx)
                .collect();

            // Draw message with search highlighting
            // Search match indices are byte offsets from the find() call in update_search
            let mut col_dx: u16 = 0;
            let mut byte_pos: usize = 0;
            for ch in entry.message.chars() {
                let cw = char_width(ch) as u16;
                if col_dx + cw > message_width {
                    break;
                }
                // Check if this character's byte range overlaps a search match
                let ch_byte_len = ch.len_utf8();
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
                byte_pos += ch_byte_len;
            }
            let _ = col_dx; // message column is always last
        }

        // Draw scroll indicator
        if filtered.len() > visible_height && area.width > 0 && area.height > 0 {
            let scroll_ratio = self.scroll as f32 / (filtered.len() - visible_height) as f32;
            let indicator_pos = (scroll_ratio * (area.height as f32 - 1.0)) as u16;
            let indicator_y = indicator_pos.min(area.height - 1);

            let mut cell = Cell::new('█');
            cell.fg = Some(DARK_GRAY);
            ctx.set(area.width - 1, indicator_y, cell);
        }

        // Draw tail mode indicator
        if self.tail_mode {
            let indicator = "◉ TAIL";
            let indicator_w = display_width(indicator) as u16;
            let x = area.width.saturating_sub(indicator_w + 2);
            let y = 0u16;
            let mut dx: u16 = 0;
            for ch in indicator.chars() {
                let cw = char_width(ch) as u16;
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::GREEN);
                cell.bg = self.bg;
                ctx.set(x + dx, y, cell);
                dx += cw;
            }
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
