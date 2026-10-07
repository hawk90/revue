# Error Handling Guidelines

This guide defines the standard error handling patterns for revue.

## Overview

Revue uses a consistent error handling strategy to provide clear, actionable error messages while maintaining code ergonomics.

## Core Principles

1. **Public APIs always return `Result`** - Never panic in public APIs
2. **Use appropriate types** - `Result` for recoverable errors, `Option` for optional values
3. **Provide context** - Errors should explain what failed and why
4. **Never silently fail** - Either handle the error explicitly or propagate it

## Error Type Hierarchy

### Standard Library Types

```rust
// Use std::io::Result for IO operations
use std::io;

pub fn read_file(path: &Path) -> io::Result<String> {
    std::fs::read_to_string(path)
}
```

### Custom Error Types

Use `thiserror` for domain-specific errors:

```rust
use thiserror::Error;

/// Core revue error type
#[derive(Debug, Error)]
pub enum Error {
    /// CSS parsing error.
    #[error("CSS error: {0}")]
    Css(#[from] style::ParseError),

    /// I/O error (file loading, etc.).
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Layout computation error.
    #[error("Layout error: {0}")]
    Layout(#[from] layout::LayoutError),

    /// Rendering / buffer error.
    #[error("Render error: {0}")]
    Render(String),

    /// Generic error with a custom message (catch-all).
    #[error("Unexpected error: {0}")]
    Other(#[from] anyhow::Error),
}

/// Result type alias for revue operations
pub type Result<T> = std::result::Result<T, Error>;
```

## Usage Patterns

### Public APIs

Always return `Result<T>` for public APIs:

```rust
/// Parse CSS text into a StyleSheet
pub fn parse_css(input: &str) -> Result<StyleSheet> {
    if input.is_empty() {
        return Err(Error::Other(anyhow::anyhow!("CSS input is empty")));
    }
    // ...
    Ok(stylesheet)
}

/// Render a widget to a buffer
pub fn render(widget: &dyn View, buffer: &mut Buffer) -> Result<()> {
    if buffer.is_empty() {
        return Err(Error::Render("Buffer cannot be empty".to_string()));
    }
    // ...
    Ok(())
}
```

### Internal APIs

For internal functions, use the most appropriate type:

```rust
// Recoverable error - use Result
fn parse_color(value: &str) -> Result<Color> {
    if value.starts_with('#') {
        Color::from_hex(value)
            .map_err(|_| Error::Other(anyhow::anyhow!("Invalid hex color: {}", value)))
    } else {
        Color::from_name(value)
            .ok_or_else(|| Error::Other(anyhow::anyhow!("Unknown color name: {}", value)))
    }
}

// Optional value - use Option
fn get_widget(id: WidgetId) -> Option<&Widget> {
    widgets.get(&id)
}

// Truly invariant - use expect (with clear message)
fn get_parent(&self) -> &Widget {
    self.parent.expect("Widget must have a parent (invariant violated)")
}
```

### Error Context

Provide context when converting errors:

```rust
use anyhow::Context;

pub fn load_config(path: &Path) -> Result<Config> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| Error::Io(e))?
        .parse::<Config>()
        .map_err(|e| Error::Other(anyhow::anyhow!("Failed to parse config from {}: {}", path.display(), e)))?;
    Ok(content)
}
```

## Error Propagation

### Use the `?` Operator

The `?` operator is the preferred way to propagate errors:

```rust
pub fn process_file(input: &Path) -> Result<Output> {
    let content = std::fs::read_to_string(input)?;
    let parsed = parse_css(&content)?;
    let processed = process_stylesheet(&parsed)?;
    Ok(processed)
}
```

### Map Errors

Convert errors with `map_err`:

```rust
fn read_user_config() -> Result<Config> {
    let path = get_config_path();
    std::fs::read_to_string(&path)
        .map_err(|e| Error::Io(e))
}
```

### Add Context with `anyhow`

For complex error chains, use `anyhow`:

```rust
use anyhow::Context;

pub fn load_theme(path: &Path) -> Result<Theme> {
    let content = std::fs::read_to_string(path)
        .context("Failed to read theme file")?;

    let theme: Theme = serde_json::from_str(&content)
        .context("Failed to parse theme JSON")?;

    Ok(theme)
}
```

## Forbidden Patterns

### Never Use `.unwrap()` in Public APIs

```rust
// BAD - Will panic on error
pub fn parse_css(input: &str) -> StyleSheet {
    parse_css_internal(input).unwrap()
}

// GOOD - Propagate error
pub fn parse_css(input: &str) -> Result<StyleSheet> {
    parse_css_internal(input)
}
```

### Avoid Generic `.expect()` Messages

```rust
// BAD - Generic message
let value = map.get("key").expect("Key not found");

// GOOD - Specific context
let value = map.get("key")
    .expect("Config key 'key' must exist (checked during validation)");
```

### Don't Silence Errors

