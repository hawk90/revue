//! Reactive state management system.
//!
//! Vue/SolidJS-inspired reactivity with automatic dependency tracking.
//! Create reactive state with [`signal()`], derived values with [`computed()`],
//! and side effects with [`effect()`].
//!
//! # Core Primitives
//!
//! | Primitive | Description | Use Case |
//! |-----------|-------------|----------|
//! | [`Signal`] | Reactive value | Mutable state |
//! | [`Computed`] | Derived value | Automatic recalculation |
//! | [`Effect`] | Side effect | React to changes |
//!
//! # Quick Start
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! // Create reactive state
//! let count = signal(0);
//!
//! // Derived value (auto-updates when count changes)
//! let doubled = computed(move || count.get() * 2);
//!
//! // Side effect (runs when dependencies change)
//! effect(move || {
//!     println!("Count is now: {}", count.get());
//! });
//!
//! count.set(5);
//! // Output: "Count is now: 5"
//! // doubled.get() returns 10
//! ```
//!
//! # Signal
//!
//! A [`Signal`] holds a reactive value that can be read and written:
//!
//! ```rust,ignore
//! let name = signal(String::from("World"));
//!
//! // Read the value
//! println!("Hello, {}!", name.get());
//!
//! // Update the value
//! name.set(String::from("Revue"));
//!
//! // Update based on current value
//! name.update(|n| n.push_str("!"));
//! ```
//!
//! # Computed
//!
//! A [`Computed`] value automatically recalculates when its dependencies change:
//!
//! ```rust,ignore
//! let items = signal(vec![1, 2, 3, 4, 5]);
//!
//! let sum = computed(move || items.get().iter().sum::<i32>());
//! let avg = computed(move || {
//!     let v = items.get();
//!     v.iter().sum::<i32>() as f64 / v.len() as f64
//! });
//!
//! println!("Sum: {}, Avg: {}", sum.get(), avg.get()); // 15, 3.0
//!
//! items.update(|v| v.push(10));
//! println!("Sum: {}, Avg: {}", sum.get(), avg.get()); // 25, ~4.17
//! ```
//!
//! # Effect
//!
//! An [`Effect`] runs a callback whenever its dependencies change:
//!
//! ```rust,ignore
//! let theme = signal("dark");
//!
//! effect(move || {
//!     let current = theme.get();
//!     apply_theme(current);
//! });
//!
//! theme.set("light"); // apply_theme("light") is called
//! ```
//!
//! # Best Practices
//!
//! 1. **Keep signals granular**: Prefer multiple small signals over one large object
//! 2. **Use computed for derived state**: Don't duplicate state that can be calculated
//! 3. **Avoid side effects in computed**: Use `effect` for side effects
//! 4. **Clone signals freely**: Signals are cheap to clone (reference-counted)

mod async_state;
mod batch;
mod computed;
mod effect;
mod runtime;
mod signal;
mod tracker;

pub use async_state::{
    use_async, use_async_immediate, use_async_poll, AsyncResource, AsyncResult, AsyncState,
};
pub use batch::{
    batch, batch_count, batch_depth, end_batch, flush, is_batching, pending_count, queue_update,
    start_batch, BatchGuard, Transaction,
};
pub use computed::Computed;
pub use effect::Effect;
pub use runtime::ReactiveRuntime;
pub use signal::Signal;
pub use tracker::{
    dispose_subscriber, is_tracking, notify_dependents, start_tracking, stop_tracking, track_read,
    with_tracker, DependencyTracker, Subscriber, SubscriberCallback, SubscriberId,
};

use std::sync::atomic::{AtomicU64, Ordering};

/// Unique identifier for signals
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SignalId(u64);

impl SignalId {
    /// Create a new unique signal identifier
    pub fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

impl Default for SignalId {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a new reactive signal
pub fn signal<T: Clone + 'static>(value: T) -> Signal<T> {
    Signal::new(value)
}

/// Create a computed value
///
/// The closure must be `Send + Sync` since Signals are thread-safe.
pub fn computed<T: Clone + Send + Sync + 'static>(
    f: impl Fn() -> T + Send + Sync + 'static,
) -> Computed<T> {
    Computed::new(f)
}

