//! Moving the selection through the process list

use super::{ProcessInfo, ProcessMonitor};

impl ProcessMonitor {
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
}
