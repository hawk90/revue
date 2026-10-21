//! Moving the selection through the process list

use super::{ProcessInfo, ProcessMonitor};

impl ProcessMonitor {
    /// Select next process
    pub fn select_next(&mut self) {
        self.move_selection(1);
    }

    /// Select previous process
    pub fn select_prev(&mut self) {
        self.move_selection(-1);
    }

    /// Page down
    pub fn page_down(&mut self, page_size: usize) {
        self.move_selection(page_size.min(isize::MAX as usize) as isize);
    }

    /// Page up
    pub fn page_up(&mut self, page_size: usize) {
        self.move_selection(-(page_size.min(isize::MAX as usize) as isize));
    }

    /// Move the selection by `delta` rows, stopping at either end
    fn move_selection(&mut self, delta: isize) {
        let processes = &self.processes;
        self.selected
            .move_by(delta, processes.len(), |i| processes[i].pid);
    }

    /// Get selected process
    pub fn selected_process(&self) -> Option<&ProcessInfo> {
        self.processes.get(self.selected.index()?)
    }

    /// Get process count
    pub fn process_count(&self) -> usize {
        self.processes.len()
    }
}