/// Create a side effect
///
/// The closure must be `Send + Sync` since Signals are thread-safe.
/// This is automatically satisfied when capturing Signals.
pub fn effect(f: impl Fn() + Send + Sync + 'static) -> Effect {
    Effect::new(f)
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::sync::RwLock;

    #[test]
    fn test_automatic_dependency_tracking() {
        // Create a signal
        let count = signal(0);
        let run_count = Arc::new(AtomicUsize::new(0));

        // Create an effect that reads the signal
        let run_count_clone = run_count.clone();
        let count_clone = count.clone();
        let _effect = effect(move || {
            let _ = count_clone.get(); // This should register dependency
            run_count_clone.fetch_add(1, Ordering::SeqCst);
        });

        // Effect runs once on creation
        assert_eq!(run_count.load(Ordering::SeqCst), 1);

        // Changing the signal should trigger the effect
        count.set(1);
        assert_eq!(run_count.load(Ordering::SeqCst), 2);

        // Changing again
        count.set(2);
        assert_eq!(run_count.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn test_multiple_signals_dependency() {
        let a = signal(1);
        let b = signal(2);
        let sum = Arc::new(AtomicUsize::new(0));
        let run_count = Arc::new(AtomicUsize::new(0));

        let sum_clone = sum.clone();
        let run_count_clone = run_count.clone();
        let a_clone = a.clone();
        let b_clone = b.clone();
        let _effect = effect(move || {
            sum_clone.store((a_clone.get() + b_clone.get()) as usize, Ordering::SeqCst);
            run_count_clone.fetch_add(1, Ordering::SeqCst);
        });

        // Initial run
        assert_eq!(sum.load(Ordering::SeqCst), 3);
        assert_eq!(run_count.load(Ordering::SeqCst), 1);

        // Change a
        a.set(10);
        assert_eq!(sum.load(Ordering::SeqCst), 12);
        assert_eq!(run_count.load(Ordering::SeqCst), 2);

        // Change b
        b.set(20);
        assert_eq!(sum.load(Ordering::SeqCst), 30);
        assert_eq!(run_count.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn test_effect_stop_removes_tracking() {
        let count = signal(0);
        let run_count = Arc::new(AtomicUsize::new(0));

        let run_count_clone = run_count.clone();
        let count_clone = count.clone();
        let effect_handle = effect(move || {
            let _ = count_clone.get();
            run_count_clone.fetch_add(1, Ordering::SeqCst);
        });

        assert_eq!(run_count.load(Ordering::SeqCst), 1);

        // Stop the effect
        effect_handle.stop();

        // Changes should NOT trigger the effect anymore
        count.set(1);
        assert_eq!(run_count.load(Ordering::SeqCst), 1); // Still 1, not 2
    }

    #[test]
    fn test_effect_with_borrow() {
        let items = signal(vec![1, 2, 3]);
        let sum = Arc::new(AtomicUsize::new(0));

        let sum_clone = sum.clone();
        let items_clone = items.clone();
        let _effect = effect(move || {
            // Using borrow() should also track dependency
            let s: i32 = items_clone.borrow().iter().sum();
            sum_clone.store(s as usize, Ordering::SeqCst);
        });

        assert_eq!(sum.load(Ordering::SeqCst), 6);

        items.update(|v| v.push(4));
        assert_eq!(sum.load(Ordering::SeqCst), 10);
    }

    #[test]
    fn test_effect_with_with() {
        let name = signal(String::from("World"));
        let greeting = Arc::new(RwLock::new(String::new()));

        let greeting_clone = greeting.clone();
        let name_clone = name.clone();
        let _effect = effect(move || {
            // Using with() should also track dependency
            let g = name_clone.with(|n| format!("Hello, {}!", n));
            *greeting_clone.write().unwrap() = g;
        });

        assert_eq!(*greeting.read().unwrap(), "Hello, World!");

        name.set(String::from("Revue"));
        assert_eq!(*greeting.read().unwrap(), "Hello, Revue!");
    }

    #[test]
    fn test_effect_dropped_cleans_up() {
        let count = signal(0);
        let run_count = Arc::new(AtomicUsize::new(0));

        {
            let run_count_clone = run_count.clone();
            let count_clone = count.clone();
            let _effect = effect(move || {
                let _ = count_clone.get();
                run_count_clone.fetch_add(1, Ordering::SeqCst);
            });

            assert_eq!(run_count.load(Ordering::SeqCst), 1);
            count.set(1);
            assert_eq!(run_count.load(Ordering::SeqCst), 2);
            // Effect is dropped here
        }

        // After effect is dropped, changes should NOT trigger it
        count.set(2);
        assert_eq!(run_count.load(Ordering::SeqCst), 2); // Still 2, not 3
    }

    #[test]
    fn test_conditional_dependency() {
        let flag = signal(true);
        let a = signal(1);
        let b = signal(2);
        let result = Arc::new(AtomicUsize::new(0));
        let run_count = Arc::new(AtomicUsize::new(0));

        let result_clone = result.clone();
        let run_count_clone = run_count.clone();
        let flag_clone = flag.clone();
        let a_clone = a.clone();
        let b_clone = b.clone();

        let _effect = effect(move || {
            let value = if flag_clone.get() {
                a_clone.get()
            } else {
                b_clone.get()
            };
            result_clone.store(value as usize, Ordering::SeqCst);
            run_count_clone.fetch_add(1, Ordering::SeqCst);
        });

        // Initial: flag=true, so we depend on `a`
        assert_eq!(result.load(Ordering::SeqCst), 1);
        assert_eq!(run_count.load(Ordering::SeqCst), 1);

        // Changing `a` triggers effect (because we depend on it)
        a.set(10);
        assert_eq!(result.load(Ordering::SeqCst), 10);
        assert_eq!(run_count.load(Ordering::SeqCst), 2);

        // Note: In our simple implementation, changing `b` WILL trigger
        // because we track all reads that happen during the effect.
        // A more sophisticated system would use fine-grained tracking.
    }
}
