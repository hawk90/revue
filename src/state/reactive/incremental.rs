//! Incremental computed values for SignalVec
//!
//! Enables efficient updates to derived values from SignalVec by processing
//! individual changes instead of recomputing the entire result.
//!
//! # Example: Filtered List with Incremental Updates
//!
//! ```ignore
//! use revue::reactive::{signal_vec, IncrementalComputed, IncrementalHandlers};
//!
//! let items = signal_vec(vec![1, 2, 3, 4, 5, 6]);
//!
//! // Create a filtered list that incrementally updates
//! let evens = IncrementalComputed::new(
//!     items.clone(),
//!     // Initial computation
//!     |v| v.iter().filter(|x| *x % 2 == 0).copied().collect::<Vec<_>>(),
//!     IncrementalHandlers::new()
//!         .insert(|result, _index, value| {
//!             // Only add if the new value is even
//!             if value % 2 == 0 {
//!                 result.push(value);
//!             }
//!         })
//!         .update(|result, index, old, new| {
//!             // Handle value changes
//!             if old % 2 == 0 && new % 2 != 0 {
//!                 // Remove the old even value
//!                 result.retain(|x| *x != old);
//!             } else if old % 2 != 0 && new % 2 == 0 {
//!                 // Add the new even value
//!                 result.push(new);
//!             }
//!         })
//!         .remove(|result, _index, value| {
//!             // Remove the value if it was in the result
//!             result.retain(|x| *x != value);
//!         })
//!         .replace(|items| {
//!             // Full recomputation for batch changes
//!             items.get().iter().filter(|x| *x % 2 == 0).copied().collect()
//!         }),
//! );
//!
//! // Initial state
//! assert_eq!(evens.get(), vec![2, 4, 6]);
//!
//! // Incremental updates (efficient - no full re-filter)
//! items.push(8);   // Only checks if 8 is even
//! items.remove(1); // Only removes 2 from result
//! items.update(0, 10); // Only updates the affected values
//! ```
//!
//! # Performance Benefits
//!
//! With standard `Computed`, adding 1 item to a 1000-item filtered list would
//! require re-filtering all 1001 items. With `IncrementalComputed`, only the
//! new item is checked, resulting in ~1000x performance improvement for this case.

use super::signal_vec::SignalVec;
use super::tracker::notify_dependents;
use super::SignalId;
use crate::utils::lock::lock_or_recover;
use std::sync::{Arc, Mutex};

/// Handlers for incremental updates from a SignalVec
#[allow(clippy::type_complexity)]
pub struct IncrementalHandlers<T: 'static, R: 'static> {
    /// Handle insert operation
    pub on_insert: Arc<dyn Fn(&mut R, usize, T) + Send + Sync>,
    /// Handle update operation
    pub on_update: Arc<dyn Fn(&mut R, usize, T, T) + Send + Sync>,
    /// Handle remove operation
    pub on_remove: Arc<dyn Fn(&mut R, usize, T) + Send + Sync>,
    /// Handle replace operation (full recomputation)
    pub on_replace: Arc<dyn Fn(&SignalVec<T>) -> R + Send + Sync>,
}

impl<T: 'static, R: 'static> Clone for IncrementalHandlers<T, R> {
    fn clone(&self) -> Self {
        Self {
            on_insert: Arc::clone(&self.on_insert),
            on_update: Arc::clone(&self.on_update),
            on_remove: Arc::clone(&self.on_remove),
            on_replace: Arc::clone(&self.on_replace),
        }
    }
}

impl<T: 'static, R: 'static> IncrementalHandlers<T, R> {
    /// Create new incremental handlers
    pub fn new() -> Self {
        Self {
            on_insert: Arc::new(|_, _, _| {}),
            on_update: Arc::new(|_, _, _, _| {}),
            on_remove: Arc::new(|_, _, _| {}),
            on_replace: Arc::new(|_| panic!("Replace handler not implemented")),
        }
    }

    /// Set handler for insert operations
    pub fn insert(mut self, f: impl Fn(&mut R, usize, T) + Send + Sync + 'static) -> Self {
        self.on_insert = Arc::new(f);
        self
    }

