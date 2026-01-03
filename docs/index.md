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
    App::new()
        .stylesheet("styles.css")
        .run(Counter::default())
}

#[derive(Default)]
struct Counter {
    count: Signal<i32>,
}

impl View for Counter {
    fn view(&self) -> impl View {
        vstack()
            .child(text!("Count: {}", self.count.get()))
            .child(button("+").on_click(|| self.count.update(|n| n + 1)))
    }
}
```

---

## Features

| | |
|:--|:--|
| **CSS Styling** | Familiar CSS syntax with variables and animations |
| **Reactive State** | Vue-inspired Signal/Computed/Effect system |
| **70+ Widgets** | Inputs, tables, charts, markdown, and more |
| **Hot Reload** | See CSS changes instantly |
| **Developer Tools** | Inspector, snapshot testing, profiler |

---

## Tutorials

| Level | Tutorial | Description |
|:------|:---------|:------------|
| Beginner | [Getting Started](tutorials/01-getting-started.md) | Install and create your first app |
| Beginner | [Counter App](tutorials/02-counter.md) | Learn state management |
| Intermediate | [Todo App](tutorials/03-todo.md) | Build a full-featured todo list |

---

## Guides

| Guide | Description |
|:------|:------------|
| [Styling](guides/styling.md) | CSS properties and theming |
| [State Management](guides/state.md) | Reactive state with signals |
| [Testing](guides/testing.md) | Test your TUI apps |
| [Accessibility](guides/accessibility.md) | Build inclusive apps |
| [Performance](guides/performance.md) | Optimization tips |

---

## Architecture

- [System Architecture](ARCHITECTURE.md) - Design overview
- [Features](FEATURES.md) - Widget catalog
- [Framework Comparison](FRAMEWORK_COMPARISON.md) - vs Ratatui, Cursive, Textual

---

<div align="center">

[crates.io](https://crates.io/crates/revue) · [docs.rs](https://docs.rs/revue) · [GitHub](https://github.com/hawk90/revue)

</div>
