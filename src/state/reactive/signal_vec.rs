//! Fine-grained reactive vector implementation
//!
//! `SignalVec<T>` tracks individual changes (insert/remove/update) to enable
//! incremental updates in derived computations, avoiding full recomputation.

#![allow(clippy::type_complexity)]
use super::signal::{Signal, Subscription};
use super::tracker::notify_dependents;
use super::SignalId;
use std::sync::Arc;
use std::sync::Mutex as StdMutex;

/// Granular change to a vector element
#[derive(Debug, Clone, PartialEq)]
pub enum VecDiff<T> {
    /// Insert element at index
    Insert {
        /// Index where element is inserted
        index: usize,
        /// Value being inserted
        value: T,
    },
    /// Update element at index
    Update {
        /// Index of element being updated
        index: usize,
        /// Old value before update
        old_value: T,
        /// New value after update
        new_value: T,
    },
    /// Remove element at index
    Remove {
        /// Index where element is removed
        index: usize,
        /// Value being removed
        value: T,
    },
    /// Move element from old_index to new_index
    Move {
        /// Original index before move
        old_index: usize,
        /// New index after move
        new_index: usize,
        /// Value being moved
        value: T,
    },
    /// Replace entire vector (for batch changes)
    Replace {
        /// Old values before replacement
        old_values: Vec<T>,
        /// New values after replacement
        new_values: Vec<T>,
    },
}

impl<T> VecDiff<T> {
    /// Get the index affected by this change
    pub fn index(&self) -> Option<usize> {
        match self {
            Self::Insert { index, .. } => Some(*index),
            Self::Update { index, .. } => Some(*index),
            Self::Remove { index, .. } => Some(*index),
            Self::Move { old_index, .. } => Some(*old_index),
            Self::Replace { .. } => None,
        }
    }

    /// Get the value affected by this change
    pub fn value(&self) -> Option<&T> {
        match self {
            Self::Insert { value, .. } => Some(value),
            Self::Update { new_value, .. } => Some(new_value),
            Self::Remove { value, .. } => Some(value),
            Self::Move { value, .. } => Some(value),
            Self::Replace { .. } => None,
        }
    }
}

/// A reactive vector that emits granular changes
///
/// Unlike `Signal<Vec<T>>` which treats any modification as "entire vector changed",
/// `SignalVec<T>` tracks individual operations (insert/remove/update) for efficient
/// incremental updates.
///
/// # Example
///
/// ```ignore
/// let items = signal_vec(vec![1, 2, 3]);
///
/// // Subscribe to granular changes
/// let sub = items.subscribe_diff(|diff| match diff {
///     VecDiff::Insert { index, value } => {
///         println!("Inserted {value} at {index}");
///     }
///     VecDiff::Remove { index, value } => {
///         println!("Removed {value} from {index}");
///     }
///     _ => {}
/// });
///
/// items.push(4);  // Prints: "Inserted 4 at 3"
/// items.remove(1); // Prints: "Removed 2 from 1"
/// ```
pub struct SignalVec<T> {
    /// Underlying signal storing the vector
    inner: Signal<Vec<T>>,
    /// Unique ID for this signal vector
    id: SignalId,
    /// Diff subscribers (callbacks for granular changes)
    diff_subscribers: Arc<StdMutex<Vec<Arc<dyn Fn(VecDiff<T>) + Send + Sync>>>>,
}

