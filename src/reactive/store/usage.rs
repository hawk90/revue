//! Store helper functions

use super::Store;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Type-specific store cache
///
/// Maps TypeId to Arc<dyn Any + Send + Sync> for singleton access by type.
struct TypeStoreCache {
    stores: RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>,
}

impl TypeStoreCache {
    fn new() -> Self {
        Self {
            stores: RwLock::new(HashMap::new()),
        }
    }

    /// Get or create a store instance for the given type
    fn get_or_create<T>(&self, factory: impl FnOnce() -> T) -> Arc<T>
    where
        T: Store + 'static,
    {
        let type_id = TypeId::of::<T>();

        // Try to read from cache
        {
            let stores = self.stores.read().unwrap();
            if let Some(store_any) = stores.get(&type_id) {
                // Try to downcast to Arc<T>
                if let Ok(store) = Arc::downcast::<T>(store_any.clone()) {
                    return store;
                }
            }
        }

        // Not in cache, create new instance
        let new_store = Arc::new(factory());
        let store_any: Arc<dyn Any + Send + Sync> = new_store.clone();

        // Insert into cache
        {
            let mut stores = self.stores.write().unwrap();
            stores.insert(type_id, store_any);
        }

        new_store
    }
}

/// Global type store cache
fn type_store_cache() -> &'static TypeStoreCache {
    use std::sync::OnceLock;
    static CACHE: OnceLock<TypeStoreCache> = OnceLock::new();
    CACHE.get_or_init(TypeStoreCache::new)
}

/// Use a store in the current component
///
/// This returns a singleton instance of the store, creating it if necessary.
/// The same instance will be returned for the same store type throughout the application.
///
/// # Example
///
/// ```rust,ignore
/// use revue::reactive::{signal, use_store};
/// use revue::reactive::store::Store;
///
/// #[derive(Store)]
/// struct CounterStore {
///     count: Signal<i32>,
/// }
///
/// impl CounterStore {
///     fn new() -> Self {
///         Self {
///             count: signal(0),
///         }
///     }
///
///     fn increment(&self) {
///         self.count.update(|c| *c += 1);
///     }
/// }
///
/// // In component - always returns the same instance
/// let counter = use_store::<CounterStore>();
/// counter.increment();
/// ```
///
/// # Singleton Behavior
///
/// - First call creates the store instance using `Default::default()`
/// - Subsequent calls return the same instance
/// - The store is kept alive for the duration of the program
pub fn use_store<T>() -> Arc<T>
where
    T: Store + Default + 'static,
{
    type_store_cache().get_or_create::<T>(|| T::default())
}

/// Create a new store instance (always creates a new instance)
///
/// This is a convenience function for creating fresh store instances.
/// Unlike `use_store()`, this always creates a new instance rather than
/// returning a singleton.
///
/// # Example
///
/// ```rust,ignore
/// use revue::reactive::{signal, create_store};
/// use revue::reactive::store::Store;
///
/// #[derive(Store)]
/// struct TestStore {
///     value: Signal<i32>,
/// }
///
/// // Each call creates a new, independent instance
/// let store1 = create_store::<TestStore>();
/// let store2 = create_store::<TestStore>();
/// // store1 and store2 are different instances
/// ```
pub fn create_store<T>() -> Arc<T>
where
    T: Store + Default + 'static,
{
    Arc::new(T::default())
}
