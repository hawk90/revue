//! The prefix columns drawn before an entry's message: timestamp, level icon,
//! level label and source

use super::{LogEntry, LogFormat, RichLog};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::{char_width, truncate_to_width};
use crate::widget::traits::RenderContext;

/// Which prefix columns the format shows, and how wide each one is
#[derive(Clone, Copy)]
pub(super) struct Columns {
    show_timestamps: bool,
    show_sources: bool,
    show_icons: bool,
    show_labels: bool,
    timestamp_width: u16,
    icon_width: u16,
    label_width: u16,
    source_width: u16,
}

impl Columns {
    /// The width of all shown prefix columns; the message starts here
    pub(super) fn prefix_width(&self) -> u16 {
        self.timestamp_width + self.icon_width + self.label_width + self.source_width
    }
}

impl RichLog {
    /// The prefix columns the format and the column builders choose
    pub(super) fn columns(&self) -> Columns {
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

        Columns {
            show_timestamps,
            show_sources,
            show_icons,
            show_labels,
            timestamp_width,
            icon_width,
            label_width,
            source_width,
        }
    }

    /// Draw the prefix columns on an entry's first row
    pub(super) fn draw_prefix(
        &self,
        ctx: &mut RenderContext,
        columns: &Columns,
        entry: &LogEntry,
        y: u16,
    ) {
        let Columns {
            show_timestamps,
            show_sources,
            show_icons,
            show_labels,
            timestamp_width,
            icon_width,
            label_width,
            source_width,
        } = *columns;
        let level_color = entry.level.color();
        let mut x: u16 = 0;

        // Draw timestamp
        if show_timestamps {
            if let Some(ref ts) = entry.timestamp {
                let ts_display = truncate_to_width(ts, timestamp_width as usize - 1);
                self.draw_column_text(ctx, x, y, ts_display, self.timestamp_fg);
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
                self.draw_column_text(ctx, x, y, src_display, self.source_fg);
            }
        }
    }

    /// Draw a timestamp or source from `x`, one cell per character
    fn draw_column_text(&self, ctx: &mut RenderContext, mut x: u16, y: u16, text: &str, fg: Color) {
        for ch in text.chars() {
            let cw = char_width(ch) as u16;
            let mut cell = Cell::new(ch);
            cell.fg = Some(fg);
            cell.bg = self.bg;
            ctx.set(x, y, cell);
            x += cw;
        }
    }
}
