//! Pinia-inspired centralized state stores
//!
//! Provides a scalable pattern for managing application state using
//! reactive signals with actions, getters, and devtools integration.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::prelude::*;
//! use revue::reactive::store::{Store, StoreExt};
//!
//! #[derive(Store)]
//! struct CounterStore {
//!     count: Signal<i32>,
//! }
//!
//! impl CounterStore {
//!     fn new() -> Self {
//!         Self {
//!             count: signal(0),
//!         }
//!     }
//!
//!     fn increment(&self) {
//!         let mut count = self.count.write();
//!         *count += 1;
//!     }
//!
//!     fn double(&self) -> Computed<i32> {
//!         computed(move || {
//!             self.count.get() * 2
//!         })
//!     }
//! }
//! ```

pub mod usage;

use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

/// Unique identifier for a store instance
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StoreId(pub u64);

impl StoreId {
    #[allow(dead_code)]
    fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

/// Base store trait
///
/// All stores implement this trait to provide common functionality.
pub trait Store: Any + Send + Sync {
    /// Get the unique ID of this store
    fn id(&self) -> StoreId;

    /// Get the store name for debugging
    fn name(&self) -> &str;

    /// Get all state as a map of property names to values
    /// Used for devtools integration and state inspection
    fn get_state(&self) -> HashMap<String, String>;

    /// Get all getter values as a map
    fn get_getters(&self) -> HashMap<String, String>;
}

/// Store extension trait for common operations
pub trait StoreExt: Store {
    /// Subscribe to state changes (simplified)
    fn subscribe(&self) -> StoreSubscription;

    /// Check if store matches a given name
    fn is_name(&self, name: &str) -> bool {
        self.name() == name
    }
}

impl<T: Store> StoreExt for T {
    fn subscribe(&self) -> StoreSubscription {
        // Placeholder for future subscription system
        StoreSubscription {
            store_id: self.id(),
            _phantom: std::marker::PhantomData,
        }
    }
}

/// A subscription to a store
///
/// When dropped, the subscription is automatically cancelled.
#[derive(Clone, Debug)]
pub struct StoreSubscription {
    #[allow(dead_code)]
    store_id: StoreId,
    _phantom: std::marker::PhantomData<()>,
}

impl Drop for StoreSubscription {
    fn drop(&mut self) {
        // TODO: Unsubscribe from store when subscription system is implemented
    }
}

/// Global store registry
///
/// Maintains a registry of all active stores for devtools and debugging.
pub struct StoreRegistry {
    stores: Arc<std::sync::RwLock<HashMap<StoreId, Arc<dyn Store>>>>,
}

impl StoreRegistry {
    /// Create a new store registry
    pub fn new() -> Self {
        Self {
            stores: Arc::new(std::sync::RwLock::new(HashMap::new())),
        }
    }

    /// Register a store
    pub fn register(&self, store: Arc<dyn Store>) {
        let mut stores = self.stores.write().unwrap();
        stores.insert(store.id(), store);
    }

    /// Unregister a store
    pub fn unregister(&self, id: StoreId) {
        let mut stores = self.stores.write().unwrap();
        stores.remove(&id);
    }

    /// Get a store by ID
    pub fn get(&self, id: StoreId) -> Option<Arc<dyn Store>> {
        let stores = self.stores.read().unwrap();
        stores.get(&id).cloned()
    }

    /// Get all stores
    pub fn all(&self) -> Vec<Arc<dyn Store>> {
        let stores = self.stores.read().unwrap();
        stores.values().cloned().collect()
    }

    /// Find a store by name
    pub fn find_by_name(&self, name: &str) -> Option<Arc<dyn Store>> {
        let stores = self.stores.read().unwrap();
        stores.values().find(|s| s.name() == name).cloned()
    }
}

impl Default for StoreRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Global store registry instance
///
/// Use `store_registry()` to access the global registry.
pub fn store_registry() -> &'static StoreRegistry {
    use std::sync::OnceLock;
    static REGISTRY: OnceLock<StoreRegistry> = OnceLock::new();
    REGISTRY.get_or_init(StoreRegistry::new)
}

// Re-export usage helpers
pub use usage::{create_store, use_store};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_id_unique() {
        let id1 = StoreId::new();
        let id2 = StoreId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_store_registry_register() {
        let registry = StoreRegistry::new();

        // Create a mock store
        #[derive(Debug)]
        struct MockStore {
            id: StoreId,
            name: String,
        }

        impl Store for MockStore {
            fn id(&self) -> StoreId {
                self.id
            }

            fn name(&self) -> &str {
                &self.name
            }

            fn get_state(&self) -> HashMap<String, String> {
                HashMap::new()
            }

            fn get_getters(&self) -> HashMap<String, String> {
                HashMap::new()
            }
        }

        let store = Arc::new(MockStore {
            id: StoreId::new(),
            name: "test".to_string(),
        });

        registry.register(store.clone());
        assert!(registry.get(store.id).is_some());
    }

    #[test]
    fn test_store_registry_find() {
        let registry = StoreRegistry::new();

        #[derive(Debug)]
        struct MockStore {
            id: StoreId,
            name: String,
        }

        impl Store for MockStore {
            fn id(&self) -> StoreId {
                self.id
            }

            fn name(&self) -> &str {
                &self.name
            }

            fn get_state(&self) -> HashMap<String, String> {
                HashMap::new()
            }

            fn get_getters(&self) -> HashMap<String, String> {
                HashMap::new()
            }
        }

        let store = Arc::new(MockStore {
            id: StoreId::new(),
            name: "my_store".to_string(),
        });

        registry.register(store);
        assert!(registry.find_by_name("my_store").is_some());
        assert!(registry.find_by_name("nonexistent").is_none());
    }

    #[test]
    fn test_store_subscription() {
        let _registry = StoreRegistry::new();

        #[derive(Debug)]
        struct MockStore {
            id: StoreId,
            name: String,
        }

        impl Store for MockStore {
            fn id(&self) -> StoreId {
                self.id
            }

            fn name(&self) -> &str {
                &self.name
            }

            fn get_state(&self) -> HashMap<String, String> {
                HashMap::new()
            }

            fn get_getters(&self) -> HashMap<String, String> {
                HashMap::new()
            }
        }

        let store = Arc::new(MockStore {
            id: StoreId::new(),
            name: "test".to_string(),
        });

        let sub = store.subscribe();
        assert_eq!(sub.store_id, store.id());
    }

    #[test]
    fn test_use_store_singleton() {
        use super::usage::use_store;

        // Create a simple store that implements Store manually
        struct SingletonTestStore {
            _value: i32,
        }

        impl Store for SingletonTestStore {
            fn id(&self) -> StoreId {
                StoreId(42)
            }

            fn name(&self) -> &str {
                "SingletonTestStore"
            }

            fn get_state(&self) -> HashMap<String, String> {
                HashMap::new()
            }

            fn get_getters(&self) -> HashMap<String, String> {
                HashMap::new()
            }
        }

        impl Default for SingletonTestStore {
            fn default() -> Self {
                Self { _value: 0 }
            }
        }

        let store1 = use_store::<SingletonTestStore>();
        let store2 = use_store::<SingletonTestStore>();

        // Both should point to the same instance
        assert!(Arc::ptr_eq(&store1, &store2));
    }
}
