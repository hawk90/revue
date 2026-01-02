//! Reactive runtime for managing updates

use super::SignalId;
use std::collections::HashSet;

/// Runtime for managing reactive updates
pub struct ReactiveRuntime {
    dirty: HashSet<SignalId>,
    pending_effects: Vec<Box<dyn Fn()>>,
}

impl ReactiveRuntime {
    /// Create a new reactive runtime
    pub fn new() -> Self {
        Self {
            dirty: HashSet::new(),
            pending_effects: Vec::new(),
        }
    }

    /// Mark a signal as dirty
    pub fn mark_dirty(&mut self, id: SignalId) {
        self.dirty.insert(id);
    }

    /// Schedule an effect to run
    pub fn schedule_effect(&mut self, f: Box<dyn Fn()>) {
        self.pending_effects.push(f);
    }

    /// Flush all pending updates
    pub fn flush(&mut self) {
        self.dirty.clear();
        let effects = std::mem::take(&mut self.pending_effects);
        for effect in effects {
            effect();
        }
    }

    /// Check if there are pending updates
    pub fn has_pending(&self) -> bool {
        !self.dirty.is_empty() || !self.pending_effects.is_empty()
    }
}

impl Default for ReactiveRuntime {
    fn default() -> Self {
        Self::new()
    }
}
