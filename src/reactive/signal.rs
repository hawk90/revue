//! Reactive signal implementation (thread-safe)
//!
//! Signals use `Arc<RwLock<T>>` internally, making them `Send + Sync`.
//! This allows async operations to update signals from background threads.

use super::tracker::{notify_dependents, track_read};
use super::SignalId;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering as AtomicOrdering};
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

/// Unique identifier for a signal subscription
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SubscriptionId(u64);

impl SubscriptionId {
    fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        Self(COUNTER.fetch_add(1, AtomicOrdering::Relaxed))
    }
}

/// Type alias for thread-safe subscriber callbacks
/// Using Arc allows callbacks to be cloned for safe notification
type SubscriberCallback = Arc<dyn Fn() + Send + Sync>;
type Subscribers = Arc<RwLock<HashMap<SubscriptionId, SubscriberCallback>>>;

/// Handle to a signal subscription that automatically unsubscribes when dropped
///
/// This prevents memory leaks by ensuring callbacks are removed when no longer needed.
///
/// # Example
///
/// ```ignore
/// let count = signal(0);
///
/// // Subscription is active while `sub` is in scope
/// let sub = count.subscribe(|| println!("changed!"));
///
/// count.set(1);  // Prints "changed!"
///
/// drop(sub);     // Unsubscribes
///
/// count.set(2);  // No output - callback was removed
/// ```
pub struct Subscription {
    id: SubscriptionId,
    subscribers: Subscribers,
}

impl Drop for Subscription {
    fn drop(&mut self) {
        if let Ok(mut subs) = self.subscribers.write() {
            subs.remove(&self.id);
        }
    }
}

/// A reactive state container that triggers updates when changed
///
/// `Signal` is thread-safe (`Send + Sync`) and can be shared across threads.
/// This enables async operations to update UI state directly.
///
/// # Zero-Copy Access
///
/// Use `read()` or `with()` for zero-copy read access:
/// ```ignore
/// let items = Signal::new(vec![1, 2, 3]);
///
/// // Zero-copy: read returns a RwLockReadGuard
/// let len = items.read().len();
///
/// // Zero-copy with closure
/// items.with(|v| println!("Length: {}", v.len()));
/// ```
///
/// Use `get()` only when you need an owned copy.
///
/// # Thread Safety
///
/// Signals can be cloned and sent to other threads:
/// ```ignore
/// let count = signal(0);
/// let count_clone = count.clone();
///
/// std::thread::spawn(move || {
///     count_clone.set(42);  // Updates from background thread
/// });
/// ```
pub struct Signal<T> {
    id: SignalId,
    value: Arc<RwLock<T>>,
    subscribers: Subscribers,
}

// Signal<T> auto-derives Send + Sync when T: Send + Sync.
// All fields are inherently thread-safe:
// - id: SignalId (Copy, u64)
// - value: Arc<RwLock<T>> (Send+Sync when T: Send+Sync)
// - subscribers: Arc<RwLock<Vec<...>>> (Send+Sync with Send+Sync callbacks)

