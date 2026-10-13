//! Axes: the grid with its value labels, and the time labels under the plot

use super::types::TimeFormat as Format;
use super::view::Plot;
use super::TimeSeries;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::{char_width, display_width};
use crate::widget::traits::RenderContext;

impl TimeSeries {
    /// Grid lines at four even steps of the value range, each labelled
    /// with its value left of the plot
    pub(super) fn draw_grid(&self, ctx: &mut RenderContext, plot: &Plot, chrome: Color) {
        let Plot {
            x: plot_x,
            y: plot_y,
            width: plot_width,
            height: plot_height,
            val_max,
            val_range,
            ..
        } = *plot;
        let area = ctx.area;
        let y_label_width = plot_x;

        let grid_rows = 4.min(plot_height as usize);
        for i in 0..=grid_rows {
            let y = plot_y + (i * plot_height as usize / grid_rows) as u16;
            for x in plot_x..plot_x + plot_width {
                let ch = if i == grid_rows { '─' } else { '┄' };
                let mut cell = Cell::new(ch);
                cell.fg = Some(self.grid_color);
                ctx.set(x, y, cell);
            }

            // Y-axis labels
            let val = val_max - (i as f64 * val_range / grid_rows as f64);
            let label = if val.abs() >= 1000.0 {
                format!("{:.1}k", val / 1000.0)
            } else if val.abs() >= 1.0 {
                format!("{:.1}", val)
            } else {
                format!("{:.2}", val)
            };
            let label_x = y_label_width.saturating_sub(display_width(&label) as u16 + 1);
            let mut dx: u16 = 0;
            for ch in label.chars() {
                let x = label_x + dx;
                if x < area.width {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(chrome);
                    cell.bg = self.bg_color;
                    ctx.set(x, y, cell);
                }
                dx += char_width(ch) as u16;
            }
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
                let label_x = x.saturating_sub(display_width(&label) as u16 / 2);
                let mut dx: u16 = 0;
                for ch in label.chars() {
                    let lx = label_x + dx;
                    if lx < area.width {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(chrome);
                        cell.bg = self.bg_color;
                        ctx.set(lx, x_label_y, cell);
                    }
                    dx += char_width(ch) as u16;
                }
            }
        }
    }

    /// Format a timestamp for display
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
        let days = (timestamp / 86400) % 31;
        let months = ((timestamp / 86400) / 30) % 12;

        match format {
            Format::Seconds => format!("{:02}:{:02}:{:02}", hours, mins, secs),
            Format::Minutes => format!("{:02}:{:02}", hours, mins),
            Format::Hours => format!("{:02}:00", hours),
            Format::Days => format!("{:02}/{:02}", months + 1, days + 1),
            Format::Months => format!("{}/{:02}", 1970 + timestamp / 31536000, months + 1),
            Format::Unix => format!("{}", timestamp),
            Format::Relative => {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
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
            Format::Auto => {
                if timestamp < 3600 {
                    format!("{:02}:{:02}:{:02}", hours, mins, secs)
                } else if timestamp < 86400 {
                    format!("{:02}:{:02}", hours, mins)
                } else if timestamp < 31536000 {
                    format!("{:02}/{:02}", months + 1, days + 1)
                } else {
                    format!("{}/{:02}", 1970 + timestamp / 31536000, months + 1)
                }
            }
        }
    }
}
