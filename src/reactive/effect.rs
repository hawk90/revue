//! Side effects that run when dependencies change
//!
//! Effects automatically track their dependencies and re-run when those
//! dependencies change.

use super::tracker::{dispose_subscriber, start_tracking, stop_tracking, Subscriber, SubscriberId};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

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

        // Self-referential callback pattern:
        //
        // When a signal changes, it needs to re-run the effect AND re-register
        // the same callback for future changes. This creates a chicken-and-egg
        // problem: the callback needs to reference itself to re-register.
        //
        // Solution: Use an RwLock<Option<CallbackType>> as an indirection layer.
        // 1. Create the cell with None
        // 2. Create the callback that reads from the cell to get "itself"
        // 3. Store the callback in the cell
        // 4. Now the callback can access itself through the cell
        //
        // This is similar to the "lazy initialization" pattern for self-referential
        // structures, but using interior mutability instead of unsafe code.
        type CallbackType = Arc<dyn Fn() + Send + Sync>;
        let callback_cell: Arc<RwLock<Option<CallbackType>>> = Arc::new(RwLock::new(None));

        let callback_cell_clone = callback_cell.clone();
        let callback: CallbackType = Arc::new(move || {
            // Get reference to ourselves for re-registration
            // SAFETY: callback_cell is always initialized before this closure can be called
            // - Initial call: stored at line 137-139 before any callback invocation
            // - Subsequent calls: callback persists in cell for the lifetime of the effect
            let self_callback = callback_cell_clone
                .read()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .as_ref()
                .expect("Callback must be initialized before invocation")
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
        *callback_cell
            .write()
            .unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(callback.clone());

        // Initial run with tracking
        let subscriber = Subscriber { id, callback };

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