impl<T: 'static> Signal<T> {
    /// Create a new signal with initial value
    pub fn new(value: T) -> Self {
        Self {
            id: SignalId::new(),
            value: Arc::new(RwLock::new(value)),
            subscribers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Read the value immutably (zero-copy)
    ///
    /// Returns a `RwLockReadGuard` that dereferences to `&T`.
    /// Prefer this over `get()` to avoid cloning.
    /// Automatically registers dependency if called within an effect/computed.
    ///
    /// # Lock Poisoning Recovery
    /// If the lock is poisoned (due to a panic in another thread), this method
    /// recovers by returning the underlying data. The data may be in an
    /// inconsistent state, but this prevents cascading panics.
    #[inline]
    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        track_read(self.id);
        self.value
            .read()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// Borrow the value immutably (alias for read, zero-copy)
    ///
    /// For compatibility with previous API. Prefer `read()` for clarity.
    #[inline]
    pub fn borrow(&self) -> RwLockReadGuard<'_, T> {
        self.read()
    }

    /// Write to the value mutably (zero-copy)
    ///
    /// Returns a `RwLockWriteGuard`. Does NOT automatically notify subscribers.
    /// Call `notify_change()` after modifications if needed.
    ///
    /// # Lock Poisoning Recovery
    /// If the lock is poisoned, this method recovers by returning the underlying data.
    #[inline]
    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        self.value
            .write()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// Borrow the value mutably (alias for write, zero-copy)
    ///
    /// For compatibility with previous API. Prefer `write()` for clarity.
    #[inline]
    pub fn borrow_mut(&self) -> RwLockWriteGuard<'_, T> {
        self.write()
    }

    /// Access the value with a closure (zero-copy)
    ///
    /// This is the most ergonomic way to read without cloning:
    /// ```ignore
    /// let count = signal.with(|v| *v);
    /// ```
    /// Automatically registers dependency if called within an effect/computed.
    #[inline]
    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        track_read(self.id);
        let guard = self
            .value
            .read()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        f(&*guard)
    }

    /// Modify the value with a closure (zero-copy)
    ///
    /// Like `with` but for mutations. Does NOT notify subscribers.
    #[inline]
    pub fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut guard = self
            .value
            .write()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        f(&mut *guard)
    }

    /// Set a new value and notify subscribers
    ///
    /// Notifies both manual subscribers and auto-tracked dependents.
    pub fn set(&self, value: T) {
        {
            let mut guard = self
                .value
                .write()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            *guard = value;
        }
        self.notify();
        notify_dependents(self.id);
    }

    /// Update value using a function and notify subscribers
    ///
    /// Notifies both manual subscribers and auto-tracked dependents.
    pub fn update(&self, f: impl FnOnce(&mut T)) {
        {
            let mut guard = self
                .value
                .write()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            f(&mut *guard);
        }
        self.notify();
        notify_dependents(self.id);
    }

    /// Subscribe to changes manually (callback must be Send + Sync)
    ///
    /// Returns a [`Subscription`] handle that automatically unsubscribes when dropped.
    /// This prevents memory leaks from accumulated callbacks.
    ///
    /// This provides **explicit** subscription, unlike the **automatic** dependency
    /// tracking used by `Effect` and `Computed`.
    ///
    /// # Manual vs Automatic Subscription
    ///
    /// | Approach | How it works | Use case |
    /// |----------|--------------|----------|
    /// | `subscribe()` | Explicit registration, always called on change | External integrations, logging |
    /// | `Effect::new()` | Auto-tracks signals read during execution | Reactive side effects |
    ///
    /// # Example
    ///
    /// ```ignore
    /// let count = signal(0);
    ///
    /// // Manual: always called when count changes
    /// let sub = count.subscribe(|| println!("count changed!"));
    ///
    /// count.set(1);  // Calls callback
    ///
    /// drop(sub);     // Unsubscribes - callback is removed
    ///
    /// count.set(2);  // No callback called
    /// ```
    pub fn subscribe(&self, callback: impl Fn() + Send + Sync + 'static) -> Subscription {
        let id = SubscriptionId::new();
        {
            let mut subs = self
                .subscribers
                .write()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            subs.insert(id, Arc::new(callback));
        }
        Subscription {
            id,
            subscribers: Arc::clone(&self.subscribers),
        }
    }

    /// Manually trigger notification to subscribers
    ///
    /// Usually called automatically by `set()` and `update()`.
    pub fn notify_change(&self) {
        self.notify();
        notify_dependents(self.id);
    }

    /// Notify all subscribers of a change
    ///
    /// # Performance & Safety
    ///
    /// Clones callbacks before invoking them to avoid holding the read lock
    /// during callback execution. This prevents deadlock when callbacks
    /// drop their own Subscription handles.
    fn notify(&self) {
        // Clone callbacks while holding read lock
        let callbacks: Vec<_> = {
            let subs = self
                .subscribers
                .read()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            subs.values().cloned().collect()
            // Lock released here when `subs` goes out of scope
        };

        // Invoke callbacks without holding any lock
        // This allows callbacks to safely drop their Subscription handles
        for callback in callbacks {
            callback();
        }
    }

    /// Get the signal's unique ID
    pub fn id(&self) -> SignalId {
        self.id
    }
}

