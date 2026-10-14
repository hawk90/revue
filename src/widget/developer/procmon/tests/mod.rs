//! Process monitor tests on fixed process lists
//!
//! KEEP HERE - the only way to give the monitor a known list is the private
//! `set_process_list`; the public API reads the live system. Rows are drawn
//! from these lists; the stats bar (live CPU and memory) is not checked here.

mod list;
mod view;

use super::{ProcessInfo, ProcessMonitor};
use crate::layout::Rect;
use crate::render::Buffer;
use crate::widget::traits::{RenderContext, View};
use sysinfo::System;

/// A monitor that reads an empty system: quick to make, and its stats bar
/// is the same on every machine
fn bare() -> ProcessMonitor {
    ProcessMonitor::with_system(System::new())
}

/// A process with every field given
#[allow(clippy::too_many_arguments)]
fn process(
    pid: u32,
    name: &str,
    cpu: f32,
    memory: u64,
    memory_percent: f32,
    status: &str,
    cmd: &str,
    user: &str,
) -> ProcessInfo {
    ProcessInfo {
        pid,
        parent_pid: None,
        name: name.to_string(),
        cpu,
        memory,
        memory_percent,
        status: status.to_string(),
        cmd: cmd.to_string(),
        user: user.to_string(),
    }
}

/// `n` processes named `p00`, `p01`, ... with PIDs 100, 101, ...
fn numbered(n: usize) -> Vec<ProcessInfo> {
    (0..n)
        .map(|i| {
            process(
                100 + i as u32,
                &format!("p{i:02}"),
                0.0,
                0,
                0.0,
                "Run",
                "",
                "0",
            )
        })
        .collect()
}

/// A monitor holding `list` (sorted by PID, ascending, so the order is the
/// list's when the PIDs rise)
fn monitor_with(list: Vec<ProcessInfo>) -> ProcessMonitor {
    let mut monitor = bare().sort_by(super::ProcessSort::Pid).ascending(true);
    monitor.set_process_list(list);
    monitor
}

/// Draw `monitor` into a `w`x`h` buffer at the origin
fn draw(monitor: &ProcessMonitor, w: u16, h: u16) -> Buffer {
    let mut buffer = Buffer::new(w, h);
    monitor.render(&mut RenderContext::new(&mut buffer, Rect::new(0, 0, w, h)));
    buffer
}

/// Row `y` of `buffer` as text
fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).unwrap().symbol)
        .collect()
}

/// The PIDs of the process rows (from row 2), in screen order
fn pids_on_screen(buffer: &Buffer) -> Vec<u32> {
    (2..buffer.height())
        .filter_map(|y| row(buffer, y)[..6].trim().parse().ok())
        .collect()
}

/// The rows (from row 2) whose cells carry the selected background
fn highlighted_rows(monitor: &ProcessMonitor, buffer: &Buffer) -> Vec<u16> {
    (2..buffer.height())
        .filter(|&y| buffer.get(6, y).unwrap().bg == Some(monitor.colors.selected_bg))
        .collect()
}
