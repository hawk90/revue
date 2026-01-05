# Plugin System Guide

Revue provides a plugin system for extending applications with reusable functionality.

## Overview

The plugin system consists of:

- **Plugin trait** - Interface for creating plugins
- **PluginContext** - Shared context for plugin data and utilities
- **PluginRegistry** - Manages plugin lifecycle and ordering

## Plugin Trait

### Basic Implementation

```rust
use revue::plugin::{Plugin, PluginContext};
use revue::Result;
use std::time::Duration;

struct MyPlugin {
    counter: usize,
}

impl MyPlugin {
    fn new() -> Self {
        Self { counter: 0 }
    }
}

impl Plugin for MyPlugin {
    fn name(&self) -> &str {
        "my-plugin"
    }

    fn on_init(&mut self, ctx: &mut PluginContext) -> Result<()> {
        ctx.log("Plugin initialized");
        Ok(())
    }

    fn on_mount(&mut self, ctx: &mut PluginContext) -> Result<()> {
        ctx.log("App started");
        Ok(())
    }

    fn on_tick(&mut self, ctx: &mut PluginContext, delta: Duration) -> Result<()> {
        self.counter += 1;
        Ok(())
    }

    fn on_unmount(&mut self, ctx: &mut PluginContext) -> Result<()> {
        ctx.log(&format!("Shutting down after {} ticks", self.counter));
        Ok(())
    }
}
```

### Registering Plugins

```rust
use revue::prelude::*;

fn main() -> Result<()> {
    App::builder()
        .plugin(MyPlugin::new())
        .plugin(AnotherPlugin::new())
        .build()
        .run(MyApp::new())
}
```

## Lifecycle Hooks

Plugins receive lifecycle callbacks in this order:

| Hook | When Called | Use Case |
|------|-------------|----------|
| `on_init` | Plugin registered | Load config, set up resources |
| `on_mount` | App starts running | Initialize runtime state |
| `on_tick` | Each frame | Periodic tasks, metrics |
| `on_unmount` | App shutting down | Cleanup, save state |

### Lifecycle Example

```rust
impl Plugin for MyPlugin {
    fn name(&self) -> &str { "my-plugin" }

    fn on_init(&mut self, ctx: &mut PluginContext) -> Result<()> {
        // One-time setup
        ctx.set_data("config", load_config()?);
        Ok(())
    }

    fn on_mount(&mut self, ctx: &mut PluginContext) -> Result<()> {
        // App is ready
        let (width, height) = ctx.terminal_size();
        ctx.log(&format!("Terminal: {}x{}", width, height));
        Ok(())
    }

    fn on_tick(&mut self, ctx: &mut PluginContext, delta: Duration) -> Result<()> {
        // Called every frame - keep lightweight
        self.elapsed += delta;
        Ok(())
    }

    fn on_unmount(&mut self, ctx: &mut PluginContext) -> Result<()> {
        // Cleanup
        save_state()?;
        Ok(())
    }
}
```

## Plugin Context

### Storing Data

```rust
fn on_init(&mut self, ctx: &mut PluginContext) -> Result<()> {
    // Store typed data (namespaced by plugin)
    ctx.set_data("counter", 0i32);
    ctx.set_data("name", "My Plugin".to_string());
    ctx.set_data("config", MyConfig::default());
    Ok(())
}
```

### Reading Data

```rust
fn on_tick(&mut self, ctx: &mut PluginContext, _delta: Duration) -> Result<()> {
    // Read data
    if let Some(counter) = ctx.get_data::<i32>("counter") {
        println!("Counter: {}", counter);
    }
    Ok(())
}
```

### Modifying Data

```rust
fn on_tick(&mut self, ctx: &mut PluginContext, _delta: Duration) -> Result<()> {
    // Modify data in place
    if let Some(counter) = ctx.get_data_mut::<i32>("counter") {
        *counter += 1;
    }
    Ok(())
}
```

### Cross-Plugin Data Access

```rust
fn on_tick(&mut self, ctx: &mut PluginContext, _delta: Duration) -> Result<()> {
    // Read data from another plugin (read-only)
    if let Some(fps) = ctx.get_plugin_data::<f64>("performance", "fps") {
        println!("FPS: {:.1}", fps);
    }
    Ok(())
}
```

