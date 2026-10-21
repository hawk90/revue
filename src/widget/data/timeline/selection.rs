//! The selected event: set, move and read it

use super::{Timeline, TimelineEvent};

impl Timeline {
    /// Select an event. An index past the end selects the last event, and
    /// with no events nothing is selected (#975).
    pub fn select(&mut self, index: Option<usize>) {
        self.selected =
            index.and_then(|i| self.events.len().checked_sub(1).map(|last| i.min(last)));
    }

    /// Select next event
    pub fn select_next(&mut self) {
        let len = self.events.len();
        self.selected = match self.selected {
            _ if len == 0 => None,
            Some(i) => Some((i + 1).min(len - 1)),
            None => Some(0),
        };
    }

    /// Select previous event
    pub fn select_prev(&mut self) {
        let len = self.events.len();
        self.selected = match self.selected {
            _ if len == 0 => None,
            Some(i) => Some(i.min(len - 1).saturating_sub(1)),
            None => None,
        };
    }

    /// Get selected event
    pub fn selected_event(&self) -> Option<&TimelineEvent> {
        self.selected.and_then(|i| self.events.get(i))
    }
}
