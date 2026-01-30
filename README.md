<div align="center">

<img src="assets/banner.svg" alt="Revue" width="480">

**/rɪˈvjuː/** — *Re + Vue* — A Vue-style TUI framework for Rust

[![crates.io](https://img.shields.io/crates/v/revue?style=for-the-badge&logo=rust&logoColor=white&color=orange)](https://crates.io/crates/revue)
[![docs.rs](https://img.shields.io/docsrs/revue?style=for-the-badge&logo=docs.rs&logoColor=white)](https://docs.rs/revue)
[![GitHub Stars](https://img.shields.io/github/stars/hawk90/revue?style=for-the-badge&logo=github)](https://github.com/hawk90/revue/stargazers)

[![CI](https://img.shields.io/github/actions/workflow/status/hawk90/revue/ci.yml?style=flat-square&label=CI)](https://github.com/hawk90/revue/actions/workflows/ci.yml)
[![codecov](https://img.shields.io/codecov/c/github/hawk90/revue?style=flat-square&logo=codecov&logoColor=white)](https://codecov.io/gh/hawk90/revue)
[![license](https://img.shields.io/badge/license-MIT-green?style=flat-square)](LICENSE)
[![Rust 1.87+](https://img.shields.io/badge/MSRV-1.87%2B-orange?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![downloads](https://img.shields.io/crates/d/revue?style=flat-square&label=downloads&color=blue)](https://crates.io/crates/revue)

[Quick Start](#quick-start) · [Tutorials](docs/tutorials/) · [Examples](examples/) · [Documentation](docs/) · [Contributing](CONTRIBUTING.md)

</div>

<br>

## Why Revue?

> Build terminal UIs like you build web apps — with **CSS** and **reactive state**.

- **CSS Styling** — Write styles in familiar CSS syntax with variables, selectors, and animations
- **Reactive State** — Vue-inspired `Signal`/`Computed`/`Effect` system for automatic UI updates
- **100+ Widgets** — Rich component library: inputs, tables, charts, markdown, images, and more
- **Hot Reload** — See CSS changes instantly without restarting your app
- **Developer Tools** — Widget inspector, snapshot testing, and performance profiler built-in
- **Single Binary** — Pure Rust, no runtime dependencies, blazing fast

<br>

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
        let count = self.count.get();
        vstack()
            .class("container")
            .child(Text::new(format!("Count: {}", count)).bold())
            .child(
                hstack().gap(1)
                    .child(Text::new("↑/↓ to change, q to quit"))
            )
            .render(ctx);
    }
}
```

```css
/* styles.css */
.container {
    padding: 2;
    gap: 1;
    border: rounded cyan;
    align-items: center;
}

button {
    padding: 0 2;
    background: var(--primary);
}

button:hover {
    background: var(--primary-dark);
}
```

<br>

## Key Features

### Reactive Forms

Automatic validation with type-safe form state:

```rust
use revue::patterns::form::FormState;

let form = FormState::new()
    .field("email", |f| f
        .label("Email")
        .email()
        .required())
    .field("password", |f| f
        .label("Password")
        .password()
        .min_length(8))
    .field("confirm", |f| f
        .label("Confirm Password")
        .password()
        .matches("password"))
    .build();

// Reactive validation - errors auto-update when values change
form.set_value("email", "invalid");
assert!(!form.is_valid());

form.set_value("email", "user@example.com");
// Validation automatically recalculates
```

See [Forms Tutorial](docs/tutorials/06-forms.md) for complete guide.

### Animation System

Rich animations with easing functions and keyframes:

```rust
use revue::animation::{Animation, Easing};

// Fade in with custom easing
text("Hello!")
    .animation(Animation::fade_in()
        .duration(300)
        .easing(Easing::EaseInOutCubic))

// Slide in from left
text("Welcome!")
    .animation(Animation::slide_in_left()
        .duration(500)
        .delay(100))

// Keyframe animation
text("Pulsing!")
    .animation(Animation::keyframe(|keyframes| {
        keyframes
            .at(0, |kf| kf.scale(1.0).opacity(1.0))
            .at(50, |kf| kf.scale(1.2).opacity(0.8))
            .at(100, |kf| kf.scale(1.0).opacity(1.0))
    }))
```

### Worker Pool

Execute background tasks without blocking the UI:

```rust
use revue::worker::{WorkerHandle, WorkerPool};

// Spawn blocking task
let handle = WorkerHandle::spawn_blocking(|| {
    heavy_computation()
});

// Use worker pool
let pool = WorkerPool::new(4);
pool.submit(|| {
    fetch_data_from_api()
});

