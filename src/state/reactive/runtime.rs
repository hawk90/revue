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

// Tests that access private fields stay inline
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mark_dirty_duplicate() {
        let mut runtime = ReactiveRuntime::new();
        runtime.mark_dirty(SignalId(1));
        runtime.mark_dirty(SignalId(1)); // duplicate
        assert!(runtime.has_pending());
        assert_eq!(runtime.dirty.len(), 1); // accesses private field
    }

    #[test]
    fn test_flush_clears_effects() {
        let mut runtime = ReactiveRuntime::new();
        runtime.schedule_effect(Box::new(|| {}));
        assert!(runtime.has_pending());

        runtime.flush();
        assert!(!runtime.has_pending());
        assert!(runtime.pending_effects.is_empty()); // accesses private field
    }
}
