//! Reading processes from the system, filtering and sorting them, and system totals

use super::{ProcessInfo, ProcessMonitor, ProcessSort, ProcessView};
use sysinfo::System;

impl ProcessMonitor {
    /// Set filter string
    pub fn filter(&mut self, filter: impl Into<String>) {
        self.filter = filter.into().to_lowercase();
        self.apply_list_settings();
    }

    /// Clear filter
    pub fn clear_filter(&mut self) {
        self.filter.clear();
        self.apply_list_settings();
    }

    /// Toggle sort column
    pub fn toggle_sort(&mut self, column: ProcessSort) {
        if self.sort == column {
            self.sort_asc = !self.sort_asc;
        } else {
            self.sort = column;
            self.sort_asc = false;
        }
        self.apply_list_settings();
    }

    /// Refresh process list
    pub fn refresh(&mut self) {
        if let Some(system) = &mut self.system {
            system.refresh_all();
        }
        self.update_process_list();
        self.last_update = crate::utils::clock::now();
    }

    /// Check if update is needed
    pub fn needs_update(&self) -> bool {
        crate::utils::clock::since(self.last_update).as_millis() >= self.update_interval as u128
    }

    /// Tick (auto-refresh if needed)
    pub fn tick(&mut self) {
        if self.needs_update() {
            self.refresh();
        }
    }

    /// Update process list from system
    fn update_process_list(&mut self) {
        let Some(system) = &self.system else {
            self.current_user = None;
            self.set_process_list(Vec::new());
            return;
        };
        let total_memory = system.total_memory();

        let all = system
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
                    memory_percent: memory_percent(memory, total_memory),
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
            .collect();
        self.current_user = sysinfo::get_current_pid()
            .ok()
            .and_then(|pid| system.process(pid))
            .and_then(|proc| proc.user_id())
            .map(|uid| uid.to_string());
        self.set_process_list(all);
    }

    /// Keep a freshly read process list and list it through the view, the
    /// filter and the sort
    pub(super) fn set_process_list(&mut self, all: Vec<ProcessInfo>) {
        self.all = all;
        self.apply_list_settings();
    }

    /// List the last list read through the current view, filter and sort
    pub(super) fn apply_list_settings(&mut self) {
        self.processes = self
            .all
            .iter()
            // The `User` view keeps the processes owned by whoever runs this
            // one; when that is unknown it keeps them all.
            .filter(|p| match (self.view, &self.current_user) {
                (ProcessView::User, Some(user)) => &p.user == user,
                _ => true,
            })
            .filter(|p| {
                if self.filter.is_empty() {
                    true
                } else {
                    p.name.to_lowercase().contains(&self.filter)
                        || p.cmd.to_lowercase().contains(&self.filter)
                }
            })
            .cloned()
            .collect();

        // Sort
        self.processes.sort_by(|a, b| {
            let ord = match self.sort {
                ProcessSort::Pid => a.pid.cmp(&b.pid),
                ProcessSort::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                ProcessSort::Cpu => a.cpu.total_cmp(&b.cpu),
                ProcessSort::Memory => a.memory.cmp(&b.memory),
                ProcessSort::Status => a.status.cmp(&b.status),
            };
            if self.sort_asc {
                ord
            } else {
                ord.reverse()
            }
        });

        // The selection stays on its process, or the one now in its place
        // when it exited or was filtered out; with none, the first
        self.selected.sync(self.processes.iter().map(|p| p.pid));
        if self.selected.index().is_none() {
            self.selected
                .first(self.processes.len(), |i| self.processes[i].pid);
        }
    }

    /// Get system CPU usage
    pub fn cpu_usage(&self) -> f32 {
        self.system.as_ref().map_or(0.0, System::global_cpu_usage)
    }

    /// Get system memory usage
    pub fn memory_usage(&self) -> (u64, u64) {
        self.system.as_ref().map_or((0, 0), |system| {
            (system.used_memory(), system.total_memory())
        })
    }
}

/// `memory` as a percentage of `total`; 0 when the system reports no total
/// memory (sysinfo gives 0 where it cannot read it)
pub(super) fn memory_percent(memory: u64, total: u64) -> f32 {
    if total == 0 {
        0.0
    } else {
        memory as f32 / total as f32 * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_process_list() {
        // The User view keeps the processes of whoever runs this one
        let mut monitor = ProcessMonitor::new().view(ProcessView::User);
        monitor.refresh();
        let me = monitor
            .processes
            .iter()
            .find(|p| p.pid == std::process::id())
            .expect("this test process is listed");
        if let Some(user) = &monitor.current_user {
            assert_eq!(&me.user, user);
            assert!(monitor.processes.iter().all(|p| &p.user == user));
        }
        // MEM% is the share of the system's memory
        let total = monitor.system.as_ref().map_or(0, System::total_memory) as f32;
        assert_eq!(me.memory_percent, me.memory as f32 / total * 100.0);
    }
}
