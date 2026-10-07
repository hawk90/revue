//! Drawing the log: the prefix columns, the message and the scroll indicator

use super::{LogLevel, RichLog};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::{char_width, truncate_to_width};
use crate::widget::theme::DISABLED_FG;
use crate::widget::traits::{RenderContext, View};

impl View for RichLog {
    crate::impl_view_meta!("RichLog");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let entries = self.visible_entries();

        if entries.is_empty() {
            return;
        }

        // Calculate prefix widths
        let timestamp_width = if self.show_timestamps { 12 } else { 0 };
        let icon_width = if self.show_icons { 2 } else { 0 };
        let label_width = if self.show_labels { 7 } else { 0 };
        let source_width = if self.show_sources { 15 } else { 0 };

        let prefix_width = timestamp_width + icon_width + label_width + source_width;
        let message_width = area.width.saturating_sub(prefix_width);

        // Calculate visible range
        let visible_height = area.height as usize;
        let start = self.scroll;

        for (i, entry) in entries.iter().enumerate().skip(start).take(visible_height) {
            let y = (i - start) as u16;
            if y >= area.height {
                break;
            }

            let is_selected = self.selected == Some(i);
            let level_color = entry.level.color();

            // Fill background
            if let Some(bg) = self.bg {
                for x in 0..area.width {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(bg);
                    ctx.set(x, y, cell);
                }
            }

            let mut x: u16 = 0;

            // Draw timestamp
            if self.show_timestamps {
                if let Some(ref ts) = entry.timestamp {
                    let ts_display = truncate_to_width(ts, timestamp_width as usize - 1);
                    for ch in ts_display.chars() {
                        let cw = char_width(ch) as u16;
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(self.timestamp_fg);
                        cell.bg = self.bg;
                        ctx.set(x, y, cell);
                        x += cw;
                    }
                }
                x = timestamp_width;
            }

            // Draw icon
            if self.show_icons {
                let icon = entry.level.icon();
                let mut cell = Cell::new(icon);
                cell.fg = Some(level_color);
                cell.bg = self.bg;
                ctx.set(x, y, cell);
                x += icon_width;
            }

            // Draw label
            if self.show_labels {
                let label = entry.level.label();
                for ch in label.chars() {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(level_color);
                    cell.bg = self.bg;
                    cell.modifier |= Modifier::BOLD;
                    ctx.set(x, y, cell);
                    x += 1;
                }
                x = timestamp_width + icon_width + label_width;
            }

            // Draw source
            if self.show_sources {
                if let Some(ref src) = entry.source {
                    let src_display = truncate_to_width(src, source_width as usize - 1);
                    for ch in src_display.chars() {
                        let cw = char_width(ch) as u16;
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(self.source_fg);
                        cell.bg = self.bg;
                        ctx.set(x, y, cell);
                        x += cw;
                    }
                }
                x = prefix_width;
            }

            // Draw message
            let msg_fg = if is_selected {
                Color::WHITE
            } else {
                level_color
            };
            let msg_truncated = truncate_to_width(&entry.message, message_width as usize);
            for ch in msg_truncated.chars() {
                let cw = char_width(ch) as u16;
                if x + cw > prefix_width + message_width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(msg_fg);
                cell.bg = self.bg;
                if is_selected {
                    cell.modifier |= Modifier::BOLD;
                }
                if entry.level >= LogLevel::Error {
                    cell.modifier |= Modifier::BOLD;
                }
                ctx.set(x, y, cell);
                x += cw;
            }
        }

        // Draw scroll indicator
        if entries.len() > visible_height {
            let scroll_pos = if entries.len() <= visible_height {
                0
            } else {
                (self.scroll * (area.height as usize - 1)) / (entries.len() - visible_height)
            };

            let indicator_y = scroll_pos as u16;
            if indicator_y < area.height {
                let mut cell = Cell::new('█');
                cell.fg = Some(DISABLED_FG);
                ctx.set(area.width - 1, indicator_y, cell);
            }
        }
    }
}
