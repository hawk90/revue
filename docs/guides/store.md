# Store Guide

Revue provides a Pinia-inspired centralized state management system using reactive signals with actions, getters, and devtools integration.

## Overview

Stores are singleton state containers that organize related state, actions, and getters into reusable units. They build on Revue's reactive primitives (`Signal`, `Computed`) to provide a scalable pattern for application-wide state.

## Defining a Store

Implement the `Store` trait to create a store:

```rust
use revue::reactive::{signal, Signal};
use revue::reactive::store::{Store, StoreId};
use std::collections::HashMap;

struct CounterStore {
    id: StoreId,
    count: Signal<i32>,
}

impl Store for CounterStore {
    fn id(&self) -> StoreId {
        self.id
    }

    fn name(&self) -> &str {
        "CounterStore"
    }

    fn get_state(&self) -> HashMap<String, String> {
        let mut state = HashMap::new();
        state.insert("count".to_string(), self.count.get().to_string());
        state
    }

    fn get_getters(&self) -> HashMap<String, String> {
        let mut getters = HashMap::new();
        getters.insert("doubled".to_string(), (self.count.get() * 2).to_string());
        getters
    }
}

impl Default for CounterStore {
    fn default() -> Self {
        Self {
            id: StoreId(1),
            count: signal(0),
        }
    }
}
```

### Store Trait Methods

| Method | Purpose |
|--------|---------|
| `id()` | Unique identifier for this store instance |
| `name()` | Human-readable name (used in devtools) |
| `get_state()` | Snapshot of all state as string key-value pairs |
| `get_getters()` | Snapshot of all computed/derived values |

## Using a Store

### Singleton Access with `use_store`

`use_store` returns a singleton instance. The first call creates the store via `Default::default()`, and subsequent calls return the same `Arc<T>`:

```rust
use revue::reactive::store::use_store;

// In any component - always returns the same instance
let counter = use_store::<CounterStore>();
counter.count.set(42);

// Elsewhere in the app - same instance
let counter2 = use_store::<CounterStore>();
assert_eq!(counter2.count.get(), 42);
```

### Fresh Instances with `create_store`

`create_store` always creates a new, independent instance:

```rust
use revue::reactive::store::create_store;

let store1 = create_store::<CounterStore>();
let store2 = create_store::<CounterStore>();
// store1 and store2 are independent instances
```

This is useful for testing or when you need isolated state.

## Actions

Define actions as methods on your store struct:

```rust
impl CounterStore {
    fn increment(&self) {
        self.count.update(|c| *c += 1);
    }

    fn decrement(&self) {
        self.count.update(|c| *c -= 1);
    }

    fn reset(&self) {
        self.count.set(0);
    }

    fn add(&self, amount: i32) {
        self.count.update(|c| *c += amount);
    }
}
```

Usage:

```rust
let counter = use_store::<CounterStore>();
counter.increment();
counter.add(10);
```

## Getters (Computed Values)

Use `Computed` signals for derived values:

```rust
use revue::reactive::computed;

impl CounterStore {
    fn doubled(&self) -> i32 {
        self.count.get() * 2
    }

    fn is_positive(&self) -> bool {
        self.count.get() > 0
    }
}
```

## Store Registry

The global `StoreRegistry` tracks all active stores for devtools and debugging:

```rust
use revue::reactive::store::{store_registry, StoreRegistry};
use std::sync::Arc;

// Register a store
let store = Arc::new(CounterStore::default());
store_registry().register(store.clone());

// Find stores
let found = store_registry().find_by_name("CounterStore");
let all = store_registry().all();

// Unregister
store_registry().unregister(store.id());
```

### Registry Methods

| Method | Returns | Purpose |
|--------|---------|---------|
| `register(store)` | `()` | Add a store to the registry |
| `unregister(id)` | `()` | Remove a store by ID |
| `get(id)` | `Option<Arc<dyn Store>>` | Find by ID |
| `find_by_name(name)` | `Option<Arc<dyn Store>>` | Find by name |
| `all()` | `Vec<Arc<dyn Store>>` | List all stores |

## Subscriptions

Subscribe to store changes using the `StoreExt` trait:

```rust
use revue::reactive::store::StoreExt;

let counter = use_store::<CounterStore>();
let subscription = counter.subscribe();

// Subscription is automatically cleaned up when dropped
drop(subscription);
```

## Real-World Example: Todo Store

```rust
use revue::reactive::{signal, Signal};
use revue::reactive::store::{Store, StoreId};
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Todo {
    id: usize,
    text: String,
    completed: bool,
}

struct TodoStore {
    id: StoreId,
    todos: Signal<Vec<Todo>>,
    next_id: Signal<usize>,
    filter: Signal<TodoFilter>,
}

#[derive(Clone, Copy, PartialEq)]
enum TodoFilter {
    All,
    Active,
    Completed,
}

impl Default for TodoStore {
    fn default() -> Self {
        Self {
            id: StoreId(100),
            todos: signal(vec![]),
            next_id: signal(1),
            filter: signal(TodoFilter::All),
        }
    }
}

impl TodoStore {
    // Actions
    fn add(&self, text: String) {
        let id = self.next_id.get();
        self.next_id.update(|n| *n += 1);
        self.todos.update(|todos| {
            todos.push(Todo { id, text, completed: false });
        });
    }

    fn toggle(&self, id: usize) {
        self.todos.update(|todos| {
            if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                todo.completed = !todo.completed;
            }
        });
    }

    fn remove(&self, id: usize) {
        self.todos.update(|todos| {
            todos.retain(|t| t.id != id);
        });
    }

    fn set_filter(&self, filter: TodoFilter) {
        self.filter.set(filter);
    }

    // Getters
    fn filtered_todos(&self) -> Vec<Todo> {
        let todos = self.todos.get();
        let filter = self.filter.get();
        todos.iter().filter(|t| match filter {
            TodoFilter::All => true,
            TodoFilter::Active => !t.completed,
            TodoFilter::Completed => t.completed,
        }).cloned().collect()
    }

    fn remaining_count(&self) -> usize {
        self.todos.get().iter().filter(|t| !t.completed).count()
    }
}

impl Store for TodoStore {
    fn id(&self) -> StoreId { self.id }
    fn name(&self) -> &str { "TodoStore" }

    fn get_state(&self) -> HashMap<String, String> {
        let mut state = HashMap::new();
        state.insert("count".into(), self.todos.get().len().to_string());
        state.insert("remaining".into(), self.remaining_count().to_string());
        state
    }

    fn get_getters(&self) -> HashMap<String, String> {
        let mut getters = HashMap::new();
        getters.insert("filtered_count".into(), self.filtered_todos().len().to_string());
        getters
    }
}
```

## DevTools Integration

Stores expose their state via `get_state()` and `get_getters()`, which the devtools inspector uses to display live state. Enable devtools to inspect stores at runtime:

```rust
let app = App::builder()
    .devtools(true)  // F12 to toggle
    .build();
```

The **State Debugger** panel shows all registered stores with their current state and getter values.

## See Also

- [State Management Guide](state.md) - Signals, Computed, Effects
- [Testing Guide](testing.md) - Testing stores with MockState
