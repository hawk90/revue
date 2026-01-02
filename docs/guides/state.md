# State Management Guide

Revue provides reactive state primitives inspired by Vue.js and SolidJS.

## Signals

Signals are reactive values that automatically track dependencies.

### Creating Signals

```rust
use revue::prelude::*;

// Create a signal with initial value
let count = signal(0);

// Read the value
let current = count.get();  // 0

// Set a new value
count.set(5);

// Update with a function
count.update(|v| *v += 1);
```

### Signals are Thread-Safe

Signals use `Arc<RwLock<T>>` internally, making them safe to share across threads:

```rust
let count = signal(0);
let count_clone = count.clone();

std::thread::spawn(move || {
    count_clone.set(42);
});
```

## Computed Values

Computed values derive from other signals and automatically update:

```rust
let count = signal(0);
let doubled = computed(move || count.get() * 2);

count.set(5);
println!("{}", doubled.get()); // 10
```

Computed values are lazy - they only recalculate when accessed.

## Effects

Effects run side effects when dependencies change:

```rust
let count = signal(0);

effect(move || {
    println!("Count changed to: {}", count.get());
});

count.set(1); // Prints: "Count changed to: 1"
count.set(2); // Prints: "Count changed to: 2"
```

## Async State

For asynchronous operations:

```rust
// Load data asynchronously
let data = use_async(async {
    fetch_data().await
});

// Check state
match data.get() {
    AsyncResult::Loading => show_spinner(),
    AsyncResult::Ok(value) => show_data(value),
    AsyncResult::Err(e) => show_error(e),
}
```

### Polling

For periodic updates:

```rust
let stats = use_async_poll(Duration::from_secs(1), async {
    fetch_system_stats().await
});
```

### Immediate Execution

Start loading immediately:

```rust
let data = use_async_immediate(async {
    fetch_important_data().await
});
```

## State Patterns

### Form State

```rust
use revue::patterns::FormState;

let form = FormState::new()
    .field("username", FieldType::Text)
    .field("email", FieldType::Email)
    .field("age", FieldType::Number);

// Validate
if form.validate() {
    let values = form.values();
    submit(values);
}
```

### Search State

```rust
use revue::patterns::SearchState;

let search = SearchState::new(items)
    .mode(SearchMode::Fuzzy);

search.set_query("hello");
let results = search.filtered();
```

### Navigation State

```rust
use revue::patterns::NavigationState;

let nav = NavigationState::new();

nav.push("/home");
nav.push("/settings");
nav.back();  // Returns to /home
```

## Best Practices

### 1. Keep State Minimal

```rust
// Good: Store only what you need
struct App {
    items: Signal<Vec<Item>>,
    selected: Signal<usize>,
}

// Bad: Duplicate derived data
struct App {
    items: Signal<Vec<Item>>,
    selected: Signal<usize>,
    selected_item: Signal<Option<Item>>, // Derived!
}
```

### 2. Use Computed for Derived Values

```rust
let items = signal(vec![...]);
let selected_idx = signal(0);

// Derive selected item
let selected_item = computed(move || {
    items.get().get(selected_idx.get()).cloned()
});
```

### 3. Avoid Signal in Loops

```rust
// Bad: Creates many signals
for item in items {
    let sig = signal(item);
}

// Good: One signal for collection
let items = signal(vec![...]);
```

### 4. Use Effects Sparingly

Effects are for side effects (logging, persistence), not rendering:

```rust
// Good: Logging side effect
effect(move || {
    log::info!("Selection changed: {}", selected.get());
});

// Bad: Don't use effects for rendering logic
effect(move || {
    // This should be in render()
});
```

## Example: Todo App State

```rust
struct TodoApp {
    todos: Signal<Vec<Todo>>,
    input: Signal<String>,
    filter: Signal<Filter>,
}

impl TodoApp {
    fn new() -> Self {
        Self {
            todos: signal(vec![]),
            input: signal(String::new()),
            filter: signal(Filter::All),
        }
    }

    fn filtered_todos(&self) -> Vec<&Todo> {
        let todos = self.todos.get();
        let filter = self.filter.get();

        todos.iter().filter(|t| match filter {
            Filter::All => true,
            Filter::Active => !t.completed,
            Filter::Completed => t.completed,
        }).collect()
    }

    fn add_todo(&self) {
        let text = self.input.get();
        if !text.is_empty() {
            self.todos.update(|todos| {
                todos.push(Todo::new(text.clone()));
            });
            self.input.set(String::new());
        }
    }

    fn toggle(&self, id: usize) {
        self.todos.update(|todos| {
            if let Some(todo) = todos.get_mut(id) {
                todo.completed = !todo.completed;
            }
        });
    }
}
```
