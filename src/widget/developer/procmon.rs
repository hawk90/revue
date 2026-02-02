//! Process Monitor widget (htop-style)
//!
//! Displays system processes with CPU/memory usage,
//! sorting, filtering, and process management.

#[cfg(feature = "sysinfo")]
use sysinfo::System;

use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::format_size_compact;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Sort column for process list
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ProcessSort {
    /// Sort by PID
    Pid,
    /// Sort by process name
    Name,
    /// Sort by CPU usage (default)
    #[default]
    Cpu,
    /// Sort by memory usage
    Memory,
    /// Sort by status
    Status,
}

/// Process display mode
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ProcessView {
    /// Show all processes
    #[default]
    All,
    /// Show only user processes
    User,
    /// Show tree view
    Tree,
}

/// Process information
#[derive(Clone, Debug)]
pub struct ProcessInfo {
    /// Process ID
    pub pid: u32,
    /// Parent PID
    pub parent_pid: Option<u32>,
    /// Process name
    pub name: String,
    /// CPU usage percentage
    pub cpu: f32,
    /// Memory usage in bytes
    pub memory: u64,
    /// Memory usage percentage
    pub memory_percent: f32,
    /// Process status
    pub status: String,
    /// Command line
    pub cmd: String,
    /// User
    pub user: String,
}

/// Color scheme for process monitor
#[derive(Clone, Debug)]
pub struct ProcColors {
    /// Header background
    pub header_bg: Color,
    /// Header foreground
    pub header_fg: Color,
    /// Selected row background
    pub selected_bg: Color,
    /// High CPU color
    pub high_cpu: Color,
    /// Medium CPU color
    pub medium_cpu: Color,
    /// Low CPU color
    pub low_cpu: Color,
    /// High memory color
    pub high_mem: Color,
    /// Process name color
    pub name: Color,
    /// PID color
    pub pid: Color,
}

impl Default for ProcColors {
    fn default() -> Self {
        Self {
            header_bg: Color::rgb(40, 40, 60),
            header_fg: Color::WHITE,
            selected_bg: Color::rgb(60, 80, 120),
            high_cpu: Color::RED,
            medium_cpu: Color::YELLOW,
            low_cpu: Color::GREEN,
            high_mem: Color::MAGENTA,
            name: Color::WHITE,
            pid: Color::CYAN,
        }
    }
}

/// Process Monitor widget
///
/// # Example
///
/// ```rust,ignore
/// use revue::prelude::*;
///
/// let mut monitor = ProcessMonitor::new();
/// monitor.refresh();  // Update process list
///
/// // Sort by memory
/// monitor.sort_by(ProcessSort::Memory);
///
/// // Filter by name
/// monitor.filter("rust");
/// ```
pub struct ProcessMonitor {
    /// System info handle
    system: System,
    /// Cached process list
    processes: Vec<ProcessInfo>,
    /// Sort column
    sort: ProcessSort,
    /// Sort ascending
    sort_asc: bool,
    /// Filter string
    filter: String,
    /// Selected row
    selected: usize,
    /// Scroll offset
    scroll: usize,
    /// View mode
    view: ProcessView,
    /// Colors
    colors: ProcColors,
    /// Show command line
    show_cmd: bool,
    /// Update interval (ms)
    update_interval: u64,
    /// Last update time
    last_update: std::time::Instant,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
}