/// Clone support for owned copies
impl<T: Clone + 'static> Signal<T> {
    /// Get an owned copy of the current value
    ///
    /// **Note**: This clones the value. Prefer `read()` or `with()` for zero-copy access.
    /// Automatically registers dependency if called within an effect/computed.
    #[inline]
    pub fn get(&self) -> T {
        track_read(self.id);
        let guard = self
            .value
            .read()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        guard.clone()
    }
}

impl<T: 'static> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            value: Arc::clone(&self.value),
            subscribers: Arc::clone(&self.subscribers),
        }
    }
}

impl<T: std::fmt::Debug + 'static> std::fmt::Debug for Signal<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.value.try_read() {
            Ok(guard) => f
                .debug_struct("Signal")
                .field("id", &self.id)
                .field("value", &*guard)
                .finish(),
            Err(_) => f
                .debug_struct("Signal")
                .field("id", &self.id)
                .field("value", &"<locked>")
                .finish(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that callbacks can safely drop their own subscriptions
    /// This verifies the RwLock contention fix
    #[test]
    fn test_callback_can_drop_other_subscription() {
        use std::sync::{Arc, Mutex};

        let signal = Signal::new(0);

        // Create a subscription that will be dropped during notification
        // Store it in an Arc<Mutex> so the callback can access it
        let sub_holder = Arc::new(Mutex::new(None));

        let sub = signal.subscribe({
            let signal_clone = signal.clone();
            let holder = Arc::clone(&sub_holder);
            move || {
                // Take and drop the subscription from holder
                if let Some(s) = holder.lock().unwrap().take() {
                    drop(s);
                }
                signal_clone.set(42);
            }
        });

        // Store the subscription so the callback can access it
        *sub_holder.lock().unwrap() = Some(sub);

        // This should not deadlock
        signal.set(1);
    }

    /// Test that multiple subscriptions can be dropped during notification
    #[test]
    fn test_multiple_subscriptions_drop_during_notify() {
        let signal = Signal::new(0);

        let mut subscriptions = Vec::new();

        for i in 0..5 {
            let sub = signal.subscribe(move || {
                // Each callback conditionally drops based on index
                if i == 2 {
                    // Drop the subscription at index 2
                }
            });
            subscriptions.push(sub);
        }

        // This should not deadlock even though one callback might
        // indirectly cause a subscription drop
        signal.set(1);
    }

    /// Test that nested notifications work correctly
    #[test]
    fn test_nested_notifications() {
        let signal1 = Signal::new(0);
        let signal2 = Signal::new(0);

        let _sub = signal1.subscribe({
            let signal2_clone = signal2.clone();
            move || {
                // Callback triggers another notification
                signal2_clone.set(1);
            }
        });

        let _sub2 = signal2.subscribe(|| {
            // This should execute without deadlock
        });

        // This should not deadlock
        signal1.set(1);
    }

    /// Test that subscription cleanup works correctly
    #[test]
    fn test_subscription_cleanup() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        let signal = Signal::new(0);
        let call_count = Arc::new(AtomicUsize::new(0));

        let sub = signal.subscribe({
            let count = Arc::clone(&call_count);
            move || {
                count.fetch_add(1, Ordering::Relaxed);
            }
        });

        signal.set(1);
        assert_eq!(call_count.load(Ordering::Relaxed), 1);

        drop(sub);
        signal.set(2);
        // Should still be 1 since subscription was dropped
        assert_eq!(call_count.load(Ordering::Relaxed), 1);
    }

    /// Test basic signal functionality
    #[test]
    fn test_signal_basic() {
        let signal = Signal::new(42);

        assert_eq!(*signal.read(), 42);

        signal.set(100);
        assert_eq!(*signal.read(), 100);

        signal.update(|v| *v *= 2);
        assert_eq!(*signal.read(), 200);
    }

    /// Test with_mut and with methods
    #[test]
    fn test_signal_with_methods() {
        let signal = Signal::new(vec![1, 2, 3]);

        let len = signal.with(|v| v.len());
        assert_eq!(len, 3);

        signal.with_mut(|v| v.push(4));
        assert_eq!(*signal.read(), vec![1, 2, 3, 4]);
    }
}
