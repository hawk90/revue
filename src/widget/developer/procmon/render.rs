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

/// Where the command column starts, after the status column
const CMD_X: u16 = 59;

/// The column header sits under the stats bar (row 0).
const HEADER_ROW: u16 = 1;

impl ProcessMonitor {
    /// Format bytes to human readable
    fn format_bytes(bytes: u64) -> String {
        format_size_compact(bytes)
    }

    /// Render the column header on [`HEADER_ROW`]
    fn render_header(&self, ctx: &mut RenderContext) {
        let area = ctx.area;

        // Header background
        for x in 0..area.width {
            let mut cell = Cell::new(' ');
            cell.bg = Some(self.colors.header_bg);
            ctx.set(x, HEADER_ROW, cell);
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
                ctx.set(hx, HEADER_ROW, cell);
                hx += cw;
            }
            x_offset += width as u16;
        }

        if self.show_cmd {
            ctx.put_str_with(CMD_X, HEADER_ROW, "COMMAND", area.width, |ch| {
                let mut cell = Cell::new(ch);
                cell.fg = Some(self.colors.header_fg);
                cell.bg = Some(self.colors.header_bg);
                cell.modifier = Modifier::BOLD;
                cell
            });
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

            // Command line (the name when the system gave none)
            if self.show_cmd {
                let cmd = if proc.cmd.is_empty() {
                    &proc.name
                } else {
                    &proc.cmd
                };
                ctx.put_str_with(CMD_X, y, cmd, area.width, |ch| {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(LIGHT_GRAY);
                    cell.bg = bg;
                    cell
                });
            }
        }
    }

    crate::impl_view_meta!("ProcessMonitor");
}

#[cfg(test)]
mod tests {
    use super::super::{ProcessInfo, ProcessView};
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    fn info(pid: u32, name: &str, user: &str, cmd: &str) -> ProcessInfo {
        ProcessInfo {
            pid,
            parent_pid: None,
            name: name.to_string(),
            cpu: 0.0,
            memory: 0,
            memory_percent: 0.0,
            status: "Run".to_string(),
            cmd: cmd.to_string(),
            user: user.to_string(),
        }
    }

    fn rows(monitor: &ProcessMonitor, w: u16, h: u16) -> Vec<String> {
        let mut buf = Buffer::new(w, h);
        monitor.render(&mut RenderContext::new(&mut buf, Rect::new(0, 0, w, h)));
        (0..h)
            .map(|y| (0..w).map(|x| buf.get(x, y).unwrap().symbol).collect())
            .collect()
    }

    #[test]
    fn the_stats_bar_and_the_column_header_each_get_their_row() {
        let mut monitor = ProcessMonitor::new();
        monitor.set_process_list(vec![info(7, "sh", "501", "")]);
        let rows = rows(&monitor, 100, 5);
        assert!(
            rows[0].contains("Processes:"),
            "stats bar missing: {:?}",
            rows[0]
        );
        assert!(
            rows[1].contains("PID"),
            "column header missing: {:?}",
            rows[1]
        );
        assert!(
            rows[2].contains("sh"),
            "first process missing: {:?}",
            rows[2]
        );
    }

    #[test]
    fn show_cmd_draws_the_command_column() {
        let list = vec![info(7, "sh", "501", "/bin/sh -c true")];

        let mut monitor = ProcessMonitor::new();
        monitor.set_process_list(list.clone());
        let hidden = rows(&monitor, 100, 5);
        // Stats bar, column header, then the processes.
        assert!(!hidden[1].contains("COMMAND"), "{:?}", hidden[1]);
        assert!(!hidden[2].contains("/bin/sh"), "{:?}", hidden[2]);

        let mut monitor = ProcessMonitor::new().show_cmd(true);
        monitor.set_process_list(list);
        let shown = rows(&monitor, 100, 5);
        assert!(shown[1].contains("COMMAND"), "{:?}", shown[1]);
        assert!(shown[2].contains("/bin/sh -c true"), "{:?}", shown[2]);
    }

    #[test]
    fn user_view_lists_only_the_current_users_processes() {
        let list = vec![info(1, "init", "0", ""), info(2, "mine", "501", "")];

        let mut monitor = ProcessMonitor::new();
        monitor.current_user = Some("501".to_string());
        monitor.set_process_list(list.clone());
        assert_eq!(monitor.process_count(), 2);

        let mut monitor = ProcessMonitor::new().view(ProcessView::User);
        monitor.current_user = Some("501".to_string());
        monitor.set_process_list(list);
        assert_eq!(monitor.process_count(), 1);
        assert_eq!(monitor.selected_process().unwrap().name, "mine");
    }
}
