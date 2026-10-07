//! Process Monitor widget (htop-style)
//!
//! Displays system processes with CPU/memory usage,
//! sorting, filtering, and process management.

#[cfg(feature = "sysinfo")]
use sysinfo::System;

mod navigation;
mod refresh;
mod render;
mod types;

pub use types::{ProcColors, ProcessInfo, ProcessSort, ProcessView};

use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

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
}

impl Default for ProcessMonitor {
    fn default() -> Self {
        Self::new()
    }
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
