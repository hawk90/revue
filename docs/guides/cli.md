# CLI Guide

The Revue CLI (`revue`) provides development tools for building terminal UI applications.

## Installation

```bash
cargo install revue-cli
```

## Commands Overview

| Command | Description |
|---------|-------------|
| `revue new` | Create a new project |
| `revue dev` | Start development server |
| `revue build` | Build for release |
| `revue add` | Add components/patterns |
| `revue snapshot` | Run snapshot tests |
| `revue inspect` | Launch widget inspector |
| `revue themes` | List available themes |
| `revue theme` | Install a theme |
| `revue docs` | Generate documentation |
| `revue benchmark` | Run benchmarks |
| `revue plugin` | Manage plugins |

## Project Commands

### `revue new`

Create a new Revue project.

```bash
revue new <name> [options]
```

**Options:**

| Option | Description | Default |
|--------|-------------|---------|
| `-t, --template` | Project template | `basic` |
| `--no-git` | Skip git initialization | `false` |

**Templates:**

| Template | Description |
|----------|-------------|
| `basic` | Minimal starter template |
| `dashboard` | System monitoring dashboard |
| `todo` | Todo app with persistence |
| `chat` | Messaging interface |

**Examples:**

```bash
# Basic project
revue new my-app

# Dashboard template
revue new monitor --template dashboard

# Without git
revue new quick-test --no-git
```

**Generated structure:**

```
my-app/
├── Cargo.toml
├── src/
│   ├── main.rs
│   └── app.rs
├── styles/
│   └── main.css
├── assets/
└── .gitignore
```

### `revue dev`

Start development server with hot reload.

```bash
revue dev [options]
```

**Options:**

| Option | Description | Default |
|--------|-------------|---------|
| `-p, --port` | Dev server port | `3000` |
| `-w, --watch` | Additional watch paths | `src`, `styles` |

**Examples:**

```bash
# Start dev server
revue dev

# Watch additional paths
revue dev --watch config --watch templates
```

### `revue build`

Build the project.

```bash
revue build [options]
```

**Options:**

| Option | Description | Default |
|--------|-------------|---------|
| `-r, --release` | Build in release mode | `false` |
| `-t, --target` | Target platform | current |

**Examples:**

```bash
# Debug build
revue build

# Release build
revue build --release

# Cross-compile
revue build --release --target x86_64-unknown-linux-gnu
```

## Component Commands

### `revue add`

Add a component or pattern to your project.

```bash
revue add <component> [options]
```

**Options:**

| Option | Description |
|--------|-------------|
| `-n, --name` | Custom filename |

**Component Types:**

| Component | Description | Generated File |
|-----------|-------------|----------------|
| `search` | Search with filter state | `src/search.rs` |
| `form` | Form with validation | `src/form.rs` |
| `navigation` | Navigation with history | `src/navigation.rs` |
| `modal` | Modal dialog | `src/modal.rs` |
| `toast` | Toast notifications | `src/toast.rs` |
| `command-palette` | Command palette | `src/command_palette.rs` |
| `table` | Data table | `src/data_table.rs` |
| `tabs` | Tab navigation | `src/tabs.rs` |

**Examples:**

```bash
# Add search component
revue add search

# Add form with custom name
revue add form --name user_form

# Add modal
revue add modal
```

**Usage after adding:**

```rust
// main.rs
mod search;

use search::SearchComponent;
```

## Testing Commands

### `revue snapshot`

Run snapshot tests for UI consistency.

```bash
revue snapshot [options]
```

**Options:**

| Option | Description |
|--------|-------------|
| `-u, --update` | Update snapshots instead of comparing |
| `-f, --filter` | Filter tests by name |

**Examples:**

```bash
# Run all snapshot tests
revue snapshot

# Update snapshots
revue snapshot --update

# Filter specific tests
revue snapshot --filter button
```

### `revue inspect`

Launch the widget inspector for debugging.

```bash
revue inspect [options]
```

**Options:**

| Option | Description | Default |
|--------|-------------|---------|
| `-m, --mode` | Inspector mode | `overlay` |

**Modes:**

| Mode | Description |
|------|-------------|
| `overlay` | Shows inspector as overlay |
| `panel` | Side panel inspector |

**Example:**

```bash
revue inspect --mode panel
```

