//! Sort column, view mode, process record and color scheme types

use crate::style::Color;

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
    #[deprecated(
        since = "3.5.0",
        note = "the monitor has no tree layout and shows the flat list; group by `ProcessInfo::parent_pid` yourself"
    )]
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
    /// Process row text color. `None` lets the stylesheet's `color` decide,
    /// falling back to white; `Some` outranks the stylesheet.
    pub name: Option<Color>,
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
            name: None,
            pid: Color::CYAN,
        }
    }
}