impl<T: Send + Sync + Clone + 'static> SignalVec<T> {
    /// Create a new signal vector with initial values
    pub fn new(values: Vec<T>) -> Self {
        let inner = Signal::new(values);
        let id = SignalId::new();
        Self {
            inner,
            id,
            diff_subscribers: Arc::new(StdMutex::new(Vec::new())),
        }
    }

    /// Get the current vector as a slice (zero-copy)
    pub fn read(&self) -> std::sync::RwLockReadGuard<'_, Vec<T>> {
        self.inner.read()
    }

    /// Get owned copy of the vector
    pub fn get(&self) -> Vec<T> {
        self.inner.get()
    }

    /// Get length of the vector
    pub fn len(&self) -> usize {
        self.inner.with(|v| v.len())
    }

    /// Check if vector is empty
    pub fn is_empty(&self) -> bool {
        self.inner.with(|v| v.is_empty())
    }

    /// Get element at index
    pub fn get_index(&self, index: usize) -> Option<T> {
        self.inner.with(|v| v.get(index).cloned())
    }

    /// Insert value at index
    pub fn insert(&self, index: usize, value: T) {
        self.inner.update(|v| {
            v.insert(index, value.clone());
        });
        self.notify_diff(VecDiff::Insert { index, value });
    }

    /// Push value to the end
    pub fn push(&self, value: T) {
        let index = self.len();
        self.insert(index, value);
    }

    /// Remove element at index, returning the old value
    pub fn remove(&self, index: usize) -> Option<T> {
        let old_value = self.get_index(index)?;
        self.inner.update(|v| {
            v.remove(index);
        });
        self.notify_diff(VecDiff::Remove {
            index,
            value: old_value.clone(),
        });
        Some(old_value)
    }

    /// Update element at index with a new value
    pub fn update(&self, index: usize, new_value: T) {
        let old_value = self.get_index(index);
        if let Some(old) = old_value {
            self.inner.update(|v| {
                if let Some(elem) = v.get_mut(index) {
                    *elem = new_value.clone();
                }
            });
            self.notify_diff(VecDiff::Update {
                index,
                old_value: old,
                new_value,
            });
        }
    }

    /// Modify element at index using a function
    pub fn modify(&self, index: usize, f: impl FnOnce(&mut T)) {
        let old_value = self.get_index(index);
        self.inner.update(|v| {
            if let Some(elem) = v.get_mut(index) {
                f(elem);
            }
        });
        if let Some(old) = old_value {
            let new_value = self.get_index(index);
            if let Some(new) = new_value {
                self.notify_diff(VecDiff::Update {
                    index,
                    old_value: old,
                    new_value: new,
                });
            }
        }
    }

    /// Pop element from the end
    pub fn pop(&self) -> Option<T> {
        let index = self.len().checked_sub(1)?;
        self.remove(index)
    }

    /// Clear all elements
    pub fn clear(&self) {
        let old_values = self.get();
        self.inner.update(|v| v.clear());
        self.notify_diff(VecDiff::Replace {
            old_values,
            new_values: vec![],
        });
    }

    /// Replace entire vector
    pub fn replace(&self, new_values: Vec<T>) {
        let old_values = self.get();
        self.inner.set(new_values.clone());
        self.notify_diff(VecDiff::Replace {
            old_values,
            new_values,
        });
    }

    /// Subscribe to granular changes
    ///
    /// Returns a subscription handle that automatically unsubscribs when dropped.
    pub fn subscribe_diff(
        &self,
        callback: impl Fn(VecDiff<T>) + Send + Sync + 'static,
    ) -> VecSubscription<T> {
        let callback = Arc::new(callback);

        // Register the callback
        if let Ok(mut subs) = self.diff_subscribers.lock() {
            subs.push(callback.clone());
        }

        // Create a subscription to the inner signal (to keep it alive)
        let inner_sub = self.inner.subscribe(|| {});

        VecSubscription {
            _inner: inner_sub,
            _callback: callback,
            _diff_subscribers: self.diff_subscribers.clone(),
        }
    }

    /// Notify diff subscribers
    fn notify_diff(&self, diff: VecDiff<T>) {
        // Emit to all diff subscribers
        if let Ok(subs) = self.diff_subscribers.lock() {
            for callback in subs.iter() {
                callback(diff.clone());
            }
        }
        // Also notify regular signal dependents
        notify_dependents(self.id);
    }

    /// Get the inner signal for direct access
    pub fn inner(&self) -> &Signal<Vec<T>> {
        &self.inner
    }

    /// Get the signal vector's ID
    pub fn id(&self) -> SignalId {
        self.id
    }
}

impl<T: Send + Sync + Clone + 'static> Clone for SignalVec<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            id: self.id,
            diff_subscribers: Arc::clone(&self.diff_subscribers),
        }
    }
}

