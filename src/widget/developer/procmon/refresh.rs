//! Reading processes from the system, filtering and sorting them, and system totals

use super::{ProcessInfo, ProcessMonitor, ProcessSort};

impl ProcessMonitor {
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

    /// Get system CPU usage
    pub fn cpu_usage(&self) -> f32 {
        self.system.global_cpu_usage()
    }

    /// Get system memory usage
    pub fn memory_usage(&self) -> (u64, u64) {
        (self.system.used_memory(), self.system.total_memory())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_process_list() {
        // Test private method implementation
        let mut monitor = ProcessMonitor::new();
        // Just ensure the method exists and can be called
        // Actual functionality depends on sysinfo being available
        if cfg!(feature = "sysinfo") {
            monitor.refresh();
            let _count = monitor.process_count(); // Just verify the method works
        }
    }
}