## Theme Commands

### `revue themes`

List available themes.

```bash
revue themes [options]
```

**Options:**

| Option | Description |
|--------|-------------|
| `-v, --verbose` | Show detailed info |

**Available Themes:**

| Theme | Description |
|-------|-------------|
| `dracula` | Dark theme with purple accents |
| `nord` | Arctic, north-bluish palette |
| `monokai` | Sublime Text inspired |
| `gruvbox` | Retro groove color scheme |
| `catppuccin` | Soothing pastel theme |
| `tokyo-night` | Clean Tokyo-inspired dark theme |
| `one-dark` | Atom One Dark theme |
| `solarized-dark` | Precision colors |

**Example:**

```bash
# List themes
revue themes

# Detailed info
revue themes --verbose
```

### `revue theme`

Install a theme.

```bash
revue theme <name>
```

**Example:**

```bash
revue theme dracula
```

**Generated file:** `styles/<name>.css`

**Usage:**

```rust
App::builder()
    .style("styles/dracula.css")
    .build()
```

## Documentation Commands

### `revue docs`

Generate project documentation.

```bash
revue docs [options]
```

**Options:**

| Option | Description | Default |
|--------|-------------|---------|
| `-o, --output` | Output directory | `docs` |

**Example:**

```bash
revue docs

# Open generated docs
open target/doc/my_app/index.html
```

## Benchmark Commands

### `revue benchmark`

Run Criterion benchmarks.

```bash
revue benchmark [options]
```

**Options:**

| Option | Description |
|--------|-------------|
| `-f, --filter` | Specific benchmark to run |
| `-s, --save` | Save results to file |

**Prerequisites:**

Add to `Cargo.toml`:

```toml
[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "my_bench"
harness = false
```

**Examples:**

```bash
# Run all benchmarks
revue benchmark

# Filter specific benchmark
revue benchmark --filter rendering

# Save results
revue benchmark --save
```

**Results:** `target/criterion/report/index.html`

## Plugin Commands

### `revue plugin list`

List installed plugins.

```bash
revue plugin list
```

### `revue plugin search`

Search for plugins on crates.io.

```bash
revue plugin search <query>
```

**Example:**

```bash
revue plugin search git
```

### `revue plugin install`

Install a plugin.

```bash
revue plugin install <name> [options]
```

**Options:**

| Option | Description |
|--------|-------------|
| `-v, --version` | Specific version |

**Examples:**

```bash
# Install plugin
revue plugin install git

# Specific version
revue plugin install git --version 0.2.0
```

**Usage after install:**

```rust
use revue_plugin_git::GitPlugin;

App::builder()
    .plugin(GitPlugin::new())
    .build()
```

### `revue plugin info`

Show plugin information.

```bash
revue plugin info <name>
```

### `revue plugin new`

Create a new plugin project.

```bash
revue plugin new <name>
```

**Example:**

```bash
revue plugin new my-feature
```

**Generated structure:**

```
revue-plugin-my-feature/
├── Cargo.toml
├── src/
│   └── lib.rs
└── README.md
```

**Plugin template:**

```rust
use revue::plugin::{Plugin, PluginContext};
use revue::Result;

pub struct MyFeaturePlugin {
    // Plugin state
}

impl Plugin for MyFeaturePlugin {
    fn name(&self) -> &str {
        "revue-plugin-my-feature"
    }

    fn on_init(&mut self, ctx: &mut PluginContext) -> Result<()> {
        ctx.info("Plugin initialized");
        Ok(())
    }

    fn on_tick(&mut self, _ctx: &mut PluginContext, _delta: Duration) -> Result<()> {
        Ok(())
    }

    fn styles(&self) -> Option<&str> {
        Some(r#".my-widget { border: solid blue; }"#)
    }
}
```

## Quick Reference

```bash
# Create and run new project
revue new my-app && cd my-app && revue dev

# Add components
revue add search
revue add form --name login_form
revue add modal

# Install theme
revue theme catppuccin

# Run tests
revue snapshot
revue benchmark

# Build for release
revue build --release

# Plugin management
revue plugin search auth
revue plugin install auth
revue plugin new my-plugin
```

## See Also

- [Getting Started Tutorial](../tutorials/01-getting-started.md)
- [Plugin System Guide](plugins.md)
