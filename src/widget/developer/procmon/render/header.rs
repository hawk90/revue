//! The two rows above the processes: the system stats bar and the column header

use super::{put_chars, CMD_X, CPU_X, HEADER_ROW, MEM_PERCENT_X, MEM_X, NAME_X, STATUS_X};
use crate::render::{Cell, Modifier};
use crate::widget::developer::procmon::refresh::memory_percent;
use crate::widget::developer::procmon::{ProcessMonitor, ProcessSort};
use crate::widget::theme::LIGHT_GRAY;
use crate::widget::traits::RenderContext;

impl ProcessMonitor {
    /// A header cell: header colors, bold
    fn header_cell(&self, ch: char) -> Cell {
        let mut cell = Cell::new(ch);
        cell.fg = Some(self.colors.header_fg);
        cell.bg = Some(self.colors.header_bg);
        cell.modifier = Modifier::BOLD;
        cell
    }

    /// Render the column header on [`HEADER_ROW`]
    pub(super) fn render_header(&self, ctx: &mut RenderContext) {
        let area = ctx.area;

        // Header background
        for x in 0..area.width {
            let mut cell = Cell::new(' ');
            cell.bg = Some(self.colors.header_bg);
            ctx.set(x, HEADER_ROW, cell);
        }

        // Column headers, each where its column starts
        let headers = [
            ("PID", 0, ProcessSort::Pid),
            ("NAME", NAME_X, ProcessSort::Name),
            ("CPU%", CPU_X, ProcessSort::Cpu),
            ("MEM%", MEM_PERCENT_X, ProcessSort::Memory),
            ("MEM", MEM_X, ProcessSort::Memory),
            ("STATUS", STATUS_X, ProcessSort::Status),
        ];

        for (name, x, sort) in headers {
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
            put_chars(ctx, x, HEADER_ROW, &text, area.width, |ch| {
                self.header_cell(ch)
            });
        }

        if self.show_cmd {
            ctx.put_str_with(CMD_X, HEADER_ROW, "COMMAND", area.width, |ch| {
                self.header_cell(ch)
            });
        }
    }

    /// Render system stats bar
    pub(super) fn render_stats(&self, ctx: &mut RenderContext, y: u16) {
        let area = ctx.area;
        let (used_mem, total_mem) = self.memory_usage();
        let cpu = self.cpu_usage();

        let stats = format!(
            "CPU: {:5.1}%  MEM: {} / {} ({:.1}%)  Processes: {}",
            cpu,
            Self::format_bytes(used_mem),
            Self::format_bytes(total_mem),
            memory_percent(used_mem, total_mem),
            self.process_count()
        );

        put_chars(ctx, 0, y, &stats, area.width, |ch| {
            let mut cell = Cell::new(ch);
            cell.fg = Some(LIGHT_GRAY);
            cell
        });
    }
}