/// Subscription handle for SignalVec
pub struct VecSubscription<T> {
    _inner: Subscription,
    _callback: Arc<dyn Fn(VecDiff<T>) + Send + Sync>,
    _diff_subscribers: Arc<StdMutex<Vec<Arc<dyn Fn(VecDiff<T>) + Send + Sync>>>>,
}

impl<T> Drop for VecSubscription<T> {
    fn drop(&mut self) {
        // Remove the callback from subscribers when dropped
        if let Ok(mut subs) = self._diff_subscribers.lock() {
            subs.retain(|cb| !Arc::ptr_eq(cb, &self._callback));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_vec_new() {
        let vec = SignalVec::new(vec![1, 2, 3]);
        assert_eq!(vec.len(), 3);
        assert!(!vec.is_empty());
    }

    #[test]
    fn test_signal_vec_push() {
        let vec = SignalVec::new(vec![1, 2, 3]);
        vec.push(4);
        assert_eq!(vec.len(), 4);
        assert_eq!(vec.get(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_signal_vec_insert() {
        let vec = SignalVec::new(vec![1, 2, 4]);
        vec.insert(2, 3);
        assert_eq!(vec.get(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_signal_vec_remove() {
        let vec = SignalVec::new(vec![1, 2, 3, 4]);
        let removed = vec.remove(2);
        assert_eq!(removed, Some(3));
        assert_eq!(vec.get(), vec![1, 2, 4]);
    }

    #[test]
    fn test_signal_vec_update() {
        let vec = SignalVec::new(vec![1, 2, 3]);
        vec.update(1, 20);
        assert_eq!(vec.get(), vec![1, 20, 3]);
    }

    #[test]
    fn test_signal_vec_modify() {
        let vec = SignalVec::new(vec![1, 2, 3]);
        vec.modify(1, |v| *v *= 10);
        assert_eq!(vec.get(), vec![1, 20, 3]);
    }

    #[test]
    fn test_signal_vec_pop() {
        let vec = SignalVec::new(vec![1, 2, 3]);
        let popped = vec.pop();
        assert_eq!(popped, Some(3));
        assert_eq!(vec.get(), vec![1, 2]);
    }

    #[test]
    fn test_signal_vec_clear() {
        let vec = SignalVec::new(vec![1, 2, 3]);
        vec.clear();
        assert_eq!(vec.len(), 0);
        assert!(vec.is_empty());
    }

    #[test]
    fn test_signal_vec_replace() {
        let vec = SignalVec::new(vec![1, 2, 3]);
        vec.replace(vec![4, 5, 6]);
        assert_eq!(vec.get(), vec![4, 5, 6]);
    }

    #[test]
    fn test_signal_vec_get_index() {
        let vec = SignalVec::new(vec![1, 2, 3]);
        assert_eq!(vec.get_index(0), Some(1));
        assert_eq!(vec.get_index(1), Some(2));
        assert_eq!(vec.get_index(10), None);
    }

    #[test]
    fn test_vec_diff_insert() {
        let diff = VecDiff::Insert {
            index: 0,
            value: 42,
        };
        assert_eq!(diff.index(), Some(0));
        assert_eq!(diff.value(), Some(&42));
    }

    #[test]
    fn test_vec_diff_remove() {
        let diff = VecDiff::Remove {
            index: 1,
            value: "test",
        };
        assert_eq!(diff.index(), Some(1));
        assert_eq!(diff.value(), Some(&"test"));
    }

    #[test]
    fn test_vec_diff_update() {
        let diff = VecDiff::Update {
            index: 2,
            old_value: 1,
            new_value: 10,
        };
        assert_eq!(diff.index(), Some(2));
        assert_eq!(diff.value(), Some(&10));
    }

    #[test]
    fn test_vec_diff_replace_no_index() {
        let diff = VecDiff::Replace {
            old_values: vec![1, 2],
            new_values: vec![3, 4],
        };
        assert_eq!(diff.index(), None);
        assert_eq!(diff.value(), None);
    }

    #[test]
    fn test_signal_vec_clone() {
        let vec1 = SignalVec::new(vec![1, 2, 3]);
        let vec2 = vec1.clone();
        vec1.push(4);
        assert_eq!(vec2.get(), vec![1, 2, 3, 4]);
    }
}
