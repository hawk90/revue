//! The selected event: set, move and read it

use super::{Timeline, TimelineEvent};

impl Timeline {
    /// Select an event
    pub fn select(&mut self, index: Option<usize>) {
        self.selected = index;
    }

    /// Select next event
    pub fn select_next(&mut self) {
        match self.selected {
            Some(i) if i < self.events.len() - 1 => self.selected = Some(i + 1),
            None if !self.events.is_empty() => self.selected = Some(0),
            _ => {}
        }
    }

    /// Select previous event
    pub fn select_prev(&mut self) {
        match self.selected {
            Some(i) if i > 0 => self.selected = Some(i - 1),
            _ => {}
        }
    }

    /// Get selected event
    pub fn selected_event(&self) -> Option<&TimelineEvent> {
        self.selected.and_then(|i| self.events.get(i))
    }
}