### Terminal Information

```rust
fn on_mount(&mut self, ctx: &mut PluginContext) -> Result<()> {
    let (width, height) = ctx.terminal_size();
    let running = ctx.is_running();
    Ok(())
}
```

### Logging

```rust
fn on_tick(&mut self, ctx: &mut PluginContext, _delta: Duration) -> Result<()> {
    ctx.log("Debug message");
    ctx.warn("Warning message");
    ctx.error("Error message");
    Ok(())
}
```

## Plugin Priority

Control execution order with priority:

```rust
impl Plugin for MyPlugin {
    fn name(&self) -> &str { "my-plugin" }

    fn priority(&self) -> i32 {
        50  // Higher = runs first
    }
}
```

| Priority | Use Case |
|----------|----------|
| `100+` | Early initialization (logging, config) |
| `0` | Default (most plugins) |
| `-100-` | Late processing (metrics, cleanup) |

### Priority Example

```rust
// Runs first (priority 100)
struct LoggerPlugin;
impl Plugin for LoggerPlugin {
    fn name(&self) -> &str { "logger" }
    fn priority(&self) -> i32 { 100 }
}

// Runs last (priority -100)
struct MetricsPlugin;
impl Plugin for MetricsPlugin {
    fn name(&self) -> &str { "metrics" }
    fn priority(&self) -> i32 { -100 }
}
```

## Plugin Styles

Plugins can contribute CSS styles:

```rust
impl Plugin for MyPlugin {
    fn name(&self) -> &str { "my-plugin" }

    fn styles(&self) -> Option<&str> {
        Some(r#"
.my-plugin-widget {
    border: solid cyan;
    padding: 1;
}

.my-plugin-highlight {
    background: #7aa2f7;
    color: #1a1b26;
}
"#)
    }
}
```

Styles are automatically merged with the app's stylesheet.

## Built-in Plugins

### LoggerPlugin

Simple logging for debugging:

```rust
use revue::plugin::LoggerPlugin;

App::builder()
    .plugin(LoggerPlugin::new()
        .verbose(true)       // Log tick counts
        .log_interval(60))   // Log every 60 ticks
    .build()
```

### PerformancePlugin

Performance monitoring:

```rust
use revue::plugin::PerformancePlugin;

App::builder()
    .plugin(PerformancePlugin::new()
        .max_samples(120)
        .report_interval(Duration::from_secs(5)))
    .build()
```

Access metrics from other plugins:

```rust
fn on_tick(&mut self, ctx: &mut PluginContext, _delta: Duration) -> Result<()> {
    if let Some(fps) = ctx.get_plugin_data::<f64>("performance", "fps") {
        // Use FPS value
    }
    if let Some(frame_time) = ctx.get_plugin_data::<f64>("performance", "frame_time_ms") {
        // Use frame time
    }
    Ok(())
}
```

## Creating a Plugin Package

### Project Structure

```
revue-plugin-my-feature/
├── Cargo.toml
├── src/
│   └── lib.rs
└── README.md
```

### Cargo.toml

```toml
[package]
name = "revue-plugin-my-feature"
version = "0.1.0"
edition = "2021"
description = "A Revue plugin for my feature"
keywords = ["tui", "revue", "plugin"]

[dependencies]
revue = "0.6"
```

### lib.rs

```rust
use revue::plugin::{Plugin, PluginContext};
use revue::Result;
use std::time::Duration;

pub struct MyFeaturePlugin {
    // State
}

impl MyFeaturePlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for MyFeaturePlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for MyFeaturePlugin {
    fn name(&self) -> &str {
        "revue-plugin-my-feature"
    }

    fn on_init(&mut self, ctx: &mut PluginContext) -> Result<()> {
        ctx.log("My feature plugin initialized");
        Ok(())
    }

    fn on_tick(&mut self, _ctx: &mut PluginContext, _delta: Duration) -> Result<()> {
        Ok(())
    }

    fn styles(&self) -> Option<&str> {
        Some(r#"
.my-feature-widget {
    border: solid blue;
}
"#)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_name() {
        let plugin = MyFeaturePlugin::new();
        assert_eq!(plugin.name(), "revue-plugin-my-feature");
    }
}
```

