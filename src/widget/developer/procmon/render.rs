//! Drawing the process monitor: stats bar, column header and process rows

use super::{ProcessMonitor, ProcessSort};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::format_size_compact;
use crate::widget::theme::LIGHT_GRAY;
use crate::widget::traits::{RenderContext, View};

/// The process-row text color a monitor uses when neither
/// [`ProcColors::name`](crate::widget::ProcColors::name) nor the stylesheet names one.
const PROC_FG: Color = Color::WHITE;

impl ProcessMonitor {
    /// Format bytes to human readable
    fn format_bytes(bytes: u64) -> String {
        format_size_compact(bytes)
    }

    /// Render header
    fn render_header(&self, ctx: &mut RenderContext) {
        let area = ctx.area;

        // Header background
        for x in 0..area.width {
            let mut cell = Cell::new(' ');
            cell.bg = Some(self.colors.header_bg);
            ctx.set(x, 0, cell);
        }

        // Column headers
        let headers = [
            ("PID", 7, ProcessSort::Pid),
            ("NAME", 20, ProcessSort::Name),
            ("CPU%", 7, ProcessSort::Cpu),
            ("MEM%", 7, ProcessSort::Memory),
            ("MEM", 8, ProcessSort::Memory),
            ("STATUS", 10, ProcessSort::Status),
        ];

        let mut x_offset = 0u16;
        for (name, width, sort) in headers {
            let indicator = if self.sort == sort {
                if self.sort_asc {
                    "▲"
                } else {
                    "▼"
                }
            } else {
                ""
            };

            let text = format!("{}{}", name, indicator);
            let mut hx = x_offset;
            for ch in text.chars() {
                let cw = crate::utils::char_width(ch) as u16;
                if hx + cw > area.width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(self.colors.header_fg);
                cell.bg = Some(self.colors.header_bg);
                cell.modifier = Modifier::BOLD;
                ctx.set(hx, 0, cell);
                hx += cw;
            }
            x_offset += width as u16;
        }
    }

    /// Render system stats bar
    fn render_stats(&self, ctx: &mut RenderContext, y: u16) {
        let area = ctx.area;
        let (used_mem, total_mem) = self.memory_usage();
        let cpu = self.cpu_usage();

        let stats = format!(
            "CPU: {:5.1}%  MEM: {} / {} ({:.1}%)  Processes: {}",
            cpu,
            Self::format_bytes(used_mem),
            Self::format_bytes(total_mem),
            (used_mem as f64 / total_mem as f64) * 100.0,
            self.process_count()
        );

        let mut sx: u16 = 0;
        for ch in stats.chars() {
            let cw = crate::utils::char_width(ch) as u16;
            if sx + cw > area.width {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(LIGHT_GRAY);
            ctx.set(sx, y, cell);
            sx += cw;
        }
    }
}

impl View for ProcessMonitor {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 40 || area.height < 5 {
            return;
        }

        // The CPU and memory thresholds carry the reading - red is why you are
        // looking - so they stay. The process rows are the base.
        let row_fg = self.colors.name.unwrap_or_else(|| ctx.css_color(PROC_FG));

        // Stats bar
        self.render_stats(ctx, 0);

        // Header (row 1)
        self.render_header(ctx);

        // Process list
        let list_start = 2u16;
        let visible_rows = (area.height - list_start) as usize;

        // Adjust scroll to keep selection visible
        let scroll = if self.selected < self.scroll {
            self.selected
        } else if self.selected >= self.scroll + visible_rows {
            self.selected - visible_rows + 1
        } else {
            self.scroll
        };

        for (i, proc) in self
            .processes
            .iter()
            .skip(scroll)
            .take(visible_rows)
            .enumerate()
        {
            let y = list_start + i as u16;
            let is_selected = scroll + i == self.selected;

            // Background
            if is_selected {
                for x in 0..area.width {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(self.colors.selected_bg);
                    ctx.set(x, y, cell);
                }
            }

            let bg = if is_selected {
                Some(self.colors.selected_bg)
            } else {
                None
            };

            // PID
            let pid_str = format!("{:>6}", proc.pid);
            for (j, ch) in pid_str.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(self.colors.pid);
                cell.bg = bg;
                ctx.set(j as u16, y, cell);
            }

            // Name (truncated)
            let name = crate::utils::truncate_to_width(&proc.name, 19);
            let mut nx: u16 = 7;
            for ch in name.chars() {
                let cw = crate::utils::char_width(ch) as u16;
                if nx + cw > 26 {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(row_fg);
                cell.bg = bg;
                ctx.set(nx, y, cell);
                nx += cw;
            }

            // CPU%
            let cpu_str = format!("{:>6.1}", proc.cpu);
            let cpu_color = if proc.cpu > 80.0 {
                self.colors.high_cpu
            } else if proc.cpu > 30.0 {
                self.colors.medium_cpu
            } else {
                self.colors.low_cpu
            };
            for (j, ch) in cpu_str.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(cpu_color);
                cell.bg = bg;
                ctx.set(27 + j as u16, y, cell);
            }

            // MEM%
            let mem_pct_str = format!("{:>6.1}", proc.memory_percent);
            let mem_color = if proc.memory_percent > 10.0 {
                self.colors.high_mem
            } else {
                row_fg
            };
            for (j, ch) in mem_pct_str.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(mem_color);
                cell.bg = bg;
                ctx.set(34 + j as u16, y, cell);
            }

            // MEM (bytes)
            let mem_str = format!("{:>7}", Self::format_bytes(proc.memory));
            for (j, ch) in mem_str.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(row_fg);
                cell.bg = bg;
                ctx.set(41 + j as u16, y, cell);
            }

            // Status
            if area.width > 55 {
                let status = crate::utils::truncate_to_width(&proc.status, 8);
                let mut stx: u16 = 49;
                for ch in status.chars() {
                    let cw = crate::utils::char_width(ch) as u16;
                    if stx + cw > 57 {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(LIGHT_GRAY);
                    cell.bg = bg;
                    ctx.set(stx, y, cell);
                    stx += cw;
                }
            }
        }
    }

    crate::impl_view_meta!("ProcessMonitor");
}
