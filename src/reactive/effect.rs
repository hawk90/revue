//! Side effects that run when dependencies change
//!
//! Effects automatically track their dependencies and re-run when those
//! dependencies change.

use super::tracker::{start_tracking, stop_tracking, dispose_subscriber, Subscriber, SubscriberId};
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};

/// A side effect that runs when its dependencies change
///
/// Effects automatically track which signals they read and re-run when any
/// of those signals change. This is the core of the reactive system.
///
/// # Example
///
/// ```rust,ignore
/// let count = signal(0);
///
/// // This effect automatically tracks `count` as a dependency
/// let _effect = Effect::new(move || {
///     println!("Count: {}", count.get());
/// });
///
/// count.set(1); // Effect re-runs, prints "Count: 1"
/// count.set(2); // Effect re-runs, prints "Count: 2"
/// ```
///
/// # Thread Safety
///
/// Effect closures must be `Send + Sync` since Signals are now thread-safe.
/// This is automatically satisfied when capturing Signals.
pub struct Effect {
    /// The effect function wrapped in Arc for sharing with tracker
    effect_fn: Arc<dyn Fn() + Send + Sync>,
    /// Whether the effect is currently active
    active: Arc<AtomicBool>,
    /// Unique ID for this effect (for dependency tracking)
    id: SubscriberId,
}

impl Effect {
    /// Create a new effect that runs immediately with automatic dependency tracking
    pub fn new(f: impl Fn() + Send + Sync + 'static) -> Self {
        let active = Arc::new(AtomicBool::new(true));
        let id = SubscriberId::new();

        // Wrap the user function to include active check
        let active_clone = active.clone();
        let effect_fn: Arc<dyn Fn() + Send + Sync> = Arc::new(move || {
            if active_clone.load(Ordering::SeqCst) {
                f();
            }
        });

        let effect = Self {
            effect_fn,
            active,
            id,
        };

        // Run immediately on creation (with tracking)
        effect.run_tracked();
        effect
    }

    /// Create an effect without running immediately (lazy initialization)
    pub fn lazy(f: impl Fn() + Send + Sync + 'static) -> Self {
        let active = Arc::new(AtomicBool::new(true));
        let id = SubscriberId::new();

        let active_clone = active.clone();
        let effect_fn: Arc<dyn Fn() + Send + Sync> = Arc::new(move || {
            if active_clone.load(Ordering::SeqCst) {
                f();
            }
        });

        Self {
            effect_fn,
            active,
            id,
        }
    }

    /// Run the effect with dependency tracking
    ///
    /// This clears old dependencies and tracks new ones based on which
    /// signals are read during execution.
    fn run_tracked(&self) {
        if !self.active.load(Ordering::SeqCst) {
            return;
        }

        let effect_fn = self.effect_fn.clone();
        let id = self.id;

        // Create self-referencing callback using RwLock
        type CallbackType = Arc<dyn Fn() + Send + Sync>;
        let callback_cell: Arc<RwLock<Option<CallbackType>>> = Arc::new(RwLock::new(None));

        let callback_cell_clone = callback_cell.clone();
        let callback: CallbackType = Arc::new(move || {
            // Get reference to ourselves for re-registration
            let self_callback = callback_cell_clone
                .read()
                .expect("Callback lock poisoned")
                .as_ref()
                .expect("Callback not initialized")
                .clone();

            // Re-establish tracking with same callback
            let subscriber = Subscriber {
                id,
                callback: self_callback,
            };

            start_tracking(subscriber);
            effect_fn();
            stop_tracking();
        });

        // Store callback in cell so it can reference itself
        *callback_cell.write().expect("Callback lock poisoned") = Some(callback.clone());

        // Initial run with tracking
        let subscriber = Subscriber {
            id,
            callback,
        };

        start_tracking(subscriber);
        (self.effect_fn)();
        stop_tracking();
    }

    /// Run the effect if active (manual run, also tracks dependencies)
    pub fn run(&self) {
        self.run_tracked();
    }

    /// Stop the effect from running and clear its dependencies
    pub fn stop(&self) {
        self.active.store(false, Ordering::SeqCst);
        dispose_subscriber(self.id);
    }

    /// Resume the effect (will need to be run manually to re-establish dependencies)
    pub fn resume(&self) {
        self.active.store(true, Ordering::SeqCst);
    }

    /// Check if effect is active
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::SeqCst)
    }

    /// Get the effect's unique ID
    pub fn id(&self) -> SubscriberId {
        self.id
    }
}

impl Drop for Effect {
    fn drop(&mut self) {
        self.active.store(false, Ordering::SeqCst);
        dispose_subscriber(self.id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;

    #[test]
    fn test_effect_runs_immediately() {
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = called.clone();

        let _effect = Effect::new(move || {
            called_clone.store(true, Ordering::SeqCst);
        });

        assert!(called.load(Ordering::SeqCst));
    }

    #[test]
    fn test_effect_lazy_does_not_run() {
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = called.clone();

        let effect = Effect::lazy(move || {
            called_clone.store(true, Ordering::SeqCst);
        });

        assert!(!called.load(Ordering::SeqCst));

        effect.run();
        assert!(called.load(Ordering::SeqCst));
    }

    #[test]
    fn test_effect_stop_and_resume() {
        let count = Arc::new(AtomicUsize::new(0));
        let count_clone = count.clone();

        let effect = Effect::lazy(move || {
            count_clone.fetch_add(1, Ordering::SeqCst);
        });

        effect.run();
        assert_eq!(count.load(Ordering::SeqCst), 1);

        effect.stop();
        effect.run();
        assert_eq!(count.load(Ordering::SeqCst), 1); // Still 1, didn't run

        effect.resume();
        effect.run();
        assert_eq!(count.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn test_effect_is_active() {
        let effect = Effect::lazy(|| {});

        assert!(effect.is_active());

        effect.stop();
        assert!(!effect.is_active());

        effect.resume();
        assert!(effect.is_active());
    }

    #[test]
    fn test_effect_dynamic_dependency_retracking() {
        use crate::reactive::signal;

        let flag = signal(true);
        let a = signal(1);
        let b = signal(2);
        let result = Arc::new(AtomicUsize::new(0));

        let res = result.clone();
        let f = flag.clone();
        let a_c = a.clone();
        let b_c = b.clone();

        let _effect = Effect::new(move || {
            // Dynamic dependency: read a or b based on flag
            let val = if f.get() {
                a_c.get()
            } else {
                b_c.get()
            };
            res.store(val as usize, Ordering::SeqCst);
        });

        // Initially: flag=true, depends on a
        assert_eq!(result.load(Ordering::SeqCst), 1);

        // Change a - should trigger effect
        a.set(10);
        assert_eq!(result.load(Ordering::SeqCst), 10);

        // Change flag - effect should re-track and now depend on b
        flag.set(false);
        assert_eq!(result.load(Ordering::SeqCst), 2);

        // Change b - should NOW trigger effect (dynamic re-tracking!)
        b.set(20);
        assert_eq!(result.load(Ordering::SeqCst), 20);

        // Change a - should NOT trigger effect anymore (we no longer depend on it)
        a.set(100);
        assert_eq!(result.load(Ordering::SeqCst), 20); // Still 20, not affected by a
    }
}