```rust
// BAD - Ignores errors
let _ = some_operation_that_might_fail();

// GOOD - Explicitly handle or propagate
if let Err(e) = some_operation_that_might_fail() {
    eprintln!("Warning: operation failed: {}", e);
    // Or: return Err(e.into());
}
```

## Testing Error Paths

Always test error conditions:

```rust
#[test]
fn test_parse_css_empty_input() {
    let result = parse_css("");
    assert!(matches!(result, Err(Error::Other(_))));
}

#[test]
fn test_parse_css_invalid_syntax() {
    let result = parse_css("invalid css content");
    assert!(matches!(result, Err(Error::Css(_))));
}

#[test]
fn test_parse_css_success() {
    let result = parse_css(".button { color: red; }");
    assert!(result.is_ok());
}
```

## Recovery Strategies

### Graceful Degradation

Provide fallbacks when possible:

```rust
pub fn load_or_default_theme(path: &Path) -> Theme {
    load_theme(path).unwrap_or_else(|e| {
        eprintln!("Warning: failed to load theme from {:?}: {}, using default", path, e);
        Theme::default()
    })
}
```

### Terminal Restoration on Panic

A TUI process that dies in raw mode with the alternate screen active leaves the
user with a terminal that echoes nothing and shows no cursor. Revue installs a
panic hook the moment a terminal enters TUI mode, so this is handled for you:

```rust
// Called automatically by Terminal::init / init_with_mouse.
// Call it yourself only if you drive the terminal through your own backend.
use revue::render::install_panic_hook;

crossterm::terminal::enable_raw_mode()?;
install_panic_hook();
```

The hook chains to the previously installed hook, so the panic message still
prints - and it prints *after* the alternate screen is torn down, where the user
can actually read it.

The terminal is restored **once** per TUI session, by whichever path gets there
first: the panic hook, `restore_terminal()`, or the `Terminal`'s own
`restore`/`Drop`. The later ones write nothing. This matters for unwinding
panics: the `Drop` that runs after the hook would otherwise leave the alternate
screen a second time, which moves the cursor back above the panic message so
the shell prompt overwrites it.

**Do not rely on `Drop` for this.** `Terminal` and `CrosstermBackend` do
implement `Drop`, and that covers a normal exit, but the release profile sets
`panic = "abort"`:

```toml
[profile.release]
panic = "abort"
```

On abort the process dies without unwinding, so **no destructor runs**. A panic
hook runs in both unwind and abort mode; that is why the hook, not `Drop`, is
what upholds the guarantee.

**Panics revue catches leave the terminal alone.** Some panics do not end the
app: a `TaskRunner`, `PooledTaskRunner`, `WorkerPool` or `WorkerHandle` task
that panics is reported as a failed task, `AsyncState` turns it into an error,
and `ErrorBoundary` shows its fallback for a child that panics while rendering.
The hook knows these panics are caught and keeps the terminal in TUI mode. It
also skips the chained hook, whose message would be printed onto the alternate
screen; with the `tracing` feature the panic is logged as a warning instead. A
panic caught by *your own* `catch_unwind` cannot be told apart from a fatal one,
so it still restores the terminal. And under `panic = "abort"` nothing is
caught at all: every panic ends the process, after the hook restores the
terminal.

For other paths out of TUI mode - shelling out to `$EDITOR`, or your own
custom panic hook - call the restore directly:

```rust
use revue::render::restore_terminal;

restore_terminal(); // ends the session: the Terminal's own restore/Drop then writes nothing
```

If you enter TUI mode again by hand afterwards, call `install_panic_hook()`
again to start a new session.

### Termination Signals

On unix, `App::run` treats `SIGTERM` (`kill`), `SIGHUP` (the terminal window
closed) and `SIGINT` (`kill -INT`) as a quit: the event loop stops, plugins'
`on_unmount` runs, the terminal is restored, and `run` returns `Ok(())` - the
same path as `app.quit()`. Ctrl+C is unaffected: in raw mode it is a key event,
not a signal.

- A second signal while that shutdown is still running kills the process.
- Outside `App::run`, the signals keep their default behavior.
- A signal that is already ignored (for example `SIGHUP` under `nohup`) stays
  ignored, and a handler another library installed first keeps running.

Windows has no such hook: Ctrl+Break or closing the console window ends the
process without unmounting plugins.

### Validation Errors

For user input validation, collect multiple errors:

```rust
pub fn validate_form(data: &FormData) -> Result<Form> {
    let mut errors = Vec::new();

    if data.email.is_empty() {
        errors.push("Email is required".to_string());
    }
    if !data.email.contains('@') {
        errors.push("Email format is invalid".to_string());
    }
    if data.password.len() < 8 {
        errors.push("Password must be at least 8 characters".to_string());
    }

    if !errors.is_empty() {
        return Err(Error::Other(anyhow::anyhow!(errors.join("; "))));
    }

    Ok(Form::from(data))
}
```

## Related Documentation

- [Testing Guide](testing.md)
- [State Management Guide](state.md)
- [Architecture](../ARCHITECTURE.md)
