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

[Quick Start](#quick-start) · [Examples](examples/) · [Documentation](https://docs.rs/revue) · [Contributing](CONTRIBUTING.md)

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

## Widgets

| Category | Components |
|:---------|:-----------|
| **Layout** | `vstack` `hstack` `grid` `scroll` `tabs` `accordion` `splitter` `layers` |
| **Input** | `input` `textarea` `select` `checkbox` `radio` `switch` `slider` `number_input` |
| **Forms** | `form` `form_field` validation system |
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
Form::new()
    .field(FormField::new("email").label("Email").required())
    .field(FormField::new("password").label("Password").min_length(8))
    .on_submit(|data| handle_login(data))
```

### Animation System
Rich animations with easing functions:
```rust
text("Hello!")
    .animation(Animation::fade_in().duration(300))
    .animation(Animation::slide_in_left())
```

### Worker Pool
Background task execution:
```rust
let handle = WorkerHandle::spawn_blocking(|| heavy_computation());
pool.submit(|| fetch_data_from_api());
```

### Hot Reload
CSS changes update instantly without restart:
```rust
App::builder()
    .style("styles.css")
    .hot_reload(true)
    .build()
```

### DevTools
Built-in widget inspector and profiler:
```rust
App::builder()
    .devtools(true)
    .build()
```

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

<br>

## Comparison

| | Revue | Ratatui | Cursive | Textual |
|:--|:--:|:--:|:--:|:--:|
| **Language** | Rust | Rust | Rust | Python |
| **Styling** | CSS | Code | Theme | CSS |
| **Reactivity** | Signal | Manual | Event | Reactive |
| **Forms** | ✅ Built-in | ❌ | ❌ | ✅ |
| **Animation** | ✅ Tween+Keyframes | ❌ | ❌ | ✅ |
| **Worker Pool** | ✅ | ❌ | ❌ | ❌ |
| **Hot Reload** | ✅ | ❌ | ❌ | ✅ |
| **Devtools** | ✅ | ❌ | ❌ | ✅ |
| **Single Binary** | ✅ | ✅ | ✅ | ❌ |

<br>

## Documentation

- **[Getting Started](docs/tutorials/01-getting-started.md)** — 5-minute tutorial
- **[Widget Catalog](docs/FEATURES.md)** — Complete widget reference
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

<br>

## License

MIT License — see [LICENSE](LICENSE) for details.

<div align="center">
<br>

**[↑ Back to Top](#why-revue)**

<sub>Built with Rust</sub>

</div>
