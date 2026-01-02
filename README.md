<div align="center">

<img src="assets/banner.svg" alt="Revue" width="500">

<br>

**Build beautiful terminal UIs with the power of Rust and the elegance of CSS.**

[![CI](https://img.shields.io/github/actions/workflow/status/hawk90/revue/ci.yml?style=flat-square&logo=github&label=CI)](https://github.com/hawk90/revue/actions/workflows/ci.yml)
[![codecov](https://img.shields.io/codecov/c/github/hawk90/revue?style=flat-square&logo=codecov)](https://codecov.io/gh/hawk90/revue)
[![crates.io](https://img.shields.io/crates/v/revue?style=flat-square&logo=rust&label=crates.io)](https://crates.io/crates/revue)
[![downloads](https://img.shields.io/crates/d/revue?style=flat-square&label=downloads)](https://crates.io/crates/revue)
[![docs.rs](https://img.shields.io/docsrs/revue?style=flat-square&logo=docs.rs&label=docs.rs)](https://docs.rs/revue)
[![Release](https://img.shields.io/github/v/release/hawk90/revue?style=flat-square&logo=github&label=release)](https://github.com/hawk90/revue/releases)
[![license](https://img.shields.io/badge/license-MIT-green?style=flat-square)](LICENSE)
[![Rust 1.85+](https://img.shields.io/badge/rust-1.85+-orange?style=flat-square&logo=rust)](https://www.rust-lang.org)

[![Linux](https://img.shields.io/badge/Linux-supported-success?style=flat-square&logo=linux&logoColor=white)](https://github.com/hawk90/revue)
[![macOS](https://img.shields.io/badge/macOS-supported-success?style=flat-square&logo=apple&logoColor=white)](https://github.com/hawk90/revue)
[![Windows](https://img.shields.io/badge/Windows-supported-success?style=flat-square&logo=windows&logoColor=white)](https://github.com/hawk90/revue)

[Getting Started](#-quick-start) · [Examples](#-examples) · [Documentation](#-documentation) · [Contributing](#-contributing)

</div>

---

## ✨ Highlights

<table>
<tr>
<td width="50%">

### 🎨 CSS Styling
Write styles in familiar CSS syntax with variables, selectors, transitions, and animations.

</td>
<td width="50%">

### ⚡ Reactive State
Vue-inspired Signal/Computed/Effect system for automatic UI updates.

</td>
</tr>
<tr>
<td width="50%">

### 📦 70+ Widgets
Rich widget library: inputs, tables, charts, markdown, images, and more.

</td>
<td width="50%">

### 🔥 Hot Reload
See CSS changes instantly without restarting your app.

</td>
</tr>
<tr>
<td width="50%">

### 🛠️ Developer Tools
Widget inspector, snapshot testing, and performance profiler built-in.

</td>
<td width="50%">

### 🚀 Fast & Lightweight
Pure Rust, single binary, blazing fast performance.

</td>
</tr>
</table>

## 🆚 Comparison

| Feature | Ratatui | Cursive | Textual | **Revue** |
|:--------|:-------:|:-------:|:-------:|:---------:|
| Language | Rust | Rust | Python | **Rust** |
| Styling | Code | Theme | CSS | **CSS** |
| Reactivity | Manual | Event | Reactive | **Signal** |
| Hot Reload | ❌ | ❌ | ✅ | **✅** |
| Devtools | ❌ | ❌ | ✅ | **✅** |
| Binary | Single | Single | Python env | **Single** |

## 🚀 Quick Start

Add Revue to your project:

```bash
cargo add revue
```

Create your first app:

```rust
use revue::prelude::*;

fn main() -> Result<()> {
    let mut app = App::builder()
        .stylesheet("styles.css")
        .build();

    app.run_with_handler(Counter::new(), |event, state| {
        state.handle_event(event)
    })
}

struct Counter { count: i32 }

impl Counter {
    fn new() -> Self { Self { count: 0 } }

    fn handle_event(&mut self, event: &KeyEvent) -> bool {
        match event.key {
            Key::Up => self.count += 1,
            Key::Down => self.count -= 1,
            Key::Char('q') => return false,
            _ => {}
        }
        true
    }
}

impl View for Counter {
    fn render(&self, ctx: &mut RenderContext) {
        Border::rounded()
            .child(
                vstack().gap(1)
                    .child(Text::new(format!("Count: {}", self.count)).bold())
                    .child(Text::muted("[↑/↓] Change  [q] Quit"))
            )
            .render(ctx);
    }
}
```

Style with CSS:

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

## 📦 Widgets

<details>
<summary><b>Layout</b> - Stack, Grid, Scroll, Tabs, Accordion, Splitter</summary>

```rust
vstack().gap(1).child(/* ... */);
hstack().justify_content(Center);
grid().cols(3).gap(1);
tabs().tab("Home", home_view).tab("Settings", settings_view);
```

</details>

<details>
<summary><b>Input</b> - Input, TextArea, Select, Checkbox, Switch, Slider</summary>

```rust
input().placeholder("Enter name...");
textarea().rows(5);
select().options(["Option 1", "Option 2"]);
checkbox("Enable feature");
switch().on_change(|on| /* ... */);
```

</details>

<details>
<summary><b>Display</b> - Text, Markdown, Table, Progress, Badge, Image</summary>

```rust
text("Hello").bold().fg(Color::CYAN);
markdown("# Title\n**bold** and *italic*");
table().columns(["Name", "Age"]).rows(data);
progress(0.75).label("Loading...");
image_from_file("logo.png");
```

</details>

<details>
<summary><b>Feedback</b> - Modal, Toast, Notification, Tooltip</summary>

```rust
modal().title("Confirm").content(/* ... */);
toast("Saved!").level(Success);
tooltip("Click to submit").child(button("Submit"));
```

</details>

<details>
<summary><b>Data Viz</b> - BarChart, LineChart, Sparkline, Heatmap</summary>

```rust
barchart().data([("A", 10), ("B", 20), ("C", 15)]);
line_chart().series("Sales", sales_data);
sparkline(cpu_history);
```

</details>

## 🎯 Examples

```bash
cargo run --example counter      # Basic counter
cargo run --example todo         # Todo app
cargo run --example dashboard    # Charts & widgets
cargo run --example markdown     # Markdown viewer
cargo run --example forms        # Form inputs
```

## 📚 Documentation

| Resource | Description |
|:---------|:------------|
| [📖 API Docs](https://docs.rs/revue) | Full API reference |
| [🏗️ Architecture](docs/ARCHITECTURE.md) | System design |
| [🎨 CSS Reference](docs/CSS.md) | Supported CSS properties |
| [🧩 Widgets](docs/WIDGETS.md) | Widget catalog |

## 🗺️ Roadmap

| Version | Theme | Status |
|---------|-------|--------|
| v0.1.0 | Foundation | ✅ Released |
| v0.2.0 | Polish | ✅ Released |
| v0.3.0 | Plugin System | ✅ Released |
| v0.4.0 | Async & A11y | ✅ Released |
| v0.5.0 | DX & Testing | ✅ Released |
| v0.6.0 | Advanced UI | ✅ Released |
| v0.7.0 | Ecosystem | ✅ Released |
| v0.8.0 | Stability | ✅ Released |
| v0.9.0 | Documentation | ✅ Released |
| **v1.0.0** | **Production Ready** | **🎉 Current** |

## 🤝 Contributing

Contributions are welcome! See our [Contributing Guide](CONTRIBUTING.md) for details.

```bash
# Clone and setup
git clone https://github.com/hawk90/revue.git
cd revue

# Install git hooks (recommended)
brew install lefthook && lefthook install

# Build and test
cargo build
cargo test

# Run an example
cargo run --example counter
```

## 💡 Inspired By

- [Textual](https://github.com/Textualize/textual) - CSS styling for TUI
- [Ratatui](https://github.com/ratatui/ratatui) - Rust TUI ecosystem
- [Vue.js](https://vuejs.org/) - Reactivity system

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

---

<div align="center">

**[⬆ Back to Top](#-revue)**

Made with ❤️ by the Revue contributors

</div>