// Get result when ready
if let Some(result) = handle.try_recv() {
    // Update UI with result
}
```

### Hot Reload

CSS changes update instantly without restart:

```rust
let mut app = App::builder()
    .style("styles.css")
    .hot_reload(true)  // Enable hot reload
    .build();

app.run(view, handler)?;
```

Edit `styles.css` and see changes immediately - no restart needed!

### DevTools

Built-in widget inspector and profiler:

```rust
let mut app = App::builder()
    .devtools(true)  // Enable devtools
    .build();

app.run(view, handler)?;
```

**Keyboard shortcuts:**
- `Ctrl+D` — Toggle devtools overlay
- `Ctrl+I` — Open widget inspector

**Features:**
- Widget inspector with computed styles
- Performance profiler for identifying bottlenecks
- Snapshot testing for UI regression testing

<br>

## Widgets

| Category | Components |
|:---------|:-----------|
| **Layout** | `vstack` `hstack` `grid` `scroll` `tabs` `accordion` `splitter` `layers` |
| **Input** | `input` `textarea` `select` `checkbox` `radio` `switch` `slider` `number_input` |
| **Forms** | `form` `form_field` — Built-in validation system |
| **Display** | `text` `markdown` `table` `tree` `list` `progress` `badge` `image` `presentation` |
| **Feedback** | `modal` `toast` `notification` `tooltip` `popover` `alert` `callout` |
| **Charts** | `barchart` `line_chart` `sparkline` `heatmap` `gauge` `boxplot` `histogram` |
| **Advanced** | `rich_text_editor` `json_viewer` `csv_viewer` `diagram` `command_palette` |
| **Dev** | `debug_overlay` `snapshot_test` `profiler` |

> **100+ Widgets** — See [FEATURES.md](docs/FEATURES.md) for complete catalog

<br>

## Key Features

### Reactive Forms

Automatic validation with type-safe form state:

```rust
use revue::patterns::form::FormState;

let form = FormState::new()
    .field("email", |f| f
        .label("Email")
        .email()
        .required())
    .field("password", |f| f
        .label("Password")
        .password()
        .min_length(8))
    .field("confirm", |f| f
        .label("Confirm Password")
        .password()
        .matches("password"))
    .build();

// Reactive validation - errors auto-update when values change
form.set_value("email", "invalid");
assert!(!form.is_valid());

form.set_value("email", "user@example.com");
// Validation automatically recalculates
```

See [Forms Tutorial](docs/tutorials/06-forms.md) for complete guide.

### Animation System

Rich animations with easing functions and keyframes:

```rust
use revue::animation::{Animation, Easing};

// Fade in with custom easing
text("Hello!")
    .animation(Animation::fade_in()
        .duration(300)
        .easing(Easing::EaseInOutCubic))

// Slide in from left
text("Welcome!")
    .animation(Animation::slide_in_left()
        .duration(500)
        .delay(100))

// Keyframe animation
text("Pulsing!")
    .animation(Animation::keyframe(|keyframes| {
        keyframes
            .at(0, |kf| kf.scale(1.0).opacity(1.0))
            .at(50, |kf| kf.scale(1.2).opacity(0.8))
            .at(100, |kf| kf.scale(1.0).opacity(1.0))
    }))
```

### Worker Pool

Execute background tasks without blocking the UI:

```rust
use revue::worker::{WorkerHandle, WorkerPool};

// Spawn blocking task
let handle = WorkerHandle::spawn_blocking(|| {
    heavy_computation()
});

// Use worker pool
let pool = WorkerPool::new(4);
pool.submit(|| {
    fetch_data_from_api()
});

// Get result when ready
if let Some(result) = handle.try_recv() {
    // Update UI with result
}
```

### Hot Reload

CSS changes update instantly without restart:

```rust
let mut app = App::builder()
    .style("styles.css")
    .hot_reload(true)  // Enable hot reload
    .build();

app.run(view, handler)?;
```

Edit `styles.css` and see changes immediately - no restart needed!

### DevTools

Built-in widget inspector and profiler:

```rust
let mut app = App::builder()
    .devtools(true)  // Enable devtools
    .build();

app.run(view, handler)?;
```

**Keyboard shortcuts:**
- `Ctrl+D` — Toggle devtools overlay
- `Ctrl+I` — Open widget inspector

**Features:**
- Widget inspector with computed styles
- Performance profiler for identifying bottlenecks
- Snapshot testing for UI regression testing

<br>

## Examples

```bash
# Basics
cargo run --example counter       # Reactive counter with Signal
cargo run --example todo          # Full-featured todo app
cargo run --example hello_world   # Minimal "Hello World"

