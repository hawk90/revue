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
| [Routing](guides/routing.md) | Navigation and routing |
| [Store](guides/store.md) | Centralized state management |
| [Query](guides/query.md) | Query DSL for filtering, sorting and searching items |
| [Constructor Patterns](guides/constructor-patterns.md) | Widget construction patterns |
| [Feature Flags](guides/feature-flags.md) | Cargo feature configuration |

---

## Migration

| Guide | From |
|-------|------|
| [v3.0.0](migration/v3.0.0.md) | 2.x: CSS reaches every widget, and the APIs removed in 3.0 |
| [v0.8.0](migration/v0.8.0.md) | 0.7.x |

## Design Notes

| Document | Description |
|:---------|:------------|
| [Anti-Pattern Catalog](anti-patterns/README.md) | 154 structural failure modes across 24 subsystems, with a machine-readable [`catalog.yaml`](anti-patterns/catalog.yaml) |
| [Architecture Review](anti-patterns/architecture-review.md) | External review of the current stack and alternative directions |
| [Phase 0 Baseline](refactor/phase0-baseline.md) | Committed benchmark numbers the 3.0 refactor is measured against |
| [Phase 0 Invariants](refactor/phase0-invariants.md) | Which design invariants are pinned by tests, and why the rest are not |
| [Phase 1 Reconciliation](refactor/phase1-reconciliation.md) | Keyed reconciliation, per-frame DOM updates, and what the benchmarks say |
| [Render Pipeline Findings](refactor/findings-render-pipeline.md) | Verified defects in how repaints are decided, and what they mean for 3.0 |
| [Layout Findings](refactor/findings-layout.md) | Why CSS layout properties did nothing, and what `css_layout` does about it |
| [Phase 2 Hit Test](refactor/phase2-hit-test.md) | Why `:hover` never matched in a running app, and what now drives it |
| [Style Invalidation Findings](refactor/findings-style-invalidation.md) | Why CSS stopped updating after the first frame, and what makes the cascade re-run |
| [Phase 2 Cascade Precedence](refactor/phase2-cascade-precedence.md) | Where a widget's own colors rank against the stylesheet, and why disabled was in the wrong row |
| [Selector Matcher Findings](refactor/findings-selector-matchers.md) | Two selector matchers that disagreed, and why descendant selectors stopped one level down |
| [Prelude Coverage](refactor/findings-prelude-coverage.md) | 74 widgets the docs use from the prelude but the prelude does not export |
| [Default Flip](refactor/findings-default-flip.md) | What turning `dom_from_render` and `css_layout` on by default changes, measured on every example |
| [Content-Sized Stack](refactor/design-content-sized-stack.md) | Why stacks size children by their content, and how `measure`/`fills` get there |
| [Event Dispatch Findings](refactor/findings-event-dispatch.md) | Why widget event dispatch is the prerequisite for moving to `on_key(&self, ctx)` |
| [Named Color Precedence](refactor/findings-named-color-precedence.md) | Where a builder's color still loses to the stylesheet, and what is left to fix |
| [Single-Responsibility Audit](refactor/srp-audit.md) | The per-file 3.x audit: criteria, every file's verdict, and the bugs found on the way |
| [Widget Matrix](refactor/findings-widget-matrix.md) | Widgets drawn at every edge size, content and state combination, and what broke |
| [Fault Injection](refactor/findings-fault-injection.md) | Failing output, odd events, broken text, missing resources and concurrency faults |
| [Event Sequences](refactor/findings-event-sequences.md) | Property-tested key and mouse sequences, and the widget states they put out of sync |
| [Widget Clone](refactor/findings-widget-clone.md) | Which widgets are `Clone`, which cannot be yet, and why |

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

> **100+ Widgets** — See the [feature registry](specs/features.yaml) for the complete list and status ([overview](FEATURES.md))

---

## Architecture

- [**System Architecture**](ARCHITECTURE.md) - Design overview and components
- [**Features**](FEATURES.md) - Overview and design notes (status: [specs/features.yaml](specs/features.yaml))
- [**Framework Comparison**](FRAMEWORK_COMPARISON.md) - vs Textual, ratatui, r3bl_tui, tui-realm, iocraft
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
