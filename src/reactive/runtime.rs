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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[test]
    fn test_new_runtime() {
        let runtime = ReactiveRuntime::new();
        assert!(!runtime.has_pending());
    }

    #[test]
    fn test_default_runtime() {
        let runtime = ReactiveRuntime::default();
        assert!(!runtime.has_pending());
    }

    #[test]
    fn test_mark_dirty() {
        let mut runtime = ReactiveRuntime::new();
        assert!(!runtime.has_pending());

        runtime.mark_dirty(SignalId(1));
        assert!(runtime.has_pending());

        runtime.mark_dirty(SignalId(2));
        assert!(runtime.has_pending());
    }

    #[test]
    fn test_mark_dirty_duplicate() {
        let mut runtime = ReactiveRuntime::new();
        runtime.mark_dirty(SignalId(1));
        runtime.mark_dirty(SignalId(1)); // duplicate
        assert!(runtime.has_pending());
        assert_eq!(runtime.dirty.len(), 1);
    }

    #[test]
    fn test_schedule_effect() {
        let mut runtime = ReactiveRuntime::new();
        assert!(!runtime.has_pending());

        runtime.schedule_effect(Box::new(|| {}));
        assert!(runtime.has_pending());
    }

    #[test]
    fn test_flush_clears_dirty() {
        let mut runtime = ReactiveRuntime::new();
        runtime.mark_dirty(SignalId(1));
        runtime.mark_dirty(SignalId(2));
        assert!(runtime.has_pending());

        runtime.flush();
        assert!(!runtime.has_pending());
    }

    #[test]
    fn test_flush_runs_effects() {
        let mut runtime = ReactiveRuntime::new();
        let counter = Arc::new(AtomicUsize::new(0));

        let counter_clone = counter.clone();
        runtime.schedule_effect(Box::new(move || {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        }));

        let counter_clone = counter.clone();
        runtime.schedule_effect(Box::new(move || {
            counter_clone.fetch_add(10, Ordering::SeqCst);
        }));

        assert_eq!(counter.load(Ordering::SeqCst), 0);
        runtime.flush();
        assert_eq!(counter.load(Ordering::SeqCst), 11);
    }

    #[test]
    fn test_flush_clears_effects() {
        let mut runtime = ReactiveRuntime::new();
        runtime.schedule_effect(Box::new(|| {}));
        assert!(runtime.has_pending());

        runtime.flush();
        assert!(!runtime.has_pending());
        assert!(runtime.pending_effects.is_empty());
    }

    #[test]
    fn test_has_pending_dirty_only() {
        let mut runtime = ReactiveRuntime::new();
        runtime.mark_dirty(SignalId(1));
        assert!(runtime.has_pending());
    }

    #[test]
    fn test_has_pending_effects_only() {
        let mut runtime = ReactiveRuntime::new();
        runtime.schedule_effect(Box::new(|| {}));
        assert!(runtime.has_pending());
    }

    #[test]
    fn test_has_pending_both() {
        let mut runtime = ReactiveRuntime::new();
        runtime.mark_dirty(SignalId(1));
        runtime.schedule_effect(Box::new(|| {}));
        assert!(runtime.has_pending());
    }
}
