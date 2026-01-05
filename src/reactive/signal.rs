//! Reactive signal implementation (thread-safe)
//!
//! Signals use `Arc<RwLock<T>>` internally, making them `Send + Sync`.
//! This allows async operations to update signals from background threads.

use super::tracker::{notify_dependents, track_read};
use super::SignalId;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

/// Type alias for thread-safe subscriber callbacks
type Subscribers = Arc<RwLock<Vec<Box<dyn Fn() + Send + Sync>>>>;

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
            subscribers: Arc::new(RwLock::new(Vec::new())),
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
    /// count.subscribe(|| println!("count changed!"));
    ///
    /// // Automatic: only tracks signals actually read
    /// Effect::new(move || {
    ///     println!("count is {}", count.get()); // auto-subscribes to count
    /// });
    /// ```
    pub fn subscribe(&self, callback: impl Fn() + Send + Sync + 'static) {
        let mut subs = self
            .subscribers
            .write()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        subs.push(Box::new(callback));
    }

    /// Manually trigger notification to subscribers
    ///
    /// Usually called automatically by `set()` and `update()`.
    pub fn notify_change(&self) {
        self.notify();
        notify_dependents(self.id);
    }

    /// Notify all subscribers of a change
    fn notify(&self) {
        let subs = self
            .subscribers
            .read()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        for subscriber in subs.iter() {
            subscriber();
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
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread;

    #[test]
    fn test_signal_get_set() {
        let count = Signal::new(0);
        assert_eq!(count.get(), 0);

        count.set(5);
        assert_eq!(count.get(), 5);
    }

    #[test]
    fn test_signal_update() {
        let count = Signal::new(10);
        count.update(|n| *n += 5);
        assert_eq!(count.get(), 15);

        count.update(|n| *n *= 2);
        assert_eq!(count.get(), 30);
    }

    #[test]
    fn test_signal_subscribe() {
        let count = Signal::new(0);
        let called = Arc::new(AtomicUsize::new(0));

        let called_clone = called.clone();
        count.subscribe(move || {
            called_clone.fetch_add(1, Ordering::SeqCst);
        });

        assert_eq!(called.load(Ordering::SeqCst), 0);

        count.set(1);
        assert_eq!(called.load(Ordering::SeqCst), 1);

        count.set(2);
        assert_eq!(called.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn test_signal_clone_shares_value() {
        let count = Signal::new(0);
        let count2 = count.clone();

        count.set(42);
        assert_eq!(count2.get(), 42);

        count2.set(100);
        assert_eq!(count.get(), 100);
    }

    #[test]
    fn test_signal_with_string() {
        let name = Signal::new(String::from("hello"));
        assert_eq!(name.get(), "hello");

        name.set(String::from("world"));
        assert_eq!(name.get(), "world");

        name.update(|s| s.push_str("!"));
        assert_eq!(name.get(), "world!");
    }

    #[test]
    fn test_signal_with_vec() {
        let items = Signal::new(vec![1, 2, 3]);
        assert_eq!(items.get(), vec![1, 2, 3]);

        items.update(|v| v.push(4));
        assert_eq!(items.get(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_signal_unique_ids() {
        let s1 = Signal::new(1);
        let s2 = Signal::new(2);
        assert_ne!(s1.id(), s2.id());
    }

    #[test]
    fn test_signal_read_zero_copy() {
        let items = Signal::new(vec![1, 2, 3]);

        // Zero-copy access via read()
        assert_eq!(items.read().len(), 3);
        assert_eq!(items.read()[0], 1);
    }

    #[test]
    fn test_signal_borrow_zero_copy() {
        let items = Signal::new(vec![1, 2, 3]);

        // Zero-copy access via borrow() (compatibility)
        assert_eq!(items.borrow().len(), 3);
        assert_eq!(items.borrow()[0], 1);
    }

    #[test]
    fn test_signal_with_zero_copy() {
        let items = Signal::new(vec![1, 2, 3]);

        // Zero-copy access via with()
        let len = items.with(|v| v.len());
        assert_eq!(len, 3);

        let sum: i32 = items.with(|v| v.iter().sum());
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_signal_thread_safety() {
        let count = Signal::new(0);
        let count_clone = count.clone();

        let handle = thread::spawn(move || {
            for _ in 0..100 {
                count_clone.update(|n| *n += 1);
            }
        });

        for _ in 0..100 {
            count.update(|n| *n += 1);
        }

        handle.join().unwrap();
        assert_eq!(count.get(), 200);
    }

    #[test]
    fn test_signal_cross_thread_subscribe() {
        let count = Signal::new(0);
        let notified = Arc::new(AtomicUsize::new(0));

        let notified_clone = notified.clone();
        count.subscribe(move || {
            notified_clone.fetch_add(1, Ordering::SeqCst);
        });

        let count_clone = count.clone();
        let handle = thread::spawn(move || {
            count_clone.set(42);
        });

        handle.join().unwrap();

        assert_eq!(count.get(), 42);
        assert_eq!(notified.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_signal_debug() {
        let sig = Signal::new(42);
        let debug_str = format!("{:?}", sig);
        assert!(debug_str.contains("Signal"));
        assert!(debug_str.contains("42"));
    }
}
