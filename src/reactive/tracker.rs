//! Automatic dependency tracking for reactive primitives
//!
//! This module provides the core dependency tracking mechanism that enables
//! automatic subscription between Signals and Effects/Computed values.
//!
//! # How It Works
//!
//! 1. When an Effect or Computed runs, it registers itself as the "current subscriber"
//! 2. When a Signal is read during that execution, it automatically registers
//!    the current subscriber as a dependent
//! 3. When the Signal changes, all registered dependents are notified
//!
//! # Example
//!
//! ```rust,ignore
//! let count = signal(0);
//!
//! // This effect automatically tracks `count` as a dependency
//! effect(|| {
//!     println!("Count: {}", count.get()); // Reading registers dependency
//! });
//!
//! count.set(1); // Automatically re-runs the effect
//! ```

use super::SignalId;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

// ─────────────────────────────────────────────────────────────────────────────
// Subscriber Types
// ─────────────────────────────────────────────────────────────────────────────

/// Unique identifier for a subscriber (effect or computed)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SubscriberId(u64);

impl SubscriberId {
    /// Create a new unique subscriber ID
    pub fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

impl Default for SubscriberId {
    fn default() -> Self {
        Self::new()
    }
}

/// A subscriber callback that can be notified when a signal changes
///
/// Uses Arc for thread-safe reference counting, but the callback itself
/// only needs to be callable (not Send/Sync) since it runs on the main thread.
pub type SubscriberCallback = Arc<dyn Fn() + Send + Sync>;

/// Information about a subscriber
#[derive(Clone)]
pub struct Subscriber {
    /// Unique identifier for this subscriber
    pub id: SubscriberId,
    /// Callback to invoke when dependencies change
    pub callback: SubscriberCallback,
}

impl Subscriber {
    /// Create a new subscriber
    pub fn new(callback: impl Fn() + Send + Sync + 'static) -> Self {
        Self {
            id: SubscriberId::new(),
            callback: Arc::new(callback),
        }
    }