### Publishing

```bash
# Build and test
cargo build
cargo test

# Publish to crates.io
cargo publish
```

### Installing via CLI

```bash
revue plugin install my-feature
```

## Example: State Persistence Plugin

```rust
use revue::plugin::{Plugin, PluginContext};
use revue::Result;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

pub struct PersistencePlugin {
    save_path: PathBuf,
    auto_save_interval: Duration,
    last_save: std::time::Instant,
}

impl PersistencePlugin {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            save_path: path.into(),
            auto_save_interval: Duration::from_secs(30),
            last_save: std::time::Instant::now(),
        }
    }

    pub fn auto_save_interval(mut self, interval: Duration) -> Self {
        self.auto_save_interval = interval;
        self
    }
}

impl Plugin for PersistencePlugin {
    fn name(&self) -> &str {
        "persistence"
    }

    fn priority(&self) -> i32 {
        -50  // Run late to capture final state
    }

    fn on_init(&mut self, ctx: &mut PluginContext) -> Result<()> {
        // Load saved state
        if self.save_path.exists() {
            let data = fs::read_to_string(&self.save_path)?;
            ctx.set_data("loaded_state", data);
            ctx.log("Loaded saved state");
        }
        Ok(())
    }

    fn on_tick(&mut self, ctx: &mut PluginContext, _delta: Duration) -> Result<()> {
        // Auto-save periodically
        if self.last_save.elapsed() >= self.auto_save_interval {
            if let Some(state) = ctx.get_data::<String>("app_state") {
                fs::write(&self.save_path, state)?;
                self.last_save = std::time::Instant::now();
                ctx.log("Auto-saved state");
            }
        }
        Ok(())
    }

    fn on_unmount(&mut self, ctx: &mut PluginContext) -> Result<()> {
        // Final save on shutdown
        if let Some(state) = ctx.get_data::<String>("app_state") {
            fs::write(&self.save_path, state)?;
            ctx.log("Saved state on shutdown");
        }
        Ok(())
    }
}
```

## Best Practices

### 1. Keep `on_tick` Lightweight

```rust
// Good: Quick check
fn on_tick(&mut self, ctx: &mut PluginContext, delta: Duration) -> Result<()> {
    self.elapsed += delta;
    if self.elapsed >= self.interval {
        self.do_work(ctx)?;
        self.elapsed = Duration::ZERO;
    }
    Ok(())
}

// Bad: Heavy work every tick
fn on_tick(&mut self, ctx: &mut PluginContext, _delta: Duration) -> Result<()> {
    self.expensive_operation()?;  // Don't do this
    Ok(())
}
```

### 2. Use Meaningful Names

```rust
// Good
fn name(&self) -> &str { "revue-plugin-git-status" }

// Bad
fn name(&self) -> &str { "plugin1" }
```

### 3. Handle Errors Gracefully

```rust
fn on_tick(&mut self, ctx: &mut PluginContext, _delta: Duration) -> Result<()> {
    match self.try_operation() {
        Ok(_) => {}
        Err(e) => {
            ctx.warn(&format!("Operation failed: {}", e));
            // Continue running, don't crash
        }
    }
    Ok(())
}
```

### 4. Document Your Plugin

```rust
/// Git status plugin for Revue
///
/// Shows git repository status in the UI.
///
/// # Example
///
/// ```rust,ignore
/// App::builder()
///     .plugin(GitStatusPlugin::new()
///         .poll_interval(Duration::from_secs(5)))
///     .build()
/// ```
pub struct GitStatusPlugin { /* ... */ }
```

### 5. Respect Priority Conventions

| Range | Purpose |
|-------|---------|
| `50-100` | Logging, config, early setup |
| `0-49` | Feature plugins |
| `-49-0` | UI enhancements |
| `-100--50` | Metrics, cleanup |

## See Also

- [CLI Guide - Plugin Commands](cli.md#plugin-commands)
- [Built-in Plugins API](../api/plugins.md)
