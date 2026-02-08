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
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::Arc;

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

    // Public API tests
    #[test]
    fn test_reactive_runtime_new() {
        let runtime = ReactiveRuntime::new();
        assert!(!runtime.has_pending());
    }

    #[test]
    fn test_reactive_runtime_default() {
        let runtime = ReactiveRuntime::default();
        assert!(!runtime.has_pending());
    }

    #[test]
    fn test_reactive_runtime_mark_dirty() {
        let mut runtime = ReactiveRuntime::new();
        runtime.mark_dirty(SignalId(1));
        assert!(runtime.has_pending());
    }

    #[test]
    fn test_reactive_runtime_mark_dirty_multiple() {
        let mut runtime = ReactiveRuntime::new();
        runtime.mark_dirty(SignalId(1));
        runtime.mark_dirty(SignalId(2));
        runtime.mark_dirty(SignalId(3));
        assert!(runtime.has_pending());
    }

    #[test]
    fn test_reactive_runtime_schedule_effect() {
        let mut runtime = ReactiveRuntime::new();
        runtime.schedule_effect(Box::new(|| {}));
        assert!(runtime.has_pending());
    }

    #[test]
    fn test_reactive_runtime_schedule_effect_multiple() {
        let mut runtime = ReactiveRuntime::new();
        runtime.schedule_effect(Box::new(|| {}));
        runtime.schedule_effect(Box::new(|| {}));
        runtime.schedule_effect(Box::new(|| {}));
        assert!(runtime.has_pending());
    }

    #[test]
    fn test_reactive_runtime_flush_executes_effects() {
        let executed = Arc::new(AtomicBool::new(false));
        let executed_clone = executed.clone();

        let mut runtime = ReactiveRuntime::new();
        runtime.schedule_effect(Box::new(move || {
            executed_clone.store(true, Ordering::SeqCst);
        }));

        assert!(!executed.load(Ordering::SeqCst));
        runtime.flush();
        assert!(executed.load(Ordering::SeqCst));
    }

    #[test]
    fn test_reactive_runtime_flush_multiple_effects() {
        let count = Arc::new(AtomicUsize::new(0));

        let mut runtime = ReactiveRuntime::new();
        for _ in 0..5 {
            let count_clone = count.clone();
            runtime.schedule_effect(Box::new(move || {
                count_clone.fetch_add(1, Ordering::SeqCst);
            }));
        }

        runtime.flush();
        assert_eq!(count.load(Ordering::SeqCst), 5);
    }

    #[test]
    fn test_reactive_runtime_flush_clears_pending() {
        let mut runtime = ReactiveRuntime::new();
        runtime.mark_dirty(SignalId(1));
        runtime.schedule_effect(Box::new(|| {}));

        assert!(runtime.has_pending());
        runtime.flush();
        assert!(!runtime.has_pending());
    }

    #[test]
    fn test_reactive_runtime_has_pending_initially_false() {
        let runtime = ReactiveRuntime::new();
        assert!(!runtime.has_pending());
    }

    #[test]
    fn test_reactive_runtime_has_pending_after_mark_dirty() {
        let mut runtime = ReactiveRuntime::new();
        runtime.mark_dirty(SignalId(1));
        assert!(runtime.has_pending());
    }

    #[test]
    fn test_reactive_runtime_has_pending_after_schedule_effect() {
        let mut runtime = ReactiveRuntime::new();
        runtime.schedule_effect(Box::new(|| {}));
        assert!(runtime.has_pending());
    }

    #[test]
    fn test_reactive_runtime_flush_when_empty() {
        let mut runtime = ReactiveRuntime::new();
        runtime.flush(); // Should not panic
        assert!(!runtime.has_pending());
    }

    #[test]
    fn test_reactive_runtime_flush_idempotent() {
        let mut runtime = ReactiveRuntime::new();
        runtime.schedule_effect(Box::new(|| {}));

        runtime.flush();
        runtime.flush(); // Second flush should be fine
    }
}