impl ProcessMonitor {
    /// Create a new process monitor
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        Self {
            system: sys,
            processes: Vec::new(),
            sort: ProcessSort::default(),
            sort_asc: false,
            filter: String::new(),
            selected: 0,
            scroll: 0,
            view: ProcessView::default(),
            colors: ProcColors::default(),
            show_cmd: false,
            update_interval: 1000,
            last_update: std::time::Instant::now(),
            props: WidgetProps::new(),
        }
    }

    /// Set sort column
    pub fn sort_by(mut self, sort: ProcessSort) -> Self {
        self.sort = sort;
        self
    }

    /// Set sort direction
    pub fn ascending(mut self, asc: bool) -> Self {
        self.sort_asc = asc;
        self
    }

    /// Set view mode
    pub fn view(mut self, view: ProcessView) -> Self {
        self.view = view;
        self
    }

    /// Set colors
    pub fn colors(mut self, colors: ProcColors) -> Self {
        self.colors = colors;
        self
    }

    /// Show/hide command line
    pub fn show_cmd(mut self, show: bool) -> Self {
        self.show_cmd = show;
        self
    }

    /// Set update interval (ms)
    pub fn update_interval(mut self, ms: u64) -> Self {
        self.update_interval = ms;
        self
    }

    /// Set filter string
    pub fn filter(&mut self, filter: impl Into<String>) {
        self.filter = filter.into().to_lowercase();
        self.selected = 0;
        self.scroll = 0;
    }

    /// Clear filter
    pub fn clear_filter(&mut self) {
        self.filter.clear();
    }

    /// Toggle sort column
    pub fn toggle_sort(&mut self, column: ProcessSort) {
        if self.sort == column {
            self.sort_asc = !self.sort_asc;
        } else {
            self.sort = column;
            self.sort_asc = false;
        }
    }

    /// Refresh process list
    pub fn refresh(&mut self) {
        self.system.refresh_all();
        self.update_process_list();
        self.last_update = std::time::Instant::now();
    }

    /// Check if update is needed
    pub fn needs_update(&self) -> bool {
        self.last_update.elapsed().as_millis() >= self.update_interval as u128
    }

    /// Tick (auto-refresh if needed)
    pub fn tick(&mut self) {
        if self.needs_update() {
            self.refresh();
        }
    }

    /// Update process list from system
    fn update_process_list(&mut self) {
        let total_memory = self.system.total_memory() as f32;

        self.processes = self
            .system
            .processes()
            .iter()
            .map(|(pid, proc): (&sysinfo::Pid, &sysinfo::Process)| {
                let memory = proc.memory();
                ProcessInfo {
                    pid: pid.as_u32(),
                    parent_pid: proc.parent().map(|p| p.as_u32()),
                    name: proc.name().to_string_lossy().into_owned(),
                    cpu: proc.cpu_usage(),
                    memory,
                    memory_percent: (memory as f32 / total_memory) * 100.0,
                    status: format!("{:?}", proc.status()),
                    cmd: proc
                        .cmd()
                        .iter()
                        .map(|s| s.to_string_lossy().into_owned())
                        .collect::<Vec<_>>()
                        .join(" "),
                    user: proc.user_id().map(|u| u.to_string()).unwrap_or_default(),
                }
            })
            .filter(|p| {
                if self.filter.is_empty() {
                    true
                } else {
                    p.name.to_lowercase().contains(&self.filter)
                        || p.cmd.to_lowercase().contains(&self.filter)
                }
            })
            .collect();

        // Sort
        self.processes.sort_by(|a, b| {
            let ord = match self.sort {
                ProcessSort::Pid => a.pid.cmp(&b.pid),
                ProcessSort::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                ProcessSort::Cpu => a
                    .cpu
                    .partial_cmp(&b.cpu)
                    .unwrap_or(std::cmp::Ordering::Equal),
                ProcessSort::Memory => a.memory.cmp(&b.memory),
                ProcessSort::Status => a.status.cmp(&b.status),
            };
            if self.sort_asc {
                ord
            } else {
                ord.reverse()
            }
        });

        // Adjust selection
        if self.selected >= self.processes.len() {
            self.selected = self.processes.len().saturating_sub(1);
        }
    }

    /// Select next process
    pub fn select_next(&mut self) {
        if self.selected < self.processes.len().saturating_sub(1) {
            self.selected += 1;
        }
    }

    /// Select previous process
    pub fn select_prev(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    /// Page down
    pub fn page_down(&mut self, page_size: usize) {
        self.selected = (self.selected + page_size).min(self.processes.len().saturating_sub(1));
    }

    /// Page up
    pub fn page_up(&mut self, page_size: usize) {
        self.selected = self.selected.saturating_sub(page_size);
    }

    /// Get selected process
    pub fn selected_process(&self) -> Option<&ProcessInfo> {
        self.processes.get(self.selected)
    }

    /// Get process count
    pub fn process_count(&self) -> usize {
        self.processes.len()
    }

    /// Get system CPU usage
    pub fn cpu_usage(&self) -> f32 {
        self.system.global_cpu_usage()
    }

    /// Get system memory usage
    pub fn memory_usage(&self) -> (u64, u64) {
        (self.system.used_memory(), self.system.total_memory())
    }

    /// Format bytes to human readable
    fn format_bytes(bytes: u64) -> String {
        format_size_compact(bytes)
    }

    /// Render header
    fn render_header(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let y = area.y;

        // Header background
        for x in 0..area.width {
            let mut cell = Cell::new(' ');
            cell.bg = Some(self.colors.header_bg);
            ctx.buffer.set(area.x + x, y, cell);
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
            for (i, ch) in text.chars().enumerate() {
                if x_offset + i as u16 >= area.width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(self.colors.header_fg);
                cell.bg = Some(self.colors.header_bg);
                cell.modifier = Modifier::BOLD;
                ctx.buffer.set(area.x + x_offset + i as u16, y, cell);
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

        for (i, ch) in stats.chars().enumerate() {
            if i as u16 >= area.width {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(Color::rgb(150, 150, 150));
            ctx.buffer.set(area.x + i as u16, area.y + y, cell);
        }
    }
}

impl Default for ProcessMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl View for ProcessMonitor {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 40 || area.height < 5 {
            return;
        }

        // Stats bar
        self.render_stats(ctx, 0);

        // Header (row 1)
        let _header_ctx = RenderContext::new(
            ctx.buffer,
            crate::layout::Rect {
                x: area.x,
                y: area.y + 1,
                width: area.width,
                height: 1,
            },
        );
        // We need to create a new RenderContext properly
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
            let y = area.y + list_start + i as u16;
            let is_selected = scroll + i == self.selected;

            // Background
            if is_selected {
                for x in 0..area.width {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(self.colors.selected_bg);
                    ctx.buffer.set(area.x + x, y, cell);
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
                ctx.buffer.set(area.x + j as u16, y, cell);
            }

            // Name (truncated)
            let name: String = proc.name.chars().take(19).collect();
            for (j, ch) in name.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(self.colors.name);
                cell.bg = bg;
                ctx.buffer.set(area.x + 7 + j as u16, y, cell);
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
                ctx.buffer.set(area.x + 27 + j as u16, y, cell);
            }

            // MEM%
            let mem_pct_str = format!("{:>6.1}", proc.memory_percent);
            let mem_color = if proc.memory_percent > 10.0 {
                self.colors.high_mem
            } else {
                Color::WHITE
            };
            for (j, ch) in mem_pct_str.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(mem_color);
                cell.bg = bg;
                ctx.buffer.set(area.x + 34 + j as u16, y, cell);
            }

            // MEM (bytes)
            let mem_str = format!("{:>7}", Self::format_bytes(proc.memory));
            for (j, ch) in mem_str.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::WHITE);
                cell.bg = bg;
                ctx.buffer.set(area.x + 41 + j as u16, y, cell);
            }

            // Status
            if area.width > 55 {
                let status: String = proc.status.chars().take(8).collect();
                for (j, ch) in status.chars().enumerate() {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(Color::rgb(150, 150, 150));
                    cell.bg = bg;
                    ctx.buffer.set(area.x + 49 + j as u16, y, cell);
                }
            }
        }
    }

    crate::impl_view_meta!("ProcessMonitor");
}

