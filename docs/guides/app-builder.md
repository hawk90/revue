# App Builder Guide

The App Builder is the entry point for all Revue applications. It provides a fluent API for configuring your app with plugins, styles, hot reload, and developer tools.

## Overview

```rust
use revue::prelude::*;

fn main() -> Result<()> {
    let mut app = App::builder()
        .plugin(MyPlugin::new())
        .style("styles.css")
        .hot_reload(true)
        .devtools(true)
        .build();

    app.run(MyView, handler)?;
    Ok(())
}
```

## Builder Methods

### new()

Creates a new AppBuilder with default settings.

```rust
let builder = App::builder();
```

**Default values:**
- `hot_reload`: `false`
- `devtools`: `true` (when `devtools` feature is enabled)
- `mouse_capture`: `true`

### plugin(P)

Registers a plugin to extend application functionality.

```rust
use revue::plugin::LoggerPlugin;

App::builder()
    .plugin(LoggerPlugin::new().verbose(true))
    .plugin(PerformancePlugin::new())
```

See [Plugin System Guide](./plugins.md) for details on creating and using plugins.

### style(path)

Loads CSS styles from a file.

```rust
App::builder()
    .style("styles.css")
    .style("theme/dark.css")
```

Multiple `.style()` calls merge the stylesheets together.

### css(css_string)

Adds inline CSS styles directly.

```rust
App::builder()
    .css(r#"
        .container {
            padding: 2;
            border: rounded cyan;
        }
    "#)
```

Useful for quick prototyping or small apps.

### hot_reload(enabled)

Enables automatic CSS reloading when files change.

```rust
App::builder()
    .style("styles.css")
    .hot_reload(true)
```

**How it works:**
- Watches all CSS files added via `.style()`
- Automatically reloads when file changes are detected
- No application restart required

**Requirements:**
- `hot-reload` feature must be enabled (default)
- At least one CSS file must be specified via `.style()`

### devtools(enabled)

Enables or disables the built-in developer tools.

```rust
App::builder()
    .devtools(true)   // Enable devtools
```

**DevTools features:**
- Widget inspector - examine component structure
- Performance profiler - identify bottlenecks
- Snapshot testing - capture and compare UI states
- Style debugging - view computed styles

**Keyboard shortcuts:**
| Key | Action |
|-----|--------|
| `Ctrl+D` | Toggle devtools overlay |
| `Ctrl+I` | Open widget inspector |

> **Note:** DevTools are enabled by default when the `devtools` feature is active.

### mouse_capture(enabled)

Controls whether the app captures mouse events.

```rust
App::builder()
    .mouse_capture(false)  // Disable mouse handling
```

**When to disable:**
- Running in environments without mouse support
- Passing mouse events through to terminal
- Debugging keyboard-only workflows

**Default:** `true`

### build()

Constructs the `App` instance with all configured settings.

```rust
let mut app = App::builder()
    .style("styles.css")
    .hot_reload(true)
    .build();
```

**What happens during `build()`:**
1. Collects and merges all CSS stylesheets
2. Merges plugin-provided styles
3. Initializes all plugins
4. Sets up hot reload watcher (if enabled)
5. Creates the app instance

## App Instance Methods

After calling `.build()`, you get an `App` instance with these methods:

### run<V, H>(view, handler)

Starts the event loop with full event handling.

```rust
app.run(MyView, |event, view, app| {
    match event {
        Event::Key(key) => {
            match key.key {
                Key::Char('q') => false,  // Return false to quit
                Key::Up => {
                    view.move_up();
                    true  // Return true to request redraw
                }
                _ => true,
            }
        }
        Event::Resize(_, _) => true,  // Redraw on resize
        _ => false,
    }
});
```

**Parameters:**
- `view`: Your view implementation (implements `View` trait)
- `handler`: Event handler closure that returns `bool` (true = continue/keep running, false = quit)

**Returns:** `Result<()>`

### run_with_handler<V, H>(view, handler)

Simplified version for keyboard-only event handling.

```rust
app.run_with_handler(MyView, |key, view| {
    matches!(key, Key::Char('q') | Key::Escape)
});
```

## Common Patterns

### Minimal App

```rust
use revue::prelude::*;

fn main() -> Result<()> {
    let mut app = App::builder().build();
    app.run(HelloWorld, |event, _, _| {
        !matches!(event, Event::Key(k) if k.key == Key::Char('q'))
    })
}
```

### Production App with All Features

