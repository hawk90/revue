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
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Invalid configuration: {0}")]
    Config(String),

    #[error("Style error: {0}")]
    Style(#[from] crate::style::StyleError),

    #[error("Widget not found: {0}")]
    WidgetNotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
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
        return Err(Error::Parse("CSS input is empty".to_string()));
    }
    // ...
    Ok(stylesheet)
}

/// Render a widget to a buffer
pub fn render(widget: &dyn View, buffer: &mut Buffer) -> Result<()> {
    if buffer.is_empty() {
        return Err(Error::InvalidInput("Buffer cannot be empty".to_string()));
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
            .map_err(|_| Error::Parse(format!("Invalid hex color: {}", value)))
    } else {
        Color::from_name(value)
            .ok_or_else(|| Error::Parse(format!("Unknown color name: {}", value)))
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
        .map_err(|e| Error::Parse(format!("Failed to parse config from {}: {}", path.display(), e)))?;
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
    assert!(matches!(result, Err(Error::Parse(_))));
}

#[test]
fn test_parse_css_invalid_syntax() {
    let result = parse_css("invalid css content");
    assert!(matches!(result, Err(Error::Parse(_))));
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
        return Err(Error::InvalidInput(errors.join("; ")));
    }

    Ok(Form::from(data))
}
```

## Related Documentation

- [Testing Guide](testing.md)
- [State Management Guide](state.md)
- [Architecture](../ARCHITECTURE.md)