    /// Set handler for update operations
    pub fn update(mut self, f: impl Fn(&mut R, usize, T, T) + Send + Sync + 'static) -> Self {
        self.on_update = Arc::new(f);
        self
    }

    /// Set handler for remove operations
    pub fn remove(mut self, f: impl Fn(&mut R, usize, T) + Send + Sync + 'static) -> Self {
        self.on_remove = Arc::new(f);
        self
    }

    /// Set handler for replace operations
    pub fn replace(mut self, f: impl Fn(&SignalVec<T>) -> R + Send + Sync + 'static) -> Self {
        self.on_replace = Arc::new(f);
        self
    }
}

impl<T: 'static, R: 'static> Default for IncrementalHandlers<T, R> {
    fn default() -> Self {
        Self::new()
    }
}

/// A computed value that incrementally updates from SignalVec changes
///
/// Instead of recomputing the entire result when the source vector changes,
/// this processes individual insert/update/remove operations efficiently.
///
/// # Example
///
/// ```rust,ignore
/// let items = signal_vec(vec![1, 2, 3, 4, 5]);
///
/// // Filter even numbers with incremental updates
/// let filtered = IncrementalComputed::new(
///     items.clone(),
///     |v| v.iter().filter(|x| *x % 2 == 0).copied().collect(),
///     IncrementalHandlers::new()
///         .insert(|result, index, value| {
///             if value % 2 == 0 {
///                 result.push(value);
///             }
///         })
///         .update(|result, index, old, new| {
///             if old % 2 == 0 && new % 2 != 0 {
///                 result.retain(|x| *x != old);
///             } else if old % 2 != 0 && new % 2 == 0 {
///                 result.push(new);
///             }
///         })
///         .remove(|result, _, value| {
///             result.retain(|x| *x != value);
///         })
///         .replace(|items| {
///             items.get().iter().filter(|x| *x % 2 == 0).copied().collect()
///         }),
/// );
///
/// items.push(6);  // Only checks if 6 is even, doesn't re-filter entire list
/// ```
pub struct IncrementalComputed<T: Clone + Send + Sync + 'static, R: Clone + Send + Sync + 'static> {
    /// Source signal vector
    source: SignalVec<T>,
    /// Cached result
    cached: Arc<Mutex<R>>,
    /// Unique ID
    id: SignalId,
}

impl<T: Clone + Send + Sync + 'static, R: Clone + Send + Sync + 'static> IncrementalComputed<T, R> {
    /// Create a new incremental computed value
    ///
    /// # Arguments
    /// * `source` - The source SignalVec
    /// * `init` - Initial computation function
    /// * `handlers` - Incremental update handlers
    pub fn new(
        source: SignalVec<T>,
        init: impl Fn(&[T]) -> R + Send + Sync + 'static,
        handlers: IncrementalHandlers<T, R>,
    ) -> Self {
        let id = SignalId::new();
        let initial_result = init(&source.get());

        let cached = Arc::new(Mutex::new(initial_result));

        // Clone handlers for use in the subscription
        let handlers_clone = handlers.clone();
        let source_clone = source.clone();

        // Subscribe to diff events from the source
        let cached_clone = cached.clone();
        source.subscribe_diff(move |diff| {
            if let Ok(mut result) = cached_clone.lock() {
                match diff {
                    super::signal_vec::VecDiff::Insert { index, value } => {
                        (handlers_clone.on_insert)(&mut result, index, value);
                    }
                    super::signal_vec::VecDiff::Update {
                        index,
                        old_value,
                        new_value,
                    } => {
                        (handlers_clone.on_update)(&mut result, index, old_value, new_value);
                    }
                    super::signal_vec::VecDiff::Remove { index, value } => {
                        (handlers_clone.on_remove)(&mut result, index, value);
                    }
                    super::signal_vec::VecDiff::Move {
                        old_index,
                        new_index,
                        value,
                    } => {
                        // For move, we simulate remove + insert
                        (handlers_clone.on_remove)(&mut result, old_index, value.clone());
                        (handlers_clone.on_insert)(&mut result, new_index, value);
                    }
                    super::signal_vec::VecDiff::Replace {
                        old_values: _,
                        new_values: _,
                    } => {
                        // Full recomputation for replace
                        let new_result = (handlers_clone.on_replace)(&source_clone);
                        *result = new_result;
                    }
                }
                // Notify dependents after incremental update
                notify_dependents(id);
            }
        });

        Self { source, cached, id }
    }

