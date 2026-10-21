//! Drawing the log: which entries fit, each entry's rows (message and
//! expanded details) and the scroll indicator. The prefix columns are in
//! `prefix.rs`.

use super::{LogEntry, LogLevel, RichLog};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::{char_width, truncate_to_width, wrap_to_width};
use crate::widget::theme::DISABLED_FG;
use crate::widget::traits::{RenderContext, View};

impl RichLog {
    /// The rows one entry takes: its message (one row, or several when
    /// wrapping) and, when expanded, its detail lines. `true` marks a detail.
    fn entry_lines(&self, entry: &LogEntry, message_width: usize) -> Vec<(String, bool)> {
        let message = if self.wrap && message_width > 0 {
            wrap_to_width(&entry.message, message_width)
        } else {
            Vec::new()
        };
        let mut lines: Vec<(String, bool)> = if message.is_empty() {
            vec![(entry.message.clone(), false)]
        } else {
            message.into_iter().map(|l| (l, false)).collect()
        };
        if entry.expanded {
            lines.extend(entry.details.iter().map(|d| (format!("  {d}"), true)));
        }
        lines
    }

    /// The rows `entry` takes when messages are `width` wide (`None` before
    /// the first draw: one row per message): as many as
    /// [`entry_lines`](Self::entry_lines) gives, without building them.
    pub(super) fn entry_rows(&self, entry: &LogEntry, width: Option<usize>) -> usize {
        let message = match width {
            Some(width) if self.wrap && width > 0 => {
                wrap_to_width(&entry.message, width).len().max(1)
            }
            _ => 1,
        };
        let details = if entry.expanded {
            entry.details.len()
        } else {
            0
        };
        message + details
    }

    /// Fill row `y` with the background color, when one is set
    fn fill_row(&self, ctx: &mut RenderContext, y: u16, width: u16) {
        if let Some(bg) = self.bg {
            for x in 0..width {
                let mut cell = Cell::new(' ');
                cell.bg = Some(bg);
                ctx.set(x, y, cell);
            }
        }
    }

    /// The message (or detail) text color, and whether it is bold
    fn line_style(&self, entry: &LogEntry, is_detail: bool, is_selected: bool) -> (Color, bool) {
        let fg = if is_detail {
            self.source_fg
        } else if is_selected {
            Color::WHITE
        } else {
            entry.level.color()
        };
        let bold = !is_detail && (is_selected || entry.level >= LogLevel::Error);
        (fg, bold)
    }

    /// Draw one row of message (or detail) text from `prefix_width`, cut at
    /// `message_width` columns
    fn draw_message(
        &self,
        ctx: &mut RenderContext,
        text: &str,
        y: u16,
        prefix_width: u16,
        message_width: u16,
        (fg, bold): (Color, bool),
    ) {
        let mut x = prefix_width;
        let truncated = truncate_to_width(text, message_width as usize);
        for ch in truncated.chars() {
            let cw = char_width(ch) as u16;
            if x + cw > prefix_width + message_width {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(fg);
            cell.bg = self.bg;
            if bold {
                cell.modifier |= Modifier::BOLD;
            }
            ctx.set(x, y, cell);
            x += cw;
        }
    }

    /// Draw the scroll marker in the last column, at the row matching how far
    /// the view is scrolled
    fn draw_scroll_indicator(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let (offset, max) = (self.scroll.offset(), self.scroll.max_offset());
        if let Some(scroll_pos) = (offset * (area.height as usize - 1)).checked_div(max) {
            let indicator_y = scroll_pos as u16;
            if indicator_y < area.height {
                let mut cell = Cell::new('█');
                cell.fg = Some(DISABLED_FG);
                ctx.set(area.width - 1, indicator_y, cell);
            }
        }
    }
}

impl View for RichLog {
    crate::impl_view_meta!("RichLog");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let entries = self.visible_entries();

        // Nothing fits in a zero-sized area, and the scroll indicator's math
        // below needs at least one row and one column.
        if entries.is_empty() || area.width == 0 || area.height == 0 {
            return;
        }

        let columns = self.columns();
        let prefix_width = columns.prefix_width();
        let message_width = area.width.saturating_sub(prefix_width);

        // The view in rows, at this width
        let width = message_width as usize;
        self.message_width.set(Some(width));
        let total: usize = entries
            .iter()
            .map(|e| self.entry_rows(e, Some(width)))
            .sum();
        self.scroll.set_content(total);
        self.scroll.set_viewport(area.height as usize);
        let visible = self.scroll.visible();

