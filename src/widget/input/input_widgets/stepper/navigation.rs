//! Moving between steps, marking their status and reading progress

use super::{Step, StepStatus, Stepper};

impl Stepper {
    /// Go to next step
    pub fn next_step(&mut self) -> bool {
        if self.current < self.steps.len().saturating_sub(1) {
            self.current += 1;
            self.update_statuses();
            true
        } else {
            false
        }
    }

    /// Go to previous step
    pub fn prev(&mut self) -> bool {
        if self.current > 0 {
            self.current -= 1;
            self.update_statuses();
            true
        } else {
            false
        }
    }

    /// Go to specific step
    pub fn go_to(&mut self, index: usize) {
        if index < self.steps.len() {
            self.current = index;
            self.update_statuses();
        }
    }

    /// Complete current step and advance
    pub fn complete_current(&mut self) {
        if let Some(step) = self.steps.get_mut(self.current) {
            step.status = StepStatus::Completed;
        }
        self.next_step();
    }

    /// Mark step as error
    pub fn mark_error(&mut self, index: usize) {
        if let Some(step) = self.steps.get_mut(index) {
            step.status = StepStatus::Error;
        }
    }

    /// Mark step as skipped
    pub fn skip(&mut self, index: usize) {
        if let Some(step) = self.steps.get_mut(index) {
            step.status = StepStatus::Skipped;
        }
    }

    /// Get current step
    pub fn current_step(&self) -> Option<&Step> {
        self.steps.get(self.current)
    }

    /// Get step count
    pub fn len(&self) -> usize {
        self.steps.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    /// Check if completed (on last step and it's completed)
    pub fn is_completed(&self) -> bool {
        self.steps
            .last()
            .is_some_and(|s| s.status == StepStatus::Completed)
    }

    /// Get progress as percentage
    pub fn progress(&self) -> f64 {
        if self.steps.is_empty() {
            return 0.0;
        }
        let completed = self
            .steps
            .iter()
            .filter(|s| s.status == StepStatus::Completed)
            .count();
        completed as f64 / self.steps.len() as f64
    }
}
