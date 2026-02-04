<div align="center">

# Revue

**/rɪˈvjuː/** — *Re + Vue* — A Vue-style TUI framework for Rust

Build terminal UIs like you build web apps — with **CSS** and **reactive state**.

[![crates.io](https://img.shields.io/crates/v/revue?style=flat-square&logo=rust&logoColor=white)](https://crates.io/crates/revue)
[![docs.rs](https://img.shields.io/docsrs/revue?style=flat-square&logo=docs.rs)](https://docs.rs/revue)
[![GitHub Stars](https://img.shields.io/github/stars/hawk90/revue?style=flat-square&logo=github)](https://github.com/hawk90/revue/stargazers)

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
            .class("container")
            .gap(1)
            .child(Text::new(format!("Count: {}", self.count.get())).bold())
            .child(Text::new("↑/↓ to change, q to quit").muted())
            .render(ctx);
    }
}
```

```css
/* styles.css */
.container {
    padding: 2;
    border: rounded cyan;
    align-items: center;
}
```

---

## Features

| Feature | Description |
|:--------|:------------|
| **CSS Styling** | External CSS files with variables, selectors, transitions, and hot reload |
| **Reactive State** | Vue-inspired Signal/Computed/Effect system for automatic UI updates |
| **100+ Widgets** | Inputs, tables, charts, markdown, images, and more |
| **Hot Reload** | See CSS changes instantly without restarting |
| **Developer Tools** | Widget inspector, snapshot testing, and performance profiler |
| **Single Binary** | Pure Rust, no runtime dependencies |

---

## Tutorials

| Level | Tutorial | Description | Time |
|:------|:---------|:------------|:-----|
| Beginner | [Getting Started](tutorials/01-getting-started.md) | Install and create your first app | 5 min |
| Beginner | [Counter App](tutorials/02-counter.md) | Learn state management with signals | 10 min |
| Intermediate | [Todo App](tutorials/03-todo.md) | Build a full-featured todo list | 20 min |
| Intermediate | [Reactive State](tutorials/04-reactive.md) | Deep dive into Signal, Computed, Effect | 15 min |
| Intermediate | [Styling](tutorials/05-styling.md) | CSS styling and theming | 15 min |
| Intermediate | [Forms](tutorials/06-forms.md) | Form handling with validation | 20 min |

---

## Guides

| Guide | Description |
|:------|:------------|
| [App Builder](guides/app-builder.md) | Complete App Builder API reference |
| [Styling](guides/styling.md) | CSS properties, selectors, and theming |
| [State Management](guides/state.md) | Reactive state with signals |
| [Testing](guides/testing.md) | Test your TUI apps |
| [Accessibility](guides/accessibility.md) | Build inclusive apps |
| [Performance](guides/performance.md) | Optimization tips |
| [Animations](guides/animations.md) | Tween and keyframe animations |
| [Drag & Drop](guides/drag-drop.md) | Drag and drop system |
| [Plugins](guides/plugins.md) | Create and use plugins |
| [CLI](guides/cli.md) | Command-line interface patterns |
| [Error Handling](guides/error-handling.md) | Error management strategies |

---

## Widget Catalog

| Category | Widgets |
|:---------|:--------|
| **Layout** | vstack, hstack, grid, scroll, tabs, accordion, splitter, layers |
| **Input** | input, textarea, select, checkbox, radio, switch, slider |
| **Display** | text, markdown, table, tree, list, progress, badge, image |
| **Feedback** | modal, toast, notification, tooltip, popover, alert |
| **Charts** | barchart, line_chart, sparkline, heatmap, gauge |
| **Advanced** | rich_text_editor, json_viewer, csv_viewer, diagram |

> **100+ Widgets** — See [FEATURES.md](FEATURES.md) for complete catalog

---

## Architecture

- [**System Architecture**](ARCHITECTURE.md) - Design overview and components
- [**Features**](FEATURES.md) - Complete widget catalog
- [**Framework Comparison**](FRAMEWORK_COMPARISON.md) - vs ratatui, reratui, Cursive, Textual
- [**Tech Stack**](TECH_STACK.md) - Dependencies and tools

---

## API Reference

- **[crates.io](https://crates.io/crates/revue)** — Package registry
- **[docs.rs](https://docs.rs/revue)** — Full API documentation
- **[GitHub](https://github.com/hawk90/revue)** — Source code

---

<div align="center">

**MIT License** · Built with [Rust](https://www.rust-lang.org/)

</div>