        // Walk the entries by row and draw the rows in view; the first entry
        // drawn can start above the view
        let mut row = 0;
        for (i, entry) in entries.iter().enumerate() {
            if row >= visible.end {
                break;
            }
            let height = self.entry_rows(entry, Some(width));
            if row + height <= visible.start {
                row += height;
                continue;
            }

            let is_selected = self.selection.is_selected(i);
            for (line_idx, (text, is_detail)) in
                self.entry_lines(entry, width).into_iter().enumerate()
            {
                let at = row + line_idx;
                if at < visible.start {
                    continue;
                }
                if at >= visible.end {
                    break;
                }
                let y = (at - visible.start) as u16;

                self.fill_row(ctx, y, area.width);

                // The prefix columns go on the entry's first row only
                if line_idx == 0 {
                    self.draw_prefix(ctx, &columns, entry, y);
                }

                // Draw the message (or detail) text
                let style = self.line_style(entry, is_detail, is_selected);
                self.draw_message(ctx, &text, y, prefix_width, message_width, style);
            }
            row += height;
        }

        self.draw_scroll_indicator(ctx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    fn rows(log: &RichLog, w: u16, h: u16) -> Vec<String> {
        let mut buf = Buffer::new(w, h);
        let mut ctx = RenderContext::new(&mut buf, Rect::new(0, 0, w, h));
        log.render(&mut ctx);
        (0..h)
            .map(|y| {
                (0..w)
                    .map(|x| buf.get(x, y).map(|c| c.symbol).unwrap_or(' '))
                    .collect::<String>()
                    .trim_end()
                    .to_string()
            })
            .collect()
    }

    #[test]
    fn auto_scroll_fills_the_last_page() {
        let mut log = RichLog::new().timestamps(false).sources(false).icons(false);
        for i in 0..10 {
            log.info(format!("m{i}"));
        }
        // The indicator column sits at the right edge; the messages are short.
        let rows: Vec<String> = rows(&log, 10, 4)
            .into_iter()
            .map(|r| r.trim_end_matches('█').trim_end().to_string())
            .collect();
        assert_eq!(rows, ["m6", "m7", "m8", "m9"]);
    }

    fn entry() -> super::super::LogEntry {
        super::super::LogEntry::new("hi")
            .timestamp("12:00")
            .source("app")
    }

    #[test]
    fn simple_format_draws_only_the_message() {
        let mut log = RichLog::new().format(super::super::LogFormat::Simple);
        log.log(entry());
        assert_eq!(rows(&log, 40, 1), ["hi"]);
    }

    #[test]
    fn detailed_format_draws_timestamp_and_source_even_when_hidden() {
        let mut log = RichLog::new()
            .timestamps(false)
            .sources(false)
            .icons(false)
            .format(super::super::LogFormat::Detailed);
        log.log(entry());
        let row = &rows(&log, 40, 1)[0];
        assert!(row.starts_with("12:00"), "{row:?}");
        assert!(row.contains("app"), "{row:?}");
        assert!(row.ends_with("hi"), "{row:?}");
    }

    #[test]
    fn labels_builder_draws_the_level_label() {
        let mut log = RichLog::new()
            .timestamps(false)
            .sources(false)
            .icons(false)
            .labels(true);
        log.error("boom");
        assert_eq!(rows(&log, 40, 1), ["ERROR  boom"]);
    }

    #[test]
    fn expanded_entry_draws_its_details_below_the_message() {
        let mut log = RichLog::new().timestamps(false).sources(false).icons(false);
        log.log(super::super::LogEntry::new("boom").detail("at main.rs:1"));
        log.info("next");
        assert_eq!(rows(&log, 40, 3), ["boom", "next", ""]);

        log.select_next();
        log.toggle_selected();
        assert_eq!(rows(&log, 40, 3), ["boom", "  at main.rs:1", "next"]);
    }

    #[test]
    fn wrap_continues_a_long_message_on_the_next_rows() {
        let mut log = RichLog::new().timestamps(false).sources(false).icons(false);
        log.info("alpha beta gamma");
        log.info("end");
        assert_eq!(rows(&log, 10, 3), ["alpha beta", "end", ""]);

        let log = log.wrap(true);
        assert_eq!(rows(&log, 10, 3), ["alpha beta", "gamma", "end"]);
    }
}