    /// Invoke the subscriber callback
    pub fn notify(&self) {
        (self.callback)();
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Dependency Tracker
// ─────────────────────────────────────────────────────────────────────────────

/// Thread-local dependency tracker for automatic subscription management
pub struct DependencyTracker {
    /// Stack of currently executing subscribers (for nested effects)
    subscriber_stack: Vec<Subscriber>,
    /// Map from signal ID to its dependents
    dependencies: HashMap<SignalId, HashSet<SubscriberId>>,
    /// Map from subscriber ID to its callback (for notification)
    subscribers: HashMap<SubscriberId, SubscriberCallback>,
    /// Map from subscriber ID to signals it depends on (for cleanup)
    subscriber_deps: HashMap<SubscriberId, HashSet<SignalId>>,
}

impl DependencyTracker {
    /// Create a new dependency tracker
    pub fn new() -> Self {
        Self {
            subscriber_stack: Vec::new(),
            dependencies: HashMap::new(),
            subscribers: HashMap::new(),
            subscriber_deps: HashMap::new(),
        }
    }

    /// Begin tracking for a subscriber (push onto stack)
    pub fn start_tracking(&mut self, subscriber: Subscriber) {
        // Clear old dependencies for this subscriber (re-tracking)
        self.clear_subscriber_deps(subscriber.id);

        // Store callback for later notification
        self.subscribers.insert(subscriber.id, subscriber.callback.clone());

        // Push onto stack
        self.subscriber_stack.push(subscriber);
    }

    /// End tracking for current subscriber (pop from stack)
    pub fn stop_tracking(&mut self) -> Option<Subscriber> {
        self.subscriber_stack.pop()
    }

    /// Get the current subscriber being tracked (if any)
    pub fn current_subscriber(&self) -> Option<&Subscriber> {
        self.subscriber_stack.last()
    }

    /// Register a dependency: current subscriber depends on signal_id
    pub fn track_read(&mut self, signal_id: SignalId) {
        if let Some(subscriber) = self.subscriber_stack.last() {
            let sub_id = subscriber.id;

            // Add signal -> subscriber dependency
            self.dependencies
                .entry(signal_id)
                .or_default()
                .insert(sub_id);

            // Add subscriber -> signal reverse mapping (for cleanup)
            self.subscriber_deps
                .entry(sub_id)
                .or_default()
                .insert(signal_id);
        }
    }

    /// Notify all subscribers that depend on a signal
    pub fn notify_subscribers(&self, signal_id: SignalId) {
        if let Some(subscriber_ids) = self.dependencies.get(&signal_id) {
            // Collect callbacks to call (avoid borrow issues)
            let callbacks: Vec<_> = subscriber_ids
                .iter()
                .filter_map(|id| self.subscribers.get(id).cloned())
                .collect();

            for callback in callbacks {
                callback();
            }
        }
    }

    /// Clear all dependencies for a subscriber (called before re-tracking)
    fn clear_subscriber_deps(&mut self, subscriber_id: SubscriberId) {
        if let Some(signal_ids) = self.subscriber_deps.remove(&subscriber_id) {
            for signal_id in signal_ids {
                if let Some(deps) = self.dependencies.get_mut(&signal_id) {
                    deps.remove(&subscriber_id);
                }
            }
        }
    }

    /// Remove a subscriber completely (called when effect is disposed)
    pub fn dispose_subscriber(&mut self, subscriber_id: SubscriberId) {
        self.clear_subscriber_deps(subscriber_id);
        self.subscribers.remove(&subscriber_id);
    }

    /// Check if currently tracking (inside an effect/computed)
    pub fn is_tracking(&self) -> bool {
        !self.subscriber_stack.is_empty()
    }

    /// Get the number of dependents for a signal (for testing/debugging)
    pub fn dependent_count(&self, signal_id: SignalId) -> usize {
        self.dependencies.get(&signal_id).map_or(0, |s| s.len())
    }
}

impl Default for DependencyTracker {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Thread-Local Tracker Instance
// ─────────────────────────────────────────────────────────────────────────────

thread_local! {
    static TRACKER: RefCell<DependencyTracker> = RefCell::new(DependencyTracker::new());
}

/// Access the thread-local dependency tracker
pub fn with_tracker<R>(f: impl FnOnce(&mut DependencyTracker) -> R) -> R {
    TRACKER.with(|tracker| f(&mut tracker.borrow_mut()))
}

/// Start tracking dependencies for a subscriber
pub fn start_tracking(subscriber: Subscriber) {
    with_tracker(|t| t.start_tracking(subscriber));
}

/// Stop tracking and return the subscriber
pub fn stop_tracking() -> Option<Subscriber> {
    with_tracker(|t| t.stop_tracking())
}

/// Track a signal read (called from Signal::get/borrow/with)
pub fn track_read(signal_id: SignalId) {
    with_tracker(|t| t.track_read(signal_id));
}

/// Notify all dependents of a signal (called from Signal::set/update)
///
/// Note: Collects callbacks first to avoid borrow conflicts when
/// callbacks trigger more signal reads/writes.
pub fn notify_dependents(signal_id: SignalId) {
    // Collect callbacks while holding borrow, then call them after releasing
    let callbacks: Vec<SubscriberCallback> = with_tracker(|t| {
        t.dependencies
            .get(&signal_id)
            .map(|subscriber_ids| {
                subscriber_ids
                    .iter()
                    .filter_map(|id| t.subscribers.get(id).cloned())
                    .collect()
            })
            .unwrap_or_default()
    });

    // Now call callbacks without holding tracker borrow
    for callback in callbacks {
        callback();
    }
}

/// Dispose a subscriber (called when effect is dropped)
pub fn dispose_subscriber(subscriber_id: SubscriberId) {
    with_tracker(|t| t.dispose_subscriber(subscriber_id));
}

/// Check if currently tracking dependencies
pub fn is_tracking() -> bool {
    with_tracker(|t| t.is_tracking())
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_subscriber_id_unique() {
        let id1 = SubscriberId::new();
        let id2 = SubscriberId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_tracker_basic_tracking() {
        let mut tracker = DependencyTracker::new();
        let signal_id = SignalId::new();

        let called = Arc::new(AtomicUsize::new(0));
        let called_clone = called.clone();

        let subscriber = Subscriber::new(move || {
            called_clone.fetch_add(1, Ordering::SeqCst);
        });

        // Start tracking
        tracker.start_tracking(subscriber);

        // Track a read
        tracker.track_read(signal_id);

        // Stop tracking
        tracker.stop_tracking();

        // Verify dependency was registered
        assert_eq!(tracker.dependent_count(signal_id), 1);

        // Notify and check callback was invoked
        tracker.notify_subscribers(signal_id);
        assert_eq!(called.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_tracker_nested_tracking() {
        let mut tracker = DependencyTracker::new();
        let signal1 = SignalId::new();
        let signal2 = SignalId::new();

        let sub1 = Subscriber::new(|| {});
        let sub2 = Subscriber::new(|| {});

        // Outer subscriber tracks signal1
        tracker.start_tracking(sub1);
        tracker.track_read(signal1);

        // Inner subscriber tracks signal2
        tracker.start_tracking(sub2);
        tracker.track_read(signal2);
        tracker.stop_tracking();

        // Back to outer, track another signal
        tracker.track_read(signal1);
        tracker.stop_tracking();

        assert_eq!(tracker.dependent_count(signal1), 1);
        assert_eq!(tracker.dependent_count(signal2), 1);
    }

    #[test]
    fn test_tracker_retracking_clears_old_deps() {
        let mut tracker = DependencyTracker::new();
        let signal1 = SignalId::new();
        let signal2 = SignalId::new();

        let sub_id = SubscriberId::new();
        let subscriber = Subscriber {
            id: sub_id,
            callback: Arc::new(|| {}),
        };

        // First run: track signal1
        tracker.start_tracking(subscriber.clone());
        tracker.track_read(signal1);
        tracker.stop_tracking();

        assert_eq!(tracker.dependent_count(signal1), 1);
        assert_eq!(tracker.dependent_count(signal2), 0);

        // Second run (re-tracking): track signal2 only
        tracker.start_tracking(subscriber);
        tracker.track_read(signal2);
        tracker.stop_tracking();

        // Old dependency on signal1 should be cleared
        assert_eq!(tracker.dependent_count(signal1), 0);
        assert_eq!(tracker.dependent_count(signal2), 1);
    }

    #[test]
    fn test_tracker_dispose_subscriber() {
        let mut tracker = DependencyTracker::new();
        let signal_id = SignalId::new();

        let sub_id = SubscriberId::new();
        let subscriber = Subscriber {
            id: sub_id,
            callback: Arc::new(|| {}),
        };

        tracker.start_tracking(subscriber);
        tracker.track_read(signal_id);
        tracker.stop_tracking();

        assert_eq!(tracker.dependent_count(signal_id), 1);

        // Dispose the subscriber
        tracker.dispose_subscriber(sub_id);

        assert_eq!(tracker.dependent_count(signal_id), 0);
    }
}
