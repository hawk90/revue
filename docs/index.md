<div align="center">

# Revue

**A Vue-style TUI framework for Rust with CSS styling**

[Quick Start](#quick-start) · [Tutorials](#tutorials) · [API Docs](https://docs.rs/revue) · [GitHub](https://github.com/hawk90/revue)

</div>

---

## Quick Start

```bash
cargo add revue
```

```rust
use revue::prelude::*;

fn main() -> Result<()> {
    let mut app = App::builder()
        .style("styles.css")
        .build();

    let counter = Counter::new();
    app.run(counter, |event, counter, _app| {
        if let Event::Key(key) = event {
            counter.handle_key(&key.key)
        } else {
            false
        }
    })
}

struct Counter {
    count: Signal<i32>,
}

impl Counter {
    fn new() -> Self {
        Self { count: signal(0) }
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Up => { self.count.update(|n| *n += 1); true }
            Key::Down => { self.count.update(|n| *n -= 1); true }
            _ => false,
        }
    }
}

impl View for Counter {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .child(Text::new(format!("Count: {}", self.count.get())))
            .child(Text::new("↑/↓ to change"))
            .render(ctx);
    }
}
```

---

## Features

| | |
|:--|:--|
| **CSS Styling** | Familiar CSS syntax with variables and animations |
| **Reactive State** | Vue-inspired Signal/Computed/Effect system |
| **100+ Widgets** | Inputs, tables, charts, markdown, and more |
| **Hot Reload** | See CSS changes instantly |
| **Developer Tools** | Inspector, snapshot testing, profiler |

---

## Tutorials

| Level | Tutorial | Description |
|:------|:---------|:------------|
| Beginner | [Getting Started](tutorials/01-getting-started.md) | Install and create your first app |
| Beginner | [Counter App](tutorials/02-counter.md) | Learn state management |
| Intermediate | [Todo App](tutorials/03-todo.md) | Build a full-featured todo list |
| Intermediate | [Reactive State](tutorials/04-reactive.md) | Signal, Computed, and Effect |
| Intermediate | [Styling](tutorials/05-styling.md) | CSS styling and theming |
| Intermediate | [Forms](tutorials/06-forms.md) | Form handling with validation |

---

## Guides

| Guide | Description |
|:------|:------------|
| [App Builder](guides/app-builder.md) | Complete App Builder API reference |
| [Styling](guides/styling.md) | CSS properties and theming |
| [State Management](guides/state.md) | Reactive state with signals |
| [Testing](guides/testing.md) | Test your TUI apps |
| [Accessibility](guides/accessibility.md) | Build inclusive apps |
| [Performance](guides/performance.md) | Optimization tips |

---

## Architecture

- [System Architecture](ARCHITECTURE.md) - Design overview
- [Features](FEATURES.md) - Widget catalog
- [Framework Comparison](FRAMEWORK_COMPARISON.md) - vs ratatui, reratui, Cursive, Textual

---

<div align="center">

[crates.io](https://crates.io/crates/revue) · [docs.rs](https://docs.rs/revue) · [GitHub](https://github.com/hawk90/revue)

</div>