```rust
use revue::prelude::*;
use revue::plugin::{LoggerPlugin, PerformancePlugin};

fn main() -> Result<()> {
    let mut app = App::builder()
        // Styling
        .style("styles/theme.css")
        .css(".inline-style { color: cyan; }")
        .hot_reload(true)

        // Development
        .devtools(true)

        // Plugins
        .plugin(LoggerPlugin::new().verbose(true))
        .plugin(PerformancePlugin::new())

        // Input
        .mouse_capture(true)

        .build();

    app.run(MyApp, event_handler)?;
    Ok(())
}
```

### Plugin-Only App

```rust
use revue::prelude::*;
use revue::plugin::MyPlugin;

fn main() -> Result<()> {
    let mut app = App::builder()
        .plugin(MyPlugin::new())
        .build();

    app.run(MyView, handler)?;
    Ok(())
}
```

### CSS-Only Styling

```rust
let mut app = App::builder()
    .style("base.css")
    .style("components.css")
    .style("theme.css")
    .build();
```

### Inline CSS for Prototyping

```rust
let mut app = App::builder()
    .css(r#"
        .header { background: blue; color: white; }
        .content { padding: 2; }
        button {
            padding: 0 2;
            background: var(--primary);
        }
    "#)
    .build();
```

## Configuration Examples

### Development Mode

```rust
#[cfg(debug_assertions)]
let app = App::builder()
    .hot_reload(true)
    .devtools(true)
    .plugin(LoggerPlugin::new())
    .build();

#[cfg(not(debug_assertions))]
let app = App::builder()
    .hot_reload(false)
    .devtools(false)
    .build();
```

### Feature Flags

```toml
# Cargo.toml
[dependencies]
revue = { version = "2", features = ["devtools", "hot-reload"] }
```

### Conditional Hot Reload

```rust
let mut builder = App::builder()
    .style("styles.css");

// Only enable hot reload in debug builds
#[cfg(debug_assertions)]
{
    builder = builder.hot_reload(true);
}

let mut app = builder.build();
```

## Best Practices

### 1. Use `.style()` for Production

```rust
// Good for production
App::builder().style("styles.css")

// Avoid inline CSS in production
App::builder().css(".inline { ... }")
```

### 2. Enable Hot Reload in Development

```rust
#[cfg(debug_assertions)]
let builder = App::builder()
    .style("styles.css")
    .hot_reload(true);

#[cfg(not(debug_assertions))]
let builder = App::builder()
    .style("styles.css")
    .hot_reload(false);
```

### 3. Order Matters

Style methods can be called in any order, but a common pattern is:

```rust
App::builder()
    // 1. Plugins first
    .plugin(LoggerPlugin::new())
    .plugin(MyPlugin::new())

    // 2. Styles
    .style("base.css")
    .style("theme.css")
    .css(".overrides { }")

    // 3. Features
    .hot_reload(true)
    .devtools(true)
    .mouse_capture(true)

    // 4. Build
    .build()
```

### 4. Handle Build Errors

```rust
use revue::prelude::*;

fn main() -> Result<()> {
    let mut app = App::builder()
        .style("styles.css")
        .build();

    app.run(MyView, handler)?;
    Ok(())
}
```

The `?` operator propagates any errors that occur during app construction or execution.

## Error Handling

### Missing CSS Files

```rust
// Missing files log a warning but don't crash
let app = App::builder()
    .style("missing.css")  // Logs warning, continues
    .build();
```

### Invalid CSS

```rust
// Invalid CSS logs a warning but doesn't crash
let app = App::builder()
    .css("not valid {{{ css")  // Logs warning, continues
    .build();
```

### Plugin Initialization Failures

```rust
// Plugin failures log a warning but don't prevent startup
let app = App::builder()
    .plugin(FailingPlugin::new())  // Logs warning, continues
    .build();
```

## Feature Flags

| Feature | Default | Description |
|---------|:-------:|-------------|
| `devtools` | No | Enable developer tools |
| `hot-reload` | Yes | Enable CSS hot reload |

```toml
# Enable all features
revue = { version = "2", features = ["full"] }

# Minimal (no devtools, no hot reload)
revue = { version = "2", default-features = false }

# Selective
revue = { version = "2", features = ["devtools"] }
```

## See Also

- [Getting Started Tutorial](../tutorials/01-getting-started.md)
- [Plugin System Guide](./plugins.md)
- [Styling Guide](./styling.md)
- [DevTools Guide](../tutorials/05-styling.md#developer-tools)
