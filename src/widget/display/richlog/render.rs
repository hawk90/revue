//! Drawing the log: the prefix columns, the message, expanded details and the
//! scroll indicator

use super::{LogEntry, LogFormat, LogLevel, RichLog};
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

        // Which prefix columns the format shows
        let (show_timestamps, show_sources, show_icons, show_labels) = match self.format {
            LogFormat::Simple => (false, false, false, false),
            LogFormat::Detailed => (true, true, self.show_icons, self.show_labels),
            _ => (
                self.show_timestamps,
                self.show_sources,
                self.show_icons,
                self.show_labels,
            ),
        };

        // Calculate prefix widths
        let timestamp_width = if show_timestamps { 12 } else { 0 };
        let icon_width = if show_icons { 2 } else { 0 };
        let label_width = if show_labels { 7 } else { 0 };
        let source_width = if show_sources { 15 } else { 0 };

        let prefix_width = timestamp_width + icon_width + label_width + source_width;
        let message_width = area.width.saturating_sub(prefix_width);

        // Calculate visible range
        let visible_height = area.height as usize;
        // `scroll` is the first entry to show, but the scroll methods do not
        // know the viewport height (`scroll_to_bottom` puts it on the last
        // entry), so keep the last page full here. An entry can take more
        // than one row (wrapped, or expanded details).
        let mut max_start = entries.len();
        let mut filled = 0;
        while max_start > 0 && filled < visible_height {
            max_start -= 1;
            filled += self
                .entry_lines(entries[max_start], message_width as usize)
                .len();
        }
        let start = self.scroll.min(max_start);

        let mut y: u16 = 0;
        for (i, entry) in entries.iter().enumerate().skip(start) {
            if y >= area.height {
                break;
            }

            let is_selected = self.selected == Some(i);
            let level_color = entry.level.color();

            for (line_idx, (text, is_detail)) in self
                .entry_lines(entry, message_width as usize)
                .into_iter()
                .enumerate()
            {
                if y >= area.height {
                    break;
                }

                // Fill background
                if let Some(bg) = self.bg {
                    for x in 0..area.width {
                        let mut cell = Cell::new(' ');
                        cell.bg = Some(bg);
                        ctx.set(x, y, cell);
                    }
                }

                let mut x: u16 = 0;

                // The prefix columns go on the entry's first row only
                if line_idx == 0 {
                    // Draw timestamp
                    if show_timestamps {
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
                    if show_icons {
                        let icon = entry.level.icon();
                        let mut cell = Cell::new(icon);
                        cell.fg = Some(level_color);
                        cell.bg = self.bg;
                        ctx.set(x, y, cell);
                        x += icon_width;
                    }

                    // Draw label
                    if show_labels {
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
                    if show_sources {
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
                    }
                }
                x = prefix_width;

                // Draw the message (or detail) text
                let fg = if is_detail {
                    self.source_fg
                } else if is_selected {
                    Color::WHITE
                } else {
                    level_color
                };
                let bold = !is_detail && (is_selected || entry.level >= LogLevel::Error);
                let truncated = truncate_to_width(&text, message_width as usize);
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

                y += 1;
            }
        }

        // Draw scroll indicator
        if let Some(scroll_pos) = (start * (area.height as usize - 1)).checked_div(max_start) {
            let indicator_y = scroll_pos as u16;
            if indicator_y < area.height {
                let mut cell = Cell::new('█');
                cell.fg = Some(DISABLED_FG);
                ctx.set(area.width - 1, indicator_y, cell);
            }
        }
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