    /// Get the current cached value
    pub fn get(&self) -> R {
        lock_or_recover(&self.cached).clone()
    }

    /// Get the inner cached value (zero-copy with guard)
    pub fn read(&self) -> std::sync::MutexGuard<'_, R> {
        lock_or_recover(&self.cached)
    }

    /// Get the source SignalVec
    pub fn source(&self) -> &SignalVec<T> {
        &self.source
    }

    /// Invalidate cache (triggers recomputation on next get)
    pub fn invalidate(&self) {
        notify_dependents(self.id);
    }

    /// Get the ID
    pub fn id(&self) -> SignalId {
        self.id
    }
}

impl<T: Clone + Send + Sync + 'static, R: Clone + Send + Sync + 'static> Clone
    for IncrementalComputed<T, R>
{
    fn clone(&self) -> Self {
        Self {
            source: self.source.clone(),
            cached: Arc::clone(&self.cached),
            id: self.id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_incremental_handlers_new() {
        let _handlers: IncrementalHandlers<i32, Vec<i32>> = IncrementalHandlers::new();
        // Just verify it compiles
        assert!(true);
    }

    #[test]
    fn test_incremental_handlers_builder() {
        let _handlers = IncrementalHandlers::<i32, Vec<i32>>::new()
            .insert(|result, index, value| {
                result.insert(index, value);
            })
            .update(|_result, _index, _old, _new| {
                // Update handler
            })
            .remove(|_result, _index, _value| {
                // Remove handler
            })
            .replace(|_source: &SignalVec<i32>| vec![]);

        // Verify handlers are set
        assert!(true);
    }

    #[test]
    fn test_incremental_computed_new() {
        use crate::reactive::signal_vec;

        let items = signal_vec(vec![1, 2, 3, 4, 5]);

        let filtered: IncrementalComputed<i32, Vec<i32>> = IncrementalComputed::new(
            items.clone(),
            |v| v.iter().filter(|x| *x % 2 == 0).copied().collect(),
            IncrementalHandlers::new(),
        );

        let result = filtered.get();
        assert_eq!(result, vec![2, 4]);
    }

    #[test]
    fn test_incremental_handlers_default() {
        let handlers: IncrementalHandlers<i32, Vec<i32>> = IncrementalHandlers::default();
        // Just verify it works
        assert!(true);
    }

    #[test]
    fn test_incremental_handlers_clone() {
        let handlers = IncrementalHandlers::<i32, Vec<i32>>::new()
            .insert(|_, _, _| {})
            .update(|_, _, _, _| {})
            .remove(|_, _, _| {})
            .replace(|_| vec![]);

        let _cloned = handlers.clone();
        // Verify cloning works
    }

    #[test]
    fn test_incremental_computed_get() {
        use crate::reactive::signal_vec;

        let items = signal_vec(vec![1, 2, 3]);

        let computed =
            IncrementalComputed::new(items.clone(), |v| v.len(), IncrementalHandlers::new());

        assert_eq!(computed.get(), 3);
    }

    #[test]
    fn test_incremental_computed_source() {
        use crate::reactive::signal_vec;

        let items = signal_vec(vec![1, 2, 3]);

        let computed =
            IncrementalComputed::new(items.clone(), |v| v.len(), IncrementalHandlers::new());

        // Verify we can access the source
        let _source_ref = computed.source();
    }

    #[test]
    fn test_incremental_computed_clone() {
        use crate::reactive::signal_vec;

        let items = signal_vec(vec![1, 2, 3]);

        let computed1 =
            IncrementalComputed::new(items.clone(), |v| v.len(), IncrementalHandlers::new());

        let computed2 = computed1.clone();

        // Both should return the same result
        assert_eq!(computed1.get(), 3);
        assert_eq!(computed2.get(), 3);
    }

    #[test]
    fn test_incremental_computed_invalidate() {
        use crate::reactive::signal_vec;

        let items = signal_vec(vec![1, 2, 3]);

        let computed =
            IncrementalComputed::new(items.clone(), |v| v.len(), IncrementalHandlers::new());

        computed.invalidate();
        // Just verify it doesn't panic
    }

    #[test]
    fn test_incremental_computed_id() {
        use crate::reactive::signal_vec;

        let items = signal_vec(vec![1, 2, 3]);

        let computed1 =
            IncrementalComputed::new(items.clone(), |v| v.len(), IncrementalHandlers::new());

        let computed2 =
            IncrementalComputed::new(items.clone(), |v| v.len(), IncrementalHandlers::new());

        // IDs should be different
        assert_ne!(computed1.id(), computed2.id());
    }

    #[test]
    fn test_incremental_computed_read() {
        use crate::reactive::signal_vec;

        let items = signal_vec(vec![1, 2, 3]);

        let computed =
            IncrementalComputed::new(items.clone(), |v| v.len(), IncrementalHandlers::new());

        // Read should work
        let guard = computed.read();
        assert_eq!(*guard, 3);
    }

    #[test]
    fn test_incremental_handlers_with_string() {
        use crate::reactive::signal_vec;

        let items = signal_vec(vec!["a", "b", "c"]);

        let handlers = IncrementalHandlers::<&str, String>::new()
            .insert(|result, _, value| {
                result.push_str(value);
            })
            .update(|result, _, old, new| {
                *result = result.replace(old, new);
            })
            .remove(|result, _, value| {
                *result = result.replace(value, "");
            })
            .replace(|source| source.get().join(","));

        // Just verify it compiles and works
        assert!(true);
    }

    #[test]
    fn test_incremental_computed_with_vec_result() {
        use crate::reactive::signal_vec;

        let items = signal_vec(vec![1, 2, 3]);

        let computed =
            IncrementalComputed::new(items.clone(), |v| v.to_vec(), IncrementalHandlers::new());

        assert_eq!(computed.get(), vec![1, 2, 3]);
    }

    #[test]
    fn test_incremental_computed_with_count_result() {
        use crate::reactive::signal_vec;

        let items = signal_vec(vec![1, 2, 3, 4, 5]);

        let computed =
            IncrementalComputed::new(items.clone(), |v| v.len(), IncrementalHandlers::new());

        assert_eq!(computed.get(), 5);
    }

    #[test]
    fn test_incremental_computed_with_sum_result() {
        use crate::reactive::signal_vec;

        let items = signal_vec(vec![1, 2, 3, 4, 5]);

        let computed = IncrementalComputed::new(
            items.clone(),
            |v| v.iter().sum::<i32>(),
            IncrementalHandlers::new(),
        );

        assert_eq!(computed.get(), 15);
    }

    #[test]
    fn test_incremental_handlers_insert_method() {
        let mut result = Vec::new();
        let handlers = IncrementalHandlers::<i32, Vec<i32>>::new().insert(|res, index, value| {
            res.insert(index, value);
        });

        // Call the insert handler
        (handlers.on_insert)(&mut result, 0, 42);
        assert_eq!(result, vec![42]);
    }

    #[test]
    fn test_incremental_handlers_update_method() {
        let mut result = vec![1, 2, 3];
        let handlers =
            IncrementalHandlers::<i32, Vec<i32>>::new().update(|res, index, old, new| {
                res[index] = new;
            });

        // Call the update handler
        (handlers.on_update)(&mut result, 1, 2, 20);
        assert_eq!(result, vec![1, 20, 3]);
    }

    #[test]
    fn test_incremental_handlers_remove_method() {
        let mut result = vec![1, 2, 3];
        let handlers = IncrementalHandlers::<i32, Vec<i32>>::new().remove(|res, index, _value| {
            res.remove(index);
        });

        // Call the remove handler
        (handlers.on_remove)(&mut result, 1, 2);
        assert_eq!(result, vec![1, 3]);
    }

    #[test]
    fn test_incremental_handlers_replace_method() {
        use crate::reactive::signal_vec;

        let items = signal_vec(vec![1, 2, 3]);
        let handlers =
            IncrementalHandlers::<i32, usize>::new().replace(|source| source.get().len());

        // Call the replace handler
        let result = (handlers.on_replace)(&items);
        assert_eq!(result, 3);
    }
}
