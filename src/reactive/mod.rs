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
mod context;
mod effect;
mod incremental;
mod runtime;
mod signal;
mod signal_vec;
pub mod store;
mod tracker;

pub use async_state::{
    use_async, use_async_immediate, use_async_poll, AsyncResource, AsyncResult, AsyncState,
};
pub use batch::{
    batch, batch_count, batch_depth, end_batch, flush, is_batching, pending_count, queue_update,
    start_batch, BatchGuard, Transaction,
};
pub use computed::Computed;
pub use context::{
    clear_all_contexts, clear_context, create_context, create_context_with_default, has_context,
    provide, provide_signal, use_context, use_context_signal, with_context_scope, Context,
    ContextId, ContextScope, Provider,
};
pub use effect::Effect;
pub use incremental::{IncrementalComputed, IncrementalHandlers};
pub use runtime::ReactiveRuntime;
pub use signal::{Signal, Subscription, SubscriptionId};
pub use signal_vec::{SignalVec, VecDiff, VecSubscription};
pub use store::{
    create_store, store_registry, use_store, Store, StoreExt, StoreId, StoreRegistry,
    StoreSubscription,
};
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

/// Create a reactive vector with fine-grained change tracking
///
/// Unlike `signal(Vec<T>)`, this tracks individual insert/remove/update
/// operations for efficient incremental updates.
///
/// # Example
///
/// ```rust,ignore
/// let items = signal_vec(vec![1, 2, 3]);
///
/// // Each operation emits a granular change
/// items.push(4);      // VecDiff::Insert { index: 3, value: 4 }
/// items.remove(1);    // VecDiff::Remove { index: 1, value: 2 }
/// items.update(0, 10); // VecDiff::Update { index: 0, old: 1, new: 10 }
/// ```
pub fn signal_vec<T: Send + Sync + Clone + 'static>(values: Vec<T>) -> SignalVec<T> {
    SignalVec::new(values)
}
