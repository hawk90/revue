//! Axes: the grid with its value labels, and the time labels under the plot

use super::types::TimeFormat as Format;
use super::view::Plot;
use super::TimeSeries;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::{char_width, display_width};
use crate::widget::data::chart::chart_render::put_label;
use crate::widget::traits::RenderContext;

impl TimeSeries {
    /// The value labels of the grid lines on a plot of `plot_height` rows,
    /// from `val_max` at the top down to `val_min`, at four even steps (fewer
    /// on a shorter plot)
    pub(super) fn value_labels(&self, plot_height: u16, val_min: f64, val_max: f64) -> Vec<String> {
        let grid_rows = 4.min(plot_height as usize);
        let val_range = val_max - val_min;
        (0..=grid_rows)
            .map(|i| {
                let val = val_max - (i as f64 * val_range / grid_rows as f64);
                if val.abs() >= 1000.0 {
                    format!("{:.1}k", val / 1000.0)
                } else if val.abs() >= 1.0 {
                    format!("{:.1}", val)
                } else {
                    format!("{:.2}", val)
                }
            })
            .collect()
    }

    /// Grid lines at four even steps of the value range, each labelled
    /// with its value left of the plot, right-aligned against the gap
    /// column before it; a wider label is cut with `…`
    pub(super) fn draw_grid(&self, ctx: &mut RenderContext, plot: &Plot, chrome: Color) {
        let Plot {
            x: plot_x,
            y: plot_y,
            width: plot_width,
            height: plot_height,
            val_min,
            val_max,
            ..
        } = *plot;

        let grid_rows = 4.min(plot_height as usize);
        let labels = self.value_labels(plot_height, val_min, val_max);
        let style = Cell {
            fg: Some(chrome),
            bg: self.bg_color,
            ..Cell::default()
        };
        for (i, label) in labels.iter().enumerate() {
            let y = plot_y + (i * plot_height as usize / grid_rows) as u16;
            for x in plot_x..plot_x + plot_width {
                let ch = if i == grid_rows { '─' } else { '┄' };
                let mut cell = Cell::new(ch);
                cell.fg = Some(self.grid_color);
                ctx.set(x, y, cell);
            }

            // Y-axis labels
            put_label(ctx, label, (0, y), plot_x.saturating_sub(1), true, style);
        }
    }

    /// Time labels on the row under the plot, about one per 12 columns
    pub(super) fn draw_time_labels(
        &self,
        ctx: &mut RenderContext,
        plot: &Plot,
        height: u16,
        chrome: Color,
    ) {
        let Plot {
            x: plot_x,
            y: plot_y,
            width: plot_width,
            height: plot_height,
            time_min,
            time_range,
            ..
        } = *plot;
        let area = ctx.area;

        let x_label_y = plot_y + plot_height + 1;
        if x_label_y < height {
            let num_labels = (plot_width / 12).max(2) as usize;
            for i in 0..num_labels {
                let ratio = i as f64 / (num_labels - 1) as f64;
                let ts = time_min + (ratio * time_range as f64) as u64;
                let label = self.format_time(ts, time_range);
                let x = plot_x + (ratio * (plot_width - 1) as f64) as u16;
                let half = u16::try_from(display_width(&label) / 2).unwrap_or(u16::MAX);
                let label_x = x.saturating_sub(half);
                let mut dx: u16 = 0;
                for ch in label.chars() {
                    let lx = label_x.saturating_add(dx);
                    if lx >= area.width {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(chrome);
                    cell.bg = self.bg_color;
                    ctx.set(lx, x_label_y, cell);
                    dx = dx.saturating_add(char_width(ch) as u16);
                }
            }
        }
    }

    /// Format a timestamp (Unix seconds) for display
    ///
    /// Times and dates are in UTC: `Days` is the UTC calendar date
    /// (`MM/DD`) and `Months` the UTC year and month (`YYYY/MM`), with real
    /// month lengths and leap years. A timestamp past the calendar's range
    /// (9999-12-30 22:00:00 UTC) is printed as its Unix seconds in those
    /// two formats. `Auto` picks the format from `range`, the span of the
    /// time axis in seconds.
    pub fn format_time(&self, timestamp: u64, range: u64) -> String {
        let format = match self.time_format {
            Format::Auto => {
                if range < 60 {
                    Format::Seconds
                } else if range < 3600 {
                    Format::Minutes
                } else if range < 86400 {
                    Format::Hours
                } else if range < 86400 * 30 {
                    Format::Days
                } else {
                    Format::Months
                }
            }
            f => f,
        };

        let secs = timestamp % 60;
        let mins = (timestamp / 60) % 60;
        let hours = (timestamp / 3600) % 24;
        // The UTC calendar date, or None past jiff's range
        let date = || {
            let ts = jiff::Timestamp::from_second(i64::try_from(timestamp).ok()?).ok()?;
            Some(jiff::tz::Offset::UTC.to_datetime(ts).date())
        };

        match format {
            Format::Seconds => format!("{:02}:{:02}:{:02}", hours, mins, secs),
            Format::Minutes => format!("{:02}:{:02}", hours, mins),
            Format::Hours => format!("{:02}:00", hours),
            // `Auto` was resolved to one of the formats above
            Format::Days | Format::Auto => match date() {
                Some(d) => format!("{:02}/{:02}", d.month(), d.day()),
                None => timestamp.to_string(),
            },
            Format::Months => match date() {
                Some(d) => format!("{}/{:02}", d.year(), d.month()),
                None => timestamp.to_string(),
            },
            Format::Unix => format!("{}", timestamp),
            Format::Relative => {
                let now = super::types::unix_now();
                let diff = now.saturating_sub(timestamp);
                if diff < 60 {
                    format!("{}s", diff)
                } else if diff < 3600 {
                    format!("{}m", diff / 60)
                } else if diff < 86400 {
                    format!("{}h", diff / 3600)
                } else {
                    format!("{}d", diff / 86400)
                }
            }
        }
    }
}
