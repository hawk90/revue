# Reactive State Management

Learn how to use Revue's reactive system: Signal, Computed, and Effect.

## Overview

Revue's reactive system is inspired by Vue.js:

| Primitive | Purpose |
|-----------|---------|
| `Signal<T>` | Reactive value that triggers updates when changed |
| `Computed<T>` | Derived value that auto-updates when dependencies change |
| `Effect` | Side effect that runs when dependencies change |

## Signal

A `Signal` is a reactive container for a value.

```rust
use revue::prelude::*;

// Create a signal
let count = signal(0);

// Read the value
let value = count.get();

// Set a new value
count.set(10);

// Update based on current value
count.update(|v| *v += 1);
```

### Example: Counter

```rust
struct Counter {
    count: Signal<i32>,
}

impl Counter {
    fn new() -> Self {
        Self { count: signal(0) }
    }

    fn increment(&mut self) {
        self.count.update(|v| *v += 1);
    }

    fn decrement(&mut self) {
        self.count.update(|v| *v -= 1);
    }
}

impl View for Counter {
    fn render(&self, ctx: &mut RenderContext) {
        let count = self.count.get();

        vstack()
            .child(Text::new(format!("Count: {}", count)).bold())
            .child(Text::muted("[+] increment  [-] decrement"))
            .render(ctx);
    }
}
```

## Computed

A `Computed` value derives from other reactive values and auto-updates.

```rust
let count = signal(0);

// Create computed that doubles the count
let count_clone = count.clone();
let doubled = computed(move || count_clone.get() * 2);

// Computed values are cached
let value = doubled.get();  // Only recalculates when count changes
```

### Example: Derived State

```rust
struct App {
    count: Signal<i32>,
    doubled: Computed<i32>,
    status: Computed<String>,
}

impl App {
    fn new() -> Self {
        let count = signal(0);

        let count_clone = count.clone();
        let doubled = computed(move || count_clone.get() * 2);

        let count_clone = count.clone();
        let status = computed(move || {
            let v = count_clone.get();
            if v > 0 { "Positive".into() }
            else if v < 0 { "Negative".into() }
            else { "Zero".into() }
        });

        Self { count, doubled, status }
    }
}

impl View for App {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .child(Text::new(format!("Count: {}", self.count.get())))
            .child(Text::new(format!("Doubled: {}", self.doubled.get())))
            .child(Text::new(format!("Status: {}", self.status.get())))
            .render(ctx);
    }
}
```

## Effect

An `Effect` runs side effects when dependencies change.

```rust
let count = signal(0);

let count_clone = count.clone();
effect(move || {
    let value = count_clone.get();
    println!("Count changed to: {}", value);
});

// Changing count will trigger the effect
count.set(5);  // Prints: "Count changed to: 5"
```

### Common Use Cases

```rust
// Logging
effect(move || {
    println!("State: {:?}", state.get());
});

// Saving to file
effect(move || {
    let data = app_state.get();
    fs::write("state.json", serde_json::to_string(&data).unwrap()).ok();
});

// Announcing to screen reader
effect(move || {
    if error.get().is_some() {
        announce_error("An error occurred");
    }
});
```

## Combining Primitives

Here's a complete example using all three:

```rust
struct Form {
    name: Signal<String>,
    email: Signal<String>,
    is_valid: Computed<bool>,
    error: Computed<Option<String>>,
}

impl Form {
    fn new() -> Self {
        let name = signal(String::new());
        let email = signal(String::new());

        // Computed validation
        let name_clone = name.clone();
        let email_clone = email.clone();
        let is_valid = computed(move || {
            !name_clone.get().is_empty() && email_clone.get().contains('@')
        });

        // Computed error message
        let name_clone = name.clone();
        let email_clone = email.clone();
        let error = computed(move || {
            if name_clone.get().is_empty() {
                Some("Name is required".into())
            } else if !email_clone.get().contains('@') {
                Some("Invalid email".into())
            } else {
                None
            }
        });

        // Effect: log when form becomes valid
        let is_valid_clone = is_valid.clone();
        effect(move || {
            if is_valid_clone.get() {
                println!("Form is now valid!");
            }
        });

        Self { name, email, is_valid, error }
    }
}
```

## Best Practices

### 1. Clone signals for computed/effect

```rust
let count = signal(0);

// Clone before moving into closure
let count_clone = count.clone();
let doubled = computed(move || count_clone.get() * 2);
```

### 2. Keep computed values focused

```rust
// Good: single responsibility
let is_valid = computed(move || !name.get().is_empty());
let error_msg = computed(move || if name.get().is_empty() { Some("Required") } else { None });

// Avoid: doing too much
let validation = computed(move || {
    // Too complex, hard to reuse
});
```

### 3. Use effects sparingly

Effects are for side effects only (logging, saving, announcements). Don't use them for derived state - use `Computed` instead.

## Next Steps

- [Styling Guide](./05-styling.md) - CSS styling and theming
- [Forms Tutorial](./06-forms.md) - Form handling with validation
- [Counter Example](../../examples/counter.rs) - See reactive patterns in action
