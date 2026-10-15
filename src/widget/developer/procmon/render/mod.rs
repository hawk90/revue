//! Drawing the process monitor: stats bar, column header and process rows

mod header;
mod row;

use super::ProcessMonitor;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::format_size_compact;
use crate::widget::traits::{RenderContext, View};

/// The process-row text color a monitor uses when neither
/// [`ProcColors::name`](crate::widget::ProcColors::name) nor the stylesheet names one.
const PROC_FG: Color = Color::WHITE;

/// Where the command column starts, after the status column
const CMD_X: u16 = 59;

/// The column header sits under the stats bar (row 0).
const HEADER_ROW: u16 = 1;

/// Draw `text` from column `x` on row `y`, one cell per character built by
/// `make_cell`, advancing by each character's width and stopping before a
/// character that would pass column `max_x`
fn put_chars(
    ctx: &mut RenderContext,
    x: u16,
    y: u16,
    text: &str,
    max_x: u16,
    make_cell: impl Fn(char) -> Cell,
) {
    let mut cx = x;
    for ch in text.chars() {
        let cw = crate::utils::char_width(ch) as u16;
        if cx + cw > max_x {
            break;
        }
        ctx.set(cx, y, make_cell(ch));
        cx += cw;
    }
}

impl ProcessMonitor {
    /// Format bytes to human readable
    fn format_bytes(bytes: u64) -> String {
        format_size_compact(bytes)
    }

    /// The first process to draw when `visible_rows` fit: the stored scroll,
    /// moved just enough to keep the selection visible
    fn first_visible(&self, visible_rows: usize) -> usize {
        if self.selected < self.scroll {
            self.selected
        } else if self.selected >= self.scroll + visible_rows {
            self.selected - visible_rows + 1
        } else {
            self.scroll
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
        let scroll = self.first_visible(visible_rows);

        for (i, proc) in self
            .processes
            .iter()
            .skip(scroll)
            .take(visible_rows)
            .enumerate()
        {
            let y = list_start + i as u16;
            let is_selected = scroll + i == self.selected;
            self.render_row(ctx, y, proc, is_selected, row_fg);
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
