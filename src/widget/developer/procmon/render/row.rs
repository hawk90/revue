//! One process row: PID, name, CPU%, MEM%, memory, status and command

use super::{put_chars, CMD_X};
use crate::render::Cell;
use crate::style::Color;
use crate::widget::developer::procmon::{ProcessInfo, ProcessMonitor};
use crate::widget::theme::LIGHT_GRAY;
use crate::widget::traits::RenderContext;

/// A row cell: `fg` on the row background (`None` unless selected)
fn row_cell(ch: char, fg: Color, bg: Option<Color>) -> Cell {
    let mut cell = Cell::new(ch);
    cell.fg = Some(fg);
    cell.bg = bg;
    cell
}

impl ProcessMonitor {
    /// The CPU% color: high above 80, medium above 30, low otherwise
    fn cpu_color(&self, cpu: f32) -> Color {
        if cpu > 80.0 {
            self.colors.high_cpu
        } else if cpu > 30.0 {
            self.colors.medium_cpu
        } else {
            self.colors.low_cpu
        }
    }

    /// The MEM% color: high above 10, the row color otherwise
    fn mem_color(&self, memory_percent: f32, row_fg: Color) -> Color {
        if memory_percent > 10.0 {
            self.colors.high_mem
        } else {
            row_fg
        }
    }

    /// Render the CPU%, MEM% and memory columns of `proc` on row `y`
    fn render_usage(
        &self,
        ctx: &mut RenderContext,
        y: u16,
        proc: &ProcessInfo,
        bg: Option<Color>,
        row_fg: Color,
    ) {
        let area = ctx.area;

        // CPU%
        let cpu_str = format!("{:>6.1}", proc.cpu);
        let cpu_color = self.cpu_color(proc.cpu);
        put_chars(ctx, 27, y, &cpu_str, area.width, |ch| {
            row_cell(ch, cpu_color, bg)
        });

        // MEM%
        let mem_pct_str = format!("{:>6.1}", proc.memory_percent);
        let mem_color = self.mem_color(proc.memory_percent, row_fg);
        put_chars(ctx, 34, y, &mem_pct_str, area.width, |ch| {
            row_cell(ch, mem_color, bg)
        });

        // MEM (bytes)
        let mem_str = format!("{:>7}", Self::format_bytes(proc.memory));
        put_chars(ctx, 41, y, &mem_str, area.width, |ch| {
            row_cell(ch, row_fg, bg)
        });
    }

    /// Render `proc` on row `y`, in `row_fg` where no column color applies
    pub(super) fn render_row(
        &self,
        ctx: &mut RenderContext,
        y: u16,
        proc: &ProcessInfo,
        is_selected: bool,
        row_fg: Color,
    ) {
        let area = ctx.area;

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
        put_chars(ctx, 0, y, &pid_str, area.width, |ch| {
            row_cell(ch, self.colors.pid, bg)
        });

        // Name (truncated)
        let name = crate::utils::truncate_to_width(&proc.name, 19);
        put_chars(ctx, 7, y, name, 26, |ch| row_cell(ch, row_fg, bg));

        self.render_usage(ctx, y, proc, bg, row_fg);

        // Status
        if area.width > 55 {
            let status = crate::utils::truncate_to_width(&proc.status, 8);
            put_chars(ctx, 49, y, status, 57, |ch| row_cell(ch, LIGHT_GRAY, bg));
        }

        // Command line (the name when the system gave none)
        if self.show_cmd {
            let cmd = if proc.cmd.is_empty() {
                &proc.name
            } else {
                &proc.cmd
            };
            ctx.put_str_with(CMD_X, y, cmd, area.width, |ch| row_cell(ch, LIGHT_GRAY, bg));
        }
    }
}