# UI Components
cargo run --example form          # Form validation demo
cargo run --example dashboard     # Charts and data widgets
cargo run --example gallery       # Widget showcase

# Advanced
cargo run --example animations    # Animation system
cargo run --example worker        # Background tasks
cargo run --example slideshow     # Terminal presentations
cargo run --example ide           # Rich text editor

# Real-world
cargo run --example chat          # Multi-user chat
cargo run --example data_explorer # JSON/CSV viewer
```

Browse all examples in the [examples/](examples/) directory.

<br>

## Tutorials

| Tutorial | Description | Time |
|:---------|:------------|:-----|
| [Getting Started](docs/tutorials/01-getting-started.md) | Install and create your first app | 5 min |
| [Counter App](docs/tutorials/02-counter.md) | Learn state management with signals | 10 min |
| [Todo App](docs/tutorials/03-todo.md) | Build a full-featured todo list | 20 min |
| [Reactive State](docs/tutorials/04-reactive.md) | Deep dive into Signal, Computed, Effect | 15 min |
| [Styling](docs/tutorials/05-styling.md) | CSS styling and theming | 15 min |
| [Forms](docs/tutorials/06-forms.md) | Form handling with validation | 20 min |

<br>

## Guides

| Guide | Description |
|:------|:------------|
| [App Builder](docs/guides/app-builder.md) | Complete App Builder API reference |
| [Styling](docs/guides/styling.md) | CSS properties, selectors, and theming |
| [State Management](docs/guides/state.md) | Reactive state with signals |
| [Testing](docs/guides/testing.md) | Test your TUI apps |
| [Accessibility](docs/guides/accessibility.md) | Build inclusive apps |
| [Performance](docs/guides/performance.md) | Optimization tips |
| [Plugin System](docs/guides/plugins.md) | Create and use plugins |

<br>

## Comparison

| | Revue | ratatui | reratui | Cursive | Textual |
|:--|:--:|:--:|:--:|:--:|:--:|
| **Type** | Framework | Library | Framework | Framework | Framework |
| **Language** | Rust | Rust | Rust | Rust | Python |
| **Architecture** | Retained | Immediate | Immediate | Retained | Retained |
| **Styling** | CSS Files | Inline | Inline-style | TOML | CSS |
| **State** | Signals | Manual | Hooks | Event | Reactive |
| **Widgets** | 100+ | 15 built-in | Components | 40+ | 35+ |
| **Layout** | Flex+Grid | Constraint | Flex | Dock | Dock+Grid |
| **Forms** | ✅ Built-in | ❌ | ❌ | ❌ | ✅ |
| **Animation** | ✅ Tween+Keyframes | ❌ | ❌ | ❌ | ✅ |
| **Worker Pool** | ✅ | ❌ | ❌ | ❌ | ❌ |
| **Hot Reload** | ✅ | ❌ | ❌ | ❌ | ✅ |
| **Devtools** | ✅ | ❌ | ❌ | ❌ | ✅ |
| **Single Binary** | ✅ | ✅ | ✅ | ✅ | ❌ |

> **Note**: [ratatui](https://crates.io/crates/ratatui) is a low-level TUI library (like React's DOM), while [reratui](https://crates.io/crates/reratui) is a React-like framework built on top of ratatui. See [Framework Comparison](docs/FRAMEWORK_COMPARISON.md#11-ratatui-vs-reratui-which-one-to-choose) for detailed analysis.

<br>

## Documentation

- **[Getting Started](docs/tutorials/01-getting-started.md)** — 5-minute tutorial
- **[Widget Catalog](docs/FEATURES.md)** — Complete widget reference
- **[App Builder Guide](docs/guides/app-builder.md)** — Complete App Builder API reference
- **[Styling Guide](docs/guides/styling.md)** — CSS properties and theming
- **[State Management](docs/guides/state.md)** — Signals, Computed, Effects
- **[API Reference](https://docs.rs/revue)** — Full API documentation
- **[Architecture](docs/ARCHITECTURE.md)** — System design

<br>

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

```bash
git clone https://github.com/hawk90/revue.git
cd revue && cargo test
```

**Development workflow:**
1. Fork the repository
2. Create a feature branch (`feat/your-feature`, `fix/your-bug`)
3. Make your changes with tests
4. Run `cargo test` and `cargo clippy`
5. Submit a pull request

<br>

## License

MIT License — see [LICENSE](LICENSE) for details.

<div align="center">
<br>

**[↑ Back to Top](#why-revue)**

[crates.io](https://crates.io/crates/revue) · [docs.rs](https://docs.rs/revue) · [GitHub](https://github.com/hawk90/revue)

<sub>Built with Rust</sub>

</div>