impl_styled_view!(ProcessMonitor);
impl_props_builders!(ProcessMonitor);

/// Create a new process monitor
pub fn process_monitor() -> ProcessMonitor {
    ProcessMonitor::new()
}

/// Alias for htop-style monitor
pub fn htop() -> ProcessMonitor {
    ProcessMonitor::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_process_monitor_creation() {
        let monitor = ProcessMonitor::new();
        // Just verify the monitor can be created (process count may be 0 in some CI environments)
        let _ = monitor.process_count();
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(ProcessMonitor::format_bytes(500), "500B");
        assert_eq!(ProcessMonitor::format_bytes(1024), "1K");
        assert_eq!(ProcessMonitor::format_bytes(1024 * 1024), "1M");
        assert_eq!(ProcessMonitor::format_bytes(1024 * 1024 * 1024), "1G");
    }

    #[test]
    fn test_navigation() {
        let mut monitor = ProcessMonitor::new();
        monitor.refresh();

        let initial = monitor.selected;
        monitor.select_next();
        if monitor.process_count() > 1 {
            assert_eq!(monitor.selected, initial + 1);
        }
        monitor.select_prev();
        assert_eq!(monitor.selected, initial);
    }

    #[test]
    fn test_sorting() {
        let mut monitor = ProcessMonitor::new();
        monitor.toggle_sort(ProcessSort::Memory);
        assert_eq!(monitor.sort, ProcessSort::Memory);
        assert!(!monitor.sort_asc);

        monitor.toggle_sort(ProcessSort::Memory);
        assert!(monitor.sort_asc);
    }

    #[test]
    fn test_render() {
        let mut monitor = ProcessMonitor::new();
        monitor.refresh();

        let mut buffer = Buffer::new(100, 30);
        let area = Rect::new(0, 0, 100, 30);
        let mut ctx = RenderContext::new(&mut buffer, area);

        monitor.render(&mut ctx);
    }
}
